# Appearance-session finalization — release evidence

Reviewer-facing evidence packet for the lane that finalizes the **appearance
session** across theme packages, token overlays, imported-theme mapping,
live-appearance changes, and extension/embedded-surface appearance descriptors:
one canonical record per appearance posture that binds versioned theme package
manifests, inspectable session summaries, token-overlay validation by scope,
imported-theme mapping honesty, extension inheritance-gap visibility,
live-change disclosure, provenance preservation, a public claim ceiling, an
automatic narrow-below-Stable verdict, recovery and route parity across the
settings appearance panel / command palette / status bar / menus, accessibility
across normal / high-contrast / zoomed layouts, and rows that stay available
without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/`](../../../fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/)
- Schema: [`/schemas/ux/finalize-appearance-session-theme-packages-token-overlays.schema.json`](../../../schemas/ux/finalize-appearance-session-theme-packages-token-overlays.schema.json)
- Companion doc: [`/docs/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md`](../../../docs/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md)
- Typed source: `aureline_settings::finalize_appearance_session_theme_packages_token_overlays` (`model`, `corpus`)
- Headless emitter: `aureline_settings_finalize_appearance_session_theme_packages_token_overlays`
- Replay + invariant gate: `crates/aureline-settings/tests/finalize_appearance_session_theme_packages_token_overlays_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | — |
| `token_overlay_silently_dropped_drill.json` | token-overlay drill | beta (narrowed) | stable | `token_overlay_silently_dropped` |
| `extension_gap_undisclosed.json` | extension gap undisclosed | beta (narrowed) | stable | `extension_gap_undisclosed` |
| `import_report_missing_rollback.json` | import missing rollback | beta (narrowed) | stable | `import_report_missing_visible_gaps` |

Coverage verdict: **1 Stable, 3 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Theme package manifests are versioned and declare provenance, supported
  modes, density defaults, motion flags, and minimum contrast metadata.**
  `theme_packages[]` carries `manifest_versioned`, `provenance_declared`,
  `supported_theme_classes`, `supported_density_classes`,
  `supported_motion_postures`, and contrast targets. On the Stable posture every
  row `conforms`.
- **Appearance-session summaries are exportable and cite one canonical theme
  package source.** `session_summaries[]` asserts `summary_exportable` and
  `cites_one_package_source`; the session id matches the binding
  `appearance_session.appearance_session_id`.
- **Token overlays are validated by scope; unknown or unsupported tokens are
  preserved inert or downgraded, never silently dropped.**
  `token_overlays[]` covers all four `OverlayScopeClass` values; each asserts
  `unknown_tokens_preserved`, `unsupported_tokens_preserved`, and
  `scope_lineage_recorded`. The drill posture exercises the narrowing path when
  a scope silently drops tokens.
- **Imported-theme mapping reports name translated, unsupported, unresolved,
  and fallback slots, and block full-fidelity claims without evidence.**
  `import_reports[]` asserts `syntax_coverage_reported`,
  `parity_notes_visible`, `fallback_behavior_documented`, and
  `full_fidelity_claim_blocked_when_unsupported`. The drill posture exercises
  the narrowing path when a report lacks a rollback path.
- **Extension/embedded surfaces declare inheritance or surface visible gaps in
  product, exported appearance packets, and migration/support diagnostics.**
  `extension_descriptors[]` declares `theme_inheritance`, `density_inheritance`,
  `high_contrast_inheritance`, `focus_inheritance`, and
  `reduced_motion_inheritance`. Every non-`inherits` surface asserts
  `gap_visible_in_product`, `gap_visible_in_export`, and
  `gap_visible_in_diagnostics`. The drill posture exercises the narrowing path
  when a gap is undisclosed.
- **Live OS appearance changes apply coherently or disclose reload/restart.**
  `live_changes[]` covers all six `LiveAppearanceAxisClass` values; every row
  asserts `applies_coherently_or_discloses` and `silently_lags_system = false`;
  reload/restart rows set `disclosure_required = true`.
- **Appearance provenance survives import/export/sync without flattening.**
  `provenance[]` covers all four `ProvenanceDimensionClass` values; each
  asserts `survives_sync_without_flattening = true`.
- **The same record is reachable keyboard-first from settings appearance panel,
  command palette, status bar, and menu command, across normal / high-contrast /
  zoomed layouts.** `routes[]` covers all four `RouteSurface` values; every
  route is `keyboard_reachable` and `activates_same_record`.
  `accessibility.layout_modes[]` covers all three `LayoutMode` values with
  `row_narration_available` and `recovery_affordances_reachable`.
- **Every record stays available without an account or managed services.**
  `available_without_account` and `available_without_managed_services` are both
  `true` on every posture.
