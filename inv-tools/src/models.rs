#[derive(Debug)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub business_name: String,
    pub email: String,
    pub address: String
}

#[derive(Debug)]
pub struct InvoiceItem {
    pub id: i32,
    pub description: String,
    pub hours: f64,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Debug)]
pub struct Invoice {
    pub id: i32,
    pub client_id: i32,
    pub date: String,
    pub items: Vec<InvoiceItem>,
}

#[derive(Debug)]
pub struct InvoiceForPdf {
    pub id: i32,
    pub name: String,
    pub business_name: String,
    pub email: String,
    pub address: String,
    pub date: String,
    pub items: Vec<InvoiceItem>,
}
