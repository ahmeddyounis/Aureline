//! Smoke harness for the reactive-state prototype.
//!
//! One row per ADR 0005 acceptance sub-case: nominal snapshot
//! plus delta, warming → full, out-of-order delta triggering a
//! resync, upstream-input drift demoting a derived view,
//! replay never advancing the live epoch, imported frame
//! attached read-only, terminal-unavailable, backpressure
//! switch forcing a resync, and a cross-lane refresh-ordering
//! scenario where the authority's fresh snapshot precedes the
//! derived view's.
//!
//! Each row runs a scripted sequence against a fresh
//! [`ReactiveStore`] and captures a [`ScenarioReport`] with
//! the frames it emitted, the trace it accumulated, and the
//! hook-counter snapshot taken at end of run. Metrics are
//! counts only — no wall-clock — so the committed artifacts
//! stay byte-stable across hosts.

use std::fmt::Write as _;

use crate::envelope::{
    write_key, write_kv_string, write_kv_u64, write_string_literal, BackpressureMode, CausedBy,
    Completeness, Freshness, InputDigest, Invalidation, ProducerRef, StaleReason,
    SubscriptionEnvelope, TerminalReason,
};
use crate::hooks::HookCounters;
use crate::producers::{
    derived_diagnostics, file_identity, graph_neighborhood, provider_overlay, shell_health,
    synthetic_instance, window, workspace, workspace_readiness,
};
use crate::store::{ReactiveStore, SamplePayload, StoreError};
use crate::trace::{comma_nl, newline_indent, write_trace_event, TraceEvent};

/// Frozen corpus identifier. Bumped only when the harness's
/// output schema itself changes (not on scenario additions).
pub const CORPUS_ID: &str = "aureline.reactive_state_invalidation_trace_examples.v1";

/// Schema version for the emitted invalidation-trace JSON.
pub const SCHEMA_VERSION: u32 = 1;

/// One row in the scenario table. `run` is a pure function
/// from an empty store to a scenario report.
#[derive(Debug, Clone, Copy)]
pub struct ScenarioSpec {
    pub label: &'static str,
    pub scenario_summary: &'static str,
    pub primary_hooks: &'static [&'static str],
    pub adr_sections: &'static [&'static str],
    pub run: fn() -> ScenarioReport,
}

/// Per-scenario report. `label` is the stable filename stem
/// under `artifacts/state/invalidation_trace_examples/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub label: &'static str,
    pub scenario_summary: &'static str,
    pub primary_hooks: &'static [&'static str],
    pub adr_sections: &'static [&'static str],
    pub trace: Vec<TraceEvent>,
    pub hook_counters: HookCounters,
    pub final_consumer_notes: Vec<String>,
    pub featured_envelopes: Vec<FeaturedEnvelope>,
    pub tick_count: u64,
}

/// A single envelope worth pinning in the per-scenario
/// artifact as a "representative frame". Most scenarios pin the
/// snapshot or the resync_required frame; some pin more than
/// one. The fixture index under
/// `fixtures/state/envelope_examples/` exports these as
/// boundary-schema-valid JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeaturedEnvelope {
    pub label: &'static str,
    pub envelope: SubscriptionEnvelope,
    pub note: String,
}

/// Aggregate counters across scenarios.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AggregateReport {
    pub total_scenarios: u64,
    pub total_snapshot_frames: u64,
    pub total_delta_frames: u64,
    pub total_resync_required_frames: u64,
    pub total_terminal_frames: u64,
    pub total_freshness_downgrades: u64,
    pub total_completeness_changes: u64,
    pub total_imported_attaches: u64,
    pub total_replay_sessions: u64,
    pub total_backpressure_switches: u64,
}

/// Full harness output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReport {
    pub schema_version: u32,
    pub corpus_id: &'static str,
    pub scenarios: Vec<ScenarioReport>,
    pub aggregate: AggregateReport,
}

/// Run the full scenario table and produce a report. Byte-
/// stable: same inputs, same outputs, same emitted bytes.
pub fn run_harness() -> HarnessReport {
    let mut scenarios = Vec::with_capacity(SCENARIOS.len());
    let mut agg = AggregateReport::default();
    for spec in SCENARIOS {
        let report = (spec.run)();
        for ev in &report.trace {
            if let TraceEvent::FrameEmit { envelope, .. } = ev {
                match envelope.frame_class {
                    crate::envelope::FrameClass::Snapshot => agg.total_snapshot_frames += 1,
                    crate::envelope::FrameClass::Delta => agg.total_delta_frames += 1,
                    crate::envelope::FrameClass::ResyncRequired => {
                        agg.total_resync_required_frames += 1
                    }
                    crate::envelope::FrameClass::Terminal => agg.total_terminal_frames += 1,
                }
            }
        }
        agg.total_freshness_downgrades += report.hook_counters.subscription_freshness_downgrade;
        agg.total_completeness_changes += report.hook_counters.subscription_completeness_changed;
        agg.total_imported_attaches += report.hook_counters.subscription_imported_attach;
        agg.total_replay_sessions += report.hook_counters.subscription_replay_begin;
        agg.total_backpressure_switches +=
            report.hook_counters.subscription_snapshot_required_switch;
        scenarios.push(report);
    }
    agg.total_scenarios = SCENARIOS.len() as u64;
    HarnessReport {
        schema_version: SCHEMA_VERSION,
        corpus_id: CORPUS_ID,
        scenarios,
        aggregate: agg,
    }
}

// -------------------------------------------------------------------------
// Scenario table.
// -------------------------------------------------------------------------

pub const SCENARIOS: &[ScenarioSpec] = &[
    ScenarioSpec {
        label: "shell_health_nominal",
        scenario_summary:
            "Shell-health subscription emits its first authoritative snapshot followed by an authoritative delta. Happy path for an execution-lane producer; consumer projection stays fresh/full across both frames.",
        primary_hooks: &[
            "subscription_subscribe",
            "subscription_snapshot_emit",
            "subscription_delta_emit",
            "subscription_delta_apply",
        ],
        adr_sections: &[
            "Subscription envelope fields",
            "Subscription lifecycle (subscribe, snapshot, delta)",
            "Freshness labels: authoritative",
            "Completeness labels: full",
        ],
        run: scenarios::shell_health_nominal,
    },
    ScenarioSpec {
        label: "workspace_readiness_warming_then_full",
        scenario_summary:
            "Workspace readiness begins warming / partial, then completes. Two snapshot frames are emitted: the first labels freshness = warming and completeness = partial; the second moves to authoritative / full. Consumer observes one freshness upgrade and one completeness change.",
        primary_hooks: &[
            "subscription_subscribe",
            "subscription_snapshot_emit",
            "subscription_completeness_changed",
        ],
        adr_sections: &[
            "Freshness labels: warming -> authoritative",
            "Completeness labels: partial -> full",
            "Minimal ordering, replay, and imported-history assumptions",
        ],
        run: scenarios::workspace_readiness_warming_then_full,
    },
    ScenarioSpec {
        label: "file_identity_delta_gap_triggers_resync",
        scenario_summary:
            "File-identity stream emits snapshot then two deltas; harness scripts an out-of-order third delta (seq=4 when seq=3 was expected). Store detects the gap and emits a resync_required frame with stale_reason = causality_lost, followed by a fresh snapshot on a bumped snapshot_epoch.",
        primary_hooks: &[
            "subscription_subscribe",
            "subscription_snapshot_emit",
            "subscription_delta_emit",
            "subscription_resync_required_emit",
            "subscription_freshness_downgrade",
        ],
        adr_sections: &[
            "Subscription lifecycle (resync_required)",
            "Resubscribe and full-refresh behaviour",
            "Stale-reason vocabulary: causality_lost",
        ],
        run: scenarios::file_identity_delta_gap_triggers_resync,
    },
    ScenarioSpec {
        label: "derived_view_upstream_input_stale",
        scenario_summary:
            "Derived diagnostics view rides on file-identity. File-identity advances (new input digest); derived producer emits resync_required with stale_reason = upstream_input_stale, then a fresh snapshot with a bumped derivation_epoch and the new input digest set. Consumer observes one freshness downgrade (authoritative-style derived -> stale) and one freshness refresh.",
        primary_hooks: &[
            "subscription_subscribe",
            "subscription_snapshot_emit",
            "subscription_delta_emit",
            "subscription_resync_required_emit",
            "subscription_freshness_downgrade",
        ],
        adr_sections: &[
            "Authoritative versus derived state",
            "Stale-reason vocabulary: upstream_input_stale",
            "Subscription lifecycle (resync_required)",
        ],
        run: scenarios::derived_view_upstream_input_stale,
    },
    ScenarioSpec {
        label: "refresh_ordering_authority_before_derived",
        scenario_summary:
            "Cross-lane refresh ordering: both the authoritative file-identity subscription and the derived diagnostics subscription are live. File-identity publishes a new authoritative snapshot first; only after that does the derived producer observe the upstream digest drift, emit resync_required, and publish its own fresh snapshot. The trace proves the authority's fresh snapshot precedes the derived view's.",
        primary_hooks: &[
            "subscription_snapshot_emit",
            "subscription_resync_required_emit",
            "subscription_freshness_downgrade",
        ],
        adr_sections: &[
            "Authoritative versus derived state",
            "Minimal ordering, replay, and imported-history assumptions",
            "Resubscribe and full-refresh behaviour",
        ],
        run: scenarios::refresh_ordering_authority_before_derived,
    },
    ScenarioSpec {
        label: "replay_does_not_advance_live_epoch",
        scenario_summary:
            "Graph-neighborhood subscription is warming live at epoch 1; a support-bundle replay session attaches and emits two replayed deltas with freshness = replayed and stale_reason = replayed_from_bundle. The live snapshot_epoch remains at 1 and the consumer's ever_replayed flag flips, but its last_delta_seq does not advance off the replay deltas.",
        primary_hooks: &[
            "subscription_replay_begin",
            "subscription_delta_emit",
            "subscription_replay_end",
        ],
        adr_sections: &[
            "Freshness labels: replayed",
            "Stale-reason vocabulary: replayed_from_bundle",
            "Minimal ordering, replay, and imported-history assumptions",
        ],
        run: scenarios::replay_does_not_advance_live_epoch,
    },
    ScenarioSpec {
        label: "imported_snapshot_attached_read_only",
        scenario_summary:
            "Imported LSIF-style graph bundle is attached alongside a live derived graph neighborhood. The attach emits a snapshot with freshness = imported and stale_reason = imported_from_external; the live projection's authoritative lineage is unaffected. The store refuses any attempt to promote the imported frame to authoritative.",
        primary_hooks: &[
            "subscription_imported_attach",
            "subscription_snapshot_emit",
        ],
        adr_sections: &[
            "Freshness labels: imported",
            "Stale-reason vocabulary: imported_from_external",
            "Authoritative versus derived state (imported frames never claim authority)",
        ],
        run: scenarios::imported_snapshot_attached_read_only,
    },
    ScenarioSpec {
        label: "provider_terminal_unavailable",
        scenario_summary:
            "Provider-overlay (CI checks) subscription terminates because the provider's watcher dropped. Terminal frame carries completeness = unavailable, freshness = stale, terminal_reason = unavailable, and stale_reason = watcher_dropped. Consumer marks the projection terminal and waits for explicit repair before resubscribing.",
        primary_hooks: &["subscription_terminate", "subscription_freshness_downgrade"],
        adr_sections: &[
            "Subscription lifecycle (terminal)",
            "Completeness labels: unavailable",
            "Stale-reason vocabulary: watcher_dropped",
        ],
        run: scenarios::provider_terminal_unavailable,
    },
    ScenarioSpec {
        label: "backpressure_snapshot_required_switch",
        scenario_summary:
            "Consumer subscribed with backpressure_mode = coalesced falls behind; the producer coalesces two pending deltas, then the consumer switches to snapshot_required. The switch forces a resync_required with stale_reason = causality_lost; a fresh snapshot follows on a bumped epoch.",
        primary_hooks: &[
            "subscription_backpressure_coalesce",
            "subscription_snapshot_required_switch",
            "subscription_resync_required_emit",
        ],
        adr_sections: &[
            "Subscription envelope fields (backpressure_mode)",
            "Subscription lifecycle (resync_required)",
            "Stale-reason vocabulary: causality_lost",
        ],
        run: scenarios::backpressure_snapshot_required_switch,
    },
];

// -------------------------------------------------------------------------
// Scenario bodies.
// -------------------------------------------------------------------------

mod scenarios {
    use super::*;

    fn final_consumer_notes(store: &ReactiveStore) -> Vec<String> {
        store
            .consumers()
            .iter()
            .map(|c| {
                format!(
                    "sub_id={} query={} scope={}/{} last_epoch={} last_delta_seq={} freshness={} completeness={} stale={} terminal={} ever_replayed={} ever_imported={}",
                    c.subscription_id,
                    c.query_family,
                    c.scope_ref.class.as_str(),
                    c.scope_ref.id,
                    c.last_snapshot_epoch,
                    c.last_delta_seq,
                    c.last_freshness.as_str(),
                    c.last_completeness.as_str(),
                    c.is_stale,
                    c.is_terminal,
                    c.ever_replayed,
                    c.ever_imported,
                )
            })
            .collect()
    }

    fn base_payload(summary: &str, entry_count: u64) -> SamplePayload {
        SamplePayload {
            summary: summary.to_owned(),
            entry_count,
            coverage_ready: 1,
            coverage_total: 1,
            detail_lines: vec![],
        }
    }

    fn partial_payload(summary: &str, ready: u64, total: u64) -> SamplePayload {
        SamplePayload {
            summary: summary.to_owned(),
            entry_count: ready,
            coverage_ready: ready,
            coverage_total: total,
            detail_lines: vec![],
        }
    }

    pub(super) fn shell_health_nominal() -> ScenarioReport {
        let spec = &SCENARIOS[0];
        let mut store = ReactiveStore::new();
        store.register_producer(shell_health(window("win-1")));
        let sid = store
            .subscribe(
                "execution.shell_health",
                &window("win-1"),
                BackpressureMode::Realtime,
            )
            .expect("subscribe");
        let snap = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("shell ready; 0 failures", 0),
                None,
            )
            .expect("snapshot");
        let delta = store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("shell_tick=1; 1 completed command", 1),
                None,
            )
            .expect("delta");

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "snapshot",
                    envelope: snap.envelope.clone(),
                    note: "initial authoritative snapshot; consumer replaces any placeholder projection".to_owned(),
                },
                FeaturedEnvelope {
                    label: "delta",
                    envelope: delta.envelope.clone(),
                    note: "delta_seq=1 applied in order; consumer stays fresh/full".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn workspace_readiness_warming_then_full() -> ScenarioReport {
        let spec = &SCENARIOS[1];
        let mut store = ReactiveStore::new();
        store.register_producer(workspace_readiness(workspace("ws-aureline-primary")));
        let sid = store
            .subscribe(
                "vfs.workspace_readiness",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Coalesced,
            )
            .unwrap();
        let warming = store
            .emit_snapshot(
                sid,
                Freshness::Warming,
                Completeness::Partial,
                partial_payload("roots indexed: 1/3", 1, 3),
                None,
            )
            .unwrap();
        // A second snapshot bumps the epoch and announces
        // full / authoritative coverage.
        let full = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                partial_payload("roots indexed: 3/3", 3, 3),
                None,
            )
            .unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "warming_snapshot",
                    envelope: warming.envelope,
                    note: "first snapshot; freshness=warming, completeness=partial".to_owned(),
                },
                FeaturedEnvelope {
                    label: "full_snapshot",
                    envelope: full.envelope,
                    note: "second snapshot; freshness=authoritative, completeness=full; consumer observes completeness change".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn file_identity_delta_gap_triggers_resync() -> ScenarioReport {
        let spec = &SCENARIOS[2];
        let mut store = ReactiveStore::new();
        store.register_producer(file_identity(workspace("ws-aureline-primary")));
        let sid = store
            .subscribe(
                "vfs.file_identity",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Realtime,
            )
            .unwrap();
        store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("10 canonical objects tracked", 10),
                None,
            )
            .unwrap();
        store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("+1 canonical object", 11),
                None,
            )
            .unwrap();
        store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("+1 canonical object", 12),
                None,
            )
            .unwrap();
        // Scripted out-of-order arrival: the "missing" delta
        // seq=3 never arrives on the wire; the consumer sees
        // seq=4 and reports a gap. The store escalates to
        // resync_required with causality_lost.
        let resync = store.report_delta_gap(sid, 4, 3).expect("report_delta_gap");
        let fresh = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("re-enumerated canonical objects; 12 total", 12),
                None,
            )
            .unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "resync_required",
                    envelope: resync.envelope,
                    note: "frame_class=resync_required; stale_reason=causality_lost; epoch 1 abandoned".to_owned(),
                },
                FeaturedEnvelope {
                    label: "fresh_snapshot",
                    envelope: fresh.envelope,
                    note: "fresh snapshot on snapshot_epoch=2; delta_seq resets to 0".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn derived_view_upstream_input_stale() -> ScenarioReport {
        let spec = &SCENARIOS[3];
        let mut store = ReactiveStore::new();
        let upstream_digest_initial =
            "sha256:aa99887766554433221100ffeeddccbbaa99887766554433221100ffeeddccbb";
        let upstream_digest_updated =
            "sha256:112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00";
        store.register_producer(derived_diagnostics(
            workspace("ws-aureline-primary"),
            upstream_digest_initial,
        ));
        let sid = store
            .subscribe(
                "language.diagnostics",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Realtime,
            )
            .unwrap();
        // First snapshot: derived producer is warming against
        // the upstream digest.
        let warm = store
            .emit_snapshot(
                sid,
                Freshness::Warming,
                Completeness::Partial,
                partial_payload("3 files parsed; 2 remaining", 3, 5),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        note: Some(
                            "first-pass parse in progress; coverage will widen on follow-up frames"
                                .to_owned(),
                        ),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();
        // Second snapshot: derived producer is done, but it
        // still never claims authoritative (derived frames
        // MAY NOT). Label it `cached` with a stale_reason of
        // `cache_served` while fresh results sit against the
        // current upstream digest.
        let cached = store
            .emit_snapshot(
                sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("5 files parsed; derivation current", 5),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        upstream_digest: Some(upstream_digest_initial.to_owned()),
                        note: Some("derivation current against upstream digest".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();
        // Upstream moves; derived producer must emit
        // resync_required with upstream_input_stale.
        let resync = store
            .emit_resync_required(
                sid,
                StaleReason::UpstreamInputStale,
                Some(CausedBy {
                    upstream_digest: Some(upstream_digest_updated.to_owned()),
                    trace_id: Some("0af7651916cd43dd8448eb211c80319c".to_owned()),
                    note: Some(
                        "upstream file_identity advanced; current diagnostics reflect a stale input"
                            .to_owned(),
                    ),
                    ..CausedBy::default()
                }),
                Completeness::Partial,
            )
            .unwrap();
        // Fresh snapshot re-declares the new upstream digest
        // on the producer_refs; derivation_epoch bumps.
        let new_refs = vec![ProducerRef {
            producer_id: "aureline.derived.diagnostics".to_owned(),
            producer_instance: synthetic_instance("aureline.derived.diagnostics", 2026041900, 5120),
            producer_version: Some("language-0.0.1-pre".to_owned()),
            input_digests: vec![InputDigest {
                name: "vfs.file_identity@upstream".to_owned(),
                digest: upstream_digest_updated.to_owned(),
            }],
            derivation_epoch: Some(2),
            source: None,
        }];
        let fresh = store
            .emit_snapshot_with_refs(
                sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("5 files re-parsed against updated upstream", 5),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        upstream_digest: Some(upstream_digest_updated.to_owned()),
                        note: Some("derivation re-run against updated upstream".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
                Some(new_refs),
            )
            .unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "warming_snapshot",
                    envelope: warm.envelope,
                    note: "first derived snapshot; freshness=warming, completeness=partial"
                        .to_owned(),
                },
                FeaturedEnvelope {
                    label: "cached_snapshot",
                    envelope: cached.envelope,
                    note: "derived producer complete but never authoritative; freshness=cached"
                        .to_owned(),
                },
                FeaturedEnvelope {
                    label: "resync_required",
                    envelope: resync.envelope,
                    note: "stale_reason=upstream_input_stale; upstream digest moved".to_owned(),
                },
                FeaturedEnvelope {
                    label: "post_refresh_snapshot",
                    envelope: fresh.envelope,
                    note: "fresh snapshot on new snapshot_epoch; derivation_epoch bumped; producer_refs.input_digests updated".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn refresh_ordering_authority_before_derived() -> ScenarioReport {
        let spec = &SCENARIOS[4];
        let mut store = ReactiveStore::new();
        let upstream_initial = "sha256:aa00bb11cc22dd33ee44ff55aa00bb11cc22dd33ee44ff55aa00bb11";
        let upstream_updated = "sha256:11aa22bb33cc44dd55ee66ff77aa88bb99cc00dd11ee22ff33aa44bb";

        store.register_producer(file_identity(workspace("ws-aureline-primary")));
        store.register_producer(derived_diagnostics(
            workspace("ws-aureline-primary"),
            upstream_initial,
        ));
        let authority_sid = store
            .subscribe(
                "vfs.file_identity",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Realtime,
            )
            .unwrap();
        let derived_sid = store
            .subscribe(
                "language.diagnostics",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Realtime,
            )
            .unwrap();

        // Initial fresh state on both.
        let authority_snap_v1 = store
            .emit_snapshot(
                authority_sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("10 canonical objects", 10),
                None,
            )
            .unwrap();
        let derived_snap_v1 = store
            .emit_snapshot(
                derived_sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("10 files parsed", 10),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        upstream_digest: Some(upstream_initial.to_owned()),
                        note: Some("derivation current against initial upstream".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();
        let authority_tick_v1 = authority_snap_v1.consumer_observation.snapshot_epoch;
        let _ = derived_snap_v1;

        // Authority publishes a new snapshot first.
        let authority_snap_v2 = store
            .emit_snapshot(
                authority_sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("11 canonical objects (added main.rs)", 11),
                None,
            )
            .unwrap();
        // Derived consumer observes upstream digest drift and
        // only then invalidates. The tick-order below is what
        // the scenario proves: authority precedes derived.
        let derived_resync = store
            .emit_resync_required(
                derived_sid,
                StaleReason::UpstreamInputStale,
                Some(CausedBy {
                    upstream_digest: Some(upstream_updated.to_owned()),
                    note: Some(
                        "observed upstream input digest drift after authority snapshot".to_owned(),
                    ),
                    ..CausedBy::default()
                }),
                Completeness::Partial,
            )
            .unwrap();
        let new_refs = vec![ProducerRef {
            producer_id: "aureline.derived.diagnostics".to_owned(),
            producer_instance: synthetic_instance("aureline.derived.diagnostics", 2026041900, 5120),
            producer_version: Some("language-0.0.1-pre".to_owned()),
            input_digests: vec![InputDigest {
                name: "vfs.file_identity@upstream".to_owned(),
                digest: upstream_updated.to_owned(),
            }],
            derivation_epoch: Some(2),
            source: None,
        }];
        let derived_snap_v2 = store
            .emit_snapshot_with_refs(
                derived_sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("11 files re-parsed", 11),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        upstream_digest: Some(upstream_updated.to_owned()),
                        note: Some("derivation current against updated upstream".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
                Some(new_refs),
            )
            .unwrap();

        let featured = vec![
            FeaturedEnvelope {
                label: "authority_snapshot_v2",
                envelope: authority_snap_v2.envelope,
                note: format!(
                    "authoritative snapshot at snapshot_epoch=2; precedes any derived refresh (authority_tick_v1={authority_tick_v1})"
                ),
            },
            FeaturedEnvelope {
                label: "derived_resync_required",
                envelope: derived_resync.envelope,
                note: "stale_reason=upstream_input_stale observed after authority advanced"
                    .to_owned(),
            },
            FeaturedEnvelope {
                label: "derived_snapshot_v2",
                envelope: derived_snap_v2.envelope,
                note: "derived view re-publishes on bumped snapshot_epoch and derivation_epoch"
                    .to_owned(),
            },
        ];

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: featured,
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn replay_does_not_advance_live_epoch() -> ScenarioReport {
        let spec = &SCENARIOS[5];
        let mut store = ReactiveStore::new();
        store.register_producer(graph_neighborhood(
            workspace("ws-aureline-primary"),
            "sha256:aa00",
        ));
        let sid = store
            .subscribe(
                "graph.neighborhood",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Coalesced,
            )
            .unwrap();
        let live_snap = store
            .emit_snapshot(
                sid,
                Freshness::Warming,
                Completeness::Partial,
                partial_payload("nodes warming: 40/120", 40, 120),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        note: Some("graph warm-up".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();
        // Replay two deltas from a captured bundle. Neither
        // advances the live epoch or delta_seq.
        store.begin_replay(sid).unwrap();
        let replay_one = store
            .emit_replayed_delta(
                sid,
                1,
                base_payload("replay: comment anchored on line 124", 0),
                "replay-bundle:support-9f2c/2026-04-18T22:11:03Z".to_owned(),
            )
            .unwrap();
        let replay_two = store
            .emit_replayed_delta(
                sid,
                2,
                base_payload("replay: comment anchored on line 301", 0),
                "replay-bundle:support-9f2c/2026-04-18T22:11:03Z".to_owned(),
            )
            .unwrap();
        store.end_replay(sid).unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "live_snapshot",
                    envelope: live_snap.envelope,
                    note: "live graph subscription at snapshot_epoch=1".to_owned(),
                },
                FeaturedEnvelope {
                    label: "replayed_delta_1",
                    envelope: replay_one.envelope,
                    note: "freshness=replayed; does not advance live delta_seq".to_owned(),
                },
                FeaturedEnvelope {
                    label: "replayed_delta_2",
                    envelope: replay_two.envelope,
                    note: "freshness=replayed; live epoch stays at 1".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn imported_snapshot_attached_read_only() -> ScenarioReport {
        let spec = &SCENARIOS[6];
        let mut store = ReactiveStore::new();
        store.register_producer(graph_neighborhood(
            workspace("ws-aureline-primary"),
            "sha256:bb11",
        ));
        let sid = store
            .subscribe(
                "graph.neighborhood",
                &workspace("ws-aureline-primary"),
                BackpressureMode::SnapshotRequired,
            )
            .unwrap();
        let live_snap = store
            .emit_snapshot(
                sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("live graph nodes: 120", 120),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: None,
                }),
            )
            .unwrap();
        let imported = store
            .attach_imported_snapshot(
                sid,
                base_payload(
                    "imported lsif: 63 nodes anchored on aureline_rpc::EventEnvelope",
                    63,
                ),
                "imported-lsif:ws-aureline-primary/2026-04-18.lsif".to_owned(),
            )
            .unwrap();

        // Contract assertion: attempting to promote the imported
        // frame to authoritative is a store error. The scenario
        // records the refusal on the trace as a reviewer note
        // so the artifact captures the contract.
        let promote_attempt = store.emit_snapshot(
            sid,
            Freshness::Authoritative,
            Completeness::Full,
            base_payload("illegal promotion attempt", 0),
            None,
        );
        // The store call above should have returned
        // DerivedCannotClaimAuthoritative; assert that so the
        // scenario panics if the contract ever regresses.
        let promote_err = match &promote_attempt {
            Ok(_) => "unexpected: promotion succeeded".to_owned(),
            Err(e) => format!("promotion refused: {}", e.as_str()),
        };
        assert_eq!(
            promote_attempt.as_ref().err(),
            Some(&StoreError::DerivedCannotClaimAuthoritative),
            "derived frames must not be promotable to authoritative"
        );
        let tick = store.trace.last().map(|e| e.tick()).unwrap_or(0) + 1;
        store.trace.push(TraceEvent::note(
            tick,
            "store_contract_refusal",
            format!("derived frames may not claim freshness=authoritative; {promote_err}"),
        ));

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "live_cached_snapshot",
                    envelope: live_snap.envelope,
                    note: "live derived graph subscription; freshness=cached".to_owned(),
                },
                FeaturedEnvelope {
                    label: "imported_snapshot",
                    envelope: imported.envelope,
                    note: "freshness=imported; stale_reason=imported_from_external; never merged into authoritative state".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn provider_terminal_unavailable() -> ScenarioReport {
        let spec = &SCENARIOS[7];
        let mut store = ReactiveStore::new();
        store.register_producer(provider_overlay(workspace("review-pr-1342")));
        let sid = store
            .subscribe(
                "provider.ci_checks",
                &workspace("review-pr-1342"),
                BackpressureMode::Coalesced,
            )
            .unwrap();
        let snap = store
            .emit_snapshot(
                sid,
                Freshness::Cached,
                Completeness::Full,
                base_payload("2 running, 3 queued CI checks", 5),
                Some(Invalidation {
                    stale_reason: StaleReason::CacheServed,
                    caused_by: Some(CausedBy {
                        note: Some("provider last polled successfully".to_owned()),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();
        let terminal = store
            .emit_terminal(
                sid,
                TerminalReason::Unavailable,
                Freshness::Stale,
                Completeness::Unavailable,
                Some(Invalidation {
                    stale_reason: StaleReason::WatcherDropped,
                    caused_by: Some(CausedBy {
                        trace_id: Some("c9f1a2b3d4e5f60718293a4b5c6d7e8f".to_owned()),
                        note: Some(
                            "provider webhook timed out and long-poll watcher closed without a reconnect grant"
                                .to_owned(),
                        ),
                        ..CausedBy::default()
                    }),
                }),
            )
            .unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "last_live_snapshot",
                    envelope: snap.envelope,
                    note: "last snapshot consumer saw before the provider went unavailable"
                        .to_owned(),
                },
                FeaturedEnvelope {
                    label: "terminal_frame",
                    envelope: terminal.envelope,
                    note: "terminal_reason=unavailable; freshness=stale; consumer preserves last-known projection and waits for repair".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }

    pub(super) fn backpressure_snapshot_required_switch() -> ScenarioReport {
        let spec = &SCENARIOS[8];
        let mut store = ReactiveStore::new();
        store.register_producer(workspace_readiness(workspace("ws-aureline-primary")));
        let sid = store
            .subscribe(
                "vfs.workspace_readiness",
                &workspace("ws-aureline-primary"),
                BackpressureMode::Coalesced,
            )
            .unwrap();
        store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("3 roots ready", 3),
                None,
            )
            .unwrap();
        store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("+1 root", 4),
                None,
            )
            .unwrap();
        // Producer coalesces two pending deltas rather than
        // emit them individually. The coalesce hook is fired
        // with count=2; the consumer sees one coalesced frame.
        store.record_coalesce(2);
        let coalesced = store
            .emit_delta(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("+2 roots (coalesced)", 6),
                None,
            )
            .unwrap();
        // Consumer switches to snapshot_required; the store
        // emits a resync_required and awaits a fresh snapshot.
        let resync = store.request_snapshot_required_switch(sid).unwrap();
        let fresh = store
            .emit_snapshot(
                sid,
                Freshness::Authoritative,
                Completeness::Full,
                base_payload("6 roots ready (post-resync)", 6),
                None,
            )
            .unwrap();

        ScenarioReport {
            label: spec.label,
            scenario_summary: spec.scenario_summary,
            primary_hooks: spec.primary_hooks,
            adr_sections: spec.adr_sections,
            featured_envelopes: vec![
                FeaturedEnvelope {
                    label: "coalesced_delta",
                    envelope: coalesced.envelope,
                    note: "backpressure_mode=coalesced; producer coalesced 2 pending deltas into one".to_owned(),
                },
                FeaturedEnvelope {
                    label: "resync_required",
                    envelope: resync.envelope,
                    note: "consumer switched to snapshot_required; causal continuity abandoned; stale_reason=causality_lost".to_owned(),
                },
                FeaturedEnvelope {
                    label: "post_switch_snapshot",
                    envelope: fresh.envelope,
                    note: "fresh snapshot on bumped snapshot_epoch; backpressure now snapshot_required".to_owned(),
                },
            ],
            final_consumer_notes: final_consumer_notes(&store),
            tick_count: store.trace.len() as u64,
            hook_counters: store.hooks.clone(),
            trace: store.trace,
        }
    }
}

// -------------------------------------------------------------------------
// JSON emission for harness reports.
// -------------------------------------------------------------------------

/// Render a single scenario report as a byte-stable JSON blob
/// suitable for committing under
/// `artifacts/state/invalidation_trace_examples/<label>.json`.
pub fn scenario_to_json(report: &ScenarioReport) -> String {
    let mut out = String::new();
    out.push('{');
    let ind = 1;
    newline_indent(&mut out, ind);
    write_kv_string(&mut out, "label", report.label);
    comma_nl(&mut out, ind);
    write_kv_string(&mut out, "scenario_summary", report.scenario_summary);
    comma_nl(&mut out, ind);
    write_key(&mut out, "primary_hooks");
    write_string_array(&mut out, report.primary_hooks, ind);
    comma_nl(&mut out, ind);
    write_key(&mut out, "adr_sections");
    write_string_array(&mut out, report.adr_sections, ind);
    comma_nl(&mut out, ind);
    write_kv_u64(&mut out, "tick_count", report.tick_count);
    comma_nl(&mut out, ind);
    write_key(&mut out, "hook_counters");
    write_hook_counters(&mut out, &report.hook_counters, ind);
    comma_nl(&mut out, ind);
    write_key(&mut out, "featured_envelopes");
    write_featured_envelopes(&mut out, &report.featured_envelopes, ind);
    comma_nl(&mut out, ind);
    write_key(&mut out, "final_consumer_notes");
    write_owned_string_array(&mut out, &report.final_consumer_notes, ind);
    comma_nl(&mut out, ind);
    write_key(&mut out, "trace");
    out.push('[');
    for (i, ev) in report.trace.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(&mut out, ind + 1);
        write_trace_event(&mut out, ev, ind + 1);
    }
    if !report.trace.is_empty() {
        newline_indent(&mut out, ind);
    }
    out.push(']');
    newline_indent(&mut out, 0);
    out.push('}');
    out.push('\n');
    out
}

/// Render the aggregate harness report.
pub fn report_to_json(report: &HarnessReport) -> String {
    let mut out = String::new();
    out.push('{');
    let ind = 1;
    newline_indent(&mut out, ind);
    write_kv_string(&mut out, "corpus_id", report.corpus_id);
    comma_nl(&mut out, ind);
    write_kv_u64(&mut out, "schema_version", report.schema_version as u64);
    comma_nl(&mut out, ind);
    write_key(&mut out, "aggregate");
    write_aggregate(&mut out, &report.aggregate, ind);
    comma_nl(&mut out, ind);
    write_key(&mut out, "scenarios");
    out.push('[');
    for (i, sc) in report.scenarios.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(&mut out, ind + 1);
        // Inline each scenario as an object.
        write_scenario_inline(&mut out, sc, ind + 1);
    }
    if !report.scenarios.is_empty() {
        newline_indent(&mut out, ind);
    }
    out.push(']');
    newline_indent(&mut out, 0);
    out.push('}');
    out.push('\n');
    out
}

fn write_scenario_inline(out: &mut String, sc: &ScenarioReport, indent: usize) {
    // Same shape as scenario_to_json but inlined at the given
    // indent without a trailing newline.
    out.push('{');
    let ind = indent + 1;
    newline_indent(out, ind);
    write_kv_string(out, "label", sc.label);
    comma_nl(out, ind);
    write_kv_string(out, "scenario_summary", sc.scenario_summary);
    comma_nl(out, ind);
    write_key(out, "primary_hooks");
    write_string_array(out, sc.primary_hooks, ind);
    comma_nl(out, ind);
    write_key(out, "adr_sections");
    write_string_array(out, sc.adr_sections, ind);
    comma_nl(out, ind);
    write_kv_u64(out, "tick_count", sc.tick_count);
    comma_nl(out, ind);
    write_key(out, "hook_counters");
    write_hook_counters(out, &sc.hook_counters, ind);
    comma_nl(out, ind);
    write_key(out, "featured_envelopes");
    write_featured_envelopes(out, &sc.featured_envelopes, ind);
    comma_nl(out, ind);
    write_key(out, "final_consumer_notes");
    write_owned_string_array(out, &sc.final_consumer_notes, ind);
    comma_nl(out, ind);
    write_key(out, "trace");
    out.push('[');
    for (i, ev) in sc.trace.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, ind + 1);
        write_trace_event(out, ev, ind + 1);
    }
    if !sc.trace.is_empty() {
        newline_indent(out, ind);
    }
    out.push(']');
    newline_indent(out, indent);
    out.push('}');
}

fn write_aggregate(out: &mut String, agg: &AggregateReport, indent: usize) {
    out.push('{');
    let ind = indent + 1;
    newline_indent(out, ind);
    write_kv_u64(out, "total_scenarios", agg.total_scenarios);
    comma_nl(out, ind);
    write_kv_u64(out, "total_snapshot_frames", agg.total_snapshot_frames);
    comma_nl(out, ind);
    write_kv_u64(out, "total_delta_frames", agg.total_delta_frames);
    comma_nl(out, ind);
    write_kv_u64(
        out,
        "total_resync_required_frames",
        agg.total_resync_required_frames,
    );
    comma_nl(out, ind);
    write_kv_u64(out, "total_terminal_frames", agg.total_terminal_frames);
    comma_nl(out, ind);
    write_kv_u64(
        out,
        "total_freshness_downgrades",
        agg.total_freshness_downgrades,
    );
    comma_nl(out, ind);
    write_kv_u64(
        out,
        "total_completeness_changes",
        agg.total_completeness_changes,
    );
    comma_nl(out, ind);
    write_kv_u64(out, "total_imported_attaches", agg.total_imported_attaches);
    comma_nl(out, ind);
    write_kv_u64(out, "total_replay_sessions", agg.total_replay_sessions);
    comma_nl(out, ind);
    write_kv_u64(
        out,
        "total_backpressure_switches",
        agg.total_backpressure_switches,
    );
    newline_indent(out, indent);
    out.push('}');
}

fn write_hook_counters(out: &mut String, counters: &HookCounters, indent: usize) {
    out.push('[');
    for (i, (id, protected, count)) in counters.entries().iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, indent + 1);
        out.push('{');
        newline_indent(out, indent + 2);
        write_kv_string(out, "hook_id", id);
        comma_nl(out, indent + 2);
        let _ = write!(out, "\"protected_hot_path\": {protected}");
        comma_nl(out, indent + 2);
        write_kv_u64(out, "count", *count);
        newline_indent(out, indent + 1);
        out.push('}');
    }
    newline_indent(out, indent);
    out.push(']');
}

fn write_featured_envelopes(out: &mut String, items: &[FeaturedEnvelope], indent: usize) {
    out.push('[');
    for (i, fe) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, indent + 1);
        out.push('{');
        newline_indent(out, indent + 2);
        write_kv_string(out, "label", fe.label);
        comma_nl(out, indent + 2);
        write_kv_string(out, "note", &fe.note);
        comma_nl(out, indent + 2);
        write_key(out, "envelope");
        crate::envelope::write_envelope(out, &fe.envelope, indent + 2);
        newline_indent(out, indent + 1);
        out.push('}');
    }
    if !items.is_empty() {
        newline_indent(out, indent);
    }
    out.push(']');
}

fn write_string_array(out: &mut String, items: &[&str], indent: usize) {
    out.push('[');
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, indent + 1);
        write_string_literal(out, s);
    }
    if !items.is_empty() {
        newline_indent(out, indent);
    }
    out.push(']');
}

fn write_owned_string_array(out: &mut String, items: &[String], indent: usize) {
    out.push('[');
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline_indent(out, indent + 1);
        write_string_literal(out, s);
    }
    if !items.is_empty() {
        newline_indent(out, indent);
    }
    out.push(']');
}

// -------------------------------------------------------------------------
// Tests.
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::DerivationClass;

    #[test]
    fn every_scenario_label_is_unique() {
        let mut labels: Vec<&str> = SCENARIOS.iter().map(|s| s.label).collect();
        labels.sort();
        let mut dedup = labels.clone();
        dedup.dedup();
        assert_eq!(labels, dedup);
    }

    #[test]
    fn scenarios_cover_acceptance_surface() {
        // The harness MUST cover nominal, warming/partial,
        // out-of-order, upstream-stale, replay, imported,
        // terminal, backpressure, and refresh-ordering.
        let labels: Vec<&str> = SCENARIOS.iter().map(|s| s.label).collect();
        assert!(labels.contains(&"shell_health_nominal"));
        assert!(labels.contains(&"workspace_readiness_warming_then_full"));
        assert!(labels.contains(&"file_identity_delta_gap_triggers_resync"));
        assert!(labels.contains(&"derived_view_upstream_input_stale"));
        assert!(labels.contains(&"refresh_ordering_authority_before_derived"));
        assert!(labels.contains(&"replay_does_not_advance_live_epoch"));
        assert!(labels.contains(&"imported_snapshot_attached_read_only"));
        assert!(labels.contains(&"provider_terminal_unavailable"));
        assert!(labels.contains(&"backpressure_snapshot_required_switch"));
    }

    #[test]
    fn harness_is_byte_stable() {
        let first = run_harness();
        let second = run_harness();
        assert_eq!(report_to_json(&first), report_to_json(&second));
    }

    #[test]
    fn authority_precedes_derived_in_refresh_ordering_scenario() {
        let report = scenarios::refresh_ordering_authority_before_derived();
        // Locate the tick at which the authority emitted its
        // second (fresh) snapshot.
        let mut authority_refresh_tick: Option<u64> = None;
        let mut derived_resync_tick: Option<u64> = None;
        for ev in &report.trace {
            if let TraceEvent::FrameEmit { tick, envelope, .. } = ev {
                if envelope.query_family == "vfs.file_identity"
                    && envelope.snapshot_epoch == 2
                    && envelope.frame_class.as_str() == "snapshot"
                    && authority_refresh_tick.is_none()
                {
                    authority_refresh_tick = Some(*tick);
                }
                if envelope.query_family == "language.diagnostics"
                    && envelope.frame_class.as_str() == "resync_required"
                    && derived_resync_tick.is_none()
                {
                    derived_resync_tick = Some(*tick);
                }
            }
        }
        let authority = authority_refresh_tick.expect("authority snapshot v2");
        let derived = derived_resync_tick.expect("derived resync");
        assert!(
            authority < derived,
            "authority fresh snapshot must precede derived resync"
        );
    }

    #[test]
    fn derived_producers_never_claim_authoritative() {
        let report = scenarios::derived_view_upstream_input_stale();
        for ev in &report.trace {
            if let TraceEvent::FrameEmit { envelope, .. } = ev {
                if matches!(envelope.derivation_class, DerivationClass::Derived) {
                    assert!(
                        !matches!(envelope.freshness, Freshness::Authoritative),
                        "derived frame must not claim freshness=authoritative"
                    );
                }
            }
        }
    }

    #[test]
    fn replay_frames_never_advance_live_epoch() {
        let report = scenarios::replay_does_not_advance_live_epoch();
        let mut live_epoch_high_water: u64 = 0;
        for ev in &report.trace {
            if let TraceEvent::FrameEmit { envelope, .. } = ev {
                if matches!(envelope.freshness, Freshness::Replayed) {
                    // Replay frame: must not exceed the
                    // highest live epoch seen so far.
                    assert!(
                        envelope.snapshot_epoch <= live_epoch_high_water,
                        "replay must not advance live epoch"
                    );
                } else {
                    live_epoch_high_water = live_epoch_high_water.max(envelope.snapshot_epoch);
                }
            }
        }
    }
}
