# Workflow-bundle lifecycle and certification conformance (beta)

This is the beta contract for the M3 workflow-bundle lifecycle proof lane. It
turns workflow-bundle lifecycle behavior, dependency markers, and
archetype-certification truth into current, regression-gated evidence rather than
implementation-only intent, so the launch-wedge and imported-user bundle rows can
be claimed for beta.

## Boundary under proof

- Runtime model: [`aureline_workspace::bundles::WorkflowBundleReviewRecord`](../../../crates/aureline-workspace/src/bundles/mod.rs)
- Record boundary schema: [`/schemas/workspace/workflow_bundle_review.schema.json`](../../../schemas/workspace/workflow_bundle_review.schema.json)
- Corpus manifest schema: [`/schemas/workspace/workflow_bundle_conformance.schema.json`](../../../schemas/workspace/workflow_bundle_conformance.schema.json)

The `WorkflowBundleReviewRecord` is the workspace-owned review and projection
boundary every mutating bundle engine must pass through before durable state is
installed, updated, rebased, removed, or rolled back. It does not execute bundle
changes; it composes bundle detail, install/update preview, drift/override
review, remove/rollback review, certification truth, mirror/offline posture, CLI
parity, diagnostics, and support export into one validated, support-safe artifact.

## Corpus and harness

- Corpus: `fixtures/workspace/m3/workflow_bundle_lifecycle/`
- Harness: [`crates/aureline-qe/src/workflow_bundle_lifecycle/`](../../../crates/aureline-qe/src/workflow_bundle_lifecycle/)
- Replay: `cargo test -p aureline-qe --test workflow_bundle_lifecycle_conformance`

`manifest.json` is authoritative. Positive drills MUST parse, validate, project,
and match **every** `expected_*` field in the manifest. Negative drills MUST FAIL
validation with an error whose message contains `expected_failure_substring`. The
fixtures carry only the scenario records and a `$schema` prelude — they do **not**
restate the expectations, so there is exactly one place to read and audit the
pinned truth.

## What the corpus proves

### Source classes and lifecycle flows

The corpus keeps a drill for every source class — **Certified**, **Managed
approved**, **Community**, **Imported**, and **Local draft** — and proves diff
review across all twelve required install/update axes (extension sets, profile and
surface presets, settings/tokens, task/launch/debug recipes, docs and tour packs,
template references, migration mappings, and certification targets). Across the
positive set the drills exercise install, update, rebase/adopt, keep-local,
remove/rollback, drift-banner, mirror-only, and offline-install flows.

### Automatic badge downgrades

Badges downgrade automatically — never silently — when evidence, a dependency, or
a mirror is stale:

| Trigger | Effective result |
| --- | --- |
| Certification evidence past its window | `retest_pending` (Retest pending) |
| Stale evidence / lifecycle-sensitive dependency / stale mirror on a managed bundle | `limited` (Limited) |
| Community / design-partner support promise | `experimental` support class |

The runtime validator refuses to render a `certified` or `managed_approved`
effective badge on stale or retest-required evidence, so an over-claim is rejected
before a beta row hardens (negative drill
`negative.certified_badge_on_stale_evidence`).

### Dependency-marker propagation

Capability-dependency markers (for example `policy_gated`, `host_specific`,
`beta_only`, `labs`, `community_supported`) and lifecycle-sensitive dependencies
must propagate across surfaces. The harness asserts each capability marker appears
on the certification evidence, the install/update review sheet, **and** the
support/diagnostics export, and that each lifecycle-sensitive dependency appears
on both a drift row and the install/update sheet — so no consumer surface can
silently drop a marker. The imported-user round-trip keeps its capability markers
and certification provenance through review, drift, override, and remove flows.

### User-asset and rollback guarantees

Every claimed beta bundle row preserves user-created assets: any `user_owned`
removable asset stays `not_safe_to_remove_user_owned` and requires explicit
review, adopted-versus-created ownership stays explicit, and removal retains local
overrides. The install/update rollback checkpoint is reversible (carries a
checkpoint ref) and names the bundle-owned axes it restores, so a rollback
restores bundle-owned state without deleting unrelated local work.

### Trust, egress, policy, and approval guardrails

No bundle path may silently widen workspace trust, network egress, admin-policy
scope, or approval defaults, and provider/remote/template recommendations stay
recommendation-only. The harness fails if a guardrail widens (negative drill
`negative.guardrail_widens_workspace_trust`) and forbids any raw secret, raw user
content, or raw path export.

## Published evidence

- Certification freshness matrix: [`artifacts/cert/m3/workflow_bundle_certification_matrix.json`](../../../artifacts/cert/m3/workflow_bundle_certification_matrix.json) — one row per claimed beta bundle.
- Lifecycle compatibility report: [`artifacts/compat/m3/workflow_bundle_lifecycle_report.md`](../../../artifacts/compat/m3/workflow_bundle_lifecycle_report.md).

The conformance suite asserts that both artifacts cover every drill id and every
capability marker, so the published certification and compatibility truth cannot
drift from the corpus.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw secrets, private keys, credentials, raw local paths, hostnames,
command lines, logs, source content, and signing material never appear. The runner
scans each fixture for forbidden raw-content tokens before validation. Removing
any positive or negative drill without a replacement is a breaking contract change
for the `workspace.workflow_bundle_lifecycle.beta` corpus.
