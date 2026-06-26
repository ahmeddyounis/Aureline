# Docs-Pack Manager Fixtures

Each fixture is a case file with a `record_kind` of `docs_pack_manager_case`, a
`scenario` describing what the case proves, a packet `input`, and an `expect`
block naming the derived promotion state and the finding kinds the validator must
raise. The integration test materializes each `input` and asserts the promotion
state and expected findings, so these fixtures pin the guardrails the canonical
support export keeps green.

## baseline_stable.json

The baseline packet certifies `stable`: docs-pack manager rows over the canonical
manifest carry signer, channel, mirror source, version range, refresh state, and
pin/offline posture across local-only, mirrored, managed, and air-gapped flows,
with pin/refresh/remove/mirror-source/offline actions and import/export continuity
preserved on every claimed manager profile.

## manager_row_hides_mirror_source_blocks_stable.json

A mirrored docs-pack row stops showing its mirror source. The validator raises
`manager_row_hides_manifest_truth` and blocks stable because every manager row must
keep signer/channel/mirror source visible.

## unavailable_payload_hidden_blocks_stable.json

A pack whose payload is unavailable locally stops disclosing the unavailable
payload state. The validator raises `unavailable_payload_hidden` and blocks stable
because the manager must not hide unavailable payload or signature state.

## mirror_offline_degraded_to_cache_blocks_stable.json

An air-gapped pack collapses into an opaque cache badge. The validator raises
`mirror_offline_degraded` and blocks stable because mirror and offline flows must
stay first-class and never degrade into opaque cache or browser-only fallback
wording.

## import_export_continuity_lost_blocks_stable.json

A docs pack stops preserving its identity on export. The validator raises
`import_export_continuity_lost` and blocks stable because import/export must retain
docs-pack identity and lifecycle state rather than flattening into generic cache
metadata.

## manager_action_reason_missing_blocks_stable.json

A disabled manager action drops its disclosed reason. The validator raises
`manager_action_reason_missing` and blocks stable because a disabled or
not-applicable action must always name why it is unavailable.

## lifecycle_flow_origin_mismatch_blocks_stable.json

An air-gapped pack claims a fresh-install import origin. The validator raises
`lifecycle_flow_origin_mismatch` and blocks stable because a pack's lifecycle flow
and its import provenance must stay consistent.

## profile_projection_drops_truth_blocks_stable.json

A manager-profile projection stops preserving import/export continuity. The
validator raises `profile_projection_drift` and blocks stable because every claimed
profile must reuse the manager packet without dropping truth.
