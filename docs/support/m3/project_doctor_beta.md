# Project Doctor beta probe-pack catalog and finding contract

The beta Project Doctor lane promotes diagnosis from the alpha probe runtime
to a versioned, attributable, confidence-labeled system. Findings stop being
opaque troubleshooting text and become typed records that name the probe pack
that emitted them, the rule they trip, the evidence refs that justify them,
and the confidence and severity classes that bound how the user should treat
them. The probe packs themselves stop being implicit code paths and become a
named catalog of bounded, versioned, read-only-by-default packs that are safe
to run in headless or support-guided sessions.

The implementation lives in
[`crates/aureline-doctor/src/probes/beta.rs`](../../../crates/aureline-doctor/src/probes/beta.rs)
with the support/export consumer at
[`crates/aureline-support/src/project_doctor/beta.rs`](../../../crates/aureline-support/src/project_doctor/beta.rs).
The boundary schema lives at
[`/schemas/support/project_doctor.schema.json`](../../../schemas/support/project_doctor.schema.json)
and the protected fixture corpus lives at
[`/fixtures/support/project_doctor_beta/`](../../../fixtures/support/project_doctor_beta/).

## What this beta row owns

- A typed `ProjectDoctorProbePackCatalog` that names every supported probe
  pack with a stable `pack_id`, `pack_class`, `pack_version`,
  `lifecycle_status`, `read_only_posture`, `headless_admission`, and
  `support_guided_admission`. Closed vocabularies bound every field so the
  catalog can never silently grow a pack that mutates state or skips
  attribution.
- A typed `ProjectDoctorProbePack` wrapper that lets a single pack record be
  exchanged or stored on its own with the same record-kind and schema-version
  discriminators.
- A typed `ProjectDoctorBetaFinding` record that pins every emitted finding
  to a stable `finding_code` (`doctor.finding.*`), the owning probe pack ref,
  a `severity_class`, a `confidence_class`, a `diagnosis_posture`, a
  `recovery_handoff_class`, and the `render_surfaces` (UI finding card, CLI
  finding row, support export row, headless JSON row) that may carry it.
- A typed `AttributionRef` row that names the kind of attribution
  (`probe_pack_ref`, `doctor_rule_ref`, `evidence_ref`,
  `support_bundle_ref`, `runbook_ref`, `escalation_packet_ref`) and the
  opaque reference; every finding must carry at least one
  `probe_pack_ref` attribution that resolves to its declared
  `probe_pack_ref`.
- A `ProjectDoctorBetaEvaluator` that validates the catalog, the bound
  findings, the cross-reference from each finding back to its pack, and that
  refuses combinations that violate the read-only and admission contracts.
  The evaluator folds one catalog plus a batch of findings into a metadata-
  safe `ProjectDoctorBetaSupportPacket` quoting the doc and schema refs
  verbatim.

## Acceptance and how this row meets it

- **Doctor findings are typed, attributable, and confidence-labeled
  instead of opaque troubleshooting text.** Every
  `project_doctor_finding_record` carries a closed `finding_code` prefixed
  `doctor.finding.`, a closed `severity_class`, a closed `confidence_class`,
  and a non-empty `attribution_refs` list that includes a `probe_pack_ref`
  attribution. The evaluator refuses an empty evidence list, a missing pack
  attribution, an empty summary, an unknown pack ref, or a pack-class or
  version that does not match the catalog entry the finding claims to come
  from.
- **Probe packs are versioned, read-only by default, and safe to run in
  headless or support-guided scenarios.** Every pack carries a
  `pack_version`, a closed `lifecycle_status`, a
  `read_only_posture` of either `read_only_by_default_no_mutation` or
  `metadata_local_evidence_only`, and explicit headless and support-guided
  admission classes. The evaluator refuses a pack that is denied under both
  headless and support-guided admission, a default redaction class that is
  not metadata-safe, and a finding whose support context is denied by the
  pack's admission posture.
- **UI, CLI, and support exports can render the same finding packet.**
  Every finding declares a non-empty `render_surfaces` set drawn from the
  closed `ui_finding_card`, `cli_finding_row`, `support_export_row`, and
  `headless_json_row` vocabulary. The `ProjectDoctorBetaSupportPacket`
  projection bundles the catalog rows and finding rows verbatim, pins
  `raw_private_material_excluded = true` and
  `ambient_authority_excluded = true`, and quotes the doc and schema refs
  used by every consuming surface, so the headless JSON, support export,
  CLI, and UI all reconstruct the same packet from the same record.

## Failure-drill posture

The evaluator fails closed before widening diagnosis or hiding evidence:

- A catalog without a pack, with duplicate `pack_id`s, with a missing or
  mismatched `schema_ref` / `doc_ref`, or with a pack whose
  `default_redaction_class` is not metadata-safe is refused.
- A pack with no supported finding codes, mis-prefixed finding codes,
  duplicate codes, no supported support contexts, no supported recovery
  handoffs, or denied under both headless and support-guided admission is
  refused.
- A finding whose `finding_code` does not start with `doctor.finding.`,
  whose record kind or schema version is wrong, whose attribution list is
  missing the `probe_pack_ref` row, whose evidence list is empty, whose
  render surfaces are empty, or whose redaction class is not metadata-safe
  is refused.
- A finding whose `probe_pack_ref` does not resolve to a catalog entry,
  whose `probe_pack_class` or `probe_pack_version` does not match the
  declared pack, whose `finding_code` is not in the pack's
  `supported_finding_codes`, whose `support_context_class` is not in the
  pack's `supported_support_contexts`, or whose `recovery_handoff_class`
  is not in the pack's `supported_recovery_handoffs` is refused.
- A finding emitted under `cli_headless` against a pack that denies
  headless admission, or emitted under `support_guided` against a pack
  that denies support-guided admission, is refused.

## First consumers

- The `aureline-support` beta module is the canonical projection for
  support-export and recovery-ladder review.
  `aureline_support::project_doctor::beta::beta_support_packet` folds one
  catalog and its bound findings into a metadata-safe
  `ProjectDoctorBetaSupportPacket` that the support-export pipeline can
  serialize verbatim.
- The boundary schema is the contract the headless export writer, the CLI
  finding renderer, and the support-export chrome share — every surface
  reconstructs the same packet shape from the on-disk record verbatim,
  never re-derives it from a side channel.

## Related contracts

- [Project Doctor alpha contract](../project_doctor_contract_alpha.md) —
  the parent alpha lane this beta promotes; alpha runtime findings still
  flow through the alpha runtime projection.
- [Project Doctor probe contract](../project_doctor_probe_contract.md) —
  the read-only / no-hidden-side-effects rule the beta packs inherit.
- [Safe-mode beta](safe_mode_beta.md) — the parent safe-mode profile that
  cites a Project Doctor finding ref. Beta findings emitted here are the
  refs safe-mode profiles point back to.
- [Extension-bisect beta](extension_bisect_beta.md) — extension bisect
  sessions cite a Project Doctor finding ref; the beta finding code is the
  stable id the bisect support packet pins.

## Out of scope for this beta row

- Live runtime probe orchestration (scheduling, retries, throttling) —
  beta consumers ingest typed records produced by the owning surfaces.
- Cross-tenant managed catalogue distribution; the catalog ships as a
  protected fixture under `fixtures/support/project_doctor_beta/`.
- Hosted support-bundle ingestion or live tickets; beta findings carry
  refs that headless export and support-bundle preview can serialize, but
  intake transport lands in later milestones.
