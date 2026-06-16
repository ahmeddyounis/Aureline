# Track third-party import provenance, generated-code records, and local-fork sponsor/replace decisions for protected M5 dependencies

This document is the human-readable companion to the canonical import-provenance and local-fork review register checked in at `artifacts/governance/m5-import-provenance-and-fork-review.json` and described by the schema at `schemas/governance/m5-import-provenance-and-fork-review.schema.json`. The typed consumer is `aureline_governance::m5_import_provenance_and_fork_review`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) records, per asset lane, *whether* a third-party-import or generated-code control is satisfied as one coarse satisfied/unsatisfied flag, and the repository-compliance and notice-binding register (`artifacts/governance/m5-compliance-and-notice-binding.json`) publishes DCO/CLA, licensing, and SBOM/notice truth per artifact family. Neither makes each protected-path import inspectable as a durable record rather than an ad hoc build note: where it came from, what license it carries, which upstream version it pins, how far it has diverged, who owns its updates, who generated it and how to regenerate it, and — for a long-lived fork or an effectively single-source import — whether an explicit sponsor/fork/replace decision and a current divergence review exist.

This register is that import-truth layer. For every protected-path import used by an M5 family it records one record that states, in one copy-safe record:

- the **import provenance** — whether the origin is `attributed` (`origin_state`), the SPDX license is `identified` (`license_state`, `spdx_license_id`), and the upstream version is `pinned` (`upstream_pin_state`, `upstream_version`);
- the **update ownership** — whether the import has an assigned update owner (`ownership_state`, `update_owner_ref`), so a critical import is never left ownerless because it is "just build-time";
- the **divergence profile** — the local-modification posture (`divergence_state`), the divergence age, and the divergence-review state (`review_state`);
- the **sponsor/fork/replace decision** — required for a long-lived fork or single-source import (`decision_state`, `disposition`), so a curated dependency never drifts into quiet permanent divergence;
- the **generated-code provenance** — for checked-in generated code, whether the generator identity and the regeneration path are recorded (`generator_identity_present`, `regeneration_path_present`).

The same import truth is published for vendored third-party imports, checked-in generated artifacts, long-lived local forks, and effectively single-source curated imports — so a gap on a generated artifact or a quietly drifting fork cannot hide behind a clean vendored import. Every import kind is exercised by at least one record.

## The two anti-patterns the spec forbids

The register makes the two guardrails from the source documents impossible to ship silently:

- **A critical import may not remain ownerless or provenance-free because it is "just build-time".** Each record carries a `manifest_scan_posture` (what the dependency-health/import scan found) and a `surface_posture` (what the user/admin import surface shows). The two **must agree**, and an import left ownerless still narrows on the ownership axis even when its license and SBOM look clean. Every structural gap surfaces its reason (an `origin_unattributed` or `update_owner_missing` gap can never be present without its narrowing reason), and the per-dimension control state is derived from the facts, so a control can never assert `satisfied` over a gap.
- **Generator identity and regeneration posture may not be buried for checked-in generated code.** A `generated_artifact` record whose generator identity is missing narrows on the generator axis, as does one whose regeneration path is missing — the checked-in generated code stays reproducible and attributable rather than opaque.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence:

- `narrowed_provenance` — the origin is unattributed, the license is unidentified, or the upstream version is floating (`origin_unattributed`, `license_unidentified`, `upstream_version_floating`).
- `narrowed_ownership` — the import has no assigned update owner (`update_owner_missing`).
- `narrowed_divergence` — a divergence review is stale or missing, or a sponsor/fork/replace decision is missing on a long-lived fork or single-source import (`divergence_review_stale`, `divergence_review_missing`, `decision_record_missing`).
- `narrowed_generator` — checked-in generated code does not record its generator identity or its regeneration path (`generator_identity_missing`, `regeneration_path_missing`).
- `narrowed_stale` — the proof packet, owner sign-off, or waiver thinned out (`import_proof_stale`, `import_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **cleared** record has attributed provenance, an assigned owner, a current review and recorded decision where required, complete generated-code provenance, fresh proof, and an owner sign-off. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a subject whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. An **import-layer** failure on a subject whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Long-lived forks and single-source imports

A `local_fork` or `curated_single_source` import is long-lived by nature, so it is required to carry an explicit sponsor/fork/replace decision (`disposition` is one of `sponsor_upstream`, `maintain_fork`, or `replace_dependency`) and, while it diverges, a current divergence review. A pending decision (`decision_record_missing`) or a stale/missing review narrows the record on the divergence axis instead of letting the dependency drift permanently. The decision and review references join the architecture-board review inputs recorded in `artifacts/governance/upstream_health_scorecard.yaml`.

## Consumption

Downstream Help/About, service-health, release-center, support-export, and architecture-board surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the import kind, the declared and effective labels, the support class, the import state, the scan/surface-agreement flag, the divergence posture, the recorded decision disposition, the active reasons, and the reuse surfaces for every record.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_import_provenance_and_fork_review.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_import_provenance_and_fork_review/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_import_provenance_and_fork_review.rs`, run by `.github/workflows/check_m5_import_provenance_and_fork_review.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
