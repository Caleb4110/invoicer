use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct NewClientArgs {
    pub name: Option<String>,
    pub business_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct NewInvoiceArgs {
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct NewItemArgs {
    pub invoice_id: Option<String>,
    pub description: Option<String>,
    pub hours: Option<String>,
    pub rate: Option<String>,
    pub amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct ListInvoicesArgs {
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct DeleteClientArgs {
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct DeleteInvoiceArgs {
    pub invoice_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Args)]
pub struct GenerateArgs {
    pub invoice_id: Option<String>,
}
