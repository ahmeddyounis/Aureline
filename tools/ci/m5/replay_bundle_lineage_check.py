#!/usr/bin/env python3
"""M5 replay-bundle raw-payload lineage gate.

This gate enforces that the checked-in replay bundle stays honest: the
normalized history binds every event to exactly one typed, bounded raw-payload
lineage entry whose source kind and retention class agree with the event; every
lineage entry stays within its retention-class byte bound and carries the
canonical disclosure posture (approval-gated payloads are never support- or
AI-safe); the four join projections preserve normalized and raw truth and honor
redaction; the support, incident, and AI evidence joins gate the approval-only
payload instead of leaking it; and the four replay robustness cases stay stable
under truncation, duplicate delivery, adapter drift, and export/import
round-trip. It reads:

- the bundle at ``artifacts/m5/tooling/raw-plus-normalized-replay/packet.json``;
- the support export, AI evidence join, incident packet join, and CLI/headless
  view alongside it;
- the boundary schemas at ``schemas/tooling/replay-bundle.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/replay-and-raw-payload-lineage.md``.

The typed Rust consumer mints the same bundle, so
``cargo test -p aureline-runtime --test m5_replay_bundles`` enforces the same
invariants and that the fixtures and artifacts are bit-for-bit derivable from
the seed.

Exit codes:

- ``0`` -- bundle is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/tooling/raw-plus-normalized-replay")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
INCIDENT_PACKET_REL = ARTIFACT_DIR / "incident_packet.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
BUNDLE_SCHEMA_REL = Path("schemas/tooling/replay-bundle.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/replay-and-raw-payload-lineage.md")

EXPECTED_RECORD_KIND = "m5_replay_bundle"
EXPECTED_SUPPORT_RECORD_KIND = "m5_replay_bundle_support_export"
EXPECTED_EVIDENCE_RECORD_KIND = "m5_replay_bundle_evidence_join"
EXPECTED_CLI_RECORD_KIND = "m5_replay_bundle_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

PRIORITY_RANK = {
    "native": 1,
    "bsp": 2,
    "bazel-bep": 3,
    "structured-output": 4,
    "heuristic-parser": 5,
}
RETENTION_BYTE_BOUND = {
    "metadata_digest_only": 0,
    "redacted_reference": 4_096,
    "support_approval_required": 65_536,
}
# Approval-gated payloads stay replay-resolvable but never export- or AI-safe.
SUPPORT_AI_SAFE = {
    "metadata_digest_only": True,
    "redacted_reference": True,
    "support_approval_required": False,
}
REQUIRED_SURFACES = {"replay", "support_bundle", "incident_packet", "ai_evidence"}
EXPORT_SURFACES = {"support_bundle", "incident_packet", "ai_evidence"}
JOIN_PRESERVE_FIELDS = (
    "binds_normalized_envelope",
    "binds_raw_lineage",
    "preserves_source_kind",
    "preserves_priority_rank",
    "preserves_confidence",
    "preserves_provenance",
    "preserves_downgrade_disclosure",
    "honors_retention_redaction",
)
REQUIRED_FAILURE_MODES = {
    "truncation",
    "duplicate_delivery",
    "adapter_drift",
    "export_import_round_trip",
}
CANONICAL_RECOVERY = {
    "truncation": "reconstructed_from_lineage",
    "duplicate_delivery": "deduplicated_stable",
    "adapter_drift": "downgraded_visibly",
    "export_import_round_trip": "round_trip_stable",
}

DOC_BACKLINKS = (
    "schemas/tooling/replay-bundle.schema.json",
    "artifacts/m5/tooling/raw-plus-normalized-replay/",
    "fixtures/tooling/m5/replay-bundles/",
    "tools/ci/m5/replay_bundle_lineage_check.py",
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


def ordered_event_ids(events: list[dict[str, Any]]) -> list[str]:
    ordered = sorted(
        (e for e in events if isinstance(e, dict)),
        key=lambda e: (str(e.get("trace_id")), int(e.get("sequence", 0)), str(e.get("event_id"))),
    )
    return [str(e.get("event_id")) for e in ordered]


def citable(entry: dict[str, Any], surface: str) -> bool:
    if surface == "replay":
        return bool(entry.get("replay_safe"))
    return bool(entry.get("support_export_safe"))


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
    for ref_field in ("bundle_id", "generated_at"):
        if not str(packet.get(ref_field, "")).strip():
            findings.append(Finding("identity_missing", f"packet.{ref_field} must be non-empty"))
    for ref_field in (
        "bundle_schema_ref",
        "envelope_schema_ref",
        "doc_ref",
        "policy_baseline_ref",
        "first_consumers_packet_ref",
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
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    if packet.get("replay_digest") != replay_digest(ordered_event_ids(events)):
        findings.append(
            Finding("replay_digest_drift", "packet.replay_digest must match the normalized history")
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
        expected_rank = PRIORITY_RANK.get(source)
        if expected_rank is None:
            findings.append(Finding("event_unknown_source", "a record names an unknown source kind", subject=event_id))
        elif event.get("priority_rank") != expected_rank:
            findings.append(
                Finding("event_priority_mismatch", "a record's priority rank disagrees with its source", subject=event_id)
            )
        if event_id in seen_ids:
            findings.append(Finding("event_duplicate_id", "a record id is not unique", subject=event_id))
        seen_ids.add(event_id)
        key = (str(event.get("trace_id")), int(event.get("sequence", 0)))
        if key in seen_trace_seq:
            findings.append(
                Finding("replay_sequence_collision", "a trace reuses a sequence number", subject=event_id)
            )
        seen_trace_seq.add(key)


def derive_references(events: list[dict[str, Any]]) -> dict[str, list[str]]:
    by_ref: dict[str, list[str]] = {}
    for event in events:
        if not isinstance(event, dict):
            continue
        by_ref.setdefault(str(event.get("raw_payload_ref")), []).append(str(event.get("event_id")))
    return {ref: sorted(set(ids)) for ref, ids in by_ref.items()}


def check_lineage(packet: dict[str, Any], findings: list[Finding]) -> None:
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    lineage = packet.get("raw_lineage")
    if not isinstance(lineage, list):
        findings.append(Finding("lineage_missing", "packet.raw_lineage must be an array"))
        return
    by_ref = {str(e.get("raw_payload_ref")): e for e in lineage if isinstance(e, dict)}
    derived = derive_references([e for e in events if isinstance(e, dict)])

    for event in events:
        if not isinstance(event, dict):
            continue
        ref = str(event.get("raw_payload_ref"))
        entry = by_ref.get(ref)
        if entry is None:
            findings.append(
                Finding("lineage_entry_missing", "an event cites a raw payload with no lineage entry", subject=ref)
            )
            continue
        if entry.get("source_kind") != event.get("source_kind"):
            findings.append(Finding("lineage_source_mismatch", "a lineage entry's source disagrees with its event", subject=ref))
        if entry.get("retention_class") != event.get("raw_payload_retention_class"):
            findings.append(Finding("lineage_retention_mismatch", "a lineage entry's retention disagrees with its event", subject=ref))

    for entry in lineage:
        if not isinstance(entry, dict):
            continue
        ref = str(entry.get("raw_payload_ref"))
        retention = entry.get("retention_class")
        bound = RETENTION_BYTE_BOUND.get(retention)
        if bound is None:
            findings.append(Finding("lineage_unknown_retention", "a lineage entry names an unknown retention class", subject=ref))
            continue
        if not str(entry.get("payload_digest", "")).strip():
            findings.append(Finding("lineage_digest_missing", "a lineage entry has no digest", subject=ref))
        if entry.get("retained_byte_bound") != bound or int(entry.get("payload_byte_len", -1)) > bound:
            findings.append(
                Finding(
                    "raw_payload_unbounded",
                    "a lineage entry exceeds its retention byte bound",
                    subject=ref,
                    detail={"bound": bound, "byte_len": entry.get("payload_byte_len")},
                )
            )
        if entry.get("replay_safe") is not True:
            findings.append(Finding("retention_posture_mismatch", "a lineage entry is not replay-safe", subject=ref))
        expected_export = SUPPORT_AI_SAFE.get(retention)
        if entry.get("support_export_safe") is not expected_export or entry.get("ai_evidence_safe") is not expected_export:
            findings.append(
                Finding(
                    "retention_posture_mismatch",
                    "a lineage entry's disclosure posture disagrees with its retention class",
                    subject=ref,
                    detail={"retention_class": retention},
                )
            )
        derived_refs = derived.get(ref)
        if derived_refs is None:
            findings.append(Finding("lineage_entry_orphan", "a lineage entry is cited by no event", subject=ref))
        elif entry.get("referencing_event_ids") != derived_refs:
            findings.append(Finding("lineage_reference_drift", "a lineage entry's referencing list disagrees with the derivation", subject=ref))


def check_join_projections(packet: dict[str, Any], findings: list[Finding]) -> None:
    projections = packet.get("join_projections")
    if not isinstance(projections, list):
        findings.append(Finding("join_projections_missing", "packet.join_projections must be an array"))
        return
    lineage = packet.get("raw_lineage") if isinstance(packet.get("raw_lineage"), list) else []
    present = {p.get("surface") for p in projections if isinstance(p, dict)}
    for surface in sorted(REQUIRED_SURFACES - present):
        findings.append(Finding("join_projection_missing", "a required join projection is absent", subject=surface))
    for projection in projections:
        if not isinstance(projection, dict):
            continue
        surface = projection.get("surface")
        if not str(projection.get("join_ref", "")).strip() or any(
            projection.get(name) is not True for name in JOIN_PRESERVE_FIELDS
        ):
            findings.append(Finding("join_projection_drops_truth", "a join projection drops normalized or raw truth", subject=surface))
        expected = sum(1 for e in lineage if isinstance(e, dict) and citable(e, str(surface)))
        if projection.get("citable_payload_count") != expected:
            findings.append(
                Finding(
                    "join_count_drift",
                    "a join projection citable count disagrees with the lineage",
                    subject=surface,
                    detail={"expected": expected, "declared": projection.get("citable_payload_count")},
                )
            )


def check_robustness(packet: dict[str, Any], findings: list[Finding]) -> None:
    cases = packet.get("robustness_cases")
    if not isinstance(cases, list):
        findings.append(Finding("robustness_missing", "packet.robustness_cases must be an array"))
        return
    present = {c.get("failure_mode") for c in cases if isinstance(c, dict)}
    for mode in sorted(REQUIRED_FAILURE_MODES - present):
        findings.append(Finding("robustness_case_missing", "a required robustness case is absent", subject=mode))
    for case in cases:
        if not isinstance(case, dict):
            continue
        mode = case.get("failure_mode")
        if case.get("recovery_posture") != CANONICAL_RECOVERY.get(mode):
            findings.append(Finding("robustness_recovery_mismatch", "a robustness case has the wrong recovery posture", subject=mode))
        if case.get("stable") is not True or case.get("replay_digest_before") != case.get("replay_digest_after"):
            findings.append(Finding("replay_not_stable", "a robustness case is not stable under replay", subject=mode))


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("support_schema_version_mismatch", "support_export.schema_version mismatch"))
    if not str(export.get("export_id", "")).strip() or not str(export.get("exported_at", "")).strip():
        findings.append(Finding("support_identity_missing", "support_export must carry an id and timestamp"))
    if export.get("bundle_id_ref") != packet.get("bundle_id"):
        findings.append(Finding("support_bundle_ref_mismatch", "support_export.bundle_id_ref must quote the bundle id"))
    if export.get("bundle") != packet:
        findings.append(Finding("support_bundle_drift", "support_export.bundle must equal the checked-in bundle"))


def check_evidence_join(packet: dict[str, Any], view: dict[str, Any], surface: str, findings: list[Finding]) -> None:
    label = f"{surface}_join"
    if view.get("record_kind") != EXPECTED_EVIDENCE_RECORD_KIND:
        findings.append(Finding("evidence_record_kind_mismatch", f"{label}.record_kind must be {EXPECTED_EVIDENCE_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("evidence_schema_version_mismatch", f"{label}.schema_version mismatch"))
    if view.get("surface") != surface:
        findings.append(Finding("evidence_surface_mismatch", f"{label}.surface must be {surface}"))
    if view.get("bundle_id_ref") != packet.get("bundle_id"):
        findings.append(Finding("evidence_bundle_ref_mismatch", f"{label}.bundle_id_ref must quote the bundle id"))
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    if view.get("replay_digest") != replay_digest(ordered_event_ids(events)):
        findings.append(Finding("evidence_replay_digest_drift", f"{label}.replay_digest must match the bundle"))

    lineage = packet.get("raw_lineage") if isinstance(packet.get("raw_lineage"), list) else []
    expected_disclosed = sum(1 for e in lineage if isinstance(e, dict) and citable(e, surface))
    expected_gated = len(lineage) - expected_disclosed
    if view.get("disclosed_payload_count") != expected_disclosed or view.get("gated_payload_count") != expected_gated:
        findings.append(
            Finding(
                "evidence_disclosure_drift",
                f"{label} disclosure counts disagree with the retention posture",
                detail={"expected_disclosed": expected_disclosed, "expected_gated": expected_gated},
            )
        )

    rows = view.get("lineage_rows") if isinstance(view.get("lineage_rows"), list) else []
    for row in rows:
        if not isinstance(row, dict):
            continue
        if row.get("disclosed") is False and not str(row.get("raw_payload_ref", "")).startswith("<gated:"):
            findings.append(Finding("evidence_exposes_gated_ref", f"{label} leaks a gated raw reference", subject=row.get("retention_class")))
        if not str(row.get("payload_digest", "")).strip():
            findings.append(Finding("evidence_drops_digest", f"{label} lineage row drops its digest"))

    normalized = view.get("normalized_rows") if isinstance(view.get("normalized_rows"), list) else []
    if len(normalized) != len(events):
        findings.append(Finding("evidence_row_count_drift", f"{label} must carry one normalized row per event"))
    for row in normalized:
        if not isinstance(row, dict):
            continue
        if not str(row.get("source_kind", "")).strip() or not str(row.get("adapter_id", "")).strip() or not str(row.get("explanation", "")).strip():
            findings.append(Finding("evidence_flattens_provenance", f"{label} normalized row drops provenance or explanation", subject=row.get("event_id")))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("bundle_id_ref") != packet.get("bundle_id"):
        findings.append(Finding("cli_bundle_ref_mismatch", "cli_headless.bundle_id_ref must quote the bundle id"))
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    if view.get("replay_digest") != replay_digest(ordered_event_ids(events)):
        findings.append(Finding("cli_replay_digest_drift", "cli_headless.replay_digest must match the bundle"))
    rows = view.get("rows")
    if not isinstance(rows, list) or len(rows) != len(events):
        findings.append(Finding("cli_row_count_drift", "cli_headless.rows must carry one row per event"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if (
            not str(row.get("source_kind", "")).strip()
            or not str(row.get("raw_payload_ref", "")).strip()
            or not str(row.get("explanation", "")).strip()
        ):
            findings.append(Finding("cli_row_cannot_join", "a CLI/headless row cannot join normalized and raw truth", subject=row.get("event_id")))


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
    ai_evidence = ensure_dict(load_json(repo_root / AI_EVIDENCE_REL), "ai_evidence")
    incident = ensure_dict(load_json(repo_root / INCIDENT_PACKET_REL), "incident_packet")
    view = ensure_dict(load_json(repo_root / CLI_HEADLESS_REL), "cli_headless")
    for schema_rel in (BUNDLE_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_events(packet, findings)
    check_lineage(packet, findings)
    check_join_projections(packet, findings)
    check_robustness(packet, findings)
    check_support_export(packet, export, findings)
    check_evidence_join(packet, ai_evidence, "ai_evidence", findings)
    check_evidence_join(packet, incident, "incident_packet", findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 replay-bundle lineage: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
