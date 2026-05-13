//! Search-facing shell projections.
//!
//! This module contains shell-level search surfaces that are not the search
//! engine itself. They consume canonical `aureline-search` records and project
//! compact, inspectable UI/support artifacts without minting a second search
//! truth model.

pub mod ranking_reason_card;

pub use ranking_reason_card::{
    ranking_reason_cards_for_planned_result_set, ranking_reason_cards_for_quick_open_snapshot,
    RankingReasonCard, RankingReasonSignal, RankingReasonSupportExport,
    RANKING_REASON_CARD_SCHEMA_VERSION, RANKING_REASON_SUPPORT_EXPORT_SCHEMA_VERSION,
};
