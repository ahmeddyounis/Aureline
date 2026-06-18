#!/usr/bin/env python3
"""M5 adapter-confidence audit gate.

This gate enforces that the checked-in adapter-confidence audit stays honest:
every claimed surface binds one confidence label that keeps the source class and
confidence as distinct chips, shows the heuristic-fallback banner and its reason,
and keeps the overwrite decision and claim lineage inspectable; every claim's
priority rank binds to its source kind and its confidence stays at or below the
source ceiling; a heuristic label carries a banner and a fallback reason while a
non-heuristic label carries neither; each subject names the authoritative claim
the canonical arbitration picks and blocks every weaker, overwrite-attempting
re-report instead of letting it silently overwrite stronger truth; the
source-quality change agrees with the derived arbitration; and the support, CLI,
and AI projections preserve the label and the full lineage. It reads:

- the audit at ``artifacts/m5/tooling/adapter-confidence-audit/packet.json``;
- the support export, CLI/headless view, and AI evidence view alongside it;
- the boundary schemas at
  ``schemas/tooling/adapter-confidence-audit.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/adapter-confidence-and-fallback.md``.

The typed Rust consumer mints the same audit, so
``cargo test -p aureline-runtime --test m5_adapter_confidence_labels`` enforces
the same invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.

Exit codes:

- ``0`` -- audit is clean.
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

ARTIFACT_DIR = Path("artifacts/m5/tooling/adapter-confidence-audit")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
AUDIT_SCHEMA_REL = Path("schemas/tooling/adapter-confidence-audit.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/adapter-confidence-and-fallback.md")

EXPECTED_RECORD_KIND = "m5_adapter_confidence_audit"
EXPECTED_SUPPORT_RECORD_KIND = "m5_adapter_confidence_audit_support_export"
EXPECTED_CLI_RECORD_KIND = "m5_adapter_confidence_audit_cli_headless"
EXPECTED_AI_RECORD_KIND = "m5_adapter_confidence_audit_ai_evidence"
EXPECTED_SCHEMA_VERSION = 1

PRIORITY_RANK = {
    "native": 1,
    "bsp": 2,
    "bazel-bep": 3,
    "structured-output": 4,
    "heuristic-parser": 5,
}
CONFIDENCE_WEIGHT = {"high": 4, "medium-high": 3, "medium": 2, "low": 1}
CONFIDENCE_CEILING = {
    "native": "high",
    "bsp": "high",
    "bazel-bep": "high",
    "structured-output": "medium-high",
    "heuristic-parser": "low",
}
AUTHORITATIVE_SOURCES = {"native", "bsp", "bazel-bep"}
REQUIRED_SURFACES = {
    "task_center",
    "test_tree",
    "coverage_flaky",
    "pipeline_overlay",
    "notebook_run_history",
    "support_export",
    "cli_headless",
    "ai_evidence",
}
BINDING_CHIP_FLAGS = (
    "reads_canonical_label",
    "shows_source_class_chip",
    "shows_confidence_chip",
    "keeps_source_and_confidence_distinct",
)
BINDING_BANNER_FLAGS = ("shows_heuristic_fallback_banner", "shows_fallback_reason")
BINDING_LINEAGE_FLAGS = ("shows_overwrite_decision", "keeps_lineage_inspectable")

DOC_BACKLINKS = (
    "schemas/tooling/adapter-confidence-audit.schema.json",
    "artifacts/m5/tooling/adapter-confidence-audit/",
    "fixtures/tooling/m5/confidence-preservation/",
    "tools/ci/m5/adapter_confidence_audit_check.py",
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


def label_digest(tokens: list[str]) -> str:
    """Order-stable FNV-1a 64-bit digest, matching the Rust implementation."""
    mask = (1 << 64) - 1
    prime = 0x0000_0100_0000_01B3
    h = 0xCBF2_9CE4_8422_2325
    for token in sorted(tokens):
        for byte in token.encode("utf-8"):
            h ^= byte
            h = (h * prime) & mask
        h ^= 0x0A
        h = (h * prime) & mask
    return f"fnv1a64:{h:016x}"


def derived_digest(packet: dict[str, Any]) -> str:
    tokens: list[str] = []
    for resolution in packet.get("subjects", []):
        if not isinstance(resolution, dict):
            continue
        subject_id = str(resolution.get("subject", {}).get("subject_id"))
        for claim in resolution.get("claims", []):
            if not isinstance(claim, dict):
                continue
            label = claim.get("label", {})
            tokens.append(
                "|".join(
                    [
                        subject_id,
                        str(claim.get("claim_id")),
                        str(label.get("source_kind")),
                        str(label.get("confidence")),
                    ]
                )
            )
    return label_digest(tokens)


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
        findings.append(Finding("schema_version_mismatch", f"packet.schema_version must be {EXPECTED_SCHEMA_VERSION}"))
    for ref_field in ("audit_id", "generated_at"):
        if not str(packet.get(ref_field, "")).strip():
            findings.append(Finding("identity_missing", f"packet.{ref_field} must be non-empty"))
    for ref_field in ("audit_schema_ref", "envelope_schema_ref", "doc_ref", "policy_baseline_ref"):
        ref = packet.get(ref_field)
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding("schema_ref_missing", f"packet.{ref_field} must point at an existing path", detail={ref_field: ref})
            )
    if packet.get("promotion_state") != "stable":
        findings.append(
            Finding("promotion_not_stable", "packet.promotion_state must be stable", detail={"promotion_state": packet.get("promotion_state")})
        )
    if packet.get("validation_findings"):
        findings.append(
            Finding("validation_findings_present", "packet.validation_findings must be empty", detail={"count": len(packet.get("validation_findings", []))})
        )
    if packet.get("label_digest") != derived_digest(packet):
        findings.append(Finding("label_digest_drift", "packet.label_digest must match the claim history"))


def check_surface_bindings(packet: dict[str, Any], findings: list[Finding]) -> None:
    bindings = packet.get("surface_bindings")
    if not isinstance(bindings, list) or not bindings:
        findings.append(Finding("surface_bindings_missing", "packet.surface_bindings must be a non-empty array"))
        return
    present: set[str] = set()
    for binding in bindings:
        if not isinstance(binding, dict):
            continue
        surface = str(binding.get("surface"))
        present.add(surface)
        if not str(binding.get("binding_ref", "")).strip():
            findings.append(Finding("binding_ref_missing", f"binding for {surface} has no ref", subject=surface))
        if not all(binding.get(flag) is True for flag in BINDING_CHIP_FLAGS):
            findings.append(Finding("surface_collapses_label", f"surface {surface} merges source class and confidence", subject=surface))
        if not all(binding.get(flag) is True for flag in BINDING_BANNER_FLAGS):
            findings.append(Finding("surface_hides_banner", f"surface {surface} hides the heuristic-fallback banner", subject=surface))
        if not all(binding.get(flag) is True for flag in BINDING_LINEAGE_FLAGS):
            findings.append(Finding("surface_drops_lineage", f"surface {surface} drops the overwrite decision or lineage", subject=surface))
    for surface in sorted(REQUIRED_SURFACES - present):
        findings.append(Finding("surface_binding_missing", f"surface {surface} has no label binding", subject=surface))


def strength_key(claim: dict[str, Any]) -> tuple[int, int, str]:
    label = claim.get("label", {})
    rank = PRIORITY_RANK.get(str(label.get("source_kind")), 99)
    weight = CONFIDENCE_WEIGHT.get(str(label.get("confidence")), 0)
    return (rank, -weight, str(claim.get("claim_id")))


def canonical_decisions(claims: list[dict[str, Any]]) -> tuple[str, dict[str, str]]:
    """Returns the authoritative claim id and the canonical decision per claim."""
    ordered = sorted((c for c in claims if isinstance(c, dict)), key=strength_key)
    if not ordered:
        return "", {}
    authoritative = ordered[0]
    auth_id = str(authoritative.get("claim_id"))
    auth_rank = PRIORITY_RANK.get(str(authoritative.get("label", {}).get("source_kind")), 99)
    decisions: dict[str, str] = {}
    for claim in claims:
        claim_id = str(claim.get("claim_id"))
        if claim_id == auth_id:
            decisions[claim_id] = "accepted_authoritative"
        elif claim.get("attempts_overwrite") is True:
            decisions[claim_id] = "blocked_lower_confidence"
        else:
            decisions[claim_id] = "enriched_context_only"
    return auth_id, decisions


def derived_quality_change(resolution: dict[str, Any], auth_id: str, decisions: dict[str, str]) -> str:
    claims = resolution.get("claims", [])
    current = next(
        (str(c.get("label", {}).get("source_kind")) for c in claims if isinstance(c, dict) and str(c.get("claim_id")) == auth_id),
        "",
    )
    prior = resolution.get("subject", {}).get("prior_authoritative_source")
    if isinstance(prior, str) and prior:
        prior_rank = PRIORITY_RANK.get(prior, 99)
        current_rank = PRIORITY_RANK.get(current, 99)
        if current_rank < prior_rank:
            return "upgraded_to_authoritative"
        if current_rank > prior_rank:
            return "downgraded_to_fallback"
    values = decisions.values()
    if "blocked_lower_confidence" in values:
        return "overwrite_blocked"
    if "enriched_context_only" in values:
        return "enriched_without_overwrite"
    return "held_authoritative"


def check_subjects(packet: dict[str, Any], findings: list[Finding]) -> None:
    subjects = packet.get("subjects")
    if not isinstance(subjects, list) or not subjects:
        findings.append(Finding("subjects_missing", "packet.subjects must be a non-empty array"))
        return
    for resolution in subjects:
        if not isinstance(resolution, dict):
            continue
        check_subject(resolution, findings)


def check_subject(resolution: dict[str, Any], findings: list[Finding]) -> None:
    subject = resolution.get("subject", {})
    subject_id = str(subject.get("subject_id") or "<unknown>")
    claims = resolution.get("claims")
    if not isinstance(claims, list) or not claims:
        findings.append(Finding("subject_has_no_claims", f"subject {subject_id} carries no claims", subject=subject_id))
        return

    claim_ids = {str(c.get("claim_id")) for c in claims if isinstance(c, dict)}
    for claim in claims:
        if not isinstance(claim, dict):
            continue
        label = claim.get("label", {})
        source = str(label.get("source_kind"))
        confidence = str(label.get("confidence"))
        claim_id = str(claim.get("claim_id"))
        if claim.get("priority_rank") != PRIORITY_RANK.get(source):
            findings.append(Finding("claim_priority_mismatch", f"claim {claim_id} has a non-canonical priority rank", subject=subject_id))
        if CONFIDENCE_WEIGHT.get(confidence, 0) > CONFIDENCE_WEIGHT.get(CONFIDENCE_CEILING.get(source, "low"), 0):
            findings.append(Finding("claim_confidence_overclaim", f"claim {claim_id} overclaims confidence for {source}", subject=subject_id))
        heuristic = source == "heuristic-parser"
        banner = label.get("heuristic_fallback_banner") is True
        has_reason = bool(label.get("fallback_reason"))
        if banner != heuristic or has_reason != heuristic:
            findings.append(Finding("label_banner_inconsistent", f"claim {claim_id} has an inconsistent fallback banner", subject=subject_id))

    decisions = resolution.get("overwrite_decisions", [])
    for decision in decisions:
        if isinstance(decision, dict) and str(decision.get("claim_id")) not in claim_ids:
            findings.append(Finding("lineage_dropped", f"decision references dropped claim {decision.get('claim_id')}", subject=subject_id))

    auth_id, canonical = canonical_decisions(claims)
    if str(resolution.get("authoritative_claim_id")) != auth_id:
        findings.append(Finding("authoritative_claim_mismatch", f"subject {subject_id} names the wrong authoritative claim", subject=subject_id))
    auth_label = next(
        (c.get("label", {}) for c in claims if isinstance(c, dict) and str(c.get("claim_id")) == auth_id),
        {},
    )
    if str(resolution.get("current_authoritative_source")) != str(auth_label.get("source_kind")):
        findings.append(Finding("authoritative_source_mismatch", f"subject {subject_id} reports the wrong authoritative source", subject=subject_id))
    if str(resolution.get("current_confidence")) != str(auth_label.get("confidence")):
        findings.append(Finding("authoritative_confidence_mismatch", f"subject {subject_id} reports the wrong authoritative confidence", subject=subject_id))

    stored = {str(row.get("claim_id")): str(row.get("decision")) for row in decisions if isinstance(row, dict)}
    for claim_id, canonical_decision in canonical.items():
        if stored.get(claim_id) == canonical_decision:
            continue
        if canonical_decision == "blocked_lower_confidence":
            findings.append(Finding("lower_confidence_overwrite_accepted", f"subject {subject_id} let weaker claim {claim_id} overwrite stronger truth", subject=subject_id))
        else:
            findings.append(Finding("overwrite_decision_inconsistent", f"subject {subject_id} stores a non-canonical decision for {claim_id}", subject=subject_id))

    if str(resolution.get("source_quality_change")) != derived_quality_change(resolution, auth_id, canonical):
        findings.append(Finding("source_quality_change_mismatch", f"subject {subject_id} stores a non-canonical source-quality change", subject=subject_id))


def check_support_export(packet: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_SUPPORT_RECORD_KIND:
        findings.append(Finding("support_record_kind_mismatch", f"support_export.record_kind must be {EXPECTED_SUPPORT_RECORD_KIND}"))
    if export.get("audit_id_ref") != packet.get("audit_id"):
        findings.append(Finding("support_audit_ref_mismatch", "support_export.audit_id_ref must quote the audit id"))
    if export.get("label_digest") != packet.get("label_digest"):
        findings.append(Finding("support_digest_mismatch", "support_export.label_digest must match the audit"))
    if export.get("audit") != packet:
        findings.append(Finding("support_audit_drift", "support_export.audit must carry the exact audit"))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("audit_id_ref") != packet.get("audit_id"):
        findings.append(Finding("cli_audit_ref_mismatch", "cli_headless.audit_id_ref must quote the audit id"))
    if view.get("label_digest") != packet.get("label_digest"):
        findings.append(Finding("cli_digest_mismatch", "cli_headless.label_digest must match the audit"))
    expected_rows = sum(len(r.get("claims", [])) for r in packet.get("subjects", []) if isinstance(r, dict))
    rows = view.get("rows")
    if not isinstance(rows, list) or len(rows) != expected_rows:
        findings.append(Finding("cli_row_count_drift", "cli_headless.rows must carry one row per claim"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not str(row.get("source_kind", "")).strip() or not str(row.get("confidence", "")).strip() or not str(row.get("overwrite_decision", "")).strip():
            findings.append(Finding("cli_row_drops_label", "a CLI/headless row drops source, confidence, or decision", subject=row.get("claim_id")))
        if row.get("heuristic_fallback_banner") is True and not row.get("fallback_reason"):
            findings.append(Finding("cli_banner_without_reason", "a CLI/headless banner row drops its fallback reason", subject=row.get("claim_id")))


def check_ai_evidence(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_AI_RECORD_KIND:
        findings.append(Finding("ai_record_kind_mismatch", f"ai_evidence.record_kind must be {EXPECTED_AI_RECORD_KIND}"))
    if view.get("audit_id_ref") != packet.get("audit_id"):
        findings.append(Finding("ai_audit_ref_mismatch", "ai_evidence.audit_id_ref must quote the audit id"))
    if view.get("label_digest") != packet.get("label_digest"):
        findings.append(Finding("ai_digest_mismatch", "ai_evidence.label_digest must match the audit"))
    subjects = view.get("subjects")
    if not isinstance(subjects, list) or len(subjects) != len(packet.get("subjects", [])):
        findings.append(Finding("ai_subject_count_drift", "ai_evidence.subjects must carry one row per subject"))
        return
    for subject in subjects:
        if not isinstance(subject, dict):
            continue
        claims = subject.get("claims")
        if not isinstance(claims, list) or not claims:
            findings.append(Finding("ai_drops_lineage", "an AI evidence subject drops its claim lineage", subject=subject.get("subject_id")))
            continue
        for claim in claims:
            if not isinstance(claim, dict):
                continue
            if not str(claim.get("source_kind", "")).strip() or not str(claim.get("overwrite_decision", "")).strip() or not str(claim.get("raw_payload_ref", "")).strip():
                findings.append(Finding("ai_flattens_provenance", "an AI evidence claim drops source, decision, or raw ref", subject=subject.get("subject_id")))


def check_doc(repo_root: Path, findings: list[Finding]) -> None:
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink}))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    packet = ensure_dict(load_json(repo_root / PACKET_REL), "packet")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    cli = ensure_dict(load_json(repo_root / CLI_HEADLESS_REL), "cli_headless")
    ai = ensure_dict(load_json(repo_root / AI_EVIDENCE_REL), "ai_evidence")
    for schema_rel in (AUDIT_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_surface_bindings(packet, findings)
    check_subjects(packet, findings)
    check_support_export(packet, export, findings)
    check_cli_headless(packet, cli, findings)
    check_ai_evidence(packet, ai, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 adapter-confidence audit: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
