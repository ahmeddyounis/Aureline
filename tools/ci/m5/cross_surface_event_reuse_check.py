#!/usr/bin/env python3
"""M5 cross-surface event-reuse gate.

This gate enforces that the checked-in cross-surface event-reuse packet stays
honest: one shared task/test/debug execution history is bound to every major M5
consumer (task center, test trees, coverage/flaky/snapshot intelligence, pipeline
overlays, notebook runs, incident runbooks, and the CLI/headless and support
exports); no consumer forks a private history, reconstructs it from rendered
logs, rewrites stable ids, or drops provenance; every bound trace id exists in
the shared history; every required consumer surface and flow kind is present; and
the reopen / export / rerun-review / evidence-link flows each resolve to exactly
one shared authoritative event whose trace agrees, with stable ids and provenance
preserved across the hop. It reads:

- the packet at ``artifacts/m5/tooling/cross-surface-event-reuse/packet.json``;
- the support export, AI evidence join, incident packet join, and CLI/headless
  view alongside it;
- the boundary schemas at
  ``schemas/tooling/cross-surface-event-reuse.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/cross-surface-event-reuse.md``.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_cross_surface_event_reuse`` enforces
the same invariants and that the fixtures and artifacts are bit-for-bit derivable
from the seed.

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

ARTIFACT_DIR = Path("artifacts/m5/tooling/cross-surface-event-reuse")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
INCIDENT_PACKET_REL = ARTIFACT_DIR / "incident_packet.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
REUSE_SCHEMA_REL = Path("schemas/tooling/cross-surface-event-reuse.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/cross-surface-event-reuse.md")

EXPECTED_RECORD_KIND = "m5_cross_surface_event_reuse_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_cross_surface_event_reuse_support_export"
EXPECTED_EVIDENCE_RECORD_KIND = "m5_cross_surface_event_reuse_evidence_join"
EXPECTED_CLI_RECORD_KIND = "m5_cross_surface_event_reuse_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

PRIORITY_RANK = {
    "native": 1,
    "bsp": 2,
    "bazel-bep": 3,
    "structured-output": 4,
    "heuristic-parser": 5,
}
REQUIRED_SURFACES = {
    "task_center",
    "test_tree",
    "coverage_flaky_snapshot",
    "pipeline_overlay",
    "notebook_run",
    "incident_runbook",
    "cli_headless_export",
    "support_export",
}
REQUIRED_FLOW_KINDS = {"reopen", "export", "rerun_review", "evidence_link"}
BINDING_TRUTH_FIELDS = (
    "reads_shared_history",
    "preserves_stable_ids",
    "preserves_provenance",
    "preserves_source_and_confidence",
)

DOC_BACKLINKS = (
    "schemas/tooling/cross-surface-event-reuse.schema.json",
    "artifacts/m5/tooling/cross-surface-event-reuse/",
    "fixtures/tooling/m5/consumer-parity/",
    "tools/ci/m5/cross_surface_event_reuse_check.py",
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


def history_digest(event_ids: list[str]) -> str:
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


def event_index(events: list[dict[str, Any]]) -> dict[str, str]:
    """Map from event id to trace id for events in the shared history."""
    return {
        str(e.get("event_id")): str(e.get("trace_id"))
        for e in events
        if isinstance(e, dict)
    }


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
        "reuse_schema_ref",
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
    if packet.get("shared_history_digest") != history_digest(ordered_event_ids(events)):
        findings.append(
            Finding("shared_history_digest_drift", "packet.shared_history_digest must match the shared history")
        )


def check_events(packet: dict[str, Any], findings: list[Finding]) -> None:
    events = packet.get("events")
    if not isinstance(events, list) or not events:
        findings.append(Finding("shared_history_missing", "packet.events must be a non-empty array"))
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
            findings.append(Finding("event_unknown_source", "a shared event names an unknown source kind", subject=event_id))
        elif event.get("priority_rank") != expected_rank:
            findings.append(
                Finding("event_priority_mismatch", "a shared event's priority rank disagrees with its source", subject=event_id)
            )
        if event_id in seen_ids:
            findings.append(Finding("event_duplicate_id", "a shared event id is not unique", subject=event_id))
        seen_ids.add(event_id)
        key = (str(event.get("trace_id")), int(event.get("sequence", 0)))
        if key in seen_trace_seq:
            findings.append(
                Finding("replay_sequence_collision", "a trace reuses a sequence number", subject=event_id)
            )
        seen_trace_seq.add(key)


def observed_count(bound: list[str], events: list[dict[str, Any]]) -> int:
    bound_set = {str(t) for t in bound}
    return sum(1 for e in events if isinstance(e, dict) and str(e.get("trace_id")) in bound_set)


def check_bindings(packet: dict[str, Any], findings: list[Finding]) -> None:
    bindings = packet.get("consumer_bindings")
    if not isinstance(bindings, list):
        findings.append(Finding("consumer_bindings_missing", "packet.consumer_bindings must be an array"))
        return
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    trace_ids = {str(e.get("trace_id")) for e in events if isinstance(e, dict)}
    present = {b.get("surface") for b in bindings if isinstance(b, dict)}
    for surface in sorted(REQUIRED_SURFACES - present):
        findings.append(Finding("consumer_binding_missing", "a required consumer binding is absent", subject=surface))
    for binding in bindings:
        if not isinstance(binding, dict):
            continue
        surface = binding.get("surface")
        bound = binding.get("bound_trace_ids") if isinstance(binding.get("bound_trace_ids"), list) else []
        if not str(binding.get("binding_ref", "")).strip() or not bound:
            findings.append(Finding("consumer_binding_malformed", "a consumer binding has no ref or no bound traces", subject=surface))
        if binding.get("reconstructs_from_logs") is True:
            findings.append(Finding("consumer_reconstructs_from_logs", "a consumer reconstructs history from rendered logs", subject=surface))
        for name, code in (
            ("reads_shared_history", "consumer_forks_history"),
            ("preserves_stable_ids", "consumer_rewrites_stable_ids"),
            ("preserves_provenance", "consumer_drops_provenance"),
            ("preserves_source_and_confidence", "consumer_drops_source_confidence"),
        ):
            if binding.get(name) is not True:
                findings.append(Finding(code, f"a consumer fails {name}", subject=surface))
        for trace_id in bound:
            if str(trace_id) not in trace_ids:
                findings.append(Finding("binding_trace_unknown", "a consumer binds a trace not in the shared history", subject=str(trace_id)))
        expected = observed_count(bound, events)
        if binding.get("observed_event_count") != expected:
            findings.append(
                Finding(
                    "binding_count_drift",
                    "a consumer's observed count disagrees with the shared history",
                    subject=surface,
                    detail={"expected": expected, "declared": binding.get("observed_event_count")},
                )
            )


def check_flows(packet: dict[str, Any], findings: list[Finding]) -> None:
    flows = packet.get("cross_surface_flows")
    if not isinstance(flows, list):
        findings.append(Finding("cross_surface_flows_missing", "packet.cross_surface_flows must be an array"))
        return
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    index = event_index(events)
    bindings = packet.get("consumer_bindings") if isinstance(packet.get("consumer_bindings"), list) else []
    bound_surfaces = {b.get("surface") for b in bindings if isinstance(b, dict)}
    present = {f.get("flow_kind") for f in flows if isinstance(f, dict)}
    for kind in sorted(REQUIRED_FLOW_KINDS - present):
        findings.append(Finding("flow_kind_missing", "a required cross-surface flow kind is absent", subject=kind))
    for flow in flows:
        if not isinstance(flow, dict):
            continue
        kind = flow.get("flow_kind")
        if not str(flow.get("flow_ref", "")).strip():
            findings.append(Finding("flow_malformed", "a flow has no ref", subject=kind))
        event_id = str(flow.get("authoritative_event_id"))
        resolved_trace = index.get(event_id)
        if resolved_trace is None:
            findings.append(Finding("flow_target_missing", "a flow points at an object not in the shared history", subject=kind, detail={"event_id": event_id}))
        elif resolved_trace != str(flow.get("authoritative_trace_id")):
            findings.append(Finding("flow_trace_mismatch", "a flow's authoritative trace disagrees with the resolved event", subject=kind))
        for surface in (flow.get("origin_surface"), flow.get("target_surface")):
            if surface not in bound_surfaces:
                findings.append(Finding("flow_surface_unbound", "a flow names an unbound surface", subject=str(surface)))
        if flow.get("preserves_stable_ids") is not True:
            findings.append(Finding("flow_rewrites_stable_ids", "a flow rewrites stable ids across the boundary", subject=kind))
        if flow.get("preserves_provenance") is not True:
            findings.append(Finding("flow_drops_provenance", "a flow drops provenance across the boundary", subject=kind))


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("support_schema_version_mismatch", "support_export.schema_version mismatch"))
    if not str(export.get("export_id", "")).strip() or not str(export.get("exported_at", "")).strip():
        findings.append(Finding("support_identity_missing", "support_export must carry an id and timestamp"))
    if export.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("support_packet_ref_mismatch", "support_export.packet_id_ref must quote the packet id"))
    if export.get("packet") != packet:
        findings.append(Finding("support_packet_drift", "support_export.packet must equal the checked-in packet"))


def check_evidence_join(packet: dict[str, Any], view: dict[str, Any], surface: str, findings: list[Finding]) -> None:
    label = f"{surface}_join"
    if view.get("record_kind") != EXPECTED_EVIDENCE_RECORD_KIND:
        findings.append(Finding("evidence_record_kind_mismatch", f"{label}.record_kind must be {EXPECTED_EVIDENCE_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("evidence_schema_version_mismatch", f"{label}.schema_version mismatch"))
    if view.get("surface") != surface:
        findings.append(Finding("evidence_surface_mismatch", f"{label}.surface must be {surface}"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("evidence_packet_ref_mismatch", f"{label}.packet_id_ref must quote the packet id"))
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    if view.get("shared_history_digest") != history_digest(ordered_event_ids(events)):
        findings.append(Finding("evidence_history_digest_drift", f"{label}.shared_history_digest must match the packet"))

    shared = view.get("shared_event_rows") if isinstance(view.get("shared_event_rows"), list) else []
    if len(shared) != len(events):
        findings.append(Finding("evidence_row_count_drift", f"{label} must carry one shared row per event"))
    for row in shared:
        if not isinstance(row, dict):
            continue
        if not str(row.get("source_kind", "")).strip() or not str(row.get("adapter_id", "")).strip() or not str(row.get("explanation", "")).strip():
            findings.append(Finding("evidence_flattens_provenance", f"{label} shared row drops provenance or explanation", subject=row.get("event_id")))

    index = event_index(events)
    flows = view.get("flow_rows") if isinstance(view.get("flow_rows"), list) else []
    for row in flows:
        if not isinstance(row, dict):
            continue
        event_id = str(row.get("authoritative_event_id"))
        resolves = index.get(event_id) == str(row.get("authoritative_trace_id"))
        if row.get("resolves_to_shared_object") is not resolves:
            findings.append(Finding("evidence_flow_resolution_drift", f"{label} flow resolution disagrees with the shared history", subject=row.get("flow_kind")))
        if not resolves:
            findings.append(Finding("evidence_flow_unresolved", f"{label} flow does not resolve to a shared object", subject=row.get("flow_kind")))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("cli_packet_ref_mismatch", "cli_headless.packet_id_ref must quote the packet id"))
    events = packet.get("events") if isinstance(packet.get("events"), list) else []
    if view.get("shared_history_digest") != history_digest(ordered_event_ids(events)):
        findings.append(Finding("cli_history_digest_drift", "cli_headless.shared_history_digest must match the packet"))
    bindings = packet.get("consumer_bindings") if isinstance(packet.get("consumer_bindings"), list) else []
    rows = view.get("binding_rows")
    if not isinstance(rows, list) or len(rows) != len(bindings):
        findings.append(Finding("cli_binding_row_count_drift", "cli_headless.binding_rows must carry one row per binding"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if row.get("reads_shared_history") is not True or not str(row.get("explanation", "")).strip():
            findings.append(Finding("cli_binding_not_reused", "a CLI/headless binding row does not reuse the shared history", subject=row.get("surface")))


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
    for schema_rel in (REUSE_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_events(packet, findings)
    check_bindings(packet, findings)
    check_flows(packet, findings)
    check_support_export(packet, export, findings)
    check_evidence_join(packet, ai_evidence, "ai_evidence", findings)
    check_evidence_join(packet, incident, "incident_packet", findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 cross-surface event-reuse: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
