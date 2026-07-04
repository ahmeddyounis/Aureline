# M5 manifest-authoring primitive contract

This packet narrows three families of the frozen
[M5 manifest / build component matrix](./m5_manifest_build_component_matrix.md)
— the manifest-editor header, the schema / validator row, and the
target-context chip group — plus the apply-review banner they imply into
one working primitive with a real **resolver**. A single manifest-authoring
context resolves onto four surfaces that share one authoring identity and one
truth class, so config-authoring surfaces are truthful *before* users validate,
preview, or mutate live infrastructure.

The primitive is minted and validated in
[`crates/aureline-infra`](../../crates/aureline-infra/src/implement_the_m5_manifest_editor_schema_validator_and_target_context_apply_review_primitive/mod.rs)
(`record_kind = m5_manifest_authoring_primitive`, `schema_version = 1`).
The Rust resolver `resolve_manifest_authoring()` and the builder
`seeded_m5_manifest_authoring_packet()` are the source of truth; the
checked-in artifacts below are byte-for-byte emissions of the builder
(`current_stable_m5_manifest_authoring_export()` re-reads the support export
via `include_str!`).

If this doc, the machine-readable schema, and the checked-in artifacts
disagree, the schema plus the Rust builder win and all companion artifacts
update in the same change.

## The resolver

`resolve_manifest_authoring(&M5ManifestAuthoringInput)` projects one
authoring context onto:

- **`M5ResolvedManifestHeader`** — file / artifact identity, source type,
  truth class, schema freshness, edit posture, execution origin, and the
  preview / apply entry points.
- **`M5ResolvedSchemaValidatorRow`** — schema source, version / snapshot-date
  label, freshness, validator verdict, whether it blocks apply, a policy /
  offline note, and the open-docs action.
- **`M5ResolvedTargetContextChips`** — the cluster / project / namespace /
  account chips, the target-identity ref, and whether the context is complete;
  the chip group stays visible as the surface scrolls.
- **`M5ResolvedApplyReviewBanner`** — the create / update / delete counts where
  known, dry-run availability, rollback / checkpoint posture, whether apply is
  available, and — when it is not — a precise, reconstructable blocked reason.

All four carry the same `authoring_id`; the header, chips, and banner disclose
the same truth class.

## Acceptance criteria

- **AC1 — environment and schema source are never hidden.** Every projection
  carries the target-identity ref and an explicit schema source kind. The
  resolver refuses an empty target-identity ref and an apply / write posture
  against an unresolved target context.
- **AC2 — desired / rendered / live / preview / apply state stays explicit
  before mutation.** The apply-review banner discloses the truth class,
  mutation counts, dry-run availability, and rollback posture, and never offers
  an apply until the target is resolved, the validator permits it, no dry-run
  policy block applies, and no active narrowing (drift, connector loss, policy
  block) is in effect. Mutation counts are refused on a surface with no write
  path.
- **AC3 — schema / validator freshness is visible wherever a manifest is
  trusted.** The header and the validator row always carry the same schema
  freshness; a writable manifest may not claim an editable posture with no
  resolvable schema.

## Reused and minted vocabulary

The primitive **reuses** the frozen matrix vocabulary rather than restating it:
`TruthMode`, `M5SchemaFreshness`, `M5SchemaValidationState`,
`M5ManifestEditPosture`, `M5ManifestBuildDowngradeTrigger`, and `DegradedState`.

It **mints** only the authoring-specific vocabulary:
`M5ManifestAuthoringSurfaceFamily` (6 parity surfaces), `M5ManifestSourceType`,
`M5ExecutionOrigin`, `M5SchemaSourceKind`, `M5DryRunAvailability`,
`M5RollbackPosture`, and `M5ManifestAuthoringExportField`.

## Companion artifacts

- [`/schemas/ui/m5-manifest-authoring-primitive.schema.json`](../../schemas/ui/m5-manifest-authoring-primitive.schema.json)
  — boundary schema for the primitive packet, its surface rows, worked
  authoring cases (input + resolved), vocabulary set, governance review,
  consumer projection, and release posture.
- [`/artifacts/release/m5-manifest-authoring-primitive-proof/support_export.json`](../../artifacts/release/m5-manifest-authoring-primitive-proof/support_export.json)
  — the `include_str!` canonical support export (release / support proof).
- [`/artifacts/release/m5-manifest-authoring-primitive-proof/matrix.csv`](../../artifacts/release/m5-manifest-authoring-primitive-proof/matrix.csv)
  — machine-readable per-surface CSV.
- [`/artifacts/release/m5-manifest-authoring-primitive-proof/report.md`](../../artifacts/release/m5-manifest-authoring-primitive-proof/report.md)
  — human-readable Markdown report.
- [`/fixtures/ui/m5-manifest-authoring-primitive/`](../../fixtures/ui/m5-manifest-authoring-primitive/)
  — protected fixtures (byte-identical copies of the support export and CSV).

The fixture-emitting bin is
`crates/aureline-infra/src/bin/emit_manifest_authoring_primitive_fixture.rs`
(`support` | `csv` | `summary`); its `support` output is the byte-for-byte
`include_str!` canonical.

## Privacy boundary

Raw manifest bodies, file contents, credentials, connector tokens, and endpoint
data never cross this boundary. The packet carries only opaque refs, typed class
tokens, booleans, redacted labels, and non-negative counts, so support and
diagnostics exports reconstruct exactly what a surface would have shown without
leaking source or live payloads.
