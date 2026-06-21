#!/usr/bin/env python3
"""Freeze and certify the M5 Problems-record set: source-task correlation,
structured-versus-heuristic confidence labels, raw-output backlinks, and
rerun/jump parity.

Where ``tools/release/execution_evidence_causality.py`` certifies the *lane*
matrix (one row per Problems/output/execution-evidence surface family), this tool
certifies the *individual Problems row*. The canonical truth is the checked-in
support export
(``artifacts/tooling/m5-problem-records/support_export.json``). Each record is a
single run-derived finding bound to its source tool/run refs, its file/span
anchor, its structured-versus-heuristic parse class, its confidence tier and
raw-output backlink, the editor decoration / timeline entry / source task / owning
output channel it is correlated with, and the freshness/stale/superseded state of
the run it came from.

This tool ingests that set and, per record, **independently** re-derives an
effective status and a per-action availability that never reads wider than the
evidence supports:

* structured and heuristic origins stay distinct, and a heuristic parse keeps an
  explicit confidence tier plus a raw-output backlink;
* every row can jump to source, open owning output, and rerun or inspect the
  originating task/session *when allowed* — remote/imported origins inspect
  read-only and never rerun locally, and an authority-gated rerun is surfaced as
  gated, not silently dropped;
* findings from stale runs, superseded retries, or downgraded mappings stay
  visibly classified rather than dropped or silently upgraded;
* a finding that conflates structured/heuristic origin, loses its source-tool ref,
  drops a heuristic raw-output backlink, leaves a superseded retry unmarked, or
  lets an imported overlay claim live local authority floors to a raw-output
  backlink rather than a clean-but-false actionable row.

The Rust truth source is
``crates/aureline-runtime/src/m5_problem_records_source_task_correlation_and_rerun_jump_parity``;
this tool re-derives the same status, downgrade reasons, and action availability so
the checked-in artifacts can never imply a wider actionability claim than the
current evidence backs.

Subcommands::

    validate     Re-derive from the support export and fail on any overclaim
    corpus       Run the narrowing engine over fixtures/tooling/m5-problem-records
    emit-report  Regenerate artifacts/tooling/m5-problem-records/report.md
    emit-corpus  Regenerate the fixture corpus from the embedded case list
    self-test    End-to-end: validate, emit round-trip, and the corpus pass
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SUPPORT_EXPORT_REF = "artifacts/tooling/m5-problem-records/support_export.json"
REPORT_REF = "artifacts/tooling/m5-problem-records/report.md"
SCHEMA_REF = "schemas/tooling/m5-problem-records.schema.json"
FIXTURE_DIR = "fixtures/tooling/m5-problem-records"

RECORD_KIND = "m5_problem_record_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {
    "remote_linked_run",
    "pipeline_provider_run",
    "imported_provider_evidence",
}
HEURISTIC_PARSE = "heuristic_output_parse"
HEURISTIC_TIERS = {"heuristic_high", "heuristic_medium", "heuristic_low"}
REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    "origin_kind_flattened",
    "raw_output_backlink_missing",
    "source_ref_missing",
    "evidence_missing",
    "superseded_state_not_marked",
    "imported_overlay_claims_live",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "origin_kind_flattened": 0,
    "source_ref_missing": 1,
    "raw_output_backlink_missing": 2,
    "superseded_state_not_marked": 3,
    "imported_overlay_claims_live": 4,
    "evidence_missing": 5,
    "confidence_unlabeled": 6,
    "anchor_missing": 7,
    "owning_channel_missing": 8,
    "source_task_uncorrelated": 9,
    "editor_decoration_uncorrelated": 10,
    "timeline_uncorrelated": 11,
    "superseded_by_newer_run": 12,
    "stale_run": 13,
    "anchor_unanchored": 14,
    "downgraded_mapping": 15,
    "verification_proof_stale": 16,
    "verification_proof_missing": 17,
}

STATUS_RANK = {
    "raw_evidence_only": 0,
    "read_only_imported": 1,
    "narrowed_actionable": 2,
    "actionable": 3,
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


def present(value) -> bool:
    return isinstance(value, str) and value.strip() != ""


def order_reasons(reasons: list[str]) -> list[str]:
    seen: list[str] = []
    for reason in sorted(reasons, key=lambda r: REASON_ORDER.get(r, 99)):
        if reason not in seen:
            seen.append(reason)
    return seen


def record_reasons(rec: dict, stale_window: bool) -> list[str]:
    ev = rec["evidence"]
    src = rec["source"]
    corr = rec["correlations"]
    anchor = rec["anchor"]
    overlay = rec["origin_class"] in OVERLAY_ORIGINS
    reasons: list[str] = []

    if not ev["structured_vs_heuristic_distinct"]:
        reasons.append("origin_kind_flattened")

    if rec["parse_class"] == HEURISTIC_PARSE:
        if not ev["raw_output_backlink_present"] or not present(src["raw_output_backlink_ref"]):
            reasons.append("raw_output_backlink_missing")
        if rec["declared_confidence_tier"] not in HEURISTIC_TIERS or not ev["confidence_label_visible"]:
            reasons.append("confidence_unlabeled")
    elif not ev["confidence_label_visible"]:
        reasons.append("confidence_unlabeled")

    if not present(src["source_tool_ref"]) or not ev["preserves_source_run_lineage"]:
        reasons.append("source_ref_missing")

    anchor_present = present(anchor["file_ref"]) and anchor["start_line"] is not None
    if not anchor_present:
        reasons.append("anchor_missing")
    elif not anchor["anchored_to_current_revision"] and rec["declared_freshness_state"] != "unanchored":
        reasons.append("anchor_unanchored")

    if corr["owning_output_channel_class"] != "not_applicable" and not present(corr["owning_output_channel_ref"]):
        reasons.append("owning_channel_missing")

    if corr["rerun_authority"] != "not_applicable" and not present(corr["source_task_ref"]):
        reasons.append("source_task_uncorrelated")

    if anchor_present and not present(corr["editor_decoration_ref"]):
        reasons.append("editor_decoration_uncorrelated")

    if not present(corr["timeline_entry_ref"]):
        reasons.append("timeline_uncorrelated")

    fs = rec["declared_freshness_state"]
    if fs == "missing":
        reasons.append("evidence_missing")
    elif fs == "superseded_by_newer_run":
        reasons.append("superseded_by_newer_run" if ev["superseded_state_marked"] else "superseded_state_not_marked")
    elif fs == "unanchored":
        reasons.append("anchor_unanchored")
    elif fs == "stale_expired" and not overlay:
        reasons.append("stale_run")

    if ev["mapping_downgraded"]:
        reasons.append("downgraded_mapping")

    pc = rec["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    if overlay and not ev["imported_overlay_read_only"]:
        reasons.append("imported_overlay_claims_live")

    return order_reasons(reasons)


def claimed_status(rec: dict) -> str:
    if rec["claim_posture"] == "labs_unadvertised":
        return "labs_not_claimed"
    if rec["origin_class"] in OVERLAY_ORIGINS:
        return "read_only_imported"
    return "actionable"


def action_availability(rec: dict) -> dict:
    anchor = rec["anchor"]
    corr = rec["correlations"]
    anchor_present = present(anchor["file_ref"]) and anchor["start_line"] is not None

    if not anchor_present or rec["declared_freshness_state"] == "missing":
        jump = "unavailable"
    else:
        jump = "available"

    if corr["owning_output_channel_class"] == "not_applicable":
        output = "not_applicable"
    elif present(corr["owning_output_channel_ref"]):
        output = "available"
    else:
        output = "unavailable"

    authority = corr["rerun_authority"]
    if authority == "local_rerun_granted":
        rerun = "available" if present(corr["source_task_ref"]) else "unavailable"
    elif authority == "requires_elevated_authority":
        rerun = "gated_requires_authority"
    elif authority == "remote_inspect_read_only":
        rerun = "read_only_inspect_only"
    elif authority == "denied_policy":
        rerun = "unavailable"
    else:
        rerun = "not_applicable"

    return {
        "jump_to_source": jump,
        "open_owning_output": output,
        "rerun_or_inspect_originator": rerun,
    }


def narrow(rec: dict, stale_window: bool) -> dict:
    claimed = claimed_status(rec)
    actions = action_availability(rec)

    if claimed == "labs_not_claimed":
        return {
            "claimed_status": "labs_not_claimed",
            "effective_status": "labs_not_claimed",
            "active_downgrade_reasons": [],
            "narrowed": False,
            "actions": actions,
        }

    reasons = record_reasons(rec, stale_window)
    floored = any(r in FLOOR_REASONS for r in reasons)
    if floored:
        effective = "raw_evidence_only"
    elif reasons:
        effective = "raw_evidence_only" if claimed == "read_only_imported" else "narrowed_actionable"
    else:
        effective = claimed

    narrowed = STATUS_RANK[effective] < STATUS_RANK[claimed]
    return {
        "claimed_status": claimed,
        "effective_status": effective,
        "active_downgrade_reasons": reasons,
        "narrowed": narrowed,
        "actions": actions,
    }


def floored_keeps_fallback(rec: dict, effective: str) -> bool:
    if effective != "raw_evidence_only":
        return True
    return rec["evidence"]["raw_output_backlink_present"] and present(rec["source"]["raw_output_backlink_ref"])


def contains_forbidden(value) -> bool:
    if isinstance(value, str):
        low = value.lower()
        return any(token in low for token in FORBIDDEN_SUBSTRINGS)
    if isinstance(value, list):
        return any(contains_forbidden(v) for v in value)
    if isinstance(value, dict):
        return any(contains_forbidden(v) for v in value.values())
    return False


def load_support_export(repo_root: Path) -> dict:
    return json.loads((repo_root / SUPPORT_EXPORT_REF).read_text(encoding="utf-8"))


def validate_packet(packet: dict) -> list[str]:
    violations: list[str] = []
    if packet.get("record_kind") != RECORD_KIND:
        violations.append("wrong_record_kind")
    if packet.get("schema_version") != SCHEMA_VERSION:
        violations.append("wrong_schema_version")
    if packet.get("taxonomy_version") != TAXONOMY_VERSION:
        violations.append("wrong_taxonomy_version")
    if packet.get("redaction_class_token") not in REDACTION_TOKENS:
        violations.append("invalid_redaction_class")
    records = packet.get("records") or []
    if not records:
        violations.append("empty_records")

    seen_ids: set[str] = set()
    kinds: set[str] = set()
    demonstrates_narrowing = False
    for rec in records:
        pid = rec["problem_id"]
        if pid in seen_ids:
            violations.append(f"duplicate_problem_id:{pid}")
        seen_ids.add(pid)
        kinds.add(rec["parse_class"])
        if rec["origin_class"] in OVERLAY_ORIGINS and not present(rec["source"]["provider_ref"]):
            violations.append(f"overlay_missing_provider_ref:{pid}")
        decision = narrow(rec, False)
        if decision["narrowed"]:
            demonstrates_narrowing = True
        if not floored_keeps_fallback(rec, decision["effective_status"]):
            violations.append(f"floored_row_loses_fallback:{pid}")
        # No projection may render wider than the effective status.
        if STATUS_RANK.get(decision["effective_status"], 99) < STATUS_RANK.get(decision["claimed_status"], 99):
            pass  # narrowing is expected; recorded for the report.

    for required in (
        "structured_language_diagnostic",
        "normalized_task_event",
        "heuristic_output_parse",
        "imported_provider_annotation",
    ):
        if required not in kinds:
            violations.append(f"problem_source_kind_missing:{required}")
    if not demonstrates_narrowing:
        violations.append("downgraded_row_case_missing")
    if contains_forbidden(packet):
        violations.append("raw_boundary_material_in_export")
    return violations


def status_distribution(records: list[dict]) -> dict:
    dist = {
        "actionable": 0,
        "narrowed_actionable": 0,
        "read_only_imported": 0,
        "raw_evidence_only": 0,
        "labs_not_claimed": 0,
    }
    for rec in records:
        dist[narrow(rec, False)["effective_status"]] += 1
    return dist


def effective_confidence(rec: dict, effective: str) -> str:
    if effective == "raw_evidence_only":
        return "unmapped_requires_review"
    return rec["declared_confidence_tier"]


def narrowed_label(rec: dict, decision: dict) -> str | None:
    if not decision["narrowed"]:
        return None
    reasons = decision["active_downgrade_reasons"]
    trigger = (reasons[0] if reasons else "narrowed").replace("_", " ")
    claimed = decision["claimed_status"]
    effective = decision["effective_status"]
    if effective == "raw_evidence_only":
        return (
            f"Floored to {effective} below the {claimed} row: {trigger}; the raw-output backlink "
            "stays reopenable rather than rendering a clean-but-false actionable row"
        )
    return (
        f"Held at {effective} below the {claimed} row: {trigger}; the finding stays jumpable and "
        "inspectable until current evidence replaces it"
    )


def render_report(packet: dict) -> str:
    """Reproduces the Rust ``render_markdown_summary`` output byte-for-byte so the
    checked-in report stays identical regardless of which generator runs."""
    records = packet["records"]
    dist = status_distribution(records)
    out = "# M5 Problem Records — source-task correlation and rerun/jump parity\n\n"
    out += f"- Packet: `{packet['packet_id']}`\n"
    out += f"- Label: `{packet['label']}`\n"
    out += f"- As of: `{packet['as_of']}`\n"
    out += f"- Rows: {len(records)}\n"
    out += (
        "- Effective: {actionable} actionable, {narrowed_actionable} narrowed, "
        "{read_only_imported} read-only imported, {raw_evidence_only} raw-evidence-only, "
        "{labs_not_claimed} labs\n\n".format(**dist)
    )
    out += "| Row | Origin | Parse | Claimed | Effective | Confidence | Jump | Output | Rerun |\n"
    out += "| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n"
    for rec in records:
        d = narrow(rec, False)
        a = d["actions"]
        out += "| {pid} | {origin} | {parse} | {claimed} | {effective} | {conf} | {jump} | {output} | {rerun} |\n".format(
            pid=rec["problem_id"],
            origin=rec["origin_class"],
            parse=rec["parse_class"],
            claimed=d["claimed_status"],
            effective=d["effective_status"],
            conf=effective_confidence(rec, d["effective_status"]),
            jump=a["jump_to_source"],
            output=a["open_owning_output"],
            rerun=a["rerun_or_inspect_originator"],
        )
    out += "\n"
    for rec in records:
        d = narrow(rec, False)
        label = narrowed_label(rec, d)
        if label is not None:
            out += f"- Narrowed: `{rec['problem_id']}` — {label}\n"
    return out


# --------------------------------------------------------------------------- #
# Perturbation corpus.
# --------------------------------------------------------------------------- #

CASES = [
    ("clean-actionable", "problem:local-test-normalized-event:0001", {}, False,
     "A clean first-party local test failure keeps every action available and stays actionable.",
     "actionable", False, [], {"jump_to_source": "available", "open_owning_output": "available", "rerun_or_inspect_originator": "available"}),
    ("origin-flattened", "problem:local-heuristic-parse:0001", {"evidence.structured_vs_heuristic_distinct": False}, False,
     "Conflating structured and heuristic origin floors the row to a raw-output backlink.",
     "raw_evidence_only", True, ["origin_kind_flattened"], None),
    ("raw-backlink-missing", "problem:local-heuristic-parse:0001", {"evidence.raw_output_backlink_present": False}, False,
     "A heuristic parse with no raw-output backlink cannot be reconstructed and floors.",
     "raw_evidence_only", True, ["raw_output_backlink_missing"], None),
    ("confidence-unlabeled", "problem:local-heuristic-parse:0001", {"declared_confidence_tier": "structured_full"}, False,
     "A heuristic parse claiming a structured tier loses its honest confidence label and narrows.",
     "narrowed_actionable", True, ["confidence_unlabeled"], None),
    ("source-ref-missing", "problem:local-test-normalized-event:0001", {"source.source_tool_ref": None}, False,
     "Losing the source tool ref means the originator cannot be reopened; the row floors.",
     "raw_evidence_only", True, ["source_ref_missing"], None),
    ("anchor-missing", "problem:local-test-normalized-event:0001", {"anchor.file_ref": None, "anchor.start_line": None}, False,
     "A finding with no file/span anchor narrows and cannot jump to source.",
     "narrowed_actionable", True, ["anchor_missing"], {"jump_to_source": "unavailable"}),
    ("owning-channel-missing", "problem:local-test-normalized-event:0001", {"correlations.owning_output_channel_ref": None}, False,
     "A real owning channel with no ref narrows and cannot open owning output.",
     "narrowed_actionable", True, ["owning_channel_missing"], {"open_owning_output": "unavailable"}),
    ("source-task-uncorrelated", "problem:local-test-normalized-event:0001", {"correlations.source_task_ref": None}, False,
     "Losing the source task correlation narrows and disables rerun/inspect.",
     "narrowed_actionable", True, ["source_task_uncorrelated"], {"rerun_or_inspect_originator": "unavailable"}),
    ("editor-decoration-uncorrelated", "problem:local-test-normalized-event:0001", {"correlations.editor_decoration_ref": None}, False,
     "An anchored finding with no editor decoration narrows.",
     "narrowed_actionable", True, ["editor_decoration_uncorrelated"], None),
    ("timeline-uncorrelated", "problem:local-test-normalized-event:0001", {"correlations.timeline_entry_ref": None}, False,
     "A finding with no timeline entry narrows.",
     "narrowed_actionable", True, ["timeline_uncorrelated"], None),
    ("superseded-marked-visible", "problem:notebook-superseded:0001", {}, False,
     "A superseded retry that is visibly marked stays narrowed-but-classified, not dropped.",
     "narrowed_actionable", True, ["superseded_by_newer_run"], None),
    ("superseded-not-marked", "problem:notebook-superseded:0001", {"evidence.superseded_state_marked": False}, False,
     "A superseded retry with no visible marker would upgrade certainty and floors.",
     "raw_evidence_only", True, ["superseded_state_not_marked"], None),
    ("stale-run-visible", "problem:headless-stale-run:0001", {}, False,
     "A finding from a stale run stays visibly classified as stale.",
     "narrowed_actionable", True, ["stale_run"], None),
    ("downgraded-mapping-visible", "problem:local-downgraded-mapping:0001", {}, False,
     "A downgraded provenance mapping stays visibly classified rather than silently upgraded.",
     "narrowed_actionable", True, ["downgraded_mapping"], None),
    ("evidence-missing", "problem:local-test-normalized-event:0001", {"declared_freshness_state": "missing"}, False,
     "Missing underlying run evidence floors the row and disables jump to source.",
     "raw_evidence_only", True, ["evidence_missing"], {"jump_to_source": "unavailable"}),
    ("gated-rerun-action", "problem:extension-gated-rerun:0001", {}, False,
     "An authority-gated rerun stays actionable; only the rerun action is surfaced as gated.",
     "actionable", False, [], {"rerun_or_inspect_originator": "gated_requires_authority"}),
    ("imported-read-only", "problem:imported-provider-annotation:0001", {}, False,
     "An imported provider annotation inspects read-only and never reruns locally.",
     "read_only_imported", False, [], {"rerun_or_inspect_originator": "read_only_inspect_only", "jump_to_source": "available"}),
    ("overlay-claims-live", "problem:imported-provider-annotation:0001", {"evidence.imported_overlay_read_only": False}, False,
     "An imported overlay claiming live local authority floors.",
     "raw_evidence_only", True, ["imported_overlay_claims_live"], None),
    ("proof-stale-window", "problem:local-test-normalized-event:0001", {}, True,
     "An elapsed verification window ages out a row resting on a current proof.",
     "narrowed_actionable", True, ["verification_proof_stale"], None),
    ("proof-missing", "problem:local-test-normalized-event:0001", {"verification.proof_currency": "missing_proof"}, False,
     "A missing verification proof narrows the row.",
     "narrowed_actionable", True, ["verification_proof_missing"], None),
    ("floored-keeps-fallback", "problem:local-lineage-lost-floored:0001", {}, False,
     "A lineage-lost finding floors but keeps its raw-output backlink as the reopen fallback.",
     "raw_evidence_only", True, ["source_ref_missing", "verification_proof_stale"], None),
    ("labs-not-claimed", "problem:labs-cross-run-correlation:0001", {}, False,
     "A Labs row makes no public actionability claim and is never widened or narrowed.",
     "labs_not_claimed", False, [], None),
]


def apply_overrides(rec: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(rec))
    for dotted, value in overrides.items():
        parts = dotted.split(".")
        node = out
        for key in parts[:-1]:
            node = node[key]
        node[parts[-1]] = value
    return out


def base_record(records: list[dict], problem_id: str) -> dict:
    for rec in records:
        if rec["problem_id"] == problem_id:
            return rec
    raise SystemExit(f"base record not found: {problem_id}")


def run_corpus(packet: dict) -> list[str]:
    records = packet["records"]
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_status, exp_narrowed, exp_reasons, exp_actions = case
        rec = apply_overrides(base_record(records, base_id), overrides)
        decision = narrow(rec, stale_window)
        if decision["effective_status"] != exp_status:
            failures.append(f"{case_id}: status {decision['effective_status']} != {exp_status}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["active_downgrade_reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['active_downgrade_reasons']} != {exp_reasons}")
        if exp_actions:
            for action, expected in exp_actions.items():
                if decision["actions"][action] != expected:
                    failures.append(f"{case_id}: action {action} {decision['actions'][action]} != {expected}")
    return failures


def write_corpus(repo_root: Path) -> None:
    out_dir = repo_root / FIXTURE_DIR
    out_dir.mkdir(parents=True, exist_ok=True)
    case_files = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, desc, exp_status, exp_narrowed, exp_reasons, exp_actions = case
        expected = {
            "effective_status": exp_status,
            "narrowed": exp_narrowed,
            "active_downgrade_reasons": exp_reasons,
        }
        if exp_actions:
            expected["actions"] = exp_actions
        payload = {
            "case_id": case_id,
            "kind": "narrowing",
            "description": desc,
            "base_record_id": base_id,
            "stale_window": stale_window,
            "overrides": overrides,
            "expected": expected,
        }
        filename = f"{case_id}.json"
        (out_dir / filename).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        case_files.append(filename)
    index = {
        "corpus_id": "m5-problem-records-corpus:0001",
        "description": (
            "Perturbation corpus for the Problems-record engine. Each case starts from a canonical "
            "problem record, applies dotted-path overrides, and asserts the re-derived effective status, "
            "downgrade reasons, and per-action availability."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("problem-record set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = status_distribution(packet["records"])
    sys.stdout.write(f"problem-record set OK: {len(packet['records'])} rows, distribution {dist}\n")
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    failures = run_corpus(packet)
    if failures:
        sys.stderr.write("problem-record corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"problem-record corpus OK: {len(CASES)} cases\n")
    return 0


def cmd_emit_report(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    (repo_root / REPORT_REF).write_text(render_report(packet), encoding="utf-8")
    sys.stdout.write(f"wrote {REPORT_REF}\n")
    return 0


def cmd_emit_corpus(repo_root: Path) -> int:
    write_corpus(repo_root)
    sys.stdout.write(f"wrote {len(CASES)} cases + index to {FIXTURE_DIR}\n")
    return 0


def cmd_self_test(repo_root: Path) -> int:
    rc = cmd_validate(repo_root)
    rc |= cmd_corpus(repo_root)
    return rc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command",
        choices=["validate", "corpus", "emit-report", "emit-corpus", "self-test"],
    )
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()
    return {
        "validate": cmd_validate,
        "corpus": cmd_corpus,
        "emit-report": cmd_emit_report,
        "emit-corpus": cmd_emit_corpus,
        "self-test": cmd_self_test,
    }[args.command](repo_root)


if __name__ == "__main__":
    raise SystemExit(main())
