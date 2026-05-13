use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_terminal::{
    evaluate_clipboard_write, evaluate_escape_control, evaluate_paste_review,
    restore_conformance_from_header, restore_session_as_transcript, HostClass, OpenSessionRequest,
    PtyHost, ScrollbackRedactionClass, TerminalClipboardSuppressionClass,
    TerminalClipboardWriteInput, TerminalClipboardWriteKind, TerminalClipboardWriteReport,
    TerminalEscapeControlInput, TerminalEscapeControlReport, TerminalHeaderRecord,
    TerminalPasteReviewInput, TerminalPasteReviewReport, TerminalRestoreConformanceReport,
    TerminalScrollback, TerminalTrustState, TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS,
    TERMINAL_PROTOCOL_CORPUS_CASE_KIND, TERMINAL_PROTOCOL_CORPUS_FIXTURE_SET_ID,
    TERMINAL_PROTOCOL_CORPUS_MANIFEST_KIND, TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION,
};

#[derive(Debug, Deserialize)]
struct CorpusManifest {
    record_kind: String,
    schema_version: u32,
    fixture_set_id: String,
    #[allow(dead_code)]
    sources: BTreeMap<String, String>,
    required_coverage: RequiredCoverage,
    cases: Vec<ManifestCase>,
}

#[derive(Debug, Deserialize)]
struct RequiredCoverage {
    escape_sequence_tokens: Vec<String>,
    paste_edge_tokens: Vec<String>,
    clipboard_edge_tokens: Vec<String>,
    restore_state_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ManifestCase {
    case_id: String,
    case_kind: aureline_terminal::TerminalProtocolCorpusCaseKind,
    path: String,
}

#[derive(Debug, Deserialize)]
struct CorpusCase {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    case_kind: aureline_terminal::TerminalProtocolCorpusCaseKind,
    protected_launch_wedge: bool,
    #[allow(dead_code)]
    scenario: String,
    coverage_tags: Vec<String>,
    input: CorpusInput,
    expect: CorpusExpect,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "input_kind", rename_all = "snake_case")]
enum CorpusInput {
    EscapeControl {
        #[serde(flatten)]
        input: TerminalEscapeControlInput,
    },
    PasteReview {
        #[serde(flatten)]
        input: TerminalPasteReviewInput,
    },
    ClipboardWrite {
        #[serde(flatten)]
        input: TerminalClipboardWriteInput,
    },
    RestoreConformance {
        driver: RestoreDriver,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "expect_kind", rename_all = "snake_case")]
enum CorpusExpect {
    EscapeControl {
        #[serde(flatten)]
        report: TerminalEscapeControlReport,
    },
    PasteReview {
        #[serde(flatten)]
        report: TerminalPasteReviewReport,
    },
    ClipboardWrite {
        #[serde(flatten)]
        report: TerminalClipboardWriteReport,
    },
    RestoreConformance {
        #[serde(flatten)]
        report: TerminalRestoreConformanceReport,
    },
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RestoreDriver {
    EndedSession,
    ReconnectRequired,
    RestoredTranscript,
}

#[derive(Default)]
struct ObservedCoverage {
    escape_sequence_tokens: BTreeSet<String>,
    paste_edge_tokens: BTreeSet<String>,
    clipboard_edge_tokens: BTreeSet<String>,
    restore_state_tokens: BTreeSet<String>,
    protected_high_risk_paste_review: bool,
    protected_clipboard_gate_audit: bool,
}

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/terminal/protocol_corpus_alpha")
}

fn load_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let payload = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn open_restore_session(host: &mut PtyHost, host_class: HostClass) -> aureline_terminal::PtySessionId {
    host.open_session(OpenSessionRequest {
        workspace_id: "ws-alpha",
        host_class,
        display_title: match host_class {
            HostClass::HostDesktop => "zsh",
            HostClass::RemoteAgentPrimary => "remote shell",
            HostClass::LocalContainer => "container shell",
        },
        cwd_hint: Some(match host_class {
            HostClass::HostDesktop => "/workspace",
            HostClass::RemoteAgentPrimary => "/srv/app",
            HostClass::LocalContainer => "/workspaces/app",
        }),
        execution_context_ref: match host_class {
            HostClass::HostDesktop => "exec:ws-alpha:local-terminal",
            HostClass::RemoteAgentPrimary => "exec:ws-alpha:remote-terminal",
            HostClass::LocalContainer => "exec:ws-alpha:container-terminal",
        },
        trust_state: TerminalTrustState::Trusted,
        observed_at: "mono:0",
    })
}

fn header_for_restore_driver(driver: RestoreDriver) -> TerminalHeaderRecord {
    match driver {
        RestoreDriver::EndedSession => {
            let mut host = PtyHost::new();
            let id = open_restore_session(&mut host, HostClass::HostDesktop);
            host.close(&id, "mono:1", Some("user_closed"))
                .expect("session closes");
            let prior = host.session(&id).expect("prior session exists");
            let restored = restore_session_as_transcript(prior, None, "mono:restart");
            TerminalHeaderRecord::project_restored(&restored)
        }
        RestoreDriver::ReconnectRequired => {
            let mut host = PtyHost::new();
            let id = open_restore_session(&mut host, HostClass::RemoteAgentPrimary);
            host.mark_lost_transport(&id, "mono:1", Some("network_drop"))
                .expect("transport can drop");
            let session = host.session(&id).expect("session exists");
            TerminalHeaderRecord::project_session(session)
        }
        RestoreDriver::RestoredTranscript => {
            let mut host = PtyHost::new();
            let id = open_restore_session(&mut host, HostClass::HostDesktop);
            host.close(&id, "mono:1", Some("user_closed"))
                .expect("session closes");
            let mut scrollback = TerminalScrollback::new(id.clone());
            scrollback.record_line(
                "$ cargo test",
                ScrollbackRedactionClass::SupportBundleScoped,
                "mono:0",
            );
            let prior = host.session(&id).expect("prior session exists");
            let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");
            TerminalHeaderRecord::project_restored(&restored)
        }
    }
}

fn assert_case_matches(
    path: &Path,
    case: &CorpusCase,
    manifest_case: &ManifestCase,
    observed: &mut ObservedCoverage,
) {
    assert_eq!(case.record_kind, TERMINAL_PROTOCOL_CORPUS_CASE_KIND);
    assert_eq!(case.schema_version, TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION);
    assert_eq!(case.case_id, manifest_case.case_id);
    assert_eq!(case.case_kind, manifest_case.case_kind);

    match (&case.input, &case.expect) {
        (CorpusInput::EscapeControl { input }, CorpusExpect::EscapeControl { report }) => {
            assert_eq!(case.case_kind.as_str(), "escape_control");
            let actual = evaluate_escape_control(input);
            assert_eq!(&actual, report, "escape report mismatch in {path:?}");
            observed
                .escape_sequence_tokens
                .extend(input.covered_sequence_tokens.iter().cloned());
        }
        (CorpusInput::PasteReview { input }, CorpusExpect::PasteReview { report }) => {
            assert_eq!(case.case_kind.as_str(), "paste_review");
            let actual = evaluate_paste_review(input);
            assert_eq!(&actual, report, "paste report mismatch in {path:?}");
            if input.line_count > 1 {
                observed.paste_edge_tokens.insert("multiline_paste".to_owned());
            }
            if input.bracketed_paste_available {
                observed.paste_edge_tokens.insert("bracketed_paste".to_owned());
            }
            if input.remote_clipboard_bridge {
                observed
                    .paste_edge_tokens
                    .insert("remote_clipboard_bridge".to_owned());
            }
            if input.production_labeled_target {
                observed
                    .paste_edge_tokens
                    .insert("production_labeled_target".to_owned());
            }
            if actual.policy_result_visible {
                observed
                    .paste_edge_tokens
                    .insert("policy_result_before_commit".to_owned());
            }
            if actual.line_count_visible {
                observed
                    .paste_edge_tokens
                    .insert("line_count_before_commit".to_owned());
            }
            if actual.auto_submit_forbidden {
                observed.paste_edge_tokens.insert("no_auto_submit".to_owned());
            }
            observed.protected_high_risk_paste_review |= case.protected_launch_wedge
                && actual.high_risk
                && actual.target_boundary_visible
                && actual.policy_result_visible
                && actual.line_count_visible
                && actual.review_required_before_commit
                && actual.auto_submit_forbidden
                && actual.commit_without_review_forbidden;
        }
        (CorpusInput::ClipboardWrite { input }, CorpusExpect::ClipboardWrite { report }) => {
            assert_eq!(case.case_kind.as_str(), "clipboard_write");
            let actual = evaluate_clipboard_write(input);
            assert_eq!(&actual, report, "clipboard report mismatch in {path:?}");
            match input.write_kind {
                TerminalClipboardWriteKind::Osc52Write => {
                    observed.clipboard_edge_tokens.insert("osc52_write".to_owned());
                }
                TerminalClipboardWriteKind::RemoteClipboardBridgeWrite => {
                    observed
                        .clipboard_edge_tokens
                        .insert("remote_clipboard_bridge_write".to_owned());
                }
            }
            if actual.suppression_class == TerminalClipboardSuppressionClass::SuppressedByPolicy {
                observed
                    .clipboard_edge_tokens
                    .insert("policy_blocked_write".to_owned());
            }
            if actual.admin_gate_enforced {
                observed.clipboard_edge_tokens.insert("admin_gate".to_owned());
            }
            if actual.trust_gate_enforced {
                observed.clipboard_edge_tokens.insert("trust_gate".to_owned());
            }
            if actual.audit_safe_label_present {
                observed
                    .clipboard_edge_tokens
                    .insert("audit_safe_label".to_owned());
            }
            observed.protected_clipboard_gate_audit |= case.protected_launch_wedge
                && actual.write_blocked
                && actual.admin_gate_enforced
                && actual.trust_gate_enforced
                && actual.audit_safe_label_present
                && actual.raw_payload_excluded
                && actual.boundary_label_visible;
        }
        (
            CorpusInput::RestoreConformance { driver },
            CorpusExpect::RestoreConformance { report },
        ) => {
            assert_eq!(case.case_kind.as_str(), "restore_conformance");
            let header = header_for_restore_driver(*driver);
            let actual = restore_conformance_from_header(&header);
            assert_eq!(&actual, report, "restore report mismatch in {path:?}");
            observed
                .restore_state_tokens
                .insert(actual.state_token.to_owned());
        }
        _ => panic!("input and expectation kinds differ in {path:?}"),
    }

    for tag in &case.coverage_tags {
        match case.case_kind {
            aureline_terminal::TerminalProtocolCorpusCaseKind::PasteReview => {
                observed.paste_edge_tokens.insert(tag.clone());
            }
            aureline_terminal::TerminalProtocolCorpusCaseKind::ClipboardWrite => {
                observed.clipboard_edge_tokens.insert(tag.clone());
            }
            aureline_terminal::TerminalProtocolCorpusCaseKind::RestoreConformance => {
                observed.restore_state_tokens.insert(tag.clone());
            }
            aureline_terminal::TerminalProtocolCorpusCaseKind::EscapeControl => {}
        }
    }
}

#[test]
fn protocol_corpus_alpha_fixtures_drive_restore_conformance() {
    let dir = corpus_dir();
    let manifest_path = dir.join("manifest.json");
    let manifest: CorpusManifest = load_json(&manifest_path);
    assert_eq!(manifest.record_kind, TERMINAL_PROTOCOL_CORPUS_MANIFEST_KIND);
    assert_eq!(
        manifest.schema_version,
        TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION
    );
    assert_eq!(
        manifest.fixture_set_id,
        TERMINAL_PROTOCOL_CORPUS_FIXTURE_SET_ID
    );
    assert_eq!(
        manifest.required_coverage.escape_sequence_tokens,
        TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS
            .iter()
            .map(|token| token.to_string())
            .collect::<Vec<_>>()
    );

    let cases_dir = dir.join("cases");
    let disk_cases: BTreeSet<PathBuf> = std::fs::read_dir(&cases_dir)
        .unwrap_or_else(|err| panic!("cases dir must exist at {cases_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    let manifest_cases: BTreeMap<PathBuf, &ManifestCase> = manifest
        .cases
        .iter()
        .map(|case| (dir.join(&case.path), case))
        .collect();
    assert_eq!(
        disk_cases,
        manifest_cases.keys().cloned().collect::<BTreeSet<_>>(),
        "manifest must list exactly the case files on disk"
    );

    let mut observed = ObservedCoverage::default();
    for (path, manifest_case) in manifest_cases {
        let case: CorpusCase = load_json(&path);
        assert_case_matches(&path, &case, manifest_case, &mut observed);
    }

    for token in &manifest.required_coverage.escape_sequence_tokens {
        assert!(
            observed.escape_sequence_tokens.contains(token),
            "missing escape/control coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.paste_edge_tokens {
        assert!(
            observed.paste_edge_tokens.contains(token),
            "missing paste edge coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.clipboard_edge_tokens {
        assert!(
            observed.clipboard_edge_tokens.contains(token),
            "missing clipboard edge coverage token {token}"
        );
    }
    for token in &manifest.required_coverage.restore_state_tokens {
        assert!(
            observed.restore_state_tokens.contains(token),
            "missing restore state coverage token {token}"
        );
    }
    assert!(
        observed.protected_high_risk_paste_review,
        "expected a protected high-risk paste case with boundary, policy, line count, and no-auto-submit"
    );
    assert!(
        observed.protected_clipboard_gate_audit,
        "expected a protected clipboard-write case with admin/trust gating and audit-safe labeling"
    );
}
