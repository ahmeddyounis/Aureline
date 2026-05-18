# Provider Work-Item Transition Beta

Provider-backed work items now have a typed beta page in
`aureline-provider` that groups:

- durable work-item detail rows with provider, project or space,
  canonical ID, title, state rows, owner rows, freshness, write
  authority, open-external posture, and related branch/review/evidence
  refs;
- status-transition packets that preview mutation mode, target state,
  permissions, side effects, and confirm/export/cancel affordances;
- transition-review sheets that name provider mutation, local metadata,
  linked review, notification, and queued automation fanout before apply;
- offline-handoff packets that preserve intended transitions, evidence
  refs, redaction manifests, retry/export actions, publish target, and
  provider acceptance truth across outage and replay paths.

The provider-facing schema entry points are:

- `schemas/providers/work_item_detail.schema.json`
- `schemas/providers/offline_handoff_packet.schema.json`

The canonical work-item vocabularies remain in `schemas/work_items/`.
The Rust boundary is `crates/aureline-provider/src/work_items/`.

## Verification

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_transition_beta -- validate
cargo test -p aureline-provider --test work_item_transition_beta
```

The support export path is:

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_transition_beta -- support-export
```

The export intentionally carries only opaque refs, typed postures, acting
identity class, target identity, and redaction-safe summaries. It does not
carry raw provider URLs, provider payloads, comment bodies, or token material.

