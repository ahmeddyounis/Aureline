#!/usr/bin/env python3
"""M5 task-event adapter-policy gate.

This gate enforces that the checked-in task-event adapter-policy baseline stays
frozen and honest: the adapter-priority ladder is the canonical native-first
order with the canonical confidence ceilings and authority flags, the
raw-payload-retention matrix covers every source/class cell with exactly one
allowed non-gated default per source, the downgrade vocabulary equals the closed
four-reason set, all six M5 consumer surfaces bind to the canonical envelope, and
every arbitration row keeps the highest-priority adapter authoritative while each
shadowing emission stays strictly lower priority and visibly downgraded. It
reads:

- the baseline at
  ``artifacts/m5/tooling/event-interop-baseline/baseline.json``;
- the support export at
  ``artifacts/m5/tooling/event-interop-baseline/support_export.json``;
- the boundary schemas at ``schemas/tooling/adapter-capability.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/task-event-and-adapter-policy.md``.

The typed Rust consumer mints the same baseline, so
``cargo test -p aureline-runtime --test m5_task_event_adapter_policy`` enforces
the same invariants and that the fixtures are bit-for-bit derivable from the
seed.

Exit codes:

- ``0`` -- baseline is clean.
- ``1`` -- one or more findings.
- ``2`` -- usage error or missing input file.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

BASELINE_REL = Path("artifacts/m5/tooling/event-interop-baseline/baseline.json")
SUPPORT_EXPORT_REL = Path("artifacts/m5/tooling/event-interop-baseline/support_export.json")
CAPABILITY_SCHEMA_REL = Path("schemas/tooling/adapter-capability.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/task-event-and-adapter-policy.md")

EXPECTED_RECORD_KIND = "m5_task_event_adapter_policy_baseline"
EXPECTED_SUPPORT_RECORD_KIND = "m5_task_event_adapter_policy_support_export"
EXPECTED_SCHEMA_VERSION = 1

# Canonical adapter-priority ladder: source kind -> (rank, ceiling, authoritative).
PRIORITY_LADDER = {
    "native": (1, "high", True),
    "bsp": (2, "high", True),
    "bazel-bep": (3, "high", True),
    "structured-output": (4, "medium-high", False),
    "heuristic-parser": (5, "low", False),
}
SOURCE_KINDS = list(PRIORITY_LADDER.keys())
RETENTION_CLASSES = {"metadata_digest_only", "redacted_reference", "support_approval_required"}
DOWNGRADE_REASONS = {
    "partial_support",
    "heuristic_fallback",
    "replay_gap",
    "unsupported_adapter_capability",
}
REQUIRED_CONSUMERS = {
    "pipeline",
    "coverage",
    "snapshot_flaky",
    "notebook_run",
    "cli_headless",
    "support_export",
}
CONSUMER_PRESERVE_FIELDS = (
    "reads_canonical_envelope",
    "preserves_source_kind",
    "preserves_priority_rank",
    "preserves_confidence",
    "preserves_downgrade_reason",
    "preserves_raw_payload_ref",
)
CONFIDENCE_WEIGHT = {"high": 4, "medium-high": 3, "medium": 2, "low": 1}

DOC_BACKLINKS = (
    "schemas/tooling/adapter-capability.schema.json",
    "schemas/tooling/task-event-envelope.schema.json",
    "artifacts/m5/tooling/event-interop-baseline/",
    "fixtures/tooling/m5/bsp-bep-native/",
    "tools/ci/m5/task_event_adapter_policy_check.py",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    subject: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject is not None:
            out["subject"] = self.subject
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Path to the repository root (default: cwd).")
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format for the findings report.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def check_envelope_block(repo_root: Path, baseline: dict[str, Any], findings: list[Finding]) -> None:
    if baseline.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "record_kind_mismatch",
                f"baseline.record_kind must be {EXPECTED_RECORD_KIND}",
                detail={"record_kind": baseline.get("record_kind")},
            )
        )
    if baseline.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("schema_version_mismatch", f"baseline.schema_version must be {EXPECTED_SCHEMA_VERSION}")
        )
    for ref_field in ("baseline_id", "generated_at"):
        if not str(baseline.get(ref_field, "")).strip():
            findings.append(Finding("identity_missing", f"baseline.{ref_field} must be non-empty"))
    for ref_field in (
        "envelope_schema_ref",
        "adapter_capability_schema_ref",
        "doc_ref",
        "seed_contract_ref",
    ):
        ref = baseline.get(ref_field)
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "schema_ref_missing",
                    f"baseline.{ref_field} must point at an existing path",
                    detail={ref_field: ref},
                )
            )
    if baseline.get("promotion_state") != "stable":
        findings.append(
            Finding(
                "promotion_not_stable",
                "baseline.promotion_state must be stable",
                detail={"promotion_state": baseline.get("promotion_state")},
            )
        )
    if baseline.get("validation_findings"):
        findings.append(
            Finding(
                "validation_findings_present",
                "baseline.validation_findings must be empty",
                detail={"count": len(baseline.get("validation_findings", []))},
            )
        )


def check_priority_ladder(baseline: dict[str, Any], findings: list[Finding]) -> None:
    ladder = baseline.get("priority_ladder")
    if not isinstance(ladder, list):
        findings.append(Finding("ladder_missing", "baseline.priority_ladder must be an array"))
        return
    seen = {}
    for rung in ladder:
        if not isinstance(rung, dict):
            continue
        source = rung.get("source_kind")
        seen[source] = rung
        expected = PRIORITY_LADDER.get(source)
        if expected is None:
            findings.append(Finding("ladder_unknown_source", "unknown source kind on the ladder", subject=source))
            continue
        rank, ceiling, authoritative = expected
        if rung.get("priority_rank") != rank:
            findings.append(
                Finding(
                    "ladder_rank_mismatch",
                    "a rung carries a non-canonical priority rank",
                    subject=source,
                    detail={"expected": rank, "declared": rung.get("priority_rank")},
                )
            )
        if rung.get("confidence_ceiling") != ceiling:
            findings.append(
                Finding(
                    "ladder_ceiling_mismatch",
                    "a rung carries a non-canonical confidence ceiling",
                    subject=source,
                    detail={"expected": ceiling, "declared": rung.get("confidence_ceiling")},
                )
            )
        if rung.get("authoritative") is not authoritative:
            findings.append(
                Finding("ladder_authority_mismatch", "a rung carries the wrong authority flag", subject=source)
            )
        if not authoritative and rung.get("masquerade_blocked") is not True:
            findings.append(
                Finding("ladder_masquerade_open", "a non-authoritative rung must block masquerade", subject=source)
            )
    for source in SOURCE_KINDS:
        if source not in seen:
            findings.append(Finding("ladder_source_missing", "the ladder must cover every source kind", subject=source))


def check_retention_matrix(baseline: dict[str, Any], findings: list[Finding]) -> None:
    matrix = baseline.get("retention_matrix")
    if not isinstance(matrix, list):
        findings.append(Finding("retention_missing", "baseline.retention_matrix must be an array"))
        return
    for source in SOURCE_KINDS:
        cells = [c for c in matrix if isinstance(c, dict) and c.get("source_kind") == source]
        classes = {c.get("retention_class") for c in cells}
        for retention_class in RETENTION_CLASSES:
            if retention_class not in classes:
                findings.append(
                    Finding(
                        "retention_cell_missing",
                        "the retention matrix must cover every source/class cell",
                        subject=source,
                        detail={"retention_class": retention_class},
                    )
                )
        defaults = [c for c in cells if c.get("is_default") is True]
        if len(defaults) != 1:
            findings.append(
                Finding(
                    "retention_default_count",
                    "each source must declare exactly one default retention class",
                    subject=source,
                    detail={"defaults": len(defaults)},
                )
            )
        elif defaults[0].get("allowed") is not True or defaults[0].get("approval_required") is True:
            findings.append(
                Finding(
                    "retention_default_gated",
                    "the default retention class must be allowed without approval",
                    subject=source,
                )
            )
    for cell in matrix:
        if not isinstance(cell, dict):
            continue
        if cell.get("allowed") is not True:
            continue
        approval_expected = cell.get("retention_class") == "support_approval_required"
        if bool(cell.get("approval_required")) is not approval_expected:
            findings.append(
                Finding(
                    "retention_approval_mismatch",
                    "a retention cell's approval flag is inconsistent with its class",
                    subject=cell.get("source_kind"),
                    detail={"retention_class": cell.get("retention_class")},
                )
            )


def check_downgrade_vocabulary(baseline: dict[str, Any], findings: list[Finding]) -> None:
    vocab = baseline.get("downgrade_vocabulary")
    if not isinstance(vocab, list):
        findings.append(Finding("downgrade_missing", "baseline.downgrade_vocabulary must be an array"))
        return
    present = [e.get("reason") for e in vocab if isinstance(e, dict)]
    if sorted(set(present)) != sorted(DOWNGRADE_REASONS) or len(present) != len(set(present)):
        findings.append(
            Finding(
                "downgrade_vocabulary_drift",
                "the downgrade vocabulary must equal the closed four-reason set",
                detail={"present": sorted(set(present))},
            )
        )
    for entry in vocab:
        if not isinstance(entry, dict):
            continue
        if entry.get("forces_visible_downgrade") is not True or not str(entry.get("summary", "")).strip():
            findings.append(
                Finding(
                    "downgrade_entry_weak",
                    "each downgrade reason must force a visible, summarized downgrade",
                    subject=entry.get("reason"),
                )
            )


def check_consumer_bindings(baseline: dict[str, Any], findings: list[Finding]) -> None:
    bindings = baseline.get("consumer_bindings")
    if not isinstance(bindings, list):
        findings.append(Finding("consumers_missing", "baseline.consumer_bindings must be an array"))
        return
    present = {b.get("consumer") for b in bindings if isinstance(b, dict)}
    for consumer in sorted(REQUIRED_CONSUMERS - present):
        findings.append(Finding("consumer_binding_missing", "a required consumer binding is absent", subject=consumer))
    for binding in bindings:
        if not isinstance(binding, dict):
            continue
        if not str(binding.get("binding_ref", "")).strip() or any(
            binding.get(field) is not True for field in CONSUMER_PRESERVE_FIELDS
        ):
            findings.append(
                Finding(
                    "consumer_binding_drops_truth",
                    "a consumer binding drops canonical envelope truth",
                    subject=binding.get("consumer"),
                )
            )


def envelope_rank(envelope: dict[str, Any]) -> int | None:
    source = envelope.get("source_kind")
    expected = PRIORITY_LADDER.get(source)
    return expected[0] if expected else None


def check_envelope(envelope: dict[str, Any], subject: str, findings: list[Finding]) -> None:
    source = envelope.get("source_kind")
    expected = PRIORITY_LADDER.get(source)
    if expected is None:
        findings.append(Finding("envelope_unknown_source", "an envelope names an unknown source kind", subject=subject))
        return
    rank, ceiling, _authoritative = expected
    if envelope.get("priority_rank") != rank:
        findings.append(
            Finding("envelope_priority_mismatch", "an envelope's priority rank disagrees with its source", subject=subject)
        )
    confidence = envelope.get("confidence")
    if CONFIDENCE_WEIGHT.get(confidence, 99) > CONFIDENCE_WEIGHT.get(ceiling, 0):
        findings.append(
            Finding("envelope_confidence_overclaim", "an envelope overclaims confidence for its source", subject=subject)
        )
    downgraded = bool(envelope.get("downgraded"))
    has_reason = bool(envelope.get("downgrade_reason"))
    if downgraded != has_reason:
        findings.append(
            Finding("envelope_downgrade_inconsistent", "an envelope's downgrade flag and reason disagree", subject=subject)
        )


def check_arbitration(baseline: dict[str, Any], findings: list[Finding]) -> None:
    rows = baseline.get("arbitration_rows")
    if not isinstance(rows, list) or not rows:
        findings.append(Finding("arbitration_missing", "baseline.arbitration_rows must be a non-empty array"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        arb_id = row.get("arbitration_id") or "<unknown>"
        winner = row.get("winning_event")
        if not isinstance(winner, dict):
            findings.append(Finding("arbitration_winner_missing", "an arbitration row lacks a winning event", subject=arb_id))
            continue
        check_envelope(winner, arb_id, findings)
        if bool(winner.get("downgraded")) or winner.get("downgrade_reason"):
            findings.append(
                Finding("arbitration_winner_downgraded", "an arbitration winner must not be downgraded", subject=arb_id)
            )
        winner_rank = envelope_rank(winner)
        if winner.get("trace_id") != row.get("trace_id") or winner.get("target_id") != row.get("target_id") or winner.get("event_kind") != row.get("event_kind"):
            findings.append(
                Finding("arbitration_winner_correlation", "an arbitration winner must share trace/target/kind", subject=arb_id)
            )
        for shadow in row.get("shadow_events", []) or []:
            if not isinstance(shadow, dict):
                continue
            check_envelope(shadow, arb_id, findings)
            shadow_rank = envelope_rank(shadow)
            if winner_rank is not None and shadow_rank is not None and shadow_rank <= winner_rank:
                findings.append(
                    Finding(
                        "arbitration_shadow_priority",
                        "a shadow must be strictly lower priority than the winner",
                        subject=arb_id,
                        detail={"winner_rank": winner_rank, "shadow_rank": shadow_rank},
                    )
                )
            if not bool(shadow.get("downgraded")) or not shadow.get("downgrade_reason"):
                findings.append(
                    Finding("arbitration_shadow_not_downgraded", "a shadow must be visibly downgraded", subject=arb_id)
                )
            if shadow.get("trace_id") != row.get("trace_id") or shadow.get("target_id") != row.get("target_id") or shadow.get("event_kind") != row.get("event_kind"):
                findings.append(
                    Finding("arbitration_shadow_correlation", "a shadow must share trace/target/kind", subject=arb_id)
                )


def check_support_export(baseline: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(
            Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}")
        )
    if export.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("support_schema_version_mismatch", "support_export.schema_version mismatch"))
    if not str(export.get("export_id", "")).strip() or not str(export.get("exported_at", "")).strip():
        findings.append(Finding("support_identity_missing", "support_export must carry an id and timestamp"))
    if export.get("baseline_id_ref") != baseline.get("baseline_id"):
        findings.append(Finding("support_baseline_ref_mismatch", "support_export.baseline_id_ref must quote the baseline id"))
    if export.get("baseline") != baseline:
        findings.append(Finding("support_baseline_drift", "support_export.baseline must equal the checked-in baseline"))


def check_doc(repo_root: Path, findings: list[Finding]) -> None:
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(
                Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink})
            )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    baseline = ensure_dict(load_json(repo_root / BASELINE_REL), "baseline")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    for schema_rel in (CAPABILITY_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_envelope_block(repo_root, baseline, findings)
    check_priority_ladder(baseline, findings)
    check_retention_matrix(baseline, findings)
    check_downgrade_vocabulary(baseline, findings)
    check_consumer_bindings(baseline, findings)
    check_arbitration(baseline, findings)
    check_support_export(baseline, export, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 task-event adapter-policy: clean")
        else:
            for finding in findings:
                location = finding.subject or "baseline"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
