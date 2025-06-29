use clap::Parser;
use clinv::cli::{map_command_words, Cli};
use clinv::utils::prompt_for_missing_args;
use clinv::utils::read_and_add_invoice_items;
use inv_tools::commands::Command;
use inv_tools::database;
use inv_tools::exec::*;
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::open("./clinv.db")?;
    database::init_db(&conn)?;

    // Determine the command to execute
    let mut command = match cli.command {
        // If the command is a valid command, map it straight
        Some(cmd) => cmd,

        // Otherwise, check if it's in natural language form
        // and map it based on that
        None => match map_command_words(&cli.raw_command.words) {
            Some(cmd) => cmd,

            // If neither, it's an invalid command
            None => {
                return Err(format!("Unknown command: {}", cli.raw_command.words.join(" ")).into());
            }
        },
    };

    // Since the commands can expect args such as user id, invoice id etc.
    // We need a way of prompting for them if they have not been supplied
    prompt_for_missing_args(&mut command)?;

    // Execute the command
    // Check the result and show the result based on the command
    match exec_command(&conn, &command)? {
        CommandResult::Id(id) => {
            if matches!(command, Command::NewInvoice(_)) {
                read_and_add_invoice_items(&conn, id);
            }
            println!("Invoice created!");
        }
        CommandResult::Clients(clients) => {
            println!("Clients:");
            for client in clients {
                println!("{:?}", client); // Or custom pretty print
            }
        }
        CommandResult::Invoices(invoices) => {
            println!("Invoices:");
            for invoice in invoices {
                println!("{:?}", invoice);
            }
        }
        CommandResult::InvoiceForPdf(invoice_for_pdf) => {
            println!("Generated Invoice PDF data:");
            println!("{:?}", invoice_for_pdf);
        }
        CommandResult::None => {
            println!("Operation completed.");
        }
    }

    Ok(())
}

