import PlayArrow from "@mui/icons-material/PlayArrow";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import { useQuery } from "../api";

export interface StreamDropdownMenuProps {
  responsive: boolean;
}

export default function StreamDropdownMenu(props: StreamDropdownMenuProps) {
  const { data } = useQuery("get", "/api/cameras", {
    params: {
      query: {
        name__asc: "",
      },
    },
  });

  return (
    <Dropdown>
      <MenuButton
        variant="soft"
        color="danger"
        size={"sm"}
        sx={{ marginLeft: 0.5, marginRight: 1, width: 33 }}
      >
        <PlayArrow />
      </MenuButton>
      <Menu>
        {data?.objects?.map((camera) => {
          return (
            <MenuItem
              key={`stream-dropdown-menu-item-${camera.id}`}
              onClick={(event) => {
                window.open(
                  `https://camry-stream.initialed85.cc/streams/${camera.id}`,
                  "_blank",
                );
              }}
            >
              {camera.name}
            </MenuItem>
          );
        })}
      </Menu>
    </Dropdown>
  );
}
