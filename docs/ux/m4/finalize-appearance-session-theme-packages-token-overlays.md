# Appearance-session finalization — theme packages, token overlays, imported-theme mapping, live-appearance changes, and extension descriptors — contract

This is the reviewer-facing companion for the stable lane that finalizes the
**appearance session** as a governed launch property: one governed record per
appearance posture that binds **versioned theme package manifests**,
**inspectable appearance-session summaries**, **token-overlay validation by
scope**, **imported-theme mapping honesty**, **extension/embedded-surface
appearance descriptors**, **live-appearance change disclosure**, and
**provenance preservation across import/export/sync** — all to a public claim
ceiling and an automatic narrow-below-Stable verdict.

This lane is a settings-side certification that *projects* the design-system
appearance runtime. It does not render a theme; it proves the appearance state
is inspectable, exportable, reversible, and cannot silently redefine trust or
severity semantics through color alone. It builds on the appearance-session beta
contract (`aureline_design_system::appearance_session`), the theme-package
manifest (`aureline_ui::themes::package`), the token registry
(`aureline_ui::tokens`), the imported-theme mapping report
(`aureline_ui::themes::import_review`), and the extension
appearance-conformance packet (`aureline_extensions::appearance_conformance`).

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/`](../../../fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/)
- Schema:
  [`/schemas/ux/finalize-appearance-session-theme-packages-token-overlays.schema.json`](../../../schemas/ux/finalize-appearance-session-theme-packages-token-overlays.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md`](../../../artifacts/ux/m4/finalize-appearance-session-theme-packages-token-overlays.md)
- Typed source: `aureline_settings::finalize_appearance_session_theme_packages_token_overlays` (`model`, `corpus`)
- Headless emitter: `aureline_settings_finalize_appearance_session_theme_packages_token_overlays`
- Replay + invariant gate:
  `crates/aureline-settings/tests/finalize_appearance_session_theme_packages_token_overlays_fixtures.rs`

## Why one governed certification record

Appearance state is consumed by every shell surface, by extension-contributed
UI, by migration and import flows, and by support/diagnostics exports. If each
surface improvises its own theme reading, silently drops unsupported tokens,
flattens imported-theme gaps into generic settings, or hides extension
inheritance gaps behind host chrome, then a theme change can silently break
trust cues, severity badges, or focus rings — and the exported packet may claim
parity that the runtime cannot prove.

An `appearance_session_finalization_certification_record` closes that gap. For
one appearance posture it binds:

- **One appearance-session value.** `appearance_session.value_ref` records the
  active session id and revision. Every row's export packet cites that same
  value, so diagnostics and support read one source of truth.
- **Versioned theme package manifests.** `theme_packages[]` carries manifest
  refs, version labels, supported modes, density defaults, motion flags, and
  minimum contrast metadata. A package that is unversioned or lacks provenance
  narrows the posture below Stable.
- **Inspectable session summaries.** `session_summaries[]` carries active
  package refs, follow-system state, theme/mode, accent source, text scale,
  density, reduced-motion/high-contrast state, and checkpoint/rollback
  information. A summary that is not exportable or does not cite one package
  source narrows the posture.
- **Token-overlay validation.** `token_overlays[]` validates overlays by scope
  (user, profile, workspace, policy). Unknown or unsupported tokens are
  preserved inert or downgraded; a scope that silently drops tokens narrows the
  posture.
- **Imported-theme mapping honesty.** `import_reports[]` names translated,
  unsupported, unresolved, and fallback slots. An imported theme that claims
  full fidelity without evidence or hides gaps narrows the posture.
- **Extension appearance descriptors.** `extension_descriptors[]` declares
  whether each UI-bearing extension inherits theme, density, contrast, focus,
  and reduced-motion posture, or surfaces a visible inheritance gap in product,
  export, and diagnostics. An undisclosed gap narrows the posture.
- **Live-appearance change disclosure.** `live_changes[]` declares whether each
  OS appearance signal applies live, behind a checkpoint, requires confirmation,
  or requires a disclosed reload/restart. A silent lag narrows the posture.
- **Provenance preservation.** `provenance[]` proves package identity,
  unresolved-slot notes, overlay-scope lineage, and inheritance gaps survive
  import/export/sync without flattening into generic profile settings.
- **A public claim ceiling and automatic narrowing.** A posture that cannot
  prove a pillar narrows below Stable with a named reason rather than
  inheriting an adjacent green row.

## Acceptance criteria

- **Theme-package, appearance-session, token-overlay, import-report, and
  extension-descriptor objects are exportable, inspectable, and referenced by
  the stable proof index and migration docs.** The certification record carries
  `diagnostics_export_ref`, `support_export_ref`, and `evidence_refs` that
  dashboards, docs, Help/About, and support exports ingest verbatim.
- **Live OS appearance-change fixtures prove Aureline either applies the new
  mode coherently or names which surfaces require reload/restart.**
  `live_changes[]` covers all six `LiveAppearanceAxisClass` values; every row
  sets `applies_coherently_or_discloses = true` and `silently_lags_system =
  false`; reload/restart rows set `disclosure_required = true`.
- **Theme preview and import flows can always revert atomically to a prior
  checkpoint without partial apply or state corruption.**
  `session_summaries[].checkpoint_active` and `import_reports[].rollback_path_present`
  prove rollback paths exist.
- **Extension UI appearance gaps are visible to users and reviewers and cannot
  quietly inherit a Stable parity claim they do not meet.**
  `extension_descriptors[]` requires `gap_visible_in_product`,
  `gap_visible_in_export`, and `gap_visible_in_diagnostics` for every surface
  that does not fully inherit all five axes.
- **Appearance provenance stays intact across import/export/sync.**
  `provenance[]` covers all four `ProvenanceDimensionClass` values; each
  asserts `survives_sync_without_flattening = true`.
