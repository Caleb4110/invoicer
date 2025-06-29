# CLINV (Command Line Invoicer)

CLINV is a command-line tool for generating and managing invoices for clients, built on top of the reusable core library `inv-tools`.

## What is CLINV?

CLINV provides an easy way to create, list, and manage invoices and clients directly from your terminal. It is ideal for freelancers and small businesses who want a fast, scriptable invoicing workflow without a full GUI.

CLINV is a part of the broader [Invoicer](https://github.com/Caleb4110/invoicer) project, which also includes a reusable core library (`inv-tools`) and future plans for a standalone desktop application (`sainv`).

---

## Features

- **Create clients and invoices**: Add new clients and generate invoices with simple commands.
- **List clients and invoices**: View all clients and their invoices from the CLI.
- **Delete clients and invoices**: Remove obsolete clients or invoices.
- **Generate PDF invoices**: Produce professional PDF invoices for your records or to send to clients.
- **Interactive prompts**: If required data is missing, CLINV will prompt you interactively.
- **Natural language support**: Commands can be given in natural language (e.g., `clinv new client`).

---

## Usage

### Creating invoices or clients

```sh
clinv new client
clinv new invoice
```

You will be prompted for the required fields (e.g., name, business name, email, etc.).

### Listing invoices or clients

```sh
clinv list clients
clinv list invoices
```

### Deleting invoices or clients

```sh
clinv delete client
clinv delete invoice
```

You will be prompted for the relevant ID.

### Generating a PDF invoice

```sh
clinv generate
```

You will be prompted for the invoice ID to generate a PDF for.

---

## Notes

- CLINV stores its data in a local SQLite database (`clinv.db` by default).
- PDF invoices are generated using a customizable HTML template.
- This tool is currently sufficient for most basic invoicing needs; for a graphical interface, stay tuned for `sainv`.
- For more details on the core invoice engine, see the [inv-tools README](../inv-tools/README.md).

---

## Project Structure

- **clinv/**: This command-line application.
- **inv-tools/**: The core library providing invoice management and PDF generation logic.
- **sainv/** _(planned)_: Standalone desktop GUI application.

---

## License

MIT OR Apache-2.0

---

For more info, see the main [Invoicer README](../README.md).
