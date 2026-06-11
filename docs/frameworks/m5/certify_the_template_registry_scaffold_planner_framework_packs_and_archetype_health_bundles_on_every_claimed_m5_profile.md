# M5 Template-Registry, Scaffold-Planner, Framework-Pack, and Archetype-Health Certification

This document is the contract for the M5 certification packet that qualifies
every claimed M5 template, scaffold, framework-pack, and archetype-health
*profile* on the mainline branch. Where the
[frozen maturity matrix](freeze_the_m5_template_registry_framework_pack_and_support_class_matrix.md)
locks four lanes at lane granularity, this packet certifies the individual
profiles that feed those lanes and aggregates their verdicts into a single
promotion verdict. Galleries, framework-pack headers, scaffold-run and
diff-review surfaces, diagnostics, Help/About surfaces, and support exports
ingest the checked-in packet rather than cloning status text.

- Record kind: `certify_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile`
- Schema: [`schemas/templates/certify-the-template-registry-scaffold-planner-framework-packs-and-archetype-health-bundles-on-every-claimed-m5-profile.schema.json`](../../../schemas/templates/certify-the-template-registry-scaffold-planner-framework-packs-and-archetype-health-bundles-on-every-claimed-m5-profile.schema.json)
- Canonical support export: [`artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/support_export.json`](../../../artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/support_export.json)
- Summary artifact: [`artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md`](../../../artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md)
- Fixtures: [`fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/`](../../../fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/)
- Producer / first consumer: `aureline_templates::certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile::certify_from_current_exports`
- Reader: `aureline_templates::certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile::current_m5_template_certification_export`

## Certified profiles

Each profile binds a certification verdict to its upstream evidence packet — the
record kind, support-export artifact, schema, and contract doc that back the
claim — plus the downgrade triggers, rollback posture, and proof freshness for
that profile.

| Profile | Lane | Claimed | Verdict | Evidence packet |
| --- | --- | --- | --- | --- |
| `signed_template_registry` | `template_registry` | Stable | Certified | [signed template registry](implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows.md) |
| `generation_recovery` | `scaffold_planner` | Stable | Certified | [generation diff-review and recovery](add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty.md) |
| `framework_generator_run` | `scaffold_planner` | Stable | Certified | [framework generators / codemods](implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse.md) |
| `framework_pack_header` | `framework_pack` | Stable | Certified | [framework-pack headers](implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners.md) |
| `richer_framework_packs` | `framework_pack` | Beta | Narrowed | [richer framework packs](add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter.md) |
| `app_topology_views` | `framework_pack` | Stable | Certified | [route/component/topology views](ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth.md) |
| `convention_diagnostics` | `framework_pack` | Beta | Narrowed | [convention diagnostics](add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure.md) |
| `archetype_health_bundle` | `archetype_health` | Stable | Certified | [certified archetype health bundles](ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance.md) |

The richer framework-pack catalog and convention diagnostics are claimed at Beta
and certified as `narrowed`: both lanes legitimately carry bridged, heuristic, or
confidence-labeled rows that must never be presented as exact first-party truth,
so the certification narrows the claim rather than overstating it.

## First-consumer certification

`certify_from_current_exports` is the first real consumer of the claimed
profiles: it reads each profile's checked-in support export through that
profile's own producer and certifies the profile only when its evidence
currently validates. A profile whose upstream export fails to parse or validate
is recorded as `blocked`, so a stale or underqualified profile narrows the
certification automatically instead of leaving it greener than the evidence. The
checked-in support export is exactly the packet that function produces, so
`cargo test -p aureline-templates` fails if any upstream export drifts.

## Compatibility report

`compatibility_report` aggregates the per-profile verdicts: counts of certified,
narrowed, blocked, and uncertified profiles; `all_profiles_publishable` (true
only when no profile is blocked or uncertified); and a human-readable promotion
note. The report must agree with the profile verdicts or the packet fails
validation, so the report can never claim more than the profiles prove.

## Downgrade automation and freshness

`apply_downgrade_automation` takes per-profile observations (evidence validity,
proof freshness, upstream narrowing) and narrows the affected profiles: invalid
evidence moves a profile to `blocked`; stale proof or a narrowed upstream narrows
a `certified` profile to `narrowed_certified`; the per-profile `proof_fresh` flag
is updated. The report is recomputed so CI or release tooling can fail promotion
or narrow the claim automatically. The packet-level `proof_freshness` carries the
SLO (168 hours) and last-refresh timestamp; `auto_narrow_on_stale` records that
stale proof narrows the certification. Supported downgrade triggers are
`proof_stale`, `evidence_packet_invalid`, `policy_blocked`, `signature_unverified`,
`template_revision_unavailable`, `scaffold_preview_unavailable`,
`support_class_narrowed`, `heuristic_presented_as_exact`, `archetype_health_stale`,
`lineage_truth_missing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/)
show a framework-pack-header profile blocked on invalid evidence and an
archetype-health profile narrowed on stale proof; both remain valid packets
because narrowing is explicit, not hidden.

## Boundary

Raw template source bodies, raw generator output, raw provider payloads,
repository URLs, hostnames, credentials, and secret values never cross this
boundary. The packet carries only metadata, certification verdicts, and contract
references. Starter convenience never outruns provenance, preview, or rollback,
and framework packs never present heuristic or bridge behavior as exact
first-party truth without current support-class and downgrade cues.
