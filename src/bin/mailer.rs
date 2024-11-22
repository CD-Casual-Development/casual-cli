extern crate email_format;
extern crate mailstrom;
extern crate serde;
extern crate serde_json;
extern crate sqlx;

use casual_cli_lib::email::*;
use casual_cli_lib::models::{Account, Contract, Schedule};
use chrono::{Datelike, NaiveDateTime, Timelike};
use lettre::Transport;

use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

use anyhow::{anyhow, Ok, Result};
use mailstrom::config::Config;
use mailstrom::Mailstrom;
use sqlx::SqlitePool;

const SLEEP_TIME_MINUTES: u64 = 5;
const SLEEP_TIME_SECONDS: u64 = 60 * SLEEP_TIME_MINUTES;

fn add_months(date: NaiveDateTime, months: u32) -> Result<NaiveDateTime> {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let hour = date.hour();
    let minute = date.minute();
    let second = date.second();
    let subsec_nanos = date.nanosecond();
    let mut new_month = month + months;
    let mut new_year = year;
    let mut new_day = day;
    while new_month > 12 {
        new_month -= 12;
        new_year += 1;
    }
    while new_month < 1 {
        new_month += 12;
        new_year -= 1;
    }
    if day > 28 {
        while new_day > 28 {
            new_day -= 1;
            if new_day == 0 {
                new_day = 28;
                break;
            }
        }
    }

    Ok(NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(new_year, new_month, new_day).ok_or(anyhow!(
            "Error: add_months couldn't make new date {new_year}-{new_month}-{new_day}"
        ))?,
        chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, subsec_nanos).ok_or(anyhow!(
            "Error: add_months couldn't make new date T {hour}:{minute}:{second}:{subsec_nanos}"
        ))?,
    ))
}

async fn auto_schedule_contracts(db_pool: &SqlitePool) -> Result<()> {
    let contracts = sqlx::query_as!(Contract, "SELECT * FROM contracts")
        .fetch_all(db_pool)
        .await?;

    println!("Contracts: {:?}", contracts);

    for contract in contracts {
        println!("Auto-renew: {:?}", contract.auto_renew);
        if contract.auto_renew.unwrap_or(false) {
            println!("Processing contract: {:?}", contract);
            let sender = sqlx::query_as!(
                Account,
                "SELECT * FROM accounts WHERE id = ?",
                contract.sender_id
            )
            .fetch_one(db_pool)
            .await?;
            let recipient = sqlx::query_as!(
                Account,
                "SELECT * FROM accounts WHERE id = ?",
                contract.recipient_id
            )
            .fetch_one(db_pool)
            .await?;
            let invoice_peroid_months = contract.invoice_period_months.unwrap_or(1);
            let start_date = contract
                .start_date
                .unwrap_or(chrono::Local::now().naive_local());
            let end_date = contract.end_date.unwrap_or(add_months(
                start_date,
                invoice_peroid_months.try_into().unwrap(),
            )?);
            let sender_email = sender
                .email
                .unwrap_or("kenrick@casualdevelopment.nl".to_string());
            let sender_name = sender.name.unwrap_or("Casual Development".to_string());
            let recipient_email = recipient
                .email
                .expect("Recipient must have an email address");
            let recipient_name = recipient.name.unwrap_or("Client".to_string());
            let end_date_string = end_date.format("%Y-%m-%d").to_string();

            if end_date < chrono::Local::now().naive_local() + chrono::Duration::days(2) {
                let mut email = email_format::Email::new(
                    sender_email.as_str(),
                    chrono::Local::now()
                        .format("%a, %d %b %Y %H:%M:%S %z")
                        .to_string()
                        .as_str(),
                )
                .unwrap();

                email.set_sender(sender_email.as_str()).unwrap();
                email
                    .set_reply_to("CD Mailer <no-reply@casualdevelopment.nl>")
                    .unwrap();
                email
                    .set_to(format!("{} <{}>", recipient_name, recipient_email).as_str())
                    .unwrap();
                email.set_subject("Contract Renewal").unwrap();
                email.set_body(format!("Hello {},\r\n\
                \r\n\
                This is a friendly reminder that your contract with {} is set to expire on {}.\r\n\
                \r\n\
                If you would like to renew your contract, please contact us at your earliest convenience.\r\n\
                \r\n\
                Best regards,\r\n\
                Casual Development", recipient_name, sender_name, end_date_string).as_str()).unwrap();
                
                email
                    .write_to_file(
                        format!("contract-renewal-{}.eml", recipient_name).as_str(),
                        Some(end_date_string.as_str()),
                    )
                    .unwrap();
                let new_end_date = add_months(end_date, invoice_peroid_months.try_into().unwrap())?;
                sqlx::query!(
                    "UPDATE contracts SET end_date = ? WHERE id = ?",
                    new_end_date,
                    contract.id
                )
                .execute(db_pool)
                .await?;
            }
        }
    }
    Ok(())
}

async fn auto_schedule_schedule(db_pool: &SqlitePool) -> Result<()> {
    let schedule_items = sqlx::query_as!(Schedule, "SELECT * FROM schedule")
        .fetch_all(db_pool)
        .await?;

    println!("Schedule items: {:?}", schedule_items);
    Ok(())
}

async fn process_scheduled_emails() -> Result<Vec<email_format::Email>> {
    let mut emails = vec![];
    let path_name = format!(
        "./mails/schedule/{}",
        chrono::Local::now()
            .naive_local()
            .format("%Y-%m-%d")
            .to_string()
    );
    let path = std::path::Path::new(path_name.as_str());
    if !path.exists() || !path.is_dir() {
        println!("No emails to send today.");
        return Ok(emails);
    }
    fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .for_each(|path| {
            println!("Processing email: {:?}", path);
            let email = email_format::Email::from_file(path.to_str().unwrap()).unwrap();
            emails.push(email);
            fs::remove_file(path).unwrap();
        });

    Ok(emails)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    while !should_quit_daemon {
        dotenv::dotenv().ok();

        let db_pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        sqlx::migrate!().run(&db_pool).await?;

        auto_schedule_contracts(&db_pool).await?;
        auto_schedule_schedule(&db_pool).await?;

        let emails: Vec<Email> = process_scheduled_emails().await?;

        let mut mailstrom = Mailstrom::new(
            Config {
                helo_name: "casualdevelopment.nl".to_owned(),
                ..Default::default()
            },
            FileStorage::new(),
        );

        // We must explicitly tell mailstrom to start actually sending emails.  If we
        // were only interested in reading the status of previously sent emails, we
        // would not send this command.
        mailstrom.start().unwrap();

        let mut message_ids = Vec::<String>::with_capacity(emails.len());
        for email in emails {
            message_ids.push(mailstrom.send_email(email).unwrap());
        }

        let mut should_quit = false;
        let mut success = false;
        let mut amount_delivered = 0;
        let mut amount_failed = 0;
        let mut amount_deferred = 0;
        let mut amount_sent = 0;

        // Later on, after the worker thread has had time to process the request,
        // you can check the status:
        while !should_quit {
            amount_delivered = 0;
            amount_failed = 0;
            amount_deferred = 0;
            amount_sent = 0;
            let mut completed: Vec<bool> = vec![];
            let mut succeeded: Vec<bool> = vec![];
            for message_id in &message_ids {
                let status = mailstrom.query_status(&*message_id)?;
                amount_sent += status.recipient_status.len();
                println!("{:?} {:?}", mailstrom.worker_status(), status);

                if status.completed() {
                    completed.push(true);
                    if status.succeeded() {
                        succeeded.push(true);
                    } else {
                        succeeded.push(false);
                    }
                }

                status.recipient_status.iter().for_each(|r| match r.result {
                    mailstrom::DeliveryResult::Delivered(_) => amount_delivered += 1,
                    mailstrom::DeliveryResult::Failed(_) => amount_failed += 1,
                    mailstrom::DeliveryResult::Deferred(_, _) => amount_deferred += 1,
                    _ => {}
                });
            }

            if completed.len() == message_ids.len() {
                if succeeded.iter().all(|&x| x) {
                    success = true;
                }
                should_quit = true;
            }

            if amount_sent <= amount_delivered + amount_failed + amount_deferred {
                should_quit = true;
            }

            if !should_quit {
                sleep(Duration::from_secs(5));
            }
        }

        if success {
            println!(
                "Emails ({}) sent successfully! {} delivered, {} failed, {} deferred",
                amount_sent, amount_delivered, amount_failed, amount_deferred
            );
        } else {
            println!(
                "Failed sending some or all emails ({}). {} delivered, {} failed, {} deferred.",
                amount_sent, amount_delivered, amount_failed, amount_deferred
            );
        }
        println!("Sleeping for {} minutes...", SLEEP_TIME_MINUTES);

        sleep(Duration::from_secs(SLEEP_TIME_SECONDS));
    }
    Ok(())
}
