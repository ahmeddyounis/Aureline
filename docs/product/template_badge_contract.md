# Template gallery badge + disclosure contract

This document freezes the badge and disclosure posture for **template and
starter discovery surfaces** (template gallery cards, template detail
pages, and the preflight/health drill-downs they open) so that users can
always see:

- **source** (who/where it came from),
- **support posture** (what support promise is being made),
- **certification / claim posture** (what has been certified/reviewed),
- **compatibility posture** (what is compatible with this client/runtime),
- **why availability is narrowed** (policy, mirror/offline, missing
  connectivity, incompatible runtime/toolchain, stale/expired evidence).

Where this document disagrees with upstream contracts, the upstream wins
and this document MUST update in the same change. This contract does not
mint new vocabularies; it binds template surfaces to existing ones.

## Companion sources (authoritative)

- Template gallery disclosure contract:
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
- Template-source-class + narrowing matrix:
  [`/artifacts/ux/template_source_class_matrix.yaml`](../../artifacts/ux/template_source_class_matrix.yaml)
- Template registry row contract:
  [`/docs/templates/template_registry_and_scaffold_contract.md`](../templates/template_registry_and_scaffold_contract.md)
- Template registry row schema:
  [`/schemas/templates/template_registry_entry.schema.json`](../../schemas/templates/template_registry_entry.schema.json)
- Scaffold card/preflight/health contract:
  [`/docs/scaffolding/template_health_and_preflight_contract.md`](../scaffolding/template_health_and_preflight_contract.md)
- Template-health-state matrix:
  [`/artifacts/scaffolding/template_health_states.yaml`](../../artifacts/scaffolding/template_health_states.yaml)
- Badge-family and evidence-aging rules:
  [`/docs/ux/capability_lifecycle_badge_contract.md`](../ux/capability_lifecycle_badge_contract.md)

## Non-negotiable disclosure rules

1. **Source and support are mandatory.** A template row MUST expose
   `template_source_class` and `support_class` (template/prebuild contract
   §3.2–§3.3). A surface MUST NOT replace either with marketing-only
   labels like “Recommended” or “Featured”.
2. **Certification is never implied by placement.** Any “certified”,
   “supported”, or “approved” wording MUST be backed by
   `template_certification_class` (template registry contract §2.3) plus
   the required evidence references for that class. Placement in a
   gallery, mirror, or repository MUST NOT upgrade the claim.
3. **Compatibility is never implied by popularity.** Any compatibility
   cue MUST be backed by:
   - the registry row’s declared envelope (`supported_platform_class_set`,
     `supported_ecosystem_class_set`, `compatible_runtime_range`); and
   - the health/compatibility checks the surface is actually able to run
     (`template_health_row_record` with check classes like
     `toolchain_compatibility` / `os_runtime_compatibility`).
4. **Narrowing is never silent.** When availability is narrowed, the
   surface MUST render a typed `starter_policy_notice_record` in-place
   (template/prebuild contract §10) and MUST name:
   `availability_narrowing_class` plus the typed policy/mirror basis refs
   (not free-form “blocked” strings).
5. **Evidence staleness narrows claims.** If the evidence freshness floor
   for a claim-bearing badge is unmet, the badge group MUST visibly
   narrow per the lifecycle-badge contract (no “Certified” badge that
   survives stale validation evidence).

## Binding template cards to registry + health truth

Template discovery surfaces project from three sources of truth:

1. **Registry row** (`template_registry_entry_record`):
   source + certification claim posture + support posture + declared
   compatibility envelope + freshness/mirror posture.
2. **Scaffold card / preflight / health** (`template_card_record`,
   `generation_preflight_record`, `template_health_state_class`):
   what the user is about to do (side effects) and whether the current
   surface can validate it (fresh vs stale vs failed).
3. **Policy narrowing notice** (`starter_policy_notice_record`):
   why the set is narrowed right now, and what the user can do next.

No template card may present a stronger claim than the strongest claim
supportable by the intersection of (1) + (2) under the narrowing rules
in (3).

## Policy-narrowing disclosure audit (must-have cases)

The fixture corpus for the template/prebuild disclosure contract MUST
include worked examples covering, at minimum:

- organization/admin policy narrowing (`policy_narrowed_admin`);
- missing connectivity (`offline_no_bundle`) and missing required runtime (`target_runtime_unavailable`);
- mirror-only/offline narrowing (`mirror_only_cached_subset`);
- incompatible OS/toolchain surfaced via health checks
  (`toolchain_compatibility` / `os_runtime_compatibility`);
- stale/expired validation evidence narrowing a claim-bearing badge
  (health signal `cached` with an age class, or `stale_or_invalid`).

The canonical seed fixtures live under:
[`/fixtures/ux/template_and_prebuild_states/`](../../fixtures/ux/template_and_prebuild_states/).
