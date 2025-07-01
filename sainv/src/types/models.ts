export interface Client_t {
  id: number,
  name: string,
  business_name: string,
  email: string,
  address: string,
};

export interface InvoiceItem_t {
  id: number;
  description: string;
  hours: number;
  rate: number;
  amount: number;
}

export interface Invoice_t {
  id: number;
  client_id: number;
  date: string;
  items: InvoiceItem_t[];
}

export interface InvoiceForPdf_t {
  id: number;
  name: string;
  business_name: string;
  email: string;
  address: string;
  date: string;
  items: InvoiceItem_t[];
}
