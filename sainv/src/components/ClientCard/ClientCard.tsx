import { Client_t } from "../../types/models"
import Button from "../Button/Button";
import "./ClientCard.css"

interface Props {
  client: Client_t;
}

export default function ClientCard({ client }: Props) {
  return (
    <div className="card-container">
      <h3 className="card-header">{client.business_name}</h3>
      <p className="card-data">{client.name}</p>
      <p className="card-data">{client.email}</p>
      <Button label="Delete" onClick={() => console.log("delete me")}/>
    </div>
  )
}
