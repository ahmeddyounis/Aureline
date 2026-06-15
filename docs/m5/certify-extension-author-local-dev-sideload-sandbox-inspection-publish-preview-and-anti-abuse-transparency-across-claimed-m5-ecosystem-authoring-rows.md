# M5 evidence pointer — author-side certification across local-dev, sideload, sandbox inspection, publish preview, and anti-abuse transparency

Reviewer contract for the canonical M5 author-side certification that decides whether each
marketed M5 ecosystem family's author lane still backs the end-user install claim it
advertises. It rolls the author drills — local-dev workspace, sideload review,
sandbox/runtime inspection, publish preview, hot-reload and last-loaded-build continuity,
and anti-abuse transparency — into one qualification decision per claimed authoring row and
narrows the marketed row automatically when author-side trust or publish truth is weaker
than the install claim. This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-author-certification.json`
- Boundary schema: `schemas/ecosystem/m5-author-certification.schema.json`
- Reviewer contract: `docs/m5/certify-extension-author-local-dev-sideload-sandbox-inspection-publish-preview-and-anti-abuse-transparency-across-claimed-m5-ecosystem-authoring-rows.md`
- Human-readable rendering: `artifacts/m5/certify-extension-author-local-dev-sideload-sandbox-inspection-publish-preview-and-anti-abuse-transparency-across-claimed-m5-ecosystem-authoring-rows.md`
- Overview companion: `docs/ecosystem/m5/m5-author-certification.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-author-certification/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_author_certification/`

## Aggregates the author lanes and the install claim

The packet is the author-side counterpart to the install-side certification
(`artifacts/ecosystem/m5/m5-ecosystem-certification.json`). Each entry resolves through a
real author-and-publish-preview matrix row
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`) and a real install-side
certification entry, and reuses the closed artifact-family, source-class, runtime-class,
host/ABI, signing-state, workspace-origin, registry-binding, trust-posture, support-class,
evidence-freshness, and publish-readiness vocabulary frozen by those lanes — one entry per
marketed family — rather than minting a parallel set. Each entry carries one evidence record
for every author drill lane, so a row can never be certified by running a subset of the
author drills.

## What the certification proves

- **Trust is recomputed and never inherited.** The effective trust posture is the weakest of
  the declared posture, the signing-state ceiling, the workspace-origin ceiling, and the
  registry-binding ceiling. An unsigned local-dev build (`local_model_pack`), a **signed**
  recipe built in a local-dev workspace (`signed_recipe_pack`), an unsigned side-load
  (`side_loaded_package`), and a revoked mirror variant (`mirrored_registry_variant`) all
  render `unsigned_local_only`, proving a package never inherits a verified/enterprise badge
  just because the machine holds a trusted key.
- **The marketed row narrows automatically.** When the author-side ceiling lands below the
  end-user install claim, the entry records `author_claim_below_install_claim` and applies a
  downgrade. `local_model_pack` narrows a community install claim to unsupported because its
  local-dev build caps trust to local-only. The author claim may never exceed the install
  claim it guards.
- **Publish preview stays a review, not a linter.** Blocker/warning truth flows from the
  publish gate: `docs_pack` is `conditionally_certified` on disclosed warnings, while
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`, and
  `side_loaded_package` are `uncertified` because the publish gate is blocked, and
  `mirrored_registry_variant` is `uncertified` on a quarantine hold.
- **Widening hot reloads force a fresh review.** A reload that would widen permissions
  (`signed_recipe_pack`), the runtime class (`template_artifact`), or add an external
  executable (`bridge_backed_package`) raises a `fresh_review_required` lane and, through a
  blocked publish gate, an uncertified row.
- **The unhappy paths are exercised, not skipped.** Stale evidence (`template_artifact`), a
  failed build and a missing anti-abuse lane (`side_loaded_package`), and a quarantine
  withholding (`mirrored_registry_variant`) all appear, so the certification is not a
  public-registry happy-path-only proof.

## Narrowing / cross-check

- `M5AuthorCertification::validate` recomputes every entry's certification signals,
  disposition, effective trust posture, effective author support class, and downgrade path;
  a checked-in packet that drifts fails the gate.
- A board-level cross-check proves no entry renders a stronger trust posture than the
  author-and-publish-preview publish gate grants the same family, so the author certification
  and the publish preview project one trust truth.
- Downstream surfaces — local authoring surfaces, marketplace badges, diagnostics, support,
  and release evidence — consume `export_projection()` (a certification index plus a flat
  downgrade report) rather than cloning status text.
