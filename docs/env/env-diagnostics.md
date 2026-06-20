# Environment-artifact diagnostics, import/export, and compare

This document describes the mirror/offline import-export, schema-version
compare, and materialization-diagnostics lane for environment artifacts. The
canonical implementation is
[`crates/aureline-env/src/env_diagnostics/mod.rs`](../../crates/aureline-env/src/env_diagnostics/mod.rs);
the boundary schema is
[`schemas/env/env-artifacts.schema.json`](../../schemas/env/env-artifacts.schema.json),
the corpus and expected diagnoses are checked in under
[`fixtures/env/env-diagnostics/`](../../fixtures/env/env-diagnostics/), and the
operator runbook is
[`artifacts/env/env-diagnostics-runbook.md`](../../artifacts/env/env-diagnostics-runbook.md).

It builds directly on the four environment objects already materialized in this
crate — the typed capsule
([`docs/env/environment-capsule.md`](environment-capsule.md)), the workspace
template ([`docs/env/workspace-template.md`](workspace-template.md)), the
prebuild fingerprint
([`docs/env/prebuild-fingerprint.md`](prebuild-fingerprint.md)), and the runtime
materialization
([`docs/env/runtime-materialization.md`](runtime-materialization.md)) — and their
metadata-first exports. This lane does **not** invent a fifth model; it composes
the existing exports and reuses their verdicts.

## Why this exists

Each environment object can already be inspected, diffed, and exported on its
own surface. What was missing was a *portable contract* that carries those
exports together and a *diagnostics engine* that explains, in one vocabulary,
why a capsule, template, prebuild, or runtime could not be trusted, hydrated, or
reused — and that keeps that vocabulary identical whether the vendor network is
present, only a mirror is reachable, or the import is fully offline.

Without it, mirror and offline users fall back to an opaque import: a workspace
either "starts" or "fails", with no shared reason for *why* a warm start went
cold, a template's provenance is unverified, or a runtime ran on the wrong
target. This lane closes that gap.

## The bundle and its provenance

An `EnvArtifactBundle` composes the existing metadata-first exports —
`CapsuleExport`, `TemplateExport`, `PrebuildExport`, `RuntimeExport` — under one
`ArtifactProvenance`:

- **`schema_version`** — the env-artifacts schema version the bundle was
  produced against.
- **`producer_surface`** and **`producer_build_ref`** — which surface
  (desktop, headless, support) produced it and from which build.
- **`source_channel`** — the load-bearing field: `online` (first-party origin
  over a reachable vendor network), `mirror` (a managed or community mirror with
  the vendor network absent), or `offline` (a sealed import that reached no
  network).
- **`source_truth`** — a review-safe label describing the source, and
  **`mirror_origin_ref`**, required when the channel is `mirror`.
- **`redaction_class`** — always `metadata_only`; no secret, raw environment
  body, hook command, or provider payload crosses the boundary.

`assemble_env_bundle` is the export flow: callers project their raw objects
through the existing `export_*` functions, then hand the exports here.

## Import, compare, diagnose

- **`import_env_bundle`** validates the bundle contract — record kind, schema
  version, provenance completeness, metadata-first redaction, a named mirror
  origin when the channel is `mirror`, and at least one artifact — then folds it
  into a diagnostics report so the importer immediately sees why each artifact
  is or is not trusted.
- **`compare_env_bundles`** diffs two bundles across schema versions and source
  channels. A schema-version drift (`schema_version_compatible: false`) or a
  channel change (`source_channel_changed: true`) is explicit, and each
  added / removed / changed artifact is a metadata-token delta — never a body.
- **`diagnose_bundle`** maps each artifact's existing verdict — a capsule or
  template `RowVerdict`, a prebuild `StartOutcome`, a runtime `RuntimeParity` —
  onto one `FindingCode` and one `HydrationOutcome`.

## Finding codes and outcomes

| Finding code | Outcome | Blocks share |
| --- | --- | --- |
| `trusted` | trusted | no |
| `maturity_narrowed` | degraded | no |
| `warm_start_downgraded` | degraded | no |
| `prebuild_partial_reuse` | degraded | no |
| `materialization_degraded` | degraded | no |
| `mirror_source_unverified` | degraded | no |
| `prebuild_cold_rebuild` | unreusable | no |
| `prebuild_invalidated` | unreusable | no |
| `claim_withheld` | untrusted | yes |
| `materialization_mismatch` | untrusted | yes |
| `schema_version_unsupported` | untrusted | yes |
| `redaction_violation` | untrusted | yes |

The invariant: **share is blocked exactly when an artifact is `untrusted`.** A
cold or invalidated prebuild is `unreusable` — the environment still hydrates via
a rebuild — so it is surfaced as a notice rather than a block. A `degraded`
artifact is usable but visibly narrowed.

The report carries the per-artifact diagnostics, the trusted / degraded /
unreusable / untrusted roll-ups, the blocking-artifact tokens, the
`share_blocked` flag, and a `ReviewState` (`pending_review` when clean, `blocked`
when any artifact is untrusted) that keeps the review-before-share posture
explicit.

## One report, every surface

`desktop_env_diagnostics`, `headless_env_diagnostics`, and
`support_env_diagnostics` all return the **same** `EnvDiagnosticsReport`.
`doctor_env_probes` projects that report, one diagnostic at a time, into
Project-Doctor-shaped `EnvDoctorProbe`s carrying the finding code, the evidence
refs, the explanation, the exact recovery path, and a severity (`healthy`,
`notice`, `blocking`). Project Doctor and the support export therefore explain an
environment-hydration failure from one source of truth, not a private clone.

## The corpus

The checked-in corpus carries one bundle per source channel, each re-derived
through `diagnose_bundle` so its recorded outcome can never drift from the
engine:

| Fixture | Channel | Outcome |
| --- | --- | --- |
| `local_online_trusted` | online | every artifact trusted; pending review |
| `remote_mirror_degraded` | mirror | stale fingerprint, community template, partial prebuild, degraded runtime all downgrade visibly; shareable |
| `offline_sealed_blocked` | offline | ungated capsule hook and wrong-target runtime block the bundle; invalidated prebuild is not reusable |
