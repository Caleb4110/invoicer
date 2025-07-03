import Button from "../Button/Button"
import Input from "../Input/Input"
import "./Modal.css"

interface Props {
  hidden: boolean;
}


/*
 * TODO:
 * - new invoice button directly on the client cards
 * - Three dots. click and get edit/delete
 */
export default function Modal({ hidden }: Props) {

  return (
    <form className={"modal-container " + (hidden ? "hidden" : "")}>
      <div className="modal-content">
        <h3 className="card-header">Enter Client Info</h3>
        <Input placeholder="Name..." value="" onChange={() => console.log("N change")} />
        <Input placeholder="Business Name..." value="" onChange={() => console.log("BN change")} />
        <Input placeholder="Address..." value="" onChange={() => console.log("BN change")} />
        <Input placeholder="Email..." value="" onChange={() => console.log("BN change")} />
        <Button label="Submit" onClick={() => console.log("Ya")} />
      </div>
    </form>
  )
}
