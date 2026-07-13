# M5 system-appearance live-apply and appearance-source-provenance registries

This lane is the live system-appearance-response implement lane over the frozen
[M5 platform-fit matrix](./m5_platform_fit_contract.md). It turns the concrete *live theme / contrast / accent
/ text-scale response* grammar of the `theme_contrast_live_change` family into registry resolvers that produce
export-safe, honest projections, so shell, settings, docs, onboarding, CLI, and support surfaces resolve one
canonical appearance-response and source-provenance truth instead of per-surface, hand-copied behavior.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_system_appearance_live_apply_and_source_provenance_registries`
  (the authoritative validator).
- **Combined schema:**
  `schemas/platform/m5-system-appearance-live-apply-and-source-provenance-registries.schema.json`.
- **Domain schema:** every row points at
  [`schemas/platform/m5-file-path-and-reveal.schema.json`](../../schemas/platform/m5-file-path-and-reveal.schema.json)
  as its single canonical appearance / terminology domain contract (the matrix maps the
  `theme_contrast_live_change` family onto this domain).
- **Checked proof:**
  `artifacts/release/m5-system-appearance-live-apply-and-source-provenance-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/platform/m5-system-appearance-live-apply-and-source-provenance-registries/`
  (`docs_help_beta_narrowed.json`, `restart_posture_preview_narrowed.json`).

## Two registries

1. **Appearance live-apply** (`resolve_appearance_live_apply_entry`) — applies system theme, contrast, accent,
   and text-scale changes live wherever the host platform supports it, and exposes an explicit fallback or
   restart-required posture where live reapplication is unavailable or unsafe, while preserving active shell /
   editor / dialog continuity. A clean entry names a canonical registry token, a classified support posture,
   and a theme-contrast-live-change role, covers the applied / canonical / accessible response forms, records a
   posture label and live-reapply state that match the claimed posture, preserves active-context continuity,
   and explains any narrower-than-live behavior. Otherwise it degrades honestly.
2. **Appearance source provenance** (`resolve_appearance_source_provenance_entry`) — records the active
   platform-appearance source and any fallback posture in settings, diagnostics, and support exports. A clean
   entry names a classified record surface and provides the stable-ID / record-surface / source-signal
   recording triple; a record that hides the active source or posture degrades to `source_or_posture_not_recorded`.

## Support-posture reference

The support posture carries its canonical label and whether it applies live, so the registry — never a
hand-copied per-platform behavior — is the single source of truth. `appearance_response_matches_posture`
rejects a drifted entry.

| support posture | applies live | posture label |
| --- | --- | --- |
| live_apply | yes | applies live |
| restart_required | no | restart required |
| unsupported | no | not supported on this host |

A live-apply entry that did not reapply live, and a restart-required or unsupported entry that claims to have
reapplied live, degrade to `posture_mislabeled_for_support` so a mislabeled diagnostics panel can never turn
release evidence green. A restart-required or unsupported change that does not explain its narrower behavior
degrades to `narrower_behavior_not_explained`.

## Acceptance criteria (proven by resolved examples)

- **Claimed desktop profiles either apply host appearance changes live or clearly explain the narrower
  supported behavior.** Clean response entries cover the `appearance` / `command_stability` semantic-role
  families and the first shell / editor / dialog / settings / docs surfaces, a hand-copied example degrades,
  and no clean entry is unbound.
- **Live theme / contrast / accent / text-scale changes do not corrupt focus, layout, or meaning on protected
  paths.** A change that resets local context degrades to `active_context_continuity_not_preserved`, and the
  row-level invariants forbid a live change corrupting focus, layout, or meaning or forcing a mystery repaint.
- **Diagnostics and support exports can distinguish live-apply from restart-required or unsupported platform
  behavior.** Clean provenance entries cover the settings / diagnostics / support-export record surfaces with
  full response-form coverage while providing the recording triple, and an unrecorded source degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- support-export
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- csv
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- report
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- posture-table
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- fixture-docs-help-beta-narrowed
cargo run -p aureline-ui --example dump_m5_system_appearance_live_apply_and_source_provenance_registries -- fixture-restart-posture-preview-narrowed
```
