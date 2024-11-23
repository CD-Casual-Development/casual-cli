extern crate anyhow;
extern crate chrono;
extern crate dotenv;
extern crate lettre;
extern crate serde;
extern crate serde_json;
extern crate sqlx;
extern crate tokio;

use address::Envelope;
use casual_cli_lib::clapargs::InvoiceMakeArgs;
use casual_cli_lib::models::{Account, Contract, Schedule};
use casual_cli_lib::queries::make_invoice;
use chrono::{Datelike, NaiveDateTime, Timelike};
use lettre::*;
use transport::smtp;
use transport::smtp::response::Response;

use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

use anyhow::{anyhow, Ok, Result};
use sqlx::SqlitePool;

const SLEEP_TIME_MINUTES: u64 = 5;
const SLEEP_TIME_SECONDS: u64 = 60 * SLEEP_TIME_MINUTES;

type Message = lettre::Message;

struct RawMessage {
    envelope: Envelope,
    message: Vec<u8>,
    path: Option<PathBuf>,
}

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

            if end_date < chrono::Local::now().naive_local() + chrono::Duration::days(31) {
                let invoice_make_args = InvoiceMakeArgs {
                    contract_id: Some(contract.id),
                    quote_id: None,
                    project_id: None,
                    remarks: None,
                    discount: None,
                };

                let filename = make_invoice(&db_pool, &invoice_make_args).await?;
                let filebody = fs::read(&filename)?;
                let content_type = message::header::ContentType::parse("application/pdf").unwrap();
                let attachment = message::Attachment::new(filename).body(filebody, content_type);

                let email = Message::builder()
                    .from(sender_email.as_str().parse()?)
                    .date(
                    chrono::Local::now()
                        .into()
                    )
                    .reply_to("CD Mailer <no-reply@casualdevelopment.nl>".parse()?)
                    .to(format!("{} <{}>", recipient_name, recipient_email).parse()?)
                    .subject("Contract Renewal")
                    .multipart(
                        message::MultiPart::mixed().singlepart(
                            message::SinglePart::plain(format!("Hello {},\r\n\
                            \r\n\
                            This is a friendly reminder that your contract with {} is set to expire on {}.\r\n\
                            \r\n\
                            If you would like to renew your contract, please contact us at your earliest convenience.\r\n\
                            \r\n\
                            Best regards,\r\n\
                            Casual Development", recipient_name, sender_name, end_date_string)).into()
                        ).singlepart(attachment)
                    ).unwrap();
                let path = format!(
                    "./mails/schedule/{}",
                    chrono::Local::now()
                        .naive_local()
                        .format("%Y-%m-%d")
                        .to_string()
                );
                let path_dir = std::path::Path::new(path.as_str());
                if !path_dir.exists() || !path_dir.is_dir() {
                    fs::create_dir_all(path_dir)?;
                }
                let file_transport =
                    lettre::transport::file::FileTransport::with_envelope(path_dir);
                file_transport.send(&email).unwrap();

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

fn process_scheduled_emails(date: &str) -> Result<Vec<RawMessage>> {
    let mut emails = vec![];
    println!("Processing scheduled emails for date: {}", date);
    let all_scheduled_dates = fs::read_dir("./mails/schedule")?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            println!("Checking path: {:?}", path);
            if path.is_dir() {
                let schedule_date = path.file_name().unwrap().to_str().unwrap().to_string();
                let datetime =
                    NaiveDateTime::parse_from_str(format!("{} 00:00:00", schedule_date).as_str(), "%Y-%m-%d %H:%M:%S");
                let datetime_now = NaiveDateTime::parse_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S");
                println!("Checking datetime {}: {:?} <= {:?}", schedule_date, datetime, datetime_now);
                if datetime.is_ok()
                    && datetime_now.is_ok()
                    && datetime.unwrap() <= datetime_now.unwrap()
                {
                    Some(schedule_date)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    println!("Scheduled dates: {:?}", all_scheduled_dates);
    for scheduled_date in all_scheduled_dates {
        let schedule_path_name = format!("./mails/schedule/{}", scheduled_date);
        let schedule_dir = std::path::Path::new(schedule_path_name.as_str());
        if !schedule_dir.exists() || !schedule_dir.is_dir() {
            println!("No emails to send today ({}).", scheduled_date);
            continue;
        }
        let queue_path_name = format!("./mails/queue/{}", date);
        let queue_dir = std::path::Path::new(queue_path_name.as_str());
        if !queue_dir.exists() || !queue_dir.is_dir() {
            fs::create_dir_all(queue_dir)?;
        }

        fs::read_dir(schedule_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().unwrap() == "eml" {
                    Some(path)
                } else {
                    None
                }
            })
            .for_each(|path| {
                let mut path = path;
                println!("Processing email: {:?}", path);
                // let email = email_format::Email::from_file(path.to_str().unwrap()).unwrap();
                // emails.push(email);
                let transport = lettre::transport::file::FileTransport::with_envelope(schedule_dir);
                let (envelope, raw_msg) = transport
                    .read(path.file_stem().unwrap().to_str().unwrap())
                    .unwrap();

                let queue_file_name = format!(
                    "{}/{}",
                    queue_path_name,
                    &path.file_name().unwrap().to_str().unwrap()
                );
                let queue_file_path = PathBuf::from(queue_file_name);
                fs::rename(&path, &queue_file_path).unwrap();

                if path.set_extension("json") {
                    fs::rename(
                        &path,
                        format!(
                            "{}/{}",
                            queue_path_name,
                            &path.file_name().unwrap().to_str().unwrap()
                        ),
                    )
                    .unwrap();
                }

                let raw_message = RawMessage {
                    envelope,
                    message: raw_msg,
                    path: Some(queue_file_path),
                };

                emails.push(raw_message);
            });
        if schedule_dir.read_dir()?.count() == 0 {
            fs::remove_dir(schedule_dir)?;
        }
    }
    Ok(emails)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let should_quit_daemon = false;
    while !should_quit_daemon {
        let date_string = chrono::Local::now().format("%Y-%m-%d").to_string();

        let db_pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        sqlx::migrate!().run(&db_pool).await?;

        auto_schedule_contracts(&db_pool).await?;
        auto_schedule_schedule(&db_pool).await?;

        let emails = process_scheduled_emails(&date_string)?;

        let smtp_transport = if env::var("SMTP_SERVER").is_ok()
            && env::var("SMTP_USERNAME").is_ok()
            && env::var("SMTP_PASSWORD").is_ok()
        {
            println!("Using SMTP server: {}", env::var("SMTP_SERVER")?);
            lettre::transport::smtp::SmtpTransport::relay(&env::var("SMTP_SERVER")?)
                .unwrap()
                .credentials(lettre::transport::smtp::authentication::Credentials::new(
                    env::var("SMTP_USERNAME")?,
                    env::var("SMTP_PASSWORD")?,
                ))
                .build()
        } else {
            println!("Using unencrypted localhost SMTP server");
            transport::smtp::SmtpTransport::unencrypted_localhost()
        };
        let mut message_results =
            Vec::<(Result<Response, smtp::Error>, Option<PathBuf>)>::with_capacity(emails.len());
        for email in &emails {
            message_results.push((
                smtp_transport.send_raw(&email.envelope, &email.message.as_slice()),
                email.path.clone(),
            ));
            // message_ids.push(mailstrom.send_email(email).unwrap());
        }

        // let mut should_quit = false;
        let mut success = true;
        // let mut amount_delivered = 0;
        let mut amount_failed = 0;
        // let mut amount_deferred = 0;
        let mut amount_sent = 0;

        let sent_path_name = format!("./mails/sent/{}", date_string);
        let sent_dir = std::path::Path::new(sent_path_name.as_str());
        if !sent_dir.exists() || !sent_dir.is_dir() {
            fs::create_dir_all(sent_dir)?;
        }
        let fail_path_name = format!("./mails/failed/{}", date_string);
        let fail_dir = std::path::Path::new(fail_path_name.as_str());
        if !fail_dir.exists() || !fail_dir.is_dir() {
            fs::create_dir_all(fail_dir)?;
        }

        for message_result in message_results {
            match message_result {
                (std::result::Result::Ok(_), path) => {
                    amount_sent += 1;
                    if let Some(mut p) = path {
                        let sent_file_name = format!(
                            "{}/{}",
                            sent_path_name,
                            &p.file_name().unwrap().to_str().unwrap()
                        );
                        let sent_file_path = PathBuf::from(sent_file_name);
                        fs::rename(&p, &sent_file_path).unwrap();

                        if p.set_extension("json") {
                            fs::rename(
                                &p,
                                format!(
                                    "{}/{}",
                                    sent_path_name,
                                    &p.file_name().unwrap().to_str().unwrap()
                                ),
                            )
                            .unwrap();
                        }
                    }
                }
                (Err(e), path) => {
                    println!("Error sending email: {:?}", e);
                    amount_failed += 1;
                    success = false;
                    if let Some(mut p) = path {
                        let fail_file_name = format!(
                            "{}/{}",
                            fail_path_name,
                            &p.file_name().unwrap().to_str().unwrap()
                        );
                        let fail_file_path = PathBuf::from(fail_file_name);
                        fs::rename(&p, &fail_file_path).unwrap();

                        if p.set_extension("json") {
                            fs::rename(
                                &p,
                                format!(
                                    "{}/{}",
                                    fail_path_name,
                                    &p.file_name().unwrap().to_str().unwrap()
                                ),
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }

        if success {
            println!(
                "Emails ({}) sent successfully! {} sent {} failed.",
                &emails.len(),
                amount_sent,
                amount_failed
            );
        } else {
            println!(
                "Failed sending some or all emails ({}). {} sent {} failed.",
                &emails.len(),
                amount_sent,
                amount_failed
            );
        }
        println!("Sleeping for {} minutes...", SLEEP_TIME_MINUTES);

        sleep(Duration::from_secs(SLEEP_TIME_SECONDS));
    }
    Ok(())
}
