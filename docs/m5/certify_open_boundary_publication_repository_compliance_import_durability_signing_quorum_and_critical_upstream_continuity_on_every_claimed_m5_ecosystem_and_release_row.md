# Certify open-boundary publication, repository compliance, import durability, signing quorum, and critical-upstream continuity on every claimed M5 ecosystem and release row

This document is the human-readable companion to the canonical open-durability certification register checked in at `artifacts/governance/m5-open-durability-certification.json` and described by the schema at `schemas/governance/m5-open-durability-certification.schema.json`. The typed consumer is `aureline_governance::m5_open_durability_certification`.

## Purpose

Six sibling registers each make one durability axis inspectable, but none certifies a whole claimed row across all of them at once:

- the versioned boundary-manifest register (`artifacts/governance/m5-versioned-boundary-manifests.json`) publishes the open-versus-paid boundary per family;
- the repository-compliance and notice-binding register (`artifacts/governance/m5-compliance-and-notice-binding.json`) binds REUSE/SPDX/notice/SBOM hygiene;
- the import-provenance and fork-review register (`artifacts/governance/m5-import-provenance-and-fork-review.json`) attributes third-party and generated imports;
- the release-authority continuity register (`artifacts/governance/m5-release-authority-continuity.json`) names signer quorum and backup coverage;
- the emergency-response evidence register (`artifacts/governance/m5-emergency-response-evidence.json`) records advisory/revocation/disable drills;
- the critical-upstream health register (`artifacts/governance/m5-critical-upstream-health.json`) rates the protected-path dependencies.

A claimed row could therefore carry a green boundary card while its critical import is ownerless, or a healthy upstream while its emergency authority is one irreplaceable human. This register is that certification layer. For every claimed M5 ecosystem/release row it records one copy-safe record binding the six durability axes:

- the **boundary manifest** — the versioned open-boundary manifest is published and release-linked, with no hidden proprietary baseline (`boundary.state`, `manifest_published`, `proprietary_baseline_hidden`);
- the **repository compliance** — REUSE/SPDX licensing is current and the notice inventory and SBOM are bound (`compliance.state`, `licensing_current`, `notice_sbom_bound`);
- the **import durability** — third-party/generated import provenance is attributed and every critical import is owned (`import_durability.state`, `provenance_attributed`, `critical_import_owned`);
- the **signer authority** — the signer quorum is met and the emergency authority is not one irreplaceable human (`authority.state`, `required_distinct_humans`, `available_distinct_humans`, `backup_present`);
- the **emergency response** — the advisory/revocation/disable drill evidence is current (`emergency.state`, `drill_current`);
- the **critical upstream** — the protected-path dependencies are healthy and owned (`upstream.state`, `upstream_healthy`).

The same certification truth is published for ecosystem rows (extension/provider, registry) and release rows (artifact-graph, channels) — so an ownerless ecosystem import cannot hide behind a healthy release row. Every row kind is exercised by at least one record (`row_kind`).

## The anti-patterns the spec forbids

The register makes the three "do not certify" guardrails from the source documents impossible to ship silently. Each is a first-class narrowing reason and a first-class summary count:

- **A row may not be certified open while it depends on a hidden proprietary baseline.** `boundary.proprietary_baseline_hidden` narrows on the boundary axis as `hidden_proprietary_baseline` and is counted in `summary.hidden_proprietary_baseline_gaps`.
- **A row may not be certified while it depends on an ownerless critical import.** `import_durability.critical_import_owned = false` narrows on the import axis as `ownerless_critical_import` and is counted in `summary.ownerless_critical_import_gaps`.
- **A row may not be certified while its emergency authority is a single irreplaceable human.** An authority with one available human or no backup narrows on the authority axis as `single_person_emergency_authority` and is counted in `summary.single_person_authority_gaps`.

Each record carries a `scan_posture` (what the certification scan found) and a `surface_posture` (what the service-health/release-center/support surface shows). The two **must agree**, and every structural gap surfaces its reason, so a green certification card can never mask a hidden proprietary baseline, an ownerless critical import, a single-person emergency authority, a stale notice/SBOM, an uncovered drill, or an unhealthy upstream. A certification gap on a row still claiming a label at or above the cutline holds promotion through the stop rule recorded in `publication`.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence (the three guardrails lead: a single-person emergency authority, then an ownerless critical import, then a hidden proprietary baseline; then upstream, emergency, compliance, and finally proof-staleness):

- `narrowed_boundary` — the boundary manifest is unpublished (`boundary_manifest_missing`) or rests on a hidden proprietary baseline (`hidden_proprietary_baseline`).
- `narrowed_compliance` — REUSE/SPDX licensing is stale (`repository_compliance_stale`) or the notice/SBOM binding is missing (`notice_binding_missing`).
- `narrowed_import` — an import lacks provenance (`import_provenance_missing`) or a critical import is ownerless (`ownerless_critical_import`).
- `narrowed_authority` — the signer quorum is unmet (`signer_quorum_unmet`) or the emergency authority is one irreplaceable human (`single_person_emergency_authority`).
- `narrowed_emergency` — the advisory/revocation/disable drill evidence is stale (`emergency_response_stale`).
- `narrowed_upstream` — a protected-path dependency is red-risk or unowned (`critical_upstream_unhealthy`).
- `narrowed_stale` — the certification proof packet, owner sign-off, or waiver thinned out (`certification_proof_stale`, `certification_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **certified** record has a published boundary manifest, current compliance, attributed and owned imports, a met quorum with a backup authority, a current drill, healthy upstreams, fresh proof, and an owner sign-off. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

## Inherited vs. certification narrowing, and the promotion gate

The register separates a *certification* failure from an *inherited* one:

- A row that is **release-blocking**, declares a label **at or above the cutline** (Stable/LTS), is **narrowed** by a certification gap, and is **not** held by an unexpired waiver holds promotion. Its id appears in `publication.blocking_record_ids` and the firing stop rules in `publication.blocking_rule_ids`; `publication.decision` is then `hold`.
- A row already below the cutline (Beta/Preview), or one whose gap is held by an unexpired waiver, is **gated upstream**: it stays visible and narrowed but does not itself hold promotion.

The proof is referenced from the canonical M5 evidence index (`source_contract_refs.m5_evidence_index_ref`) and the stable-promotion packet (`source_contract_refs.stable_promotion_packet_ref`) so open-project durability is visible outside governance meetings. Each record reuses the train-wide `family`, `support_class`, lifecycle-label, proof-packet, waiver, and owner-sign-off vocabulary rather than minting a local synonym set, and projects a copy-safe reuse row for the Help/About, service-health, release-center, and support-export surfaces named in `surfaces`.

## Verification

- `cargo test -p aureline-governance --test m5_open_durability_certification` runs the protected register contract tests: the checked-in register parses and validates, every row kind and reason is wired, the typed model agrees with the frozen validation capture (`artifacts/governance/captures/m5-open-durability-certification_validation_capture.json`), a green surface never masks one of the three guardrails, a certification failure on a still-stable row holds promotion while inherited and waived narrowings do not, and the checked-in negative fixtures (`fixtures/governance/m5-open-durability-certification/`) are all rejected by the typed model.
- `python3 tools/regenerate_m5_open_durability_certification.py` regenerates the register artifact, the negative fixtures, the cases manifest, and the frozen validation capture from one source of truth.
