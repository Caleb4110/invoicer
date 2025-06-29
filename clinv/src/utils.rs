use email_address::EmailAddress;
use inv_tools::{
    args::NewItemArgs,
    commands::Command,
    exec::{exec_command, CommandResult},
};
use phonenumber::{country, parse};
use rusqlite::Connection;
use std::io::{self, Write};

pub fn prompt_for_missing_args(cmd: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::NewClient(args) => {
            if args.name.is_none() {
                args.name = Some(prompt_for_str("Enter client name: "));
            }
            if args.business_name.is_none() {
                args.business_name = Some(prompt_for_str("Enter business name: "));
            }
            if args.email.is_none() {
                args.email = Some(prompt_for_str("Enter client email: "));
            }
            if args.address.is_none() {
                args.address = Some(prompt_for_str("Enter business address: "))
            }
        }
        Command::NewInvoice(args) => {
            if args.client_id.is_none() {
                args.client_id = Some(prompt_for_str(
                    "What is the ID of the client to invoice?: ",
                ));
            }
        }
        Command::ListInvoices(args) => {
            if args.client_id.is_none() {
                args.client_id = Some(prompt(
                    "All or client specific? (client ID OR [Enter]): ",
                ));
            }
        }
        Command::DeleteClient(args) => {
            if args.client_id.is_none() {
                args.client_id = Some(prompt_for_str(
                    "Enter client ID: ",
                ));
            }
        }
        Command::DeleteInvoice(args) => {
            if args.invoice_id.is_none() {
                args.invoice_id = Some(prompt_for_str("Enter invoice ID: "));
            }
        }
        Command::Generate(args) => {
            if args.invoice_id.is_none() {
                args.invoice_id = Some(prompt_for_str("Enter invoice ID: "));
            }
        }
        _ => {}
    }

    Ok(())
}

// Generic prompt function
pub fn prompt(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn prompt_for_f64(prompt_msg: &str) -> f64 {
    loop {
        let input = prompt(prompt_msg);
        if input.is_empty() {
            println!("Value must not be empty");
            continue;
        }
        match input.parse::<f64>() {
            Ok(n) => return n,
            Err(e) => println!("Not a valid number: {}", e),
        }
    }
}

pub fn prompt_for_str(prompt_msg: &str) -> String {
    loop {
        let input = prompt(prompt_msg);
        if input.is_empty() {
            println!("Value must not be empty");
            continue;
        }
        return input;
    }
}

pub fn read_and_add_invoice_items(conn: &Connection, invoice_id: i64) {
    loop {
        let description = prompt("Description (leave empty to finish): ");
        if description.is_empty() {
            break;
        }
        let hours = prompt_for_f64("Hours: ");
        let rate = prompt_for_f64("Rate: ");
        let amount = hours * rate;

        let command = Command::NewItem(NewItemArgs {
            invoice_id: Some(invoice_id.to_string()),
            description: Some(description),
            hours: Some(hours.to_string()),
            rate: Some(rate.to_string()),
            amount: Some(amount.to_string()),
        });

        match exec_command(conn, &command) {
            Ok(CommandResult::Id(_)) => {
                println!("Item added.\n")
            }
            _ => {
                println!("An error occured")
            }
        }
    }
}

pub fn is_valid_phone(number: &str) -> bool {
    parse(Some(country::AU), number).is_ok()
}

pub fn is_valid_email(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
