#!/usr/bin/env python3
"""M5 task-event first-consumers gate.

This gate enforces that the checked-in task-event first-consumers packet stays
honest: the canonical record history binds every spec-named field, every
emitting lane carries at least one record so no surface falls back to log-only
truth, every record keeps its priority rank, confidence ceiling, payload class,
and downgrade flag consistent, the seven consumer-surface projections preserve
record truth (and the two export surfaces can explain source and confidence),
the derived trace summaries match a re-derivation, and the support export and
CLI/headless view quote the packet without drift. It reads:

- the packet at
  ``artifacts/m5/tooling/event-envelope-first-consumers/packet.json``;
- the support export at
  ``artifacts/m5/tooling/event-envelope-first-consumers/support_export.json``;
- the CLI/headless view at
  ``artifacts/m5/tooling/event-envelope-first-consumers/cli_headless.json``;
- the boundary schemas at
  ``schemas/tooling/task-event-first-consumers.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/task-event-envelope.md``.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_task_event_envelope_bus`` enforces the
same invariants and that the fixtures are bit-for-bit derivable from the seed.

Exit codes:

- ``0`` -- packet is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/tooling/event-envelope-first-consumers")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
FIRST_CONSUMERS_SCHEMA_REL = Path("schemas/tooling/task-event-first-consumers.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/task-event-envelope.md")

EXPECTED_RECORD_KIND = "m5_task_event_first_consumers_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_task_event_first_consumers_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_task_event_first_consumers_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

# Canonical adapter-priority ladder: source kind -> (rank, ceiling).
PRIORITY_LADDER = {
    "native": (1, "high"),
    "bsp": (2, "high"),
    "bazel-bep": (3, "high"),
    "structured-output": (4, "medium-high"),
    "heuristic-parser": (5, "low"),
}
CONFIDENCE_WEIGHT = {"high": 4, "medium-high": 3, "medium": 2, "low": 1}
DOWNGRADE_REASONS = {
    "partial_support",
    "heuristic_fallback",
    "replay_gap",
    "unsupported_adapter_capability",
}
EMITTING_LANES = ("notebook_run", "task_center", "test_session", "debug_session", "pipeline")
REQUIRED_SURFACES = {
    "notebook_run",
    "task_center",
    "test_session",
    "debug_session",
    "pipeline",
    "support_export",
    "cli_headless",
}
EXPORT_SURFACES = {"support_export", "cli_headless"}
PROJECTION_PRESERVE_FIELDS = (
    "reads_canonical_envelope",
    "preserves_event_id",
    "preserves_source_kind",
    "preserves_priority_rank",
    "preserves_confidence",
    "preserves_payload_kind",
    "preserves_downgrade_disclosure",
    "preserves_raw_payload_ref",
    "preserves_provenance",
)
PAYLOAD_BY_EVENT = {
    "TaskQueued": "lifecycle",
    "TargetGraphReady": "lifecycle",
    "TaskStarted": "lifecycle",
    "TaskFinished": "lifecycle",
    "ProgressUpdated": "progress",
    "DiagnosticEmitted": "diagnostic",
    "TestCaseStarted": "test",
    "TestCaseFinished": "test",
    "ArtifactPublished": "artifact",
}

DOC_BACKLINKS = (
    "schemas/tooling/task-event-first-consumers.schema.json",
    "artifacts/m5/tooling/event-envelope-first-consumers/",
    "fixtures/tooling/m5/event-envelope/",
    "tools/ci/m5/task_event_envelope_bus_check.py",
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


def replay_digest(event_ids: list[str]) -> str:
    """Order-stable FNV-1a 64-bit digest, matching the Rust implementation."""
    mask = (1 << 64) - 1
    prime = 0x0000_0100_0000_01B3
    h = 0xCBF2_9CE4_8422_2325
    for event_id in event_ids:
        for byte in event_id.encode("utf-8"):
            h ^= byte
            h = (h * prime) & mask
        h ^= 0x0A
        h = (h * prime) & mask
    return f"fnv1a64:{h:016x}"


def derive_trace_summaries(events: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    by_trace: dict[str, list[dict[str, Any]]] = {}
    for event in events:
        by_trace.setdefault(str(event.get("trace_id")), []).append(event)
    summaries: dict[str, dict[str, Any]] = {}
    for trace_id, records in by_trace.items():
        ordered = sorted(records, key=lambda e: (int(e.get("sequence", 0)), str(e.get("event_id"))))
        first = ordered[0]
        sequences = [int(e.get("sequence", 0)) for e in ordered]
        summaries[trace_id] = {
            "trace_id": trace_id,
            "workspace_id": first.get("workspace_id"),
            "target_id": first.get("target_id"),
            "event_count": len(ordered),
            "first_sequence": min(sequences),
            "last_sequence": max(sequences),
            "source_kinds": sorted({str(e.get("source_kind")) for e in ordered}),
            "downgraded_event_count": sum(1 for e in ordered if bool(e.get("downgraded"))),
            "replay_digest": replay_digest([str(e.get("event_id")) for e in ordered]),
        }
    return summaries


def check_packet_block(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "record_kind_mismatch",
                f"packet.record_kind must be {EXPECTED_RECORD_KIND}",
                detail={"record_kind": packet.get("record_kind")},
            )
        )
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("schema_version_mismatch", f"packet.schema_version must be {EXPECTED_SCHEMA_VERSION}")
        )
    for ref_field in ("packet_id", "generated_at"):
        if not str(packet.get(ref_field, "")).strip():
            findings.append(Finding("identity_missing", f"packet.{ref_field} must be non-empty"))
    for ref_field in (
        "first_consumers_schema_ref",
        "envelope_schema_ref",
        "doc_ref",
        "policy_baseline_ref",
    ):
        ref = packet.get(ref_field)
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "schema_ref_missing",
                    f"packet.{ref_field} must point at an existing path",
                    detail={ref_field: ref},
                )
            )
    if packet.get("promotion_state") != "stable":
        findings.append(
            Finding(
                "promotion_not_stable",
                "packet.promotion_state must be stable",
                detail={"promotion_state": packet.get("promotion_state")},
            )
        )
    if packet.get("validation_findings"):
        findings.append(
            Finding(
                "validation_findings_present",
                "packet.validation_findings must be empty",
                detail={"count": len(packet.get("validation_findings", []))},
            )
        )


def check_events(packet: dict[str, Any], findings: list[Finding]) -> None:
    events = packet.get("events")
    if not isinstance(events, list) or not events:
        findings.append(Finding("events_missing", "packet.events must be a non-empty array"))
        return
    seen_ids: set[str] = set()
    seen_trace_seq: set[tuple[str, int]] = set()
    for event in events:
        if not isinstance(event, dict):
            continue
        event_id = str(event.get("event_id") or "<unknown>")
        source = event.get("source_kind")
        expected = PRIORITY_LADDER.get(source)
        if expected is None:
            findings.append(Finding("event_unknown_source", "a record names an unknown source kind", subject=event_id))
            continue
        rank, ceiling = expected
        if event.get("priority_rank") != rank:
            findings.append(
                Finding("event_priority_mismatch", "a record's priority rank disagrees with its source", subject=event_id)
            )
        if CONFIDENCE_WEIGHT.get(event.get("confidence"), 99) > CONFIDENCE_WEIGHT.get(ceiling, 0):
            findings.append(
                Finding("event_confidence_overclaim", "a record overclaims confidence for its source", subject=event_id)
            )
        lane = event.get("producer_lane")
        if lane not in EMITTING_LANES:
            findings.append(
                Finding("event_lane_not_emitting", "a record names a non-emitting producer lane", subject=event_id)
            )
        event_kind = event.get("event_kind")
        payload_kind = event.get("payload_kind")
        canonical_payload = PAYLOAD_BY_EVENT.get(event_kind)
        debug_allowance = lane == "debug_session" and payload_kind == "debug"
        if payload_kind != canonical_payload and not debug_allowance:
            findings.append(
                Finding(
                    "event_payload_kind_mismatch",
                    "a record's payload class disagrees with its event kind",
                    subject=event_id,
                    detail={"event_kind": event_kind, "payload_kind": payload_kind},
                )
            )
        downgraded = bool(event.get("downgraded"))
        reason = event.get("downgrade_reason")
        if downgraded != bool(reason):
            findings.append(
                Finding("event_downgrade_inconsistent", "a record's downgrade flag and reason disagree", subject=event_id)
            )
        if reason is not None and reason not in DOWNGRADE_REASONS:
            findings.append(
                Finding("event_downgrade_reason_unknown", "a record names an unknown downgrade reason", subject=event_id)
            )
        for ref_field in (
            "trace_id",
            "workspace_id",
            "target_id",
            "captured_at",
            "execution_context_id",
            "raw_payload_ref",
        ):
            if not str(event.get(ref_field, "")).strip():
                findings.append(
                    Finding("event_identity_incomplete", "a record has incomplete identity", subject=event_id, detail={"field": ref_field})
                )
        if event_id in seen_ids:
            findings.append(Finding("event_duplicate_id", "a record id is not unique", subject=event_id))
        seen_ids.add(event_id)
        key = (str(event.get("trace_id")), int(event.get("sequence", 0)))
        if key in seen_trace_seq:
            findings.append(
                Finding("replay_sequence_collision", "a trace reuses a sequence number", subject=event_id, detail={"trace_seq": list(key)})
            )
        seen_trace_seq.add(key)

    for lane in EMITTING_LANES:
        if not any(isinstance(e, dict) and e.get("producer_lane") == lane for e in events):
            findings.append(
                Finding("lane_missing_canonical_events", "an emitting lane carries no canonical records", subject=lane)
            )


def check_surface_projections(packet: dict[str, Any], findings: list[Finding]) -> None:
    projections = packet.get("surface_projections")
    if not isinstance(projections, list):
        findings.append(Finding("projections_missing", "packet.surface_projections must be an array"))
        return
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    present = {p.get("surface") for p in projections if isinstance(p, dict)}
    for surface in sorted(REQUIRED_SURFACES - present):
        findings.append(Finding("surface_projection_missing", "a required surface projection is absent", subject=surface))
    for projection in projections:
        if not isinstance(projection, dict):
            continue
        surface = projection.get("surface")
        if not str(projection.get("projection_ref", "")).strip() or any(
            projection.get(name) is not True for name in PROJECTION_PRESERVE_FIELDS
        ):
            findings.append(
                Finding("surface_projection_drops_truth", "a surface projection drops canonical record truth", subject=surface)
            )
        if surface in EXPORT_SURFACES and projection.get("explains_source_and_confidence") is not True:
            findings.append(
                Finding("export_cannot_explain", "an export surface cannot explain source and confidence", subject=surface)
            )
        if surface in EMITTING_LANES:
            expected_count = sum(1 for e in events if isinstance(e, dict) and e.get("producer_lane") == surface)
        elif surface in EXPORT_SURFACES:
            expected_count = len(events)
        else:
            expected_count = None
        if expected_count is not None and projection.get("observed_event_count") != expected_count:
            findings.append(
                Finding(
                    "surface_observed_count_drift",
                    "a surface projection observed count disagrees with the record history",
                    subject=surface,
                    detail={"expected": expected_count, "declared": projection.get("observed_event_count")},
                )
            )


def check_trace_summaries(packet: dict[str, Any], findings: list[Finding]) -> None:
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    stored = packet.get("trace_summaries")
    if not isinstance(stored, list):
        findings.append(Finding("trace_summaries_missing", "packet.trace_summaries must be an array"))
        return
    derived = derive_trace_summaries([e for e in events if isinstance(e, dict)])
    stored_by_trace = {str(s.get("trace_id")): s for s in stored if isinstance(s, dict)}
    if set(stored_by_trace) != set(derived):
        findings.append(
            Finding(
                "trace_summary_drift",
                "the stored trace set does not match the derived set",
                detail={"stored": sorted(stored_by_trace), "derived": sorted(derived)},
            )
        )
    for trace_id, derived_summary in derived.items():
        stored_summary = stored_by_trace.get(trace_id)
        if stored_summary != derived_summary:
            findings.append(
                Finding("trace_summary_drift", "a stored trace summary disagrees with the derived summary", subject=trace_id)
            )


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(
            Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}")
        )
    if export.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("support_schema_version_mismatch", "support_export.schema_version mismatch"))
    if not str(export.get("export_id", "")).strip() or not str(export.get("exported_at", "")).strip():
        findings.append(Finding("support_identity_missing", "support_export must carry an id and timestamp"))
    if export.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("support_packet_ref_mismatch", "support_export.packet_id_ref must quote the packet id"))
    if export.get("packet") != packet:
        findings.append(Finding("support_packet_drift", "support_export.packet must equal the checked-in packet"))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(
            Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}")
        )
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("cli_packet_ref_mismatch", "cli_headless.packet_id_ref must quote the packet id"))
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    ordered_ids = [
        str(e.get("event_id"))
        for e in sorted(
            (e for e in events if isinstance(e, dict)),
            key=lambda e: (str(e.get("trace_id")), int(e.get("sequence", 0)), str(e.get("event_id"))),
        )
    ]
    if view.get("replay_digest") != replay_digest(ordered_ids):
        findings.append(Finding("cli_replay_digest_drift", "cli_headless.replay_digest must match the record history"))
    rows = view.get("rows")
    if not isinstance(rows, list) or len(rows) != len(events):
        findings.append(Finding("cli_row_count_drift", "cli_headless.rows must carry one row per record"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if (
            not str(row.get("source_kind", "")).strip()
            or not str(row.get("confidence", "")).strip()
            or not str(row.get("explanation", "")).strip()
        ):
            findings.append(
                Finding("cli_row_cannot_explain", "a CLI/headless row cannot explain source and confidence", subject=row.get("event_id"))
            )


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

    packet = ensure_dict(load_json(repo_root / PACKET_REL), "packet")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    view = ensure_dict(load_json(repo_root / CLI_HEADLESS_REL), "cli_headless")
    for schema_rel in (FIRST_CONSUMERS_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_events(packet, findings)
    check_surface_projections(packet, findings)
    check_trace_summaries(packet, findings)
    check_support_export(packet, export, findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 task-event first-consumers: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
