# Alpha Provider-Object Model

This document is the reviewer-facing landing page for the alpha provider-object
model: every code-host, issue, and CI/check row Aureline references from the
workspace, runtime, review, or git lanes mints one typed local object record
that names its source, freshness, current publish state, the user-facing mode
the row is in (`local_draft_mode`, `publish_now_mode`, `open_in_provider_mode`,
`publish_later_mode`, or `inspect_only_mode`), and the typed degraded action
the row still offers when the upstream provider is stale, offline, revoked, or
disagrees with the local model.

The machine-readable boundary lives at
[`/schemas/providers/provider_object.schema.json`](../../../schemas/providers/provider_object.schema.json).
The Rust implementation lives at
[`/crates/aureline-provider/src/object_model/`](../../../crates/aureline-provider/src/object_model/mod.rs).
The protected fixture lives at
[`/fixtures/providers/provider_object_alpha/page.json`](../../../fixtures/providers/provider_object_alpha/page.json).

## The alpha promise

- every provider-linked row mints one
  [`ProviderObjectRow`](../../../crates/aureline-provider/src/object_model/mod.rs)
  that names provider family, object kind, source class, freshness truth,
  current publish state, the user-facing mode, and the typed degraded action;
- workspace, runtime, review, and git surfaces all consume the same record
  through the page-level
  [`ProviderObjectModelAlphaPage`](../../../crates/aureline-provider/src/object_model/mod.rs)
  and the redaction-safe
  [`ProviderObjectModelAlphaSupportExport`](../../../crates/aureline-provider/src/object_model/mod.rs)
  projection, so provider-linked truth never forks per surface;
- when the upstream provider is stale, offline, revoked, or disagrees, the row
  records one
  [`ProviderObjectContinuityObservation`](../../../crates/aureline-provider/src/object_model/mod.rs)
  that names the typed observation class, the retained capability the local
  model preserves, and the typed degraded action — the workflow never
  collapses into a generic error.

## Provider families and object kinds

| Provider family | Object kinds |
| --- | --- |
| `code_host` | `pull_request`, `branch` |
| `issue_tracker` | `issue_or_work_item` |
| `ci_checks` | `check_run`, `pipeline_run`, `pipeline_log`, `pipeline_artifact`, `pipeline_annotation` |

The validator rejects rows whose `object_kind` is not admissible for their
`provider_family`.

## Source classes

| Class | Meaning |
| --- | --- |
| `live_provider` | Live provider connection observed the row |
| `cached_provider_overlay` | Cached overlay inside a freshness window |
| `imported_snapshot` | Imported provider snapshot |
| `mirrored_or_self_hosted` | Mirrored or self-hosted route |
| `local_draft_only` | Local-only draft authored before any provider observation |
| `offline_unverified_capture` | Captured offline; never verified against live provider truth |

`local_draft_only` and `offline_unverified_capture` are local-authority
sources. The row exists without observed provider truth and continues to
support local authoring while the upstream provider is stale, offline, or
revoked.

## Publish state vs. mode

The `publish_state` is the *current state* the row sits in. The `mode` is the
*intent* the user chose when authoring the row. The two are intentionally
separate so the acceptance contract holds: `local_draft`, `publish_later`,
`open_in_provider`, and `inspect_only` modes are never collapsed into one
another.

| Mode | Admissible publish states | Required ref |
| --- | --- | --- |
| `local_draft_mode` | `local_draft_only`, `offline_unverified` | `local_draft_ref` |
| `publish_now_mode` | `publish_now_pending_review`, `publish_now_published_observed`, `published_observed_authoritative`, `disagrees_with_local` | `approval_ticket_ref` (until published) |
| `open_in_provider_mode` | `open_in_provider_pending`, `published_observed_authoritative` | `browser_handoff_packet_ref` |
| `publish_later_mode` | `publish_later_queued`, `publish_later_drained`, `published_observed_authoritative` | `publish_later_queue_item_ref` |
| `inspect_only_mode` | `inspect_only_imported`, `offline_unverified`, `revoked_at_provider`, `disagrees_with_local`, `published_observed_authoritative` | `imported_snapshot_ref` or `local_draft_ref` |

## Degraded action vocabulary

When the provider is stale, offline, revoked, or disagrees with the local
model, every row names a typed
[`DegradedActionClass`](../../../crates/aureline-provider/src/object_model/mod.rs):

- `continue_local_authoring`
- `queue_publish_later`
- `open_in_provider_browser_handoff`
- `export_evidence_packet`
- `hold_for_freshness_repair`
- `hold_for_reauth`
- `hold_for_rescope`
- `none_required` — only admitted when the row is `fresh` and the publish
  state is not in the closed-mutation family.

## Continuity observations

Continuity observations are the contract that keeps offline or stale provider
state from collapsing the workflow:

| Observation class | When it fires |
| --- | --- |
| `provider_offline` | Provider unreachable |
| `provider_stale_within_window` | Truth older than the freshness floor but inside the review window |
| `provider_expired_beyond_window` | Truth past the freshness window; refresh required |
| `provider_revoked_or_disconnected` | Grant revoked or disconnected |
| `provider_disagrees_with_local` | Provider truth disagrees with the local model |
| `provider_never_observed` | Provider has never been observed for this row |

Each observation names a typed
[`RetainedCapabilityClass`](../../../crates/aureline-provider/src/object_model/mod.rs):

- `local_draft_authoring_retained`
- `publish_later_queuing_retained`
- `inspect_only_retained`
- `browser_handoff_offered`
- `no_capability_retained` — admitted only when the workflow truly cannot
  proceed; it cannot pair with `local_editing_preserved=true` or with
  `continue_local_authoring`.

`silent_mutation_authority_widened` must remain `false` on every
observation — provider truth never silently widens mutation authority back
to the upstream provider.

## Validator invariants

[`ProviderObjectModelAlphaPage::validate`](../../../crates/aureline-provider/src/object_model/mod.rs)
enforces:

- `record_kind`, `schema_version`, and `shared_contract_ref` match the alpha
  constants on every page, row, and continuity observation;
- row ids and observation ids are unique within a page;
- every consumed upstream contract ref is non-empty;
- `provider_family` and `object_kind` are mutually admissible;
- `mode` and `publish_state` are mutually admissible, and the mode's required
  ref is present;
- `local_draft_only` rows live in `local_draft_mode` and a local-only publish
  state; `offline_unverified_capture` rows never claim fresh freshness;
- `imported_snapshot` rows cite an `import_session_ref`;
- rows whose publish state holds mutation closed or whose freshness is
  degraded name a non-`none_required` `degraded_action`;
- continuity observations reference an existing row, name a non-empty
  rationale, never silently widen mutation authority, and respect the
  `no_capability_retained` guardrails;
- the page covers code-host, issue, and CI provider families and the four
  required modes: `local_draft`, `publish_later`, `open_in_provider`,
  `inspect_only`;
- `raw_payload_refs_present` is `false` on every row;
- `local_editing_preserved` is `true` on every row.

The
[`ProviderObjectModelAlphaSupportExport`](../../../crates/aureline-provider/src/object_model/mod.rs)
projection drops action refs (`approval_ticket_ref`,
`browser_handoff_packet_ref`, `imported_snapshot_ref`, `local_draft_ref`,
`publish_later_queue_item_ref`) and any raw provider payloads so support
bundles inherit the redaction posture without per-surface review.

## Reviewer fixture

The protected fixture
[`/fixtures/providers/provider_object_alpha/page.json`](../../../fixtures/providers/provider_object_alpha/page.json)
covers:

- code-host pull-request rows in `publish_now_mode`, `open_in_provider_mode`,
  and `local_draft_mode`;
- a branch row projected from a cached provider overlay;
- an issue/work-item row in `publish_later_mode` with a stale-within-window
  continuity observation;
- CI pipeline-run, check-run, pipeline-log, pipeline-artifact, and
  pipeline-annotation rows in `inspect_only_mode` exercising the
  `expired_beyond_window`, `offline_unverified`, and `revoked_at_provider`
  publish states with matching continuity observations.

Run the fixture validator:

```bash
cargo test -p aureline-provider --test provider_object_alpha
cargo run -p aureline-provider --bin aureline_provider_alpha -- \
    --provider-object-alpha --validate-only
```

## Out of scope

This alpha lane lands the typed record family and the first consumer (the
provider crate plus its alpha bin, fixture, and reviewer doc). Wiring the
record into the workspace start center, runtime task graph, review pack
inspector, and git change-lineage surfaces is later work; this lane keeps
the contract honest before those consumers expand.
