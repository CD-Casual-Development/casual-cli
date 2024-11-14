use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
pub struct Address {
    pub id: i64,
    pub country: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub number: Option<String>,
    pub unit: Option<String>,
    pub postalcode: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub commerce_number: Option<String>,
    pub vat_number: Option<String>,
    pub iban: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Account {
    pub id: i64,
    pub company_id: Option<i64>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_id: Option<i64>,
    pub privacy_permissions: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub client_id: i64,
    pub status: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ProjectTask {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub minutes_estimated: Option<i64>,
    pub minutes_spent: Option<i64>,
    pub minutes_remaining: Option<i64>,
    pub minutes_billed: Option<i64>,
    pub minute_rate: Option<i64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub is_completed: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Contract {
    pub id: i64,
    pub sender_id: i64,
    pub recipient_id: i64,
    pub contract_type: Option<String>,
    pub send_date: Option<NaiveDateTime>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub auto_renew: Option<bool>,
    pub cancel_date: Option<NaiveDateTime>,
    pub invoice_period_months: Option<i64>,
    pub monthly_rate: Option<i64>,
    pub contract_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Quote {
    pub id: i64,
    pub sender_id: i64,
    pub recipient_id: i64,
    pub send_date: Option<NaiveDateTime>,
    pub expire_date: Option<NaiveDateTime>,
    pub project_duration: Option<String>,
    pub project_id: Option<i64>,
    pub remarks: Option<String>,
    pub total_before_vat: i64,
    pub discount: Option<i64>,
    pub vat_percentage: Option<i64>,
    pub currency: String,
    pub total_after_vat: i64,
    pub quote_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Invoice {
    pub id: i64,
    pub sender_id: i64,
    pub recipient_id: i64,
    pub invoice_number: String,
    pub quote_id: Option<i64>,
    pub send_date: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_date: Option<NaiveDateTime>,
    pub contract_id: Option<i64>,
    pub project_id: Option<i64>,
    pub remarks: Option<String>,
    pub total_before_vat: i64,
    pub discount: Option<i64>,
    pub vat_percentage: Option<i64>,
    pub currency: String,
    pub total_after_vat: i64,
    pub invoice_url: Option<String>,
    pub payment_request_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Schedule {
    pub id: i64,
    pub contract_id: Option<i64>,
    pub project_id: Option<i64>,
    pub invoice_id: Option<i64>,
    pub quote_id: Option<i64>,
    pub query_id: Option<i64>,
    pub date: Option<NaiveDateTime>,
    pub interval: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct FinanceReport {
    pub id: i64,
    pub account_id: Option<i64>,
    pub company_id: Option<i64>,
    pub query_id: Option<i64>,
    pub from_date: Option<NaiveDateTime>,
    pub to_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct FinanceQuery {
    pub id: i64,
    pub account_id: Option<i64>,
    pub company_id: Option<i64>,
    pub range: Option<String>,
}
