//! Desktop shell: canonical zones, layout, and live frame wiring.
//!
//! This crate is the production shell container. It defines the canonical
//! shell-zone ids, default metrics, and a small live desktop frame that renders
//! placeholder occupants in each declared zone.

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
pub mod admin_alpha;
pub mod ai_context_inspector;
pub mod ai_truth_strip;
pub mod app_frame;
pub mod badges;
pub mod bootstrap;
pub mod breadcrumbs;
pub mod chrome;
pub mod clone;
pub mod command_parity;
pub mod commands;
pub mod debug_seed;
pub mod debug_ui;
pub mod debugger_host_beta;
pub mod deeplink;
pub mod desktop_continuity_alpha;
pub mod diagnostics;
pub mod docs_browser;
pub mod drift_truth;
pub mod efficiency;
pub mod embedded;
pub mod embedded_boundary_audit;
pub mod env_inspect;
pub mod experiments_governance;
pub mod explorer;
pub mod extensions;
pub mod git_changes;
pub mod graph_state_card;
pub mod help;
pub mod help_about;
pub mod host_boundary_cues;
pub mod import;
pub mod inspectors;
pub mod install_review_fact_grid;
pub mod keybindings;
pub mod layout;
pub mod learning_tour_alpha;
pub mod managed_truth;
pub mod managed_workspace_labels;
pub mod migration_center;
pub mod migration_corpus;
pub mod migration_wizard;
pub mod notebook_alpha;
pub mod notebook_trust_badges;
pub mod notifications;
pub mod onboarding;
pub mod onboarding_metrics;
pub mod ownership_audit;
pub mod palette;
pub mod path_truth;
pub mod permission_prompts;
pub mod previews;
pub mod profiling_alpha;
pub mod recovery;
pub mod release_center;
pub mod request_workspace;
pub mod restore;
pub mod review_preview;
pub mod run_context;
pub mod run_debug_profiles_beta;
pub mod runtime;
pub mod runtime_adaptation;
pub mod safe_preview_card;
pub mod save_review;
pub mod scope_truth;
pub mod search;
pub mod service_health;
pub mod start_center;
pub mod state_cards;
pub mod status;
pub mod status_bar;
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
