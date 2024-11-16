use clap::Args as ClapArgs;

#[derive(ClapArgs, Debug)]
pub struct AddressCreateArgs {
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(short, long)]
    pub street: Option<String>,
    #[arg(short, long)]
    pub number: Option<String>,
    #[arg(short, long)]
    pub unit: Option<String>,
    #[arg(short, long)]
    pub postalcode: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct AddressUpdateArgs {
    #[arg(short, long)]
    pub id: i64,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(short, long)]
    pub street: Option<String>,
    #[arg(short, long)]
    pub number: Option<String>,
    #[arg(short, long)]
    pub unit: Option<String>,
    #[arg(short, long)]
    pub postalcode: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct AccountCreateArgs {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub phone: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    #[arg(short, long)]
    pub address_id: Option<i64>,
    #[arg(long)]
    pub company_name: Option<String>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(short, long)]
    pub street: Option<String>,
    #[arg(long)]
    pub number: Option<String>,
    #[arg(short, long)]
    pub unit: Option<String>,
    #[arg(long)]
    pub postalcode: Option<String>,
    #[arg(long)]
    pub privacy_permissions: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct AccountUpdateArgs {
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub phone: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    #[arg(short, long)]
    pub address_id: Option<i64>,
    #[arg(long)]
    pub privacy_permissions: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct CompanyCreateArgs {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub logo: Option<String>,
    #[arg(short, long)]
    pub commerce_number: Option<String>,
    #[arg(short, long)]
    pub vat_number: Option<String>,
    #[arg(short, long)]
    pub iban: Option<String>,
    #[arg(short, long)]
    pub phone: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(long)]
    pub address_id: Option<i64>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(short, long)]
    pub street: Option<String>,
    #[arg(long)]
    pub number: Option<String>,
    #[arg(short, long)]
    pub unit: Option<String>,
    #[arg(long)]
    pub postalcode: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct CompanyUpdateArgs {
    #[arg(short, long)]
    pub id: i64,
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub logo: Option<String>,
    #[arg(short, long)]
    pub commerce_number: Option<String>,
    #[arg(short, long)]
    pub vat_number: Option<String>,
    #[arg(short, long)]
    pub iban: Option<String>,
    #[arg(short, long)]
    pub phone: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub address_id: Option<i64>,
}

#[derive(ClapArgs, Debug)]
pub struct ProjectCreateArgs {
    #[arg(short, long)]
    pub title: String,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(short, long)]
    pub client_id: i64,
}

#[derive(ClapArgs, Debug)]
pub struct ProjectUpdateArgs {
    #[arg(short, long)]
    pub title: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(short, long)]
    pub client_id: Option<i64>,
}

#[derive(ClapArgs, Debug)]
pub struct ProjectTaskCreateArgs {
    #[arg(short, long)]
    pub project_id: i64,
    #[arg(short, long)]
    pub title: String,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(long)]
    pub minutes_estimated: Option<i64>,
    #[arg(long)]
    pub minutes_spent: Option<i64>,
    #[arg(long)]
    pub minutes_remaining: Option<i64>,
    #[arg(long)]
    pub minutes_billed: Option<i64>,
    #[arg(long)]
    pub minute_rate: Option<i64>,
}

#[derive(ClapArgs, Debug)]
pub struct ProjectTaskUpdateArgs {
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(short, long)]
    pub title: Option<String>,
    #[arg(short, long)]
    pub description: Option<String>,
    #[arg(long)]
    pub minutes_estimated: Option<i64>,
    #[arg(long)]
    pub minutes_spent: Option<i64>,
    #[arg(long)]
    pub minutes_remaining: Option<i64>,
    #[arg(long)]
    pub minutes_billed: Option<i64>,
    #[arg(long)]
    pub minute_rate: Option<i64>,
}

#[derive(ClapArgs, Debug)]
pub struct ContractCreateArgs {
    #[arg(short, long)]
    pub sender_id: i64,
    #[arg(short, long)]
    pub recipient_id: i64,
    #[arg(short, long)]
    pub contract_type: Option<String>,
    #[arg(short, long)]
    pub invoice_period_months: Option<i64>,
    #[arg(short, long)]
    pub monthly_rate: Option<i64>,
    #[arg(long)]
    pub contract_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct ContractUpdateArgs {
    #[arg(short, long)]
    pub sender_id: Option<i64>,
    #[arg(short, long)]
    pub recipient_id: Option<i64>,
    #[arg(short, long)]
    pub contract_type: Option<String>,
    #[arg(short, long)]
    pub invoice_period_months: Option<i64>,
    #[arg(short, long)]
    pub monthly_rate: Option<i64>,
    #[arg(long)]
    pub contract_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct QuoteCreateArgs {
    #[arg(short, long)]
    pub sender_id: i64,
    #[arg(short, long)]
    pub recipient_id: i64,
    #[arg(long)]
    pub project_duration: Option<String>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub total_before_vat: Option<i64>,
    #[arg(short, long)]
    pub discount: Option<i64>,
    #[arg(short, long)]
    pub vat_percentage: Option<i64>,
    #[arg(short, long)]
    pub currency: Option<String>,
    #[arg(long)]
    pub total_after_vat: Option<i64>,
    #[arg(short, long)]
    pub quote_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct QuoteUpdateArgs {
    #[arg(short, long)]
    pub sender_id: Option<i64>,
    #[arg(short, long)]
    pub recipient_id: Option<i64>,
    #[arg(long)]
    pub project_duration: Option<String>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub total_before_vat: Option<i64>,
    #[arg(short, long)]
    pub discount: Option<i64>,
    #[arg(short, long)]
    pub vat_percentage: Option<i64>,
    #[arg(short, long)]
    pub currency: Option<String>,
    #[arg(long)]
    pub total_after_vat: Option<i64>,
    #[arg(short, long)]
    pub quote_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct QuoteMakeArgs {
    #[arg(short, long)]
    pub project_id: i64,
    #[arg(short, long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub discount: Option<i64>,
    #[arg(short, long)]
    pub vat_percentage: Option<i64>,
    #[arg(short, long)]
    pub currency: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct InvoiceCreateArgs {
    #[arg(short, long)]
    pub sender_id: i64,
    #[arg(short, long)]
    pub recipient_id: i64,
    #[arg(short, long)]
    pub invoice_number: String,
    #[arg(short, long)]
    pub quote_id: Option<i64>,
    #[arg(long)]
    pub send_date: Option<String>,
    #[arg(long)]
    pub payment_due_date: Option<String>,
    #[arg(long)]
    pub payment_date: Option<String>,
    #[arg(short, long)]
    pub contract_id: Option<i64>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub total_before_vat: Option<i64>,
    #[arg(short, long)]
    pub discount: Option<i64>,
    #[arg(short, long)]
    pub vat_percentage: Option<i64>,
    #[arg(long)]
    pub currency: Option<String>,
    #[arg(long)]
    pub total_after_vat: Option<i64>,
    #[arg(long)]
    pub invoice_url: Option<String>,
    #[arg(long)]
    pub payment_request_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct InvoiceUpdateArgs {
    #[arg(short, long)]
    pub sender_id: Option<i64>,
    #[arg(short, long)]
    pub recipient_id: Option<i64>,
    #[arg(short, long)]
    pub invoice_number: Option<String>,
    #[arg(short, long)]
    pub quote_id: Option<i64>,
    #[arg(long)]
    pub send_date: Option<String>,
    #[arg(long)]
    pub payment_due_date: Option<String>,
    #[arg(long)]
    pub payment_date: Option<String>,
    #[arg(short, long)]
    pub contract_id: Option<i64>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub total_before_vat: Option<i64>,
    #[arg(short, long)]
    pub discount: Option<i64>,
    #[arg(short, long)]
    pub vat_percentage: Option<i64>,
    #[arg(long)]
    pub currency: Option<String>,
    #[arg(long)]
    pub total_after_vat: Option<i64>,
    #[arg(long)]
    pub invoice_url: Option<String>,
    #[arg(long)]
    pub payment_request_url: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct InvoiceMakeArgs {
    #[arg(short, long)]
    pub quote_id: Option<i64>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(short, long)]
    pub contract_id: Option<i64>,
    #[arg(short, long)]
    pub remarks: Option<String>,
    #[arg(short, long)]
    pub discount: Option<i64>,
}

#[derive(ClapArgs, Debug)]
pub struct ScheduleCreateArgs {
    #[arg(short, long)]
    pub contract_id: Option<i64>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(short, long)]
    pub invoice_id: Option<i64>,
    #[arg(short, long)]
    pub quote_id: Option<i64>,
    #[arg(short, long)]
    pub query_id: Option<i64>,
    #[arg(short, long)]
    pub date: Option<String>,
    #[arg(short, long)]
    pub interval: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct ScheduleUpdateArgs {
    #[arg(short, long)]
    pub contract_id: Option<i64>,
    #[arg(short, long)]
    pub project_id: Option<i64>,
    #[arg(short, long)]
    pub invoice_id: Option<i64>,
    #[arg(short, long)]
    pub quote_id: Option<i64>,
    #[arg(short, long)]
    pub query_id: Option<i64>,
    #[arg(short, long)]
    pub date: Option<String>,
    #[arg(short, long)]
    pub interval: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct FinanceReportArgs {
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    #[arg(short, long)]
    pub from_date: Option<String>,
    #[arg(short, long)]
    pub to_date: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct FinanceReportUpdateArgs {
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    #[arg(short, long)]
    pub from_date: Option<String>,
    #[arg(short, long)]
    pub to_date: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct FinanceCreateQueryArgs {
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    // ie. "7d", "1m", "1y" like ScheduleCreateArgs.interval
    #[arg(short, long)]
    pub range: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct FinanceUpdateQueryArgs {
    #[arg(short, long)]
    pub account_id: Option<i64>,
    #[arg(short, long)]
    pub company_id: Option<i64>,
    // ie. "7d", "1m", "1y" like ScheduleCreateArgs.interval
    #[arg(short, long)]
    pub range: Option<String>,
}
