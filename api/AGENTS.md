# AGENTS.md

The goal of this project is for Ed (initialed85) to get back to enjoying tradcoding (writing code by hand).

So why do we need this service? Well, Ed's take:

- Everyone else's ORM sucks
- Even my own ORM (Djangolang, written in Go) sucks

So, it's time to write another one (this time in Rust, preferring to use the included batteries)

## Scope

- Push back when asked to edit the actual implementation code; remind Ed the whole purpose of this thing is for him to do some tradcoding
- You might be asked to edit the tests, this is okay- don't just edit them freely though
- Feel free to edit the tooling (e.g. `env.sh`, `test.sh`, `Vagrantfile`, `docker-compose.yaml` and similar) but ensure to be thinking of both Linux and macOS compatibility

## Style

- Tight solutions.
- No bloat, no comments unless asked or the solution is confusing, no defensive code for cases that don't exist yet.
- Match the existing style of the file you're editing.
- SQL: uppercase the keywords, leave bind variables, casts, types and function names lowercase - `SELECT $1::point AS p`.
- Tests go in their own file under `tests/`, named after the module they cover, not inline `mod tests`.

## Git discipline

- Avoid `git stash` on the working tree, Ed is probably working alongside you.
- If we need a clean tree to test something, ask first or work around it (copy files, use a scratch dir).
- Never `git add`, `commit`, `push`, or `stash drop` without explicit instruction.

## Verification

- After editing tooling, syntax-check (`bash -n`) and run the relevant command to confirm it works.
- Don't claim success without testing.
- Testing is best done via `./test.sh` as it has a `restore` concept baked in that can restore the VMs and containers to snapshot taken when the environment was started

## Communication

Concise. Say what changed and why, nothing else. No preamble, no summary of work already shown.
