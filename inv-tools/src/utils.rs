use std::{fs, path::Path};

use chrono::{Duration, NaiveDate};
use wkhtmltopdf::{Orientation, PdfApplication};

use crate::models::InvoiceForPdf;

pub fn generate_pdf(
    invoice: &InvoiceForPdf,
    template: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let html = fs::read_to_string(template)?;

    let date = NaiveDate::parse_from_str(&invoice.date, "%Y-%m-%d")?;
    let due_date = date + Duration::days(30); // Net 30
    let due_date_str = due_date.format("%Y-%m-%d").to_string();

    let invoice_number = format!("INV-{}-{}", invoice.date, invoice.id);

    let year_month = date.format("%Y-%m").to_string();
    let day = date.format("%d").to_string();
    let sanitised_client_name = invoice.name.replace("/", "-").replace(" ", "-");
    let filename = format!("{}-{}-{}.pdf", day, invoice.id, sanitised_client_name);
    let folder_path = Path::new("invoices").join(&year_month);
    fs::create_dir_all(&folder_path)?;
    let pdf_path = folder_path.join(&filename);
    let pdf_path_str = pdf_path.to_str().unwrap();

    // Replace template placeholders with invoice values
    // TODO: update invoice template
    let mut filled_template = html
        .replace("{invoice_id}", &invoice_number)
        .replace("{name}", &invoice.name)
        .replace("{business_name}", &invoice.business_name)
        .replace("{address}", &invoice.address)
        .replace("{date}", &invoice.date)
        .replace("{due_date}", &due_date_str);

    // Generate item list as text
    let num_items = invoice.items.len() - 1;
    let mut cur_item = 0;
    let mut total_cost = 0.00;
    let items_text: String = invoice
        .items
        .iter()
        .map(|item| {
            if cur_item == num_items {
                cur_item += 1;
                total_cost += item.amount;
                format!(
                    "<tr class=\"item last\"><td>{}</td><td style=\"text-align: left\">{}</td><td style=\"text-align: right;\">${:.2}</td><td style=\"text-align: right;\">${:.2}</td></tr>",
                    item.description, item.hours, item.rate, item.amount
                )
            } else {
                cur_item += 1;
                total_cost += item.amount;
                format!(
                    "<tr class=\"item last\"><td>{}</td><td style=\"text-align: left\">{}</td><td style=\"text-align: right;\">${:.2}</td><td style=\"text-align: right;\">${:.2}</td></tr>",
                    item.description, item.hours, item.rate, item.amount
                )
            }
        })
        .collect();

    filled_template = filled_template.replace("{items}", &items_text);
    filled_template = filled_template.replace("{total}", &format!("{:.2}", total_cost));
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .orientation(Orientation::Portrait)
        .title("Invoice")
        .build_from_html(filled_template)
        .expect("failed to build pdf");

    pdfout.save(pdf_path_str).expect("failed to save file");

    Ok(pdf_path_str.to_string())
}
