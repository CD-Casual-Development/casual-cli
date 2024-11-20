use std::any::type_name_of_val;
use std::fmt::Debug;
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use serde::Serialize;

use crate::clapargs::*;
use crate::models::*;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum PrintMode {
    Normal,
    Value,
    Html,
    Json,
}

pub struct Logger {
    mode: PrintMode,
}

pub struct Jchar(std::primitive::char);

impl Jchar {
    pub fn new(value: std::primitive::char) -> Self {
        Self(value)
    }
}
impl From<std::primitive::char> for Jchar {
    fn from(value: std::primitive::char) -> Self {
        Self::new(value)
    }
}

impl Serialize for Jchar {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_char(self.0)
    }
}

impl Debug for Jchar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait ToHtml {
    fn to_html(&self) -> String {
        unimplemented!();
    }
}

impl ToHtml for String {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for i64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for &i64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for i32 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for &i32 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for f64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for &f64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for bool {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for &bool {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for u64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for &u64 {
    fn to_html(&self) -> String {
        return format!("<span>{}</span>", self);
    }
}

impl ToHtml for Jchar {
    fn to_html(&self) -> String {
        return format!("<span>{:?}</span>", self);
    }
}

impl ToHtml for Address {
    fn to_html(&self) -> String {
        let id = self.id;
        let country = self.country.clone().unwrap_or("".to_string());
        let city = self.city.clone().unwrap_or("".to_string());
        let street = self.street.clone().unwrap_or("".to_string());
        let number = self.number.clone().unwrap_or("".to_string());
        let unit = self.unit.clone().unwrap_or("".to_string());
        let postalcode = self.postalcode.clone().unwrap_or("".to_string());

        return format!(
            "<span data-id=\"{id}\">{country}, {city}, {street}, {number}, {unit}, {postalcode}</span>",
            id = id,
            country = country,
            city = city,
            street = street,
            number = number,
            unit = unit,
            postalcode = postalcode
        );
    }
}

impl ToHtml for Account {
    fn to_html(&self) -> String {
        let id = self.id;
        let name = self
            .name
            .clone()
            .ok_or("None")
            .expect("Couldn't parse account name");

        return format!("<span data-id=\"{id}\">{name}</span>");
    }
}

impl ToHtml for Company {
    fn to_html(&self) -> String {
        let id = self.id;
        let name = self.name.clone();

        return format!("<span data-id=\"{id}\">{name}</span>");
    }
}

impl ToHtml for Contract {
    fn to_html(&self) -> String {
        let id = self.id;
        let recipient_id = self.recipient_id;
        let sender_id = self.sender_id;
        let status: String = if self.cancel_date.is_some() {
            "Cancelled".to_string()
        } else if self.auto_renew.is_some_and(|x| x) {
            "Auto Renew".to_string()
        } else if self.end_date.is_some() && self.start_date.is_some() {
            let ed = self.end_date.unwrap();
            let sd = self.start_date.unwrap();
            
            if ed < chrono::Utc::now().naive_utc() {
                "Expired".to_string()
            } else if sd > chrono::Utc::now().naive_utc() {
                "Pending".to_string()
            } else {
                "Active".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\" data-sender-id=\"{sender_id}\">Contract status: {status}</span>");
    }
}

impl ToHtml for Project {
    fn to_html(&self) -> String {
        let id = self.id;
        let title = self.title.clone();

        return format!("<span data-id=\"{id}\">{title}</span>");
    }
}

impl ToHtml for ProjectTask {
    fn to_html(&self) -> String {
        let id = self.id;
        let title = self.title.clone();

        return format!("<span data-id=\"{id}\">{title}</span>");
    }
}

impl ToHtml for Quote {
    fn to_html(&self) -> String {
        let id = self.id;
        let recipient_id = self.recipient_id;
        let quote_url = self
            .quote_url
            .clone()
            .ok_or("#not-found")
            .expect("Unable to get quote_url");

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\">Quote: <a href=\"{quote_url}\" target=\"_blank\">{quote_url}</a></span>");
    }
}

impl ToHtml for Invoice {
    fn to_html(&self) -> String {
        let id = self.id;
        let recipient_id = self.recipient_id;
        let invoice_url = self
            .invoice_url
            .clone()
            .ok_or("#not-found")
            .expect("Unable to get quote_url");

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\">Invoice: <a href=\"{invoice_url}\" target=\"_blank\">{invoice_url}</a></span>");
    }
}

impl ToHtml for Schedule {
    fn to_html(&self) -> String {
        let id = self.id;
        let date = match self.date {
            Some(d) => format!("{}", d.format("%d-%m-%Y")),
            None => "never".to_string(),
        };
        let interval = self.interval.clone().unwrap_or("".to_string());

        return format!("<span data-id=\"{id}\"{}{}{}{}>Scheduled {date} every {interval}</span>",
            self.contract_id.map_or("".to_string(), |x| format!(" data-contract-id=\"{}\"", x)),
            self.project_id.map_or("".to_string(), |x| format!(" data-project-id=\"{}\"", x)),
            self.invoice_id.map_or("".to_string(), |x| format!(" data-invoice-id=\"{}\"", x)),
            self.quote_id.map_or("".to_string(), |x| format!(" data-quote-id=\"{}\"", x))
        );
    }
}

impl ToHtml for FinanceReport {
    fn to_html(&self) -> String {
        let id = self.id;
        let from_date = match self.from_date {
            Some(d) => format!("{}", d.format("%d-%m-%Y")),
            None => "never".to_string(),
        };
        let to_date = match self.to_date {
            Some(d) => format!("{}", d.format("%d-%m-%Y")),
            None => "never".to_string(),
        };

        return format!("<span data-id=\"{id}\"{}{}{}>Report: {from_date} to {to_date}</span>",
            self.account_id.map_or("".to_string(), |x| format!(" data-account-id=\"{}\"", x)),
            self.company_id.map_or("".to_string(), |x| format!(" data-company-id=\"{}\"", x)),
            self.query_id.map_or("".to_string(), |x| format!(" data-query-id=\"{}\"", x))
        );
    }
}

impl ToHtml for FinanceQuery {
    fn to_html(&self) -> String {
        let id = self.id;
        let range = self.range.clone().unwrap_or("No range".to_string());
        
        return format!("<span data-id=\"{id}\"{}{}>Query: {range}</span>",
            self.account_id.map_or("".to_string(), |x| format!(" data-account-id=\"{}\"", x)),
            self.company_id.map_or("".to_string(), |x| format!(" data-company-id=\"{}\"", x))
        );
    }
}

impl ToHtml for Vec<String> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for string in self {
            html.push_str(&string.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Account> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for account in self {
            html.push_str(&account.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Company> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for company in self {
            html.push_str(&company.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Address> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for address in self {
            html.push_str(&address.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Contract> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for contract in self {
            html.push_str(&contract.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Project> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for project in self {
            html.push_str(&project.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<ProjectTask> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for task in self {
            html.push_str(&task.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Quote> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for quote in self {
            html.push_str(&quote.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Invoice> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for invoice in self {
            html.push_str(&invoice.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<Schedule> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for schedule in self {
            html.push_str(&schedule.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<FinanceReport> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for report in self {
            html.push_str(&report.to_html());
        }
        return html;
    }
}

impl ToHtml for Vec<FinanceQuery> {
    fn to_html(&self) -> String {
        let mut html = String::new();
        for query in self {
            html.push_str(&query.to_html());
        }
        return html;
    }
}

impl Logger {
    pub fn new(mode: PrintMode) -> Self {
        Self { mode }
    }

    pub fn print<T: Debug + Serialize + ToHtml>(&self, msg: String, value: T, new_line: bool) {
        match self.mode {
            PrintMode::Normal => {
                if new_line {
                    println!("{} {:?}", msg, value);
                } else {
                    print!("{} {:?}", msg, value);
                }
            }
            PrintMode::Value => {
                if new_line {
                    println!("{:?}", value);
                } else {
                    print!("{:?}", value);
                }
            }
            PrintMode::Html => {
                if new_line {
                    println!("{}", value.to_html());
                } else {
                    print!("{}", value.to_html());
                }
            }
            PrintMode::Json => {
                if new_line {
                    println!(
                        "{}",
                        serde_json::to_string(&value).expect("Couldn't serialize value to json")
                    );
                } else {
                    // Careful with this one, it will print the json string without quotes
                    // The type_name_of_val function lists the whole crate path of the type making it hard to maintain
                    if type_name_of_val(&value) == "casual_cli_lib::commands::Jchar" {
                        print!("{:?}", value);
                    } else {
                        print!(
                            "{}",
                            serde_json::to_string(&value)
                                .expect("Couldn't serialize value to json")
                        );
                    }
                }
            }
        }
    }

    pub fn msg(&self, msg: String) {
        match self.mode {
            PrintMode::Normal => println!("{}", msg),
            PrintMode::Value => (),
            PrintMode::Html => (),
            PrintMode::Json => (),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// The `account` program
    Account {
        #[command(subcommand)]
        subcmd: Option<Box<AccountCommands>>,
    },
    /// The `project` program
    Project {
        #[command(subcommand)]
        subcmd: Option<ProjectCommands>,
    },
    /// The `schedule` program
    Schedule {
        #[command(subcommand)]
        subcmd: Option<ScheduleCommands>,
    },
    /// The `finance` program
    Finance {
        #[command(subcommand)]
        subcmd: Option<FinanceCommands>,
    },
}

#[derive(Subcommand, Debug)]
pub enum AccountCommands {
    /// Add an account
    Get {
        id: i64,
    },
    GetCompany {
        id: i64,
    },
    GetAddress {
        id: i64,
    },
    GetContract {
        id: i64,
    },
    Add {
        /// The account data
        #[command(flatten)]
        account: Box<AccountCreateArgs>,
    },
    AddCompany {
        /// The company data
        #[command(flatten)]
        company: Box<CompanyCreateArgs>,
    },
    AddContract {
        /// The contract data
        #[command(flatten)]
        contract: Box<ContractCreateArgs>,
    },
    Update {
        id: i64,
        /// The account data
        #[command(flatten)]
        account: Box<AccountUpdateArgs>,
    },
    UpdateCompany {
        id: i64,
        /// The company data
        #[command(flatten)]
        company: Box<CompanyUpdateArgs>,
    },
    UpdateAddress {
        id: i64,
        /// The address data
        #[command(flatten)]
        address: Box<AddressUpdateArgs>,
    },
    UpdateContract {
        id: i64,
        /// The contract data
        #[command(flatten)]
        contract: Box<ContractUpdateArgs>,
    },
    /// Remove an account
    Remove {
        /// The account name
        id: i64,
    },
    RemoveCompany {
        /// The company name
        id: i64,
    },
    RemoveAddress {
        /// The address id
        id: i64,
    },
    RemoveContract {
        /// The contract id
        id: i64,
    },
    /// List all accounts (alias: `ls`)
    #[command(alias = "ls")]
    List {
        #[arg(short, long)]
        company_id: Option<i64>,
    },
    ListCompanies,
    ListAddresses,
    ListContracts {
        #[arg(short, long)]
        recipient_id: Option<i64>,
        #[arg(short, long)]
        sender_id: Option<i64>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    Get {
        id: i64,
        #[arg(short, long, default_value_t = false)]
        client: bool,
    },
    /// Add a project
    Add {
        /// The project data
        #[command(flatten)]
        project: Box<ProjectCreateArgs>,
    },
    Update {
        id: i64,
        #[command(flatten)]
        project: Box<ProjectUpdateArgs>,
    },
    GetTask {
        id: i64,
    },
    AddTask {
        /// The project task data
        #[command(flatten)]
        project_task: Box<ProjectTaskCreateArgs>,
    },
    UpdateTask {
        id: i64,
        #[command(flatten)]
        project_task: Box<ProjectTaskUpdateArgs>,
    },
    CompleteTask {
        /// The project task id
        id: i64,
    },
    /// Remove a project
    Remove {
        /// The project id
        id: i64,
    },
    RemoveTask {
        /// The project task id
        id: i64,
    },
    RemoveQuote {
        /// The quote id
        id: i64,
    },
    RemoveInvoice {
        /// The invoice id
        id: i64,
    },
    GetQuote {
        id: i64,
    },
    MakeQuote {
        #[command(flatten)]
        args: Box<QuoteMakeArgs>,
    },
    UpdateQuote {
        id: i64,
        #[command(flatten)]
        args: Box<QuoteUpdateArgs>,
    },
    GetInvoice {
        id: i64,
    },
    UpdateInvoice {
        id: i64,
        #[command(flatten)]
        args: Box<InvoiceUpdateArgs>,
    },
    MakeInvoice {
        #[command(flatten)]
        args: Box<InvoiceMakeArgs>,
    },
    /// List all projects (alias: `ls`)
    #[command(alias = "ls")]
    List,
    ListTasks {
        /// The project id
        id: i64,
    },
    ListQuotes {
        /// Project id
        #[arg(short, long)]
        project_id: Option<i64>,
        #[arg(short, long)]
        recipient_id: Option<i64>
    },
    ListInvoices {
        /// Project id
        #[arg(short, long)]
        project_id: Option<i64>,
        #[arg(short, long)]
        recipient_id: Option<i64>,
        #[arg(short, long)]
        contract_id: Option<i64>,
        #[arg(short, long)]
        quote_id: Option<i64>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ScheduleCommands {
    Get {
        id: i64,
    },
    /// Create scheduled item
    /// Example: prepare an email with an invoice pdf every 3 months based on the maintenance contract
    /// command: $ casual-cli schedule add [contract_id] 10-11-2024 3m
    Add {
        #[command(flatten)]
        schedule: Box<ScheduleCreateArgs>,
    },
    /// Update scheduled item
    Update {
        id: i64,
        #[command(flatten)]
        schedule: Box<ScheduleUpdateArgs>,
    },
    /// Remove scheImport-Module $env:ChocolateyInstall\helpers\chocolateyProfile.psm1duled item
    Remove {
        id: i64,
    },
    /// List the schedule
    #[command(alias = "ls")]
    List,
}

#[derive(Subcommand, Debug)]
pub enum FinanceCommands {
    /// Create report
    Report {
        #[command(flatten)]
        report: Box<FinanceReportArgs>,
    },
    AddQuery {
        #[command(flatten)]
        query: Box<FinanceCreateQueryArgs>,
    },
    UpdateQuery {
        id: i64,
        #[command(flatten)]
        query: Box<FinanceUpdateQueryArgs>,
    },
    /// Remove an account
    Remove { id: i64 },
    RemoveQuery { id: i64 },
}

/// Command line tool for Casual Development
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(global = true, value_enum, short, long,required=false, default_value_t = PrintMode::Normal)]
    pub mode: PrintMode,
    /// The command you want to use
    #[command(subcommand)]
    pub command: Option<Commands>,
}