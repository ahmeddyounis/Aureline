#!/usr/bin/env python3
"""M5 build/test interop conformance gate.

This gate enforces that the checked-in interop conformance packet stays honest:
the four named corpora (BSP discovery, Bazel BEP/BES, structured-output
JUnit/SARIF, and problem-matcher/heuristic fallbacks) are each present once, run
across every claimed M5 archetype that depends on them, and grade every case on
the seven conformance dimensions (capability negotiation, fallback reason,
confidence preservation, raw-payload retention, replay stability, degraded-state
behavior, and export parity). A case that overclaims confidence, loses its raw
payload, drops a fallback reason, hides a degraded state, breaks replay, or
breaks export parity blocks stable; a corpus whose proof has aged past its
freshness window narrows below stable. It reads:

- the packet at ``artifacts/m5/tooling/interop-conformance/packet.json``;
- the support export, AI evidence join, incident packet join, and CLI/headless
  view alongside it;
- the boundary schemas at
  ``schemas/tooling/interop-conformance.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/build-test-interop-corpora.md``.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_interop_conformance`` enforces the
same invariants and that the fixtures and artifacts are bit-for-bit derivable
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

ARTIFACT_DIR = Path("artifacts/m5/tooling/interop-conformance")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
INCIDENT_PACKET_REL = ARTIFACT_DIR / "incident_packet.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
CONFORMANCE_SCHEMA_REL = Path("schemas/tooling/interop-conformance.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/build-test-interop-corpora.md")

EXPECTED_RECORD_KIND = "m5_interop_conformance_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_interop_conformance_support_export"
EXPECTED_EVIDENCE_RECORD_KIND = "m5_interop_conformance_evidence_join"
EXPECTED_CLI_RECORD_KIND = "m5_interop_conformance_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

FAMILY_SOURCE_KIND = {
    "bsp_discovery": "bsp",
    "bazel_bep_bes": "bazel-bep",
    "structured_output_junit_sarif": "structured-output",
    "problem_matcher_heuristic": "heuristic-parser",
}
REQUIRED_FAMILIES = list(FAMILY_SOURCE_KIND.keys())

# Each corpus must cover every archetype that depends on its family.
FAMILY_ARCHETYPES = {
    "bsp_discovery": {"jvm_build_server", "bazel_monorepo"},
    "bazel_bep_bes": {"bazel_monorepo"},
    "structured_output_junit_sarif": {
        "rust_cargo",
        "node_workspace",
        "python_pytest",
        "jvm_build_server",
        "bazel_monorepo",
        "polyglot_ci",
    },
    "problem_matcher_heuristic": {
        "rust_cargo",
        "node_workspace",
        "python_pytest",
        "polyglot_ci",
    },
}

DIMENSIONS = (
    "capability_negotiation",
    "fallback_reason",
    "confidence_preservation",
    "raw_payload_retention",
    "replay_stability",
    "degraded_state_behavior",
    "export_parity",
)

DOC_BACKLINKS = (
    "schemas/tooling/interop-conformance.schema.json",
    "artifacts/m5/tooling/interop-conformance/",
    "fixtures/tooling/m5/bsp-discovery/",
    "fixtures/tooling/m5/bazel-bep-bes/",
    "fixtures/tooling/m5/structured-output-junit-sarif/",
    "fixtures/tooling/m5/problem-matcher-heuristic/",
    "tools/ci/m5/interop_conformance_check.py",
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


def corpus_digest(case_ids: list[str]) -> str:
    """Order-stable FNV-1a 64-bit digest over sorted case ids, matching Rust."""
    mask = (1 << 64) - 1
    prime = 0x0000_0100_0000_01B3
    h = 0xCBF2_9CE4_8422_2325
    for case_id in sorted(case_ids):
        for byte in case_id.encode("utf-8"):
            h ^= byte
            h = (h * prime) & mask
        h ^= 0x0A
        h = (h * prime) & mask
    return f"fnv1a64:{h:016x}"


def all_case_ids(corpora: list[dict[str, Any]]) -> list[str]:
    return [
        str(case.get("case_id"))
        for corpus in corpora
        if isinstance(corpus, dict)
        for case in (corpus.get("cases") or [])
        if isinstance(case, dict)
    ]


def evaluate_case(case: dict[str, Any]) -> dict[str, bool]:
    """Re-derives the per-dimension pass/fail, mirroring the Rust evaluator."""
    capability = case.get("negotiated_capability")
    requires_fallback = capability in ("degraded", "unsupported")
    reason = case.get("fallback_reason")
    reason_present = bool(isinstance(reason, str) and reason.strip())
    must_be_low = case.get("source_kind") == "heuristic-parser" or capability == "unsupported"
    return {
        "capability_negotiation": bool(str(case.get("capability_packet_ref", "")).strip()),
        "fallback_reason": reason_present if requires_fallback else not reason_present,
        "confidence_preservation": not (must_be_low and case.get("observed_confidence") != "low"),
        "raw_payload_retention": bool(str(case.get("raw_payload_ref", "")).strip())
        and bool(str(case.get("payload_digest", "")).strip())
        and case.get("raw_private_material_excluded") is True,
        "replay_stability": case.get("replay_stable") is True,
        "degraded_state_behavior": (not requires_fallback)
        or case.get("degraded_state_disclosed") is True,
        "export_parity": case.get("export_parity_preserved") is True,
    }


DIMENSION_FINDING = {
    "capability_negotiation": ("capability_negotiation_missing", "a case ran no capability handshake"),
    "confidence_preservation": ("confidence_overclaim", "a case overclaims confidence for its source/capability"),
    "raw_payload_retention": ("raw_payload_not_retained", "a case does not retain its raw payload safely"),
    "replay_stability": ("replay_unstable", "a case does not replay deterministically"),
    "degraded_state_behavior": ("degraded_state_not_disclosed", "a case hides its degraded/unsupported state"),
    "export_parity": ("export_parity_broken", "a case breaks support/release/AI export parity"),
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
        "conformance_schema_ref",
        "envelope_schema_ref",
        "doc_ref",
        "policy_baseline_ref",
        "interop_packet_ref",
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
    corpora = packet.get("corpora") if isinstance(packet.get("corpora"), list) else []
    if packet.get("corpus_digest") != corpus_digest(all_case_ids(corpora)):
        findings.append(
            Finding("corpus_digest_drift", "packet.corpus_digest must match the corpora")
        )


def check_corpora(packet: dict[str, Any], findings: list[Finding]) -> None:
    corpora = packet.get("corpora")
    if not isinstance(corpora, list) or not corpora:
        findings.append(Finding("corpora_missing", "packet.corpora must be a non-empty array"))
        return
    present: dict[str, int] = {}
    for corpus in corpora:
        if isinstance(corpus, dict):
            family = str(corpus.get("family"))
            present[family] = present.get(family, 0) + 1
    for family in REQUIRED_FAMILIES:
        count = present.get(family, 0)
        if count == 0:
            findings.append(Finding("missing_corpus_family", "a required corpus family is absent", subject=family))
        elif count > 1:
            findings.append(Finding("duplicate_corpus_family", "a corpus family is declared more than once", subject=family))

    for corpus in corpora:
        if not isinstance(corpus, dict):
            continue
        check_corpus(corpus, findings)


def check_corpus(corpus: dict[str, Any], findings: list[Finding]) -> None:
    family = str(corpus.get("family"))
    expected_source = FAMILY_SOURCE_KIND.get(family)
    if expected_source is not None and corpus.get("source_kind") != expected_source:
        findings.append(Finding("case_source_kind_mismatch", "a corpus declares the wrong source kind", subject=family))
    cases = corpus.get("cases") if isinstance(corpus.get("cases"), list) else []
    if not cases:
        findings.append(Finding("corpus_empty", "a corpus carries no cases", subject=family))

    covered = {str(c.get("archetype")) for c in cases if isinstance(c, dict)}
    for archetype in sorted(FAMILY_ARCHETYPES.get(family, set()) - covered):
        findings.append(
            Finding("missing_archetype_coverage", "a corpus does not cover a dependent archetype", subject=f"{family}:{archetype}")
        )

    age = corpus.get("proof_age_days")
    window = corpus.get("freshness_window_days")
    if isinstance(age, int) and isinstance(window, int):
        expected_freshness = "stale" if age > window else "current"
        if corpus.get("freshness_state") != expected_freshness:
            findings.append(Finding("corpus_freshness_drift", "a corpus freshness state disagrees with proof age", subject=family))
        if expected_freshness == "stale":
            findings.append(Finding("corpus_evidence_stale", "a corpus proof has aged past its window", subject=family))
    else:
        findings.append(Finding("corpus_freshness_drift", "a corpus has non-integer proof age/window", subject=family))

    expected_conform = bool(cases) and all(
        all(evaluate_case(c).values()) for c in cases if isinstance(c, dict)
    )
    if corpus.get("all_cases_conform") is not expected_conform:
        findings.append(Finding("corpus_conformance_drift", "a corpus conformance roll-up disagrees with its cases", subject=family))

    for case in cases:
        if isinstance(case, dict):
            check_case(case, family, findings)


def check_case(case: dict[str, Any], family: str, findings: list[Finding]) -> None:
    case_id = str(case.get("case_id") or "<unknown>")
    expected_source = FAMILY_SOURCE_KIND.get(family)
    if expected_source is not None and case.get("source_kind") != expected_source:
        findings.append(Finding("case_source_kind_mismatch", "a case source kind disagrees with its family", subject=case_id))

    outcomes = evaluate_case(case)
    capability = case.get("negotiated_capability")
    requires_fallback = capability in ("degraded", "unsupported")
    for dimension, passed in outcomes.items():
        if passed:
            continue
        if dimension == "fallback_reason":
            if requires_fallback:
                findings.append(Finding("fallback_reason_missing", "a degraded/unsupported case names no fallback reason", subject=case_id))
            else:
                findings.append(Finding("fallback_reason_unexpected", "a negotiated case names a spurious fallback reason", subject=case_id))
            continue
        code, message = DIMENSION_FINDING[dimension]
        findings.append(Finding(code, message, subject=case_id))

    # Stored derived fields must match the recomputation.
    stored = case.get("dimension_outcomes")
    if not isinstance(stored, list) or len(stored) != len(DIMENSIONS):
        findings.append(Finding("dimension_outcome_drift", "a case stored the wrong number of dimension outcomes", subject=case_id))
    else:
        for index, dimension in enumerate(DIMENSIONS):
            row = stored[index]
            if not isinstance(row, dict) or row.get("dimension") != dimension:
                findings.append(Finding("dimension_outcome_drift", "a case dimension outcome is out of order", subject=case_id))
            elif row.get("passed") is not outcomes[dimension]:
                findings.append(Finding("dimension_outcome_drift", "a case dimension outcome disagrees with the derivation", subject=f"{case_id}:{dimension}"))
    if case.get("conforms") is not all(outcomes.values()):
        findings.append(Finding("case_conformance_drift", "a case conformance flag disagrees with the derivation", subject=case_id))


def derive_release_evidence(corpora: list[dict[str, Any]]) -> dict[str, Any]:
    all_current = all(c.get("freshness_state") == "current" for c in corpora if isinstance(c, dict))
    all_conform = bool(corpora) and all(c.get("all_cases_conform") is True for c in corpora if isinstance(c, dict))
    narrowed = [
        str(c.get("family"))
        for c in corpora
        if isinstance(c, dict)
        and not (c.get("freshness_state") == "current" and c.get("all_cases_conform") is True)
    ]
    return {"all_corpora_current": all_current, "all_cases_conform": all_conform, "narrowed_families": narrowed}


def check_release_evidence(packet: dict[str, Any], findings: list[Finding]) -> None:
    binding = packet.get("release_evidence")
    if not isinstance(binding, dict):
        findings.append(Finding("release_evidence_missing", "packet.release_evidence must be an object"))
        return
    if not str(binding.get("release_evidence_ref", "")).strip():
        findings.append(Finding("release_evidence_missing", "release_evidence.release_evidence_ref must be non-empty"))
    if not str(binding.get("conformance_summary", "")).strip():
        findings.append(Finding("release_evidence_drift", "release_evidence.conformance_summary must be non-empty"))
    corpora = packet.get("corpora") if isinstance(packet.get("corpora"), list) else []
    expected = derive_release_evidence(corpora)
    if binding.get("all_corpora_current") is not expected["all_corpora_current"]:
        findings.append(Finding("release_evidence_drift", "release_evidence.all_corpora_current disagrees with the corpora"))
    if binding.get("all_cases_conform") is not expected["all_cases_conform"]:
        findings.append(Finding("release_evidence_drift", "release_evidence.all_cases_conform disagrees with the corpora"))
    if list(binding.get("narrowed_families") or []) != expected["narrowed_families"]:
        findings.append(Finding("release_evidence_drift", "release_evidence.narrowed_families disagrees with the corpora"))


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
    if view.get("corpus_digest") != packet.get("corpus_digest"):
        findings.append(Finding("evidence_corpus_digest_drift", f"{label}.corpus_digest must match the packet"))
    if view.get("release_evidence") != packet.get("release_evidence"):
        findings.append(Finding("evidence_release_drift", f"{label}.release_evidence must match the packet"))

    corpora = packet.get("corpora") if isinstance(packet.get("corpora"), list) else []
    corpus_rows = view.get("corpus_rows") if isinstance(view.get("corpus_rows"), list) else []
    if len(corpus_rows) != len(corpora):
        findings.append(Finding("evidence_corpus_row_drift", f"{label} must carry one corpus row per corpus"))
    for row in corpus_rows:
        if not isinstance(row, dict):
            continue
        if not str(row.get("fixture_dir", "")).strip() or not str(row.get("explanation", "")).strip():
            findings.append(Finding("evidence_corpus_row_flat", f"{label} corpus row drops fixture dir or explanation", subject=row.get("family")))

    case_rows = view.get("case_rows") if isinstance(view.get("case_rows"), list) else []
    if len(case_rows) != len(all_case_ids(corpora)):
        findings.append(Finding("evidence_case_row_drift", f"{label} must carry one case row per case"))
    for row in case_rows:
        if not isinstance(row, dict):
            continue
        if not str(row.get("source_kind", "")).strip() or not str(row.get("explanation", "")).strip():
            findings.append(Finding("evidence_flattens_provenance", f"{label} case row drops source or explanation", subject=row.get("case_id")))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("cli_packet_ref_mismatch", "cli_headless.packet_id_ref must quote the packet id"))
    if view.get("corpus_digest") != packet.get("corpus_digest"):
        findings.append(Finding("cli_corpus_digest_drift", "cli_headless.corpus_digest must match the packet"))
    if view.get("promotion_state") != packet.get("promotion_state"):
        findings.append(Finding("cli_promotion_drift", "cli_headless.promotion_state must match the packet"))
    corpora = packet.get("corpora") if isinstance(packet.get("corpora"), list) else []
    rows = view.get("corpus_rows")
    if not isinstance(rows, list) or len(rows) != len(corpora):
        findings.append(Finding("cli_corpus_row_drift", "cli_headless.corpus_rows must carry one row per corpus"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not (isinstance(row.get("case_count"), int) and row.get("case_count") > 0) or not str(row.get("explanation", "")).strip():
            findings.append(Finding("cli_corpus_not_run", "a CLI/headless corpus row ran no explained case", subject=row.get("family")))


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
    for schema_rel in (CONFORMANCE_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_corpora(packet, findings)
    check_release_evidence(packet, findings)
    check_support_export(packet, export, findings)
    check_evidence_join(packet, ai_evidence, "ai_evidence", findings)
    check_evidence_join(packet, incident, "incident_packet", findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 interop conformance: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
