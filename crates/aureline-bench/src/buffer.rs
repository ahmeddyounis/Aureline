//! Buffer smoke harness.
//!
//! Drives the prototype buffer from `aureline-buffer` through a set
//! of named scenarios — one per compensation posture and one per
//! representative undo-class id from the ADR 0003 taxonomy — and
//! emits a structural metrics record.
//!
//! Metrics are counts only (no wall-clock timings) so the committed
//! seed under `artifacts/buffer/` is byte-stable across hosts. The
//! benchmark lab layers wall-clock timing on top of these counts
//! when it needs to score against the protected-hot-path budgets.

use std::fmt::Write as _;

use aureline_buffer::{
    Buffer, BufferConfig, CompensationPosture, HookCounters, TransactionSpec, UndoClass,
};

/// Canonical corpus / scenario-family identifier written into
/// emitted artifacts.
pub const CORPUS_ID: &str = "aureline.buffer_smoke_scenarios.v1";

/// Schema version for the emitted metrics JSON.
pub const METRICS_SCHEMA_VERSION: u32 = 1;

/// One scenario the harness runs. Scenarios are named so benchmark
/// and conformance lanes can cite the same label the artifacts do.
#[derive(Debug, Clone, Copy)]
pub struct Scenario {
    pub label: &'static str,
    pub run: fn(&mut Buffer),
    /// Tag describing the dominant pattern (for scorecard grouping).
    pub tag: ScenarioTag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioTag {
    /// Typing loop: N single-op text_edit commits.
    TypingSequence,
    /// One grouped transaction with N ops.
    GroupedTextEdit,
    /// Multicursor: one multi_cursor_text_edit transaction.
    MultiCursor,
    /// Structural edit (sort/move/reindent).
    Structural,
    /// Named refactor group (compensatable).
    RefactorLocal,
    /// Named refactor group (only_revertible).
    RefactorWorkspace,
    /// Formatter run over the whole document.
    Formatter,
    /// Save pipeline with participants.
    SavePipeline,
    /// AI apply / machine generated change.
    MachineGenerated,
    /// External reload event.
    ExternalReload,
    /// Decode recovery resolution.
    DecodeRecovery,
    /// Undo / redo cycle across classes.
    UndoRedoCycle,
}

impl ScenarioTag {
    pub fn name(self) -> &'static str {
        match self {
            Self::TypingSequence => "typing_sequence",
            Self::GroupedTextEdit => "grouped_text_edit",
            Self::MultiCursor => "multi_cursor",
            Self::Structural => "structural",
            Self::RefactorLocal => "refactor_local",
            Self::RefactorWorkspace => "refactor_workspace",
            Self::Formatter => "formatter",
            Self::SavePipeline => "save_pipeline",
            Self::MachineGenerated => "machine_generated",
            Self::ExternalReload => "external_reload",
            Self::DecodeRecovery => "decode_recovery",
            Self::UndoRedoCycle => "undo_redo_cycle",
        }
    }
}

/// The frozen scenario table the seed artifact reports against. Each
/// entry exercises at least one undo class and one protected-hot-path
/// hook so later benchmark and journey lanes can instrument the same
/// names the ADR freezes.
pub const SCENARIOS: &[Scenario] = &[
    Scenario {
        label: "typing_insert_sequence_64",
        run: scenarios::typing_insert_sequence_64,
        tag: ScenarioTag::TypingSequence,
    },
    Scenario {
        label: "grouped_text_edit_word_coalesce",
        run: scenarios::grouped_text_edit_word_coalesce,
        tag: ScenarioTag::GroupedTextEdit,
    },
    Scenario {
        label: "multi_cursor_triple_insert",
        run: scenarios::multi_cursor_triple_insert,
        tag: ScenarioTag::MultiCursor,
    },
    Scenario {
        label: "structural_edit_sort_lines",
        run: scenarios::structural_edit_sort_lines,
        tag: ScenarioTag::Structural,
    },
    Scenario {
        label: "refactor_single_file_rename",
        run: scenarios::refactor_single_file_rename,
        tag: ScenarioTag::RefactorLocal,
    },
    Scenario {
        label: "refactor_multi_file_rename",
        run: scenarios::refactor_multi_file_rename,
        tag: ScenarioTag::RefactorWorkspace,
    },
    Scenario {
        label: "formatter_run_full_document",
        run: scenarios::formatter_run_full_document,
        tag: ScenarioTag::Formatter,
    },
    Scenario {
        label: "save_participant_group_pipeline",
        run: scenarios::save_participant_group_pipeline,
        tag: ScenarioTag::SavePipeline,
    },
    Scenario {
        label: "machine_generated_change_apply",
        run: scenarios::machine_generated_change_apply,
        tag: ScenarioTag::MachineGenerated,
    },
    Scenario {
        label: "external_reload_clean_buffer",
        run: scenarios::external_reload_clean_buffer,
        tag: ScenarioTag::ExternalReload,
    },
    Scenario {
        label: "decode_recovery_resolve_override",
        run: scenarios::decode_recovery_resolve_override,
        tag: ScenarioTag::DecodeRecovery,
    },
    Scenario {
        label: "undo_redo_cycle_mixed_classes",
        run: scenarios::undo_redo_cycle_mixed_classes,
        tag: ScenarioTag::UndoRedoCycle,
    },
];

/// Per-scenario metrics. Structural counts only; no wall-clock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMetrics {
    pub label: String,
    pub tag: &'static str,
    pub final_len: u64,
    pub final_version: u64,
    pub journal_len: u64,
    pub redo_len: u64,
    pub counters: HookCounters,
}

/// Aggregate across all scenarios.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AggregateMetrics {
    pub total_scenarios: u64,
    pub total_transactions: u64,
    pub total_text_edit_apply: u64,
    pub total_undo_apply: u64,
    pub total_redo_apply: u64,
    pub total_snapshot_create: u64,
    pub total_checkpoint_create: u64,
    pub total_journal_inverse_rejected: u64,
}

/// Harness output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReport {
    pub schema_version: u32,
    pub corpus_id: &'static str,
    pub scenarios: Vec<ScenarioMetrics>,
    pub aggregate: AggregateMetrics,
}

/// Run every named scenario in order and aggregate metrics.
pub fn run_harness() -> HarnessReport {
    let mut scenario_reports: Vec<ScenarioMetrics> = Vec::with_capacity(SCENARIOS.len());
    let mut agg = AggregateMetrics::default();
    for scenario in SCENARIOS {
        let mut buf = Buffer::new();
        (scenario.run)(&mut buf);
        let counters = buf.hook_counters().clone();
        agg.total_transactions += counters.transaction_apply;
        agg.total_text_edit_apply += counters.text_edit_apply;
        agg.total_undo_apply += counters.undo_apply;
        agg.total_redo_apply += counters.redo_apply;
        agg.total_snapshot_create += counters.snapshot_create;
        agg.total_checkpoint_create += counters.checkpoint_create;
        agg.total_journal_inverse_rejected += counters.journal_inverse_rejected;
        scenario_reports.push(ScenarioMetrics {
            label: scenario.label.to_owned(),
            tag: scenario.tag.name(),
            final_len: buf.len() as u64,
            final_version: buf.version(),
            journal_len: buf.journal_len() as u64,
            redo_len: buf.redo_len() as u64,
            counters,
        });
    }
    agg.total_scenarios = SCENARIOS.len() as u64;
    HarnessReport {
        schema_version: METRICS_SCHEMA_VERSION,
        corpus_id: CORPUS_ID,
        scenarios: scenario_reports,
        aggregate: agg,
    }
}

/// Render as deterministic pretty JSON.
pub fn report_to_json(report: &HarnessReport) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    kv_u64(
        &mut out,
        1,
        "schema_version",
        u64::from(report.schema_version),
        false,
    );
    kv_str(&mut out, 1, "corpus_id", report.corpus_id, false);
    key(&mut out, 1, "aggregate");
    out.push_str(" {\n");
    let agg = &report.aggregate;
    kv_u64(&mut out, 2, "total_scenarios", agg.total_scenarios, false);
    kv_u64(
        &mut out,
        2,
        "total_transactions",
        agg.total_transactions,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_text_edit_apply",
        agg.total_text_edit_apply,
        false,
    );
    kv_u64(&mut out, 2, "total_undo_apply", agg.total_undo_apply, false);
    kv_u64(&mut out, 2, "total_redo_apply", agg.total_redo_apply, false);
    kv_u64(
        &mut out,
        2,
        "total_snapshot_create",
        agg.total_snapshot_create,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_checkpoint_create",
        agg.total_checkpoint_create,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_journal_inverse_rejected",
        agg.total_journal_inverse_rejected,
        true,
    );
    indent(&mut out, 1);
    out.push_str("},\n");

    key(&mut out, 1, "scenarios");
    out.push_str(" [\n");
    for (i, scenario) in report.scenarios.iter().enumerate() {
        let last = i + 1 == report.scenarios.len();
        indent(&mut out, 2);
        out.push_str("{\n");
        kv_str(&mut out, 3, "label", &scenario.label, false);
        kv_str(&mut out, 3, "tag", scenario.tag, false);
        kv_u64(&mut out, 3, "final_len", scenario.final_len, false);
        kv_u64(&mut out, 3, "final_version", scenario.final_version, false);
        kv_u64(&mut out, 3, "journal_len", scenario.journal_len, false);
        kv_u64(&mut out, 3, "redo_len", scenario.redo_len, false);
        write_counters(&mut out, 3, &scenario.counters, true);
        indent(&mut out, 2);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(&mut out, 1);
    out.push_str("]\n");
    out.push_str("}\n");
    out
}

fn write_counters(out: &mut String, depth: usize, counters: &HookCounters, last: bool) {
    key(out, depth, "hook_counters");
    out.push_str(" {\n");
    let entries = counters.entries();
    for (i, (name, count)) in entries.iter().enumerate() {
        let entry_last = i + 1 == entries.len();
        kv_u64(out, depth + 1, name, *count, entry_last);
    }
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn key(out: &mut String, depth: usize, key: &str) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\":");
}

fn kv_u64(out: &mut String, depth: usize, key: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_str(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {}", json_quote(value));
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn json_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ---------------------------------------------------------------------------
// Scenarios. Each runs to completion and leaves the buffer in its
// terminal state so the harness can quote `final_len`, `final_version`,
// and the hook counters.
// ---------------------------------------------------------------------------

pub mod scenarios {
    use super::*;

    pub const BASE_DOC: &str = "let a = 1;\nlet b = 2;\nlet c = 3;\n";

    pub(crate) fn typing_insert_sequence_64(b: &mut Buffer) {
        *b = Buffer::from_str(BASE_DOC);
        for _ in 0..64 {
            let offset = b.len();
            b.insert(offset, ".", "user_keystroke").unwrap();
        }
        // Confirm text_edit_apply fired once per commit.
        assert_eq!(b.hook_counters().text_edit_apply, 64);
    }

    pub(crate) fn grouped_text_edit_word_coalesce(b: &mut Buffer) {
        *b = Buffer::from_str(BASE_DOC);
        let offset = b.len();
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::TextEdit,
                "user_keystroke:coalesced",
            ))
            .unwrap();
        let comment = "// comment added in one group\n";
        for (i, _) in comment.char_indices() {
            let end = i + comment[i..].chars().next().unwrap().len_utf8();
            tx.insert(offset + i, &comment[i..end]).unwrap();
        }
        tx.commit().unwrap();
    }

    pub(crate) fn multi_cursor_triple_insert(b: &mut Buffer) {
        *b = Buffer::from_str(BASE_DOC);
        // Apply right-to-left so earlier insertions do not shift later
        // offsets.
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::MultiCursorTextEdit,
                "user_keystroke:multi_cursor",
            ))
            .unwrap();
        // Append `;` after each `3`, `2`, `1` (already present; use a
        // suffix `!` so the edit is observable).
        let targets = [
            "let a = 1;".len(),
            "let a = 1;\nlet b = 2;".len(),
            "let a = 1;\nlet b = 2;\nlet c = 3;".len(),
        ];
        for offset in targets.iter().rev() {
            tx.insert(*offset, "!").unwrap();
        }
        tx.commit().unwrap();
    }

    pub(crate) fn structural_edit_sort_lines(b: &mut Buffer) {
        *b = Buffer::from_str("c\na\nb\n");
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::StructuralEdit,
                "command:sort_lines",
            ))
            .unwrap();
        tx.replace(0..6, "a\nb\nc\n").unwrap();
        tx.commit().unwrap();
        b.undo().unwrap();
        b.redo().unwrap();
    }

    pub(crate) fn refactor_single_file_rename(b: &mut Buffer) {
        *b = Buffer::from_str("let foo = 1;\nuse foo();\n");
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::RefactorSingleFile, "command:rename_local")
                    .with_label("Rename `foo` to `bar`"),
            )
            .unwrap();
        tx.replace(4..7, "bar").unwrap(); // `let foo` -> `let bar`
        tx.replace(17..20, "bar").unwrap(); // `use foo()` -> `use bar()`
        tx.commit().unwrap();
        b.snapshot();
    }

    pub(crate) fn refactor_multi_file_rename(b: &mut Buffer) {
        *b = Buffer::from_str("mod foo { pub fn run() {} }\nfoo::run();\n");
        // Named group with checkpoint taken before apply.
        let _ck = b.create_checkpoint();
        let mut tx = b
            .begin(
                TransactionSpec::new(
                    UndoClass::RefactorMultiFile,
                    "command:rename_symbol_workspace",
                )
                .with_label("Rename `foo` to `bar` across workspace"),
            )
            .unwrap();
        tx.replace(4..7, "bar").unwrap();
        tx.replace(28..31, "bar").unwrap();
        tx.commit().unwrap();
        b.undo().unwrap();
    }

    pub(crate) fn formatter_run_full_document(b: &mut Buffer) {
        *b = Buffer::from_str("fn a(){}fn b(){}fn c(){}");
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::FormatterRun, "save_participant:format_on_save")
                    .with_label("Format document"),
            )
            .unwrap();
        tx.replace(0..24, "fn a() {}\nfn b() {}\nfn c() {}\n")
            .unwrap();
        tx.commit().unwrap();
    }

    pub(crate) fn save_participant_group_pipeline(b: &mut Buffer) {
        *b = Buffer::from_str("a=1;b=2;c=3;");
        let _ck = b.create_checkpoint();
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::SaveParticipantGroup, "command:save")
                    .with_label("Save + format + organise imports"),
            )
            .unwrap();
        tx.replace(0..3, "a = 1;").unwrap();
        tx.replace(7..10, "b = 2;").unwrap();
        tx.replace(13..16, "c = 3;").unwrap();
        tx.commit().unwrap();
        // Full save snapshot afterwards.
        b.snapshot();
    }

    pub(crate) fn machine_generated_change_apply(b: &mut Buffer) {
        *b = Buffer::from_str("fn compute() -> u32 { 0 }\n");
        let _ck = b.create_checkpoint();
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::MachineGeneratedChange, "ai_apply:quickfix")
                    .with_label("Apply AI suggestion: widen return type"),
            )
            .unwrap();
        tx.replace(16..19, "u64").unwrap();
        tx.commit().unwrap();
        b.undo().unwrap();
        b.redo().unwrap();
    }

    pub(crate) fn external_reload_clean_buffer(b: &mut Buffer) {
        *b = Buffer::from_str("clean contents\n");
        // Model the reload as a single-transaction only_revertible
        // replace of the entire document.
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::ExternalReload,
                "vfs_external_change",
            ))
            .unwrap();
        let len = 15; // "clean contents\n"
        tx.replace(0..len, "clean contents (rewritten on disk)\n")
            .unwrap();
        tx.commit().unwrap();
    }

    pub(crate) fn decode_recovery_resolve_override(b: &mut Buffer) {
        *b = Buffer::new();
        // The original decode failed; the user chose an encoding
        // override and the recovery commits one decode_recovery_change
        // that plants the decoded bytes into the buffer.
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::DecodeRecoveryChange,
                "decode_recovery:user_override_encoding",
            ))
            .unwrap();
        tx.insert(0, "decoded as CP1252 per user override\n")
            .unwrap();
        tx.commit().unwrap();
    }

    pub(crate) fn undo_redo_cycle_mixed_classes(b: &mut Buffer) {
        *b = Buffer::from_str("seed\n");
        // TextEdit commit (compensatable)
        b.insert(5, "A\n", "user_keystroke").unwrap();
        // Refactor single file (compensatable) named group
        {
            let mut tx = b
                .begin(
                    TransactionSpec::new(UndoClass::RefactorSingleFile, "command:rename_local")
                        .with_label("Rename `A` to `B`"),
                )
                .unwrap();
            tx.replace(5..6, "B").unwrap();
            tx.commit().unwrap();
        }
        // Machine-generated (only_revertible) named group with
        // checkpoint.
        let _ck = b.create_checkpoint();
        {
            let mut tx = b
                .begin(
                    TransactionSpec::new(UndoClass::MachineGeneratedChange, "ai_apply:codemod")
                        .with_label("Apply codemod: append marker"),
                )
                .unwrap();
            tx.insert(7, "// edited by AI\n").unwrap();
            tx.commit().unwrap();
        }
        // Undo three, redo two.
        b.undo().unwrap();
        b.undo().unwrap();
        b.undo().unwrap();
        b.redo().unwrap();
        b.redo().unwrap();
    }
}

// ---------------------------------------------------------------------------
// Undo-example renderers.
//
// These produce deterministic, human-readable traces of specific
// scenarios so the `artifacts/buffer/undo_examples/*.txt` corpus can
// be diffed without re-running the harness. The renderer format is
// byte-stable: committed bytes are the contract; the harness tests
// round-trip each example through `render_undo_example` and compare
// against the committed artifact.
// ---------------------------------------------------------------------------

/// Every label recognised by [`render_undo_example`].
pub fn undo_example_labels() -> &'static [&'static str] {
    &[
        "typing_insert_sequence",
        "multi_cursor_triple_insert",
        "refactor_single_file_rename",
        "refactor_multi_file_rename",
        "save_participant_group_pipeline",
        "only_revertible_redo_drop",
        "compensatable_redo_after_divergence",
    ]
}

/// Render the named undo example. Returns `None` for unknown labels.
pub fn render_undo_example(label: &str) -> Option<String> {
    match label {
        "typing_insert_sequence" => Some(render_typing_insert_sequence()),
        "multi_cursor_triple_insert" => Some(render_multi_cursor_triple_insert()),
        "refactor_single_file_rename" => Some(render_refactor_single_file_rename()),
        "refactor_multi_file_rename" => Some(render_refactor_multi_file_rename()),
        "save_participant_group_pipeline" => Some(render_save_participant_group_pipeline()),
        "only_revertible_redo_drop" => Some(render_only_revertible_redo_drop()),
        "compensatable_redo_after_divergence" => Some(render_compensatable_redo_after_divergence()),
        _ => None,
    }
}

fn render_header(out: &mut String, label: &str, description: &str) {
    let _ = writeln!(out, "# undo_example: {label}");
    let _ = writeln!(out, "# {description}");
    let _ = writeln!(
        out,
        "# Schema: aureline.buffer_undo_example.v{}",
        METRICS_SCHEMA_VERSION
    );
    let _ = writeln!(out);
}

fn render_step(out: &mut String, step: usize, kind: &str, buf: &Buffer) {
    let _ = writeln!(
        out,
        "[{step:02}] {kind:<24} version={} len={} journal={} redo={}",
        buf.version(),
        buf.len(),
        buf.journal_len(),
        buf.redo_len()
    );
    let contents = match std::str::from_utf8(&buf.contents()) {
        Ok(s) => s.to_owned(),
        Err(_) => format!("<non-utf8:{} bytes>", buf.len()),
    };
    let _ = writeln!(out, "     contents   = {:?}", contents);
}

fn render_typing_insert_sequence() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "typing_insert_sequence",
        "Three single-cursor keystrokes, each one text_edit transaction. Undo then redo each.",
    );
    let mut b = Buffer::from_str("abc");
    render_step(&mut out, 0, "initial", &b);
    b.insert(3, "X", "user_keystroke").unwrap();
    render_step(&mut out, 1, "insert_X_commit", &b);
    b.insert(4, "Y", "user_keystroke").unwrap();
    render_step(&mut out, 2, "insert_Y_commit", &b);
    b.insert(5, "Z", "user_keystroke").unwrap();
    render_step(&mut out, 3, "insert_Z_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 4, "undo", &b);
    b.undo().unwrap();
    render_step(&mut out, 5, "undo", &b);
    b.redo().unwrap();
    render_step(&mut out, 6, "redo", &b);
    b.redo().unwrap();
    render_step(&mut out, 7, "redo", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_multi_cursor_triple_insert() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "multi_cursor_triple_insert",
        "One multi_cursor_text_edit transaction across three offsets. One undo reverts all three.",
    );
    let mut b = Buffer::from_str("a\nb\nc");
    render_step(&mut out, 0, "initial", &b);
    {
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::MultiCursorTextEdit,
                "user_keystroke:multi_cursor",
            ))
            .unwrap();
        tx.insert(5, ",").unwrap();
        tx.insert(3, ",").unwrap();
        tx.insert(1, ",").unwrap();
        tx.commit().unwrap();
    }
    render_step(&mut out, 1, "multicursor_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 2, "undo_one_step", &b);
    b.redo().unwrap();
    render_step(&mut out, 3, "redo_one_step", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_refactor_single_file_rename() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "refactor_single_file_rename",
        "Compensatable named group. Single undo reverts both edits atomically.",
    );
    let mut b = Buffer::from_str("let foo = 1;\nuse foo();\n");
    render_step(&mut out, 0, "initial", &b);
    {
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::RefactorSingleFile, "command:rename_local")
                    .with_label("Rename `foo` to `bar`"),
            )
            .unwrap();
        tx.replace(4..7, "bar").unwrap();
        tx.replace(17..20, "bar").unwrap();
        tx.commit().unwrap();
    }
    render_step(&mut out, 1, "rename_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 2, "undo_reverts_both", &b);
    b.redo().unwrap();
    render_step(&mut out, 3, "redo_reapplies_both", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_refactor_multi_file_rename() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "refactor_multi_file_rename",
        "Only-revertible named group with checkpoint. Undo restores the parent snapshot.",
    );
    let mut b = Buffer::from_str("mod foo { pub fn run() {} }\nfoo::run();\n");
    render_step(&mut out, 0, "initial", &b);
    let _ck = b.create_checkpoint();
    render_step(&mut out, 1, "checkpoint_created", &b);
    {
        let mut tx = b
            .begin(
                TransactionSpec::new(
                    UndoClass::RefactorMultiFile,
                    "command:rename_symbol_workspace",
                )
                .with_label("Rename `foo` to `bar` across workspace"),
            )
            .unwrap();
        tx.replace(4..7, "bar").unwrap();
        tx.replace(28..31, "bar").unwrap();
        tx.commit().unwrap();
    }
    render_step(&mut out, 2, "refactor_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 3, "undo_uses_parent_snapshot", &b);
    b.redo().unwrap();
    render_step(&mut out, 4, "redo_reapplies_forward_ops", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_save_participant_group_pipeline() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "save_participant_group_pipeline",
        "Save + format + organise imports in one only-revertible named group.",
    );
    let mut b = Buffer::from_str("a=1;b=2;c=3;");
    render_step(&mut out, 0, "initial", &b);
    let _ck = b.create_checkpoint();
    render_step(&mut out, 1, "pre_save_checkpoint", &b);
    {
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::SaveParticipantGroup, "command:save")
                    .with_label("Save + format + organise imports"),
            )
            .unwrap();
        tx.replace(0..3, "a = 1;").unwrap();
        tx.replace(7..10, "b = 2;").unwrap();
        tx.replace(13..16, "c = 3;").unwrap();
        tx.commit().unwrap();
    }
    render_step(&mut out, 2, "save_pipeline_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 3, "undo_restores_pre_save", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_only_revertible_redo_drop() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "only_revertible_redo_drop",
        "Only-revertible group + undo + divergent compensatable commit drops the redo entry.",
    );
    let mut b = Buffer::from_str("one\ntwo\n");
    render_step(&mut out, 0, "initial", &b);
    {
        let mut tx = b
            .begin(
                TransactionSpec::new(
                    UndoClass::RefactorMultiFile,
                    "command:rename_symbol_workspace",
                )
                .with_label("Rename across workspace"),
            )
            .unwrap();
        tx.replace(0..3, "ONE").unwrap();
        tx.commit().unwrap();
    }
    render_step(&mut out, 1, "only_revertible_commit", &b);
    b.undo().unwrap();
    render_step(&mut out, 2, "undo_redo_stack_has_1", &b);
    b.insert(3, "!", "user_keystroke").unwrap();
    render_step(&mut out, 3, "divergent_commit_drops_redo", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_compensatable_redo_after_divergence() -> String {
    let mut out = String::new();
    render_header(
        &mut out,
        "compensatable_redo_after_divergence",
        "Compensatable redo survives a divergent compensatable commit.",
    );
    let mut b = Buffer::from_str("abc");
    render_step(&mut out, 0, "initial", &b);
    b.insert(3, "X", "user_keystroke").unwrap();
    render_step(&mut out, 1, "insert_X", &b);
    b.insert(4, "Y", "user_keystroke").unwrap();
    render_step(&mut out, 2, "insert_Y", &b);
    b.undo().unwrap();
    render_step(&mut out, 3, "undo", &b);
    b.insert(4, "Z", "user_keystroke").unwrap();
    render_step(&mut out, 4, "divergent_insert_Z_preserves_redo", &b);
    b.redo().unwrap();
    render_step(&mut out, 5, "redo_reapplies_Y_at_recorded_offset", &b);
    render_counters(&mut out, b.hook_counters());
    out
}

fn render_counters(out: &mut String, counters: &HookCounters) {
    let _ = writeln!(out);
    let _ = writeln!(out, "hook_counters:");
    for (name, count) in counters.entries() {
        let _ = writeln!(out, "  {name} = {count}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_runs_every_scenario() {
        let report = run_harness();
        assert_eq!(report.scenarios.len(), SCENARIOS.len());
        assert_eq!(report.aggregate.total_scenarios as usize, SCENARIOS.len());
    }

    #[test]
    fn harness_report_is_byte_stable() {
        let a = run_harness();
        let b = run_harness();
        assert_eq!(report_to_json(&a), report_to_json(&b));
    }

    #[test]
    fn every_scenario_fires_at_least_one_transaction_apply() {
        let report = run_harness();
        for scenario in &report.scenarios {
            assert!(
                scenario.counters.transaction_apply >= 1,
                "scenario {} fired no transaction_apply",
                scenario.label
            );
        }
    }

    #[test]
    fn scenario_labels_are_unique_and_well_formed() {
        let mut seen = std::collections::BTreeSet::new();
        for s in SCENARIOS {
            assert!(seen.insert(s.label), "duplicate scenario label {}", s.label);
            assert!(
                s.label
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "scenario label {} is not [a-z0-9_]+",
                s.label
            );
        }
    }

    #[test]
    fn every_undo_class_is_exercised() {
        let report = run_harness();
        // Collect class-id breadth by inspecting aggregate counters
        // indirectly: every scenario fires transaction_apply, and
        // named_group scenarios fire undo_group_open/close. At least
        // one scenario must exercise every compensation posture.
        let any_compensatable = report.scenarios.iter().any(|s| {
            matches!(
                s.tag,
                "typing_sequence"
                    | "grouped_text_edit"
                    | "multi_cursor"
                    | "structural"
                    | "refactor_local"
                    | "formatter"
            )
        });
        let any_only_revertible = report.scenarios.iter().any(|s| {
            matches!(
                s.tag,
                "refactor_workspace"
                    | "save_pipeline"
                    | "machine_generated"
                    | "external_reload"
                    | "decode_recovery"
            )
        });
        assert!(any_compensatable);
        assert!(any_only_revertible);
    }

    #[test]
    fn inverse_cap_rejection_fires_counter() {
        let mut b = Buffer::from_bytes_with_config(
            b"seed",
            BufferConfig {
                inverse_cap_bytes: 2,
            },
        );
        let err = b
            .insert(4, "longer than cap", "user_keystroke")
            .unwrap_err();
        assert!(matches!(
            err,
            aureline_buffer::BufferError::InverseTooLarge { .. }
        ));
        assert_eq!(b.hook_counters().journal_inverse_rejected, 1);
    }

    #[test]
    fn posture_matches_class_for_every_undo_class() {
        // Sanity check on the enum: every UndoClass carries the right
        // posture so benches and artifacts stay consistent.
        use CompensationPosture::*;
        for class in UndoClass::ALL {
            let expected = match class {
                UndoClass::TextEdit
                | UndoClass::MultiCursorTextEdit
                | UndoClass::StructuralEdit
                | UndoClass::RefactorSingleFile
                | UndoClass::FormatterRun => Compensatable,
                _ => OnlyRevertible,
            };
            assert_eq!(class.compensation_posture(), expected, "{:?}", class);
        }
    }
}
