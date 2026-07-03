---
name: frames-pulse
description: Execute a single FRAMES pulse and update its wave record.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# FRAMES Pulse

Use this skill for one focused FRAMES implementation slice.

## Protocol

1. Read `context/waves/PHASES.md`.
2. Read the active wave `WAVE.md`.
3. Read the target `pulses/pulse-XX.md`.
4. Make the smallest complete docs or catalog change.
5. Update the pulse status and outcome.
6. Run `git diff --check`.

## Validation

Current validation is docs-first:

```powershell
git diff --check
```

