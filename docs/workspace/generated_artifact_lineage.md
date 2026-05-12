# Generated-artifact lineage hints and source-canonical boundary cues

This document is the reviewer-facing entry point for the generated-artifact
lineage truth that explorer and search surfaces project. It does **not**
re-derive lineage truth — it documents the canonical model implemented in
`crates/aureline-workspace/src/generated_artifacts/` so explorer, search,
quick-open, and docs/help readers see the same vocabulary the runtime emits.

## Why this exists

Lockfiles, build outputs, and generated source siblings tend to *look* like
canonical sources at a glance — same extensions, same trees, same syntax.
Without a lineage cue, a user can edit a generated copy thinking they are
editing the source of truth and lose work the moment a build regenerates
the artifact.

The workspace therefore publishes a small, narrow lineage record next to
each generated artifact, and explorer/search surfaces render it as a
distinct cue. The model only covers cases the workspace can detect *safely*
from a relative path: there is **no build-graph reconstruction here**, no
filesystem read inside the detector, and no semantic lineage inference.

## Truth rows

Every projected hint carries the following truth rows:

| Row | Token vocabulary | Notes |
| --- | --- | --- |
| Generated class | `lockfile`, `build_output`, `generated_source_sibling`, `vendored_snapshot` | One token per record. Surfaces MUST render the badge directly. |
| Source-canonical pointer | workspace-relative path or `null` | When present, surfaces offer a "go to canonical" pivot. When `null`, the row stays generated-without-canonical (e.g. build outputs). |
| Producer identity | `producer_id` + `producer_label` | Frozen at rule-build time so logs and a11y exports cite the same producer. |
| Freshness class | `derived_from_canonical`, `possibly_stale`, `snapshot_only`, `unknown` | Surfaces MUST surface the same token. They MUST NOT collapse `possibly_stale` and `snapshot_only` into a generic "stale" badge. |
| Explainer | short string | Suitable for tooltips, hover, and a11y exports. |
| Rule id | stable string | Lets fixtures and support bundles cite the rule that fired without re-deriving truth. |

## Failure drill

When the detector returns no hint, surfaces MUST keep the row visible exactly
as they did before. The absence of a hint is **not** a license to invent one
or to relabel the row as canonical — it just means the workspace cannot
confirm lineage from the path alone.

## Catalogued cases

The seeded catalog covers the cases the M1 spec calls out:

- **Lockfiles** — `Cargo.lock`, `package-lock.json`, `yarn.lock`,
  `pnpm-lock.yaml`, `poetry.lock`, `Pipfile.lock`. Each lockfile points back
  at its co-located manifest (e.g. `Cargo.lock` → `Cargo.toml`), even when
  the lockfile lives in a nested workspace directory.
- **Build outputs** — `target/`, `dist/`, `build/`. These carry no
  source-canonical pointer because they have no single canonical sibling.
- **Vendored snapshots** — `node_modules/`, `vendor/`. These are flagged
  `snapshot_only`; surfaces should pivot to upstream rather than the local
  copy.
- **Generated source siblings** — `*.gen.rs`, `*_pb.rs`, `*.generated.ts`.
  These point back at the canonical sibling (e.g. `api.gen.rs` → `api.rs`).

## Where the truth lives

| Surface | Consumer |
| --- | --- |
| Explorer node | `aureline_shell::explorer::GeneratedArtifactHint` projects the lineage record onto the node hint, including a `aureline-ws://…` URI for the canonical sibling when the workspace and root are known. |
| Lexical search row | `aureline_search::ResultRow::generated_artifact_hint` carries the same `LineageHintRecord` so search rows label generated lanes distinctly. |
| Palette search card | `aureline_shell::palette::WorkspaceSearchSurfaceLineageHint` projects the row hint into chrome-friendly labels (badge, freshness label, explainer, source-canonical pointer). |

## Browser handoff and freshness

The lineage truth itself does not own the docs/help browser handoff — the
shell's docs/help skeleton (`crates/aureline-shell/src/docs_browser/`) owns
that contract. Surfaces that need to *open* this document in the in-product
docs/help browser must do so through that skeleton: the freshness label,
client-scope row, and browser-handoff packet stay aligned with the existing
embedded-boundary contract instead of being re-minted here.

## Fixtures

The lineage truth ships with deterministic fixtures under
`fixtures/workspace/generated_artifact_cases/*.json`. Each fixture lists a
set of workspace-relative paths and the records the catalog must emit (or
`expect_match=false` for the failure drill). The cases drive the integration
test `crates/aureline-workspace/tests/generated_artifact_cases.rs`.

## Out of scope

The lineage truth deliberately does NOT cover:

- end-to-end build-graph reconstruction;
- semantic lineage inference (e.g. tracing a generated row back through a
  multi-stage codegen pipeline);
- automatic rewrites or generated-file mutation;
- docs/help browser ownership — that belongs to the docs/help skeleton.
