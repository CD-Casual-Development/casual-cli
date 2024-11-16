mod clapargs;
mod models;
mod queries;

use std::any::type_name_of_val;
use std::env;
use std::fmt::Debug;

use anyhow::Result;
use clap::ValueEnum;
use clap::{Parser, Subcommand};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::clapargs::*;
use crate::models::*;
use crate::queries::*;

macro_rules! log_list {
    ($log:expr, $mode:expr, $items:expr) => {
        let mut i = 1;
        if &$mode == &PrintMode::Json {
            $log.print("".to_string(), Jchar::from('['), false);
        }
        for item in $items {
            if i > 1 && &$mode == &PrintMode::Json {
                $log.print("".to_string(), Jchar::from(','), false);
            }
            $log.print(format!("{i}:"), item, &$mode != &PrintMode::Json);
            i += 1;
        }
        if &$mode == &PrintMode::Json {
            $log.print("".to_string(), Jchar::from(']'), false);
        }
    };
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum PrintMode {
    Normal,
    Value,
    Html,
    Json,
}

struct Logger {
    mode: PrintMode,
}

struct Jchar(std::primitive::char);

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

trait ToHtml {
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

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\" data-sender-id=\"{sender_id}\">Contract</span>");
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

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\">Quote: <a href=\"{quote_url}\">{quote_url}</a></span>");
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

        return format!("<span data-id=\"{id}\" data-recipient-id=\"{recipient_id}\">Invoice: <a href=\"{invoice_url}\">{invoice_url}</a></span>");
    }
}

impl ToHtml for Schedule {
    fn to_html(&self) -> String {
        let id = self.id;

        return format!("<span data-id=\"{id}\">Schedule</span>");
    }
}

impl Logger {
    fn new(mode: PrintMode) -> Self {
        Self { mode }
    }

    fn print<T: Debug + Serialize + ToHtml>(&self, msg: String, value: T, new_line: bool) {
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
                    if type_name_of_val(&value) == "casual_cli::Jchar" {
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

    fn msg(&self, msg: String) {
        match self.mode {
            PrintMode::Normal => println!("{}", msg),
            PrintMode::Value => (),
            PrintMode::Html => (),
            PrintMode::Json => (),
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
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
enum AccountCommands {
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
enum ProjectCommands {
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
enum ScheduleCommands {
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
enum FinanceCommands {
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
struct Args {
    #[arg(global = true, value_enum, short, long,required=false, default_value_t = PrintMode::Normal)]
    mode: PrintMode,
    /// The command you want to use
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv()?; //.expect("Failed to load .env file");
    let args = Args::parse();
    let mode = (&args).mode.clone();
    let log = Logger::new(mode.clone());

    let db_pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&db_pool).await?;

    match &args.command {
        Some(Commands::Account { subcmd }) => match subcmd {
            Some(account_commands) => {
                let cmds = account_commands.as_ref();
                match cmds {
                    AccountCommands::Get { id } => {
                        log.msg(format!("Getting account with id {}", id));
                        let account = get_account(&db_pool, *id).await?;
                        log.print(format!("Got account {id}"), account, true);
                    }
                    AccountCommands::GetCompany { id } => {
                        log.msg(format!("Getting company with id {}", id));
                        let company = get_company(&db_pool, *id).await?;
                        log.print(format!("Got company {id}"), company, true);
                    }
                    AccountCommands::GetContract { id } => {
                        log.msg(format!("Getting contract with id {}", id));
                        let contract = get_contract(&db_pool, *id).await?;
                        log.print(format!("Got contract {id}"), contract, true);
                    }
                    AccountCommands::Add { account } => {
                        log.msg(format!("Adding account {}", account.name));
                        let id = add_account(&db_pool, account).await?;
                        log.print("Account added with id".to_string(), id, true);
                    }
                    AccountCommands::AddCompany { company } => {
                        log.msg(format!("Adding company {}", company.name));
                        let id = add_company(&db_pool, company).await?;
                        log.print("Company added with id".to_string(), id, true);
                    }
                    AccountCommands::AddContract { contract } => {
                        log.msg(format!("Adding contract"));
                        let id = add_contract(&db_pool, contract).await?;
                        log.print("Contract added with id".to_string(), id, true);
                    }
                    AccountCommands::Update { id, account } => {
                        log.msg(format!("Updating account {}", id));
                        let updated = update_account(&db_pool, *id, account).await?;
                        if updated == 0 {
                            log.print(format!("Account {} not found", id), -1, true);
                        } else {
                            log.print(format!("Account {id} updated"), id, true);
                        }
                    }
                    AccountCommands::UpdateCompany { id, company } => {
                        log.msg(format!("Updating company {}", id));
                        let updated = update_company(&db_pool, *id, company).await?;
                        if updated == 0 {
                            log.print(format!("Company {} not found", id), -1, true);
                        } else {
                            log.print(format!("Company {id} updated"), id, true);
                        }
                    }
                    AccountCommands::UpdateAddress { id, address } => {
                        log.msg(format!("Updating address {}", id));
                        let updated = update_address(&db_pool, *id, address).await?;
                        if updated == 0 {
                            log.print(format!("Address {} not found", id), -1, true);
                        } else {
                            log.print(format!("Address {id} updated"), id, true);
                        }
                    }
                    AccountCommands::UpdateContract { id, contract } => {
                        log.msg(format!("Updating contract {}", id));
                        let updated = update_contract(&db_pool, *id, contract).await?;
                        if updated == 0 {
                            log.print(format!("Contract {} not found", id), -1, true);
                        } else {
                            log.print(format!("Contract {id} updated"), id, true);
                        }
                    }
                    AccountCommands::Remove { id } => {
                        log.msg(format!("Removing account {}", id));
                        if sqlx::query!(r#"DELETE FROM accounts WHERE id = ?"#, id)
                            .execute(&db_pool)
                            .await?
                            .rows_affected()
                            == 0
                        {
                            log.print(format!("Account {} not found", id), -1, true);
                        } else {
                            log.print(format!("Account {} removed", id), id, true);
                        }
                    }
                    AccountCommands::RemoveCompany { id } => {
                        log.msg(format!("Removing company {}", id));
                        if sqlx::query!(r#"DELETE FROM companies WHERE id = ?"#, id)
                            .execute(&db_pool)
                            .await?
                            .rows_affected()
                            == 0
                        {
                            log.print(format!("Company {} not found", id), -1, true);
                        } else {
                            log.print(format!("Company {} removed", id), id, true);
                        }
                    }
                    AccountCommands::RemoveContract { id } => {
                        log.msg(format!("Removing contract {}", id));
                        if sqlx::query!(r#"DELETE FROM contracts WHERE id = ?"#, id)
                            .execute(&db_pool)
                            .await?
                            .rows_affected()
                            == 0
                        {
                            log.print(format!("Contract {} not found", id), -1, true);
                        } else {
                            log.print(format!("Contract {} removed", id), id, true);
                        }
                    }
                    AccountCommands::GetAddress { id } => {
                        log.msg(format!("Getting address with id {}", id));
                        let address = sqlx::query_as!(Address, "SELECT * FROM address WHERE id = ?", id)
                            .fetch_one(&db_pool)
                            .await?;
                        log.print(format!("Got address {id}"), address, true);
                    }
                    AccountCommands::RemoveAddress { id } => {
                        log.msg(format!("Removing address {}", id));
                        if sqlx::query!(r#"DELETE FROM address WHERE id = ?"#, id)
                            .execute(&db_pool)
                            .await?
                            .rows_affected()
                            == 0
                        {
                            log.print(format!("Address {} not found", id), -1, true);
                        } else {
                            log.print(format!("Address {} removed", id), id, true);
                        }
                    }
                    AccountCommands::List { company_id } => {
                        log.msg("Listing all accounts".to_string());
                        log.msg("-------------------".to_string());
                        let accounts =  match company_id {
                            Some(id) => sqlx::query_as!(Account, "SELECT * FROM accounts WHERE company_id = ?", id).fetch_all(&db_pool).await?,
                            None => sqlx::query_as!(Account, "SELECT * FROM accounts")
                            .fetch_all(&db_pool)
                            .await?
                        };
                        log_list!(log, mode, accounts);
                    }
                    AccountCommands::ListCompanies => {
                        log.msg("Listing all companies".to_string());
                        log.msg("---------------------".to_string());
                        let companies = sqlx::query_as!(Company, "SELECT * FROM companies")
                            .fetch_all(&db_pool)
                            .await?;

                        log_list!(log, mode, companies);
                    }
                    AccountCommands::ListAddresses => {
                        log.msg("Listing all addresses".to_string());
                        log.msg("---------------------".to_string());
                        let addresses = sqlx::query_as!(Address, "SELECT * FROM address")
                            .fetch_all(&db_pool)
                            .await?;

                        log_list!(log, mode, addresses);
                    }
                    AccountCommands::ListContracts { recipient_id, sender_id } => {
                        log.msg("Listing all contracts".to_string());
                        log.msg("----------------------".to_string());
                        let contracts = match (recipient_id, sender_id) {
                            (Some(recipient_id), Some(sender_id)) => sqlx::query_as!(Contract, "SELECT * FROM contracts WHERE recipient_id = ? AND sender_id = ?", recipient_id, sender_id)
                                .fetch_all(&db_pool)
                                .await?,
                            (Some(recipient_id), None) => sqlx::query_as!(Contract, "SELECT * FROM contracts WHERE recipient_id = ?", recipient_id)
                                .fetch_all(&db_pool)
                                .await?,
                            (None, Some(sender_id)) => sqlx::query_as!(Contract, "SELECT * FROM contracts WHERE sender_id = ?", sender_id)
                                .fetch_all(&db_pool)
                                .await?,
                            (None, None) => sqlx::query_as!(Contract, "SELECT * FROM contracts")
                                .fetch_all(&db_pool)
                                .await?,
                        };

                        log_list!(log, mode, contracts);
                    }
                }
            }
            None => {
                log.msg("No subcommand was used".to_string());
            }
        },
        Some(Commands::Project { subcmd }) => match subcmd {
            Some(ProjectCommands::Get { id, client }) => {
                log.msg(format!("Getting project with id {}", id));
                let project = get_project(&db_pool, *id).await?;
                if *client {
                    let account = get_account(&db_pool, project.client_id).await?;
                    log.print(format!("Got project {id}"), account, true);
                } else {
                    log.print(format!("Got account {id}"), project, true);
                }
            }
            Some(ProjectCommands::Add { project }) => {
                log.msg(format!("Adding project {}", project.title));
                let id = add_project(&db_pool, project).await?;
                log.print("Project added with id".to_string(), id, true);
            }
            Some(ProjectCommands::Update { id, project }) => {
                log.msg(format!("Updating project {}", id));
                let updated = update_project(&db_pool, *id, project).await?;
                if updated == 0 {
                    log.print(format!("Project {} not found", id), -1, true);
                } else {
                    log.print(format!("Project {id} updated"), id, true);
                }
            }
            Some(ProjectCommands::GetTask { id }) => {
                log.msg(format!("Getting project task with id {}", id));
                let task = get_project_task(&db_pool, *id).await?;
                log.print(format!("Got task {id}"), task, true);
            }
            Some(ProjectCommands::AddTask { project_task }) => {
                log.msg(format!("Adding task {project_task:?}"));
                let id = add_project_task(&db_pool, project_task).await?;
                log.print("Task added with id".to_string(), id, true);
            }
            Some(ProjectCommands::UpdateTask { id, project_task }) => {
                log.msg(format!("Updating task {}", id));
                let updated = update_project_task(&db_pool, *id, project_task).await?;
                if updated == 0 {
                    log.print(format!("Task {} not found", id), -1, true);
                } else {
                    log.print(format!("Task {id} updated"), id, true);
                }
            }
            Some(ProjectCommands::CompleteTask { id }) => {
                log.msg(format!("Completing task {}", id));
                let task = sqlx::query!("UPDATE tasks SET is_completed = 1 WHERE id = ?", id)
                    .execute(&db_pool)
                    .await?;
                if task.rows_affected() == 0 {
                    log.print(format!("Task {} not found", id), -1, true);
                } else {
                    log.print(format!("Task {id} completed"), id, true);
                }
            }
            Some(ProjectCommands::Remove { id }) => {
                log.msg(format!("Removing project {}", id));
                if sqlx::query!(r#"DELETE FROM projects WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Project {} not found", id), -1, true);
                } else {
                    log.print(format!("Project {} removed", id), id, true);
                }
            }
            Some(ProjectCommands::RemoveTask { id }) => {
                log.msg(format!("Removing task {}", id));
                if sqlx::query!(r#"DELETE FROM tasks WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Task {} not found", id), -1, true);
                } else {
                    log.print(format!("Task {} removed", id), id, true);
                }
            }
            Some(ProjectCommands::RemoveQuote { id }) => {
                log.msg(format!("Removing quote {}", id));
                if sqlx::query!(r#"DELETE FROM quotes WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Quote {} not found", id), -1, true);
                } else {
                    log.print(format!("Quote {} removed", id), id, true);
                }
            }
            Some(ProjectCommands::RemoveInvoice { id }) => {
                log.msg(format!("Removing invoice {}", id));
                if sqlx::query!(r#"DELETE FROM invoices WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Invoice {} not found", id), -1, true);
                } else {
                    log.print(format!("Invoice {} removed", id), id, true);
                }
            }
            Some(ProjectCommands::GetQuote { id }) => {
                log.msg(format!("Getting quote with id {}", id));
                let quote = get_quote(&db_pool, *id).await?;
                log.print(format!("Got quote {id}"), quote, true);
            }
            Some(ProjectCommands::MakeQuote { args }) => {
                log.msg(format!("Making quote for project {}", args.project_id));
                let quote_url = make_quote(&db_pool, args).await?;
                if &mode == &PrintMode::Json {
                    log.print("".to_string(), Jchar::from('['), false);
                }
                log.print(
                    "Quote made, url:".to_string(),
                    quote_url,
                    &mode != &PrintMode::Json,
                );
                if &mode == &PrintMode::Json {
                    log.print("".to_string(), Jchar::from(']'), false);
                }
            }
            Some(ProjectCommands::UpdateQuote { id, args }) => {
                log.msg(format!("Updating quote {}", id));
                let updated = update_quote(&db_pool, *id, args).await?;
                if updated == 0 {
                    log.print(format!("Quote {} not found", id), -1, true);
                } else {
                    log.print(format!("Quote {id} updated"), id, true);
                }
            }
            Some(ProjectCommands::GetInvoice { id }) => {
                log.msg(format!("Getting invoice with id {}", id));
                let invoice = get_invoice(&db_pool, *id).await?;
                log.print(format!("Got invoice {id}"), invoice, true);
            }
            Some(ProjectCommands::MakeInvoice { args }) => {
                if args.quote_id.is_some() {
                    log.msg(format!(
                        "Making invoice for quote {}",
                        args.quote_id.unwrap()
                    ));
                } else if args.project_id.is_some() {
                    log.msg(format!(
                        "Making invoice for project {}",
                        args.project_id.unwrap()
                    ));
                } else if args.contract_id.is_some() {
                    log.msg(format!(
                        "Making invoice for contract {}",
                        args.contract_id.unwrap()
                    ));
                } else {
                    log.print(
                        "No quote, project or contract id was provided".to_string(),
                        -1,
                        true,
                    );
                    return Ok(());
                }

                let invoice_url = make_invoice(&db_pool, args).await?;
                log.print("Invoice made, url:".to_string(), invoice_url, true);
            }
            Some(ProjectCommands::UpdateInvoice { id, args }) => {
                log.msg(format!("Updating invoice {}", id));
                let updated = update_invoice(&db_pool, *id, args).await?;
                if updated == 0 {
                    log.print(format!("Invoice {} not found", id), -1, true);
                } else {
                    log.print(format!("Invoice {id} updated"), id, true);
                }
            }
            Some(ProjectCommands::List) => {
                log.msg("Listing all projects".to_string());
                log.msg("--------------------".to_string());
                let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
                    .fetch_all(&db_pool)
                    .await?;

                log_list!(log, mode, projects);
            }
            Some(ProjectCommands::ListTasks { id }) => {
                log.msg(format!("Listing all tasks for project {}", id));
                log.msg("---------------------------------".to_string());
                let tasks =
                    sqlx::query_as!(ProjectTask, "SELECT * FROM tasks WHERE project_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?;

                log_list!(log, mode, tasks);
            }
            Some(ProjectCommands::ListQuotes { project_id, recipient_id }) => {
                log.msg("Listing all quotes".to_string());
                log.msg("------------------".to_string());
                let quotes = match project_id {
                    Some(id) => sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE project_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    None => match recipient_id {
                        Some(id) => sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE recipient_id = ?", id)
                            .fetch_all(&db_pool)
                            .await?,
                        None => sqlx::query_as!(Quote, "SELECT * FROM quotes")
                            .fetch_all(&db_pool)
                            .await?,
                    }
                };

                log_list!(log, mode, quotes);
            }
            Some(ProjectCommands::ListInvoices { contract_id, project_id, quote_id, recipient_id  }) => {
                log.msg("Listing all invoices".to_string());
                log.msg("--------------------".to_string());



                /*let invoices = match project_id {
                    Some(id) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    None => match contract_id {
                        Some(id) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE contract_id = ?", id)
                            .fetch_all(&db_pool)
                            .await?,
                        None => match quote_id {
                            Some(id) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE quote_id = ?", id)
                                .fetch_all(&db_pool)
                                .await?,
                            None => match recipient_id {
                                Some(id) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE recipient_id = ?", id)
                                    .fetch_all(&db_pool)
                                    .await?,
                                None => sqlx::query_as!(Invoice, "SELECT * FROM invoices")
                                    .fetch_all(&db_pool)
                                    .await?,
                            }
                        }
                    }
                };*/
                let invoices = match (project_id, contract_id, quote_id, recipient_id) {
                    (Some(id), None, None, None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, Some(id), None, None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE contract_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, None, Some(id), None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE quote_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, None, None, Some(id)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE recipient_id = ?", id)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), Some(cid), None, None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND contract_id = ?", pid, cid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), None, Some(qid), None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND quote_id = ?", pid, qid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), None, None, Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND recipient_id = ?", pid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, Some(cid), Some(qid), None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE contract_id = ? AND quote_id = ?", cid, qid)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, Some(cid), None, Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE contract_id = ? AND recipient_id = ?", cid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, None, Some(qid), Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE quote_id = ? AND recipient_id = ?", qid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), Some(cid), Some(qid), None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND contract_id = ? AND quote_id = ?", pid, cid, qid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), Some(cid), None, Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND contract_id = ? AND recipient_id = ?", pid, cid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), None, Some(qid), Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND quote_id = ? AND recipient_id = ?", pid, qid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, Some(cid), Some(qid), Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE contract_id = ? AND quote_id = ? AND recipient_id = ?", cid, qid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (Some(pid), Some(cid), Some(qid), Some(rid)) => sqlx::query_as!(Invoice, "SELECT * FROM invoices WHERE project_id = ? AND contract_id = ? AND quote_id = ? AND recipient_id = ?", pid, cid, qid, rid)
                        .fetch_all(&db_pool)
                        .await?,
                    (None, None, None, None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices")
                        .fetch_all(&db_pool)
                        .await?,
                    _ => {
                        log.print("Invalid combination of arguments".to_string(), -1, true);
                        return Ok(());
                    }
                };

                log_list!(log, mode, invoices);
            }
            None => {
                log.msg("No subcommand was used".to_string());
            }
        },
        Some(Commands::Schedule { subcmd }) => match subcmd {
            Some(ScheduleCommands::Get { id }) => {
                log.msg(format!("Getting schedule with id {}", id));
                let schedule = get_schedule(&db_pool, *id).await?;
                log.print(format!("Got schedule {id}"), schedule, true);
            }
            Some(ScheduleCommands::Add { schedule }) => {
                log.msg(format!("Adding schedule"));
                let id = add_schedule(&db_pool, schedule).await?;
                log.print("Schedule added with id".to_string(), id, true);
            }
            Some(ScheduleCommands::Update { id, schedule }) => {
                log.msg(format!("Updating schedule {}", id));
                let updated = update_schedule(&db_pool, *id, schedule).await?;
                if updated == 0 {
                    log.print(format!("Schedule {} not found", id), -1, true);
                } else {
                    log.print(format!("Schedule {id} updated"), id, true);
                }
            }
            Some(ScheduleCommands::Remove { id }) => {
                log.msg(format!("Removing schedule {}", id));
                if sqlx::query!(r#"DELETE FROM schedule WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Schedule {} not found", id), -1, true);
                } else {
                    log.print(format!("Schedule {} removed", id), id, true);
                }
            }
            Some(ScheduleCommands::List) => {
                log.msg("Listing schedule".to_string());
                log.msg("----------------".to_string());
                let schedule = sqlx::query_as!(Schedule, "SELECT * FROM schedule")
                    .fetch_all(&db_pool)
                    .await?;

                log_list!(log, mode, schedule);
            }
            None => {
                log.msg(format!("No subcommand was used"));
            }
        },
        Some(Commands::Finance { subcmd }) => match subcmd {
            Some(FinanceCommands::Report { report }) => {
                log.msg(format!("Creating report {:?}", report));

                // let report = create_report(&db_pool, report).await?;
                // log.print("Report created".to_string(), report, true);
                log.print("Report created".to_string(), -1, true);
            }
            Some(FinanceCommands::AddQuery { query }) => {
                log.msg(format!("Adding query {:?}", query));
                let id = add_query(&db_pool, query).await?;
                //log.print("Query added with id".to_string(), id, true);
            }
            Some(FinanceCommands::UpdateQuery { id, query }) => {
                log.msg(format!("Updating query {}", id));
                let updated = update_query(&db_pool, *id, query).await?;
                if updated == 0 {
                    log.print(format!("Query {} not found", id), -1, true);
                } else {
                    log.print(format!("Query {id} updated"), id, true);
                }
            }
            Some(FinanceCommands::Remove { id }) => {
                log.msg(format!("Removing query {}", id));
                if sqlx::query!(r#"DELETE FROM finance_reports WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Query {} not found", id), -1, true);
                } else {
                    log.print(format!("Query {} removed", id), id, true);
                }
            }
            Some(FinanceCommands::RemoveQuery { id }) => {
                log.msg(format!("Removing query {}", id));
                if sqlx::query!(r#"DELETE FROM finance_queries WHERE id = ?"#, id)
                    .execute(&db_pool)
                    .await?
                    .rows_affected()
                    == 0
                {
                    log.print(format!("Query {} not found", id), -1, true);
                } else {
                    log.print(format!("Query {} removed", id), id, true);
                }
            }
            None => {
                log.msg(format!("No subcommand was used"));
            }
        },
        None => {
            log.msg(format!("No command was used"));
        }
    }

    Ok(())
}
