# Proof packet: M1 locality / tenancy / key-mode vocabulary seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical locality / tenancy / key-mode vocabulary
seed. The lane proves the seed is consumable by the shell Help/About
pane, the service-health row, the support-bundle exporter, the
release-evidence pack, the docs/help truth rows, and the runtime /
CI validation lane — without re-encoding the locality, tenancy,
key-storage-mode, local-safe-fallback, residency-disclosure,
truth-badge, or diagnostic-surface vocabularies on each surface.

Reviewer entry point:
[`/docs/governance/m1_locality_tenancy_keymode_vocabulary.md`](../../../docs/governance/m1_locality_tenancy_keymode_vocabulary.md).
Upstream internal boundary manifest:
[`/artifacts/governance/m1_open_local_capability_matrix.yaml`](../../../artifacts/governance/m1_open_local_capability_matrix.yaml).

## Canonical sources

- `artifacts/governance/locality_examples.yaml` — seed rows the
  runner consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `upstream_boundary_manifest_ref`,
    `upstream_connected_provider_seed_ref`, `row_schema_ref`,
    `build_identity_ref`, `validation_lane_ref`),
  - closed envelope vocabularies for locality class, tenancy scope
    class, key storage mode class, local-safe-fallback class,
    data-residency disclosure class, truth-badge class, diagnostic-
    surface class, surface-family class, and failure-drill id,
  - required coverage lists (locality classes, tenancy scopes,
    key-storage modes, diagnostic surfaces),
  - the named runtime consumers the seed asserts are live, and
  - one locality/tenancy/key-mode profile row per typed scenario
    with the uniform shape
    `(locality_tenancy_keymode_profile_id, surface_family_class,
    locality_class, tenancy_scope_class, key_storage_mode_class,
    local_safe_fallback_class, data_residency_disclosure_class,
    truth_badge_classes, diagnostic_surface_classes,
    local_core_continuity, absence_narrows_to, owner_dri,
    failure_drill)`.

- `schemas/governance/m1_locality_tenancy_keymode_seed.schema.json`
  — envelope schema; freezes vocabularies, required coverage lists,
  named-consumer shape, matrix identity, and pins the canonical
  landing-page path.

- `schemas/governance/locality_tenancy_keymode.schema.json` — row
  schema; freezes the closed locality_class (`local_only`,
  `remote_target`, `provider_linked`,
  `managed_control_plane_bearing`, `mirrored`, `unknown_locality`),
  tenancy_scope_class, key_storage_mode_class,
  local_safe_fallback_class, data_residency_disclosure_class,
  truth_badge_class, diagnostic_surface_class, and
  surface_family_class vocabularies, plus the conditional
  invariants (local_only ⇒ local sentinels on tenancy / key-mode /
  residency / local-safe-fallback; managed_control_plane_bearing ⇒
  managed-eligible tenancy / key-mode / residency and an explicit
  local_safe_fallback_class; unknown_locality ⇒ unknown_tenancy
  AND unknown_key_mode AND residency_unknown; provider_linked ⇒
  no local-only sentinel on tenancy or key-mode).

- `tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Upstream sources the seed projects against

- `artifacts/governance/m1_open_local_capability_matrix.yaml` —
  upstream internal boundary manifest (boundary classes local_only,
  provider_linked, managed, mirrored, unsupported across the
  protected surface families). The M1 locality/tenancy/key-mode
  seed projects the boundary classes into the more granular locality
  vocabulary (local_only → local_only or remote_target;
  provider_linked → provider_linked; managed →
  managed_control_plane_bearing) without forking the upstream's
  boundary-class vocabulary.
- `docs/providers/m1_connected_provider_seed.md` — upstream
  connected-provider seed for provider-linked rows; the validation
  lane resolves the doc on disk so the seed cannot quietly outlive
  its upstream.

## Named runtime consumers

- `docs/governance/m1_locality_tenancy_keymode_vocabulary.md` —
  reviewer-facing landing page that quotes the seeded rows verbatim
  so the shell Help/About pane, the service-health row, the
  support-bundle exporter, the release-evidence pack, and the
  runtime/CI validation lane all read the same vocabulary.
- `artifacts/governance/m1_open_local_capability_matrix.yaml` —
  upstream internal boundary manifest; the runner asserts the
  matrix resolves on disk so the M1 seed cannot quietly outlive
  its upstream.
- `tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py`
  — live CI/review consumer (this lane) that replays the seed,
  asserts closed-vocabulary agreement with the row schema, the
  conditional invariants, required coverage, named-consumer
  resolution, and reproduces every named failure drill loudly.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/locality_tenancy_keymode_vocabulary_validation_capture.json`

## Locality / tenancy / key-mode profile coverage

The seed asserts the following profiles are present as typed rows:

| `locality_tenancy_keymode_profile_id` | Surface | Locality | Tenancy | Key mode | Residency |
| --- | --- | --- | --- | --- | --- |
| `local_only.editor_buffer_save` | `editor` | `local_only` | `not_applicable_local_only` | `not_applicable_local_only` | `residency_local_device_only` |
| `remote_target.workspace_remote_agent_attach` | `workspace` | `remote_target` | `single_user_local` | `os_keychain_backed` | `residency_user_owned_remote_target` |
| `provider_linked.symbol_service_attach` | `symbol_service` | `provider_linked` | `single_user_local` | `byok_user_managed` | `residency_provider_default` |
| `managed_control_plane.companion_notification_channel_seed` | `companion_notification_channel` | `managed_control_plane_bearing` | `org_tenant` | `managed_service_kms` | `residency_managed_tenant_documented_region` |
| `managed_control_plane.hosted_control_plane_uncertainty` | `hosted_control_plane_prototype` | `managed_control_plane_bearing` | `unknown_tenancy` | `unknown_key_mode` | `residency_unknown` |
| `unknown_locality.prototype_surface_under_review` | `hosted_control_plane_prototype` | `unknown_locality` | `unknown_tenancy` | `unknown_key_mode` | `residency_unknown` |

The union of every row covers all five M1 required locality
classes (`local_only`, `remote_target`, `provider_linked`,
`managed_control_plane_bearing`, `unknown_locality`), names every
diagnostic surface (`help_about_pane`, `service_health_row`,
`support_export_section`, `release_evidence_pack`, `ci_validation`),
and exercises the closed tenancy and key-mode vocabularies with
explicit uncertainty tokens on the managed and unknown-locality
rows.

## Failure-drill coverage

Six named drills, all reproducible under
`--force-drill <locality_tenancy_keymode_profile_id>:<drill_id>`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `local_only.editor_buffer_save` | `locality_tenancy_keymode_drill.local_only_relaxed_to_managed` | `locality_tenancy_keymode.local_only_invariant_relaxed_to_managed` |
| `remote_target.workspace_remote_agent_attach` | `locality_tenancy_keymode_drill.remote_target_residency_widened_to_provider_default` | `locality_tenancy_keymode.remote_target_residency_disclosure_widened` |
| `provider_linked.symbol_service_attach` | `locality_tenancy_keymode_drill.provider_linked_tenancy_relaxed_to_local_only_sentinel` | `locality_tenancy_keymode.provider_linked_tenancy_class_relaxed_to_local_only_sentinel` |
| `managed_control_plane.companion_notification_channel_seed` | `locality_tenancy_keymode_drill.managed_control_plane_drops_local_safe_fallback_disclosure` | `locality_tenancy_keymode.managed_control_plane_local_safe_fallback_must_not_be_unavailable` |
| `managed_control_plane.hosted_control_plane_uncertainty` | `locality_tenancy_keymode_drill.managed_control_plane_certifies_unknown_tenancy` | `locality_tenancy_keymode.managed_control_plane_uncertainty_token_must_not_be_silently_certified` |
| `unknown_locality.prototype_surface_under_review` | `locality_tenancy_keymode_drill.unknown_locality_certifies_key_mode` | `locality_tenancy_keymode.unknown_locality_key_storage_mode_must_be_unknown_key_mode` |

The protected-walk drill named in the spec — *Enter a managed/control-plane-bearing
surface with unknown tenancy or key mode and confirm labels show
uncertainty rather than false certainty* — is the
`managed_control_plane.hosted_control_plane_uncertainty` row's
`locality_tenancy_keymode_drill.managed_control_plane_certifies_unknown_tenancy`
drill.

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the upstream internal boundary manifest the seed projects against,
  or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocabulary
membership (locality_class, tenancy_scope_class,
key_storage_mode_class, local_safe_fallback_class,
data_residency_disclosure_class, truth_badge_classes,
diagnostic_surface_classes, surface_family_class), the conditional
invariants (local_only / managed_control_plane_bearing /
unknown_locality / provider_linked agreement and the
managed/residency-uncertainty consistency invariant), the required
coverage rules (locality classes, tenancy scopes, key-storage modes,
diagnostic surfaces), named-runtime-consumer existence, and its six
named failure drills.
