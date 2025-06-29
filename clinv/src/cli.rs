use clap::Parser;
use inv_tools::args::*;
use inv_tools::commands::Command;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Raw command input for natural language processing
    #[clap(flatten)]
    pub raw_command: RawCommandInput,
}

#[derive(Parser, Debug)]
pub struct RawCommandInput {
    /// The raw command words (only used if no structured command is provided)
    #[arg(trailing_var_arg = true)]
    pub words: Vec<String>,
}

/// Maps natural language commands to structured commands
pub fn map_command_words(words: &[String]) -> Option<Command> {
    if words.is_empty() {
        return None;
    }

    match words {
        [s1, s2, ..] if s1.to_lowercase() == "new" && s2.to_lowercase() == "client" => {
            Some(Command::NewClient(NewClientArgs {
                name: None,
                business_name: None,
                email: None,
                address: None,
            }))
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "new" && s2.to_lowercase() == "invoice" => {
            let client_id = rest.get(0).map(|s| s.clone());
            Some(Command::NewInvoice(NewInvoiceArgs { client_id }))
        }
        [s1, s2] if s1.to_lowercase() == "list" && s2.to_lowercase() == "clients" => {
            Some(Command::ListClients)
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "list" && s2.to_lowercase() == "invoices" => {
            let client_id = rest.get(0).map(|s| s.clone());
            Some(Command::ListInvoices(ListInvoicesArgs { client_id }))
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "client" => {
            let client_id = rest.get(0).map(|s| s.clone());
            Some(Command::DeleteClient(DeleteClientArgs { client_id }))
        }
        [s1, s2, rest @ ..] if s1.to_lowercase() == "delete" && s2.to_lowercase() == "invoice" => {
            let id = rest.get(0).map(|s| s.clone());
            Some(Command::DeleteInvoice(DeleteInvoiceArgs { invoice_id: id }))
        }
        [s1, rest @ ..] if s1.to_lowercase() == "generate" => {
            let id = rest.get(0).map(|s| s.clone());
            Some(Command::Generate(GenerateArgs { invoice_id: id }))
        }
        _ => None,
    }
}
