//! Deterministic seeded sample of [`ServiceHealthProbeReading`]s.
//!
//! The runtime probes that mint live readings live in other crates
//! (`aureline-runtime`, `aureline-provider`, `aureline-policy`, the
//! license broker). Until those are wired through the shell, the
//! seeded set here is the source of truth that:
//!
//! - feeds the headless inspect binary
//!   (`aureline_shell_service_health_inspect`);
//! - feeds the support-export plaintext block;
//! - feeds the fixture corpus under `fixtures/ops/m3/service_health_cards/`
//!   so reviewers can replay the exact projection on disk.
//!
//! The set deliberately covers every state in
//! [`crate::service_health::ServiceContractStateClass`] so a single
//! seeded packet exercises the whole contract-state vocabulary at once.

use super::aggregator::{
    AffectedWorkflowClass, BoundaryClass, LocalContinuityClass, ServiceContractStateClass,
    ServiceFamilyClass, ServiceHealthAggregator, ServiceHealthProbeReading,
};

/// Default deterministic `as_of` used by the seeded packet so renders are
/// stable across machines and test runs.
pub const SEEDED_AGGREGATOR_AS_OF: &str = "2026-05-19T12:30";

/// Default deterministic aggregator id used by the seeded packet.
pub const SEEDED_AGGREGATOR_ID: &str = "service_health_aggregator:m3.beta.seeded";

/// Return the deterministic seeded probe-reading set. Stable across calls.
pub fn seeded_probe_readings() -> Vec<ServiceHealthProbeReading> {
    vec![
        ServiceHealthProbeReading {
            card_id: "card:language_services".to_owned(),
            service_family: ServiceFamilyClass::LanguageServices,
            boundary_class: BoundaryClass::LocalOnly,
            contract_state: ServiceContractStateClass::Ready,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![],
            last_checked: Some("2026-05-19T12:28".to_owned()),
            state_explanation:
                "Language services are responding within their normal latency budget.".to_owned(),
            diagnostics_action: "shell.command:diagnostics.language_services".to_owned(),
            detail_tokens: vec!["framework_lsp".to_owned(), "local_indexer".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:ai_assist".to_owned(),
            service_family: ServiceFamilyClass::AiAssist,
            boundary_class: BoundaryClass::VendorProvider,
            contract_state: ServiceContractStateClass::Degraded,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![
                AffectedWorkflowClass::AiCompletion,
                AffectedWorkflowClass::AiChat,
            ],
            last_checked: Some("2026-05-19T12:25".to_owned()),
            state_explanation:
                "AI provider is responding slowly; completions and chat retries are firing."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.ai_assist".to_owned(),
            detail_tokens: vec!["provider_class:vendor_chat".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:workspace_sync".to_owned(),
            service_family: ServiceFamilyClass::Sync,
            boundary_class: BoundaryClass::LocalWithRemoteRequired,
            contract_state: ServiceContractStateClass::LocalOnly,
            local_continuity: LocalContinuityClass::LocalSafeReadOnly,
            affected_workflows: vec![AffectedWorkflowClass::WorkspaceSync],
            last_checked: Some("2026-05-19T12:20".to_owned()),
            state_explanation:
                "Workspace sync is unreachable; local edits keep working, pushes pause until \
                 reconnect."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.workspace_sync".to_owned(),
            detail_tokens: vec!["last_synced_offline".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:docs_knowledge".to_owned(),
            service_family: ServiceFamilyClass::DocsKnowledge,
            boundary_class: BoundaryClass::LocalWithRemoteOptional,
            contract_state: ServiceContractStateClass::Stale,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![AffectedWorkflowClass::DocsBrowseRemote],
            last_checked: Some("2026-05-18T08:00".to_owned()),
            state_explanation:
                "Docs mirror has not refreshed inside its review window; local docs remain \
                 available."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.docs_mirror".to_owned(),
            detail_tokens: vec!["mirror_class:docs_mirror".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:release_channel".to_owned(),
            service_family: ServiceFamilyClass::ReleaseChannel,
            boundary_class: BoundaryClass::LocalWithRemoteOptional,
            contract_state: ServiceContractStateClass::ContractMismatch,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![],
            last_checked: Some("2026-05-19T12:10".to_owned()),
            state_explanation:
                "Release-channel response did not match the agreed manifest schema; results \
                 are held until the contract clears."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.release_channel".to_owned(),
            detail_tokens: vec!["schema_skew:claim_manifest".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:telemetry".to_owned(),
            service_family: ServiceFamilyClass::Telemetry,
            boundary_class: BoundaryClass::Hosted,
            contract_state: ServiceContractStateClass::PolicyBlocked,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![AffectedWorkflowClass::TelemetryUpload],
            last_checked: Some("2026-05-19T12:15".to_owned()),
            state_explanation:
                "Telemetry upload is paused by workspace policy; local crash capture continues \
                 to write to disk."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.telemetry".to_owned(),
            detail_tokens: vec!["policy_class:workspace_policy".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:marketplace".to_owned(),
            service_family: ServiceFamilyClass::Marketplace,
            boundary_class: BoundaryClass::Hosted,
            contract_state: ServiceContractStateClass::Unavailable,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![
                AffectedWorkflowClass::MarketplaceBrowse,
                AffectedWorkflowClass::ExtensionInstall,
            ],
            last_checked: Some("2026-05-19T12:18".to_owned()),
            state_explanation:
                "Marketplace fetch is unreachable; installed extensions and cached browse \
                 remain usable."
                    .to_owned(),
            diagnostics_action: "shell.command:diagnostics.marketplace".to_owned(),
            detail_tokens: vec!["mirror_class:marketplace_mirror".to_owned()],
        },
        ServiceHealthProbeReading {
            card_id: "card:license_entitlement".to_owned(),
            service_family: ServiceFamilyClass::LicenseEntitlement,
            boundary_class: BoundaryClass::LocalWithRemoteOptional,
            contract_state: ServiceContractStateClass::Ready,
            local_continuity: LocalContinuityClass::LocalSafe,
            affected_workflows: vec![],
            last_checked: Some("2026-05-19T12:29".to_owned()),
            state_explanation: "License broker is current; local entitlements are honoured."
                .to_owned(),
            diagnostics_action: "shell.command:diagnostics.license".to_owned(),
            detail_tokens: vec!["broker_class:local_broker".to_owned()],
        },
    ]
}

/// Build the deterministic seeded aggregator. Stable across calls.
pub fn seeded_aggregator() -> ServiceHealthAggregator {
    ServiceHealthAggregator::build(
        SEEDED_AGGREGATOR_ID,
        SEEDED_AGGREGATOR_AS_OF,
        seeded_probe_readings(),
    )
    .expect("seeded aggregator must build")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_aggregator_covers_every_state_in_the_vocabulary() {
        let agg = seeded_aggregator();
        let mut seen = std::collections::BTreeSet::new();
        for card in &agg.cards {
            seen.insert(card.contract_state_token.clone());
        }
        for token in [
            "ready",
            "degraded",
            "local_only",
            "stale",
            "contract_mismatch",
            "policy_blocked",
            "unavailable",
        ] {
            assert!(seen.contains(token), "missing state {token}");
        }
    }

    #[test]
    fn seeded_aggregator_is_deterministic() {
        let a = seeded_aggregator();
        let b = seeded_aggregator();
        assert_eq!(a, b);
    }

    #[test]
    fn seeded_aggregator_overall_local_continuity_is_read_only() {
        let agg = seeded_aggregator();
        assert_eq!(
            agg.overall_local_continuity,
            LocalContinuityClass::LocalSafeReadOnly,
            "sync card with LocalWithRemoteRequired should bring overall continuity to \
             local_safe_read_only",
        );
    }

    #[test]
    fn seeded_aggregator_overall_contract_state_is_worst_severity() {
        let agg = seeded_aggregator();
        // contract_mismatch and unavailable both have severity 4. The
        // rollup is allowed to pick either; the chrome only cares that
        // it's one of those two.
        assert!(matches!(
            agg.overall_contract_state,
            ServiceContractStateClass::ContractMismatch | ServiceContractStateClass::Unavailable
        ));
    }
}
