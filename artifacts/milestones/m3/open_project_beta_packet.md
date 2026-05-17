# Open-project beta packet

This packet ties the beta standards/interchange publication and the
public/private issue/RFC routing baseline to the same lifecycle,
support, freshness, and disclosure vocabulary used by the rest of the
beta release-control lane.

It is a governed packet, not marketing copy. The canonical machine
truth is the YAML block below. The validator cross-checks it against:

- `artifacts/governance/standards_matrix.yaml`
- `artifacts/governance/issue_routing.yaml`
- `artifacts/milestones/m3/beta_enablement_starter_pack.yaml`
- `docs/governance/m3/standards_interchange_matrix.md`
- `docs/community/m3/issue_rfc_routing_beta.md`
- `docs/help/m3/community_handoff_beta.md`

If proof is stale, missing, deferred, or bridge-only, downstream
surfaces must narrow the claim instead of widening it.

## Reviewer summary

| Lane | Entry point | Governing source |
|---|---|---|
| Standards and interchange | `docs/governance/m3/standards_interchange_matrix.md` | `artifacts/governance/standards_matrix.yaml` |
| Issue and RFC routing | `docs/community/m3/issue_rfc_routing_beta.md` | `artifacts/governance/issue_routing.yaml` |
| Docs/help consumer | `docs/help/m3/community_handoff_beta.md` | Product-local handoff classes mapped back to canonical issue classes |
| Starter-pack consumer | `artifacts/milestones/m3/beta_enablement_starter_pack.yaml` | Community lane points to the beta issue/RFC entrypoint |

## Canonical machine source

<!-- BEGIN canonical:open_project_beta_packet -->
```yaml
schema_version: 1
packet_id: open_project_beta_packet
milestone_id: m3
release_channel_scope: beta
as_of: "2026-05-17"
owner: "@ahmeddyounis"
packet_state: frozen

validator:
  script_ref: ci/check_m3_open_project_beta_packet.py
  command: python3 ci/check_m3_open_project_beta_packet.py --repo-root .
  validation_capture_ref: artifacts/milestones/m3/captures/open_project_beta_packet_validation_capture.json

human_entrypoints:
  standards_interchange_matrix: docs/governance/m3/standards_interchange_matrix.md
  issue_rfc_routing: docs/community/m3/issue_rfc_routing_beta.md
  legacy_issue_routing_alias: docs/community/m3/public_private_issue_routing.md

canonical_sources:
  standards_matrix: artifacts/governance/standards_matrix.yaml
  standards_deviation_ledger: artifacts/governance/standards_deviation_ledger.yaml
  standards_evidence_gate: docs/governance/standards_adoption_evidence_gate.md
  issue_routing_matrix: docs/governance/issue_routing_matrix.md
  issue_routing_yaml: artifacts/governance/issue_routing.yaml
  beta_enablement_starter_pack: artifacts/milestones/m3/beta_enablement_starter_pack.yaml
  public_proof_index: artifacts/milestones/m3/public_proof_index.md
  claimed_surface_register: artifacts/milestones/m3/claimed_surface_register.json
  publication_shelf_life_policy: docs/governance/m3/publication_shelf_life_policy.md

consuming_surfaces:
  - consumer_id: docs_help_community_handoff
    consumer_ref: docs/help/m3/community_handoff_beta.md
    consumes:
      - docs/community/m3/issue_rfc_routing_beta.md
      - artifacts/governance/issue_routing.yaml
    required_terms:
      - docs_truth_defect
      - compatibility_regression
      - rfc
      - security_issue
      - supportability_escalation
  - consumer_id: docs_help_release_truth
    consumer_ref: docs/help/m3/release_truth_surfaces.md
    consumes:
      - artifacts/milestones/m3/open_project_beta_packet.md
      - docs/governance/m3/standards_interchange_matrix.md
      - docs/community/m3/issue_rfc_routing_beta.md
  - consumer_id: beta_enablement_starter_pack
    consumer_ref: artifacts/milestones/m3/beta_enablement_starter_pack.yaml
    consumes:
      - docs/community/m3/issue_rfc_routing_beta.md

standards_rows:
  - row_id: standard.opentelemetry
    source_support_class: custom_with_bridge_planned
    source_import_expectation: none_planned
    source_export_expectation: placeholder_stub_only
    lifecycle_label: beta
    beta_claim_posture: bridge_reserved
    public_claim_ceiling: OTLP export lane reserved; no live OTLP export claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/opentelemetry_otlp_minimum.yaml
      - docs/governance/telemetry_and_support_schema_registry.md
  - row_id: standard.sarif
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: supported
    source_export_expectation: supported
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: SARIF 2.1.0 import/export only when validator and fixture evidence are current.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/sarif_minimum.yaml
  - row_id: standard.spdx
    source_support_class: standard_shaped_export_only
    source_import_expectation: supported
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: narrowed_export_only
    public_claim_ceiling: SPDX identifiers required; release SBOM conformance remains evidence-gated.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/spdx_sbom_minimum.yaml
      - docs/governance/provenance_and_compliance_baseline.md
  - row_id: standard.cyclonedx
    source_support_class: standard_deferred_placeholder
    source_import_expectation: best_effort
    source_export_expectation: deferred_to_later_milestone
    lifecycle_label: beta
    beta_claim_posture: deferred
    public_claim_ceiling: Reserved only; no CycloneDX export-support claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/cyclonedx_minimum.yaml
  - row_id: standard.reuse
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: required
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: REUSE-shaped SPDX file metadata on new source-bearing files.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/reuse_file_hygiene_minimum.yaml
      - CONTRIBUTING.md
  - row_id: standard.commonmark
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: required
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: CommonMark baseline with declared extensions; no undocumented rendering break.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/commonmark_minimum.yaml
  - row_id: standard.oidc
    source_support_class: standard_shaped_import_only
    source_import_expectation: required
    source_export_expectation: none_planned
    lifecycle_label: beta
    beta_claim_posture: narrowed_import_only
    public_claim_ceiling: OIDC consumption for managed/self-hosted auth only.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/oidc_minimum.yaml
      - docs/identity/offline_entitlement_and_policy_seed.md
  - row_id: standard.scim
    source_support_class: standard_shaped_import_only
    source_import_expectation: required
    source_export_expectation: none_planned
    lifecycle_label: beta
    beta_claim_posture: narrowed_import_only
    public_claim_ceiling: SCIM consumption for managed/self-hosted provisioning only.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/scim_minimum.yaml
      - docs/identity/offline_entitlement_and_policy_seed.md
  - row_id: standard.oci_distribution
    source_support_class: standard_deferred_placeholder
    source_import_expectation: best_effort
    source_export_expectation: deferred_to_later_milestone
    lifecycle_label: beta
    beta_claim_posture: deferred
    public_claim_ceiling: Candidate/reserved transport only; no OCI push/pull compatibility claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/oci_distribution_minimum.yaml
  - row_id: standard.semver
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: required
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: SemVer applies to declared public surfaces; pre-stable surfaces remain explicitly pre-release.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/semver_minimum.yaml
      - docs/governance/interface_lifecycle_policy.md
  - row_id: standard.openfeature
    source_support_class: custom_but_mirrorable
    source_import_expectation: none_planned
    source_export_expectation: none_planned
    lifecycle_label: beta
    beta_claim_posture: bridge_only
    public_claim_ceiling: Custom flag governance with a mirrorable bridge; no OpenFeature API-compatibility claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/openfeature_bridge_minimum.yaml
      - docs/governance/feature_flag_policy.md
  - row_id: standard.opa_rego
    source_support_class: custom_but_mirrorable
    source_import_expectation: none_planned
    source_export_expectation: none_planned
    lifecycle_label: beta
    beta_claim_posture: bridge_only
    public_claim_ceiling: Custom policy bundle with a mirrorable bridge; no Rego language-support claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/opa_rego_bridge_minimum.yaml
      - docs/identity/offline_entitlement_and_policy_seed.md
  - row_id: standard.json_schema_2020_12
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: required
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: JSON Schema Draft 2020-12 for repo schemas unless narrowed by deviation ADR.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/json_schema_2020_12_minimum.yaml
      - schemas/governance/decision_index.schema.json
  - row_id: standard.openapi_3_2
    source_support_class: standard_deferred_placeholder
    source_import_expectation: supported
    source_export_expectation: deferred_to_later_milestone
    lifecycle_label: beta
    beta_claim_posture: deferred
    public_claim_ceiling: Reserved for future HTTP/service APIs; no published OpenAPI document claim.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/openapi_minimum.yaml
  - row_id: standard.wasm_component_model_wit
    source_support_class: standard_shaped_import_and_export
    source_import_expectation: required
    source_export_expectation: required
    lifecycle_label: beta
    beta_claim_posture: claim_bearing
    public_claim_ceiling: WIT/component-model extension ABI within the pinned pre-1.0 range.
    evidence_refs:
      - fixtures/governance/standards_evidence_cases/wit_component_model_minimum.yaml
      - docs/extensions/wit_host_contract_seed.md

issue_routing_rows:
  - issue_class_id: oss_bug
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_immediate
    public_summary_expectation: recommended
    redaction_class: field_safe_default
  - issue_class_id: perf_regression
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_immediate
    public_summary_expectation: required
    redaction_class: field_safe_with_route_metadata
  - issue_class_id: rfc
    default_route_class: public_rfc_forum
    privacy_class: public
    disclosure_class: public_immediate
    public_summary_expectation: required
    redaction_class: field_safe_default
  - issue_class_id: security_issue
    default_route_class: private_security_channel
    privacy_class: private_with_public_advisory
    disclosure_class: public_on_advisory
    public_summary_expectation: required
    redaction_class: security_redaction_raw_allowed_under_pgp
  - issue_class_id: supportability_issue
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_on_fix
    public_summary_expectation: recommended
    redaction_class: field_safe_with_route_metadata
  - issue_class_id: supportability_escalation
    default_route_class: private_support_channel
    privacy_class: private_support_only
    disclosure_class: private_indefinite
    public_summary_expectation: none
    redaction_class: support_bundle_redaction_profile
  - issue_class_id: docs_truth_defect
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_immediate
    public_summary_expectation: required
    redaction_class: field_safe_default
  - issue_class_id: design_review_issue
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_on_fix
    public_summary_expectation: recommended
    redaction_class: field_safe_default
  - issue_class_id: accessibility_defect
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_on_fix
    public_summary_expectation: required
    redaction_class: field_safe_default
  - issue_class_id: compatibility_regression
    default_route_class: public_issue_tracker
    privacy_class: public
    disclosure_class: public_on_fix
    public_summary_expectation: required
    redaction_class: field_safe_with_route_metadata
  - issue_class_id: waiver_request
    default_route_class: governance_packet_queue
    privacy_class: public
    disclosure_class: public_immediate
    public_summary_expectation: required
    redaction_class: field_safe_default
  - issue_class_id: benchmark_dispute
    default_route_class: benchmark_council_queue
    privacy_class: public
    disclosure_class: public_on_fix
    public_summary_expectation: required
    redaction_class: field_safe_default
  - issue_class_id: private_partner_case
    default_route_class: private_partner_channel
    privacy_class: private_partner_only
    disclosure_class: private_indefinite
    public_summary_expectation: none
    redaction_class: partner_contractual_redaction
  - issue_class_id: design_partner_case
    default_route_class: private_partner_channel
    privacy_class: private_partner_only
    disclosure_class: private_indefinite
    public_summary_expectation: none
    redaction_class: partner_contractual_redaction

private_to_public_transition_refs:
  - private_security_to_public_advisory
  - private_partner_to_public_sanitised_summary
  - private_support_to_public_docs_truth
  - public_to_private_reclassification
  - private_support_to_private_security
```
<!-- END canonical:open_project_beta_packet -->

## How to verify

```sh
python3 ci/check_m3_open_project_beta_packet.py --repo-root .
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root .
```
