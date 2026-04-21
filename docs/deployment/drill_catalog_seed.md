# Deployment continuity, disaster-recovery, and impairment drill catalog seed

This document seeds the shared continuity drill catalog Aureline's
release, support, and product-boundary lanes consume together. It
exists so local-only continuity, mirror/offline behavior, cached
policy/auth behavior, remote disconnects, and future managed failover
cases are described once with explicit control-plane versus data-plane
truth.

Companion artifacts:

- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  — machine-readable seed catalog with frozen vocabularies and the
  initial drill rows.
- [`/fixtures/deployment/impairment_cases/`](../../fixtures/deployment/impairment_cases/)
  — concrete impairment-case fixtures the seed rows point at.
- [`/docs/release/install_topology_plan.md`](../release/install_topology_plan.md)
  — release and rollout planning that cites the same drill ids when a
  claimed install or deployment profile carries continuity language.
- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  — boundary rows whose `local_core_continuity` and
  `absence_narrows_to` clauses map back to the drills here.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — support/export control-artifact routing for this catalog and its
  owning lane.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53, §5.57, and Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  §9.8, and the managed-service separation fitness-function rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and
  §11.4.3.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31, §18.42, §22.5,
  Appendix BL, and Appendix DG.
- `.t2/docs/Aureline_Milestones_Document.md` §3.21 and §6.26.

If this document disagrees with those sources, those sources win and
this document plus the YAML seed update in the same change. If this
document and the YAML seed disagree, this document wins and the YAML
updates in the same change.

## Scope

Frozen at this revision:

- one drill-row shape shared by release planning, support/export, and
  boundary-manifest planning;
- a closed impairment-plane vocabulary distinguishing
  `local_only_baseline`, `control_plane`, `data_plane`, and
  `cross_plane`;
- a closed restore-class vocabulary so drills say whether Aureline
  continues locally, replays cached truth, resumes after reconnect, or
  requires manual reconcile after a boundary change;
- required locality, region, tenant, and key-mode fields on every row,
  even when the truthful value is `not_applicable`;
- explicit separation between retained local-safe capabilities and
  blocked managed-only capabilities;
- the initial seed drills for benchmark-lab continuity, docs-pack
  truth, mirror import, stale policy/auth state, remote connector loss,
  and a future managed/local boundary-change case.

Out of scope at this revision:

- live failure-injection automation or a chaos harness;
- final shiproom packets, release-evidence packets, or support-bundle
  schema bodies;
- multi-region failover implementation, production runbooks, or final
  managed-service operating procedures.

## Catalog row contract

Every drill row in the YAML seed carries:

- a stable `drill_id` and human-readable title;
- one `service_domain`, one `impairment_plane`, and one
  `trigger_class`;
- one `control_plane` state and one `workspace_runtime` state aligned
  with the UI/UX continuity-strip vocabulary;
- one locality / region / tenant / key-mode / restore-class posture;
- an `expected_degraded_behavior` sentence written in product terms;
- `retained_local_safe_capabilities` listed separately from
  `blocked_managed_only_capabilities`;
- one or more `evidence_outputs`, plus one `owner_dri`, one
  `freshness_cadence`, and at least one escalation route;
- links back to the release inputs, support/export home, and any
  boundary rows the drill qualifies today;
- one concrete `fixture_ref` into
  [`/fixtures/deployment/impairment_cases/`](../../fixtures/deployment/impairment_cases/).

The row contract exists to prevent generic "service degraded" language.
Every seeded drill must answer:

1. Which plane is impaired?
2. What still works locally and safely?
3. Which managed-only capabilities are intentionally blocked?
4. Which evidence packet, support note, or claim-manifest note proves
   the drill ran?
5. Who refreshes the row and where does escalation go when the row is
   stale or red?

## Impairment-plane taxonomy

| Plane | Meaning | Required truth |
|---|---|---|
| `local_only_baseline` | No managed service is present or required for the exercised path. | The row must still say what hosted or aggregated behavior is absent by design. |
| `control_plane` | Identity, policy, catalog, quota, or compatibility metadata is stale, missing, or mirror-only while local work remains possible. | The row must name cached truth and exactly which fresh-authority actions pause. |
| `data_plane` | Interactive transport, artifact bytes, relay traffic, attach traffic, or upload/download bytes are unavailable while control metadata remains known enough to explain the state. | The row must name which live streams or transfers stop and what local or mirrored fallback remains. |
| `cross_plane` | Impairment or failover changes both control and data assumptions, such as region, tenant, key mode, or approval scope. | The row must stop privileged replay and require an explicit boundary recheck before widening authority again. |

## Restore classes

| Restore class | Meaning |
|---|---|
| `continue_local_no_restore` | No restore object is needed; the truthful path is to keep working locally. |
| `replay_cached_snapshot` | A cached signed or verified snapshot remains authoritative enough for local-safe work. |
| `mirror_snapshot_import` | Recovery proceeds from a signed mirror or offline bundle import rather than a live service fetch. |
| `resume_after_reconnect` | Local work continues now; managed/session state resumes only after transport health returns. |
| `manual_reconcile_after_boundary_change` | Failover or migration changed region, tenant, key, or approval truth; the client must stop and recheck before replay. |

## Seeded drills

| Drill id | Plane | Main failure truth | Local-safe baseline | Primary evidence |
|---|---|---|---|---|
| `benchmark_lab_local_only_capture` | `local_only_baseline` | no managed benchmark control plane is present | local capture, replay, and export remain available | local benchmark result + support continuity note |
| `docs_pack_mirror_only_truth` | `control_plane` | upstream docs control plane is unavailable; verified mirror remains | local docs reading and citation remain available with mirror freshness labels | docs-pack status export |
| `mirror_import_offline_bundle_replay` | `data_plane` | live artifact bytes are unavailable; signed mirror bundle remains importable | installed artifacts continue; mirror snapshot import remains available | mirror import audit record |
| `stale_policy_session_cached_local_safe` | `control_plane` | policy/session freshness is stale | editing, search, Git, local docs, and bundle export remain available | policy cache epoch snapshot |
| `remote_connector_loss_continue_local` | `data_plane` | remote transport or connector drops while local shell remains up | inspect/edit local copies, export diagnostics, queue bounded follow-up | remote session loss packet |
| `failover_boundary_recheck_future_managed_case` | `cross_plane` | failover or migration changed region, tenant, or key-mode assumptions | local work and export stay available; privileged replay pauses | boundary recheck packet |

## Consumption rules

- **Release planning** cites `drill_id` values from this catalog when an
  install profile, rollout ring, mirror posture, or managed claim uses
  continuity language. Release documents must not mint their own outage
  taxonomy.
- **Support/export** uses the same `drill_id`, retained-local-safe set,
  and evidence-output vocabulary in support bundles, Doctor handoff, and
  future field runbooks.
- **Boundary-manifest planning** maps `local_core_continuity` and
  `absence_narrows_to` clauses to drills here whenever a capability row
  depends on optional services, mirrors, or cached policy/auth truth.

## Future extension rules

- New drills are additive-minor when they add rows or vocabulary values
  without repurposing existing meaning.
- Repurposing an existing plane, restore class, or evidence-output term
  is breaking and should land with a decision row.
- Claim-widening for managed, self-hosted, sovereign, or air-gapped
  language should add or refresh drill rows here before the public claim
  widens elsewhere.
