import CalendarMonthIcon from "@mui/icons-material/CalendarMonth";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import { Dispatch, SetStateAction } from "react";

export interface DateDropdownMenuProps {
  responsive: boolean;
  date: string | null | undefined;
  setDate: Dispatch<SetStateAction<string | null | undefined>>;
}

export default function DateDropdownMenu(props: DateDropdownMenuProps) {
  const formatDate = (date: Date): string => {
    const pad = (s: string): string => {
      if (s.length < 2) {
        return `0${s}`;
      }

      return s;
    };

    const year = date.getFullYear().toString();
    const month = pad((date.getMonth() + 1).toString());
    const day = pad(date.getDate().toString());

    return `${year}-${month}-${day}`;
  };

  var cursor = new Date();
  const dates = [];
  for (var i = 0; i < 14; i++) {
    dates.push(formatDate(cursor));

    cursor.setMilliseconds(cursor.getMilliseconds() - 24 * 60 * 60 * 1000);
  }

  return (
    <Dropdown>
      <MenuButton variant="soft" color="danger" size={"sm"} sx={{ marginLeft: 0.5, marginRight: 1, width: 33 }}>
        <CalendarMonthIcon />
      </MenuButton>
      <Menu>
        {dates.map((date) => {
          return (
            <MenuItem
              key={date}
              selected={props.date === date}
              onClick={(event) => {
                props.setDate(date);
              }}
            >
              {date}
            </MenuItem>
          );
        })}
      </Menu>
    </Dropdown>
  );
}
