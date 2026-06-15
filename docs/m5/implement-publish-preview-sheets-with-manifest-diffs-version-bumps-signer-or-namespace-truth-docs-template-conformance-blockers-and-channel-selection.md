# M5 evidence pointer — publish-preview sheets with manifest diffs, version bumps, signer/namespace truth, docs/template/conformance blockers, and channel selection

Reviewer contract for the canonical M5 publish-preview sheet set that gives an author the
reviewed publish action for each marketed M5 ecosystem artifact family: the manifest diff,
the version bump, the signer and namespace truth, the release channel, and the per-gate
state of schema validation, the conformance kit, accessibility and performance smoke, docs
completeness, template/sample completeness, and registry policy — with release blockers
held distinct from warnings, each naming the gate it came from. This row is a depth-lane
proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-publish-preview.json`
- Boundary schema: `schemas/ecosystem/m5-publish-preview.schema.json`
- Reviewer contract: `docs/m5/implement-publish-preview-sheets-with-manifest-diffs-version-bumps-signer-or-namespace-truth-docs-template-conformance-blockers-and-channel-selection.md`
- Human-readable rendering: `artifacts/m5/implement-publish-preview-sheets-with-manifest-diffs-version-bumps-signer-or-namespace-truth-docs-template-conformance-blockers-and-channel-selection.md`
- Overview companion: `docs/ecosystem/m5/m5-publish-preview.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-publish-preview/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_publish_preview/`

## Materializes the matrix's publish-preview reference

The publish-preview sheet is the first-class object behind the author-and-publish-preview
matrix's single `publish_preview_ref`
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`). The packet reuses the closed
artifact-family, runtime-class, host/ABI, signing-state, trust-posture, hot-reload,
finding-severity, and publish-readiness vocabulary frozen by that lane — one sheet per
marketed family — rather than minting a parallel set, and each sheet links back to its
publish-gate row and forward to the release packet that consumes it.

## What the sheet proves

- **Blockers versus warnings stay explicit and named.** Each sheet splits findings into
  blockers (hard-stop) and warnings (publish-with-disclosure), and tags each with the gate
  it came from — schema validation, the conformance kit, accessibility/perf smoke, docs
  completeness, template/sample completeness, or registry policy — versus the structural
  manifest, version, signer, namespace, channel, or hot-reload facts. The fixture exercises
  all seven named gates and all five gate outcomes.
- **The version bump must cover the change.** A covered minor bump (framework pack), an
  undersized bump (docs pack), a missing bump (bridge package), a downgrade (side-loaded
  package), and an invalid version (mirrored variant) are all proven, with the bump sized
  against the largest change impact in the diff.
- **Widening forces a fresh review.** A manifest change or hot reload that widens
  permissions, the runtime class, or an external executable raises a blocking finding until
  it is freshly reviewed; the `local_model_pack` (runtime class + permissions) and the
  `bridge_backed_package` (external executable) prove it, so widening never reaches the
  registry through a hot reload alone.
- **Signer and namespace truth cap the badge.** The published badge is the minimum of the
  signing-state and namespace ceilings: a `signed_verified` recipe pack with a
  mid-transfer namespace is capped to registry-bound, and an unclaimed (side-loaded) or
  mismatched (mirrored) namespace, an unsigned side-load, and a revoked signature all
  publish unsigned-local-only.
- **Channel consequences are explicit.** The stable channel adds a blocker to a recipe pack
  that still carries warnings, and the beta channel adds a blocker to an unsigned-local-only
  mirrored variant, so namespace, signer, and channel consequences are never hidden.
- **Quarantine withholds.** The quarantined `mirrored_registry_variant` is withheld
  entirely, taking precedence over its individual blockers.

## Narrowing / cross-check

- The typed model recomputes the published trust posture, the readiness, and the full
  finding set (in canonical source/reason order) and the summary counts from the observed
  facts; a checked-in packet that drifts fails `M5PublishPreviewSheetSet::validate`.
- `M5PublishPreviewSheetSet::cross_check_matrix` proves every sheet publishes no stronger
  than the author-lane publish gate would grant the same family.
- Downstream surfaces consume `export_projection()` rather than cloning publish status text;
  each sheet carries a `release_packet_ref` so release packets reference the preview object
  directly.
