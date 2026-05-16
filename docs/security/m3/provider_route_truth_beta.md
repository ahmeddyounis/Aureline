# Provider route, browser handoff, and authority-truth panel beta

This document is the reviewer-facing landing page for the beta projection
that keeps external and browser-linked provider lanes honest. It builds on
the connected-provider registry alpha frozen in
[`/crates/aureline-provider/src/registry/mod.rs`](../../../crates/aureline-provider/src/registry/mod.rs),
the account-scope beta frozen in
[`/crates/aureline-provider/src/account_scope/mod.rs`](../../../crates/aureline-provider/src/account_scope/mod.rs),
and the route-origin / command-reconstruction alpha frozen in
[`/crates/aureline-support/src/route_origin_alpha/mod.rs`](../../../crates/aureline-support/src/route_origin_alpha/mod.rs).

The contract is owned by
[`/crates/aureline-provider/src/route_resolution/mod.rs`](../../../crates/aureline-provider/src/route_resolution/mod.rs).
The headless inspector lives at
[`/crates/aureline-provider/src/bin/aureline_provider_route_resolution_beta.rs`](../../../crates/aureline-provider/src/bin/aureline_provider_route_resolution_beta.rs).
The source matrix lives at
[`/artifacts/security/m3/route_resolution_panels/route_resolution_matrix.yaml`](../../../artifacts/security/m3/route_resolution_panels/route_resolution_matrix.yaml).

## What the projection covers

Every claimed beta page is one
`providers_route_resolution_beta_page_record` carrying three record kinds:

- **`providers_route_resolution_beta_row_record`** — one row per
  provider-linked beta action. Every row names:
  - **Profile** — one of `connected`, `mirror_only`, `offline`,
    `enterprise_managed`. All four profiles must have at least one
    claimed row.
  - **Lane class** — `managed_provider_lane`, `managed_mirror_lane`,
    `external_provider_lane`, `tunnel_exposed_external_lane`, or
    `offline_snapshot_lane`. Managed lanes (the first two) must cite a
    managed-policy bundle ref on the owner or grant; external lanes
    must not.
  - **Provider source class** and **provider surface class** — typed
    refinements of the connected-provider registry alpha vocabulary.
  - **Action class** — `read_only_inspection`, `provider_mutation`,
    `ci_or_check_mutation`, `release_publish`, or
    `credential_projection`.
  - **Route** — typed `route_choice` (`live_provider_direct`,
    `signed_mirror_route`, `tunnel_exposed_route`,
    `imported_snapshot_route`, `system_browser_handoff_route`, or
    `local_only_route`), a redaction-safe route label, transport label
    (`https`, `signed_mirror_https`, `tunnel_https`,
    `imported_snapshot_read`, `system_browser`, etc.), opaque target
    identity ref, and the optional `tunnel_session_ref`,
    `mirror_identity_ref`, or `snapshot_publisher_ref` the route
    requires.
  - **Owner** — typed `owner_class` (`workspace_user`,
    `provider_authority`, `managed_policy_authority`,
    `tunnel_session_owner`, or `offline_import_authority`), opaque
    owner ref, and a redaction-safe label.
  - **Grant** — typed `acting_identity_class` (`connected_account`,
    `installation_grant`, `delegated_credential`), `auth_source`
    (reused from the connected-provider auth-source vocabulary), and
    the bound account-scope beta identity row ref.
  - **Fallback** — typed `fallback_mode` (`copy_or_export`,
    `open_in_provider`, `publish_later_queue`, or `inspect_only`) plus
    the structured proof ref the fallback requires: a
    `browser_handoff_packet_ref` for `open_in_provider`, a
    `publish_later_queue_item_ref` for `publish_later_queue`, an
    `inspect_only_snapshot_ref` for `inspect_only`, or a
    `copy_or_export_evidence_ref` for `copy_or_export`.
  - **Freshness** — typed `freshness_class` reused from the
    connected-provider registry alpha (`fresh`, `stale_within_window`,
    `expired_beyond_window`, `never_observed`, or
    `revoked_or_disconnected`), freshness-floor ref, observation
    timestamp, optional stale-after horizon, and redaction-safe
    degraded reason.
  - **Route degraded state** — typed
    `route_degraded_state` (`green`, `freshness_floor_drifted`,
    `mirror_lag_beyond_tolerance`, `tunnel_session_expired`,
    `snapshot_older_than_retention_floor`,
    `managed_policy_boundary_closed`, or `route_unreachable`).
  - **Command-route packet ref** — opaque ref to the route-origin /
    command-reconstruction packet so browser-handoff, command, and
    support reconstruction share the same origin lineage.
  - **Hard guardrails** — every row asserts that raw token material is
    not present, no silent public-endpoint fallback was taken, no
    silent authority widening was taken, and local editing is
    preserved.
- **`providers_route_resolution_beta_browser_handoff_panel_record`** —
  one panel per browser-routed action. Each panel pins:
  - The bound row ref.
  - The typed `handoff_reason` (`mutation_not_supported_in_product`,
    `publish_requires_browser_auth`, `license_or_portal_acceptance`,
    `admin_only_surface`, `provider_consent_flow`,
    `provider_admin_delegation`, or `step_up_required`).
  - The `projected_route_choice`, `projected_owner_class`, and
    `projected_acting_identity_class` the handoff acts under. The
    validator requires these to match the bound row's route or its
    `open_in_provider` fallback, the bound row's owner class, and the
    bound row's grant acting-identity class.
  - The opaque `browser_handoff_packet_ref` (and optional
    `return_summary_ref`). Raw URLs, raw callback bodies, and raw
    provider payloads never appear on the panel.
- **`providers_route_resolution_beta_authority_truth_panel_record`** —
  one panel per provider-linked row that names whether the green claim
  is still honest:
  - `green_claim_honest` — route is fresh, route-degraded state is
    `green`, grant lifecycle admits mutation, and managed lanes cite a
    managed-policy bundle ref.
  - `claim_degraded` — visibly degraded state; mutation authority is
    held closed.
  - `claim_stale_and_retracted` — claim is retracted; the row routes
    through the typed fallback.
  - `never_resolved` — route or grant truth never observed.
  The validator forbids `green_claim_held = true` while the bound row's
  route or freshness state holds mutation closed, while the
  `green_claim_held` flag disagrees with the typed `truth_state`, or
  while a managed lane's green claim has no managed-policy bundle ref.

## Acceptance posture

- **Current route, owner, grant type, and fallback are named on every
  claimed row.** The row record carries typed `route`, `owner`, `grant`,
  and `fallback` descriptors plus a redaction-safe display label;
  support and product surfaces can render all four facts from the same
  record without scraping UI text.
- **Browser handoff is traceable.** Every browser-handoff panel binds
  to a row, projects the same route choice (or its
  `open_in_provider` fallback), owner class, and acting-identity
  class, and cites the same browser-handoff packet ref the command and
  support packets use. Defects `browser_handoff_panel_*` block stale
  panels.
- **No green claim survives stale route or authority truth.** The
  validator fails closed on `authority_truth_panel_green_claim_while_stale`,
  `authority_truth_panel_green_claim_without_managed_bundle`,
  `authority_truth_panel_green_flag_disagrees_with_state`, and
  `route_green_state_with_stale_freshness`.
- **Fail-closed invariants.** Raw token material, silent
  public-endpoint fallback, silent authority widening, broken local
  editing, raw URLs in browser-handoff panels, raw provider payloads,
  missing tunnel / mirror / snapshot refs on the matching route, and
  mismatched managed-policy bundle bindings each surface as typed
  defects.

## Fail-closed invariants

- **No raw token, URL, or provider payload.** Every record carries
  opaque refs and structured fields only. Raw access tokens, raw
  refresh tokens, raw delegated-token bodies, raw callback bodies, raw
  URLs, and raw provider payloads never cross the projection boundary
  — they are excluded because the projection never carries them.
- **No silent fallback.** Rows asserting
  `silent_public_endpoint_fallback_taken = true` are rejected; the
  fallback path is always a typed `fallback_mode` with a structured
  proof ref.
- **No silent widening.** Rows asserting
  `silent_authority_widening_taken = true` are rejected; mutation
  authority must be backed by the grant's scope refs and the
  account-scope beta resolution lineage.
- **No green claim while stale.** Authority-truth panels cannot retain
  green when the bound row's route, freshness, or managed bundle truth
  no longer admits it.
- **Local editing is preserved.** Every row and every fallback path
  preserves local editing; rows that break local editing surface as
  `local_editing_not_preserved` defects.

## Headless inspector

```sh
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- page
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- validate
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- support-export
```

`validate` prints the typed defect list; the seeded beta page emits an empty
list and exits with status `0`. `support-export` emits the redaction-safe
[`baseline_support_export.json`](../../../artifacts/security/m3/route_resolution_panels/baseline_support_export.json)
artifact that bundles route, owner, grant, fallback, browser-handoff, and
authority-truth lineage so reviewers can name the route everywhere.
