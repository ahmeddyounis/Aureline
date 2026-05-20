# Embedded boundary audit corpus report

Generated from the seeded corpus in `crates/aureline-shell/src/embedded_boundary_corpus/mod.rs`.
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md > \
  artifacts/ux/m3/embedded_boundary_audit_report.md
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md > \
  docs/ux/m3/embedded_boundary_audit_beta.md
```

- Packet id: `shell:embedded_boundary_corpus:packet:default`
- Shared contract ref: `shell:embedded_boundary_corpus:v1`
- Boundary vocabulary schema: `schemas/ux/embedded_surface_boundary.schema.json`
- Generated at: `2026-05-20T00:00:00Z`
- Cases: 17 (9 conformant, 8 denial)
- All recorded verdicts hold: yes

## Surface family matrix

| Surface | Boundary drills | Denials | Boundary states | Conformant | Denial |
| ------- | --------------- | ------- | --------------- | ---------- | ------ |
| `embedded_docs_help` | `owner_origin_verified`, `open_in_browser_fallback` | `owner_origin_spoof`, `native_trust_chrome_spoof` | `live_verified` | 1 | 2 |
| `extension_hosted_surface` | `cross_origin_limitation`, `open_in_browser_fallback` | (none) | `cross_origin_limited` | 1 | 0 |
| `embedded_marketplace_or_account` | `stale_snapshot`, `offline_snapshot`, `open_in_browser_fallback` | `authority_widening`, `support_export_flattening`, `browser_fallback_drops_target_or_reason` | `offline_snapshot`, `stale_snapshot` | 2 | 3 |
| `embedded_service_dashboard` | `certificate_failure`, `managed_policy_deny`, `open_in_browser_fallback` | `stale_masked_as_live`, `approval_bypass` | `certificate_failed`, `live_verified`, `policy_blocked` | 2 | 2 |
| `embedded_auth_confirmation` | `owner_origin_verified`, `system_browser_first_auth`, `device_code_fallback`, `native_approval_fence_persists_restart`, `native_approval_fence_persists_reentry` | `embedded_password_collection` | `live_verified` | 3 | 1 |

## Boundary drill coverage

- `owner_origin_verified` -- 2
- `system_browser_first_auth` -- 1
- `certificate_failure` -- 1
- `managed_policy_deny` -- 1
- `cross_origin_limitation` -- 1
- `stale_snapshot` -- 1
- `offline_snapshot` -- 1
- `open_in_browser_fallback` -- 4
- `device_code_fallback` -- 1
- `native_approval_fence_persists_restart` -- 1
- `native_approval_fence_persists_reentry` -- 1

## Denial drill coverage

- `owner_origin_spoof` -- 1
- `stale_masked_as_live` -- 1
- `native_trust_chrome_spoof` -- 1
- `approval_bypass` -- 1
- `authority_widening` -- 1
- `embedded_password_collection` -- 1
- `support_export_flattening` -- 1
- `browser_fallback_drops_target_or_reason` -- 1

## Cases

### `embedded_boundary_corpus:docs_help_live_verified` -- Docs/help pane — live verified, host-owned open-in-browser

- Surface: `embedded_docs_help`
- Kind: `boundary_drill`
- Owner: Aureline project docs (`first_party_project`)
- Origin: `aureline://docs/local-signed` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_full_authority`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_full_authority` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Local signed docs pack. Owner, origin, and freshness are disclosed; open-in-browser keeps the topic and grants no embedded authority.

### `embedded_boundary_corpus:auth_system_browser_first` -- Auth handoff — system browser preferred

- Surface: `embedded_auth_confirmation`
- Kind: `boundary_drill`
- Owner: Aureline auth handoff (`host_product`)
- Origin: `github.com` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_with_native_step_up_required`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_with_native_step_up` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Sign-in opens in the system browser and returns a packet; high-risk approvals still require a host-native step-up after sign-in.

### `embedded_boundary_corpus:marketplace_stale_snapshot` -- Marketplace/account — stale provider snapshot

- Surface: `embedded_marketplace_or_account`
- Kind: `boundary_drill`
- Owner: Marketplace account (`host_product`)
- Origin: `marketplace.aureline.dev` (verified)
- Boundary state: `stale_snapshot`
- Permission: `host_owned_inspect_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_inspect_only` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Account page shows a stale in-product snapshot; it is inspect-only until the provider session is renewed in the browser.

### `embedded_boundary_corpus:extension_cross_origin_limited` -- Extension webview — cross-origin limited

- Surface: `extension_hosted_surface`
- Kind: `boundary_drill`
- Owner: Acme Cloud extension panel (`extension_bundle`)
- Origin: `status.acme.example` (cross_origin_limited)
- Boundary state: `cross_origin_limited`
- Permission: `host_owned_inspect_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_inspect_only` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

The extension panel renders a cross-origin page Aureline cannot read; the limitation is named and open-in-browser preserves the page identity.

### `embedded_boundary_corpus:service_dashboard_policy_blocked` -- Service dashboard — managed policy deny

- Surface: `embedded_service_dashboard`
- Kind: `boundary_drill`
- Owner: Payments dashboard (`customer_service_owner`)
- Origin: `console.acme.example` (policy_blocked)
- Boundary state: `policy_blocked`
- Permission: `host_owned_browser_only`
- Fallback posture: `external_open_blocked_by_policy` -> `host_native_review_or_approval`
- Authority: native `host_owned_browser_only` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Managed-workspace policy disables the embedded render; the card names the policy and routes recovery into the host-native review surface.

### `embedded_boundary_corpus:service_dashboard_certificate_failed` -- Service dashboard — certificate verification failed

- Surface: `embedded_service_dashboard`
- Kind: `boundary_drill`
- Owner: Payments dashboard (`customer_service_owner`)
- Origin: `console.acme.example` (certificate_failed)
- Boundary state: `certificate_failed`
- Permission: `host_owned_browser_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_browser_only` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Certificate verification failed; the host refuses to render the body, names the failure, and offers certificate inspection plus a system-browser handoff.

### `embedded_boundary_corpus:marketplace_offline_snapshot` -- Marketplace/account — offline snapshot, external open unavailable

- Surface: `embedded_marketplace_or_account`
- Kind: `boundary_drill`
- Owner: Marketplace account (`host_product`)
- Origin: `marketplace.aureline.dev` (offline_cached)
- Boundary state: `offline_snapshot`
- Permission: `host_owned_inspect_only`
- Fallback posture: `external_open_unavailable_offline` -> `local_inspect_or_export`
- Authority: native `host_owned_inspect_only` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

Offline cached account snapshot. The fallback posture is honest: external open is unavailable until reconnect, and the surface stays inspect-only.

### `embedded_boundary_corpus:auth_device_code_fallback` -- Auth handoff — device-code fallback

- Surface: `embedded_auth_confirmation`
- Kind: `boundary_drill`
- Owner: Aureline auth handoff (`host_product`)
- Origin: `github.com` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_with_native_step_up_required`
- Fallback posture: `device_code_fallback_offered` -> `device_code_companion_card`
- Authority: native `host_owned_with_native_step_up` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

When the browser cannot return, device-code is the auditable fallback. The card copies a code only and never collects a password.

### `embedded_boundary_corpus:approval_fence_persists` -- Auth handoff — approval fence persists across restart and re-entry

- Surface: `embedded_auth_confirmation`
- Kind: `boundary_drill`
- Owner: Aureline auth handoff (`host_product`)
- Origin: `github.com` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_with_native_step_up_required`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_with_native_step_up` / fallback `no_authority_within_product`
- Expectation: `conformant`
- Gate produced: (none)
- Verdict holds: yes

The six native-reserved approval surfaces and the host-native step-up survive an app restart and a surface re-entry; the embedded body never inherits them.

### `embedded_boundary_corpus:denial_owner_origin_spoof` -- Denial — owner/origin disclosure dropped

- Surface: `embedded_docs_help`
- Kind: `denial_drill`
- Owner: (dropped) (`first_party_project`)
- Origin: `aureline://docs/local-signed` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_full_authority`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_full_authority` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `missing_owner_label`
- Gate produced: `missing_owner_label`
- Verdict holds: yes

Dropping the owner label must be denied — the host shell owns owner/origin disclosure and an embedded body cannot suppress it.

### `embedded_boundary_corpus:denial_stale_masked_as_live` -- Denial — failed origin painted as live verified

- Surface: `embedded_service_dashboard`
- Kind: `denial_drill`
- Owner: Payments dashboard (`customer_service_owner`)
- Origin: `console.acme.example` (certificate_failed)
- Boundary state: `live_verified`
- Permission: `host_owned_browser_only`
- Fallback posture: `external_open_blocked_by_policy` -> `host_native_review_or_approval`
- Authority: native `host_owned_browser_only` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `boundary_state_inconsistent_with_origin_verification`
- Gate produced: `boundary_state_inconsistent_with_origin_verification`
- Verdict holds: yes

A boundary state of live_verified over a certificate_failed origin must be denied — the boundary state may not contradict the origin verification.

### `embedded_boundary_corpus:denial_native_trust_chrome_spoof` -- Denial — embedded body impersonates native trust/update chrome

- Surface: `embedded_docs_help`
- Kind: `denial_drill`
- Owner: Aureline project docs (`first_party_project`)
- Origin: `aureline://docs/local-signed` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_full_authority`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_full_authority` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `embedded_minted_native_reserved_surface`
- Gate produced: `embedded_minted_native_reserved_surface`
- Verdict holds: yes

Update verification and product security messaging are native-reserved; an embedded body that drops them from the host-owned set must be denied.

### `embedded_boundary_corpus:denial_approval_bypass` -- Denial — embedded body hosts a high-risk approval

- Surface: `embedded_service_dashboard`
- Kind: `denial_drill`
- Owner: Payments dashboard (`customer_service_owner`)
- Origin: `console.acme.example` (policy_blocked)
- Boundary state: `policy_blocked`
- Permission: `host_owned_browser_only`
- Fallback posture: `external_open_blocked_by_policy` -> `host_native_review_or_approval`
- Authority: native `host_owned_browser_only` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `embedded_minted_native_reserved_surface`
- Gate produced: `embedded_minted_native_reserved_surface`
- Verdict holds: yes

The high-risk approval sheet and AI apply review are native-reserved; an embedded body that tries to host them bypasses preview/approval and must be denied.

### `embedded_boundary_corpus:denial_authority_widening` -- Denial — fallback widens authority past the native command

- Surface: `embedded_marketplace_or_account`
- Kind: `denial_drill`
- Owner: Marketplace account (`host_product`)
- Origin: `marketplace.aureline.dev` (verified)
- Boundary state: `stale_snapshot`
- Permission: `host_owned_inspect_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_inspect_only` / fallback `host_owned_full_authority`
- Expectation: `denied`
- Expected denial reasons: `fallback_widens_authority_beyond_native`
- Gate produced: `fallback_widens_authority_beyond_native`
- Verdict holds: yes

A reopen path that confers full in-product authority where the product-owned command is inspect-only widens authority and must be denied.

### `embedded_boundary_corpus:denial_embedded_password_collection` -- Denial — embedded password without a registered exception

- Surface: `embedded_auth_confirmation`
- Kind: `denial_drill`
- Owner: Aureline auth handoff (`host_product`)
- Origin: `github.com` (verified)
- Boundary state: `live_verified`
- Permission: `host_owned_with_native_step_up_required`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_with_native_step_up` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `embedded_auth_exception_missing_exception_ref`
- Gate produced: `embedded_auth_exception_missing_exception_ref`
- Verdict holds: yes

An embedded password flow with no exception_id_ref must be denied — embedded credential collection is only ever a registered, lower-trust exception.

### `embedded_boundary_corpus:denial_support_export_flattening` -- Denial — support export flattens the boundary state

- Surface: `embedded_marketplace_or_account`
- Kind: `denial_drill`
- Owner: Marketplace account (`host_product`)
- Origin: `marketplace.aureline.dev` (verified)
- Boundary state: `stale_snapshot`
- Permission: `host_owned_inspect_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_inspect_only` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `support_row_vocabulary_drift`
- Gate produced: `support_row_vocabulary_drift`
- Verdict holds: yes

A support export that paints a stale row as live verified flattens the boundary truth and must be denied — support rows mirror the live row exactly.

### `embedded_boundary_corpus:denial_browser_fallback_drops_target` -- Denial — open-in-browser drops the target and reason

- Surface: `embedded_marketplace_or_account`
- Kind: `denial_drill`
- Owner: Marketplace account (`host_product`)
- Origin: `marketplace.aureline.dev` (verified)
- Boundary state: `stale_snapshot`
- Permission: `host_owned_inspect_only`
- Fallback posture: `system_browser_first` -> `system_browser_handoff_packet`
- Authority: native `host_owned_inspect_only` / fallback `no_authority_within_product`
- Expectation: `denied`
- Expected denial reasons: `open_in_browser_drops_target_or_reason`
- Gate produced: `open_in_browser_drops_target_or_reason`
- Verdict holds: yes

An open-in-browser fallback that drops the return target and the reason flattens the handoff into a generic jump and must be denied.

