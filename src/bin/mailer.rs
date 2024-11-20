extern crate email_format;
extern crate mailstrom;

use casual_cli_lib::email::*;

use std::thread::sleep;
use std::time::Duration;

use email_format::Email;
use mailstrom::Mailstrom;
use mailstrom::config::Config;
use mailstrom::storage::MemoryStorage;



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