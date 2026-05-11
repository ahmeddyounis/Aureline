# Locality / tenancy / key-mode vocabulary (M1 governance seed)

This document is the reviewer-facing landing page for the canonical
M1 **locality / tenancy / key-mode vocabulary seed**: the typed model
that later UI, support, release, and review surfaces consume so a
managed or control-plane-bearing surface cannot silently certify
tenant region, key custody, or residency that the product does not
back, and so a local-only surface cannot silently widen into a
managed claim without an explicit decision row.

This seed is not a managed-tenancy or key-management runtime. It
freezes the **vocabulary** and the **typed profile rows** so the
eventual Help/About pane, the service-health row, the support-bundle
exporter, the release-evidence pack, and the runtime / CI validation
lane all point to the same contract. M1 surfaces label locality,
tenancy, and key mode; they do NOT enforce residency, isolate
tenants, custody keys, or rotate provider credentials.

If this document and the row / envelope schemas disagree, the
schemas win and this document must be updated in the same change.
The upstream internal boundary manifest at
[`artifacts/governance/m1_open_local_capability_matrix.yaml`](../../artifacts/governance/m1_open_local_capability_matrix.yaml)
freezes the broader boundary-class vocabulary (local_only,
provider_linked, managed, mirrored, unsupported); the M1
locality / tenancy / key-mode seed projects against that contract
and freezes the higher-level **locality / tenancy / key-mode profile
rows** every protected M1 surface and every prototype
managed/control-plane-bearing surface reads.

## Canonical sources

- Seed YAML:
  [`artifacts/governance/locality_examples.yaml`](../../artifacts/governance/locality_examples.yaml).
- Envelope schema:
  [`schemas/governance/m1_locality_tenancy_keymode_seed.schema.json`](../../schemas/governance/m1_locality_tenancy_keymode_seed.schema.json).
- Row schema:
  [`schemas/governance/locality_tenancy_keymode.schema.json`](../../schemas/governance/locality_tenancy_keymode.schema.json).
- Upstream internal boundary manifest:
  [`artifacts/governance/m1_open_local_capability_matrix.yaml`](../../artifacts/governance/m1_open_local_capability_matrix.yaml)
  / [`docs/governance/m1_boundary_manifest.md`](./m1_boundary_manifest.md).
- Upstream connected-provider seed (for provider-linked rows):
  [`docs/providers/m1_connected_provider_seed.md`](../providers/m1_connected_provider_seed.md).
- Validation lane:
  [`tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py`](../../tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py).
- Proof packet:
  [`artifacts/milestones/m1/proof_packets/locality_tenancy_keymode_vocabulary.md`](../../artifacts/milestones/m1/proof_packets/locality_tenancy_keymode_vocabulary.md).

## Why a typed locality / tenancy / key-mode seed now

The upstream boundary manifest names whether each protected surface
family is local_only, provider_linked, managed, mirrored, or
unsupported. Without a higher-level **profile-row seed** every M1
surface that wants to *render* locality / tenancy / key-mode truth
would invent its own taxonomy: the shell would render "managed"
without a tenancy scope, the support exporter would print "key mode:
managed" without naming whether the key is BYOK or hosted, the
release pack would imply a tenant region that the seed cannot back,
and a prototype hosted control-plane surface would have nowhere to
publish "tenancy: unknown" without inventing an ad-hoc label.

This seed closes that gap before any managed/control-plane-bearing
surface ships:

- The locality vocabulary is **closed**. Six tokens
  (`local_only`, `remote_target`, `provider_linked`,
  `managed_control_plane_bearing`, `mirrored`, `unknown_locality`)
  cover every M1 surface family.
- The tenancy-scope and key-storage-mode vocabularies are **closed**
  AND carry an **explicit uncertainty token** (`unknown_tenancy`,
  `unknown_key_mode`). Surfaces that cannot truthfully name their
  tenancy or key mode publish the uncertainty token verbatim
  rather than guess.
- The local-safe-fallback class is **typed and required on every
  managed row** so a managed claim cannot quietly remove the
  local-safe floor by omission.
- The data-residency-disclosure class is **closed** and tied to the
  locality class so a managed row cannot publish
  `residency_local_device_only` and a local-only row cannot publish
  `residency_managed_tenant_documented_region`.

## Closed vocabularies

The seed envelope freezes these vocabularies. The row schema's
`$defs` is the canonical source; the envelope vocabulary MUST agree.

### Locality class

| Token | Meaning |
| --- | --- |
| `local_only` | Surface works with no network, no sign-in, and no hosted service. |
| `remote_target` | Surface attaches to a user-controlled remote target (SSH/container/WSL/dev VM) the user named themselves. |
| `provider_linked` | Optional user-supplied provider (BYOK / customer IdP / user-owned LSP / local model) attaches to the local surface. |
| `managed_control_plane_bearing` | Surface carries a hosted control-plane bearer claim (managed sync, managed companion, hosted optional surface). M1 labels only. |
| `mirrored` | Reachable via a signed offline mirror/bundle in constrained-connectivity profiles. |
| `unknown_locality` | Explicit uncertainty token; surfaces that cannot truthfully name their locality MUST publish this rather than guess. |

### Tenancy scope class

| Token | Meaning |
| --- | --- |
| `not_applicable_local_only` | Surface has no tenancy concept (local-only). |
| `single_user_local` | Single-user surface that does not cross a tenancy boundary (user-attached SSH target, local-only AI model). |
| `single_user_managed_tenant` | Managed surface scoped to a single user under a managed tenant. |
| `org_tenant` | Scoped to one organisation tenant. |
| `multi_tenant_isolated` | One tenant in a multi-tenant managed service with explicit isolation guarantees (label only at M1). |
| `multi_tenant_shared` | Surface shares state with other tenants intentionally. |
| `unknown_tenancy` | Explicit uncertainty token; surfaces that cannot truthfully name their tenancy MUST publish this rather than guess. |

### Key / storage mode class

| Token | Meaning |
| --- | --- |
| `not_applicable_local_only` | Surface neither holds keys nor stores user secrets. |
| `local_storage_only` | Key material / data lives in the local profile only. |
| `os_keychain_backed` | Local OS keychain or equivalent platform key store. |
| `byok_user_managed` | User supplies their own key material the surface treats as opaque. |
| `provider_managed_key` | User-attached provider manages the key. |
| `managed_service_kms` | Managed KMS is in scope (label only at M1). |
| `unknown_key_mode` | Explicit uncertainty token; surfaces that cannot truthfully name their key mode MUST publish this rather than guess. |

### Local-safe fallback class

| Token | Meaning |
| --- | --- |
| `local_safe_fallback_present` | Surface continues with no narrowing of its M1 baseline. |
| `local_safe_fallback_narrowed` | A strict local subset continues with documented narrowing; absence narrows the claim. |
| `local_safe_fallback_unavailable` | Surface depends on the remote/managed link to operate at all; must be optional and disclosed. |

### Data-residency disclosure class

| Token | Meaning |
| --- | --- |
| `residency_local_device_only` | Row never moves user data off the local device. |
| `residency_user_owned_remote_target` | User data may rest on a remote target the user named themselves. |
| `residency_provider_default` | The user-attached provider's default applies; the surface discloses, never enforces a region. |
| `residency_managed_tenant_documented_region` | The managed surface documents its region; the seed does NOT enforce residency in M1. |
| `residency_unknown` | Explicit uncertainty token; surfaces that cannot truthfully disclose residency MUST publish this rather than imply a region. |

### Truth-badge class (intended rendering surface)

| Token | Meaning |
| --- | --- |
| `client_scope_badge` | Status-bar/About client-scope badge. |
| `boundary_class_badge` | Status-bar/About boundary-class badge. |
| `residual_dependency_posture_badge` | Service-health posture row. |
| `freshness_badge` | Freshness label on cached/remote content. |
| `managed_disclosure_badge` | Prototype managed/control-plane-bearing disclosure badge. |

### Diagnostic-surface class (intended consumption surface)

| Token | Meaning |
| --- | --- |
| `help_about_pane` | Shell Help/About pane. |
| `service_health_row` | Service-health prototype row. |
| `support_export_section` | Support-bundle export section. |
| `release_evidence_pack` | Release-evidence pack inclusion. |
| `ci_validation` | Runtime / CI validation lane. |

## Seeded locality / tenancy / key-mode profile rows

The seed lands six typed rows covering the protected M1 surfaces and
the prototype managed/control-plane-bearing surfaces:

| `locality_tenancy_keymode_profile_id` | Surface family | Locality | Tenancy | Key mode | Residency | Local-safe fallback |
| --- | --- | --- | --- | --- | --- | --- |
| `local_only.editor_buffer_save` | `editor` | `local_only` | `not_applicable_local_only` | `not_applicable_local_only` | `residency_local_device_only` | `local_safe_fallback_present` |
| `remote_target.workspace_remote_agent_attach` | `workspace` | `remote_target` | `single_user_local` | `os_keychain_backed` | `residency_user_owned_remote_target` | `local_safe_fallback_narrowed` |
| `provider_linked.symbol_service_attach` | `symbol_service` | `provider_linked` | `single_user_local` | `byok_user_managed` | `residency_provider_default` | `local_safe_fallback_narrowed` |
| `managed_control_plane.companion_notification_channel_seed` | `companion_notification_channel` | `managed_control_plane_bearing` | `org_tenant` | `managed_service_kms` | `residency_managed_tenant_documented_region` | `local_safe_fallback_narrowed` |
| `managed_control_plane.hosted_control_plane_uncertainty` | `hosted_control_plane_prototype` | `managed_control_plane_bearing` | `unknown_tenancy` | `unknown_key_mode` | `residency_unknown` | `local_safe_fallback_narrowed` |
| `unknown_locality.prototype_surface_under_review` | `hosted_control_plane_prototype` | `unknown_locality` | `unknown_tenancy` | `unknown_key_mode` | `residency_unknown` | `local_safe_fallback_unavailable` |

## Failure drills

Every row carries a named failure drill the validation lane
reproduces under
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
uncertainty rather than false certainty* — is reproduced verbatim by
the `managed_control_plane.hosted_control_plane_uncertainty` row
and its
`locality_tenancy_keymode_drill.managed_control_plane_certifies_unknown_tenancy`
drill.

## How the vocabulary maps to badges, diagnostics, and support exports

Every row's `truth_badge_classes[]` and `diagnostic_surface_classes[]`
name the surfaces that consume the row's locality / tenancy /
key-mode tokens. Concretely in M1:

- **Help / About pane** renders `locality_class` as part of the
  boundary-class line, `tenancy_scope_class` and
  `key_storage_mode_class` next to the managed-disclosure badge on
  rows whose `truth_badge_classes` includes `managed_disclosure_badge`,
  and `data_residency_disclosure_class` as a residency-disclosure
  line. Unknown tokens render as "unknown — see service-health" and
  are never replaced with a placeholder region.
- **Service-health row** renders `local_safe_fallback_class` as the
  fallback posture (present / narrowed / unavailable) and
  `data_residency_disclosure_class` as the residency posture for
  managed and provider-linked rows.
- **Support-bundle exporter** quotes `locality_tenancy_keymode_profile_id`,
  `locality_class`, `tenancy_scope_class`, `key_storage_mode_class`,
  and `data_residency_disclosure_class` verbatim per surface family
  so a bundle pulled from a managed surface and a bundle pulled from
  a local-only surface read the same vocabulary.
- **Release-evidence pack** carries the seed's row union and the
  `required_locality_class_coverage` axes so reviewers can confirm
  the release labels the same vocabulary the product renders.
- **CI / validation lane** replays the seed and reproduces every
  named failure drill loudly so a regression in the seed's
  vocabulary is impossible to ship past CI.

## Consumer checklist

Before a managed or control-plane-bearing surface ships, it confirms:

1. It consumes one and only one `locality_class` from the closed
   vocabulary above. Surfaces MUST NOT invent their own locality
   names.
2. It publishes a `tenancy_scope_class` and a `key_storage_mode_class`
   drawn from the closed vocabularies. Surfaces that cannot
   truthfully name their tenancy or key mode publish the
   `unknown_tenancy` / `unknown_key_mode` token verbatim.
3. It publishes a `local_safe_fallback_class` even on managed rows;
   a managed row that omits the fallback class would silently make
   the surface non-optional in M1.
4. It publishes a `data_residency_disclosure_class` drawn from the
   closed vocabulary; a managed surface that publishes
   `residency_local_device_only` is a bug, and a local-only surface
   that publishes `residency_managed_tenant_documented_region` is a
   bug.
5. It reads the upstream internal boundary manifest's boundary class
   for the same surface family and asserts the seed's `locality_class`
   projects from it (local_only → local_only; provider_linked →
   provider_linked or remote_target; managed → managed_control_plane_bearing).

## Change management

- Adding a new locality, tenancy scope, key storage mode, local-safe
  fallback class, residency-disclosure class, truth badge, or
  diagnostic surface is additive-minor: bump the seed's
  `schema_version`, extend the row schema's `$defs`, extend the
  envelope vocabularies, and refresh the validation capture.
- Repurposing any existing value is breaking and requires a new
  decision row.
- Removing a row is breaking. Narrow by editing the row, not by
  deletion.
- Publishing a new release of the seed MUST refresh the validation
  capture; the refresh trigger and freshness rule are pinned in the
  proof packet.
