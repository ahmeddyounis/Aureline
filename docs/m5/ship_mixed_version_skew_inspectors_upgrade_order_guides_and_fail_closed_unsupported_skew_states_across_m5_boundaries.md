# Ship mixed-version/skew inspectors, upgrade-order guides, and fail-closed unsupported-skew states across M5 boundaries

This document is the human-readable companion to the canonical M5 boundary skew-inspector register checked in at `artifacts/release/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.json` and described by the schema at `schemas/compat/m5-boundary-skew-inspectors.schema.json`.

## Purpose

Where the M5 qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) freezes the *static* qualification row, support window, and deprecation packet each stable-facing family holds, this register speaks for the *runtime* skew inspectors bound to the M5 boundary-crossing flows. A user or support engineer can read one row and tell whether a boundary is inside or outside its supported skew window **before** trying a mutating or privileged action, what fail-closed state it lands in if it is outside, and the exact upgrade order that brings it back inside the window. Help/About, release-center, service-health, support, and export surfaces ingest the same rows; the upgrade-order requirement is a structured product object, not text buried in support docs.

## Structure

Each inspector binds one boundary-crossing flow to:

- **Boundary kind** — the flow it guards: `helper_agent_attach` (desktop↔remote helper/agent attach), `extension_runtime_load` (extension host/SDK/manifest load), `state_import_restore` (workspace/schema/save-state import or restore), or `provider_snapshot_open` (provider snapshot/imported-object open). Each kind gates exactly one mutating-or-privileged action (`attach`, `load`, `restore`, `open`), and the inspector records its `action_risk` (`mutating`, `privileged`, or `mutating_and_privileged`).
- **Downgrade subject** — the `helper`, `agent`, `host`, `schema`, or `provider` noun the inspector downgrades.
- **Skew window** — the local and peer versions, the declared supported skew class (`lockstep_only`, `bounded_skew`, `backward_compatible`, `forward_compatible`, `unsupported_skew`), the version floor/ceiling, and the negotiated fields.
- **Verdict and gate posture** — the verdict the inspector reports before the gated action runs (`inside_window` or one of the fail-closed states `unsupported_skew`, `reconnect_required`, `reinstall_required`, `migration_needed`, `retest_pending`) and the resulting gate posture (`allow` or `fail_closed`). The gate allows the action exactly when the verdict is `inside_window`.
- **Upgrade-order guide** — which side upgrades first (`local_first`, `peer_first`, `coordinated`, or `none_required`) and the ordered steps that recover an out-of-window boundary.
- **Claim linkage** — the stable claim the boundary backs, the inspector state earned, the active narrowing reasons, and the effective label after narrowing.

## Narrowing rules

- An inspector carries a Stable (or LTS) support claim only when its verdict is `inside_window` (the gate allows the action), its declared skew class is supported, its proof packet is current within its freshness SLO, the owner has signed off, and its backing claim holds. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A boundary whose verdict is any fail-closed state must drop **below** the cutline rather than mutate optimistically or inherit an adjacent in-window boundary. The verdict names the narrowing reason it maps to (`unsupported_skew` → `skew_window_exceeded`, `reconnect_required` → `reconnect_required`, and so on), and a skew-recovery verdict (unsupported skew, reconnect, reinstall, migration) must carry an upgrade-order guide with a leading side and at least one step.
- The skew gate and the support claim are distinct. A boundary can be **inside** its skew window — so the gate *allows* the mutating-or-privileged action — yet still narrow its published support claim because the inspector's proof packet went stale (`evidence_stale`) or was never captured (`evidence_missing`). This narrows the marketed surface predictably instead of over-claiming.
- A `limited` inspector holds the claim but must record at least one compatibility caveat. A boundary held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream Help/About, release-center, service-health, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries`) rather than cloning status text, so every surface renders one source of truth.

## Freshness

The register is checked in with an `as_of` date and a per-inspector proof packet freshness SLO. An inspector whose proof packet breaches its SLO narrows automatically before publication; the frozen CI validation capture at `artifacts/release/captures/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces. Regenerate with `python3 tools/regenerate_m5_boundary_skew_inspectors.py` from the repository root after changing the inspectors.
