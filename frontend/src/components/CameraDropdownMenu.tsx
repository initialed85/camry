import Videocam from "@mui/icons-material/Videocam";
import Dropdown from "@mui/joy/Dropdown";
import Menu from "@mui/joy/Menu";
import MenuButton from "@mui/joy/MenuButton";
import MenuItem from "@mui/joy/MenuItem";
import { Dispatch, SetStateAction } from "react";
import { useQuery } from "../api";

export interface CameraDropdownMenuProps {
  responsive: boolean;
  cameraId: string | undefined;
  setCameraId: Dispatch<SetStateAction<string | undefined>>;
}

export default function CameraDropdownMenu(props: CameraDropdownMenuProps) {
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
        <Videocam />
      </MenuButton>
      <Menu>
        {data?.objects?.map((camera) => {
          return (
            <MenuItem
              key={`camera-dropdown-menu-item-${camera.id}`}
              selected={props.cameraId === camera.id}
              onClick={(event) => {
                props.setCameraId(
                  props.cameraId !== camera.id ? camera.id : undefined,
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
