# Session-plan / attempt-record ledger fixtures

Proof fixtures for the session / attempt ledger packet
(`test_session_attempt_ledger_packet`). Each fixture is an export-safe packet that
`SessionAttemptLedgerPacket::validate` accepts and that exercises a specific truth
the contract guarantees.

- `imported_stale_join_stays_read_only.json` — a ledger whose sessions span the
  local, remote, notebook, and imported-provider flows with run-selected,
  rerun-failed, and import-provider-join modes (a parameterized template kept
  distinct from its concrete invocations). Its attempt history shows a local
  initial run followed by an append-only failed-only rerun, single passing remote
  and notebook runs, and — on one ledger — an imported CI join whose evidence is
  **stale** (`imported_stale`, never rolled up green) joined alongside a **local
  parity rerun** that carries its own local runtime / toolchain / env lineage, so
  the imported verdict can never read as a local rerun.

The boundary schema is
`schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json`;
the contract doc is
`docs/testing/m5/session-plans-attempt-records-and-execution-lineage.md`.
Regenerate the canonical export with:

```bash
cargo run -p aureline-runtime --example dump_session_attempt_ledger
```
