import Box from "@mui/joy/Box";
import Button from "@mui/joy/Button";
import Input from "@mui/joy/Input";
import Sheet from "@mui/joy/Sheet";
import Stack from "@mui/joy/Stack";
import Typography from "@mui/joy/Typography";
import React, { useState } from "react";
import { login } from "../auth";

export interface LoginProps {
  onAuthenticated: () => void;
}

export default function Login(props: LoginProps) {
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    setSubmitting(true);
    setError("");

    try {
      await login(password);
      setPassword("");
      props.onAuthenticated();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Login failed.");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <Box
      sx={{
        display: "grid",
        minHeight: "100vh",
        placeItems: "center",
        p: 3,
        bgcolor: "#fafafa",
      }}
    >
      <Sheet
        variant="plain"
        sx={{
          width: "min(100%, 360px)",
          p: "34px 32px 32px",
          border: "1px solid #dedede",
          borderRadius: 6,
          bgcolor: "#fff",
          boxShadow: "md",
        }}
      >
        <Stack
          direction="row"
          spacing={1.5}
          alignItems="center"
          sx={{ mb: 2.5 }}
        >
          <Box
            sx={{
              display: "grid",
              width: 42,
              height: 42,
              placeItems: "center",
              borderRadius: 7,
              bgcolor: "#222",
              color: "#fff",
              fontSize: 25,
              fontWeight: 700,
            }}
          >
            C
          </Box>
          <Typography
            level="body-xs"
            sx={{ color: "#888", letterSpacing: "0.5px", fontSize: "11px" }}
          >
            Camry
          </Typography>
        </Stack>

        <form onSubmit={handleSubmit}>
          <Input
            id="login-password"
            type="password"
            aria-label="Password"
            placeholder="Password"
            autoComplete="current-password"
            autoFocus
            required
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            sx={{
              width: "100%",
              fontSize: "12px",
              "--Input-radius": "3px",
              "--Input-paddingInline": "10px",
              "--Input-paddingBlock": "9px",
            }}
          />

          {error ? (
            <Typography
              level="body-xs"
              role="alert"
              sx={{ mt: 1, color: "#b42318", fontSize: "11px" }}
            >
              {error}
            </Typography>
          ) : null}

          <Button
            type="submit"
            color="primary"
            disabled={submitting}
            sx={{
              mt: 2.25,
              width: "100%",
              fontSize: "11px",
              "--Button-radius": "3px",
              "--Button-paddingBlock": "7px",
              "--Button-paddingInline": "12px",
            }}
          >
            {submitting ? "Logging in..." : "Log in"}
          </Button>
        </form>
      </Sheet>
    </Box>
  );
}
