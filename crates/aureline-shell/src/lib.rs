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

pub(crate) mod a11y;
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
pub mod commands;
pub mod debug_seed;
pub mod deeplink;
pub mod docs_browser;
pub mod drift_truth;
pub mod efficiency;
pub mod embedded;
pub mod explorer;
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
pub mod managed_truth;
pub mod managed_workspace_labels;
pub mod notebook_alpha;
pub mod notebook_trust_badges;
pub mod notifications;
pub mod onboarding;
pub mod palette;
pub mod path_truth;
pub mod permission_prompts;
pub mod previews;
pub mod recovery;
pub mod release_center;
pub mod restore;
pub mod review_preview;
pub mod run_context;
pub mod runtime;
pub mod safe_preview_card;
pub mod save_review;
pub mod scope_truth;
pub mod search;
pub mod start_center;
pub mod state_cards;
pub mod status;
pub mod status_bar;
pub mod support_seed;
pub mod tasks_seed;
pub mod terminal_pane;
pub mod wedge_inspector;
pub mod windowing;
pub mod workspace_switcher;
