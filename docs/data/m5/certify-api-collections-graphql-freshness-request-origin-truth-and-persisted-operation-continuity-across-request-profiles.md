# Certify API collections, GraphQL freshness, request-origin truth, and persisted-operation continuity across request profiles

## Scope

This document describes the canonical certification qualification packet that
certifies API collections, contract/GraphQL freshness, request-origin truth, and
persisted-operation continuity across every claimed request profile: desktop-local,
CLI/headless, remote, container, managed-workspace, browser-companion, and
mirror/offline. The certification is evidence-bound: each case binds a profile, a
certified dimension, and a drill corpus to an outcome that certifies, narrows, or
blocks.

## Truth sources

- Implementation: `crates/aureline-api/src/certify_api_collections_graphql_freshness_request_origin_truth_and_persisted_operation_continuity_across_request_profiles/mod.rs`
- Schema: `schemas/data/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.schema.json`
- Checked-in packet: `artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json`
- Fixtures: `fixtures/data/m5/certify_api_collections_graphql_freshness_request_origin_truth_and_persisted_operation_continuity_across_request_profiles/`
- Help: `docs/help/api-request-profile-certification.md`

## Certified request profiles

| Profile | Class | Claim | Displayed | Trust isolated | Rationale |
|---|---|---|---|---|---|
| Desktop-local send | desktop_local | stable | stable | no | Text-first collections and metadata-only history retention with export-safe redaction. |
| CLI and headless send | cli | stable | stable | no | Same collections and contract-freshness truth as the desktop workspace, including stale-schema labeling. |
| Remote send | remote | stable | stable | yes | Explicit request origin and persisted-operation continuity; remote naming is not desktop-local. |
| Container service send | container | stable | stable | yes | Service-name origin resolution with drift review. |
| Managed-workspace send | managed | stable | stable | yes | Origin truth, contract freshness, retention, and auth-source labeling; managed origin isolates desktop-local trust. |
| Browser-companion send | browser_companion | stable | stable | yes | Origin truth, persisted-operation continuity, and auth-source labeling; companion origin isolates desktop-local trust. |
| Mirror or offline collection | mirror_offline | stable | **preview** | yes | Portable collections certify, but the live-validation claim narrows because offline cannot prove live contract freshness. |

## Drill corpora

The certification packet exercises every required drill corpus:

| Corpus | Meaning | Behavior certified |
|---|---|---|
| schema_stale | Stale or cached-beyond-window contract schema | Staleness labeled; live-validated send blocked; no silent raw fallback. |
| origin_changed_rerun | Rerun whose resolved origin changed | Origin change enumerated; dispatch blocked until reviewed. |
| persisted_operation_drift | Persisted id/hash no longer matches the operation text | Send blocked behind rerun/regenerate/cancel; no silent raw fallback. |
| persisted_operation_deprecation | Persisted operation bound to a deprecated/removed contract version | Breaking-risk note surfaced; send blocked; no silent raw fallback. |
| mirror_offline_snapshot | Collection reopened offline or from a mirror | Portability preserved; live-validation claim narrowed and labeled. |
| export_redaction | Export/redaction posture check on collections and history | Metadata-only safe default; redaction classes applied; origin/environment identity preserved. |

## Downgrade rules

Every promoted surface has `downgrade_if_missing: true`, and every downgrade rule
is automatic. A profile that overclaims validation confidence or origin stability
narrows automatically; missing proof narrows a surface to `preview`.

| Trigger | Narrows to | Target |
|---|---|---|
| missing_proof | preview | surface:profile_scorecard |
| schema_stale | preview | case:cli.freshness.schema_stale |
| origin_changed | preview | case:managed.origin.changed |
| persisted_operation_drift | preview | case:companion.persisted.drift |
| persisted_operation_deprecation | preview | case:remote.persisted.deprecation |
| mirror_offline_unavailable | preview | profile:mirror_offline |
| overclaimed_validation_confidence | preview | profile:mirror_offline |
| overclaimed_origin_stability | preview | profile:browser_companion |

## Guardrails

- History retention keeps the metadata-only safe default; no case widens retention to support compare UX.
- Schema staleness and persisted-operation drift/deprecation never silently fall back to raw request execution.
- Managed and browser-companion origins never inherit desktop-local trust or naming.
- Certification is not desktop-only: non-desktop profiles and mirror/offline corpora are exercised, and no profile certifies from live-online fixtures alone.
- Certification narrows automatically when a contract or origin semantic changes.

## Redaction and privacy

- The packet never includes raw endpoint URLs, raw secrets, raw credential bodies, or raw request/response payloads.
- Cases reference fixtures and upstream packets by repo-relative path rather than inline payloads.
- Support and export bundles carry certification state with redaction classes only.

## Upstream packet references

| Upstream | Record kind | Verified |
|---|---|---|
| API-collection matrix | freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix | true |
| Contract freshness banners | ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows | true |
| Request-origin truth | implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review | true |
| Persisted-operation detail | add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules | true |
| Request-history rows | implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare | true |
| Auth sheets and portability | ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability | true |
| Operation-collection and request-list views | implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges | true |

## Verification

Run `cargo test -p aureline-api` to verify the embedded packet deserializes,
validates, covers every drill corpus, and narrows overclaiming profiles.
