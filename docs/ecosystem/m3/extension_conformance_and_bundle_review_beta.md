# Extension conformance, compatibility, and bundle review (beta)

Extension validation already produces a lot of truth: the conformance kit emits
passed/failed checks, the publication and registry lanes carry signer,
provenance, and compatibility metadata, and the mirror-import baseline keeps
source and trust-claim state aligned. Until now that truth lived in opaque CLI
logs, registry metadata, or per-lane records. This beta surface turns it into
**inspectable report records** that authors, reviewers, and registry operators
read before a publish, install, or side-load/offline decision — instead of
collapsing everything into "invalid package".

The evaluator is `aureline_extensions::conformance_reports`. The machine-readable
contracts are
[`/schemas/extensions/conformance_report.schema.json`](../../../schemas/extensions/conformance_report.schema.json)
and
[`/schemas/extensions/mirror_bundle_review.schema.json`](../../../schemas/extensions/mirror_bundle_review.schema.json).
Every consuming surface — author dashboards, reviewer packets, registry
moderation, support exports, and CI — reads one of these records by reference
rather than re-deriving validation locally.

## One severity and lifecycle vocabulary

All three reports share one vocabulary so a single word means the same thing in
authoring, install review, marketplace facts, and support packets:

- **Severity** (`ReviewSeverityClass`): `blocker` (must fix before publish or
  install), `warning` (recommendation), `info`. This mirrors the conformance-kit
  and manifest-editor severities; `ReviewSeverityClass::from_manifest_editor`
  binds them directly.
- **Check status** (`ReviewCheckStatusClass`): `pass`, `fail`, `warn`,
  `not_applicable`.
- **Lifecycle** (`ReviewLifecycleClass`): `preview`, `beta`, `stable`,
  `deprecated`, `removed`, `limited`, `revoked`. `from_catalog` and
  `from_marketplace_badge` map the catalog and marketplace lifecycle states into
  the same words.

Reviewers can always distinguish a publish blocker from a recommendation without
reading raw validator logs: the report carries explicit `blockers` and
`recommendations` counts and a decision class derived from them.

## Conformance + compatibility report

`build_conformance_report` produces an `ExtensionConformanceReport`. It carries:

- **Checks.** Each check has a stable `check_id`, `suite`, `title`, `status`,
  `severity`, `message`, and optional `field` anchor, `required_fix`,
  `repro_guidance` (steps or how to reproduce), `evidence_refs` (screenshots,
  logs), and `docs_url`. A failed blocker check must carry a required fix.
- **Compatibility section.** The target Aureline version range, SDK line, bridge
  state, **deprecated APIs** (replacement, removal horizon, migration impact,
  severity), **required shims** (what they cover, target version range,
  severity), and an overall migration-impact summary. This shows required shims
  or deprecations *before* publish or install — not after a failure.
- **Decision.** `publish_ready`, `publish_ready_with_recommendations`, or
  `blocked_on_conformance`, with a typed reason. Blocking deprecated APIs (e.g.
  an already-removed world) and blocking required shims count toward the blocker
  total, so compatibility cannot be silently green while it is actually blocked.

## Mirror / offline bundle review

`build_mirror_bundle_review` produces a `MirrorBundleReview`. The side-loaded,
mirrored, and offline path is a **first-class supported surface**, not an
undocumented exception. It carries:

- **Artifact identity.** Delivered and origin content hashes; an identity
  mismatch refuses the bundle.
- **Signing & provenance.** Signature and provenance posture, signer, and an
  optional transparency-log ref. Signing/provenance state is evaluated and
  rendered **independently of compatibility**: a missing signature or missing
  provenance refuses the bundle even when every compatibility check is green. A
  gap can never be hidden behind a compatibility green check.
- **Source.** The delivery route (primary catalog, approved mirror, offline
  bundle, manual artifact), the registry source class, a human-readable source
  label, and an optional mirror/offline origin ref.
- **Dependency graph.** Each dependency with its resolution state
  (`resolved`, `resolved_downgraded`, `unresolved`, `mismatched`), source class,
  optional content address, and notes. An unresolved or mismatched dependency
  refuses the bundle.
- **Reproducibility notes.** The reproducibility posture, optional build
  provenance and rebuild-instructions refs, and plain-language notes.
- **Decision.** `ready_for_sideload`, `ready_with_downgrades` (identity, signing,
  and provenance preserved but trust claims, dependencies, or reproducibility
  are visibly downgraded), `awaiting_admin_review` (a manual artifact needs an
  out-of-band verification receipt), or `refused`.

## Export as Markdown and JSON

Every report serializes to JSON (the schema-validated record) and renders to
reviewer-facing Markdown:

- `render_conformance_report_markdown`
- `render_mirror_bundle_review_markdown`
- `build_review_export_bundle` joins either or both reports into one
  `ReviewExportBundle` that carries the combined Markdown plus the embedded JSON
  records, so the same artifact attaches to issue reports, review packets,
  release evidence, and partner evaluations, and remains stable enough to feed CI
  and registry moderation.

## Guardrails

- Side-loaded and mirrored artifacts are documented, first-class review surfaces
  — never second-class undocumented exceptions.
- Signing/provenance gaps are surfaced and refused on their own; a green
  compatibility section never hides them. `validate_mirror_bundle_review` fails
  any record that continues the install lane with a signing or provenance gap.
- This surface is the beta review/proof lane only. It does not build marketplace
  ranking, sales, or analytics tooling.

## Fixtures

Replayable fixtures live under
[`/fixtures/extensions/m3/conformance_reports/`](../../../fixtures/extensions/m3/conformance_reports/):
publish-ready, recommendations-only, and blockers-present conformance reports;
and offline-ready, mirror-downgraded, and signing-gap-refused bundle reviews.
