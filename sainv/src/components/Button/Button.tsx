import "./Button.css"

interface Props {
  id?: string;
  label: string;
  onClick: (() => void) | ((e: any) => void);
  disabled?: boolean;
}

export default function Button({ id = "", label, onClick, disabled = false }: Props) {

  return (
    <button id={id.toString()} className="app-button" onClick={onClick} disabled={disabled}>
      {label}
    </button>
  )
}
