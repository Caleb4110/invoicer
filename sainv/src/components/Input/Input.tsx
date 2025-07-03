import "./Input.css"

type Props = {
  placeholder?: string;
  value: string;
  onChange: (e: any) => void;
}

export default function Input({ placeholder = "Search...", value, onChange }: Props) {

  return (
    <input className="app-search" type="text" placeholder={placeholder} value={value} onChange={onChange} />
  )
}
