use email_format::Email;

pub trait EmailToFile {
    fn write_to_file(&self, filename: &str) -> std::io::Result<()>;
}

impl EmailToFile for Email {
    fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        // If the file already exists, append .1.[ext], .2.[ext], etc. to the filename but before the extension
        // e.g. email.eml -> email.1.eml, email.2.eml, etc.
        // Make sure the directory exists if it doesn't
        std::fs::create_dir_all("./mails/sent")?;
        let base_filename = format!("./mails/sent/{}", filename);
        let path = std::path::Path::new(&base_filename);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        let mut filename = base_filename.clone();
        let mut i = 0;
        while std::path::Path::new(&filename).exists() {
            i += 1;
            filename = format!("{}{}.{}.{}", "./mails/sent/", stem, i, extension);
        }
        // Save to ./mails/sent/ directory

        let mut file = File::create(filename)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }
}

pub trait EmailFromString {
    fn from_string(s: &str) -> anyhow::Result<Email>;
}

impl EmailFromString for Email {
    fn from_string(s: &str) -> anyhow::Result<Email> {
        let mut from = None;
        let mut date = None;
        let mut sender = None;
        let mut reply_to = None;
        let mut to = None;
        let mut cc = None;
        let mut bcc = None;
        let mut subject = None;
        let mut body = None;

        s.lines().for_each(|line| {
            if line.starts_with("From:") {
                from = Some(line[5..].trim().to_owned());
            } else if line.starts_with("Date:") {
                date = Some(line[5..].trim().to_owned());
            } else if line.starts_with("Sender:") {
                sender = Some(line[7..].trim().to_owned());
            } else if line.starts_with("Reply-To:") {
                reply_to = Some(line[9..].trim().to_owned());
            } else if line.starts_with("To:") {
                to = Some(line[3..].trim().to_owned());
            } else if line.starts_with("Cc:") {
                cc = Some(line[3..].trim().to_owned());
            } else if line.starts_with("Bcc:") {
                bcc = Some(line[4..].trim().to_owned());
            } else if line.starts_with("Subject:") {
                subject = Some(line[8..].trim().to_owned());
            } else if line.is_empty() {
                body = Some(s.lines().skip_while(|l| !l.is_empty()).skip(1).collect::<Vec<&str>>().join("\r\n"));
            }
        });

        if from.is_none() && date.is_none() {
            return Err(anyhow::anyhow!("Email must have 'From:' and 'Date:' headers"));
        }

        let mut email = Email::new(from.unwrap().as_str(), date.unwrap().as_str())?;
        if let Some(sender) = sender {
            email.set_sender(sender.as_str())?;
        }
        if let Some(reply_to) = reply_to {
            email.set_reply_to(reply_to.as_str())?;
        }
        if let Some(to) = to {
            email.set_to(to.as_str())?;
        }
        if let Some(cc) = cc {
            email.set_cc(cc.as_str())?;
        }
        if let Some(bcc) = bcc {
            email.set_bcc(bcc.as_str())?;
        }
        if let Some(subject) = subject {
            email.set_subject(subject.as_str())?;
        }
        if let Some(body) = body {
            email.set_body(body.as_str())?;
        }

        Ok(email)
    }
}

pub trait FileToEmail {
    fn from_file(filename: &str) -> std::io::Result<Email>;
}

impl FileToEmail for Email {
    fn from_file(filename: &str) -> std::io::Result<Email> {
        use std::fs::read_to_string;
        let contents = read_to_string(format!("./mails/sent/{}", filename))?;
        // parse contents into Email
        
        Ok(Email::from_string(&contents).unwrap())
    }
}