# M5 Pipeline / Dependency / Finding Component Matrix

Record kind: `m5_pipeline_dependency_finding_component_matrix`
Schema version: `1`
Status: frozen for M5 first consumers

This matrix freezes Aureline's reusable row and card contracts for external
pipeline state, diagnostic annotations, dependency updates, manifest changes, and
security findings. Review panes, package-manager views, project-health centers,
support exports, release proof, and narrow companion clients consume these
objects by reference instead of cloning field lists or inventing local status
wording.

The matrix is metadata-only. Components carry stable ids, opaque source refs,
controlled labels, explicit freshness/suppression/degraded states, and
copy/export-safe summaries. They do not carry raw provider payloads, raw logs,
raw local paths, credentials, raw advisory bodies, exploit details, or private
tenant/user identifiers.

## Source Bindings

| Component family | Canonical sources consumed by reference | First consumers |
| --- | --- | --- |
| Pipeline run row | `artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md`, `schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`, `aureline_review::current_pipeline_viewer_export` | Review pane, pipeline viewer, project-health CI center, companion CI summary, support export, release proof |
| Annotation row | `schemas/review/ship-ai-review-evidence-finding-cards-and-review-pack-integration-with-change-objects.schema.json`, `artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects.md`, normalized diagnostic records | Review pane, diagnostics panel, project-health center, support export, release proof |
| Dependency row | `artifacts/deps/m5/package-set-inventory-and-scope-truth.json`, `artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json`, `schemas/deps/package-review-cross-surface-integration.schema.json` | Package manager, review pane, framework-pack health, project-health dependencies, companion inspect, support export |
| Manifest diff card | `artifacts/deps/m5/grouped-update-and-rollback-review.json`, `artifacts/deps/m5/manifest-scope-review.json`, `artifacts/deps/m5/reviewed-mutation-flows.json`, package mutation operation history | Package manager, review stage, project-health remediation, support export, release proof |
| Security finding card | `artifacts/deps/m4/dependency-security-compliance-export-truth.json`, `docs/security/m5_advisory_card_row_primitive_contract.md`, `schemas/security/m5-advisory-card-row.schema.json`, normalized security result packets | Package health, review pane, project-health security center, incident/support export, release proof |

## Controlled Labels

| Vocabulary | Values |
| --- | --- |
| `consumer_surface` | `review_pane`, `pipeline_viewer`, `package_manager`, `project_health_center`, `framework_pack_health`, `companion_client`, `support_export`, `release_proof` |
| `freshness_state` | `current`, `live`, `warm_cached`, `stale`, `superseded`, `partial`, `blocked`, `policy_hidden`, `expired`, `no_fix_yet`, `unknown` |
| `degraded_state` | `none`, `stale`, `superseded`, `partial`, `blocked`, `policy_hidden`, `no_fix_yet`, `provider_unreachable`, `evidence_missing`, `rollback_unavailable`, `advisory_feed_stale`, `manifest_scope_unknown` |
| `suppression_state` | `unsuppressed`, `suppressed_until_review`, `suppressed_by_policy`, `exception_expired` |
| `severity` | `info`, `low`, `medium`, `high`, `critical`, `blocking` |
| `confidence` | `confirmed`, `high`, `medium`, `low`, `unknown` |
| `copy_format` | `text`, `json`, `markdown` |

## Component Field Sets

### Pipeline Run Row

Required fields:

| Field | Contract |
| --- | --- |
| `row_id` | Stable reusable row id preserved across review, project-health, companion, support, and release surfaces. |
| `pipeline_run_id` | Canonical run id from the normalized pipeline packet; never a surface-local id. |
| `provider_run_ref`, `provider_label`, `workflow_or_job_name` | Opaque provider/run identity, controlled provider label, and workflow/job name. |
| `review_anchor_ref` | Durable review/change anchor attached to the run. |
| `trigger` | Trigger type, actor ref, actor class, and trigger event ref shown separately from status. |
| `duration` | Exact, approximate, or unknown duration disclosure with display label. |
| `branch_change_relation` | Branch ref, commit ref, change object ref, base relation, and stale-base/superseded flag. |
| `normalized_status` | Provider-normalized run status; unknown, action-required, cancelled, timed-out, and blocked are first-class. |
| `artifact_summary` | Artifact count, log count, unavailable count, and artifact browser ref. |
| `freshness_state`, `freshness_note`, `degraded_state` | Explicit current/stale/superseded/partial/blocked/policy-hidden state and stale/superseded note. |
| `run_control_authority` | Rerun/cancel availability, acting identity requirement, side-effect review requirement, and disabled reason. |
| `open_details_action`, `provider_handoff` | In-product details action plus provider-native handoff bar when richer provider context is required. |
| `copy_export` | Text, JSON, and Markdown copy carrying run id, provider, workflow/job, trigger actor class, branch/change relation, duration, artifact count, freshness, provider handoff, and rerun/cancel authority. |

Degraded states:

- `stale`
- `superseded`
- `partial`
- `blocked`
- `policy_hidden`
- `provider_unreachable`

### Annotation Row

Required fields:

| Field | Contract |
| --- | --- |
| `row_id` | Stable row id reused by review, diagnostics, project health, and support export. |
| `annotation_id`, `diagnostic_id` | Canonical annotation and diagnostic ids. |
| `anchor_ref` | Durable file, symbol, change, package, or run anchor; raw paths stay outside the component. |
| `source_packet_ref` | Opaque diagnostic/review packet ref. |
| `annotation_kind` | Diagnostic, review finding, policy finding, build annotation, or package annotation. |
| `severity`, `confidence`, `freshness_state` | Severity, confidence, and freshness rendered as separate fields. |
| `suppression_state` | One of `unsuppressed`, `suppressed_until_review`, `suppressed_by_policy`, or `exception_expired`; suppressed rows remain visible. |
| `remediation` | Remediation action, fix availability, owner ref, and due/review ref; no-fix-yet stays explicit. |
| `copy_export` | Text, JSON, and Markdown copy preserving diagnostic id, anchor, severity, confidence, freshness, suppression, and remediation. |

Degraded states:

- `stale`
- `superseded`
- `partial`
- `policy_hidden`
- `evidence_missing`
- `no_fix_yet`

### Dependency Row

Required fields:

| Field | Contract |
| --- | --- |
| `row_id` | Stable row id. |
| `package_identity_ref`, `resolved_identity_ref` | Package coordinate and resolved exact identity from package-set truth. |
| `manifest_identity` | Manifest ref, scope kind, ecosystem, and owning workset/workspace ref. |
| `dependency_relation` | `direct`, `transitive`, `workspace_local`, `path`, or `vcs`; direct/transitive truth is never hidden. |
| `version_delta` | Current, target, requested range, delta class, and lockfile authority. |
| `advisory_summary` | Advisory count, highest severity, advisory freshness, and suppression counts. |
| `license_action`, `changelog_action` | Separate license and changelog actions with refs and availability. |
| `freshness_state`, `degraded_state` | Registry/advisory/package freshness and explicit stale/partial/blocked/policy-hidden state. |
| `copy_export` | Text, JSON, and Markdown copy preserving package identity, manifest scope, relation, version delta, advisory count, license action, and changelog action. |

Degraded states:

- `stale`
- `partial`
- `blocked`
- `policy_hidden`
- `advisory_feed_stale`
- `manifest_scope_unknown`

### Manifest Diff Card

Required fields:

| Field | Contract |
| --- | --- |
| `card_id` | Stable card id. |
| `manifest_diff_id`, `operation_ref` | Canonical diff/operation ids from package mutation governance. |
| `manifest_identity` | Manifest ref, scope kind, ecosystem, and owning workspace/workset ref. |
| `change_summary` | Dependency, lockfile, scripts/hooks, peer/runtime constraint, and metadata change counts. |
| `scripts_hooks_preview` | Added/removed/changed scripts and hooks preview with policy labels. |
| `constraint_changes` | Peer and runtime constraint changes, compatibility posture, and affected package refs. |
| `checkpoint_state`, `rollback_state` | Checkpoint and rollback refs, availability, and boundary. |
| `apply_boundary` | Write authority, staged/direct mutation posture, review requirement, policy boundary, and disabled reason. |
| `freshness_state`, `degraded_state` | Explicit current/stale/partial/blocked/policy-hidden/rollback-unavailable state. |
| `copy_export` | Text, JSON, and Markdown copy preserving diff id, manifest scope, hooks/constraints, checkpoint/rollback, and apply boundary. |

Degraded states:

- `stale`
- `partial`
- `blocked`
- `policy_hidden`
- `rollback_unavailable`
- `manifest_scope_unknown`

### Security Finding Card

Required fields:

| Field | Contract |
| --- | --- |
| `card_id` | Stable finding card id. |
| `finding_id`, `security_result_packet_ref`, `advisory_ref` | Canonical finding id, security result packet ref, and advisory/ref id. |
| `affected_object_ref` | Affected package, manifest, pipeline artifact, or install surface. |
| `severity`, `confidence`, `freshness_state` | Severity, confidence, and freshness rendered separately. |
| `suppression_state` | Suppression vocabulary remains visible: `unsuppressed`, `suppressed_until_review`, `suppressed_by_policy`, or `exception_expired`. |
| `remediation` | Fix version/action, mitigation, owner, no-fix-yet flag, and blocked reason. |
| `exposure_summary` | Exposure state, affected surfaces, exploitability label, and affected manifest refs. |
| `copy_export` | Text, JSON, and Markdown copy preserving finding id, result packet, advisory, severity, confidence, freshness, suppression, exposure, and remediation. |

Degraded states:

- `stale`
- `superseded`
- `partial`
- `blocked`
- `policy_hidden`
- `no_fix_yet`
- `advisory_feed_stale`

## Copy / Export Invariants

Every component family must offer text, JSON, and Markdown copy. Copy/export
payloads must preserve controlled labels exactly as rendered in the UI and must
include source refs sufficient to reconstruct the object without a screenshot.
`screenshot_only_prohibited` is always true.

## Suppression Invariants

Suppression never deletes a row or card. Suppressed annotations and security
findings must remain visible and exportable as one of:

- `unsuppressed`
- `suppressed_until_review`
- `suppressed_by_policy`
- `exception_expired`

Suppression state is separate from severity, confidence, freshness, and
remediation. A critical finding suppressed by policy still exports as critical,
with policy suppression visible.

## Consumer Projection Rules

- Review panes may narrow mutation authority but cannot rename statuses or hide
  suppressed/stale rows.
- Package-manager views may expose apply actions only when the card's
  `apply_boundary.write_authority` allows it.
- Project-health centers are read-only aggregators and must preserve run,
  diagnostic, manifest, package, and security packet ids.
- Companion clients may be inspect-only but must keep the same field labels,
  degraded states, and copy/export payloads.
- Support export and release proof must include the same controlled labels as
  the primary UI, not generic prose.

## Narrowing Rules

Rows/cards narrow instead of disappearing when evidence is stale, partial,
superseded, blocked by policy, hidden by policy, unavailable, or no-fix-yet.
Promotion is blocked until the component has a conforming fixture, schema, source
binding, consumer projection, copy/export parity, and proof freshness row.
