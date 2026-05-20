//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone. Breadcrumbs, bookmarks, and
//! navigation history can project through [`aureline_navigation::target_model`]
//! when they need semantic target fidelity rather than path-only ancestry.

#![doc(html_root_url = "https://docs.rs/aureline-shell/0.0.0")]
#![allow(
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::derivable_impls,
    clippy::if_same_then_else,
    clippy::large_enum_variant,
    clippy::match_like_matches_macro,
    clippy::missing_const_for_thread_local,
    clippy::needless_borrow,
    clippy::needless_lifetimes,
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::permissions_set_readonly_false,
    clippy::question_mark,
    clippy::redundant_closure,
    clippy::redundant_guards,
    clippy::too_many_arguments,
    clippy::unnecessary_map_or,
    clippy::unnecessary_to_owned,
    clippy::useless_format,
    clippy::wildcard_in_or_patterns,
    clippy::wrong_self_convention
)]

pub mod a11y;
pub mod about;
pub mod activity_center;
pub mod activity_timeline;
pub mod admin_alpha;
pub mod admin_audit_export_beta;
pub mod ai_context_inspector;
pub mod ai_truth_strip;
pub mod app_frame;
pub mod background_work_status;
pub mod badges;
pub mod bootstrap;
pub mod breadcrumbs;
pub mod build_intelligence_beta;
pub mod change_object_inspector;
pub mod chrome;
pub mod clone;
pub mod collection_truth;
pub mod collection_truth_corpus;
pub mod command_forms;
pub mod command_parity;
pub mod command_reference;
pub mod commands;
pub mod companion_handoff;
pub mod debug_seed;
pub mod debug_ui;
pub mod debugger_host_beta;
pub mod deeplink;
pub mod deployment_profile;
pub mod desktop_continuity_alpha;
pub mod diagnostics;
pub mod docs_browser;
pub mod drift_truth;
pub mod durable_attention_beta;
pub mod efficiency;
pub mod embedded;
pub mod embedded_boundary;
pub mod embedded_boundary_audit;
pub mod enterprise_drill_baseline;
pub mod entry_interstitials;
pub mod env_inspect;
pub mod experiments_governance;
pub mod explorer;
pub mod extensions;
pub mod git_changes;
pub mod graph_state_card;
pub mod help;
pub mod help_about;
pub mod help_packs;
pub mod host_boundary_cues;
pub mod import;
pub mod inspectors;
pub mod install_review_fact_grid;
pub mod interaction_integrity;
pub mod interaction_transfer;
pub mod keybindings;
pub mod layout;
pub mod learning_mode;
pub mod learning_tour_alpha;
pub mod locale_pack_beta;
pub mod macros;
pub mod managed_boundary;
pub mod managed_truth;
pub mod managed_workspace_labels;
pub mod migration_center;
pub mod migration_corpus;
pub mod migration_wizard;
pub mod network_badges;
pub mod network_trust_beta;
pub mod notebook_alpha;
pub mod notebook_trust_badges;
pub mod notifications;
pub mod offline_entitlement_beta;
pub mod oidc_system_browser_beta;
pub mod onboarding;
pub mod onboarding_metrics;
pub mod ownership_audit;
pub mod palette;
pub mod passkey_step_up_beta;
pub mod path_truth;
pub mod permission_prompts;
pub mod platform_integration;
pub mod policy_pack_beta;
pub mod policy_simulation_beta;
pub mod portable_bundle_inspector;
pub mod preview_scope_labels;
pub mod preview_truth;
pub mod previews;
pub mod profiling_alpha;
pub mod public_truth;
pub mod recovery;
pub mod region_tenant_key_mode_beta;
pub mod release_center;
pub mod request_workspace;
pub mod restore;
pub mod review;
pub mod review_preview;
pub mod run_context;
pub mod run_debug_profiles_beta;
pub mod runtime;
pub mod runtime_adaptation;
pub mod safe_preview_card;
pub mod save_review;
pub mod scope_truth;
pub mod search;
pub mod secret_broker_beta;
pub mod service_health;
pub mod start_center;
pub mod state_cards;
pub mod status;
pub mod status_bar;
pub mod support_center;
pub mod support_matrix_beta;
pub mod support_seed;
pub mod system_browser_return_paths;
pub mod target_discovery_beta;
pub mod tasks_seed;
pub mod terminal_pane;
pub mod test_runner_beta;
pub mod token_state_audit;
pub mod transfer;
pub mod wedge_inspector;
pub mod windowing;
pub mod windows;
pub mod workset_switcher;
pub mod workspace_switcher;
pub mod workspace_trust_beta;

pub use aureline_navigation::target_model as navigation_target_model;
