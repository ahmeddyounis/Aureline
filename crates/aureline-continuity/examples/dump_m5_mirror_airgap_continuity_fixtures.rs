//! Emits the canonical mirror-only and air-gapped continuity fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- registry
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-silent-public-fallback-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-advisory-live-public-fetch-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-trust-root-breaks-offline-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-public-fallback-undisclosed-preview
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-trust-root-undeclared-preview
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-mirror-never-synced-preview
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-packet-evidence-missing-preview
//! cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures -- case-mirror-stale-needs-sync-beta
//! ```

use aureline_continuity::{
    seeded_mirror_airgap_input, seeded_mirror_airgap_page, AdvisoryRevocationSourceClass,
    MirrorAirgapInput, MirrorAirgapPacketEntry, MirrorAirgapPage, MirrorAirgapSupportExport,
    MirrorFreshnessStateClass, PublicFallbackPolicyClass, TrustRootPostureClass,
    TrustRootRenewalClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_mirror_airgap_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("registry") => print_json(&page.registry)?,
        Some("support-export") => {
            let export = MirrorAirgapSupportExport::from_page(
                "continuity:mirror-airgap:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-silent-public-fallback-withdrawn") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:mirror-only-self-hosted",
                |packet| {
                    packet.public_fallback_policy = PublicFallbackPolicyClass::SilentPublicFallback;
                    packet.public_fallback_policy_token =
                        PublicFallbackPolicyClass::SilentPublicFallback
                            .as_str()
                            .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:silent-public-fallback",
                "Case - a mirror-only row silently falls back to public endpoints (withdrawn)",
                input,
            ))?;
        }
        Some("case-advisory-live-public-fetch-withdrawn") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:air-gapped-sovereign",
                |packet| {
                    packet.advisory_revocation_source =
                        AdvisoryRevocationSourceClass::LivePublicFetch;
                    packet.advisory_revocation_source_token =
                        AdvisoryRevocationSourceClass::LivePublicFetch
                            .as_str()
                            .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:advisory-live-public-fetch",
                "Case - an air-gapped row sources advisories from a live public fetch (withdrawn)",
                input,
            ))?;
        }
        Some("case-trust-root-breaks-offline-withdrawn") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:mirror-only-self-hosted",
                |packet| {
                    packet.trust_root.renewal = TrustRootRenewalClass::PublicReissueRequired;
                    packet.trust_root.renewal_token = TrustRootRenewalClass::PublicReissueRequired
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:trust-root-breaks-offline",
                "Case - a mirror-only trust root requires a live public reissue to renew (withdrawn)",
                input,
            ))?;
        }
        Some("case-public-fallback-undisclosed-preview") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:self-hosted-restricted",
                |packet| {
                    packet.public_fallback_policy = PublicFallbackPolicyClass::Undisclosed;
                    packet.public_fallback_policy_token =
                        PublicFallbackPolicyClass::Undisclosed.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:public-fallback-undisclosed",
                "Case - a self-hosted-restricted row does not state its public-fallback policy (preview)",
                input,
            ))?;
        }
        Some("case-trust-root-undeclared-preview") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:mirror-only-self-hosted",
                |packet| {
                    packet.trust_root.posture = TrustRootPostureClass::TrustRootUndisclosed;
                    packet.trust_root.posture_token = TrustRootPostureClass::TrustRootUndisclosed
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:trust-root-undeclared",
                "Case - a mirror-only row does not declare its trust-root continuity (preview)",
                input,
            ))?;
        }
        Some("case-mirror-never-synced-preview") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:mirror-only-self-hosted",
                |packet| {
                    packet.mirror_freshness.state = MirrorFreshnessStateClass::NeverSynced;
                    packet.mirror_freshness.state_token =
                        MirrorFreshnessStateClass::NeverSynced.as_str().to_owned();
                    packet.mirror_freshness.last_synced_at = String::new();
                    packet.mirror_freshness.freshness_expires_at = String::new();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:mirror-never-synced",
                "Case - a mirror-only row whose mirror has never synced (preview)",
                input,
            ))?;
        }
        Some("case-packet-evidence-missing-preview") => {
            let mut input = seeded_mirror_airgap_input();
            input
                .packets
                .retain(|packet| packet.packet_id != "continuity-mirror:air-gapped-sovereign");
            print_json(&case_page(
                "continuity:mirror-airgap:case:packet-evidence-missing",
                "Case - a claimed air-gapped row carries no continuity packet (preview)",
                input,
            ))?;
        }
        Some("case-mirror-stale-needs-sync-beta") => {
            let mut input = seeded_mirror_airgap_input();
            with_packet(
                &mut input,
                "continuity-mirror:mirror-only-self-hosted",
                |packet| {
                    packet.mirror_freshness.state = MirrorFreshnessStateClass::StaleNeedsSync;
                    packet.mirror_freshness.state_token = MirrorFreshnessStateClass::StaleNeedsSync
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:mirror-airgap:case:mirror-stale-needs-sync",
                "Case - a mirror-only mirror has aged out and needs a fresh sync (beta)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_packet(
    input: &mut MirrorAirgapInput,
    packet_id: &str,
    mutate: impl FnOnce(&mut MirrorAirgapPacketEntry),
) {
    let packet = input
        .packets
        .iter_mut()
        .find(|packet| packet.packet_id == packet_id)
        .unwrap_or_else(|| panic!("missing seeded packet: {packet_id}"));
    mutate(packet);
}

fn case_page(page_id: &str, page_label: &str, input: MirrorAirgapInput) -> MirrorAirgapPage {
    MirrorAirgapPage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
