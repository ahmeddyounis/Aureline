# M5 native-desktop qualification

This doc is the human-facing companion to the native-desktop **qualification
family**: the per-desktop-profile rows and the auto-narrowing claim packet that
certify the [native-desktop matrix](native-desktop-integration-and-reopen.md)
across every claimed desktop profile. The matrix names every native-desktop
*surface*; this family proves that contract is currently exercised on each
claimed `(platform, channel)` profile and narrows the published claim the moment
a row goes stale, missing, or red.

It does not maintain its own summary. Help/About, install/update, docs, support
packets, evaluation materials, and the shiproom and release-center surfaces
ingest the canonical objects below rather than keeping parallel notes.

## Canonical objects

- Boundary schema: `schemas/platform/m5-native-desktop-qualification.schema.json`
- Report fixture: `fixtures/platform/m5-native-desktop-qualification/report.json`
- Support-export fixture:
  `fixtures/platform/m5-native-desktop-qualification/support_export.json`
- Claim-packet fixture:
  `fixtures/platform/m5-native-desktop-qualification/claim_packet.json`
- Published matrix:
  `artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md`
- Shiproom claim packet:
  `artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md`
- Typed consumer: `crates/aureline-shell/src/m5_native_desktop_qualification/`
- CI gate: `tools/ci/m5/native_desktop_qualification_check.py`

## What a profile certifies

A claimed profile is a `(platform, channel)` pair — for example
`profile:macos.stable`, `profile:windows.managed_fleet`, or
`profile:linux.portable`. Each profile binds all seven canonical qualification
dimensions, and each dimension is qualified by the platform-conformance drill
named beside it:

| Dimension | Drill |
| --------- | ----- |
| `channel_build_ownership` | `channel_ownership_audit` |
| `protocol_handler_ownership` | `handler_conflict` |
| `file_association_ownership` | `handler_conflict` |
| `reopen_fidelity` | `wrong_target_reopen` |
| `notification_privacy` | `lock_screen_privacy` |
| `external_root_recovery` | `missing_root_recovery` |
| `store_lock_recovery` | `store_lock` |

The wrong-target reopen, handler-conflict, lock-screen-privacy,
missing-root-recovery, and store-lock drills are bound into the qualification
corpus through these dimensions, so a desktop claim cannot be made without
current proof of each.

## Each claimed row needs its own current proof

A profile is never kept green because a nearby platform or channel passed. Each
qualified dimension carries its own drill ref and an evidence pack that names
*this* profile; a row that points at another profile's or channel's evidence
fails with `borrowed_proof_across_profile`. Stale evidence on a marketed profile
is a blocker, so release tooling narrows the profile instead of publishing it as
implicitly stable.

## Distinct failure classes

A red qualification is never a single generic failure. The drill that fails
maps to a distinct, exportable class:

- `ownership_unprovable`
- `protocol_handler_conflict`
- `file_association_conflict`
- `wrong_target_reopen`
- `lock_screen_leak`
- `missing_root_silent_loss`
- `store_lock_dead_end`

## Auto-narrowing claim scope

The published claim for each profile is **derived** from its dimension
qualification, so the claim can never be greener than the proof:

- `published` — every marketed dimension is qualified with fresh evidence.
- `narrowed` — some dimensions are explicitly narrowed, not applicable, stale,
  or red; the claim is published with an explicit narrowed scope and names the
  dimensions it dropped. A portable build, for example, registers no OS-level
  handler, so its `protocol_handler_ownership` and `file_association_ownership`
  dimensions are not applicable and the native-handler claim narrows.
- `withheld` — no marketed dimension qualifies; the claim is withheld.

The shiproom claim packet partitions the profiles into publishable, narrowed,
and withheld sets and exposes the downgrade rule per profile, so shiproom and
release-center surfaces inspect the same claim users implicitly rely on.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- validate
cargo test -p aureline-shell --test m5_native_desktop_qualification_fixtures
python3 tools/ci/m5/native_desktop_qualification_check.py
```
