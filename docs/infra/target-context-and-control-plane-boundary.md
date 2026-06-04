# Infrastructure Target-Context and Control-Plane Boundary

This document defines the checked-in qualification packet required before an infrastructure, cluster, incident-ops, or provider-console surface can be treated as stable.

The canonical machine-readable schema is [`/schemas/infra/environment-context-and-action-safety.schema.json`](../../schemas/infra/environment-context-and-action-safety.schema.json). The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/target_context_and_control_plane_boundary/mod.rs). Fixtures live in [`/fixtures/infra/target-context-and-control-plane-boundary`](../../fixtures/infra/target-context-and-control-plane-boundary).

## Qualification Rule

A stable infra/ops row needs a current packet proving all of the following:

- target identity is explicit: provider, account or project, cluster, namespace, region or zone, tenant, workspace root, branch or commit, execution origin, toolchain identity, credential handle class, issuance source, expiry, write scope, observation time, and completeness;
- connector class is one of `static/file-only`, `CLI-mediated`, `agent-mediated live`, or `provider/console overlay`, with an explicit action envelope and freshness posture;
- desired, rendered, planned, observed, cached, permission-limited, unavailable, and provider-overlay truth stay separate in resource rows and exports;
- terminal, logs, resource graph, incident workspace, AI action sheet, CLI JSON, browser handoff, and support export render the same target chip for the same context;
- mutate, port-forward, shell attach, exec, container exec, and browser-console launch reviews show target identity, duration, credential scope, revocation path, preview envelope where applicable, and audit lineage;
- production or high-risk contexts default to read-only or approval-pending posture until step-up and preview evidence are present;
- provider consoles are explicit handoff destinations and never substitute for local source, plan, or observed truth.

Rows without current evidence are downgraded to `file_only`, `inspect_only`, or `handoff_only`.

## Fixture Meaning

- `qualified_context_parity_packet.json` proves stable parity for a production-tagged Kubernetes namespace while keeping console actions handoff-only.
- `stale_live_overlay_downgraded_packet.json` keeps desired/rendered/cached rows visible while disabling live writes after stale overlay detection.
- `wrong_target_action_blocked_packet.json` intentionally fails validation because a terminal target chip points at a different project and cluster than the packet context.

## Support Export Posture

Support exports may include packet ids, target refs, credential handle classes, freshness labels, preview hashes, audit refs, and redaction-safe summaries. They must not include raw secrets, kubeconfig bodies, cloud tokens, SSH private keys, browser cookies, raw provider responses, or private endpoint URLs.
