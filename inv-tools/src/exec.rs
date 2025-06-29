use chrono::DateTime;
use chrono::Local;
use rusqlite::Connection;

use crate::commands::*;
use crate::database;
use crate::models::Client;
use crate::models::Invoice;
use crate::utils::generate_pdf;

pub enum CommandResult {
    Id(i64),
    Clients(Vec<Client>),
    Invoices(Vec<Invoice>),
    InvoiceForPdf(String),
    None,
}

pub fn exec_command(
    conn: &Connection,
    cmd: &Command,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
    match cmd {
        Command::NewClient(args) => {
            database::new_client(conn, &args)?;
            Ok(CommandResult::None)
        }
        Command::NewInvoice(args) => {
            let local: DateTime<Local> = Local::now();
            let date_string = local.format("%Y-%m-%d").to_string();
            let id = database::new_invoice(conn, &args, &date_string)?;
            Ok(CommandResult::Id(id))
        }
        Command::NewItem(args) => {
            let id = database::new_item(conn, &args)?;
            Ok(CommandResult::Id(id))
        }

        Command::ListClients => {
            let clients = database::list_clients(conn)?;
            Ok(CommandResult::Clients(clients))
        }
        Command::ListInvoices(args) => {
            let invoices = database::list_invoices(conn, &args)?;
            Ok(CommandResult::Invoices(invoices))
        }

        Command::DeleteClient(args) => {
            database::delete_client(conn, &args)?;
            Ok(CommandResult::None)
        }
        Command::DeleteInvoice(args) => {
            database::delete_invoice(conn, &args)?;
            Ok(CommandResult::None)
        }

        Command::Generate(args) => {
            let invoice_for_pdf = database::generate(conn, &args)?;
            let path = generate_pdf(&invoice_for_pdf, "template.html")?;
            Ok(CommandResult::InvoiceForPdf(path))
        }
    }
}
