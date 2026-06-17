#!/usr/bin/env python3
"""Native-desktop per-desktop-profile qualification family CI gate.

This gate enforces that the checked-in native-desktop qualification family stays
fresh and clean across every claimed desktop profile (platform x delivery
channel). Each profile binds the seven canonical qualification dimensions
(channel_build_ownership, protocol_handler_ownership, file_association_ownership,
reopen_fidelity, notification_privacy, external_root_recovery,
store_lock_recovery) to the platform-conformance drill it is qualified by
(channel_ownership_audit, handler_conflict, wrong_target_reopen,
lock_screen_privacy, missing_root_recovery, store_lock), and the report derives
an auto-narrowing claim scope. It reads:

- the report fixture at
  ``fixtures/platform/m5-native-desktop-qualification/report.json``;
- the support-export fixture at
  ``fixtures/platform/m5-native-desktop-qualification/support_export.json``;
- the claim-packet fixture at
  ``fixtures/platform/m5-native-desktop-qualification/claim_packet.json``;
- the boundary schema at
  ``schemas/platform/m5-native-desktop-qualification.schema.json``; and
- (when present) the published markdown at
  ``artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md``,
  the companion doc at ``docs/m5/native-desktop-qualification.md``, and the
  shiproom claim packet.

For the report the gate verifies that:

- every required dimension is qualified by at least one profile, and every
  profile declares a binding for every required dimension;
- every profile carries a platform, a delivery channel, a channel/build owner,
  a trust checkpoint, a non-empty continuity note, a downgrade rule, and
  ``registered_on_qualification_harness = true``;
- every qualified dimension carries the drill it is qualified by, an evidence
  pack, and an evidence pack that names this profile (so no row borrows another
  profile's or channel's proof);
- every binding's ``required_drill`` matches the canonical drill for its
  dimension;
- no dimension carries a failed status, so the distinct failure classes
  (ownership_unprovable, protocol_handler_conflict, file_association_conflict,
  wrong_target_reopen, lock_screen_leak, missing_root_silent_loss,
  store_lock_dead_end) are all caught;
- no marketed dimension is left unqualified and no narrowed dimension is missing
  its narrowing reason;
- no marketed profile carries stale evidence;
- each profile's derived claim_state is never greener than its dimension
  qualification;
- no profile carries any blocking finding;
- the report cross-links the matrix, channel-ownership, handler-ownership,
  reopen, notification-privacy, external-root-recovery, and install-topology
  packets;
- the support-export wrapper quotes every profile id and descriptor revision;
  and
- the claim packet partitions every profile and is publishable only when the
  report is clean with no withheld profile.

Exit codes:

- ``0`` -- family is clean (all dimensions qualified, claim derived, no
  blockers).
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

REPORT_REL = Path("fixtures/platform/m5-native-desktop-qualification/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/platform/m5-native-desktop-qualification/support_export.json")
CLAIM_PACKET_REL = Path("fixtures/platform/m5-native-desktop-qualification/claim_packet.json")
SCHEMA_REL = Path("schemas/platform/m5-native-desktop-qualification.schema.json")
MARKDOWN_REL = Path(
    "artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md"
)
CLAIM_PACKET_MD_REL = Path(
    "artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md"
)
DOC_REL = Path("docs/m5/native-desktop-qualification.md")

REQUIRED_DIMENSIONS = (
    "channel_build_ownership",
    "protocol_handler_ownership",
    "file_association_ownership",
    "reopen_fidelity",
    "notification_privacy",
    "external_root_recovery",
    "store_lock_recovery",
)

REQUIRED_DRILLS = (
    "channel_ownership_audit",
    "handler_conflict",
    "wrong_target_reopen",
    "lock_screen_privacy",
    "missing_root_recovery",
    "store_lock",
)

CANONICAL_DRILL = {
    "channel_build_ownership": "channel_ownership_audit",
    "protocol_handler_ownership": "handler_conflict",
    "file_association_ownership": "handler_conflict",
    "reopen_fidelity": "wrong_target_reopen",
    "notification_privacy": "lock_screen_privacy",
    "external_root_recovery": "missing_root_recovery",
    "store_lock_recovery": "store_lock",
}

CANONICAL_FAILURE_MODE = {
    "channel_build_ownership": "ownership_unprovable",
    "protocol_handler_ownership": "protocol_handler_conflict",
    "file_association_ownership": "file_association_conflict",
    "reopen_fidelity": "wrong_target_reopen",
    "notification_privacy": "lock_screen_leak",
    "external_root_recovery": "missing_root_silent_loss",
    "store_lock_recovery": "store_lock_dead_end",
}

CLAIM_STATES = ("published", "narrowed", "withheld")

CROSS_LINK_FIELDS = (
    "native_desktop_matrix_ref",
    "channel_ownership_ref",
    "protocol_handler_ownership_ref",
    "file_association_ownership_ref",
    "reopen_corpus_ref",
    "notification_privacy_ref",
    "external_root_recovery_ref",
    "install_topology_ref",
)

EXPECTED_RECORD_KIND_REPORT = "shell_m5_native_desktop_qualification_report_record"
EXPECTED_RECORD_KIND_PROFILE = "shell_m5_native_desktop_qualification_profile_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_native_desktop_qualification_support_export_record"
EXPECTED_RECORD_KIND_CLAIM = "shell_m5_native_desktop_qualification_claim_packet_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_native_desktop_qualification:v1"
EXPECTED_SCHEMA_VERSION = 1

DOC_BACKLINKS = (
    "artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md",
    "fixtures/platform/m5-native-desktop-qualification/report.json",
    "schemas/platform/m5-native-desktop-qualification.schema.json",
    "tools/ci/m5/native_desktop_qualification_check.py",
    "artifacts/shiproom/m5-native-desktop-claim-packet/m5_native_desktop_claim_packet.md",
)


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    profile_id: str | None = None
    dimension: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.profile_id is not None:
            out["profile_id"] = self.profile_id
        if self.dimension is not None:
            out["dimension"] = self.dimension
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


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def check_report_envelope(report: dict[str, Any], findings: list[Finding]) -> None:
    if report.get("record_kind") != EXPECTED_RECORD_KIND_REPORT:
        findings.append(
            Finding(
                "report_record_kind_mismatch",
                f"report.record_kind must be {EXPECTED_RECORD_KIND_REPORT}",
                detail={"record_kind": report.get("record_kind")},
            )
        )
    if report.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "report_schema_version_mismatch",
                f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": report.get("schema_version")},
            )
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "report_shared_contract_ref_mismatch",
                f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
                detail={"shared_contract_ref": report.get("shared_contract_ref")},
            )
        )
    if report.get("required_dimensions") != list(REQUIRED_DIMENSIONS):
        findings.append(
            Finding(
                "required_dimensions_mismatch",
                "report.required_dimensions must equal the canonical dimension list",
                detail={"declared": report.get("required_dimensions")},
            )
        )
    if report.get("required_drills") != list(REQUIRED_DRILLS):
        findings.append(
            Finding(
                "required_drills_mismatch",
                "report.required_drills must equal the canonical drill list",
                detail={"declared": report.get("required_drills")},
            )
        )
    platforms = report.get("claimed_platforms")
    if not isinstance(platforms, list) or not platforms:
        findings.append(
            Finding(
                "claimed_platforms_missing",
                "report.claimed_platforms must be a non-empty array",
                detail={"claimed_platforms": platforms},
            )
        )
    for ref_field in (
        "published_report_ref",
        "published_doc_ref",
        "claim_packet_ref",
        "matrix_report_ref",
    ):
        ref = report.get(ref_field)
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    "publication_ref_missing",
                    f"report.{ref_field} must be a non-empty string",
                    detail={ref_field: ref},
                )
            )
    if report.get("report_clean") is not True:
        findings.append(
            Finding(
                "report_not_clean",
                "report.report_clean must be true",
                detail={"report_clean": report.get("report_clean")},
            )
        )


def check_cross_links(report: dict[str, Any], findings: list[Finding]) -> None:
    cross_links = report.get("cross_links")
    if not isinstance(cross_links, dict):
        findings.append(Finding("cross_links_missing", "report.cross_links must be an object"))
        return
    for field_name in CROSS_LINK_FIELDS:
        value = cross_links.get(field_name)
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    "cross_link_missing",
                    "report.cross_links must carry every upstream packet ref",
                    detail={"field": field_name},
                )
            )


def check_required_coverage(report: dict[str, Any], findings: list[Finding]) -> None:
    profiles = ensure_list(report.get("profiles", []), "report.profiles")
    for dimension in REQUIRED_DIMENSIONS:
        any_qualified = any(
            binding.get("dimension") == dimension and binding.get("status") == "qualified"
            for profile in profiles
            for binding in ensure_list(profile.get("bindings", []), "profile.bindings")
        )
        if not any_qualified:
            findings.append(
                Finding(
                    "required_dimension_not_qualified",
                    "no qualified profile for required dimension",
                    dimension=dimension,
                )
            )


def derive_claim_state(profile: dict[str, Any]) -> str:
    descriptor = ensure_dict(profile.get("descriptor", {}), "profile.descriptor")
    bindings = ensure_list(profile.get("bindings", []), "profile.bindings")
    marketed = bool(descriptor.get("marketed"))
    qualified = sum(1 for b in bindings if b.get("status") == "qualified")
    blocked = [
        b.get("dimension")
        for b in bindings
        if b.get("status") in ("failed", "unqualified")
    ]
    narrowed = [
        b.get("dimension")
        for b in bindings
        if b.get("status") in ("explicitly_narrowed", "not_applicable")
    ]
    has_blockers = bool(profile.get("blocking_findings"))
    stale = descriptor.get("evidence_freshness") == "stale"

    if not marketed:
        return "withheld"
    if qualified == 0:
        return "withheld"
    if blocked or has_blockers or stale:
        return "narrowed"
    if narrowed:
        return "narrowed"
    return "published"


def check_binding(profile_id: str, marketed: bool, binding: dict[str, Any], findings: list[Finding]) -> None:
    dimension = binding.get("dimension")
    status = binding.get("status")

    expected_drill = CANONICAL_DRILL.get(dimension)
    if expected_drill is not None and binding.get("required_drill") != expected_drill:
        findings.append(
            Finding(
                "drill_kind_drift",
                "binding declares the wrong drill for its dimension",
                profile_id,
                dimension,
                detail={"required_drill": binding.get("required_drill"), "expected": expected_drill},
            )
        )

    if status == "failed":
        findings.append(
            Finding(
                "dimension_failed",
                "dimension carries a failed status",
                profile_id,
                dimension,
                detail={"failure_mode": binding.get("failure_mode")},
            )
        )
        if binding.get("failure_mode") != CANONICAL_FAILURE_MODE.get(dimension):
            findings.append(
                Finding("failure_mode_drift", "failed dimension declares the wrong failure mode", profile_id, dimension)
            )
        if not binding.get("drill_ref"):
            findings.append(
                Finding("missing_drill_ref", "failed dimension is missing a drill ref", profile_id, dimension)
            )
    elif status == "unqualified":
        if marketed:
            findings.append(
                Finding("unqualified_marketed_dimension", "marketed dimension is claimed but unproven", profile_id, dimension)
            )
        if binding.get("failure_mode") is not None:
            findings.append(
                Finding("failure_mode_drift", "unqualified dimension declares a failure mode", profile_id, dimension)
            )
    elif status == "qualified":
        if binding.get("failure_mode") is not None:
            findings.append(
                Finding("failure_mode_drift", "qualified dimension declares a failure mode", profile_id, dimension)
            )
        if not binding.get("drill_ref"):
            findings.append(
                Finding("missing_drill_ref", "qualified dimension is missing a drill ref", profile_id, dimension)
            )
        evidence = binding.get("evidence_pack_ref")
        if not evidence:
            findings.append(
                Finding("missing_evidence_pack", "qualified dimension is missing an evidence pack", profile_id, dimension)
            )
        elif profile_id not in evidence:
            findings.append(
                Finding(
                    "borrowed_proof_across_profile",
                    "qualified dimension's evidence pack does not name this profile",
                    profile_id,
                    dimension,
                    detail={"evidence_pack_ref": evidence},
                )
            )
    elif status in ("not_applicable", "explicitly_narrowed"):
        reason = binding.get("narrowing_reason")
        if not isinstance(reason, str) or not reason.strip():
            findings.append(
                Finding("missing_narrowing_reason", "narrowed dimension is missing a narrowing reason", profile_id, dimension)
            )
    else:
        findings.append(
            Finding("unknown_status", "dimension carries an unknown status", profile_id, dimension, detail={"status": status})
        )


def check_profile(profile: dict[str, Any], findings: list[Finding]) -> None:
    descriptor = ensure_dict(profile.get("descriptor", {}), "profile.descriptor")
    profile_id = descriptor.get("profile_id")
    if not isinstance(profile_id, str) or not profile_id.strip():
        findings.append(Finding("missing_profile_id", "descriptor.profile_id must be non-empty"))
        return

    if profile.get("record_kind") != EXPECTED_RECORD_KIND_PROFILE:
        findings.append(
            Finding(
                "profile_record_kind_mismatch",
                f"profile.record_kind must be {EXPECTED_RECORD_KIND_PROFILE}",
                profile_id=profile_id,
                detail={"record_kind": profile.get("record_kind")},
            )
        )

    required_string_fields = {
        "descriptor_revision_ref": "missing_descriptor_revision",
        "channel_build_owner_ref": "missing_channel_build_owner",
        "trust_checkpoint_ref": "missing_trust_checkpoint",
        "continuity_note": "missing_continuity_note",
        "downgrade_rule_ref": "missing_downgrade_rule",
    }
    for field_name, code in required_string_fields.items():
        value = descriptor.get(field_name)
        if not isinstance(value, str) or not value.strip():
            findings.append(Finding(code, f"descriptor.{field_name} must be non-empty", profile_id=profile_id))

    for field_name, code in (("platform", "missing_platform"), ("channel", "missing_channel")):
        if not descriptor.get(field_name):
            findings.append(Finding(code, f"descriptor.{field_name} must be set", profile_id=profile_id))

    if descriptor.get("registered_on_qualification_harness") is not True:
        findings.append(
            Finding("profile_not_on_harness", "descriptor.registered_on_qualification_harness must be true", profile_id=profile_id)
        )

    marketed = bool(descriptor.get("marketed"))
    if marketed and descriptor.get("evidence_freshness") == "stale":
        findings.append(
            Finding("stale_evidence_on_marketed_profile", "marketed profile carries stale evidence", profile_id=profile_id)
        )

    bindings = ensure_list(profile.get("bindings", []), "profile.bindings")
    present_dimensions = {binding.get("dimension") for binding in bindings}
    for dimension in REQUIRED_DIMENSIONS:
        if dimension not in present_dimensions:
            findings.append(
                Finding("missing_required_dimension", "profile is missing a required dimension binding", profile_id=profile_id, dimension=dimension)
            )
    for binding in bindings:
        check_binding(profile_id, marketed, binding, findings)

    declared_claim = profile.get("claim_state")
    derived_claim = derive_claim_state(profile)
    if declared_claim != derived_claim:
        findings.append(
            Finding(
                "claim_state_drift",
                "profile claim_state must be derived from its dimension qualification",
                profile_id=profile_id,
                detail={"declared": declared_claim, "derived": derived_claim},
            )
        )

    for blocker in ensure_list(profile.get("blocking_findings", []), "profile.blocking_findings"):
        findings.append(
            Finding(
                "blocking_finding_present",
                "profile carries a blocking finding",
                profile_id=profile_id,
                dimension=blocker.get("dimension"),
                detail={"class": blocker.get("class")},
            )
        )


def check_claim_scope(report: dict[str, Any], findings: list[Finding]) -> None:
    profiles = ensure_list(report.get("profiles", []), "report.profiles")
    claim_scope = ensure_list(report.get("claim_scope", []), "report.claim_scope")
    scope_by_id = {
        ensure_dict(claim, "claim_scope entry").get("profile_id"): claim for claim in claim_scope
    }
    for profile in profiles:
        descriptor = ensure_dict(profile.get("descriptor", {}), "profile.descriptor")
        profile_id = descriptor.get("profile_id")
        claim = scope_by_id.get(profile_id)
        if claim is None:
            findings.append(Finding("claim_scope_missing_profile", "claim_scope must cover every profile", profile_id=profile_id))
            continue
        if claim.get("claim_state") not in CLAIM_STATES:
            findings.append(
                Finding("claim_state_invalid", "claim_scope entry has an invalid claim_state", profile_id=profile_id, detail={"claim_state": claim.get("claim_state")})
            )
        if claim.get("claim_state") != profile.get("claim_state"):
            findings.append(
                Finding(
                    "claim_scope_state_mismatch",
                    "claim_scope state must match the profile row claim_state",
                    profile_id=profile_id,
                    detail={"scope": claim.get("claim_state"), "row": profile.get("claim_state")},
                )
            )

    # Published/narrowed/withheld counts must agree with the claim scope.
    expected_counts = {state: 0 for state in CLAIM_STATES}
    for claim in claim_scope:
        state = claim.get("claim_state")
        if state in expected_counts:
            expected_counts[state] += 1
    count_fields = {
        "published": "published_claim_count",
        "narrowed": "narrowed_claim_count",
        "withheld": "withheld_claim_count",
    }
    for state, field_name in count_fields.items():
        if report.get(field_name) != expected_counts[state]:
            findings.append(
                Finding(
                    "claim_count_mismatch",
                    f"report.{field_name} must match the claim scope",
                    detail={"declared": report.get(field_name), "expected": expected_counts[state]},
                )
            )


def check_support_export(report: dict[str, Any], export: dict[str, Any], findings: list[Finding]) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": export.get("record_kind")},
            )
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(Finding("support_case_ids_missing", "support_export.case_ids must be an array"))
        return
    case_set = set(case_ids)
    if report.get("report_id") not in case_set:
        findings.append(
            Finding("support_missing_report_id", "support_export.case_ids must quote the report id", detail={"report_id": report.get("report_id")})
        )
    for profile in ensure_list(report.get("profiles", []), "report.profiles"):
        descriptor = ensure_dict(profile.get("descriptor", {}), "profile.descriptor")
        profile_id = descriptor.get("profile_id")
        revision = descriptor.get("descriptor_revision_ref")
        if profile_id not in case_set:
            findings.append(Finding("support_missing_profile_id", "support_export.case_ids must quote every profile id", profile_id=profile_id))
        if revision not in case_set:
            findings.append(
                Finding("support_missing_descriptor_revision", "support_export.case_ids must quote every descriptor revision", profile_id=profile_id, detail={"descriptor_revision_ref": revision})
            )


def check_claim_packet(report: dict[str, Any], packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != EXPECTED_RECORD_KIND_CLAIM:
        findings.append(
            Finding(
                "claim_packet_record_kind_mismatch",
                f"claim_packet.record_kind must be {EXPECTED_RECORD_KIND_CLAIM}",
                detail={"record_kind": packet.get("record_kind")},
            )
        )

    publishable = set(packet.get("publishable_profiles") or [])
    narrowed = set(packet.get("narrowed_profiles") or [])
    withheld = set(packet.get("withheld_profiles") or [])
    expected = {"published": set(), "narrowed": set(), "withheld": set()}
    for claim in ensure_list(report.get("claim_scope", []), "report.claim_scope"):
        state = claim.get("claim_state")
        if state in expected:
            expected[state].add(claim.get("profile_id"))
    if publishable != expected["published"]:
        findings.append(Finding("claim_packet_publishable_mismatch", "claim_packet.publishable_profiles must match the published claim scope", detail={"declared": sorted(publishable), "expected": sorted(expected["published"])}))
    if narrowed != expected["narrowed"]:
        findings.append(Finding("claim_packet_narrowed_mismatch", "claim_packet.narrowed_profiles must match the narrowed claim scope", detail={"declared": sorted(narrowed), "expected": sorted(expected["narrowed"])}))
    if withheld != expected["withheld"]:
        findings.append(Finding("claim_packet_withheld_mismatch", "claim_packet.withheld_profiles must match the withheld claim scope", detail={"declared": sorted(withheld), "expected": sorted(expected["withheld"])}))

    expected_publishable = report.get("report_clean") is True and not expected["withheld"]
    if packet.get("claim_publishable") is not expected_publishable:
        findings.append(
            Finding(
                "claim_publishable_mismatch",
                "claim_packet.claim_publishable must be true only when the report is clean with no withheld profile",
                detail={"declared": packet.get("claim_publishable"), "expected": expected_publishable},
            )
        )

    downgrade_rules = ensure_list(packet.get("downgrade_rules", []), "claim_packet.downgrade_rules")
    rule_ids = {ensure_dict(rule, "downgrade rule").get("profile_id") for rule in downgrade_rules}
    for profile in ensure_list(report.get("profiles", []), "report.profiles"):
        descriptor = ensure_dict(profile.get("descriptor", {}), "profile.descriptor")
        if descriptor.get("profile_id") not in rule_ids:
            findings.append(
                Finding("claim_packet_missing_downgrade_rule", "claim_packet.downgrade_rules must cover every profile", profile_id=descriptor.get("profile_id"))
            )


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}"))
    packet_md = repo_root / CLAIM_PACKET_MD_REL
    if not packet_md.exists():
        findings.append(Finding("claim_packet_md_missing", f"missing shiproom claim packet: {CLAIM_PACKET_MD_REL}"))
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for dimension in REQUIRED_DIMENSIONS:
        if dimension not in body:
            findings.append(Finding("doc_missing_dimension", "companion doc must quote every required dimension", detail={"dimension": dimension}))
    for drill in REQUIRED_DRILLS:
        if drill not in body:
            findings.append(Finding("doc_missing_drill", "companion doc must quote every required drill", detail={"drill": drill}))
    for state in CLAIM_STATES:
        if state not in body:
            findings.append(Finding("doc_missing_claim_state", "companion doc must quote every claim state", detail={"claim_state": state}))
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(Finding("doc_missing_backlink", "companion doc must back-link the canonical artifacts and gate", detail={"backlink": backlink}))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    report = ensure_dict(load_json(repo_root / REPORT_REL), "report")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    packet = ensure_dict(load_json(repo_root / CLAIM_PACKET_REL), "claim_packet")
    if not (repo_root / SCHEMA_REL).exists():
        raise SystemExit(f"missing required input: {SCHEMA_REL}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    check_cross_links(report, findings)
    check_required_coverage(report, findings)
    for profile in ensure_list(report.get("profiles", []), "report.profiles"):
        check_profile(ensure_dict(profile, "profile"), findings)
    check_claim_scope(report, findings)
    check_support_export(report, export, findings)
    check_claim_packet(report, packet, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 native-desktop qualification: clean")
        else:
            for finding in findings:
                location = finding.profile_id or "report"
                if finding.dimension:
                    location = f"{location} / {finding.dimension}"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
