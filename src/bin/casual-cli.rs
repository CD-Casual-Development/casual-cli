use std::env;

use anyhow::Result;
use clap::Parser;
use sqlx::SqlitePool;

use casual_cli_lib::models::*;
use casual_cli_lib::queries::*;
use casual_cli_lib::commands::*;

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
                            (None, None) => sqlx::query_as!(Contract, "SELECT * FROM contracts")
                                .fetch_all(&db_pool)
                                .await?,
                            _ => sqlx::query_as!(
                                Contract,
                                r#"
                                SELECT * FROM contracts
                                WHERE ($1 IS NULL OR recipient_id = $1)
                                AND ($2 IS NULL OR sender_id = $2)
                                "#,
                                recipient_id,
                                sender_id
                            )
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
                
                let quotes = match (project_id, recipient_id) {
                    (None, None) => sqlx::query_as!(Quote, "SELECT * FROM quotes")
                        .fetch_all(&db_pool)
                        .await?,
                    _ => sqlx::query_as!(
                        Quote,
                        r#"
                        SELECT * FROM quotes
                        WHERE ($1 IS NULL OR project_id = $1)
                        AND ($2 IS NULL OR recipient_id = $2)
                        "#,
                        project_id,
                        recipient_id
                    )
                        .fetch_all(&db_pool)
                        .await?
                };

                log_list!(log, mode, quotes);
            }
            Some(ProjectCommands::ListInvoices { contract_id, project_id, quote_id, recipient_id  }) => {
                log.msg("Listing all invoices".to_string());
                log.msg("--------------------".to_string());

                let invoices = match (project_id, contract_id, quote_id, recipient_id) {
                    (None, None, None, None) => sqlx::query_as!(Invoice, "SELECT * FROM invoices")
                        .fetch_all(&db_pool)
                        .await?,
                    _ => {
                        sqlx::query_as!(
                            Invoice,
                            r#"
                            SELECT * FROM invoices
                            WHERE ($1 IS NULL OR project_id = $1)
                            AND ($2 IS NULL OR contract_id = $2)
                            AND ($3 IS NULL OR quote_id = $3)
                            AND ($4 IS NULL OR recipient_id = $4)
                            "#,
                            project_id,
                            contract_id,
                            quote_id,
                            recipient_id
                        )
                            .fetch_all(&db_pool)
                            .await?
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
                log.print("Query added with id".to_string(), id, true);
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
