import "./Search.css"

type Props = {
  placeholder?: string;
  value: string;
  onChange: (e: any) => void;
  debounce?: number;

}

export default function Search({ placeholder = "Search...", value, onChange }: Props) {

  return (
    <input className="app-search" type="text" placeholder={placeholder} onChange={onChange} />
  )
}
