# inv-tools

`inv-tools` is the core invoice management library for the [Invoicer](https://github.com/Caleb4110/invoicer) project.

It provides reusable tools to create, manage, and generate PDF invoices. The library is designed to be integrated into both command-line and GUI applications, and is currently used by [`clinv`](../clinv/) and will be used in the upcoming `sainv` desktop app.

---

## Features

- **Client management**: Add, list, and delete clients.
- **Invoice management**: Create, list, and delete invoices; associate invoices with clients.
- **Invoice items**: Add line items (description, hours, rate, amount) to invoices.
- **PDF generation**: Generate professional PDF invoices from customizable HTML templates.
- **SQLite storage**: Uses a local SQLite database for data persistence.
- **Extensible core**: Can be used as a library for different user interfaces (CLI, GUI).

---

## Usage

Add `inv-tools` as a dependency in your Rust project:

```toml
[dependencies]
inv_tools = { path = "../inv-tools" }
```

### Example: Creating a Client and Invoice

```rust
use inv_tools::args::*;
use inv_tools::commands::*;
use inv_tools::database::*;
use rusqlite::Connection;

let conn = Connection::open("myinvoices.db")?;
init_db(&conn)?;

// Add a client
let new_client = NewClientArgs {
    name: Some("John Doe".into()),
    business_name: Some("Doe Consulting".into()),
    email: Some("john@example.com".into()),
    address: Some("123 Main St".into()),
};
new_client(&conn, &new_client)?;

// Create an invoice for the client (client_id = 1)
let new_invoice = NewInvoiceArgs { client_id: Some("1".into()) };
let invoice_id = new_invoice(&conn, &new_invoice, "2025-06-29")?;

// Generate a PDF for the invoice
let pdf_data = generate(&conn, &GenerateArgs { invoice_id: Some(invoice_id.to_string()) })?;
```

---

## Project Structure

- **src/args.rs**: Command-line argument structures.
- **src/commands.rs**: High-level command enums.
- **src/database.rs**: SQLite data access and manipulation logic.
- **src/exec.rs**: Command execution logic.
- **src/models.rs**: Core data structures for clients, invoices, and items.
- **src/utils.rs**: Utility functions, including PDF generation.

---

## Integration

- Used by [`clinv`](../clinv/) for command-line invoicing.
- Planned to be used in `sainv`, a desktop GUI invoicing application.

---

## License

MIT OR Apache-2.0

---

For more info, see the main [Invoicer README](../README.md).
