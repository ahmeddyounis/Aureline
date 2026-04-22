# M0 signoff checklist

This checklist is the reviewer-facing companion to the shared packet at
`artifacts/milestones/M0_signoff_packet.md` and the validator at
`tools/check_m0_exit.py`.

## How to use it

1. Update the shared packet and its JSON companion together.
2. Run `python3 tools/check_m0_exit.py --repo-root .`.
3. Treat `PASS` as review-ready, `FAIL` as a blocker, and `MISSING` as an
   incomplete packet or missing artifact home.
4. Do not widen M0 language by discussion alone; if a blocker is being
   tolerated, record that decision in the shared packet and sign it there.

## Exit items

| Exit item | What the validator checks | Canonical refs |
|---|---|---|
| Architecture pack approved or explicitly held | Architecture pack exists, is review-ready, and is cited from the shared signoff packet | `artifacts/milestones/M0_architecture_pack/README.md`, `artifacts/milestones/M0_architecture_pack/packet_index.yaml` |
| Benchmark CI running as a governed seed | Workflow, dashboard seed, charter, and scorecard lane all exist together | `.github/workflows/nightly_benchmark.yml`, `artifacts/benchmarks/dashboard_seed/dashboard.json`, `docs/governance/benchmark_council_charter.md`, `artifacts/milestones/M0_scorecard.yaml` |
| Renderer spike viable | Renderer ADR, spike manifest, trace sample, and lane status are all present | `docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`, `artifacts/render/spike_capabilities.json`, `artifacts/render/spike_trace_samples/full_scene.json`, `artifacts/milestones/M0_scorecard.yaml` |
| Top ADRs opened or resolved | Decision register and approved ADR set exist and agree on the core frozen rows | `artifacts/governance/decision_index.yaml`, `artifacts/milestones/M0_architecture_pack/packet_index.yaml` |
| Ownership explicit | Ownership matrix, CODEOWNERS, and scorecard all name the current lanes | `artifacts/governance/ownership_matrix.yaml`, `CODEOWNERS`, `artifacts/milestones/M0_scorecard.yaml` |
| Requirement register present | The architecture packet still exposes the requirement-register slice, canonical register, and alias crosswalk | `artifacts/milestones/M0_architecture_pack/packet_index.yaml`, `artifacts/governance/requirement_register_seed.yaml`, `docs/governance/requirement_alias_crosswalk.md` |
| Source-anchor and canonical-reference coverage known | Source-anchor field catalog exists and architecture pack family coverage still cites canonical refs | `artifacts/evidence/evidence_metadata_fields.yaml`, `artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml` |
| Dependency ledger current | Decision dependencies and third-party dependency register exist together | `artifacts/governance/decision_index.yaml`, `artifacts/governance/dependency_register.yaml` |
| Control-artifact validation status known | Control-artifact index exists and the milestone signoff packet is listed as a control asset | `artifacts/governance/control_artifact_index.yaml`, `docs/milestones/M0_signoff_checklist.md` |
| Decision-forum charter pack seeded | Decision forums and the benchmark-council charter are both present | `artifacts/governance/ownership_matrix.yaml`, `docs/governance/benchmark_council_charter.md` |
| Qualification and ring rules linked | Qualification matrix, version-skew register, install topology, and rollout rules all exist together | `artifacts/compat/qualification_matrix_seed.yaml`, `artifacts/compat/version_skew_register.yaml`, `artifacts/release/install_topology_matrix.yaml`, `docs/governance/feature_flag_policy.md` |
| Accessibility and locale review lanes seeded or explicitly deferred | Accessibility review packet home and locale governance posture are visible and machine-checkable | `artifacts/governance/control_artifact_index.yaml`, `artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml`, `artifacts/milestones/M0_signoff_packet.json` |
| Locality/continuity and transport seeds present | Boundary, continuity, and route/transport artifacts are all linked from the shared packet | `docs/product/boundary_manifest_strawman.md`, `artifacts/support/deployment_drill_catalog_seed.yaml`, `docs/runtime/origin_target_route_taxonomy.md` |
| CLI/headless contract posture declared | Command contract, stable exit/return-code family seed, and architecture-pack note exist | `docs/commands/command_descriptor_contract.md`, `artifacts/release/silent_deployment_seed.yaml`, `artifacts/milestones/M0_architecture_pack/packet_index.yaml` |
| Docs-control policy current | Docs/help truth ADR and docs-pack contract are still the cited authority | `docs/adr/0013-docs-help-service-health-truth.md`, `docs/docs/docs_pack_manifest_contract.md` |
| Evidence freshness and packet-join rules current enough for review | Shared packet evidence rows all carry `captured_at` plus `stale_after`, claim-bearing packets expose stable evidence ids, and stale claim-bearing rows fail the review | `artifacts/milestones/M0_signoff_packet.json`, `artifacts/evidence/evidence_metadata_fields.yaml`, `artifacts/governance/evidence_id_conventions.md`, `schemas/governance/evidence_packet_header.schema.json`, `docs/governance/verification_packet_template.md` |

## Mandatory signed-packet sections

The validator fails immediately if the shared packet omits any of these:

- Deployment-profile truth
- Canonical decision register
- Notification and chronology primitives
- Local-history contract
- Security-intake baseline

## Contract-family matrix

These are the signoff-specific contract families the packet must track across
the architecture pack, compatibility inventory, QE lane registry, assurance
claim matrix, and public-proof coverage report.

| Family | Current reviewer-visible state | Why it matters now |
|---|---|---|
| `deployment_profile_truth` | `seeded` | M0 close cannot talk about locality or continuity honestly without one deployment-profile baseline. |
| `canonical_decision_register` | `frozen` | Review packets, waivers, and later releases need one decision identity source. |
| `notification_and_chronology_primitives` | `seeded` | Durable attention and record chronology cannot stay implicit once they enter review language. |
| `local_history_contract` | `seeded` | Recovery credibility depends on local-history and mutation-lineage truth staying explicit. |
| `security_intake_baseline` | `seeded` | Security signoff needs a seeded intake, incident-packet, and severity vocabulary even before live automation. |
| `cli_headless_contract_posture` | `seeded` | Automation posture must be declared before later stable CLI claims harden. |
| `accessibility_locale_review_lanes` | `deferred_signed_exception` | Accessibility review-home and locale governance remain real blockers and must stay explicit. |

## Current expected blockers

The packet is intentionally able to fail on the current branch. At the moment
the expected blockers are:

- stale benchmark dashboard seed evidence
- accessibility/input review packet family still unresolved
- locale-governance lane still deferred by signed exception
- exact-build and clean-room release posture still below green signoff
