# Operating-mode card and region or key-state worked cases

These fixtures anchor the contract in
[`/docs/service/operating_mode_and_capacity_contract.md`](../../../docs/service/operating_mode_and_capacity_contract.md)
and validate against
[`/schemas/service/operating_mode.schema.json`](../../../schemas/service/operating_mode.schema.json)
and
[`/schemas/service/region_key_state.schema.json`](../../../schemas/service/region_key_state.schema.json).

Each fixture demonstrates that capacity, region, tenant, and key-state
truth surfaces without implying whole-product failure. Capacity or
key-state failures fail closed only for the managed action family
that cannot be bounded safely; local-safe workflows continue under the
fallback posture documented on the linked API surface row.

## Coverage matrix

| Case | Operating mode | Notable posture |
| --- | --- | --- |
| [`local_only_mode.yaml`](./local_only_mode.yaml) | `local_only` | No managed services in scope; every capacity row resolves through `quota_owner: not_applicable`; `local_core_unaffected`. |
| [`enterprise_saas_with_region_strip.yaml`](./enterprise_saas_with_region_strip.yaml) | `enterprise_saas` | Healthy capacity for every family; region strip pins `customer_region_pinned` and the tenant detail card pins `customer_tenant`. |
| [`self_hosted_no_vendor_fallback.yaml`](./self_hosted_no_vendor_fallback.yaml) | `self_hosted` | Customer-operated capacity for every family with an explicit "no vendor fallback" continuity note. |
| [`ai_quota_exhaustion.yaml`](./ai_quota_exhaustion.yaml) | `enterprise_saas` | AI gateway family at `quota_exhausted` for `ai_inference_action_family`; `fail_closed_managed_only` posture; quota throttle banner names the enterprise tenant as `quota_owner`; BYOK and local AI continuity preserved. |
| [`key_state_mismatch_one_family.yaml`](./key_state_mismatch_one_family.yaml) | `enterprise_saas` | `region_key_state_record` with `key_state_class: mismatch_recheck_required` bounded to `ai_evidence_retention_action_family` only; every other managed action family remains healthy. |

## Fixture rules

- Opaque refs stand in for tenants, regions, keys, recheck packets,
  and continuity packets.
- Exact UTC timestamps appear on every record.
- No fixture uses raw URLs, raw hostnames, raw cloud-region
  identifiers, raw tenant names, raw account ids, raw key bytes, raw
  certificate bodies, raw billing values, or raw secret material.
- `display_copy.whole_product_failure_implied`,
  `display_copy.generic_unavailable_copy_used`,
  `display_copy.silent_fail_open_under_unknown_state`,
  `display_copy.quota_owner_omitted` (operating-mode card), and
  `display_copy.all_managed_blocked_overclaimed` (region or key state)
  remain false on every record.
- Every fixture quotes the contract document, the locality and
  continuity seed, and the managed-service seed in `narrative_refs`.
