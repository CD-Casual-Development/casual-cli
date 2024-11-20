extern crate email_format;
extern crate mailstrom;
extern crate serde;
extern crate serde_json;
extern crate sqlx;

use casual_cli_lib::email::*;
use casual_cli_lib::models::Contract;
use email_format::rfc5322::Parsable;
use serde::{Deserialize, Serialize};

use std::{env, fs};
use std::thread::sleep;
use std::time::Duration;

use sqlx::SqlitePool;
use anyhow::Result;
use email_format::Email;
use mailstrom::Mailstrom;
use mailstrom::config::Config;
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


pub trait EmailToFile2 {
    fn write_to_file(&self, filename: &str) -> anyhow::Result<()>;
}

pub trait EmailFromString2 {
    fn from_string(s: &str) -> anyhow::Result<PreparedEmail>;
}

struct FileStorage {}

impl FileStorage {
    fn new() -> FileStorage {
        FileStorage {}
    }
}

#[derive(Debug)]
enum FileStorageError {
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

impl EmailToFile2 for PreparedEmail {
    fn write_to_file(&self, filename: &str) -> Result<()> {
        use std::fs::write;
        Ok(write(filename, self.as_sendable_email()?.message_to_string()?.as_bytes())?)
    }
}

impl EmailFromString2 for PreparedEmail {
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
        email.write_to_file(format!("./mails/storage/{}.eml", &internal_message_status.message_id).as_str()).map_err(|e| FileStorageError::AnyhowError(anyhow::anyhow!(e)))?;
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


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let db_pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&db_pool).await?;

    let _contracts = sqlx::query_as!(Contract, "SELECT * FROM contracts")
        .fetch_all(&db_pool)
        .await?
        .iter()
        .for_each(|contract| {
            println!("{:?}", contract);
        });

    let mut email = Email::new(
        "kenrick@casualdevelopment.nl",  // "From:"
        chrono::Local::now().format("%a, %d %b %Y %H:%M:%S %z").to_string().as_str() // "Date: Wed, 05 Jan 2015 15:13:05 +1300"
    ).unwrap();

    // email.set_bcc("myself@mydomain.com").unwrap();
    email.set_sender("kenrick@casualdevelopment.nl").unwrap();
    email.set_reply_to("CD Mailer <no-reply@casualdevelopment.nl>").unwrap();
    email.set_to("Kenrick Hotmail <kenrick_half8@hotmail.com>, Kenrick Gmail <kenrickhalff@hotmail.com>, Kenrick CD <kenrick@casualdevelopment.nl>").unwrap();
    // email.set_cc("Our Friend <friend@frienddomain.com>").unwrap();
    email.set_subject("Hello Friend").unwrap();
    email.set_body("Good to hear from you.\r\n\
                    I wish you the best.\r\n\
                    \r\n\
                    Your Friend").unwrap();

    // Write email to a file
    email.write_to_file("email.eml").unwrap();

    email = Email::from_file("email.eml").unwrap();
    
    let mut mailstrom = Mailstrom::new(
        Config {
            helo_name: "casualdevelopment.nl".to_owned(),
            ..Default::default()
        },
        FileStorage::new());

    // We must explicitly tell mailstrom to start actually sending emails.  If we
    // were only interested in reading the status of previously sent emails, we
    // would not send this command.
    mailstrom.start().unwrap();
    
    let message_id = mailstrom.send_email(email).unwrap();
    let mut should_quit = false;
    let mut success = false;
    let mut amount_delivered = 0;
    let mut amount_failed = 0;
    let mut amount_deferred = 0;
    let mut amount_sent = 0;

    // Later on, after the worker thread has had time to process the request,
    // you can check the status:
    while !should_quit {
        let status = mailstrom.query_status(&*message_id)?;
        amount_sent = status.recipient_status.len();
        println!("{:?} {:?}", mailstrom.worker_status(), status);

        if status.completed() {
            should_quit = true;
            if status.succeeded() {
                success = true;
            }
        }

        amount_delivered = 0;
        amount_failed = 0;
        amount_deferred = 0;
        status.recipient_status.iter().for_each(|r| {
            match r.result {
                mailstrom::DeliveryResult::Delivered(_) => amount_delivered += 1,
                mailstrom::DeliveryResult::Failed(_) => amount_failed += 1,
                mailstrom::DeliveryResult::Deferred(_, _) => amount_deferred += 1,
                _ => {}
            }
        });

        if amount_sent <= amount_delivered + amount_failed + amount_deferred {
            should_quit = true;
        }

        if !should_quit {
            sleep(Duration::from_secs(5));
        }
    }

    if success {
        println!("Emails ({}) sent successfully! {} delivered, {} failed, {} deferred", amount_sent, amount_delivered, amount_failed, amount_deferred);
    } else {
        println!("Failed sending some or all emails ({}). {} delivered, {} failed, {} deferred.", amount_sent, amount_delivered, amount_failed, amount_deferred);
    }

    Ok(())
}