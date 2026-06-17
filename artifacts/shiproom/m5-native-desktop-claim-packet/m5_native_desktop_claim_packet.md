# Shiproom claim packet — M5 native-desktop qualification

This packet is the shiproom- and release-center-facing view of the
native-desktop qualification family. It does not maintain its own
summary: the claim scope below is derived from the canonical
qualification report and narrows automatically when a profile row goes
stale, missing, or red.

## Canonical inputs

- Qualification matrix: `artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md`
- Report fixture: `fixtures/platform/m5-native-desktop-qualification/report.json`
- Boundary schema: `schemas/platform/m5-native-desktop-qualification.schema.json`
- Companion doc: `docs/m5/native-desktop-qualification.md`
- Certifies matrix: `artifacts/platform/m5-native-desktop-matrix.md`
- CI gate: `tools/ci/m5/native_desktop_qualification_check.py`

- Claim publishable: **yes**
- Published profiles: `5`
- Narrowed profiles: `1`
- Withheld profiles: `0`

## Claim scope

| Profile | Platform | Channel | Claim | Downgrade rule | Reason |
| ------- | -------- | ------- | ----- | -------------- | ------ |
| `profile:linux.portable` | `linux` | `portable` | **Narrowed** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | narrowed_dimensions:protocol_handler_ownership,file_association_ownership |
| `profile:linux.stable` | `linux` | `stable` | **Published** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:macos.beta` | `macos` | `beta` | **Published** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:macos.stable` | `macos` | `stable` | **Published** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:windows.managed_fleet` | `windows` | `managed_fleet` | **Published** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:windows.stable` | `windows` | `stable` | **Published** | `downgrade:native_desktop_qualification:narrow_on_stale_or_red` | all_marketed_dimensions_qualified_with_fresh_evidence |

## Sign-off gate

Promotion of the native-desktop claim holds unless all of the following
are true on the current qualification report:

1. The report is clean: every claimed profile binds all seven
   dimensions and no profile carries a blocking finding
   (`report.report_clean == true`).
2. No distinct qualification failure is open — `ownership_unprovable`,
   `protocol_handler_conflict`, `file_association_conflict`,
   `wrong_target_reopen`, `lock_screen_leak`, `missing_root_silent_loss`,
   or `store_lock_dead_end`.
3. No profile borrows another profile's or channel's proof
   (`borrowed_proof_across_profile`); each claimed row carries current
   proof of its own.
4. No marketed profile carries stale evidence
   (`narrowable_marketed_profiles` is empty).
5. No profile claim is withheld, and every narrowed claim names the
   dimensions it dropped.

## Regenerating this packet

This packet is checked in alongside the report it derives from. When the
qualification contract changes, regenerate the packet and re-run the gate
before re-reviewing:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- claim-packet-md > \
  artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md
python3 tools/ci/m5/native_desktop_qualification_check.py
```
