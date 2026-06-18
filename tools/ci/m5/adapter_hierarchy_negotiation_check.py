#!/usr/bin/env python3
"""M5 adapter hierarchy negotiation gate.

This gate enforces that the checked-in per-ecosystem adapter capability
negotiation baseline stays frozen and honest: every claimed ecosystem resolves
its adapter ladder in the canonical native-first order, the selected adapter is
the highest-priority *eligible* candidate, every higher rung that was skipped
carries an explicit closed-vocabulary reason, structured-output and
heuristic-parser resolutions stay visibly downgraded, the resolved adapter names
its unsupported capabilities rather than dropping the rows, capability drift is
surfaced before it can degrade trust, and all four disclosure surfaces (UI,
CLI/headless, AI evidence, support/export) preserve the negotiation outcome. It
reads:

- the baseline at ``artifacts/m5/tooling/adapter-negotiation/baseline.json``;
- the support export at
  ``artifacts/m5/tooling/adapter-negotiation/support_export.json``;
- the boundary schemas at ``schemas/tooling/adapter-negotiation.schema.json``,
  ``schemas/tooling/task-event-envelope.schema.json``, and
  ``schemas/tooling/adapter-capability.schema.json``; and
- the companion doc at ``docs/m5/adapter-hierarchy-and-negotiation.md``.

The typed Rust consumer mints the same baseline, so
``cargo test -p aureline-runtime --test m5_adapter_hierarchy_negotiation``
enforces the same invariants and that the fixtures are bit-for-bit derivable from
the seed.

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

BASELINE_REL = Path("artifacts/m5/tooling/adapter-negotiation/baseline.json")
SUPPORT_EXPORT_REL = Path("artifacts/m5/tooling/adapter-negotiation/support_export.json")
NEGOTIATION_SCHEMA_REL = Path("schemas/tooling/adapter-negotiation.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
POLICY_SCHEMA_REL = Path("schemas/tooling/adapter-capability.schema.json")
DOC_REL = Path("docs/m5/adapter-hierarchy-and-negotiation.md")

EXPECTED_RECORD_KIND = "m5_adapter_hierarchy_negotiation_baseline"
EXPECTED_SUPPORT_RECORD_KIND = "m5_adapter_hierarchy_negotiation_support_export"
EXPECTED_SCHEMA_VERSION = 1

# source kind -> (rank, confidence ceiling, authoritative).
PRIORITY = {
    "native": (1, "high", True),
    "bsp": (2, "high", True),
    "bazel-bep": (3, "high", True),
    "structured-output": (4, "medium-high", False),
    "heuristic-parser": (5, "low", False),
}
SOURCE_KINDS = list(PRIORITY.keys())
CONFIDENCE_WEIGHT = {"high": 4, "medium-high": 3, "medium": 2, "low": 1}
FALLBACK_CLASS_FOR = {
    "native": "native_authoritative",
    "bsp": "negotiated_protocol",
    "bazel-bep": "negotiated_protocol",
    "structured-output": "structured_import",
    "heuristic-parser": "heuristic_last_resort",
}
DOWNGRADE_FOR_CLASS = {
    "native_authoritative": None,
    "negotiated_protocol": None,
    "structured_import": "partial_support",
    "heuristic_last_resort": "heuristic_fallback",
}
ECOSYSTEMS = ["cargo", "gradle_jvm", "bazel", "python_pytest", "node_js", "generic"]
CAPABILITIES = [
    "target_graph",
    "lifecycle_events",
    "diagnostics",
    "test_events",
    "artifacts",
    "progress",
]
DISCLOSURE_SURFACES = {"ui", "cli_headless", "ai_evidence", "support_export"}
DISCLOSE_FIELDS = (
    "discloses_selected_source_kind",
    "discloses_fallback_reason",
    "discloses_unsupported_capabilities",
    "discloses_capability_drift",
    "discloses_confidence",
)

DOC_BACKLINKS = (
    "schemas/tooling/adapter-negotiation.schema.json",
    "schemas/tooling/task-event-envelope.schema.json",
    "artifacts/m5/tooling/adapter-negotiation/",
    "fixtures/tooling/m5/bsp-bep-heuristic-fallbacks/",
    "tools/ci/m5/adapter_hierarchy_negotiation_check.py",
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
        "negotiation_schema_ref",
        "envelope_schema_ref",
        "policy_schema_ref",
        "doc_ref",
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


def check_ecosystem_coverage(baseline: dict[str, Any], findings: list[Finding]) -> None:
    resolutions = baseline.get("resolutions")
    if not isinstance(resolutions, list):
        findings.append(Finding("resolutions_missing", "baseline.resolutions must be an array"))
        return
    present = [r.get("ecosystem") for r in resolutions if isinstance(r, dict)]
    if sorted(set(present)) != sorted(ECOSYSTEMS) or len(present) != len(set(present)):
        findings.append(
            Finding(
                "ecosystem_coverage_incomplete",
                "resolutions must cover every ecosystem exactly once",
                detail={"present": sorted(set(present))},
            )
        )


def candidate_eligible(candidate: dict[str, Any]) -> bool:
    if candidate.get("available") is not True or candidate.get("negotiation_failed") is True:
        return False
    return any(
        cap.get("state") != "unsupported"
        for cap in candidate.get("capabilities", []) or []
        if isinstance(cap, dict)
    )


def skip_reason_consistent(candidate: dict[str, Any], reason: str) -> bool:
    available = candidate.get("available") is True
    negotiation_failed = candidate.get("negotiation_failed") is True
    caps = [c for c in candidate.get("capabilities", []) or [] if isinstance(c, dict)]
    if reason in ("adapter_unavailable", "ecosystem_unsupported"):
        return not available
    if reason == "negotiation_failed":
        return available and negotiation_failed
    if reason == "capability_unsupported":
        return available and not negotiation_failed and all(c.get("state") == "unsupported" for c in caps)
    return False


def check_candidate_ladder(ecosystem: str, ladder: list[Any], findings: list[Finding]) -> None:
    seen: list[str] = []
    for index, candidate in enumerate(ladder):
        if not isinstance(candidate, dict):
            continue
        source = candidate.get("source_kind")
        seen.append(source)
        expected = PRIORITY.get(source)
        if expected is None:
            findings.append(Finding("candidate_unknown_source", "candidate names an unknown source kind", subject=ecosystem))
            continue
        if candidate.get("priority_rank") != expected[0]:
            findings.append(
                Finding("candidate_rank_mismatch", "a candidate carries a non-canonical rank", subject=ecosystem, detail={"source": source})
            )
        if candidate.get("priority_rank") != index + 1:
            findings.append(Finding("candidate_ladder_out_of_order", "candidate ladder is out of priority order", subject=ecosystem))
    for source in SOURCE_KINDS:
        if source not in seen:
            findings.append(Finding("candidate_ladder_incomplete", "candidate ladder must cover every source kind", subject=ecosystem, detail={"source": source}))
    if len(seen) != len(set(seen)):
        findings.append(Finding("candidate_ladder_repeats_source", "candidate ladder repeats a source kind", subject=ecosystem))


def check_resolution(resolution: dict[str, Any], findings: list[Finding]) -> None:
    ecosystem = resolution.get("ecosystem") or "<unknown>"
    for ref_field in ("resolution_id", "workspace_id", "target_id", "selected_adapter_id"):
        if not str(resolution.get(ref_field, "")).strip():
            findings.append(Finding("resolution_identity_missing", f"resolution.{ref_field} must be non-empty", subject=ecosystem))

    ladder = resolution.get("candidate_ladder")
    if not isinstance(ladder, list):
        findings.append(Finding("candidate_ladder_missing", "resolution.candidate_ladder must be an array", subject=ecosystem))
        return
    check_candidate_ladder(ecosystem, ladder, findings)

    selected = [c for c in ladder if isinstance(c, dict) and c.get("selected") is True]
    if len(selected) != 1:
        findings.append(Finding("selection_invalid", "resolution must name exactly one selected candidate", subject=ecosystem))
        return
    sel = selected[0]
    sel_source = sel.get("source_kind")
    sel_rank = PRIORITY.get(sel_source, (99,))[0]

    if sel_source != resolution.get("selected_source_kind") or sel.get("adapter_id") != resolution.get("selected_adapter_id"):
        findings.append(Finding("selection_invalid", "selected candidate disagrees with the resolution header", subject=ecosystem))
    if sel.get("skip_reason"):
        findings.append(Finding("skip_reason_inconsistent", "selected candidate must not carry a skip reason", subject=ecosystem))
    if not candidate_eligible(sel):
        findings.append(Finding("selected_candidate_ineligible", "selected candidate is not eligible to serve truth", subject=ecosystem))

    for candidate in ladder:
        if not isinstance(candidate, dict):
            continue
        rank = PRIORITY.get(candidate.get("source_kind"), (99,))[0]
        if rank >= sel_rank or candidate.get("selected") is True:
            continue
        if candidate_eligible(candidate):
            findings.append(
                Finding("lower_priority_displaced_higher", "a higher eligible adapter was displaced", subject=ecosystem, detail={"source": candidate.get("source_kind")})
            )
            continue
        reason = candidate.get("skip_reason")
        if not reason:
            findings.append(Finding("skip_reason_missing", "a higher skipped adapter lacks a skip reason", subject=ecosystem, detail={"source": candidate.get("source_kind")}))
        elif not skip_reason_consistent(candidate, reason):
            findings.append(Finding("skip_reason_inconsistent", "a skip reason disagrees with the candidate posture", subject=ecosystem, detail={"source": candidate.get("source_kind")}))

    check_fallback_class(resolution, sel_source, ecosystem, findings)
    check_capabilities(resolution, sel, ecosystem, findings)
    check_fallback_reason_packet(resolution, ladder, sel_rank, ecosystem, findings)


def check_fallback_class(resolution: dict[str, Any], sel_source: str, ecosystem: str, findings: list[Finding]) -> None:
    expected_class = FALLBACK_CLASS_FOR.get(sel_source)
    if resolution.get("fallback_class") != expected_class:
        findings.append(Finding("fallback_class_mismatch", "fallback class disagrees with the selected source", subject=ecosystem))
    ceiling = PRIORITY.get(sel_source, (0, "low"))[1]
    confidence = resolution.get("confidence")
    if CONFIDENCE_WEIGHT.get(confidence, 99) > CONFIDENCE_WEIGHT.get(ceiling, 0):
        findings.append(Finding("confidence_overclaim", "resolution overclaims confidence for its source", subject=ecosystem))
    expected_reason = DOWNGRADE_FOR_CLASS.get(expected_class)
    downgraded = bool(resolution.get("downgraded"))
    if downgraded != (expected_reason is not None) or resolution.get("downgrade_reason") != expected_reason:
        findings.append(Finding("downgrade_inconsistent", "downgrade posture disagrees with the fallback class", subject=ecosystem))


def check_capabilities(resolution: dict[str, Any], sel: dict[str, Any], ecosystem: str, findings: list[Finding]) -> None:
    caps = [c for c in sel.get("capabilities", []) or [] if isinstance(c, dict)]
    present = [c.get("capability") for c in caps]
    for capability in CAPABILITIES:
        if capability not in present:
            findings.append(Finding("capability_coverage_incomplete", "selected candidate must disclose every capability", subject=ecosystem, detail={"capability": capability}))
    if len(present) != len(set(present)):
        findings.append(Finding("capability_coverage_incomplete", "selected candidate repeats a capability", subject=ecosystem))
    derived = [cap for cap in CAPABILITIES if any(c.get("capability") == cap and c.get("state") == "unsupported" for c in caps)]
    stored_raw = resolution.get("unsupported_capabilities", []) or []
    stored = [cap for cap in CAPABILITIES if cap in stored_raw]
    if derived != stored or len(set(stored_raw)) != len(stored_raw):
        findings.append(
            Finding("unsupported_capability_unnamed", "named unsupported capabilities disagree with the selected adapter", subject=ecosystem, detail={"derived": derived, "stored": stored_raw})
        )


def check_fallback_reason_packet(resolution: dict[str, Any], ladder: list[Any], sel_rank: int, ecosystem: str, findings: list[Finding]) -> None:
    derived = {}
    for candidate in ladder:
        if not isinstance(candidate, dict):
            continue
        rank = PRIORITY.get(candidate.get("source_kind"), (99,))[0]
        if rank < sel_rank and candidate.get("skip_reason"):
            derived[candidate.get("source_kind")] = (candidate.get("skip_reason"), candidate.get("adapter_id"))
    stored = resolution.get("fallback_reasons", []) or []
    stored_keys = [r.get("source_kind") for r in stored if isinstance(r, dict)]
    if sorted(derived.keys()) != sorted(stored_keys) or len(stored_keys) != len(set(stored_keys)):
        findings.append(Finding("fallback_reason_packet_mismatch", "fallback-reason packet disagrees with the candidate ladder", subject=ecosystem))
        return
    for reason in stored:
        if not isinstance(reason, dict):
            continue
        expected = derived.get(reason.get("source_kind"))
        if expected is None:
            continue
        if reason.get("skip_reason") != expected[0] or reason.get("adapter_id") != expected[1] or not str(reason.get("summary", "")).strip():
            findings.append(Finding("fallback_reason_packet_mismatch", "a fallback reason disagrees with the ladder", subject=ecosystem, detail={"source": reason.get("source_kind")}))


def check_drift(baseline: dict[str, Any], findings: list[Finding]) -> None:
    signals = baseline.get("drift_signals")
    if not isinstance(signals, list) or not signals:
        findings.append(Finding("drift_missing", "baseline.drift_signals must demonstrate at least one surfaced signal"))
        return
    for signal in signals:
        if not isinstance(signal, dict):
            continue
        if signal.get("visible_before_trust_loss") is not True or not str(signal.get("summary", "")).strip():
            findings.append(Finding("drift_not_visible", "a drift signal is not visibly surfaced", subject=signal.get("drift_id")))


def check_disclosure(baseline: dict[str, Any], findings: list[Finding]) -> None:
    bindings = baseline.get("disclosure_surfaces")
    if not isinstance(bindings, list):
        findings.append(Finding("disclosure_missing", "baseline.disclosure_surfaces must be an array"))
        return
    present = {b.get("surface") for b in bindings if isinstance(b, dict)}
    for surface in sorted(DISCLOSURE_SURFACES - present):
        findings.append(Finding("disclosure_surface_missing", "a required disclosure surface is absent", subject=surface))
    for binding in bindings:
        if not isinstance(binding, dict):
            continue
        if not str(binding.get("binding_ref", "")).strip() or any(binding.get(field) is not True for field in DISCLOSE_FIELDS):
            findings.append(Finding("disclosure_surface_drops_truth", "a disclosure surface drops negotiation truth", subject=binding.get("surface")))


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
    for schema_rel in (NEGOTIATION_SCHEMA_REL, ENVELOPE_SCHEMA_REL, POLICY_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_envelope_block(repo_root, baseline, findings)
    check_ecosystem_coverage(baseline, findings)
    for resolution in baseline.get("resolutions", []) or []:
        if isinstance(resolution, dict):
            check_resolution(resolution, findings)
    check_drift(baseline, findings)
    check_disclosure(baseline, findings)
    check_support_export(baseline, export, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 adapter hierarchy negotiation: clean")
        else:
            for finding in findings:
                location = finding.subject or "baseline"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
