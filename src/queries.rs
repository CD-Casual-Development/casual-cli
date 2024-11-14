use std::io::Write;
use std::str::FromStr;

use anyhow::Result;
use chrono::{Datelike, Months};
use simple_pdf_generator::{Asset, AssetType, PrintOptions};
use sqlx::SqlitePool;
use struct_field_names_as_array::FieldNamesAsArray;

use crate::clapargs::*;
use crate::models::*;

pub async fn add_address(db: &SqlitePool, address: &AddressCreateArgs) -> Result<i64> {
    let address_id = sqlx::query!(
        r#"
INSERT INTO address (
    country,
    city,
    street,
    number,
    unit,
    postalcode
) VALUES (?, ?, ?, ?, ?, ?)
"#,
        address.country,
        address.city,
        address.street,
        address.number,
        address.unit,
        address.postalcode
    )
    .execute(db)
    .await?
    .last_insert_rowid();

    if address.company_id.is_some() {
        let company = sqlx::query!(
            r#"SELECT * FROM companies WHERE id = ?"#,
            address.company_id
        )
        .fetch_one(db)
        .await;

        if company.is_err() {
            return Err(anyhow::anyhow!("Address was created but company not found"));
        }

        sqlx::query!(
            r#"UPDATE companies SET
            address_id = ?
            WHERE id = ?
        "#,
            address_id,
            address.company_id
        )
        .execute(db)
        .await?;
    }

    if address.account_id.is_some() {
        let account = sqlx::query!(r#"SELECT * FROM accounts WHERE id = ?"#, address.account_id)
            .fetch_one(db)
            .await;

        if account.is_err() {
            return Err(anyhow::anyhow!("Address was created but account not found"));
        }

        sqlx::query!(
            r#"UPDATE accounts SET
            address_id = ?
            WHERE id = ?
        "#,
            address_id,
            address.account_id
        )
        .execute(db)
        .await?;
    }

    Ok(address_id)
}

pub async fn get_account(db: &SqlitePool, id: i64) -> Result<Account> {
    sqlx::query_as!(Account, r#"SELECT * FROM accounts WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_account(db: &SqlitePool, account: &AccountCreateArgs) -> Result<i64> {
    let mut company_id: Option<i64> = None;
    let mut address_id: Option<i64> = None;

    if account.country.is_some() {
        let address = AddressCreateArgs {
            account_id: None,
            company_id: None,
            country: account.country.clone(),
            city: account.city.clone(),
            street: account.street.clone(),
            number: account.number.clone(),
            unit: account.unit.clone(),
            postalcode: account.postalcode.clone(),
        };

        address_id = Some(add_address(db, &address).await?);
    }

    if account.company_id.is_some() {
        company_id = account.company_id;

        let company = sqlx::query!(r#"SELECT * FROM companies WHERE id = ?"#, company_id)
            .fetch_one(db)
            .await;

        if company.is_err() {
            return Err(anyhow::anyhow!("Company not found"));
        }
    } else if account.company_name.is_some() {
        let company = CompanyCreateArgs {
            name: account.company_name.clone().unwrap(),
            logo: None,
            commerce_number: None,
            vat_number: None,
            iban: None,
            phone: None,
            email: None,
            account_id: None,
            address_id: None,
            country: None,
            city: None,
            street: None,
            number: None,
            unit: None,
            postalcode: None,
        };

        company_id = Some(add_company(db, &company).await?);
    }

    let result = sqlx::query!(
        r#"
INSERT INTO accounts (
    name,
    phone,
    email,
    company_id,
    address_id,
    privacy_permissions
) VALUES (?, ?, ?, ?, ?, ?)
"#,
        account.name,
        account.phone,
        account.email,
        company_id,
        address_id,
        account.privacy_permissions
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert account"));
    }

    Ok(result.last_insert_rowid())
}

pub async fn get_company(db: &SqlitePool, id: i64) -> Result<Company> {
    sqlx::query_as!(Company, r#"SELECT * FROM companies WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_company(db: &SqlitePool, company: &CompanyCreateArgs) -> Result<i64> {
    let mut address_id: Option<i64> = None;

    if company.address_id.is_some() {
        let address = sqlx::query!(r#"SELECT * FROM address WHERE id = ?"#, company.address_id)
            .fetch_one(db)
            .await;

        if address.is_err() {
            return Err(anyhow::anyhow!("Address not found"));
        }

        address_id = company.address_id;
    } else if company.country.is_some() {
        let address = AddressCreateArgs {
            account_id: None,
            company_id: None,
            country: company.country.clone(),
            city: company.city.clone(),
            street: company.street.clone(),
            number: company.number.clone(),
            unit: company.unit.clone(),
            postalcode: company.postalcode.clone(),
        };

        address_id = Some(add_address(db, &address).await?);
    }

    let company_id = sqlx::query!(
        r#"
INSERT INTO companies (
    name,
    logo,
    commerce_number,
    vat_number,
    iban,
    address_id
) VALUES (?, ?, ?, ?, ?, ?)
"#,
        company.name,
        company.logo,
        company.commerce_number,
        company.vat_number,
        company.iban,
        address_id
    )
    .execute(db)
    .await?
    .last_insert_rowid();

    if company.account_id.is_some() {
        let account = sqlx::query!(r#"SELECT * FROM accounts WHERE id = ?"#, company.account_id)
            .fetch_one(db)
            .await;

        if account.is_err() {
            return Err(anyhow::anyhow!("Company was created but account not found"));
        }

        sqlx::query!(
            r#"UPDATE accounts SET
            company_id = ?
            WHERE id = ?
        "#,
            company_id,
            company.account_id
        )
        .execute(db)
        .await?;
    }

    Ok(company_id)
}

pub async fn get_project(db: &SqlitePool, id: i64) -> Result<Project> {
    sqlx::query_as!(Project, r#"SELECT * FROM projects WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_project(db: &SqlitePool, project: &ProjectCreateArgs) -> Result<i64> {
    let project_id = sqlx::query!(
        r#"
INSERT INTO projects (
    title,
    description,
    client_id
) VALUES (?, ?, ?)
"#,
        project.title,
        project.description,
        project.client_id
    )
    .execute(db)
    .await?
    .last_insert_rowid();

    Ok(project_id)
}

pub async fn get_project_task(db: &SqlitePool, id: i64) -> Result<ProjectTask> {
    sqlx::query_as!(ProjectTask, r#"SELECT * FROM tasks WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_project_task(
    db: &SqlitePool,
    project_task: &ProjectTaskCreateArgs,
) -> Result<i64> {
    let result = sqlx::query!(
        r#"
INSERT INTO tasks (
    project_id,
    title,
    description,
    minutes_estimated,
    minutes_spent,
    minutes_remaining,
    minutes_billed,
    minute_rate
) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
"#,
        project_task.project_id,
        project_task.title,
        project_task.description,
        project_task.minutes_estimated,
        project_task.minutes_spent,
        project_task.minutes_remaining,
        project_task.minutes_billed,
        project_task.minute_rate
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert project task"));
    }

    Ok(result.last_insert_rowid())
}

pub async fn get_quote(db: &SqlitePool, id: i64) -> Result<Quote> {
    sqlx::query_as!(Quote, r#"SELECT * FROM quotes WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_quote(db: &SqlitePool, quote: &QuoteCreateArgs) -> Result<i64> {
    let result = sqlx::query!(
        r#"
INSERT INTO quotes (
    sender_id,
    recipient_id,
    project_id,
    project_duration,
    remarks,
    total_before_vat,
    discount,
    vat_percentage,
    currency,
    total_after_vat,
    quote_url
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
        quote.sender_id,
        quote.recipient_id,
        quote.project_id,
        quote.project_duration,
        quote.remarks,
        quote.total_before_vat,
        quote.discount,
        quote.vat_percentage,
        quote.currency,
        quote.total_after_vat,
        quote.quote_url
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert quote"));
    }

    Ok(result.last_insert_rowid())
}

mod quote {
    use serde::Serialize;
    use simple_pdf_generator_derive::PdfTemplate;
    use struct_field_names_as_array::FieldNamesAsArray;

    #[derive(Serialize, FieldNamesAsArray)]
    pub struct QuoteTableData {
        pub title: String,
        pub description: String,
        pub hours_estimated: f64,
        pub hourly_rate: String,
        pub total: String,
    }

    #[derive(PdfTemplate, FieldNamesAsArray)]
    pub struct QuoteTemplate {
        pub sender_name: String,
        pub sender_street: String,
        pub sender_postal_city: String,
        pub sender_phone: String,
        pub sender_email: String,
        pub sender_commerce_number: String,
        pub recipient_name: String,
        pub recipient_company_name: String,
        pub recipient_street: String,
        pub recipient_postal_city: String,
        pub send_date: String,
        pub project_title: String,
        pub project_description: String,
        pub project_duration: String,
        #[PdfTableData]
        pub project_tasks: Vec<QuoteTableData>,
        pub remarks: String,
        pub currency_symbol: String,
        pub total_before_vat: String,
        pub discount: String,
        pub vat_percentage: String,
        pub vat_amount: String,
        pub total_after_vat: String,
    }
}

mod invoice {
    use serde::Serialize;
    use simple_pdf_generator_derive::PdfTemplate;
    use struct_field_names_as_array::FieldNamesAsArray;

    #[derive(Serialize, FieldNamesAsArray)]
    pub struct InvoiceTableData {
        pub title: String,
        pub description: String,
        pub hours_spent: f64,
        pub hourly_rate: String,
        pub total: String,
    }

    #[derive(PdfTemplate, FieldNamesAsArray)]
    pub struct InvoiceTemplate {
        pub sender_name: String,
        pub sender_company_name: String,
        pub sender_street: String,
        pub sender_postal_city: String,
        pub sender_phone: String,
        pub sender_commerce_number: String,
        pub sender_vat_number: String,
        pub recipient_name: String,
        pub recipient_company_name: String,
        pub recipient_street: String,
        pub recipient_postal_city: String,
        pub send_date: String,
        pub project_title: String,
        pub invoice_number: String,
        pub due_date: String,
        #[PdfTableData]
        pub project_tasks: Vec<InvoiceTableData>,
        pub remarks: String,
        pub currency_symbol: String,
        pub total_before_vat: String,
        pub discount: String,
        pub vat_percentage: String,
        pub vat_amount: String,
        pub total_after_vat: String,
    }
}

mod invoice_maintenance {
    use serde::Serialize;
    use simple_pdf_generator_derive::PdfTemplate;
    use struct_field_names_as_array::FieldNamesAsArray;

    #[derive(Serialize, FieldNamesAsArray)]
    pub struct InvoiceMaintenanceTableData {
        pub description: String,
        pub months: i64,
        pub monthly_rate: String,
        pub total: String,
    }

    #[derive(PdfTemplate, FieldNamesAsArray)]
    pub struct InvoiceMaintenanceTemplate {
        pub sender_name: String,
        pub sender_company_name: String,
        pub sender_street: String,
        pub sender_postal_city: String,
        pub sender_phone: String,
        pub sender_commerce_number: String,
        pub sender_vat_number: String,
        pub recipient_name: String,
        pub recipient_company_name: String,
        pub recipient_street: String,
        pub recipient_postal_city: String,
        pub send_date: String,
        pub contract_type: String,
        pub invoice_number: String,
        pub due_date: String,
        #[PdfTableData]
        pub invoice_table: Vec<InvoiceMaintenanceTableData>,
        pub remarks: String,
        pub currency_symbol: String,
        pub total_before_vat: String,
        pub discount: String,
        pub vat_percentage: String,
        pub vat_amount: String,
        pub total_after_vat: String,
    }
}

enum PdfData<'a> {
    Quote(&'a self::quote::QuoteTemplate),
    Invoice(&'a self::invoice::InvoiceTemplate),
    InvoiceMaintenance(&'a self::invoice_maintenance::InvoiceMaintenanceTemplate),
}

struct PdfArgs<'a> {
    template: String,
    data: PdfData<'a>,
}

trait EnsureDir {
    fn ensure_dir(&self) -> Result<&Self>;
}

impl EnsureDir for std::path::PathBuf {
    fn ensure_dir(&self) -> Result<&Self> {
        if self.is_file() {
            return Err(anyhow::anyhow!("Path is a file"));
        }

        if !self.is_dir() {
            std::fs::create_dir_all(self)?;
        }

        Ok(self)
    }
}

trait EnsureFile {
    fn ensure_file(&self, contents: Option<&[u8]>) -> Result<&Self>;
}

impl EnsureFile for std::path::PathBuf {
    fn ensure_file(&self, contents: Option<&[u8]>) -> Result<&Self> {
        if self.is_dir() {
            return Err(anyhow::anyhow!("Path is a directory"));
        }

        if !self.is_file() {
            let mut file = std::fs::File::create(self)?;
            if let Some(contents) = contents {
                file.write_all(contents)?;
            }
        }

        Ok(self)
    }
}

macro_rules! get_env_or_home_dir {
    ($env_var:expr, $default:expr) => {
        match std::env::var($env_var) {
            Ok(dir) => {
                let pathbuf = std::path::PathBuf::from_str(&dir)?;
                pathbuf.ensure_dir()?;
                pathbuf
            }
            Err(_) => {
                if let Some(home) = home::home_dir() {
                    let mut pathbuf = home.clone();
                    pathbuf.push(".ccli");
                    pathbuf.push($default);
                    pathbuf.ensure_dir()?;
                    pathbuf
                } else {
                    return Err(anyhow::anyhow!(
                        "{} not found and failed to get home directory",
                        $env_var
                    ));
                }
            }
        }
    };
}

impl PdfData<'_> {
    fn get_template_name(&self) -> String {
        match self {
            PdfData::Quote(_) => "quote".to_string(),
            PdfData::Invoice(_) => "invoice".to_string(),
            PdfData::InvoiceMaintenance(_) => "invoice_maintenance".to_string(),
        }
    }

    fn get_default_html_contents(&self) -> String {
        match self {
            PdfData::Quote(_) => {
                // Get QuoteTemplate struct field names as a vec of Strings
                self::quote::QuoteTemplate::FIELD_NAMES_AS_ARRAY
                    .iter()
                    .fold("".to_string(), |acc, name| format!("{acc}%%{name}%%\n"))
            }
            PdfData::Invoice(_) => self::invoice::InvoiceTemplate::FIELD_NAMES_AS_ARRAY
                .iter()
                .fold("".to_string(), |acc, name| format!("{acc}%%{name}%%\n")),
            PdfData::InvoiceMaintenance(_) => {
                self::invoice_maintenance::InvoiceMaintenanceTemplate::FIELD_NAMES_AS_ARRAY
                    .iter()
                    .fold("".to_string(), |acc, name| format!("{acc}%%{name}%%\n"))
            }
        }
    }

    async fn export_pdf(
        &self,
        html_path: std::path::PathBuf,
        assets: &[simple_pdf_generator::Asset],
        print_options: &simple_pdf_generator::PrintOptions,
    ) -> Result<String> {
        let output_dir = get_env_or_home_dir!("CCLI_OUTPUT_DIR", "pdfs");

        match self {
            PdfData::Quote(quote_template) => {
                let pdf_buf = quote_template
                    .generate_pdf(html_path, assets, print_options)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to generate pdf: {}", e))?;
                let pdf_path = format!(
                    "{}/offerte-{}-{}.pdf",
                    output_dir.to_path_buf().display(),
                    quote_template.project_title,
                    quote_template.send_date
                );
                tokio::fs::write(&pdf_path, pdf_buf)
                    .await
                    .expect("Failed to write pdf file");
                Ok(pdf_path)
            }
            PdfData::Invoice(invoice_template) => {
                let pdf_buf = invoice_template
                    .generate_pdf(html_path, assets, print_options)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to generate pdf: {}", e))?;
                let pdf_path = format!(
                    "{}/factuur-{}-{}.pdf",
                    output_dir.to_path_buf().display(),
                    invoice_template.project_title,
                    invoice_template.send_date
                );
                tokio::fs::write(&pdf_path, pdf_buf)
                    .await
                    .expect("Failed to write pdf file");
                Ok(pdf_path)
            }
            PdfData::InvoiceMaintenance(invoice_maintenance_template) => {
                let pdf_buf = invoice_maintenance_template
                    .generate_pdf(html_path, assets, print_options)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to generate pdf: {}", e))?;
                let pdf_path = format!(
                    "{}/factuur-{}-{}.pdf",
                    output_dir.to_path_buf().display(),
                    invoice_maintenance_template.contract_type,
                    invoice_maintenance_template.send_date
                );
                tokio::fs::write(&pdf_path, pdf_buf)
                    .await
                    .expect("Failed to write pdf file");
                Ok(pdf_path)
            }
        }
    }
}

// Generate pdf with simple_pdf_generator
async fn generate_pdf(pdf_args: &PdfArgs<'_>) -> Result<String> {
    let template_dir = get_env_or_home_dir!("CCLI_TEMPLATE_DIR", "templates");

    let template_path = format!("{}/{}", template_dir.as_path().display(), pdf_args.template);
    let html_path = std::path::PathBuf::from(format!("{}.html", template_path));
    let shared_path =
        std::path::PathBuf::from(format!("{}/shared.css", template_dir.as_path().display()));
    let style_path = std::path::PathBuf::from(format!("{}.css", template_path));

    let default_html_contents: String = pdf_args.data.get_default_html_contents();

    html_path.ensure_file(Some(default_html_contents.as_bytes()))?;
    shared_path.ensure_file(None)?;
    style_path.ensure_file(None)?;

    let assets = [
        Asset {
            path: shared_path,
            r#type: AssetType::Style,
        },
        Asset {
            path: style_path,
            r#type: AssetType::Style,
        },
    ];

    let print_options = PrintOptions {
        paper_width: Some(210.0),  // A4 paper size in mm
        paper_height: Some(297.0), // A4 paper size in mm
        margin_top: Some(10.0),    // 10mm margin
        margin_bottom: Some(10.0), // 10mm margin
        margin_left: Some(10.0),   // 10mm margin
        margin_right: Some(10.0),  // 10mm margin
        ..PrintOptions::default()
    };

    pdf_args
        .data
        .export_pdf(html_path, &assets, &print_options)
        .await
}

pub async fn make_quote(db: &SqlitePool, quote_args: &QuoteMakeArgs) -> Result<String> {
    let project = sqlx::query_as!(
        Project,
        r#"SELECT * FROM projects WHERE id = ?"#,
        quote_args.project_id
    )
    .fetch_one(db)
    .await?;

    let project_tasks = sqlx::query_as!(
        ProjectTask,
        r#"SELECT * FROM tasks WHERE project_id = ?"#,
        quote_args.project_id
    )
    .fetch_all(db)
    .await?;

    let vat_percentage = quote_args.vat_percentage.unwrap_or(21);
    let currency = quote_args.currency.clone().unwrap_or("EUR".to_string());
    let total_before_vat = project_tasks.iter().fold(0, |acc, task| {
        acc + task.minutes_estimated.unwrap_or(0) * task.minute_rate.unwrap_or(0)
    });
    let total_after_vat =
        (total_before_vat - quote_args.discount.unwrap_or(0)) * (100 + vat_percentage);
    let project_duration = format!(
        "{} - {}",
        project
            .start_date
            .expect("No project.start_date")
            .format("%d-%m-%Y"),
        project
            .end_date
            .expect("No project.end_date")
            .format("%d-%m-%Y")
    );

    let quote = QuoteCreateArgs {
        sender_id: 1,
        recipient_id: project.client_id,
        project_id: Some(quote_args.project_id),
        project_duration: Some(project_duration),
        remarks: quote_args.remarks.clone(),
        total_before_vat: Some(total_before_vat),
        discount: quote_args.discount,
        vat_percentage: Some(vat_percentage),
        currency: Some(currency.clone()),
        total_after_vat: Some(total_after_vat),
        quote_url: None,
    };

    let sender_account = sqlx::query_as!(
        Account,
        r#"SELECT * FROM accounts WHERE id = ?"#,
        quote.sender_id
    )
    .fetch_one(db)
    .await?;

    let sender = sqlx::query_as!(
        Company,
        r#"SELECT * FROM companies WHERE id = ?"#,
        sender_account.company_id
    )
    .fetch_one(db)
    .await?;

    let sender_address = sqlx::query_as!(
        Address,
        r#"SELECT * FROM address WHERE id = ?"#,
        sender.address_id
    )
    .fetch_one(db)
    .await?;

    let recipient = sqlx::query_as!(
        Account,
        r#"SELECT * FROM accounts WHERE id = ?"#,
        project.client_id
    )
    .fetch_one(db)
    .await?;

    let mut recipient_address: Option<Address> = None;
    let mut recipient_company_name: Option<String> = None;

    if recipient.company_id.is_some() {
        let recipient_company = sqlx::query_as!(
            Company,
            r#"SELECT * FROM companies WHERE id = ?"#,
            recipient.company_id
        )
        .fetch_one(db)
        .await?;

        recipient_company_name = Some(recipient_company.name.clone());

        let result = sqlx::query_as!(
            Address,
            r#"SELECT * FROM address WHERE id = ?"#,
            recipient_company.address_id
        )
        .fetch_one(db)
        .await;
        recipient_address = match result {
            Ok(address) => Some(address),
            Err(_) => None,
        };
    }

    if recipient.address_id.is_some() && recipient_address.is_none() {
        let result = sqlx::query_as!(
            Address,
            r#"SELECT * FROM address WHERE id = ?"#,
            recipient.address_id
        )
        .fetch_one(db)
        .await;
        recipient_address = match result {
            Ok(address) => Some(address),
            Err(_) => None,
        };
    }

    let mut quote_table = Vec::new();

    for project_task in project_tasks {
        quote_table.push(self::quote::QuoteTableData {
            title: project_task.title.clone(),
            description: project_task.description.clone().unwrap_or("".to_string()),
            hours_estimated: project_task.minutes_estimated.unwrap_or(0) as f64 / 60.0,
            hourly_rate: format!(
                "{:.2}",
                (project_task.minute_rate.unwrap_or(0) * 60) as f64 / 100.0
            ),
            total: format!(
                "{:.2}",
                (project_task.minutes_estimated.unwrap_or(0)
                    * project_task.minute_rate.unwrap_or(0)) as f64
                    / 100.0
            ),
        });
    }

    let (recipient_street, recipient_postal_city) = match recipient_address {
        Some(address) => (
            format!(
                "{} {} {}",
                address.street.clone().unwrap_or("".to_string()),
                address.number.clone().unwrap_or("".to_string()),
                address.unit.clone().unwrap_or("".to_string()),
            ),
            format!(
                "{} {}",
                address.postalcode.clone().unwrap_or("".to_string()),
                address.city.clone().unwrap_or("".to_string()),
            ),
        ),
        None => ("".to_string(), "".to_string()),
    };

    let vat_amount = quote.total_after_vat.unwrap_or(0)
        - ((quote.total_before_vat.unwrap_or(0) - quote.discount.unwrap_or(0)) * 100);

    let quote_template = self::quote::QuoteTemplate {
        sender_name: sender.name.clone(),
        sender_street: format!(
            "{} {} {}",
            sender_address.street.clone().unwrap_or("".to_string()),
            sender_address.number.clone().unwrap_or("".to_string()),
            sender_address.unit.clone().unwrap_or("".to_string()),
        ),
        sender_postal_city: format!(
            "{} {}",
            sender_address.postalcode.clone().unwrap_or("".to_string()),
            sender_address.city.clone().unwrap_or("".to_string()),
        ),
        sender_phone: sender.phone.clone().unwrap_or("".to_string()),
        sender_email: sender.email.clone().unwrap_or("".to_string()),
        sender_commerce_number: sender.commerce_number.clone().unwrap_or("".to_string()),
        recipient_name: recipient.name.clone().unwrap_or("".to_string()),
        recipient_company_name: recipient_company_name.clone().unwrap_or("".to_string()),
        recipient_street,
        recipient_postal_city,
        send_date: chrono::Local::now().format("%d-%m-%Y").to_string(),
        project_title: project.title.clone(),
        project_description: project.description.clone().unwrap_or("".to_string()),
        project_duration: quote.project_duration.clone().unwrap_or("".to_string()),
        project_tasks: quote_table,
        remarks: quote.remarks.clone().unwrap_or("".to_string()),
        currency_symbol: match quote.currency.clone().unwrap_or("EUR".to_string()).as_str() {
            "EUR" => "€".to_string(),
            "USD" => "$".to_string(),
            _ => "".to_string(),
        }
        .to_string(),
        total_before_vat: format!("{:.2}", quote.total_before_vat.unwrap_or(0) as f64 / 100.0),
        discount: format!("{:.2}", quote.discount.unwrap_or(0) as f64 / 100.0),
        vat_percentage: quote.vat_percentage.unwrap_or(21).to_string(),
        vat_amount: format!("{:.2}", vat_amount as f64 / 10000.0),
        total_after_vat: format!("{:.2}", quote.total_after_vat.unwrap_or(0) as f64 / 10000.0),
    };

    let pdf_args = PdfArgs {
        template: "quote".to_string(),
        data: PdfData::Quote(&quote_template),
    };

    let quote_url = generate_pdf(&pdf_args).await?;

    let result = sqlx::query!(
        r#"
INSERT INTO quotes (
    sender_id,
    recipient_id,
    project_id,
    project_duration,
    remarks,
    total_before_vat,
    discount,
    vat_percentage,
    currency,
    total_after_vat,
    quote_url
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
        quote.sender_id,
        quote.recipient_id,
        quote.project_id,
        quote.project_duration,
        quote.remarks,
        quote.total_before_vat,
        quote.discount,
        vat_percentage,
        currency,
        quote.total_after_vat,
        quote_url
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert quote"));
    }

    Ok(quote_url)
}

pub async fn get_invoice(db: &SqlitePool, id: i64) -> Result<Invoice> {
    sqlx::query_as!(Invoice, r#"SELECT * FROM invoices WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}

pub async fn add_invoice(db: &SqlitePool, invoice: &InvoiceCreateArgs) -> Result<i64> {
    let result = sqlx::query!(
        r#"
INSERT INTO invoices (
    sender_id,
    recipient_id,
    invoice_number,
    quote_id,
    payment_due_date,
    payment_date,
    contract_id,
    project_id,
    remarks,
    total_before_vat,
    discount,
    vat_percentage,
    currency,
    total_after_vat,
    invoice_url,
    payment_request_url
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
        invoice.sender_id,
        invoice.recipient_id,
        invoice.invoice_number,
        invoice.quote_id,
        invoice.payment_due_date,
        invoice.payment_date,
        invoice.contract_id,
        invoice.project_id,
        invoice.remarks,
        invoice.total_before_vat,
        invoice.discount,
        invoice.vat_percentage,
        invoice.currency,
        invoice.total_after_vat,
        invoice.invoice_url,
        invoice.payment_request_url
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert invoice"));
    }

    Ok(result.last_insert_rowid())
}

pub async fn make_invoice(db: &SqlitePool, invoice_args: &InvoiceMakeArgs) -> Result<String> {
    let mut sender_id = 1;
    let mut vat_percentage = 21;
    let mut discount = invoice_args.discount.unwrap_or(0);
    let mut currency = "EUR".to_string();
    let mut project_id: Option<i64> = None;

    if invoice_args.quote_id.is_some() && invoice_args.project_id.is_some() {
        return Err(anyhow::anyhow!(
            "Cannot make invoice from quote and project"
        ));
    }

    if invoice_args.quote_id.is_none() && invoice_args.project_id.is_none() {
        return Err(anyhow::anyhow!(
            "Cannot make invoice without quote or project"
        ));
    }

    if invoice_args.quote_id.is_some() {
        let quote = sqlx::query_as!(
            Quote,
            r#"SELECT * FROM quotes WHERE id = ?"#,
            invoice_args.quote_id
        )
        .fetch_one(db)
        .await?;

        sender_id = quote.sender_id;

        if quote.vat_percentage.is_some() {
            vat_percentage = quote.vat_percentage.unwrap();
        }

        if quote.discount.is_some() {
            discount += quote.discount.unwrap_or(0);
        }

        if quote.project_id.is_some() {
            project_id = quote.project_id;
        }

        currency = quote.currency;
    }

    if invoice_args.project_id.is_some() {
        project_id = invoice_args.project_id;
    }

    let project = sqlx::query_as!(
        Project,
        r#"SELECT * FROM projects WHERE id = ?"#,
        project_id
    )
    .fetch_one(db)
    .await?;

    let project_tasks = sqlx::query_as!(
        ProjectTask,
        r#"SELECT * FROM tasks WHERE project_id = ?"#,
        project_id
    )
    .fetch_all(db)
    .await?;

    let total_before_vat = project_tasks.iter().fold(0, |acc, task| {
        acc + task
            .minutes_spent
            .unwrap_or(task.minutes_estimated.unwrap_or(0))
            * task.minute_rate.unwrap_or(0)
    });
    let total_after_vat = (total_before_vat - discount) * (100 + vat_percentage);

    let invoice = InvoiceCreateArgs {
        sender_id,
        recipient_id: project.client_id,
        invoice_number: format!("{}{:05}", chrono::Utc::now().year(), project.id),
        send_date: Some(format!("{}", chrono::Utc::now().format("%d-%m-%Y"))),
        quote_id: invoice_args.quote_id,
        payment_due_date: Some(format!(
            "{}",
            chrono::Utc::now()
                .checked_add_months(Months::new(1))
                .unwrap()
                .format("%d-%m-%Y")
        )),
        payment_date: None,
        contract_id: invoice_args.contract_id,
        project_id,
        remarks: invoice_args.remarks.clone(),
        total_before_vat: Some(total_before_vat),
        discount: Some(discount),
        currency: Some(currency),
        vat_percentage: Some(vat_percentage),
        total_after_vat: Some(total_after_vat),
        invoice_url: None,
        payment_request_url: None,
    };

    let sender_account = sqlx::query_as!(
        Account,
        r#"SELECT * FROM accounts WHERE id = ?"#,
        invoice.sender_id
    )
    .fetch_one(db)
    .await?;

    let sender = sqlx::query_as!(
        Company,
        r#"SELECT * FROM companies WHERE id = ?"#,
        sender_account.company_id
    )
    .fetch_one(db)
    .await?;

    let sender_address = sqlx::query_as!(
        Address,
        r#"SELECT * FROM address WHERE id = ?"#,
        sender.address_id
    )
    .fetch_one(db)
    .await?;

    let recipient = sqlx::query_as!(
        Account,
        r#"SELECT * FROM accounts WHERE id = ?"#,
        project.client_id
    )
    .fetch_one(db)
    .await?;

    let mut recipient_address: Option<Address> = None;
    let mut recipient_company_name: Option<String> = None;

    if recipient.company_id.is_some() {
        let recipient_company = sqlx::query_as!(
            Company,
            r#"SELECT * FROM companies WHERE id = ?"#,
            recipient.company_id
        )
        .fetch_one(db)
        .await?;

        recipient_company_name = Some(recipient_company.name.clone());

        let result = sqlx::query_as!(
            Address,
            r#"SELECT * FROM address WHERE id = ?"#,
            recipient_company.address_id
        )
        .fetch_one(db)
        .await;

        recipient_address = match result {
            Ok(address) => Some(address),
            Err(_) => None,
        };
    }

    if recipient.address_id.is_some() && recipient_address.is_none() {
        let result = sqlx::query_as!(
            Address,
            r#"SELECT * FROM address WHERE id = ?"#,
            recipient.address_id
        )
        .fetch_one(db)
        .await;

        recipient_address = match result {
            Ok(address) => Some(address),
            Err(_) => None,
        };
    }

    let (recipient_street, recipient_postal_city) = match recipient_address {
        Some(address) => (
            format!(
                "{} {} {}",
                address.street.clone().unwrap_or("".to_string()),
                address.number.clone().unwrap_or("".to_string()),
                address.unit.clone().unwrap_or("".to_string()),
            ),
            format!(
                "{} {}",
                address.postalcode.clone().unwrap_or("".to_string()),
                address.city.clone().unwrap_or("".to_string()),
            ),
        ),
        None => ("".to_string(), "".to_string()),
    };

    let vat_amount = invoice.total_after_vat.unwrap_or(0)
        - ((invoice.total_before_vat.unwrap_or(0) - invoice.discount.unwrap_or(0)) * 100);

    let invoice_url = if invoice_args.contract_id.is_some() {
        let mut invoice_table = Vec::new();

        let contract = sqlx::query_as!(
            Contract,
            r#"SELECT * FROM contracts WHERE id = ?"#,
            invoice_args.contract_id
        )
        .fetch_one(db)
        .await?;

        let last_contract_invoice = sqlx::query_as!(
            Invoice,
            r#"SELECT * FROM invoices WHERE contract_id = ? ORDER BY id DESC LIMIT 1"#,
            invoice_args.contract_id
        )
        .fetch_one(db)
        .await?;

        let last_invoiced_date = last_contract_invoice.payment_date.unwrap_or(
            last_contract_invoice
                .send_date
                .unwrap_or(contract.start_date.unwrap()),
        );

        let duration = chrono::Utc::now().signed_duration_since(last_invoiced_date.and_utc());
        let total_months = duration.num_weeks() * 52 / 12;

        for i in (0..=total_months).step_by(contract.invoice_period_months.unwrap_or(1) as usize) {
            let total =
                contract.monthly_rate.unwrap_or(0) * contract.invoice_period_months.unwrap_or(1);
            let first_month = last_invoiced_date
                .checked_add_signed(chrono::Duration::weeks(i))
                .unwrap();
            let last_month = first_month
                .checked_add_signed(chrono::Duration::weeks(
                    contract.invoice_period_months.unwrap_or(1),
                ))
                .unwrap();
            let is_new_year = first_month.year() != last_month.year();
            let description = if is_new_year {
                format!(
                    "{} {} - {} {}",
                    first_month.format("%B"),
                    first_month.year(),
                    last_month.format("%B"),
                    last_month.year()
                )
            } else {
                format!("{} - {}", first_month.format("%B"), last_month.format("%B"))
            };

            invoice_table.push(self::invoice_maintenance::InvoiceMaintenanceTableData {
                description,
                months: contract.invoice_period_months.unwrap_or(1),
                monthly_rate: format!("{:.2}", contract.monthly_rate.unwrap_or(0) as f64 / 100.0),
                total: format!("{:.2}", total as f64 / 100.0),
            });
        }

        let invoice_template = self::invoice_maintenance::InvoiceMaintenanceTemplate {
            sender_name: sender_account.name.clone().unwrap_or("".to_string()),
            sender_company_name: sender.name.clone(),
            sender_street: format!(
                "{} {} {}",
                sender_address.street.clone().unwrap_or("".to_string()),
                sender_address.number.clone().unwrap_or("".to_string()),
                sender_address.unit.clone().unwrap_or("".to_string()),
            ),
            sender_postal_city: format!(
                "{} {}",
                sender_address.postalcode.clone().unwrap_or("".to_string()),
                sender_address.city.clone().unwrap_or("".to_string()),
            ),
            sender_phone: sender.phone.clone().unwrap_or("".to_string()),
            sender_commerce_number: sender.commerce_number.clone().unwrap_or("".to_string()),
            sender_vat_number: sender.vat_number.clone().unwrap_or("".to_string()),
            recipient_name: recipient.name.clone().unwrap_or("".to_string()),
            recipient_company_name: recipient_company_name.clone().unwrap_or("".to_string()),
            recipient_street,
            recipient_postal_city,
            send_date: chrono::Local::now().format("%d-%m-%Y").to_string(),
            contract_type: contract.contract_type.clone().unwrap_or("".to_string()),
            invoice_number: invoice.invoice_number.clone(),
            due_date: invoice.payment_due_date.clone().unwrap_or("".to_string()),
            invoice_table,
            remarks: invoice.remarks.clone().unwrap_or("".to_string()),
            currency_symbol: match invoice
                .currency
                .clone()
                .unwrap_or("EUR".to_string())
                .as_str()
            {
                "EUR" => "€".to_string(),
                "USD" => "$".to_string(),
                _ => "".to_string(),
            }
            .to_string(),
            total_before_vat: format!(
                "{:.2}",
                invoice.total_before_vat.unwrap_or(0) as f64 / 100.0
            ),
            discount: format!("{:.2}", invoice.discount.unwrap_or(0) as f64 / 100.0),
            vat_percentage: invoice.vat_percentage.unwrap_or(21).to_string(),
            vat_amount: format!("{:.2}", vat_amount as f64 / 10000.0),
            total_after_vat: format!(
                "{:.2}",
                invoice.total_after_vat.unwrap_or(0) as f64 / 10000.0
            ),
        };

        let pdf_args = PdfArgs {
            template: "invoice-contract".to_string(),
            data: PdfData::InvoiceMaintenance(&invoice_template),
        };

        generate_pdf(&pdf_args).await?
    } else {
        let mut invoice_table = Vec::new();

        for project_task in project_tasks {
            invoice_table.push(self::invoice::InvoiceTableData {
                title: project_task.title.clone(),
                description: project_task.description.clone().unwrap_or("".to_string()),
                hours_spent: project_task
                    .minutes_spent
                    .unwrap_or(project_task.minutes_estimated.unwrap_or(0))
                    as f64
                    / 60.0,
                hourly_rate: format!(
                    "{:.2}",
                    (project_task.minute_rate.unwrap_or(0) * 60) as f64 / 100.0
                ),
                total: format!(
                    "{:.2}",
                    (project_task
                        .minutes_spent
                        .unwrap_or(project_task.minutes_estimated.unwrap_or(0))
                        * project_task.minute_rate.unwrap_or(0)) as f64
                        / 100.0
                ),
            });
        }

        let invoice_template = self::invoice::InvoiceTemplate {
            sender_name: sender_account.name.clone().unwrap_or("".to_string()),
            sender_company_name: sender.name.clone(),
            sender_street: format!(
                "{} {} {}",
                sender_address.street.clone().unwrap_or("".to_string()),
                sender_address.number.clone().unwrap_or("".to_string()),
                sender_address.unit.clone().unwrap_or("".to_string()),
            ),
            sender_postal_city: format!(
                "{} {}",
                sender_address.postalcode.clone().unwrap_or("".to_string()),
                sender_address.city.clone().unwrap_or("".to_string()),
            ),
            sender_phone: sender.phone.clone().unwrap_or("".to_string()),
            sender_commerce_number: sender.commerce_number.clone().unwrap_or("".to_string()),
            sender_vat_number: sender.vat_number.clone().unwrap_or("".to_string()),
            recipient_name: recipient.name.clone().unwrap_or("".to_string()),
            recipient_company_name: recipient_company_name.clone().unwrap_or("".to_string()),
            recipient_street,
            recipient_postal_city,
            send_date: chrono::Local::now().format("%d-%m-%Y").to_string(),
            project_title: project.title.clone(),
            invoice_number: invoice.invoice_number.clone(),
            due_date: invoice.payment_due_date.clone().unwrap_or("".to_string()),
            project_tasks: invoice_table,
            remarks: invoice.remarks.clone().unwrap_or("".to_string()),
            currency_symbol: match invoice
                .currency
                .clone()
                .unwrap_or("EUR".to_string())
                .as_str()
            {
                "EUR" => "€".to_string(),
                "USD" => "$".to_string(),
                _ => "".to_string(),
            }
            .to_string(),
            total_before_vat: format!(
                "{:.2}",
                invoice.total_before_vat.unwrap_or(0) as f64 / 100.0
            ),
            discount: format!("{:.2}", invoice.discount.unwrap_or(0) as f64 / 100.0),
            vat_percentage: invoice.vat_percentage.unwrap_or(21).to_string(),
            vat_amount: format!("{:.2}", vat_amount as f64 / 10000.0),
            total_after_vat: format!(
                "{:.2}",
                invoice.total_after_vat.unwrap_or(0) as f64 / 10000.0
            ),
        };

        let pdf_args = PdfArgs {
            template: "invoice-project".to_string(),
            data: PdfData::Invoice(&invoice_template),
        };

        generate_pdf(&pdf_args).await?
    };

    let result = sqlx::query!(
        r#"
INSERT INTO invoices (
    sender_id,
    recipient_id,
    invoice_number,
    quote_id,
    payment_due_date,
    payment_date,
    contract_id,
    project_id,
    remarks,
    total_before_vat,
    discount,
    vat_percentage,
    currency,
    total_after_vat,
    invoice_url,
    payment_request_url
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
        invoice.sender_id,
        invoice.recipient_id,
        invoice.invoice_number,
        invoice.quote_id,
        invoice.payment_due_date,
        invoice.payment_date,
        invoice.contract_id,
        invoice.project_id,
        invoice.remarks,
        invoice.total_before_vat,
        invoice.discount,
        invoice.vat_percentage,
        invoice.currency,
        invoice.total_after_vat,
        invoice_url,
        invoice.payment_request_url
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Failed to insert invoice"));
    }

    Ok(invoice_url)
}

pub async fn get_schedule(db: &SqlitePool, id: i64) -> Result<Schedule> {
    sqlx::query_as!(Schedule, r#"SELECT * FROM schedule WHERE id = ?"#, id)
        .fetch_one(db)
        .await
        .map_err(anyhow::Error::msg)
}
