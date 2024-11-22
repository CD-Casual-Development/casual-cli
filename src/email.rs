use std::fs::{self, write};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use email_format::{rfc5322::Parsable, Email};
use mailstrom::storage::{InternalMessageStatus, MailstromStorage, MailstromStorageError, PreparedEmail};

// extends InternalMessageStatus to include a boolean for whether the message has been retrieved
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MessageStatus {
    internal_message_status: InternalMessageStatus,
    retrieved: bool,
}

impl Into<InternalMessageStatus> for MessageStatus {
    fn into(self) -> InternalMessageStatus {
        self.internal_message_status.clone()
    }
}

impl From<InternalMessageStatus> for MessageStatus {
    fn from(status: InternalMessageStatus) -> Self {
        MessageStatus {
            internal_message_status: status,
            retrieved: false,
        }
    }
}

pub struct FileStorage {}

impl FileStorage {
    pub fn new() -> FileStorage {
        FileStorage {}
    }
}

#[derive(Debug)]
pub enum FileStorageError {
    IoError(std::io::Error),
    AnyhowError(anyhow::Error),
    SerdeJsonError(serde_json::Error),

}
impl std::fmt::Display for FileStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStorageError::IoError(e) => write!(f, "IO Error: {}", e),
            FileStorageError::AnyhowError(e) => write!(f, "Anyhow Error: {}", e),
            FileStorageError::SerdeJsonError(e) => write!(f, "Serde JSON Error: {}", e),
        }
    }
}

impl std::error::Error for FileStorageError {}

impl MailstromStorageError for FileStorageError {}

impl EmailToFile for PreparedEmail {
    fn write_to_file(&self, filename: &str, _subdir: Option<&str>) -> Result<()> {
        Ok(write(filename, self.as_sendable_email()?.message_to_string()?.as_bytes())?)
    }
}

impl EmailFromString for PreparedEmail {
    fn from_string(s: &str) -> Result<PreparedEmail> {
        let (email, _remainder) = Email::parse(s.as_bytes())?;

        Ok(PreparedEmail {
            to: email.get_to().iter().map(|a| a.to_string()).collect(),
            from: email.get_from().0.to_string(),
            message_id: email.get_message_id().unwrap().to_string(),
            message: email.to_string().as_bytes().to_vec(),
        })
    }
}

impl MailstromStorage for FileStorage {
    type Error = FileStorageError;
    /// Store a `PreparedEmail`.  This should overwrite if message-id matches an existing
    /// email.
    fn store(
        &mut self,
        email: PreparedEmail,
        internal_message_status: InternalMessageStatus,
    ) -> Result<(), Self::Error> {
        let status = MessageStatus {
            internal_message_status: internal_message_status.clone(),
            retrieved: false,
        };

        fs::create_dir_all("./mails/storage").map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        email.write_to_file(format!("./mails/storage/{}.eml", &internal_message_status.message_id).as_str(), None).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        serde_json::to_string(&status).map_err(|e| FileStorageError::SerdeJsonError(e)).and_then(|s| {
            fs::write(format!("./mails/storage/{}.json", &internal_message_status.message_id), s).map_err(|e| FileStorageError::IoError(e))
        })
    }

    /// Update the status of an email
    fn update_status(
        &mut self,
        internal_message_status: InternalMessageStatus,
    ) -> Result<(), Self::Error> {
        let status = MessageStatus {
            internal_message_status: internal_message_status.clone(),
            retrieved: false,
        };
        serde_json::to_string(&status).map_err(|e| FileStorageError::SerdeJsonError(e)).and_then(|s| {
            fs::write(format!("./mails/storage/{}.json", &internal_message_status.message_id), s).map_err(|e| FileStorageError::IoError(e))
        })
    }

    /// Retrieve a `PreparedEmail` and `InternalMessageStatus` based on the message_id
    fn retrieve(
        &self,
        message_id: &str,
    ) -> Result<(PreparedEmail, InternalMessageStatus), Self::Error> {
        let email = PreparedEmail::from_string(&fs::read_to_string(format!("./mails/storage/{}.eml", message_id)).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        let status: MessageStatus = serde_json::from_str(&fs::read_to_string(format!("./mails/storage/{}.json", message_id)).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        Ok((email, status.internal_message_status))
    }

    /// Retrieve an `InternalMessageStatus` based on the message_id
    fn retrieve_status(&self, message_id: &str) -> Result<InternalMessageStatus, Self::Error> {
        let status: MessageStatus = serde_json::from_str(
            &fs::read_to_string(format!("./mails/storage/{}.json", message_id))
                .map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?)
                .map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        Ok(status.internal_message_status)
    }

    /// Retrieve all incomplete emails (status only). This is used to continue retrying
    /// after shutdown and later startup.
    fn retrieve_all_incomplete(&self) -> Result<Vec<InternalMessageStatus>, Self::Error> { 
        fs::create_dir_all("./mails/storage").map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
        Ok(fs::read_dir("./mails/storage").map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?.filter_map(|entry| {
            let entry = entry.map_err(|e| FileStorageError::IoError(e)).unwrap();
            let path = entry.path();
            if path.is_file() {
                let status: Result<MessageStatus, FileStorageError> = serde_json::from_str(&fs::read_to_string(path).map_err(|e| FileStorageError::IoError(e)).unwrap()).map_err(|e| FileStorageError::SerdeJsonError(e));
                if let Ok(status) = status {
                    if !status.internal_message_status.as_message_status().completed() {
                        Some(status.internal_message_status)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).collect())
    }

    /// Retrieve all incomplete emails as well as all complete emails that have become
    /// complete since the last time this function was called. This can be implemented
    /// by storing a retrieved boolean as falswe when update_status saves as complete,
    /// and setting that boolean to true when this function is run.
    fn retrieve_all_recent(&mut self) -> Result<Vec<InternalMessageStatus>, Self::Error> {
        Ok(fs::read_dir("./mails/storage")
            .map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!("Error retrieve_all_recent: {}", e)))?
            .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let mut status: MessageStatus = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                if !status.internal_message_status.as_message_status().completed() || !status.retrieved {
                    if status.internal_message_status.as_message_status().completed() {
                        status.retrieved = true;
                        fs::write(path, serde_json::to_string(&status).unwrap()).unwrap();
                    }
                    Some(status.internal_message_status.clone())
                } else {
                    None
                }
            } else {
                None
            }
        }).collect())
    }
}
pub trait EmailToFile {
    fn write_to_file(&self, filename: &str, subdir: Option<&str>) -> anyhow::Result<()>;
}

impl EmailToFile for Email {
    fn write_to_file(&self, filename: &str, subdir: Option<&str>) -> anyhow::Result<()> {
        use fs::File;
        use std::io::Write;

        // If the file already exists, append .1.[ext], .2.[ext], etc. to the filename but before the extension
        // e.g. email.eml -> email.1.eml, email.2.eml, etc.
        // Make sure the directory exists if it doesn't
        match subdir {
            Some(subdir) => fs::create_dir_all(format!("./mails/schedule/{}", subdir))?,
            None => fs::create_dir_all("./mails/schedule")?,
        }
        let base_filename = match subdir {
            Some(subdir) => format!("./mails/schedule/{}/{}", subdir, filename),
            None => format!("./mails/schedule/{}", filename)
        };
        let path = std::path::Path::new(&base_filename);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        let mut filename = base_filename.clone();
        let mut i = 0;
        while std::path::Path::new(&filename).exists() {
            i += 1;
            filename = format!("{}{}.{}.{}", "./mails/schedule/", stem, i, extension);
        }
        // Save to ./mails/sent/ directory

        let mut file = File::create(filename)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }
}

pub trait EmailFromString {
    fn from_string(s: &str) -> anyhow::Result<Self> where Self: Sized;
}

impl EmailFromString for Email {
    fn from_string(s: &str) -> anyhow::Result<Email> {
        let (email, _remainder) = Email::parse(s.as_bytes())?;
        Ok(email)
    }
}

pub trait FileToEmail {
    fn from_file(filename: &str) -> std::io::Result<Self> where Self: Sized;
}

impl FileToEmail for Email {
    fn from_file(filename: &str) -> std::io::Result<Email> {
        use fs::read_to_string;
        let contents = read_to_string(format!("{}", filename))?;
        // parse contents into Email
        
        Ok(Email::from_string(&contents).unwrap())
    }
}