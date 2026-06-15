# M5 evidence pointer — local extension workspace, sideload review, sandbox inspector, and publish preview

Reviewer contract for the canonical M5 author-side and publish-preview matrix that
maps each marketed M5 ecosystem artifact family to its runtime class, host/ABI,
local-workspace build state, signing state, trust posture, hot-reload posture,
publish-review requirement, conformance output, and anti-abuse transparency. This row
is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-author-and-publish-preview.json`
- Boundary schema: `schemas/ecosystem/m5-author-and-publish-preview.schema.json`
- Reviewer contract: `docs/m5/freeze-the-m5-local-extension-workspace-sideload-review-sandbox-inspector-and-publish-preview-matrix.md`
- Human-readable rendering: `artifacts/m5/freeze-the-m5-local-extension-workspace-sideload-review-sandbox-inspector-and-publish-preview-matrix.md`
- Overview companion: `docs/ecosystem/m5/m5-author-and-publish-preview.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-author-and-publish-preview/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_author_and_publish_preview/`

## Reuses the frozen install-governance matrix

The author lane is the counterpart to the end-user install lane. The packet reuses the
closed artifact-family vocabulary frozen in the sibling install-governance matrix
(`artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`) — one author row
per marketed family — rather than minting a parallel family set, so a framework pack,
docs pack, local-model pack, recipe pack, template, or bridge-backed/side-loaded
package is authored and installed through one trust model.

## What the gate proves

- **Non-inheritance.** A locally-built (`unsigned_local_dev`), side-loaded
  (`unsigned_sideload`), or revoked (`revoked_signature`) artifact publishes only as
  `unsigned_local_only`; it can never inherit a `verified_publisher` or
  `enterprise_approved` badge. The fixture exercises this with a local-model pack that
  declares verified-publisher and a mirrored variant that declares enterprise-approved,
  both capped to local-only.
- **Hot reload cannot widen authority silently.** A hot reload that widens the runtime
  class, adds an external executable, or expands permissions raises a blocking finding
  until a fresh review clears it.
- **Publish preview is not a manifest linter.** Findings are severity-tagged as
  `blocker` versus `warning`, and the `publish_review_requirement` keeps the
  registry-policy consequence explicit.
- **No silent withholding.** A quarantined family resolves to `withheld_quarantined`;
  a family with any blocker to `blocked_from_publish`; a family with only warnings to
  `publishable_with_warnings`; and a clean family to `ready_to_publish`.

## Executable proof

`crates/aureline-ecosystem/src/m5_author_and_publish_preview/tests.rs` loads the
embedded packet, asserts it validates with zero violations, proves every closed
vocabulary is exhaustively exercised, and asserts the non-inheritance, hot-reload, and
quarantine guardrails. `M5AuthorPublishMatrix::validate()` is the CI-facing gate that
flags any overstated trust posture, inherited badge, readiness/findings mismatch, or
summary drift.

## Freshness

The packet is current as of the `as_of` date embedded in the JSON artifact. CI gates
recompute the publish gate against the embedded rows and fail if the matrix is stale or
underqualified.
