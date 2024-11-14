-- Add migration script here
CREATE TABLE IF NOT EXISTS address (
    id INTEGER PRIMARY KEY NOT NULL,
    country TEXT,
    city TEXT,
    street TEXT,
    number TEXT,
    unit TEXT,
    postalcode TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS companies (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    logo TEXT,
    commerce_number TEXT,
    vat_number TEXT,
    iban TEXT,
    phone TEXT,
    email TEXT,
    address_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (address_id) REFERENCES address (id)
);

CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT,
    phone TEXT,
    email TEXT,
    company_id INTEGER,
    address_id INTEGER,
    privacy_permissions TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies (id),
    FOREIGN KEY (address_id) REFERENCES address (id)
);

CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY NOT NULL,
    client_id INTEGER NOT NULL,
    status TEXT CHECK(status IN ('PENDING_APPROVAL', 'OPEN', 'IN_PROGRESS', 'ON_HOLD', 'FINISHED', 'BILLED', 'CANCELLED')) NOT NULL DEFAULT 'PENDING_APPROVAL',
    start_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    end_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    title TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (client_id) REFERENCES accounts (id)
);

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    minutes_estimated INTEGER,
    minutes_spent INTEGER,
    minutes_remaining INTEGER,
    minutes_billed INTEGER,
    is_completed BOOLEAN DEFAULT FALSE,
    minute_rate INTEGER,
    start_date DATETIME,
    end_date DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (id)
);

CREATE TABLE IF NOT EXISTS contracts (
    id INTEGER PRIMARY KEY NOT NULL,
    sender_id INTEGER NOT NULL,
    recipient_id INTEGER NOT NULL,
    send_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    start_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    end_date DATETIME,
    cancel_date DATETIME,
    invoice_period_months INTEGER,
    auto_renew BOOLEAN,
    monthly_rate INTEGER,
    contract_type TEXT,
    contract_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (sender_id) REFERENCES accounts (id),
    FOREIGN KEY (recipient_id) REFERENCES accounts (id)
    --  Contract
    --  | Terms
    --  - Term description
    --  - Category
);

CREATE TABLE IF NOT EXISTS quotes (
    id INTEGER PRIMARY KEY NOT NULL,
    sender_id INTEGER NOT NULL,
    recipient_id INTEGER NOT NULL,
    send_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    expire_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    project_duration TEXT,
    project_id INTEGER,
    remarks TEXT,
    total_before_vat INTEGER NOT NULL,
    discount INTEGER,
    vat_percentage INTEGER DEFAULT '21',
    currency TEXT DEFAULT 'EUR' NOT NULL,
    total_after_vat INTEGER NOT NULL,
    quote_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (sender_id) REFERENCES accounts (id),
    FOREIGN KEY (recipient_id) REFERENCES accounts (id),
    FOREIGN KEY (project_id) REFERENCES projects (id)
);

CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY NOT NULL,
    sender_id INTEGER NOT NULL,
    recipient_id INTEGER NOT NULL,
    invoice_number TEXT NOT NULL,
    quote_id INTEGER,
    send_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    payment_due_date DATETIME,
    payment_date DATETIME,
    contract_id INTEGER,
    project_id INTEGER,
    remarks TEXT,
    total_before_vat INTEGER NOT NULL,
    discount INTEGER,
    vat_percentage INTEGER DEFAULT '21',
    currency TEXT DEFAULT 'EUR' NOT NULL,
    total_after_vat INTEGER NOT NULL,
    invoice_url TEXT,
    payment_request_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (sender_id) REFERENCES accounts (id),
    FOREIGN KEY (recipient_id) REFERENCES accounts (id),
    FOREIGN KEY (contract_id) REFERENCES contracts (id),
    FOREIGN KEY (project_id) REFERENCES projects (id),
    FOREIGN KEY (quote_id) REFERENCES quotes (id)
);

CREATE TABLE IF NOT EXISTS finance_queries (
    id INTEGER PRIMARY KEY NOT NULL,
    account_id INTEGER,
    company_id INTEGER,
    range TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id),
    FOREIGN KEY (company_id) REFERENCES companies (id)
);

CREATE TABLE IF NOT EXISTS finance_reports (
    id INTEGER PRIMARY KEY NOT NULL,
    account_id INTEGER,
    company_id INTEGER,
    query_id INTEGER,
    from_date DATETIME,
    to_date DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id),
    FOREIGN KEY (company_id) REFERENCES companies (id),
    FOREIGN KEY (query_id) REFERENCES finance_queries (id)
);



CREATE TABLE IF NOT EXISTS schedule (
    id INTEGER PRIMARY KEY NOT NULL,
    contract_id INTEGER,
    project_id INTEGER,
    invoice_id INTEGER,
    quote_id INTEGER,
    query_id INTEGER,
    date DATETIME,
    interval TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts (id),
    FOREIGN KEY (project_id) REFERENCES projects (id),
    FOREIGN KEY (invoice_id) REFERENCES invoices (id),
    FOREIGN KEY (quote_id) REFERENCES quotes (id),
    FOREIGN KEY (query_id) REFERENCES finance_queries (id)
);
