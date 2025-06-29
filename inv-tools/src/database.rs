use crate::{
    args::*,
    error::AppError,
    models::{Client, Invoice, InvoiceForPdf, InvoiceItem},
};
use rusqlite::{params, Connection, OptionalExtension, Result};

pub fn init_db(conn: &Connection) -> Result<(), AppError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS client (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            business_name TEXT NOT NULL,
            email TEXT NOT NULL,
            address TEXT NOT NULL
        )",
        [],
    )
    .map_err(|e| AppError::Database(format!("Failed to create client table {}", e).to_string()))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS invoice (
            id INTEGER PRIMARY KEY,
            client_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            FOREIGN KEY (client_id) REFERENCES client(id)
        )",
        [],
    )
    .map_err(|e| AppError::Database(format!("Failed to create invoice table {}", e).to_string()))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS invoice_item (
            id INTEGER PRIMARY KEY,
            invoice_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            hours FLOAT NOT NULL,
            rate FLOAT NOT NULL,
            amount FLOAT NOT NULL,
            FOREIGN KEY (invoice_id) REFERENCES invoice(id)
        )",
        [],
    )
    .map_err(|e| {
        AppError::Database(format!("Failed to create invoice_item table {}", e).to_string())
    })?;

    Ok(())
}

pub fn new_client(conn: &Connection, args: &NewClientArgs) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO client (name, business_name, email, address) VALUES (?1, ?2, ?3, ?4)",
        params![args.name, args.business_name, args.email, args.address],
    )
    .map_err(|e| AppError::Database(format!("Failed to insert new client {}", e).to_string()))?;

    Ok(())
}

pub fn new_invoice(
    conn: &Connection,
    args: &NewInvoiceArgs,
    date_string: &str,
) -> Result<i64, AppError> {
    // Check if client exists
    let client_exists: Option<i32> = conn
        .query_row(
            "SELECT id FROM client WHERE id = ?1",
            params![args.client_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| AppError::Database(format!("Failed to check client: {}", e).to_string()))?;

    // Return database error if the client does not exist
    if client_exists.is_none() {
        return Err(AppError::Database("Client does not exist".to_string()));
    }

    // Insert new invoice
    conn.execute(
        "INSERT INTO invoice (client_id, date) VALUES (?1, ?2)",
        params![args.client_id, date_string],
    )
    .map_err(|e| AppError::Database(format!("Failed to insert new invoice {}", e).to_string()))?;
    let invoice_id = conn.last_insert_rowid();

    Ok(invoice_id)
}

pub fn new_item(conn: &Connection, args: &NewItemArgs) -> Result<i64, AppError> {
    conn.execute(
            "INSERT INTO invoice_item (invoice_id, description, hours, rate, amount) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![args.invoice_id, args.description, args.hours, args.rate, args.amount],
        ).map_err(|e| AppError::Database(format!("Failed to insert new item {}", e).to_string()))?;

    // Get the last inserted id
    let item_id = conn.last_insert_rowid();

    Ok(item_id)
}

pub fn list_clients(conn: &Connection) -> Result<Vec<Client>, AppError> {
    let mut statement = conn.prepare("SELECT * FROM client").map_err(|e| {
        AppError::Database(format!("Failed to prepare statement {}", e).to_string())
    })?;

    let client_iter = statement
        .query_map([], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                business_name: row.get(2)?,
                email: row.get(3)?,
                address: row.get(4)?,
            })
        })
        .map_err(|e| AppError::Database(format!("Failed to get clients {}", e).to_string()))?;

    let clients: Vec<Client> = client_iter.filter_map(Result::ok).collect();

    Ok(clients)
}

pub fn list_invoices(conn: &Connection, args: &ListInvoicesArgs) -> Result<Vec<Invoice>, AppError> {
    let mut statement;
    let mut rows_iter;
    match args.client_id {
        // List invoices for a specific client
        Some(_) => {
            statement = conn
                .prepare(
                    "SELECT
                invoice.id as invoice_id, invoice.client_id, invoice.date,
                invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
                invoice_item.rate, invoice_item.amount
            FROM invoice
            INNER JOIN client ON invoice.client_id = client.id
            LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
            WHERE client.id = ?1
            ORDER BY invoice.id",
                )
                .map_err(|e| {
                    AppError::Database(format!("Failed to prepare statement {}", e).to_string())
                })?;
            rows_iter = statement.query(params![args.client_id]).map_err(|e| {
                AppError::Database(format!("Failed to get invoices {}", e).to_string())
            })?;
        }
        // List all invoices in the database
        None => {
            statement = conn
                .prepare(
                    "SELECT
                invoice.id as invoice_id, invoice.client_id, invoice.date,
                invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
                invoice_item.rate, invoice_item.amount
            FROM invoice
            LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
            ORDER BY invoice.id",
                )
                .map_err(|e| {
                    AppError::Database(format!("Failed to prepare statement {}", e).to_string())
                })?;
            rows_iter = statement.query([]).map_err(|e| {
                AppError::Database(format!("Failed to get invoices {}", e).to_string())
            })?;
        }
    }

    let mut invoices = Vec::new();
    let mut current_invoice_id = None;
    let mut current_invoice = None;

    loop {
        match rows_iter.next() {
            Ok(Some(row)) => {
                let invoice_id: i32 = row.get(0)?;
                if current_invoice_id != Some(invoice_id) {
                    if let Some(invoice) = current_invoice.take() {
                        invoices.push(invoice);
                    }
                    current_invoice_id = Some(invoice_id);
                    current_invoice = Some(Invoice {
                        id: invoice_id,
                        client_id: row.get(1)?,
                        date: row.get(2)?,
                        items: Vec::new(),
                    });
                }
                // If there's an item, add it
                if let Some(item_id) = row.get::<_, Option<i32>>(3)? {
                    let item = InvoiceItem {
                        id: item_id,
                        description: row.get(4)?,
                        hours: row.get(5)?,
                        rate: row.get(6)?,
                        amount: row.get(7)?,
                    };
                    if let Some(invoice) = current_invoice.as_mut() {
                        invoice.items.push(item);
                    }
                }
            }
            Ok(None) => break, // End of iteration
            Err(e) => return Err(AppError::Database(format!("Failed to get row {}", e))),
        }
    }

    if let Some(invoice) = current_invoice {
        invoices.push(invoice);
    }

    Ok(invoices)
}

pub fn delete_client(conn: &Connection, args: &DeleteClientArgs) -> Result<(), AppError> {
    conn.execute("DELETE FROM client WHERE id = ?1", params![args.client_id])
        .map_err(|e| AppError::Database(format!("Failed to delete client {}", e).to_string()))?;
    Ok(())
}

pub fn delete_invoice(conn: &Connection, args: &DeleteInvoiceArgs) -> Result<(), AppError> {
    conn.execute(
        "DELETE FROM invoice_item WHERE invoice_id = ?1",
        params![args.invoice_id],
    )
    .map_err(|e| AppError::Database(format!("Failed to delete invoice item {}", e).to_string()))?;
    conn.execute(
        "DELETE FROM invoice WHERE id = ?1",
        params![args.invoice_id],
    )
    .map_err(|e| AppError::Database(format!("Failed to delete invoice {}", e).to_string()))?;
    Ok(())
}

pub fn generate(conn: &Connection, args: &GenerateArgs) -> Result<InvoiceForPdf, AppError> {
    let mut statement = conn.prepare(
        "SELECT 
            invoice.id as invoice_id, invoice.client_id, invoice.date,
            client.name, client.business_name, client.email, client.address,
            invoice_item.id as item_id, invoice_item.description, invoice_item.hours,
            invoice_item.rate, invoice_item.amount
        FROM invoice
        RIGHT JOIN client on invoice.client_id = client.id
        LEFT JOIN invoice_item on invoice.id = invoice_item.invoice_id
        WHERE invoice.id = ?1
        ",
    )?;

    let mut rows_iter = statement.query(params![args.invoice_id]).map_err(|e| {
        AppError::Database(format!("Failed to get invoice for pdf generation {}", e).to_string())
    })?;
    let mut items = Vec::new();
    let mut id = None;
    let mut date = None;
    let mut name = None;
    let mut business_name = None;
    let mut email = None;
    let mut address = None;

    loop {
        match rows_iter.next() {
            Ok(Some(row)) => {
                if id.is_none() {
                    id = Some(row.get(0)?);
                    date = Some(row.get(2)?);
                    name = Some(row.get(3)?);
                    business_name = Some(row.get(4)?);
                    email = Some(row.get(5)?);
                    address = Some(row.get(6)?);
                }

                if let Some(item_id) = row.get::<_, Option<i32>>(7)? {
                    items.push(InvoiceItem {
                        id: item_id,
                        description: row.get(8)?,
                        hours: row.get(9)?,
                        rate: row.get(10)?,
                        amount: row.get(11)?,
                    });
                }
            }
            Ok(None) => break, // End of iteration
            Err(e) => return Err(AppError::Database(format!("Failed to get row {}", e))),
        }
    }
    if let (Some(id), Some(name), Some(business_name), Some(email), Some(address), Some(date)) =
        (id, name, business_name, email, address, date)
    {
        Ok(InvoiceForPdf {
            id,
            name,
            business_name,
            email,
            address,
            date,
            items,
        })
    } else {
        return Err(AppError::Database("Invoice does not exist".to_string()));
    }
}
