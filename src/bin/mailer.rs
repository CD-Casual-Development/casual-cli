extern crate email_format;
extern crate mailstrom;

// This file is within /src/bin directory cargo will compile it as a binary
// Get the root crate modules thats within /src directory


use std::thread::sleep;
use std::time::Duration;

use email_format::Email;
use mailstrom::Mailstrom;
use mailstrom::config::Config;
use mailstrom::storage::MemoryStorage;

trait EmailToFile {
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

trait EmailFromString {
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

trait FileToEmail {
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

fn main() {
    dotenv::dotenv().ok();

    let mut email = Email::new(
        "kenrick@casualdevelopment.nl",  // "From:"
        "Wed, 05 Jan 2015 15:13:05 +1300" // "Date:"
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
        MemoryStorage::new());

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
        let status = mailstrom.query_status(&*message_id).unwrap();
        amount_sent = status.recipient_status.len();
        println!("{:?}", status);

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
}