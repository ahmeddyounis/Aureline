Request-workspace assertion and export truth fixtures.

These fixtures point at the runtime alpha scenarios that exercise
environment fingerprint state, auth-source class, assertion-suite
lineage, response preview labels, and portable-export redaction posture.
They are intentionally small: the Rust integration test replays the
source records from `fixtures/runtime/request_workspace_alpha/` so UI,
CLI, and support export surfaces cannot drift.
