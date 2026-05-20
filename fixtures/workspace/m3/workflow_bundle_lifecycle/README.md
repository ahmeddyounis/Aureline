# Workflow-bundle lifecycle conformance corpus

This corpus is the conformance, interoperability, certification-freshness, and
failure / recovery drill harness for the M3 workflow-bundle lifecycle beta
boundary owned by
[`aureline-workspace::bundles`](../../../../crates/aureline-workspace/src/bundles/mod.rs)
(`WorkflowBundleReviewRecord`).

It converts the workflow-bundle UX promise into a regression-gated proof system:
each drill pins the lifecycle truth a claimed beta bundle row must reproduce — the
bundle/source/status/support classes, the effective badge after evidence,
dependency, and mirror checks, the support claim it may imply, the mirror/offline
posture, the granular drift / removal / override counts, the review and
drift-resolution actions, whether removal preserves user-owned assets, whether the
rollback checkpoint restores bundle-owned state, and the capability-dependency and
lifecycle-sensitive markers it must propagate.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/workflow_bundle_lifecycle/`](../../../../crates/aureline-qe/src/workflow_bundle_lifecycle/)
and replayed by
`cargo test -p aureline-qe --test workflow_bundle_lifecycle_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST parse, validate, project,
and match **every** `expected_*` field in the manifest. Negative drills MUST FAIL
validation with an error whose message contains `expected_failure_substring`. The
fixtures carry only the scenario records and a `$schema` prelude — they do **not**
restate the expectations, so there is exactly one place to read and audit the
pinned truth.

Boundary schemas, contract, and published evidence:

- Record schema: [`/schemas/workspace/workflow_bundle_review.schema.json`](../../../../schemas/workspace/workflow_bundle_review.schema.json)
- Manifest schema: [`/schemas/workspace/workflow_bundle_conformance.schema.json`](../../../../schemas/workspace/workflow_bundle_conformance.schema.json)
- Beta contract: [`docs/workspace/m3/workflow_bundle_lifecycle_conformance.md`](../../../../docs/workspace/m3/workflow_bundle_lifecycle_conformance.md)
- Certification freshness matrix: [`artifacts/cert/m3/workflow_bundle_certification_matrix.json`](../../../../artifacts/cert/m3/workflow_bundle_certification_matrix.json)
- Lifecycle compatibility report: [`artifacts/compat/m3/workflow_bundle_lifecycle_report.md`](../../../../artifacts/compat/m3/workflow_bundle_lifecycle_report.md)

## Coverage axes

| Axis | Drill id |
| --- | --- |
| Certified — full lifecycle, live or mirror | `certified.full_lifecycle_live_or_mirror` |
| Managed approved — mirror-only catalog | `managed_approved.mirror_only_update` |
| Community — offline install, experimental support | `community.offline_install_experimental` |
| Imported — round-trip, markers + user assets preserved | `imported.round_trip_preserves_markers` |
| Local draft — keep-local, no claim | `local_draft.keep_local_no_claim` |
| Certified — stale evidence downgrades to Retest pending | `certified.stale_evidence_retest_pending` |
| Managed approved — stale evidence/dependency/mirror downgrades to Limited | `managed_approved.stale_dependency_limited` |
| Community — capability/lifecycle dependency-marker propagation | `community.dependency_marker_propagation` |
| Negative — certified badge on stale evidence | `negative.certified_badge_on_stale_evidence` |
| Negative — removal marks a user-owned asset safe | `negative.removal_marks_user_asset_safe` |
| Negative — guardrail widens workspace trust | `negative.guardrail_widens_workspace_trust` |
| Negative — imported source over-claims certified | `negative.imported_overclaims_certified` |
| Negative — drift adopt skips the change-preview route | `negative.adopt_skips_change_preview` |
| Negative — install/update drops a required diff axis | `negative.install_missing_certification_axis` |
| Negative — support export enables raw secret export | `negative.support_export_enables_raw_secret` |

## Transverse invariants

The conformance suite also pins, across the whole positive set:

- every source class (`certified`, `managed_approved`, `community`, `imported`,
  `local_draft`) keeps a drill;
- the lifecycle flows (install, update, rebase/adopt, keep-local, remove/rollback,
  drift-banner, mirror-only, offline-install) all keep a drill;
- the mirror/offline postures (`live_origin_only`, `live_or_mirror`,
  `mirror_only`, `signed_offline_bundle`) all keep a drill;
- a certified bundle with stale evidence downgrades to Retest pending, a managed
  bundle with stale evidence/dependency/mirror downgrades to Limited, and a
  community/design-partner bundle carries the Experimental support promise;
- capability-dependency and lifecycle-sensitive markers propagate across the
  certification, install/update, and export surfaces;
- every claimed beta row preserves user-owned assets, restores bundle-owned state
  on rollback, allows no raw export, and passes its guardrails;
- the published certification freshness matrix and lifecycle compatibility report
  cover every drill id, so they cannot drift from the corpus.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw secrets, private keys, credentials, raw local paths, hostnames,
command lines, logs, source content, and signing material never appear. The runner
scans each fixture for forbidden raw-content tokens before validation. Removing any
positive or negative drill without a replacement is a breaking contract change for
the `workspace.workflow_bundle_lifecycle.beta` corpus.
