# Hardened safe-mode runtime profile, retained capabilities, and support guidance — Artifact

## Status

**Stable** — hardened M4 safe-mode profile with retained capabilities,
accessibility posture, and recovery-ladder integration.

## Checked-in outputs

| Output | Path |
|--------|------|
| Implementation | `crates/aureline-support/src/harden_the_safe_mode_runtime_profile_retained_capabilities/mod.rs` |
| Boundary schema | `schemas/support/harden_the_safe_mode_runtime_profile_retained_capabilities.schema.json` |
| Reviewer doc | `docs/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities.md` |
| Fixture corpus | `fixtures/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities/` |

## What is hardened

The beta safe-mode profile is promoted to a stable contract by adding:

1. **Retained capability records** — each of the eight required capabilities
   (`local_editing`, `basic_navigation`, `local_search`, `local_git_operations`,
   `local_diagnostics_export`, `support_bundle_preview`,
   `project_doctor_surfaces`, `safe_mode_exit_action`) carries a rationale
   and user-facing support guidance.
2. **Accessibility posture rows** — every touched surface and retained
   capability attests keyboard, screen-reader, IME/grapheme/bidi, zoom,
   high-contrast, and reduced-motion behavior.
3. **Recovery-ladder bindings** — every required rung (`safe_mode`,
   `open_without_restore`, `disable_recent_extension`, `disable_recent_layout`,
   `open_logs`, `export_crash_manifest`, `report_issue`) carries a support
   class, review gate, and evidence refs.

## Seeded support scenarios

The fixture corpus covers three profile classes:

- `post_crash_loop_profile` — entered after startup crash-loop budget breach.
- `user_invoked_profile` — user explicitly chose safe mode from recovery surface.
- `policy_forced_profile` — managed policy forced safe mode.

Each fixture includes:
- one hardened profile,
- eight retained capability records,
- accessibility posture rows for all touched surfaces and capabilities,
- seven recovery-ladder binding rows.

## Verification

Run the protected tests:

```bash
cargo test -p aureline-support --test harden_safe_mode_runtime_profile
```

## Risks and follow-ups

- Live runtime enforcement (host launcher, service supervisor) is out of scope
  and lands with the chrome/runtime consumers.
- Time-bounded auto-exit is not covered; only explicit user-confirmed return
  paths are certified.
- Cross-tenant policy reconciliation remains unsupported.
