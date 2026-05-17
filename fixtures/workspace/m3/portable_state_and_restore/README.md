# Workspace portable state and restore fixtures

These fixtures exercise the beta workspace serialization boundary:

- workspace authority, window topology, profile defaults, and machine-local hints remain separate;
- missing extension, remote, provider, or live-session dependencies reopen as placeholders with preserved pane ids;
- secrets, delegated approvals, live authority, and machine-unique trust anchors are named as exclusions;
- restore provenance stays visible in diagnostics, support export, and crash recovery.

The fixture card validates against the Rust model in `crates/aureline-workspace/src/serialization/` and the schema boundary in `schemas/workspace/restore_provenance.schema.json`.
