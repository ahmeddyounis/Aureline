#!/usr/bin/env python3
"""M5 event-interop tooling-profile certification gate.

This gate enforces that the checked-in event-interop certification packet stays
honest: every claimed M5 run/test/debug/pipeline/notebook/coverage profile is
present once, reads the canonical event envelope, cites machine-readable upstream
evidence, and certifies across the eight certification dimensions (event-envelope
reuse, adapter hierarchy, fallback reason, confidence preservation, raw-payload
retention, replay stability, degraded-state disclosure, and export parity). A
profile that sources truth outside the canonical event envelope, cites no
evidence, overclaims confidence, loses its raw payload, drops a fallback reason,
hides a degraded state, breaks replay, or breaks export parity blocks stable; a
profile whose proof has aged past its freshness window narrows below stable. It
reads:

- the packet at ``artifacts/m5/tooling/event-interop-certification/packet.json``;
- the support export, AI evidence join, incident packet join, and CLI/headless
  view alongside it;
- the boundary schemas at
  ``schemas/tooling/m5-event-interop-certification.schema.json`` and
  ``schemas/tooling/task-event-envelope.schema.json``; and
- the companion doc at ``docs/m5/event-interop-certification.md``.

The typed Rust consumer mints the same packet, so
``cargo test -p aureline-runtime --test m5_event_interop_certification`` enforces
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

ARTIFACT_DIR = Path("artifacts/m5/tooling/event-interop-certification")
PACKET_REL = ARTIFACT_DIR / "packet.json"
SUPPORT_EXPORT_REL = ARTIFACT_DIR / "support_export.json"
AI_EVIDENCE_REL = ARTIFACT_DIR / "ai_evidence.json"
INCIDENT_PACKET_REL = ARTIFACT_DIR / "incident_packet.json"
CLI_HEADLESS_REL = ARTIFACT_DIR / "cli_headless.json"
CERTIFICATION_SCHEMA_REL = Path("schemas/tooling/m5-event-interop-certification.schema.json")
ENVELOPE_SCHEMA_REL = Path("schemas/tooling/task-event-envelope.schema.json")
DOC_REL = Path("docs/m5/event-interop-certification.md")

EXPECTED_RECORD_KIND = "m5_event_interop_certification_packet"
EXPECTED_SUPPORT_RECORD_KIND = "m5_event_interop_certification_support_export"
EXPECTED_EVIDENCE_RECORD_KIND = "m5_event_interop_certification_evidence_join"
EXPECTED_CLI_RECORD_KIND = "m5_event_interop_certification_cli_headless"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_PROFILES = [
    "task_center_run",
    "test_session",
    "debug_session",
    "pipeline_overlay",
    "notebook_run",
    "coverage_intelligence",
]

DIMENSIONS = (
    "event_envelope_reuse",
    "adapter_hierarchy",
    "fallback_reason",
    "confidence_preservation",
    "raw_payload_retention",
    "replay_stability",
    "degraded_state_disclosure",
    "export_parity",
)

DIMENSION_FINDING = {
    "adapter_hierarchy": ("adapter_hierarchy_missing", "a profile evidences no native-first capability handshake"),
    "confidence_preservation": ("confidence_overclaim", "a profile overclaims confidence for its source/capability"),
    "raw_payload_retention": ("raw_payload_not_retained", "a profile does not retain its raw payload safely"),
    "replay_stability": ("replay_unstable", "a profile does not replay deterministically"),
    "degraded_state_disclosure": ("degraded_state_not_disclosed", "a profile hides its degraded/unsupported state"),
    "export_parity": ("export_parity_broken", "a profile breaks support/release/AI export parity"),
}

DOC_BACKLINKS = (
    "schemas/tooling/m5-event-interop-certification.schema.json",
    "artifacts/m5/tooling/event-interop-certification/",
    "fixtures/tooling/m5/event-interop-certification/",
    "tools/ci/m5/event_interop_certification_check.py",
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


def profile_digest(profile_tokens: list[str]) -> str:
    """Order-stable FNV-1a 64-bit digest over sorted profile tokens, matching Rust."""
    mask = (1 << 64) - 1
    prime = 0x0000_0100_0000_01B3
    h = 0xCBF2_9CE4_8422_2325
    for token in sorted(profile_tokens):
        for byte in token.encode("utf-8"):
            h ^= byte
            h = (h * prime) & mask
        h ^= 0x0A
        h = (h * prime) & mask
    return f"fnv1a64:{h:016x}"


def profile_tokens(profiles: list[dict[str, Any]]) -> list[str]:
    return [str(p.get("profile")) for p in profiles if isinstance(p, dict)]


def evidence_nonempty(profile: dict[str, Any]) -> bool:
    refs = profile.get("evidence_refs") or []
    return any(isinstance(ref, str) and ref.strip() for ref in refs)


def evaluate_profile(profile: dict[str, Any]) -> dict[str, bool]:
    """Re-derives the per-dimension pass/fail, mirroring the Rust evaluator."""
    capability = profile.get("negotiated_capability")
    requires_fallback = capability in ("degraded", "unsupported")
    reason = profile.get("fallback_reason")
    reason_present = bool(isinstance(reason, str) and reason.strip())
    must_be_low = (
        profile.get("primary_source_kind") == "heuristic-parser" or capability == "unsupported"
    )
    truth_conformant = profile.get("consumer_truth_source") == "canonical_event_envelope"
    return {
        "event_envelope_reuse": truth_conformant and evidence_nonempty(profile),
        "adapter_hierarchy": bool(str(profile.get("capability_packet_ref", "")).strip()),
        "fallback_reason": reason_present if requires_fallback else not reason_present,
        "confidence_preservation": not (must_be_low and profile.get("observed_confidence") != "low"),
        "raw_payload_retention": bool(str(profile.get("raw_payload_ref", "")).strip())
        and bool(str(profile.get("payload_digest", "")).strip())
        and profile.get("raw_private_material_excluded") is True,
        "replay_stability": profile.get("replay_stable") is True,
        "degraded_state_disclosure": (not requires_fallback)
        or profile.get("degraded_state_disclosed") is True,
        "export_parity": profile.get("export_parity_preserved") is True,
    }


def claim_state(certified: bool, freshness: str) -> str:
    if not certified:
        return "blocked"
    if freshness == "stale":
        return "narrowed_below_stable"
    return "claimable"


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
        "certification_schema_ref",
        "envelope_schema_ref",
        "doc_ref",
        "policy_baseline_ref",
        "interop_packet_ref",
        "conformance_packet_ref",
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
    profiles = packet.get("profiles") if isinstance(packet.get("profiles"), list) else []
    if packet.get("profile_digest") != profile_digest(profile_tokens(profiles)):
        findings.append(Finding("profile_digest_drift", "packet.profile_digest must match the profiles"))


def check_profiles(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    profiles = packet.get("profiles")
    if not isinstance(profiles, list) or not profiles:
        findings.append(Finding("profiles_missing", "packet.profiles must be a non-empty array"))
        return
    present: dict[str, int] = {}
    for profile in profiles:
        if isinstance(profile, dict):
            token = str(profile.get("profile"))
            present[token] = present.get(token, 0) + 1
    for token in REQUIRED_PROFILES:
        count = present.get(token, 0)
        if count == 0:
            findings.append(Finding("missing_profile", "a required tooling profile is absent", subject=token))
        elif count > 1:
            findings.append(Finding("duplicate_profile", "a tooling profile is declared more than once", subject=token))

    for profile in profiles:
        if isinstance(profile, dict):
            check_profile(repo_root, profile, findings)


def check_profile(repo_root: Path, profile: dict[str, Any], findings: list[Finding]) -> None:
    token = str(profile.get("profile") or "<unknown>")

    for ref in profile.get("evidence_refs") or []:
        if not isinstance(ref, str) or not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "evidence_ref_missing",
                    "a profile cites an evidence ref that does not exist on disk",
                    subject=token,
                    detail={"evidence_ref": ref},
                )
            )

    outcomes = evaluate_profile(profile)
    capability = profile.get("negotiated_capability")
    requires_fallback = capability in ("degraded", "unsupported")
    truth_conformant = profile.get("consumer_truth_source") == "canonical_event_envelope"
    for dimension, passed in outcomes.items():
        if passed:
            continue
        if dimension == "event_envelope_reuse":
            if not truth_conformant:
                findings.append(Finding("event_envelope_not_reused", "a profile sources truth outside the canonical event envelope", subject=token))
            else:
                findings.append(Finding("missing_evidence_ref", "a profile cites no upstream evidence packet", subject=token))
            continue
        if dimension == "fallback_reason":
            if requires_fallback:
                findings.append(Finding("fallback_reason_missing", "a degraded/unsupported profile names no fallback reason", subject=token))
            else:
                findings.append(Finding("fallback_reason_unexpected", "a negotiated profile names a spurious fallback reason", subject=token))
            continue
        code, message = DIMENSION_FINDING[dimension]
        findings.append(Finding(code, message, subject=token))

    age = profile.get("proof_age_days")
    window = profile.get("freshness_window_days")
    if isinstance(age, int) and isinstance(window, int):
        expected_freshness = "stale" if age > window else "current"
        if profile.get("freshness_state") != expected_freshness:
            findings.append(Finding("profile_freshness_drift", "a profile freshness state disagrees with proof age", subject=token))
        if expected_freshness == "stale":
            findings.append(Finding("profile_evidence_stale", "a profile proof has aged past its window", subject=token))
    else:
        expected_freshness = None
        findings.append(Finding("profile_freshness_drift", "a profile has non-integer proof age/window", subject=token))

    certified_expected = all(outcomes.values())
    if profile.get("certified") is not certified_expected:
        findings.append(Finding("profile_certification_drift", "a profile certified flag disagrees with the derivation", subject=token))

    if expected_freshness is not None:
        expected_claim = claim_state(certified_expected, expected_freshness)
        if profile.get("claim_state") != expected_claim:
            findings.append(Finding("profile_claim_state_drift", "a profile claim state disagrees with the derivation", subject=token))

    stored = profile.get("dimension_outcomes")
    if not isinstance(stored, list) or len(stored) != len(DIMENSIONS):
        findings.append(Finding("dimension_outcome_drift", "a profile stored the wrong number of dimension outcomes", subject=token))
    else:
        for index, dimension in enumerate(DIMENSIONS):
            row = stored[index]
            if not isinstance(row, dict) or row.get("dimension") != dimension:
                findings.append(Finding("dimension_outcome_drift", "a profile dimension outcome is out of order", subject=token))
            elif row.get("passed") is not outcomes[dimension]:
                findings.append(Finding("dimension_outcome_drift", "a profile dimension outcome disagrees with the derivation", subject=f"{token}:{dimension}"))


def derive_index(profiles: list[dict[str, Any]]) -> dict[str, Any]:
    all_current = all(p.get("freshness_state") == "current" for p in profiles if isinstance(p, dict))
    all_certified = bool(profiles) and all(p.get("certified") is True for p in profiles if isinstance(p, dict))
    claimable = [str(p.get("profile")) for p in profiles if isinstance(p, dict) and p.get("claim_state") == "claimable"]
    narrowed = [str(p.get("profile")) for p in profiles if isinstance(p, dict) and p.get("claim_state") == "narrowed_below_stable"]
    blocked = [str(p.get("profile")) for p in profiles if isinstance(p, dict) and p.get("claim_state") == "blocked"]
    return {
        "all_profiles_current": all_current,
        "all_profiles_certified": all_certified,
        "claimable_profiles": claimable,
        "narrowed_profiles": narrowed,
        "blocked_profiles": blocked,
    }


def check_certification_index(packet: dict[str, Any], findings: list[Finding]) -> None:
    index = packet.get("certification_index")
    if not isinstance(index, dict):
        findings.append(Finding("certification_index_missing", "packet.certification_index must be an object"))
        return
    if not str(index.get("certification_ref", "")).strip():
        findings.append(Finding("certification_index_missing", "certification_index.certification_ref must be non-empty"))
    if not str(index.get("certification_summary", "")).strip():
        findings.append(Finding("certification_index_drift", "certification_index.certification_summary must be non-empty"))
    profiles = packet.get("profiles") if isinstance(packet.get("profiles"), list) else []
    expected = derive_index(profiles)
    if index.get("all_profiles_current") is not expected["all_profiles_current"]:
        findings.append(Finding("certification_index_drift", "certification_index.all_profiles_current disagrees with the profiles"))
    if index.get("all_profiles_certified") is not expected["all_profiles_certified"]:
        findings.append(Finding("certification_index_drift", "certification_index.all_profiles_certified disagrees with the profiles"))
    for key in ("claimable_profiles", "narrowed_profiles", "blocked_profiles"):
        if list(index.get(key) or []) != expected[key]:
            findings.append(Finding("certification_index_drift", f"certification_index.{key} disagrees with the profiles"))


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
    if view.get("profile_digest") != packet.get("profile_digest"):
        findings.append(Finding("evidence_profile_digest_drift", f"{label}.profile_digest must match the packet"))
    if view.get("certification_index") != packet.get("certification_index"):
        findings.append(Finding("evidence_index_drift", f"{label}.certification_index must match the packet"))

    profiles = packet.get("profiles") if isinstance(packet.get("profiles"), list) else []
    rows = view.get("profile_rows") if isinstance(view.get("profile_rows"), list) else []
    if len(rows) != len(profiles):
        findings.append(Finding("evidence_profile_row_drift", f"{label} must carry one profile row per profile"))
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not str(row.get("source_kind", "")).strip() or not str(row.get("explanation", "")).strip() or not str(row.get("claim_summary", "")).strip():
            findings.append(Finding("evidence_flattens_provenance", f"{label} profile row drops source, claim, or explanation", subject=row.get("profile")))


def check_cli_headless(packet: dict[str, Any], view: dict[str, Any], findings: list[Finding]) -> None:
    if view.get("record_kind") != EXPECTED_CLI_RECORD_KIND:
        findings.append(Finding("cli_record_kind_mismatch", f"cli_headless.record_kind must be {EXPECTED_CLI_RECORD_KIND}"))
    if view.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("cli_schema_version_mismatch", "cli_headless.schema_version mismatch"))
    if view.get("packet_id_ref") != packet.get("packet_id"):
        findings.append(Finding("cli_packet_ref_mismatch", "cli_headless.packet_id_ref must quote the packet id"))
    if view.get("profile_digest") != packet.get("profile_digest"):
        findings.append(Finding("cli_profile_digest_drift", "cli_headless.profile_digest must match the packet"))
    if view.get("promotion_state") != packet.get("promotion_state"):
        findings.append(Finding("cli_promotion_drift", "cli_headless.promotion_state must match the packet"))
    profiles = packet.get("profiles") if isinstance(packet.get("profiles"), list) else []
    rows = view.get("profile_rows")
    if not isinstance(rows, list) or len(rows) != len(profiles):
        findings.append(Finding("cli_profile_row_drift", "cli_headless.profile_rows must carry one row per profile"))
        return
    for row in rows:
        if not isinstance(row, dict):
            continue
        if not (isinstance(row.get("evidence_ref_count"), int) and row.get("evidence_ref_count") > 0) or not str(row.get("explanation", "")).strip():
            findings.append(Finding("cli_profile_unexplained", "a CLI/headless profile row cites no evidence or is unexplained", subject=row.get("profile")))


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
    for schema_rel in (CERTIFICATION_SCHEMA_REL, ENVELOPE_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_packet_block(repo_root, packet, findings)
    check_profiles(repo_root, packet, findings)
    check_certification_index(packet, findings)
    check_support_export(packet, export, findings)
    check_evidence_join(packet, ai_evidence, "ai_evidence", findings)
    check_evidence_join(packet, incident, "incident_packet", findings)
    check_cli_headless(packet, view, findings)
    check_doc(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 event-interop certification: clean")
        else:
            for finding in findings:
                location = finding.subject or "packet"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
