# Certify API, database, and browser-runtime workflows with mutation, redaction, and scale drills

## Scope

This document describes the canonical M5 certification qualification packet for API, database, and browser-runtime workflows with mutation, redaction, and scale drills in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/certify_api_database_and_browser_runtime_workflows_with_mutation_redaction_and_scale_drills/mod.rs`
- Schema: `schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json`
- Checked-in packet: `artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json`
- Fixtures: `fixtures/data/m5/certify_api_database_and_browser_runtime_workflows_with_mutation_redaction_and_scale_drills/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| API workflow certification | stable | stable | API workflows (request workspace, composer, response viewers) are certified with explicit environment, auth, mutation risk, and export posture. |
| Database workflow certification | stable | stable | Database workflows (connection browsers, statement safety, explain plans, result grids, staged row mutations, query history, handoff) are certified with bounded result sets and explicit write posture. |
| Browser-runtime certification | stable | stable | Browser-runtime workflows are certified with trust classes, timing disclosure, and response preview boundaries. |
| Mutation drill | stable | stable | Mutation drills verify that all mutating flows require preview, confirmation, rollback path visibility, explicit write posture, and completed step-up flows. |
| Redaction drill | stable | stable | Redaction drills verify that auth sources, secrets, exports, support bundles, and history are redacted or stored with secret-safe posture. |
| Scale drill | stable | stable | Scale drills verify that result sets are bounded, virtualized, and limited by timeout and memory caps across API and database workflows. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Certification packets never include raw secrets, raw credentials, raw connection strings, or unbounded result sets.
- Upstream packet references use stable record kinds and repo-relative paths rather than inline payloads.
- Support-bundle-safe exports use redaction classes only.

## Mutation drills

| Drill | Target | Pass | Rationale |
|---|---|---|---|
| Preview required | API workflow certification | true | All mutating API requests require a preview step before execution. |
| Confirmation required | Database workflow certification | true | All mutating SQL statements require explicit confirmation before apply. |
| Rollback path visible | Database workflow certification | true | Rollback paths are visible from all mutating database surfaces. |
| Write posture explicit | Database workflow certification | true | Write posture is explicit on all database tooling surfaces via statement safety classifier and write-mode bar. |
| Step-up flow completed | Database workflow certification | true | Protected-target step-up flows are completed for high-impact mutations. |

## Redaction drills

| Drill | Target | Pass | Rationale |
|---|---|---|---|
| Auth source redacted | API workflow certification | true | Auth sources in API workflows show inspectable mode and provenance without raw secrets. |
| Secret-safe storage | Database workflow certification | true | Database credentials and secrets are stored with secret-safe posture. |
| Export redaction applied | API workflow certification | true | Redaction-safe export applies full_redaction, metadata_only, or safe_preview classes to all exports. |
| Support bundle safe | Browser-runtime certification | true | Browser-runtime trust classes and response previews do not leak raw credentials or tokens. |
| History redacted | Database workflow certification | true | Query history retains metadata and refs while redacting raw secrets and full result sets. |

## Scale drills

| Drill | Target | Pass | Rationale |
|---|---|---|---|
| Row count bounded | Database workflow certification | true | Row counts are bounded and disclosed on all result grids and query outputs. |
| Result set limited | Database workflow certification | true | Result sets are hard-limited before streaming to viewers or handoff surfaces. |
| Virtualization active | Database workflow certification | true | Result-grid virtualization is active for large result sets. |
| Timeout enforced | API workflow certification | true | API requests enforce timeout boundaries and surface timeout state in timing tabs. |
| Memory cap enforced | Database workflow certification | true | Database result sets enforce memory caps before rendering or export. |

## Upstream packet references

| Upstream | Record kind | Verified |
|---|---|---|
| Request workspace documents | materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors | true |
| Request composer | implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export | true |
| REST/GraphQL response viewers | ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes | true |
| Connection browsers | implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling | true |
| Statement safety classifier | add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows | true |
| Result grid virtualization | ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth | true |
| Staged row mutations | add_staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions | true |
| Explain plan | implement_explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows | true |
| Query history | ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth | true |
| Result handoff | integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces | true |

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
