# Incident Workspace Runbook And Handoff Fixtures

This corpus anchors beta incident workspace packets that join runbook
version, step authority, action ledger outcomes, deviation notes,
console handoff metadata, and redacted export bundles in one object.

Coverage:

| Fixture | Focus |
|---|---|
| `current_runbook_with_approved_mitigation.yaml` | current runbook, target identity, approval ticket, preview hash, and evidence outputs |
| `missing_evidence_fail_closed.yaml` | missing evidence is declared and protected mutation fails closed |
| `stale_runbook_version_blocked.yaml` | stale runbook version blocks mutation |
| `blocked_approval_mitigation.yaml` | pending approval blocks mutation |
| `browser_only_vendor_docs_handoff.yaml` | browser-only vendor docs remain reference-only with explicit console handoff metadata |
| `redacted_export_bundle.yaml` | redacted export bundle reconstructs action lineage without raw private material |

The loader and validator live in
`crates/aureline-support/src/incident_workspace/mod.rs`. Fixtures must
keep raw command bodies, raw provider URLs, raw console sessions, raw
approval bodies, private content, and secrets out of the packet.
