#!/usr/bin/env python3
"""Validate the M0 exit signoff packet and its governing artifacts.

This tool is intentionally stdlib-only. It checks the shared signoff packet,
the existing M0 architecture/governance artifacts, freshness metadata, and
the signoff-specific contract-family crosswalks that must stay visible across
the architecture pack, compatibility inventory, QE lane registry, assurance
claim matrix, and public-proof coverage report.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

PACKET_JSON_REL = "artifacts/milestones/M0_signoff_packet.json"
PACKET_MD_REL = "artifacts/milestones/M0_signoff_packet.md"
PACKET_TEMPLATE_REL = "artifacts/milestones/M0_signoff_template.md"
CHECKLIST_REL = "docs/milestones/M0_signoff_checklist.md"

ARCH_PACK_README_REL = "artifacts/milestones/M0_architecture_pack/README.md"
ARCH_PACK_PACKET_REL = "artifacts/milestones/M0_architecture_pack/packet_index.yaml"
ARCH_COVERAGE_REL = "artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml"
SCORECARD_REL = "artifacts/milestones/M0_scorecard.yaml"
CONTROL_ARTIFACT_INDEX_REL = "artifacts/governance/control_artifact_index.yaml"
DECISION_INDEX_REL = "artifacts/governance/decision_index.yaml"
OWNERSHIP_REL = "artifacts/governance/ownership_matrix.yaml"
CODEOWNERS_REL = "CODEOWNERS"
DEPENDENCY_REGISTER_REL = "artifacts/governance/dependency_register.yaml"
QUAL_MATRIX_REL = "artifacts/compat/qualification_matrix_seed.yaml"
VERSION_SKEW_REL = "artifacts/compat/version_skew_register.yaml"
INSTALL_TOPOLOGY_REL = "artifacts/release/install_topology_matrix.yaml"
FEATURE_FLAG_POLICY_REL = "docs/governance/feature_flag_policy.md"
BENCH_WORKFLOW_REL = ".github/workflows/nightly_benchmark.yml"
BENCH_DASHBOARD_REL = "artifacts/benchmarks/dashboard_seed/dashboard.json"
BENCH_CHARTER_REL = "docs/governance/benchmark_council_charter.md"
RENDER_ADR_REL = "docs/adr/0002-renderer-text-stack-and-shaping-fallback.md"
RENDER_SPIKE_REL = "artifacts/render/spike_capabilities.json"
RENDER_TRACE_REL = "artifacts/render/spike_trace_samples/full_scene.json"
EVIDENCE_FIELDS_REL = "artifacts/evidence/evidence_metadata_fields.yaml"
BOUNDARY_MANIFEST_REL = "docs/product/boundary_manifest_strawman.md"
CONTINUITY_REL = "artifacts/support/deployment_drill_catalog_seed.yaml"
ROUTE_TAXONOMY_REL = "docs/runtime/origin_target_route_taxonomy.md"
COMMAND_CONTRACT_REL = "docs/commands/command_descriptor_contract.md"
COMMAND_SCHEMA_REL = "schemas/commands/command_descriptor.schema.json"
SILENT_DEPLOYMENT_REL = "artifacts/release/silent_deployment_seed.yaml"
DOCS_TRUTH_ADR_REL = "docs/adr/0013-docs-help-service-health-truth.md"
DOCS_PACK_REL = "docs/docs/docs_pack_manifest_contract.md"
REQUIREMENT_REGISTER_REL = "artifacts/governance/requirement_register_seed.yaml"
REQUIREMENT_CROSSWALK_REL = "docs/governance/requirement_alias_crosswalk.md"

REQUIRED_MD_HEADINGS = [
    "## Decision requested",
    "## Milestone objective",
    "## Hero workflow result",
    "## Readiness scorecard",
    "## Changed scope since last review",
    "## Waivers",
    "## Evidence index",
    "## Rollback / recovery posture",
    "## Next-milestone risk",
    "## Named signoffs",
    "## Mandatory signed-packet sections",
    "## Contract-family matrix",
    "## Evidence freshness",
]

EXPECTED_REVIEWERS = {
    "architecture",
    "product",
    "design",
    "qe_perf",
    "accessibility",
    "docs_truth",
    "support",
    "security",
    "release",
}

EXPECTED_ADRS = {
    "ADR-0002",
    "ADR-0003",
    "ADR-0004",
    "ADR-0005",
    "ADR-0006",
    "ADR-0007",
    "ADR-0008",
    "ADR-0009",
    "ADR-0010",
    "ADR-0011",
    "ADR-0013",
    "ADR-0014",
}


@dataclass
class Result:
    status: str
    name: str
    detail: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    return parser.parse_args()


def read_text(path: Path) -> str:
    with path.open("r", encoding="utf-8") as fh:
        return fh.read()


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def rel_path(root: Path, rel: str) -> Path:
    return root / rel


def exists(root: Path, rel: str) -> bool:
    return rel_path(root, rel).exists()


def contains_all(root: Path, rel: str, patterns: list[str]) -> bool:
    path = rel_path(root, rel)
    if not path.exists():
        return False
    text = read_text(path)
    return all(pattern in text for pattern in patterns)


def missing_paths(root: Path, refs: list[str]) -> list[str]:
    return [ref for ref in refs if not exists(root, ref)]


def parse_duration(value: str | None) -> dt.timedelta | None:
    if value is None:
        return None
    match = re.fullmatch(
        r"P(?:(?P<days>\d+)D)?(?:T(?:(?P<hours>\d+)H)?(?:(?P<minutes>\d+)M)?(?:(?P<seconds>\d+)S)?)?",
        value,
    )
    if not match:
        raise ValueError(f"unsupported ISO-8601 duration: {value}")
    days = int(match.group("days") or 0)
    hours = int(match.group("hours") or 0)
    minutes = int(match.group("minutes") or 0)
    seconds = int(match.group("seconds") or 0)
    return dt.timedelta(days=days, hours=hours, minutes=minutes, seconds=seconds)


def parse_timestamp(value: str) -> dt.datetime:
    return dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=dt.timezone.utc)


def make_result(ok: bool, name: str, detail: str, missing: bool = False) -> Result:
    if missing:
        return Result("MISSING", name, detail)
    return Result("PASS" if ok else "FAIL", name, detail)


def check_packet_structure(root: Path, packet: dict[str, Any], packet_md: str) -> Result:
    packet_paths = [PACKET_JSON_REL, PACKET_MD_REL, PACKET_TEMPLATE_REL, CHECKLIST_REL]
    missing = missing_paths(root, packet_paths)
    if missing:
        return make_result(False, "shared_packet", f"missing packet assets: {', '.join(missing)}", True)

    missing_headings = [heading for heading in REQUIRED_MD_HEADINGS if heading not in packet_md]
    if missing_headings:
        return make_result(False, "shared_packet", f"missing packet headings: {', '.join(missing_headings)}")

    required_keys = {
        "schema_version",
        "packet_id",
        "milestone_id",
        "packet_state",
        "readiness",
        "decision_requested",
        "owner",
        "evidence_owner",
        "reviewer_signoffs",
        "mandatory_sections",
        "contract_families",
        "qe_lane_registry",
        "assurance_claim_matrix",
        "public_proof_coverage_report",
        "evidence_rows",
        "validation_metadata",
    }
    missing_keys = sorted(required_keys - set(packet))
    if missing_keys:
        return make_result(False, "shared_packet", f"missing packet keys: {', '.join(missing_keys)}")

    metadata = packet["validation_metadata"]
    for key in ("source_anchor_refs", "stale_after_policy_ref", "control_packet_refs", "frozen_surface_manifest_refs"):
        if key not in metadata:
            return make_result(False, "shared_packet", f"validation metadata missing '{key}'")

    reviewers = packet["reviewer_signoffs"]
    reviewer_ids = {entry.get("reviewer") for entry in reviewers}
    if reviewer_ids != EXPECTED_REVIEWERS:
        missing_reviewers = sorted(EXPECTED_REVIEWERS - reviewer_ids)
        unexpected = sorted(reviewer_ids - EXPECTED_REVIEWERS)
        parts = []
        if missing_reviewers:
            parts.append(f"missing reviewers: {', '.join(missing_reviewers)}")
        if unexpected:
            parts.append(f"unexpected reviewers: {', '.join(unexpected)}")
        return make_result(False, "shared_packet", "; ".join(parts))

    if not packet.get("owner") or not packet.get("evidence_owner") or not packet.get("packet_id"):
        return make_result(False, "shared_packet", "packet owner, evidence_owner, and packet_id are required")

    return make_result(True, "shared_packet", "shared packet, template, checklist, metadata, and reviewer slots are present")


def check_mandatory_sections(root: Path, packet: dict[str, Any], packet_md: str) -> Result:
    problems: list[str] = []
    for section in packet["mandatory_sections"]:
        title = section["title"]
        heading = f"### {title}"
        if heading not in packet_md:
            problems.append(f"packet markdown missing heading '{heading}'")
        for ref in section["required_refs"]:
            if not exists(root, ref):
                problems.append(f"{title}: missing ref {ref}")
    if problems:
        return make_result(False, "mandatory_sections", "; ".join(problems))
    return make_result(True, "mandatory_sections", "deployment-profile, decision, notification/chronology, local-history, and security-intake sections are all present")


def check_architecture_pack(root: Path) -> Result:
    refs = [ARCH_PACK_README_REL, ARCH_PACK_PACKET_REL, SCORECARD_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "architecture_pack", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, ARCH_PACK_README_REL, ["Packet status: review-ready", "M0 signoff packet"]):
        return make_result(False, "architecture_pack", "architecture-pack README is not review-ready or does not cite the shared signoff packet")
    return make_result(True, "architecture_pack", "architecture pack exists, is review-ready, and points reviewers at the shared signoff packet")


def check_benchmark_ci(root: Path) -> Result:
    refs = [BENCH_WORKFLOW_REL, BENCH_DASHBOARD_REL, BENCH_CHARTER_REL, SCORECARD_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "benchmark_ci", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, SCORECARD_REL, ["- lane_id: benchmark_lab", BENCH_WORKFLOW_REL, BENCH_DASHBOARD_REL]):
        return make_result(False, "benchmark_ci", "benchmark_lab lane is missing the workflow or dashboard evidence refs")
    return make_result(True, "benchmark_ci", "benchmark workflow, dashboard seed, charter, and scorecard lane are all present")


def check_renderer_spike(root: Path) -> Result:
    refs = [RENDER_ADR_REL, RENDER_SPIKE_REL, RENDER_TRACE_REL, SCORECARD_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "renderer_spike", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, SCORECARD_REL, ["- lane_id: aureline-render", RENDER_SPIKE_REL]):
        return make_result(False, "renderer_spike", "renderer lane evidence is incomplete in the scorecard")
    return make_result(True, "renderer_spike", "renderer ADR, spike manifest, trace sample, and scorecard lane are aligned")


def check_top_adrs(root: Path) -> Result:
    refs = [DECISION_INDEX_REL, ARCH_PACK_PACKET_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "top_adrs", f"missing refs: {', '.join(missing)}", True)
    packet_text = read_text(rel_path(root, ARCH_PACK_PACKET_REL))
    missing_adrs = sorted(adr for adr in EXPECTED_ADRS if adr not in packet_text)
    if missing_adrs:
        return make_result(False, "top_adrs", f"approved ADR set missing: {', '.join(missing_adrs)}")
    return make_result(True, "top_adrs", "core approved ADR set is present in the architecture pack")


def check_ownership(root: Path) -> Result:
    refs = [OWNERSHIP_REL, CODEOWNERS_REL, SCORECARD_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "ownership", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, OWNERSHIP_REL, ["governance_lanes:", "decision_forums:"]):
        return make_result(False, "ownership", "ownership matrix is missing governance lanes or decision forums")
    return make_result(True, "ownership", "ownership matrix, CODEOWNERS, and scorecard all expose the current lane owners")


def check_requirement_register(root: Path) -> Result:
    refs = [ARCH_PACK_PACKET_REL, REQUIREMENT_REGISTER_REL, REQUIREMENT_CROSSWALK_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "requirement_register", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(
        root,
        ARCH_PACK_PACKET_REL,
        [
            "requirement_register_slice:",
            f"source_ref: {REQUIREMENT_REGISTER_REL}",
            f"alias_crosswalk_ref: {REQUIREMENT_CROSSWALK_REL}",
        ],
    ):
        return make_result(False, "requirement_register", "architecture packet does not expose the governed requirement-register slice")
    if not contains_all(root, REQUIREMENT_REGISTER_REL, ["requirement_rows:", "crosswalk_rows:"]):
        return make_result(False, "requirement_register", "requirement register seed is missing canonical rows or crosswalk rows")
    return make_result(True, "requirement_register", "architecture packet, governed requirement register, and alias crosswalk are aligned")


def check_anchor_and_canonical_coverage(root: Path) -> Result:
    refs = [EVIDENCE_FIELDS_REL, ARCH_COVERAGE_REL, DECISION_INDEX_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "anchor_and_canonical_coverage", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, EVIDENCE_FIELDS_REL, ["field: source_anchor_refs", "field: stale_after"]):
        return make_result(False, "anchor_and_canonical_coverage", "evidence metadata field catalog is missing source-anchor or freshness fields")
    if not contains_all(root, ARCH_COVERAGE_REL, ["canonical_refs:", "freeze_exceptions:"]):
        return make_result(False, "anchor_and_canonical_coverage", "architecture coverage matrix is missing canonical refs or freeze exceptions")
    if "source_anchors:" not in read_text(rel_path(root, DECISION_INDEX_REL)):
        return make_result(False, "anchor_and_canonical_coverage", "decision register no longer exposes source anchors")
    return make_result(True, "anchor_and_canonical_coverage", "source-anchor fields and canonical-reference coverage remain visible")


def check_dependency_ledger(root: Path) -> Result:
    refs = [DECISION_INDEX_REL, DEPENDENCY_REGISTER_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "dependency_ledger", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, DECISION_INDEX_REL, ["dependencies:", "DEP-0003"]):
        return make_result(False, "dependency_ledger", "decision dependency ledger is missing expected dependency rows")
    return make_result(True, "dependency_ledger", "decision and third-party dependency ledgers are both present")


def check_control_artifact_status(root: Path) -> Result:
    if not exists(root, CONTROL_ARTIFACT_INDEX_REL):
        return make_result(False, "control_artifact_status", f"missing ref: {CONTROL_ARTIFACT_INDEX_REL}", True)
    if not contains_all(root, CONTROL_ARTIFACT_INDEX_REL, ["id: milestone_review_packet", "id: canonical_requirement_register"]):
        return make_result(False, "control_artifact_status", "control-artifact index is missing the milestone review packet or canonical requirement-register row")
    return make_result(True, "control_artifact_status", "control-artifact index includes the milestone review packet and canonical requirement-register rows")


def check_decision_forums(root: Path) -> Result:
    refs = [OWNERSHIP_REL, BENCH_CHARTER_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "decision_forums", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, OWNERSHIP_REL, ["decision_forums:", "architecture_council", "performance_council"]):
        return make_result(False, "decision_forums", "ownership matrix is missing the expected decision forums")
    return make_result(True, "decision_forums", "decision forums and benchmark charter are both present")


def check_qualification_and_rings(root: Path) -> Result:
    refs = [QUAL_MATRIX_REL, VERSION_SKEW_REL, INSTALL_TOPOLOGY_REL, FEATURE_FLAG_POLICY_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "qualification_and_rings", f"missing refs: {', '.join(missing)}", True)
    if not contains_all(root, INSTALL_TOPOLOGY_REL, ["rollout_ring_class_vocabulary:", "canary", "lts"]):
        return make_result(False, "qualification_and_rings", "install topology matrix is missing rollout ring vocabulary")
    return make_result(True, "qualification_and_rings", "qualification matrix, skew register, install topology, and rollout policy are linked")


def check_accessibility_and_locale(root: Path, packet: dict[str, Any]) -> Result:
    family = next((item for item in packet["contract_families"] if item["family_id"] == "accessibility_locale_review_lanes"), None)
    if family is None:
        return make_result(False, "accessibility_and_locale", "signoff packet is missing the accessibility/locale contract family", True)
    refs = [CONTROL_ARTIFACT_INDEX_REL, ARCH_COVERAGE_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "accessibility_and_locale", f"missing refs: {', '.join(missing)}", True)
    if family["status"] != "deferred_signed_exception" or not family.get("signed_exception_ref"):
        return make_result(False, "accessibility_and_locale", "accessibility/locale family must stay explicitly deferred with a signed exception")
    if family["signed_exception_ref"] not in read_text(rel_path(root, ARCH_COVERAGE_REL)):
        return make_result(False, "accessibility_and_locale", "freeze exception ref is missing from the architecture coverage file")
    return make_result(True, "accessibility_and_locale", "accessibility and locale review-lane blockers remain explicit and signed")


def check_locality_and_transport(root: Path) -> Result:
    refs = [BOUNDARY_MANIFEST_REL, CONTINUITY_REL, ROUTE_TAXONOMY_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "locality_and_transport", f"missing refs: {', '.join(missing)}", True)
    return make_result(True, "locality_and_transport", "boundary manifest, continuity drill seed, and route taxonomy are all present")


def check_cli_posture(root: Path) -> Result:
    refs = [COMMAND_CONTRACT_REL, COMMAND_SCHEMA_REL, SILENT_DEPLOYMENT_REL, ARCH_PACK_PACKET_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "cli_headless_posture", f"missing refs: {', '.join(missing)}", True)
    packet_text = read_text(rel_path(root, ARCH_PACK_PACKET_REL))
    if "Stable CLI/headless surfaces exist" not in packet_text:
        return make_result(False, "cli_headless_posture", "architecture pack does not declare the CLI/headless posture")
    return make_result(True, "cli_headless_posture", "CLI/headless contract posture is declared in docs, schema, and architecture packet")


def check_docs_control(root: Path) -> Result:
    refs = [DOCS_TRUTH_ADR_REL, DOCS_PACK_REL, CONTROL_ARTIFACT_INDEX_REL]
    missing = missing_paths(root, refs)
    if missing:
        return make_result(False, "docs_control", f"missing refs: {', '.join(missing)}", True)
    if "id: docs_public_truth" not in read_text(rel_path(root, CONTROL_ARTIFACT_INDEX_REL)):
        return make_result(False, "docs_control", "control-artifact index no longer exposes the docs_public_truth row")
    return make_result(True, "docs_control", "docs-control policy remains anchored on the docs/help truth ADR and docs-pack contract")


def check_evidence_freshness(packet: dict[str, Any]) -> Result:
    now = dt.datetime.now(dt.timezone.utc)
    stale: list[str] = []
    invalid: list[str] = []
    for evidence in packet["evidence_rows"]:
        evidence_id = evidence.get("evidence_id", "<unknown>")
        if "stale_after" not in evidence or "captured_at" not in evidence or "source_anchor_refs" not in evidence:
            invalid.append(f"{evidence_id} missing captured_at/stale_after/source_anchor_refs")
            continue
        try:
            captured_at = parse_timestamp(evidence["captured_at"])
            duration = parse_duration(evidence["stale_after"])
        except ValueError as exc:
            invalid.append(f"{evidence_id}: {exc}")
            continue
        if duration is not None and now > captured_at + duration:
            stale.append(evidence_id)
    if invalid:
        return make_result(False, "evidence_freshness", "; ".join(invalid))
    if stale:
        return make_result(False, "evidence_freshness", f"stale evidence rows: {', '.join(stale)}")
    return make_result(True, "evidence_freshness", "all evidence rows carry current-enough freshness metadata")


def check_contract_family_coverage(root: Path, packet: dict[str, Any], packet_md: str) -> Result:
    arch_text = read_text(rel_path(root, ARCH_COVERAGE_REL))
    qual_text = read_text(rel_path(root, QUAL_MATRIX_REL))
    skew_text = read_text(rel_path(root, VERSION_SKEW_REL))

    qe_ids = {entry["family_id"] for entry in packet["qe_lane_registry"]}
    assurance_ids = {entry["family_id"] for entry in packet["assurance_claim_matrix"]}
    public_ids = {entry["family_id"] for entry in packet["public_proof_coverage_report"]}

    problems: list[str] = []
    for family in packet["contract_families"]:
        family_id = family["family_id"]
        if family_id not in packet_md:
            problems.append(f"{family_id} missing from packet markdown matrix")
        if family_id not in arch_text:
            problems.append(f"{family_id} missing from architecture coverage file")
        for row_ref in family["compatibility_row_refs"]:
            if row_ref not in qual_text:
                problems.append(f"{family_id} missing compatibility row {row_ref}")
            if f"qualification_row_ref: {row_ref}" not in skew_text:
                problems.append(f"{family_id} missing skew-register linkage for {row_ref}")
        if family_id not in qe_ids:
            problems.append(f"{family_id} missing from QE lane registry")
        if family_id not in assurance_ids:
            problems.append(f"{family_id} missing from assurance-claim matrix")
        if family_id not in public_ids:
            problems.append(f"{family_id} missing from public-proof coverage report")
        if family["status"] == "deferred_signed_exception" and not family.get("signed_exception_ref"):
            problems.append(f"{family_id} missing signed exception ref")
        if family.get("signed_exception_ref") and family["signed_exception_ref"] not in arch_text:
            problems.append(f"{family_id} missing exception ref {family['signed_exception_ref']} in architecture coverage")
    if problems:
        return make_result(False, "contract_family_coverage", "; ".join(problems))
    return make_result(True, "contract_family_coverage", "all signoff contract families are represented across architecture, compatibility, QE, assurance, and public-proof coverage")


def validate(root: Path) -> list[Result]:
    packet = load_json(rel_path(root, PACKET_JSON_REL))
    packet_md = read_text(rel_path(root, PACKET_MD_REL))

    checks = [
        check_packet_structure(root, packet, packet_md),
        check_mandatory_sections(root, packet, packet_md),
        check_architecture_pack(root),
        check_benchmark_ci(root),
        check_renderer_spike(root),
        check_top_adrs(root),
        check_ownership(root),
        check_requirement_register(root),
        check_anchor_and_canonical_coverage(root),
        check_dependency_ledger(root),
        check_control_artifact_status(root),
        check_decision_forums(root),
        check_qualification_and_rings(root),
        check_accessibility_and_locale(root, packet),
        check_locality_and_transport(root),
        check_cli_posture(root),
        check_docs_control(root),
        check_evidence_freshness(packet),
        check_contract_family_coverage(root, packet, packet_md),
    ]
    return checks


def print_results(results: list[Result]) -> None:
    print("M0 exit signoff validation")
    print("")
    for result in results:
        print(f"{result.status:<7} {result.name}: {result.detail}")
    print("")
    counts = {
        "PASS": sum(1 for result in results if result.status == "PASS"),
        "FAIL": sum(1 for result in results if result.status == "FAIL"),
        "MISSING": sum(1 for result in results if result.status == "MISSING"),
    }
    print(
        "Summary: "
        f"{counts['PASS']} pass, {counts['FAIL']} fail, {counts['MISSING']} missing"
    )


def main() -> int:
    args = parse_args()
    root = Path(args.repo_root).resolve()
    results = validate(root)
    print_results(results)
    return 1 if any(result.status != "PASS" for result in results) else 0


if __name__ == "__main__":
    raise SystemExit(main())
