import CalendarMonthIcon from "@mui/icons-material/CalendarMonth";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import { Dispatch, SetStateAction } from "react";
import { getDateString, truncateDate } from "../helpers";

export interface DateDropdownMenuProps {
  responsive: boolean;
  startedAtGt: string | undefined;
  setStartedAtGt: Dispatch<SetStateAction<string | undefined>>;
  startedAtLte: string | undefined;
  setStartedAtLte: Dispatch<SetStateAction<string | undefined>>;
}

export default function DateDropdownMenu(props: DateDropdownMenuProps) {
  var cursor = new Date();

  if (!props.startedAtGt) {
    props.setStartedAtGt(cursor.toISOString());
    props.setStartedAtLte(
      new Date(cursor.getTime() + 24 * 60 * 60 * 1000).toISOString(),
    );
  }

  const dates = [];
  for (var i = 0; i < 14; i++) {
    dates.push(truncateDate(cursor));
    cursor = new Date(cursor.getTime() - 24 * 60 * 60 * 1000);
  }

  return (
    <Dropdown>
      <MenuButton
        variant="soft"
        color="danger"
        size={"sm"}
        sx={{ marginLeft: 0.5, marginRight: 1, width: 33 }}
      >
        <CalendarMonthIcon />
      </MenuButton>
      <Menu>
        {dates.map((date) => {
          return (
            <MenuItem
              key={`date-dropdown-menu-item-${date}`}
              selected={
                !!(
                  props.startedAtGt && props.startedAtGt === date.toISOString()
                )
              }
              onClick={(event) => {
                props.setStartedAtGt(date.toISOString());
                props.setStartedAtLte(
                  new Date(date.getTime() + 24 * 60 * 60 * 1000).toISOString(),
                );
              }}
            >
              {getDateString(date)}
            </MenuItem>
          );
        })}
      </Menu>
    </Dropdown>
  );
}
