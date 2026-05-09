# Degraded-state vocabulary (shell surfaces)

This page is the reviewer-facing entry point for Aureline’s shared degraded-state
vocabulary as rendered in core shell surfaces (chrome, placeholders, and status
surfaces).

Authoritative source of truth:

- `/docs/governance/truth_and_degraded_state_vocabulary.md`

## Token set

The stable token set (used in exported evidence and JSON fixtures) is:

- `Warming`
- `Cached`
- `Partial`
- `Stale`
- `Offline`
- `PolicyBlocked`
- `Limited`
- `Unsupported`
- `Experimental`
- `RetestPending`

Shell UI labels may use spacing for readability (for example, `PolicyBlocked`
renders as “Policy blocked”, `RetestPending` renders as “Retest pending”), but
the serialized token values remain stable.

## Implementation notes

- Rust surfaces should reuse `crates/aureline-shell/src/state_cards/degraded_state.rs` and
  render user-facing labels via `DegradedStateToken::label()` instead of
  hardcoding strings per surface.

