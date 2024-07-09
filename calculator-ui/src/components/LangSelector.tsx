import i18n from "i18next";

import {
  Dropdown,
  DropdownTrigger,
  DropdownMenu,
  DropdownItem,
  Button,
  Selection,
} from "@nextui-org/react";
import React from "react";

type Item = {
  key: string;
  label: string;
};

const items: Item[] = [
  { key: "en", label: "English" },
  { key: "ko", label: "한국어" },
];

export const LangSelector = () => {
  const [selected, setSelected] = React.useState<Item>({
    key: "en",
    label: "English",
  });

  return (
    <Dropdown>
      <DropdownTrigger>
        <Button variant="bordered" className="capitalize">
          {selected.label}
        </Button>
      </DropdownTrigger>
      <DropdownMenu
        aria-label="Single selection"
        variant="flat"
        disallowEmptySelection
        selectionMode="single"
        selectedKeys={selected.key}
        onSelectionChange={(key: Selection) => {
          console.log(`key: ${JSON.stringify(key)}`);
          const param = key as unknown as { currentKey: string }
          const res = items.find((item) => item.key === param.currentKey)
          const found = res ? res : items[0]

          i18n.changeLanguage(found.key)
          setSelected(found);
        }}
      >
        <DropdownItem key="en">English</DropdownItem>
        <DropdownItem key="ko">한국어</DropdownItem>
      </DropdownMenu>
    </Dropdown>
  );
};
