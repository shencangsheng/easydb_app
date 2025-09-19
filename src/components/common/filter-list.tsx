import { Listbox, ListboxItem } from "@heroui/react";
import { useState } from "react";

interface FilterListProps {
  items: string[];
  placeholderText?: string;
  icon?: React.ReactNode;
  onSelect?: (item: string) => void;
}

const FilterList: React.FC<FilterListProps> = ({
  items,
  placeholderText = "Filter...",
  icon,
  onSelect,
}) => {
  const [filterText, setFilterText] = useState("");
  const filteredItems = items.filter((item) =>
    item.toLowerCase().includes(filterText.toLowerCase())
  );

  return (
    <div>
      <input
        type="text"
        placeholder={placeholderText}
        value={filterText}
        onChange={(e) => setFilterText(e.target.value)}
        style={{
          width: "98%",
          padding: "8px",
          marginBottom: "10px",
          border: "1px solid #ccc",
          borderRadius: "4px",
          marginTop: "10px",
        }}
      />
      <Listbox
        aria-label="Dynamic Actions"
        style={{
          width: "100%",
          textAlign: "left",
        }}
        onAction={(value) => onSelect?.(value.toString())}
      >
        {filteredItems.map((item) => (
          <ListboxItem key={item} startContent={icon && icon}>
            {item}
          </ListboxItem>
        ))}
      </Listbox>
    </div>
  );
};

export default FilterList;
