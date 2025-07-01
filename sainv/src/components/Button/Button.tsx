import "./Button.css"

interface Props {
  label: string;
  onClick: () => void;
  disabled?: boolean;
}

export default function Button({ label, onClick, disabled = false }: Props) {

  return (
    <button className="app-button" onClick={onClick} disabled={disabled}>
      {label}
    </button>
  )
}
