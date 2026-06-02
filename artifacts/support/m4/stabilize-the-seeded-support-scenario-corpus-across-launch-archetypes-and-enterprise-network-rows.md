# Stabilized Seeded Support-Scenario Corpus — Launch Archetypes and Enterprise-Network Rows

**Record kind:** `stabilized_scenario_report_record`  
**Schema version:** 1  
**Schema ref:** `schemas/support/stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows.schema.json`  
**Doc ref:** `docs/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows.md`

## Scope

This artifact certifies that the M4 stable lane covers blocked-user recovery, support-export verification, diagnosis routing, and repair-preview validation across the full matrix of launch archetypes and enterprise-network postures. Every row in this corpus is:

- Typed with closed vocabulary (no free-form prose in critical fields).
- Bound to a Project Doctor finding ref (`doctor.finding.*`).
- Metadata-safe (`raw_private_material_excluded = true`, `ambient_authority_excluded = true`).
- Free of destructive resets (`destructive_resets_present = false`).
- Preserving user-authored files (`preserves_user_authored_files = true`).

## Required launch archetypes

| Archetype | Token | Covered by scenario |
|-----------|-------|---------------------|
| First run | `first_run` | `stabilized_scenario:first_run:standard_internet:alpha` |
| Update restart | `update_restart` | `stabilized_scenario:update_restart:restricted_enterprise:alpha` |
| Crash recovery | `crash_recovery` | `stabilized_scenario:crash_recovery:air_gapped:alpha` |
| Enterprise managed | `enterprise_managed` | `stabilized_scenario:enterprise_managed:proxied:alpha` |
| Extension install | `extension_install` | `stabilized_scenario:extension_install:standard_internet:alpha` |
| Workspace open | `workspace_open` | `stabilized_scenario:workspace_open:offline_first:alpha` |

## Required enterprise-network rows

| Network row | Token | Covered by scenario |
|-------------|-------|---------------------|
| Standard internet | `standard_internet` | First run, Extension install |
| Air-gapped | `air_gapped` | Crash recovery |
| Proxied | `proxied` | Enterprise managed |
| Offline-first | `offline_first` | Workspace open |
| Restricted enterprise | `restricted_enterprise` | Update restart |

## Stabilized scenario classes represented

| Class | Token | Scenario |
|-------|-------|----------|
| Blocked-user recovery | `blocked_user_recovery` | First run |
| Crash-loop center evidence | `crash_loop_center_evidence` | Crash recovery |
| Diagnosis routing | `diagnosis_routing` | Enterprise managed |
| Repair-preview validation | `repair_preview_validation` | Workspace open |
| Safe-mode transition | `safe_mode_transition` | Update restart |
| Support-export verification | `support_export_verification` | Extension install |

## Claim-downgrade baseline

Every scenario declares the same three downgrade triggers:

1. `fixture_missing` → `stale_corpus`
2. `drill_step_unproven` → `yellow_aging`
3. `drill_proves_regression` → `red_blocked`

## Safety baseline

All scenarios enforce:

- `read_only_diagnosis: true`
- `raw_private_material_excluded: true`
- `destructive_resets_present: false`
- `preserves_user_authored_files: true`
- Forbidden fix classes:
  - `destructive_reset_without_preview`
  - `publish_route`
  - `reenable_quarantined_extension_without_preview`
  - `run_repo_owned_hook_for_diagnosis`
  - `widen_workspace_trust`
- No-touch boundary:
  - `user_authored_files`

## Verification

Run the protected integration test:

```
cargo test -p aureline-support --test stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows
```

## Change log

- **2026-06-02** — Initial corpus seeded with six scenarios covering all required launch archetypes and enterprise-network rows.
