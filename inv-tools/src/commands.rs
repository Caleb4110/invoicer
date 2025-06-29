use crate::args::*;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Subcommand)]
pub enum Command {
    NewClient(NewClientArgs),
    NewInvoice(NewInvoiceArgs),
    NewItem(NewItemArgs),

    // UpdateClient(UpdateClientArgs),
    // UpdateInvoice(UpdateInvoiceArgs),
    ListClients,
    ListInvoices(ListInvoicesArgs),

    DeleteClient(DeleteClientArgs),
    DeleteInvoice(DeleteInvoiceArgs),

    Generate(GenerateArgs),
}
