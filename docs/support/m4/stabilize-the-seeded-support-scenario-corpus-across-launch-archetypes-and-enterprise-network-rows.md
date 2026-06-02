# Stabilize the seeded support-scenario corpus across launch archetypes and enterprise-network rows

## Purpose

This document defines the M4 stable-lane contract for the seeded support-scenario corpus that covers launch archetypes and enterprise-network rows. The corpus ensures that blocked-user recovery, support-export verification, diagnosis routing, and repair-preview validation are typed, truthful, and narrow across the full matrix of launch conditions and network postures.

## What this seed owns

- The closed [`LaunchArchetypeClass`] and [`EnterpriseNetworkRowClass`] vocabularies.
- The [`StabilizedScenarioCorpus`] loader and validator.
- The [`StabilizedScenarioEvaluator`] that validates scenarios and projects metadata-safe support packets and reports.
- The protected fixture corpus under `fixtures/support/m4/stabilize-the-seeded-support-scenario-corpus-across-launch-archetypes-and-enterprise-network-rows/`.

## What this seed does NOT own

- Live runtime probe execution or fixture mutation.
- Hosted ticket intake or cross-tenant escalation.
- Live measurement of support-scenario latency (owned by [`scenario_scorecard`]).

## Closed vocabularies

### Launch archetypes

- `first_run` — First-time launch with no prior workspace state.
- `update_restart` — Launch after a product update or restart.
- `crash_recovery` — Launch following a crash-loop recovery event.
- `enterprise_managed` — Launch under enterprise-managed policy or device posture.
- `extension_install` — Launch triggered by an extension install or update.
- `workspace_open` — Launch opening an existing workspace.

### Enterprise-network rows

- `standard_internet` — Standard internet-connected environment.
- `air_gapped` — Air-gapped environment with no external network access.
- `proxied` — Network routed through an enterprise proxy.
- `offline_first` — Offline-first environment with intermittent connectivity.
- `restricted_enterprise` — Restricted enterprise network with managed egress.

### Stabilized scenario classes

- `blocked_user_recovery` — Blocked-user recovery scenario.
- `support_export_verification` — Support-export verification scenario.
- `diagnosis_routing` — Diagnosis routing scenario.
- `repair_preview_validation` — Repair-preview validation scenario.
- `safe_mode_transition` — Safe-mode transition scenario.
- `crash_loop_center_evidence` — Crash-loop center evidence scenario.

## Claim-downgrade rules

Every scenario must declare at least:

- `fixture_missing` → `stale_corpus`
- `drill_step_unproven` → `yellow_aging`
- `drill_proves_regression` → `red_blocked`

## Safety baseline

All scenarios must enforce:

- `read_only_diagnosis: true`
- `raw_private_material_excluded: true`
- `destructive_resets_present: false`
- `preserves_user_authored_files: true`

## Integration touchpoints

- `crates/aureline-support` — Corpus loader, evaluator, and support-packet projection.
- `crates/aureline-doctor` — Project Doctor finding refs consumed by scenarios.
- `crates/aureline-crash` — Crash-loop signal records consumed by crash-recovery scenarios.
- `docs/support/m4/` — Reviewer-facing documentation.
- `artifacts/support/m4/` — Human-readable certification artifact.
- `schemas/support/` — Boundary schema.
- `fixtures/support/m4/` — Protected fixture corpus.

## Verification

Run the protected integration test:

```
cargo test -p aureline-support --test stabilize_the_seeded_support_scenario_corpus_across_launch_archetypes_and_enterprise_network_rows
```

All tests must pass before a release candidate can claim this row as stable.
