//! Stable AI graduation state and support-export gate publication.
//!
//! Reads the checked-in stable graduation state for all three claimed AI
//! surfaces — inline chat (local-first), review chat (cheapest), and AI
//! scoped-apply — and evaluates them against the stable provider/model
//! registry. Promotes all three surfaces from preview to stable rollout state
//! under `policy-epoch:stable:0004`.
//!
//! The stable graduation follows the same wire shape and validation rules as
//! the beta graduation in [`crate::graduation`], but uses the stable registry
//! state (`provider-model-registry:stable:2026-06-01`) and promotes providers
//! to the `generally_admitted` lifecycle class.

use std::collections::BTreeMap;

pub use crate::graduation::{
    AiGraduationConsumerSurfaceClass, AiGraduationGateState, AiGraduationPacket, AiGraduationState,
    AiGraduationSupportClass, AiGraduationViolation, AI_GRADUATION_PACKET_RECORD_KIND,
    AI_GRADUATION_STATE_RECORD_KIND, AI_GRADUATION_STATE_SCHEMA_VERSION,
    REQUIRED_BETA_EVIDENCE_KINDS,
};

/// Registry state identifier for the stable release.
pub const STABLE_REGISTRY_STATE_ID: &str = "provider-model-registry:stable:2026-06-01";

/// Policy epoch identifier for the stable release.
pub const STABLE_POLICY_EPOCH: &str = "policy-epoch:stable:0004";

/// Graduation state identifier for the stable release.
pub const STABLE_GRADUATION_STATE_ID: &str = "ai-graduation-state:stable:2026-06-01";

/// Eval thresholds ref anchored to the stable threshold set.
pub const STABLE_EVAL_THRESHOLDS_REF: &str =
    "artifacts/ai/m4/eval_thresholds.yaml#threshold-set:ai-stable:2026-06-01";

/// Path to the stable eval thresholds artifact (relative to repo root).
pub const STABLE_EVAL_THRESHOLDS_ARTIFACT: &str = "artifacts/ai/m4/eval_thresholds.yaml";

/// Path to the stable graduation state artifact (relative to repo root).
pub const STABLE_GRADUATION_STATE_ARTIFACT: &str =
    "artifacts/ai/m4/publish_stable_ai_graduation_packets/graduation_state.json";

/// Path to the stable support-export artifact (relative to repo root).
pub const STABLE_SUPPORT_EXPORT_ARTIFACT: &str =
    "artifacts/ai/m4/publish_stable_ai_graduation_packets/support_export.json";

/// Path to the stable docs projection (relative to repo root).
pub const STABLE_DOCS_PROJECTION: &str = "docs/ai/m4/publish_stable_ai_graduation_packets.md";

/// Path to the stable registry fixture (relative to repo root).
pub const STABLE_FIXTURE_REGISTRY: &str =
    "fixtures/ai/m4/publish_stable_ai_graduation_packets/registry_packet.json";

/// Returns the checked-in stable AI graduation state artifact.
///
/// Loads the graduation state from `artifacts/ai/m4/publish_stable_ai_graduation_packets/graduation_state.json`
/// and populates [`AiGraduationState::packets`] from the three standalone
/// packet artifacts when not already embedded in the state.
///
/// # Errors
///
/// Returns a JSON parse error if any checked-in artifact is malformed.
pub fn current_stable_graduation_state() -> Result<AiGraduationState, serde_json::Error> {
    let mut state: AiGraduationState = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/publish_stable_ai_graduation_packets/graduation_state.json"
    )))?;
    if state.packets.is_empty() {
        state.packets = current_stable_graduation_packet_artifacts()?
            .into_values()
            .collect();
    }
    Ok(state)
}

/// Returns the checked-in standalone stable packet artifacts keyed by packet id.
///
/// Loads all three stable graduation packets from `artifacts/ai/m4/publish_stable_ai_graduation_packets/`.
///
/// # Errors
///
/// Returns a JSON parse error if any checked-in packet artifact is malformed.
pub fn current_stable_graduation_packet_artifacts(
) -> Result<BTreeMap<String, AiGraduationPacket>, serde_json::Error> {
    let packets = [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/publish_stable_ai_graduation_packets/inline_chat_local_first_stable.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/publish_stable_ai_graduation_packets/review_chat_cheapest_stable.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/publish_stable_ai_graduation_packets/ai_apply_scoped_stable.json"
        )),
    ];
    let mut parsed = BTreeMap::new();
    for packet in packets {
        let packet: AiGraduationPacket = serde_json::from_str(packet)?;
        parsed.insert(packet.graduation_packet_id.clone(), packet);
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests;
