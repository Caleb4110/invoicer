import { Client_t } from "../../types/models"
import Button from "../Button/Button";
import "./ClientCard.css"
import { invoke } from "@tauri-apps/api/tauri";

interface Props {
  client: Client_t;
  onClick: (e: any) => void;
}

export default function ClientCard({ client, onClick }: Props) {
  return (
    <div className="card-container">
      <h3 className="card-header">{client.business_name}</h3>
      <p className="card-data">{client.name}</p>
      <p className="card-data">{client.email}</p>
      <Button id={client.id.toString()} label="Delete" onClick={onClick} />
    </div>
  )
}
