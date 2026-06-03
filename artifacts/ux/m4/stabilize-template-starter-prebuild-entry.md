# Template, starter, and prebuild entry disclosure — release evidence

Reviewer-facing evidence packet for the lane that stabilizes template, starter,
and prebuild entry disclosure with side-effect envelopes, freshness truth, and
open-without-starter parity.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/stabilize-template-starter-prebuild-entry/`](../../../fixtures/ux/m4/stabilize-template-starter-prebuild-entry/)
- Schema: [`/schemas/ux/template-prebuild-entry.schema.json`](../../../schemas/ux/template-prebuild-entry.schema.json)
- Companion doc: [`/docs/ux/m4/stabilize-template-starter-prebuild-entry.md`](../../../docs/ux/m4/stabilize-template-starter-prebuild-entry.md)
- Typed source: `aureline_shell::stabilize_template_starter_prebuild_entry` (`model`, `corpus`)
- Headless emitter: `aureline_shell_stabilize_template_starter_prebuild_entry`
- Replay + invariant gate: `crates/aureline-shell/tests/stabilize_template_starter_prebuild_entry_fixtures.rs`

## What this packet proves

1. **Every accelerator surface explains what it is before setup begins.** Each
   record's `accelerator_identity`, `source_review`, and `support_review` are
   present and non-empty; the builder rejects any record that omits them.

2. **Side-effect envelope is visible before commitment.** Every record's
   `side_effect_envelope` names egress, extension install, remote provisioning,
   managed service, and credential classes, plus declared hook and task counts.
   The `starter_community_create_service` and `prebuild_managed_cloud` drills
   exercise every side-effect class.

3. **Freshness truth is disclosed for prebuilds.** Prebuild entries carry
   `freshness_review` with a closed age class; the builder rejects prebuilds with
   `unknown_requires_revalidation`. The corpus covers `fresh_under_window`,
   `near_expiry`, `stale_over_window`, and `expired`.

4. **Same-weight bypass paths are always present.** Every record carries at least
   one bypass path with `equal_weight_with_apply` continuity. The
   `template_open_without_starter` and `prebuild_expired_open_minimal` drills
   exercise open-without-starter and open-minimal bypasses explicitly.

5. **Resulting modes are distinct and honest.** `resume_live_workspace`,
   `start_from_snapshot`, `clone_fresh`, `open_prebuild_with_setup_actions`,
   `open_prebuild_minimal`, `open_without_starter`, `create_empty_workspace`,
   `create_project`, `create_service`, and `add_module` are each exercised by at
   least one scenario, and the builder rejects invalid kind/mode pairings.

6. **Trust, auth, registry, mirror, managed-service, and download boundaries are
   shown before setup.** The `prebuild_managed_cloud` drill covers managed
   workspace envelope, credential provisioning, and significant download; the
   `starter_community_create_service` drill covers community registry and Docker
   Hub boundaries.

7. **Failure summaries are non-destructive and recoverable.** The
   `starter_failure_partial_apply` drill shows succeeded, skipped, partially
   applied, failed, and cleanup-ran items with a remaining-user-review sentence.

8. **Support export is safe.** Raw secrets, command lines, and URLs are never
   allowed; the builder rejects any record that sets the corresponding flags.

## Coverage matrix

| Scenario | Entry kind | Resulting mode | Freshness | Side effects | Bypass count | Honesty |
| --- | --- | --- | --- | --- | --- | --- |
| `template_first_party_create_project` | template | create_project | fresh | none | 2 | false |
| `starter_community_create_service` | starter | create_service | near_expiry | egress, extensions, container | 2 | true |
| `prebuild_fresh_resume_live` | prebuild | resume_live_workspace | fresh | devcontainer attach | 2 | false |
| `prebuild_stale_start_snapshot` | prebuild | start_from_snapshot | stale | mirror egress, container | 3 | true |
| `prebuild_clone_fresh` | prebuild | clone_fresh | fresh | first-party egress | 2 | true |
| `template_open_without_starter` | template | open_without_starter | fresh | first-party egress, extensions | 2 | false |
| `starter_failure_partial_apply` | starter | create_project | fresh | community egress, extensions | 2 | true |
| `prebuild_managed_cloud` | prebuild | open_prebuild_with_setup_actions | near_expiry | managed workspace, credentials | 2 | true |
| `template_create_empty` | template | create_empty_workspace | fresh | none | 2 | false |
| `prebuild_expired_open_minimal` | prebuild | open_prebuild_minimal | expired | none (expired) | 2 | true |

## How to reproduce

```sh
cargo test -p aureline-shell --test stabilize_template_starter_prebuild_entry_fixtures
cargo test -p aureline-shell --lib stabilize_template_starter_prebuild_entry
cargo run -q -p aureline-shell --bin aureline_shell_stabilize_template_starter_prebuild_entry -- index
```

The replay test fails if any checked-in fixture drifts from the in-code corpus;
regenerate with the emitter's `emit-fixtures` subcommand. The builder's
negative-path unit tests prove each honesty invariant rejects a dishonest input.

## Known limits / follow-ups

- The record is the canonical truth; the live native shell, diagnostics packet,
  and support bundle are expected to **ingest** it. Wiring those existing
  consumers to read this record (instead of their current bespoke status text)
  is the natural next consumer step and is intentionally left to the surfaces
  that own those exports.
- The scaffold lane (`aureline_workspace::scaffold`) owns the actual scaffold
  plan, scaffold run, and generated-project lineage. This record only owns
  **review** — what the user is shown before any scaffolding runs.
