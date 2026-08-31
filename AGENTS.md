# Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd prime` for full workflow context.

## Git Policy

At the end of a completed task, agents may create one concise, one-line commit containing that task's changes. Do not push git branches or synchronize Beads data to a remote. An explicit instruction not to commit takes precedence.

> **Architecture in one line:** Issues live in a local Dolt database
> (`.beads/dolt/`); cross-machine sync uses `bd dolt push/pull` (a
> git-compatible protocol), stored under `refs/dolt/data` on your git
> remote — separate from `refs/heads/*` where your code lives.
> `.beads/issues.jsonl` is a passive export, not the wire protocol.
>
> See [SYNC_CONCEPTS.md](https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md)
> for the one-screen overview and anti-patterns (don't treat JSONL as the
> source of truth; don't `bd import` during normal operation; don't
> reach for third-party Dolt hosting before trying the default).

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work atomically
bd close <id>         # Complete work
bd dolt push          # Push beads data to remote
```

## Non-Interactive Shell Commands

**ALWAYS use non-interactive flags** with file operations to avoid hanging on confirmation prompts.

Shell commands like `cp`, `mv`, and `rm` may be aliased to include `-i` (interactive) mode on some systems, causing the agent to hang indefinitely waiting for y/n input.

**Use these forms instead:**

```bash
# Force overwrite without prompting
cp -f source dest           # NOT: cp source dest
mv -f source dest           # NOT: mv source dest
rm -f file                  # NOT: rm file

# For recursive operations
rm -rf directory            # NOT: rm -r directory
cp -rf source dest          # NOT: cp -r source dest
```

**Other commands that may prompt:**

- `scp` - use `-o BatchMode=yes` for non-interactive
- `ssh` - use `-o BatchMode=yes` to fail instead of prompting
- `apt-get` - use `-y` flag
- `brew` - use `HOMEBREW_NO_AUTO_UPDATE=1` env var

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:6cd5cc61 -->

## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

## Agent Context Profiles

The managed Beads block is task-tracking guidance, not permission to override repository, user, or orchestrator instructions.

- **Conservative (default)**: Use `bd` for task tracking. At the end of a completed task, agents may create one concise, one-line commit containing that task's changes. Never push or synchronize Beads data to a remote. At handoff, report changed files, validation, and issue status.
- **Minimal**: Keep tool instruction files as pointers to `bd prime`; follow the same task-end commit policy.
- **Team-maintainer**: Agents may close beads, run quality gates, and create one concise, one-line commit at task completion. Never push or synchronize Beads data to a remote. A current "do not commit" instruction still wins.

## Navigating code

Always use the LSP (rust-analyzer) to navigate Rust: `hover`,
`goToDefinition`, `findReferences`, `workspaceSymbol`, and `documentSymbol`
for an enum's variants or a file's outline. Use grep only for text outside
Rust, such as the wiki, fixtures, and C sources. Right after startup,
rust-analyzer may still be indexing and return nothing. Retry before you fall
back to grep.

## Shell

The shell is fish: write Python scripts instead of bash scripts.

## Wiki

`wiki/` is shared by both crates: `wiki/concepts/` holds durable design and
decision docs, and `wiki/log/` holds point-in-time entries. Use
`llog search "<query>"` before re-deriving a decision that may already be
recorded.

## Session Completion

1. **File issues for remaining work** - Create beads for anything that needs follow-up.
2. **Run quality gates** (only if Rust changed):
   - `cargo clippy -p <crate> --allow-dirty --fix` for each changed crate, to apply what can be fixed automatically
   - `cargo fmt`
   - `cargo nextest r --release`
3. **Update issue status** - Close finished work, update in-progress items.
4. **Log every change** - `llog new`, summarizing the change in no more than 30 lines.
5. **Commit** - `git commit -m ...` with a one-line message summarizing the task.
6. **Hand off** - Summarize changes, validation, issue status, and the commit.

**Critical rules:**

- Explicit user or orchestrator instructions override this Beads block.
- One concise, one-line task-end commit is authorized unless a current instruction says not to commit.
- Never push or run Dolt remote synchronization.
<!-- END BEADS INTEGRATION -->

<!-- BEGIN BEADS CODEX SETUP: generated by bd setup codex -->

## Beads Issue Tracker

Use Beads (`bd`) for durable task tracking in repositories that include it. Use the `beads` skill at `.agents/skills/beads/SKILL.md` (project install) or `~/.agents/skills/beads/SKILL.md` (global install) for Beads workflow guidance, then use the `bd` CLI for issue operations.

### Quick Reference

```bash
bd ready                # Find available work
bd show <id>            # View issue details
bd update <id> --claim  # Claim work
bd close <id>           # Complete work
bd prime                # Refresh Beads context
```

### Rules

- Use `bd` for all task tracking; do not create markdown TODO lists.
- Run `bd prime` when Beads context is missing or stale. Codex 0.129.0+ can load Beads context automatically through native hooks; use `/hooks` to inspect or toggle them.
- Keep persistent project memory in Beads via `bd remember`; do not create ad hoc memory files.

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

<!-- END BEADS CODEX SETUP -->

## Nassau

Nassau is meant to be an SML (Standard Meta Language) implementation. It
oracle tests against polyml, and uses cranelift as its backend.

There should be **no unit tests**, all tests should be fixtures, either
in `tests/fixtures` or `tests/repl`.
