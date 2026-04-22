# Requirement alias crosswalk

This document is the human-readable companion to
[`/artifacts/governance/requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml)
and
[`/schemas/governance/requirement_register.schema.json`](../../schemas/governance/requirement_register.schema.json).
The YAML is authoritative for tooling. This page explains how to use it
without creating parallel requirement identities.

## Canonical rules

- Cite `requirement_id` values from the requirement register in
  scorecards, verification packets, waiver packets, CI annotations,
  docs, and release evidence.
- Treat aliases and local labels as lookup handles only. They are not
  separate obligations and must not appear as the primary cited id in a
  new packet.
- When one local label covers multiple obligations, cite every mapped
  canonical requirement id from the register rather than inventing a new
  umbrella id.
- Fitness rows, scorecard calls, contract-family labels, and milestone
  exit labels are evidence labels. They may support a requirement, but
  they do not replace it.
- If a requirement changes meaning materially, add a new canonical row
  and map the old id as an alias. Do not silently rewrite history.

## Direct aliases

These are requirement-like labels already present in source docs that now
resolve to one canonical row.

| Local label | Source | Canonical requirement id | Rule |
|---|---|---|---|
| `GOV-OSS-004` | Technical architecture / design-system docs | `GOV-DATA-002` | Export-format portability language stays visible as history, but the broader data/export obligation is canonical. |
| `QE-CORPUS-001` | Technical design doc | `GOV-CORPUS-901` | Corpus-governance identity is normalized into the canonical GOV register instead of keeping a QE-only id family. |

## Evidence labels that are not requirement ids

These labels stay useful, but only as supporting evidence labels.

| Local label | Kind | Canonical requirement id(s) | How to cite it |
|---|---|---|---|
| `FIT-OPS-004` | fitness row | `OPS-BUILD-006` | Cite `OPS-BUILD-006` as the requirement and mention `FIT-OPS-004` as the supporting protected metric. |
| `renderer_viability` | architecture-pack call | `PERF-SHELL-001`, `PERF-EDITOR-002`, `ARCH-INV-001` | Use the packet call as a roll-up label only; cite the three requirement ids in packets or waivers. |
| `benchmark_governance` | architecture-pack call | `ARCH-FIT-003`, `GOV-CORPUS-901` | Cite the requirement ids; keep the call only as a dashboard or packet label. |
| `public_truth_seeds` | architecture-pack call | `GOV-OSS-001`, `GOV-TRUTH-901` | Use the call as a reviewer-facing roll-up, not as a requirement id. |
| `deployment_profile_truth` | signoff contract family | `GOV-OSS-001`, `GOV-TRUTH-901` | Contract-family labels stay valid for packet structure, but the cited obligations are the mapped requirement ids. |
| `canonical_decision_register` | signoff contract family | `ARCH-PACK-901` | Use the contract-family label as section framing only. |

## Foundations exit-item mapping

Every foundations exit item in
[`/docs/milestones/M0_signoff_checklist.md`](../milestones/M0_signoff_checklist.md)
maps to at least one canonical row in the requirement register.

| Exit item label | Canonical requirement id(s) |
|---|---|
| `architecture_pack_approved_or_explicitly_held` | `ARCH-PACK-901` |
| `benchmark_ci_running_as_a_governed_seed` | `ARCH-FIT-003`, `GOV-CORPUS-901` |
| `renderer_spike_viable` | `PERF-SHELL-001`, `PERF-EDITOR-002`, `ARCH-INV-001` |
| `top_adrs_opened_or_resolved` | `ARCH-PACK-901` |
| `ownership_explicit` | `REPO-OWN-002` |
| `requirement_register_present` | `GOV-REQ-901` |
| `source_anchor_and_canonical_reference_coverage_known` | `GOV-EVID-901` |
| `dependency_ledger_current` | `ARCH-SVC-002`, `ARCH-PACK-901` |
| `control_artifact_validation_status_known` | `ARCH-PACK-901`, `GOV-REQ-901` |
| `decision_forum_charter_pack_seeded` | `REPO-OWN-002`, `ARCH-FIT-003` |
| `qualification_and_ring_rules_linked` | `CERT-WS-001`, `OPS-BUILD-006` |
| `accessibility_and_locale_review_lanes_seeded_or_explicitly_deferred` | `A11Y-CORE-002` |
| `locality_continuity_and_transport_seeds_present` | `GOV-OSS-001`, `GOV-TRUTH-901`, `TOOL-CTX-002` |
| `cli_headless_contract_posture_declared` | `FR-AUTO-001` |
| `docs_control_policy_current` | `GOV-TRUTH-901` |
| `evidence_freshness_current_enough_for_review` | `GOV-EVID-901` |

## How to extend the register

- Add new canonical rows only when there is a new obligation that cannot
  be represented honestly by an existing requirement id.
- Add aliases when an existing source document, packet, or scorecard
  already uses a local label for the same obligation.
- Update the requirement register, this crosswalk, and any affected
  packet or scorecard in the same change.
- Prefer role-neutral or enduring domains in new ids. The foundations
  seed reserves `9xx` rows for current milestone-only governance gaps so
  future long-range rows can land without renumbering existing
  launch-facing ids.
