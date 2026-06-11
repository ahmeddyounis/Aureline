# M5 embedded-boundary owner/origin, auth, and handoff qualification audit

Generated from the seeded audit in
[`crate::m5_embedded_boundaries`](../../../../crates/aureline-shell/src/m5_embedded_boundaries/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- report-md > \
  artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md
```

- Report id: `shell:m5_embedded_boundaries:audit:v1`
- Source schema ref: `schemas/help/m5-destination-descriptor-diff.schema.json`
- Registered M5 surfaces: `8`
- High-stakes surfaces: `4`
- Marketed surfaces: `8`
- Boundary guarantees checked: `64`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-11T00:00:00Z`

## Per-guarantee coverage

| Boundary guarantee | Qualified | Narrowed | Unqualified | Missing evidence |
| ------------------ | --------: | -------: | ----------: | ---------------: |
| Owner/origin disclosure | 8 | 0 | 0 | 0 |
| Freshness disclosure | 7 | 1 | 0 | 0 |
| Trust-boundary chrome | 8 | 0 | 0 | 0 |
| System-browser auth default | 4 | 4 | 0 | 0 |
| No embedded high-risk approval | 4 | 4 | 0 | 0 |
| Return anchor present | 8 | 0 | 0 | 0 |
| Handoff reason preserved | 8 | 0 | 0 | 0 |
| Support/export parity | 8 | 0 | 0 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unqualified_local_surface` | 0 |
| `missing_evidence` | 0 |
| `missing_descriptor_ref` | 0 |
| `owner_origin_hidden` | 0 |
| `freshness_hidden` | 0 |
| `pretends_first_party` | 0 |
| `embedded_primary_auth` | 0 |
| `embedded_high_risk_approval` | 0 |
| `return_anchor_lost` | 0 |
| `handoff_reason_dropped` | 0 |
| `support_parity_divergent` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `aspect_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_return_anchor` | 0 |
| `missing_support_note` | 0 |
| `surface_not_on_governed_boundary` | 0 |
| `missing_boundary_chrome` | 0 |
| `no_declared_handoff_target` | 0 |

## Return anchor index

| Embedded surface | Surface id | Return anchor |
| ---------------- | ---------- | ------------- |
| Companion/browser handoff | `embedded:companion_browser_handoff` | `embedded:return:companion_browser_handoff` |
| Embedded docs viewer | `embedded:embedded_docs` | `embedded:return:embedded_docs` |
| Help-center pane | `embedded:help_center_pane` | `embedded:return:help_center_pane` |
| Marketplace/account surface | `embedded:marketplace_account` | `embedded:return:marketplace_account` |
| Preview-route pane | `embedded:preview_route_pane` | `embedded:return:preview_route_pane` |
| Provider-console handoff | `embedded:provider_console_handoff` | `embedded:return:provider_console_handoff` |
| Provider/review pane | `embedded:provider_review_pane` | `embedded:return:provider_review_pane` |
| Request/runtime viewer | `embedded:request_runtime_viewer` | `embedded:return:request_runtime_viewer` |

## Per-surface rows

### `embedded:companion_browser_handoff` (companion_browser_handoff, external_handoff, beta)

- Descriptor revision: `embedded-rev:companion_browser_handoff:2026.06.01-01`
- Boundary class: `external_handoff`
- Return anchor: `embedded:return:companion_browser_handoff`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| System-browser auth default | `qualified` | `owner_origin_disclosed` | `-` | `-` | `system_browser_default` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| No embedded high-risk approval | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `blocked_or_routed` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:embedded_docs` (embedded_docs, first_party_local, beta)

- Descriptor revision: `embedded-rev:embedded_docs:2026.06.01-01`
- Boundary class: `first_party_local`
- Return anchor: `embedded:return:embedded_docs`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `return_anchor_control`
- Handoff targets: `in_product_return`, `system_browser`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| System-browser auth default | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_renders_content_only_and_never_authenticates_so_there_is_no_auth_channel_to_default |
| No embedded high-risk approval | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_exposes_no_mutating_approval_so_there_is_no_high_risk_embedded_channel_to_block |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:help_center_pane` (help_center_pane, first_party_local, beta)

- Descriptor revision: `embedded-rev:help_center_pane:2026.06.01-01`
- Boundary class: `first_party_local`
- Return anchor: `embedded:return:help_center_pane`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `return_anchor_control`
- Handoff targets: `in_product_return`, `system_browser`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| System-browser auth default | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_renders_content_only_and_never_authenticates_so_there_is_no_auth_channel_to_default |
| No embedded high-risk approval | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_exposes_no_mutating_approval_so_there_is_no_high_risk_embedded_channel_to_block |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:marketplace_account` (marketplace_account, provider_owned, beta)

- Descriptor revision: `embedded-rev:marketplace_account:2026.06.01-01`
- Boundary class: `provider_owned`
- Return anchor: `embedded:return:marketplace_account`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`, `vendor_portal`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| System-browser auth default | `qualified` | `owner_origin_disclosed` | `-` | `-` | `system_browser_default` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| No embedded high-risk approval | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `blocked_or_routed` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:preview_route_pane` (preview_route_pane, embedded_webview, beta)

- Descriptor revision: `embedded-rev:preview_route_pane:2026.06.01-01`
- Boundary class: `embedded_webview`
- Return anchor: `embedded:return:preview_route_pane`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| System-browser auth default | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_renders_content_only_and_never_authenticates_so_there_is_no_auth_channel_to_default |
| No embedded high-risk approval | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_exposes_no_mutating_approval_so_there_is_no_high_risk_embedded_channel_to_block |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:provider_console_handoff` (provider_console_handoff, external_handoff, beta)

- Descriptor revision: `embedded-rev:provider_console_handoff:2026.06.01-01`
- Boundary class: `external_handoff`
- Return anchor: `embedded:return:provider_console_handoff`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`, `provider_console`, `vendor_portal`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `declared_capture_gap` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | the_provider_console_is_external_so_its_freshness_is_declared_at_handoff_not_continuously_polled |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| System-browser auth default | `qualified` | `owner_origin_disclosed` | `-` | `-` | `system_browser_default` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| No embedded high-risk approval | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `blocked_or_routed` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:provider_review_pane` (provider_review_pane, provider_owned, beta)

- Descriptor revision: `embedded-rev:provider_review_pane:2026.06.01-01`
- Boundary class: `provider_owned`
- Return anchor: `embedded:return:provider_review_pane`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`, `vendor_portal`
- Marketed on desktop: `yes`
- High-stakes: `yes`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| System-browser auth default | `qualified` | `owner_origin_disclosed` | `-` | `-` | `system_browser_default` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| No embedded high-risk approval | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `blocked_or_routed` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

### `embedded:request_runtime_viewer` (request_runtime_viewer, embedded_webview, beta)

- Descriptor revision: `embedded-rev:request_runtime_viewer:2026.06.01-01`
- Boundary class: `embedded_webview`
- Return anchor: `embedded:return:request_runtime_viewer`
- Boundary chrome: `owner_badge`, `origin_label`, `freshness_stamp`, `trust_boundary_frame`, `return_anchor_control`, `handoff_reason_banner`
- Handoff targets: `in_product_return`, `system_browser`
- Marketed on desktop: `yes`
- High-stakes: `no`

| Boundary guarantee | Status | Owner/origin | Freshness | Trust | Auth | High-risk | Return | Handoff | Support | Freshness ev. | Narrowing reason |
| ------------------ | ------ | ------------ | --------- | ----- | ---- | --------- | ------ | ------- | ------- | ------------- | ---------------- |
| Owner/origin disclosure | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Freshness disclosure | `qualified` | `owner_origin_disclosed` | `freshness_shown` | `-` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| Trust-boundary chrome | `qualified` | `owner_origin_disclosed` | `-` | `bounded_attributed` | `-` | `-` | `-` | `-` | `-` | `fresh` | - |
| System-browser auth default | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_renders_content_only_and_never_authenticates_so_there_is_no_auth_channel_to_default |
| No embedded high-risk approval | `not_applicable` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | `-` | this_surface_exposes_no_mutating_approval_so_there_is_no_high_risk_embedded_channel_to_block |
| Return anchor present | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `exact_return_resolved` | `-` | `-` | `fresh` | - |
| Handoff reason preserved | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `reason_preserved` | `-` | `fresh` | - |
| Support/export parity | `qualified` | `owner_origin_disclosed` | `-` | `-` | `-` | `-` | `-` | `-` | `same_descriptor_reused` | `fresh` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_embedded_boundaries -- validate
cargo test -p aureline-shell --test m5_embedded_boundaries_fixtures
python3 tools/ci/m5/embedded_boundaries_check.py
```
