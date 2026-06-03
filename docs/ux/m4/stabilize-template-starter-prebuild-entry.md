# Template, starter, and prebuild entry disclosure truth (stable)

## Why this lane exists

Every time a user enters Aureline through a **template**, **starter**, or
**prebuild** accelerator, the product used to hide what the accelerator actually
is, what it will do, and how to skip it. Users could not tell whether they were
resuming a live workspace, opening a snapshot, cloning fresh, or applying a setup
starter — and the "open plain" bypass was buried or missing.

This lane closes that gap with **one governed record** every surface reads
verbatim — it does **not** fork a per-surface disclosure vocabulary.

## What the record discloses

`template_starter_prebuild_entry_record` is minted by
`crates/aureline-shell/src/stabilize_template_starter_prebuild_entry` and frozen
at the boundary by `schemas/ux/template-prebuild-entry.schema.json`. The desktop
shell, diagnostics, support exports, Help/About, and docs all read this single
record.

It carries ten bound sections:

1. **`accelerator_identity`** — what the accelerator is, its version, and its
   bound manifest ref.
2. **`source_review`** — first-party, team-managed, community, local-only,
   mirror-cached, or uncertified, with signature state, publisher label, and
   trust notes.
3. **`support_review`** — officially supported, community supported,
   experimental, legacy deprecated, unsupported, or unknown.
4. **`runtime_review`** — local-only, devcontainer, container, remote image,
   managed cloud, mixed, or not-declared, with host boundary and supported
   ecosystems/platforms.
5. **`freshness_review`** — fresh, near expiry, stale, expired, or unknown,
   with producer and signer posture.
6. **`setup_review`** — expected actions, estimated duration, and connectivity
   expectation.
7. **`side_effect_envelope`** — network egress, extension installs, remote
   provisioning, managed services, credential provisioning, declared hooks, and
   declared setup tasks.
8. **`resulting_mode`** — resume live workspace, start from snapshot, clone
   fresh, open prebuild with setup, open prebuild minimal, open without starter,
   create empty workspace, create project, create service, or add module.
9. **`bypass_paths`** — same-weight alternatives such as "Open without starter"
   or "Set up later", each with `equal_weight_with_apply` continuity.
10. **`trust_auth_boundaries`** — trust posture, auth requirement,
    registry/mirror boundary, managed service boundary, and significant download
    expectation.

Plus two optional/derived blocks:

- **`cleanup_rollback`** — cleanup path and rollback path summaries.
- **`failure_summary`** — what succeeded, what was skipped, what was partially
  applied, what failed, what cleanup ran, and what remains for the user to
  review.

## The honesty invariants the builder enforces

`TemplateStarterPrebuildEntryRecord::build` refuses to mint a record that would
lie. Each is a `BuildError`:

- **Bypass parity.** Every record carries at least one bypass path, and the
  bypass continuity class is `equal_weight_with_apply`.
- **Source honesty.** Community and uncertified sources carry at least one trust
  note; missing signers are not hidden behind generic posture.
- **Runtime consistency.** A `local_only` runtime cannot require remote
  provisioning or managed services. A `managed_cloud_required` runtime must
  declare both a managed-service class and network egress.
- **Freshness for prebuilds.** Prebuild entries must declare freshness; template
  entries may but are not required to.
- **Failure transparency.** Partial application is disclosed, not hidden behind
  generic failure copy.
- **No raw secrets in export.** Support export metadata keeps raw secrets,
  command lines, and URLs redacted.
- **Entry disclosure stays separate from scaffolding.** The record never carries
  a scaffold plan, scaffold run, or generated-project lineage; those belong to
  the scaffold lane.
- **Intent preservation.** `Create project`, `Create service`, `Add module`,
  `Create without starter`, and `Open without starter` stay distinct; the record
  never quietly broadens generation scope.

## What never crosses this boundary

Raw paths, raw command lines, raw URLs, raw tokens, raw provider payloads, raw
user content, and raw secret material never appear on these records. Every
affordance carries an opaque object ref or stable token and a short reviewable
sentence. The `support_export_lines` projection is the redaction-safe,
deterministic block the support bundle and diagnostics packet quote verbatim.

## The drill corpus

`crates/aureline-shell/src/stabilize_template_starter_prebuild_entry/corpus.rs`
mints one scenario per named drill and pins each rendered record bit-for-bit
under `fixtures/ux/m4/stabilize-template-starter-prebuild-entry/`.

| Scenario | Entry kind | Resulting mode | Freshness | Side effects | Bypass count |
| --- | --- | --- | --- | --- | --- |
| `template_first_party_create_project` | template | create_project | fresh | none | 2 |
| `starter_community_create_service` | starter | create_service | near_expiry | egress, extensions, container | 2 |
| `prebuild_fresh_resume_live` | prebuild | resume_live_workspace | fresh | devcontainer attach | 2 |
| `prebuild_stale_start_snapshot` | prebuild | start_from_snapshot | stale | mirror egress, container | 3 |
| `prebuild_clone_fresh` | prebuild | clone_fresh | fresh | first-party egress | 2 |
| `template_open_without_starter` | template | open_without_starter | fresh | first-party egress, extensions | 2 |
| `starter_failure_partial_apply` | starter | create_project | fresh | community egress, extensions | 2 |
| `prebuild_managed_cloud` | prebuild | open_prebuild_with_setup_actions | near_expiry | managed workspace, credentials | 2 |
| `template_create_empty` | template | create_empty_workspace | fresh | none | 2 |
| `prebuild_expired_open_minimal` | prebuild | open_prebuild_minimal | expired | none (expired) | 2 |

## Regenerating and inspecting

```sh
# Refresh the on-disk fixtures (the test fails if they drift).
cargo run -q -p aureline-shell \
  --bin aureline_shell_stabilize_template_starter_prebuild_entry -- emit-fixtures \
  fixtures/ux/m4/stabilize-template-starter-prebuild-entry

# Stable corpus index — scenario id, kind, mode, honesty, bypass count.
cargo run -q -p aureline-shell \
  --bin aureline_shell_stabilize_template_starter_prebuild_entry -- index

# Per-scenario support-export truth block.
cargo run -q -p aureline-shell \
  --bin aureline_shell_stabilize_template_starter_prebuild_entry -- plaintext

# Replay + invariant gate.
cargo test -p aureline-shell --test stabilize_template_starter_prebuild_entry_fixtures
```

## Consumers

The shell paints from this record; diagnostics, support exports, and Help/About
**ingest** it rather than cloning its status text. Later dashboards and docs in
this lane should read the checked-in fixtures and schema as the canonical truth
source for template/starter/prebuild entry disclosure on claimed stable rows.
