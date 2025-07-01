import { Client_t } from "../../types/models"
import Button from "../Button/Button";
import "./ClientCard.css"

interface Props {
  client: Client_t;
}

export default function ClientCard({ client }: Props) {
  return (
    <div className="client-card-container">
      <h3>{client.business_name}</h3>
      <p>{client.name}</p>
      <p>{client.email}</p>
      <Button label="Delete" onClick={() => console.log("delete me")}/>
    </div>
  )
}
