-- Add migration script here
INSERT INTO address (country, city, street, number, unit, postalcode)
    VALUES ('Nederland', 'Hilversum', 'Lage Naarderweg', '28', 'B', '1211 AB');
INSERT INTO companies (name, logo, commerce_number, vat_number, iban, phone, email, address_id)
    VALUES ('Casual Development', 'https://casualdevelopment.nl/logo.png', 'BE 123.456.789', 'BE 123.456.789', 'BE12 3456 7890 1234', '+32 123 45 67 89', 'kenrick@casualdevelopment.nl', LAST_INSERT_ROWID());
INSERT INTO accounts (name, phone, email, company_id, address_id, privacy_permissions)
    VALUES ('Kenrick Halff', '+31 645 01 25 53', 'kenrick@casualdevelopment.nl', LAST_INSERT_ROWID(), NULL, 'all');

