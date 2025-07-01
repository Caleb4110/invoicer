import "./Tab.css";

type Props = {
  index: number;
  label: string;
  isActive?: boolean;
  onClick?: () => void;
};

export default function Tab({
  label,
  isActive = false,
  onClick,
}: Props) {

  return (
    <button
      className={`tab-btn ${isActive ? "active" : ""}`}
      onClick={onClick}
      type="button"
    >
      {label}
    </button>
  );
};
