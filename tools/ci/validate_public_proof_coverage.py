#!/usr/bin/env python3
"""Public-proof coverage audit.

Joins the requirement register, claim manifest, assurance-claim matrix,
public-proof packet fixtures, workflow-bundle register, known-limit
classes, docs destination descriptors, and exact-build identity fixtures
into a single mechanical coverage audit.

This tool exists so CI can fail on orphan public truth: a public-proof
packet without a bundle id, a claim row without a requirement id, a
claim row without a required known-limit note, a docs-version mismatch
without a declared narrowing reason, or a contract family published in
docs / release / marketplace / support packets without a backing claim
row.

Outputs:
- human-readable findings on stdout
- machine-readable JSON report via --report
- non-zero exit code when error-severity findings are present

The repository is YAML-heavy, and the toolchain is intentionally light:
Python stdlib drives control flow and Ruby's bundled Psych parser
decodes YAML. The wrapper at ci/check_public_proof_coverage.sh makes
the Ruby dependency explicit.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

REQUIREMENT_REGISTER_REL = "artifacts/governance/requirement_register_seed.yaml"
CLAIM_MANIFEST_REL = "artifacts/governance/claim_manifest_seed.yaml"
ASSURANCE_CLAIM_ROWS_REL = "artifacts/release/assurance_claim_rows.yaml"
WORKFLOW_BUNDLE_IDS_REL = "artifacts/qe/workflow_bundle_ids.yaml"
KNOWN_LIMIT_CLASSES_REL = "artifacts/product/known_limit_classes.yaml"
DESTINATION_DESCRIPTOR_REL = "artifacts/docs/destination_descriptor_seed.yaml"
PUBLIC_PROOF_PACKETS_DIR_REL = "fixtures/qe/public_proof_packets"
EXACT_BUILD_EXAMPLES_DIR_REL = "fixtures/build/exact_build_examples"
COVERAGE_REPORT_REL = "artifacts/governance/public_proof_coverage_report.md"
CI_WRAPPER_REL = "ci/check_public_proof_coverage.sh"

SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}

# Contract families the spec names explicitly. Each family must be backed
# by at least one assurance_claim_row (keyed by claim_subject_family).
# The mapping is many-to-many because several families share an
# assurance-claim subject (for example theme_package_portability covers
# both portability and localization/theme assets).
CONTRACT_FAMILIES: dict[str, list[str]] = {
    "language_provider_truth": ["provider_aware_language_intelligence"],
    "execution_surfaces": [
        "replay_safe_execution_history",
        "trustworthy_diagnostics_and_quick_fixes",
    ],
    "git_review_history_edit": [
        "provider_integrated_review",
        "replay_safe_execution_history",
    ],
    "portability": [
        "export_and_offboarding_support",
        "theme_package_portability",
    ],
    "localization_theme_assets": [
        "localization_readiness",
        "theme_package_portability",
    ],
    "onboarding_voice": ["voice_privacy"],
    "repair": ["repair_rollback_safety"],
    "sdk_publication": ["regulated_environment_assurance"],
    "hosted_review_state": [
        "provider_integrated_review",
        "regulated_environment_assurance",
    ],
}


@dataclass
class Finding:
    severity: str
    check_id: str
    artifact_ref: str
    owner_artifact_ref: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if not payload["details"]:
            payload.pop("details")
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


def now_utc() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def render_yaml_as_json(path: Path) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def load_json(path: Path) -> Any:
    with path.open("rb") as fh:
        return json.load(fh)


def strip_path_annotations(ref: str) -> str:
    path = ref
    for separator in ("#", " §", " line ", " @"):
        if separator in path:
            path = path.split(separator, 1)[0]
    return path.strip()


def is_path_like(ref: str) -> bool:
    if not ref or ref in SENTINEL_REFS:
        return False
    if ref.startswith(
        (
            "compat_row:",
            "skew_register:",
            "skew_case:",
            "claim_row:",
            "lane:",
            "package:",
            "build:",
            "evidence.",
            "docs-pack:",
            "dest:",
            "producer:",
            "launch_bundle:",
            "cutline:",
            "persona:",
            "archetype_row:",
            "archetype_rubric:",
            "non_claim:",
            "exclusion:",
            "deployment_profile:",
            "protected_path:",
            "corpus:",
            "reference_workspace:",
            "task_summary.",
            "success_criterion.",
            "docs_pack_revision_ref.",
            "known_limit_summary.",
            "prior_claim_state_ref.",
            "diff_summary.",
            "exact_build_identity_ref.",
            "hardware_definition_ref.",
            "environment_definition_ref.",
        )
    ):
        return False
    return (
        "/" in ref
        or ref.endswith(
            (".md", ".yaml", ".yml", ".json", ".py", ".sh", ".toml", ".rs")
        )
        or ref in {"README.md", "CONTRIBUTING.md", "CODEOWNERS"}
    )


def item_ref(row_id: str, rel: str) -> str:
    return f"{rel}#{row_id}"


def make_finding(
    severity: str,
    check_id: str,
    artifact_ref: str,
    owner_artifact_ref: str,
    message: str,
    remediation: str,
    row_ref: str | None = None,
    **details: Any,
) -> Finding:
    return Finding(
        severity=severity,
        check_id=check_id,
        artifact_ref=artifact_ref,
        owner_artifact_ref=owner_artifact_ref,
        message=message,
        remediation=remediation,
        row_ref=row_ref,
        details=details,
    )


class RepoView:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.yaml_cache: dict[str, Any] = {}
        self.json_cache: dict[str, Any] = {}

    def rel(self, relative: str) -> Path:
        return self.root / relative

    def exists(self, relative: str) -> bool:
        return self.rel(relative).exists()

    def yaml(self, relative: str) -> Any:
        if relative not in self.yaml_cache:
            self.yaml_cache[relative] = render_yaml_as_json(self.rel(relative))
        return self.yaml_cache[relative]

    def json(self, relative: str) -> Any:
        if relative not in self.json_cache:
            self.json_cache[relative] = load_json(self.rel(relative))
        return self.json_cache[relative]

    def path_exists(self, ref: str) -> bool:
        if not is_path_like(ref):
            return True
        return self.exists(strip_path_annotations(ref))


def load_requirement_ids(repo: RepoView) -> set[str]:
    payload = repo.yaml(REQUIREMENT_REGISTER_REL)
    ids: set[str] = set()
    for row in payload.get("requirement_rows", []):
        rid = row.get("requirement_id")
        if isinstance(rid, str):
            ids.add(rid)
    return ids


def validate_requirement_coverage(
    repo: RepoView, requirement_ids: set[str]
) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    for row in manifest.get("claim_rows", []):
        row_id = row.get("claim_row_id", "<unknown>")
        row_ref = item_ref(row_id, CLAIM_MANIFEST_REL)
        row_requirements = row.get("requirement_ids") or []
        if not row_requirements:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.orphan_claim_row_no_requirement",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row_id}' declares no requirement_ids",
                    "Cite at least one canonical requirement id from requirement_register_seed.yaml on every claim row.",
                    row_ref=row_id,
                )
            )
            continue
        unresolved = sorted(set(row_requirements) - requirement_ids)
        if unresolved:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.orphan_claim_row_unresolved_requirement",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row_id}' cites unknown requirement_ids: {', '.join(unresolved)}",
                    "Add the requirement row to requirement_register_seed.yaml or fix the claim row citation.",
                    row_ref=row_id,
                    unresolved_requirement_ids=unresolved,
                )
            )
    return findings


def validate_claim_manifest_known_limits(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    for row in manifest.get("claim_rows", []):
        row_id = row.get("claim_row_id", "<unknown>")
        row_ref = item_ref(row_id, CLAIM_MANIFEST_REL)
        effective = row.get("effective_claim_posture")
        known_limit_refs = row.get("known_limit_refs") or []
        policy = row.get("downgrade_policy", {}) or {}
        required_policy_refs = policy.get("required_known_limit_refs") or []
        missing_policy = sorted(set(required_policy_refs) - set(known_limit_refs))
        if missing_policy:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.claim_missing_downgrade_policy_known_limit",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row_id}' downgrade_policy requires known-limit refs the row does not carry: {', '.join(missing_policy)}",
                    "Add the required known-limit refs to the claim row, or drop them from downgrade_policy.required_known_limit_refs.",
                    row_ref=row_id,
                    missing_refs=missing_policy,
                )
            )
        if effective in {"limited", "experimental", "replacement_grade"} and not known_limit_refs:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.claim_narrowed_without_known_limit",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row_id}' projects as '{effective}' but carries no known_limit_refs",
                    "Narrowed or preview-grade claims must cite at least one known-limit note before surfaces publish narrowed wording.",
                    row_ref=row_id,
                )
            )
    return findings


def validate_claim_manifest_docs_version(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    for row in manifest.get("claim_rows", []):
        row_id = row.get("claim_row_id", "<unknown>")
        row_ref = item_ref(row_id, CLAIM_MANIFEST_REL)
        exact_build_refs = row.get("exact_build_identity_refs") or []
        bindings = row.get("channel_bindings", {}) or {}
        for channel_id, binding in bindings.items():
            state = binding.get("minimum_version_match_state")
            if state in {"exact_build_match", "compatible_minor_drift"} and not exact_build_refs:
                findings.append(
                    make_finding(
                        "warning",
                        "public_proof_coverage.docs_version_state_without_exact_build",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        (
                            f"claim row '{row_id}' channel '{channel_id}' declares "
                            f"minimum_version_match_state='{state}' but the row lists no exact_build_identity_refs"
                        ),
                        "Cite an exact_build_identity_ref on the row or relax the channel binding's minimum_version_match_state to not_applicable.",
                        row_ref=row_id,
                        channel_id=channel_id,
                        minimum_version_match_state=state,
                    )
                )
    return findings


def validate_assurance_rows(
    repo: RepoView,
) -> tuple[list[Finding], dict[str, dict[str, Any]]]:
    findings: list[Finding] = []
    rows_payload = repo.yaml(ASSURANCE_CLAIM_ROWS_REL)
    claim_classes = {
        row.get("class_id"): row for row in rows_payload.get("class_rules", [])
    }
    by_family: dict[str, dict[str, Any]] = {}
    for row in rows_payload.get("assurance_claims", []):
        claim_id = row.get("claim_id", "<unknown>")
        family = row.get("claim_subject_family")
        if isinstance(family, str):
            by_family[family] = row
        row_ref = item_ref(claim_id, ASSURANCE_CLAIM_ROWS_REL)
        effective = row.get("effective_claim_class")
        rule = claim_classes.get(effective) or {}
        minimum_known_limits = rule.get("required_known_limit_refs_minimum") or 0
        known_limit_refs = row.get("known_limit_refs") or []
        if isinstance(minimum_known_limits, int) and len(known_limit_refs) < minimum_known_limits:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.assurance_row_known_limit_floor",
                    ASSURANCE_CLAIM_ROWS_REL,
                    row_ref,
                    (
                        f"assurance claim '{claim_id}' projects as '{effective}' which requires at least "
                        f"{minimum_known_limits} known_limit_refs; row carries {len(known_limit_refs)}"
                    ),
                    "Cite enough known-limit notes on the assurance row for its effective class, or narrow the effective class.",
                    row_ref=claim_id,
                )
            )
        minimum_exclusions = rule.get("required_exclusion_refs_minimum") or 0
        exclusion_refs = row.get("exclusion_refs") or []
        if isinstance(minimum_exclusions, int) and len(exclusion_refs) < minimum_exclusions:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.assurance_row_exclusion_floor",
                    ASSURANCE_CLAIM_ROWS_REL,
                    row_ref,
                    (
                        f"assurance claim '{claim_id}' projects as '{effective}' which requires at least "
                        f"{minimum_exclusions} exclusion_refs; row carries {len(exclusion_refs)}"
                    ),
                    "Cite enough exclusion refs on the assurance row for its effective class, or narrow the effective class.",
                    row_ref=claim_id,
                )
            )
        match = row.get("docs_version_match") or {}
        if not match.get("minimum_match_state"):
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.assurance_row_missing_docs_version_match",
                    ASSURANCE_CLAIM_ROWS_REL,
                    row_ref,
                    f"assurance claim '{claim_id}' does not declare docs_version_match.minimum_match_state",
                    "Every assurance-claim row must declare a docs/help version-match floor so docs drift is not silent.",
                    row_ref=claim_id,
                )
            )
        required_evidence_refs = row.get("required_evidence_refs") or []
        for ref in required_evidence_refs:
            if isinstance(ref, str) and not repo.path_exists(ref):
                findings.append(
                    make_finding(
                        "error",
                        "public_proof_coverage.assurance_row_required_evidence_missing",
                        ASSURANCE_CLAIM_ROWS_REL,
                        row_ref,
                        f"assurance claim '{claim_id}' lists required_evidence_ref '{ref}' that does not exist",
                        "Land the cited evidence artifact or remove the stale required_evidence_ref.",
                        row_ref=claim_id,
                    )
                )
        for ref in known_limit_refs:
            if isinstance(ref, str) and not repo.path_exists(ref):
                findings.append(
                    make_finding(
                        "error",
                        "public_proof_coverage.assurance_row_known_limit_ref_missing",
                        ASSURANCE_CLAIM_ROWS_REL,
                        row_ref,
                        f"assurance claim '{claim_id}' cites missing known_limit_ref '{ref}'",
                        "Keep assurance-row known_limit_refs pointed at real caveat artifacts.",
                        row_ref=claim_id,
                    )
                )
    return findings, by_family


def validate_contract_family_coverage(
    by_family: dict[str, dict[str, Any]],
) -> list[Finding]:
    findings: list[Finding] = []
    for family, admissible in CONTRACT_FAMILIES.items():
        if not any(subject in by_family for subject in admissible):
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.contract_family_uncovered",
                    ASSURANCE_CLAIM_ROWS_REL,
                    ASSURANCE_CLAIM_ROWS_REL,
                    (
                        f"contract family '{family}' has no backing assurance_claim_row "
                        f"(one of: {', '.join(admissible)})"
                    ),
                    "Add an assurance_claim_row with a claim_subject_family from the admissible set, or extend CONTRACT_FAMILIES if the taxonomy moved.",
                    row_ref=family,
                    admissible_claim_subject_families=admissible,
                )
            )
    return findings


def list_public_proof_fixtures(repo: RepoView) -> list[Path]:
    root = repo.rel(PUBLIC_PROOF_PACKETS_DIR_REL)
    if not root.exists():
        return []
    return sorted(p for p in root.glob("*.json") if p.is_file())


def list_exact_build_fixtures(repo: RepoView) -> list[Path]:
    root = repo.rel(EXACT_BUILD_EXAMPLES_DIR_REL)
    if not root.exists():
        return []
    return sorted(p for p in root.glob("*.json") if p.is_file())


def validate_public_proof_packets(
    repo: RepoView, by_family: dict[str, dict[str, Any]]
) -> list[Finding]:
    findings: list[Finding] = []
    bundles = repo.yaml(WORKFLOW_BUNDLE_IDS_REL)
    known_bundle_ids = {
        row.get("bundle_id")
        for row in bundles.get("workflow_bundles", [])
        if isinstance(row.get("bundle_id"), str)
    }
    cutline_pairings = {
        entry.get("cutline_ref")
        for entry in bundles.get("cutline_scoreboard_pairings", [])
        if isinstance(entry.get("cutline_ref"), str)
    }
    scoreboard_family_ids = set(bundles.get("scoreboard_family_vocabulary", []))
    packet_shapes = set(bundles.get("public_proof_packet_shape_vocabulary", []))

    # Walk each assurance-claim row to collect publication postures so we
    # can cross-check which families expect a public-proof packet.
    expects_public_proof: set[str] = set()
    for family, row in by_family.items():
        destinations = row.get("publication_destinations") or []
        if "public_proof_packet" in destinations:
            expects_public_proof.add(family)

    seen_bundle_ids: set[str] = set()
    seen_cutline_refs: set[str] = set()
    seen_scoreboard_families: set[str] = set()

    for fixture_path in list_public_proof_fixtures(repo):
        fixture_rel = str(fixture_path.relative_to(repo.root))
        try:
            payload = load_json(fixture_path)
        except json.JSONDecodeError as exc:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_not_valid_json",
                    fixture_rel,
                    fixture_rel,
                    f"public-proof packet fixture is not valid JSON: {exc}",
                    "Fix the JSON syntax so the validator can read the packet.",
                )
            )
            continue
        packet_id = payload.get("packet_id") or fixture_path.name
        row_ref = item_ref(packet_id, fixture_rel)
        bundle_ref = payload.get("workflow_bundle_ref") or {}
        bundle_id = bundle_ref.get("bundle_id")
        if not bundle_id:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_missing_bundle_id",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' has no workflow_bundle_ref.bundle_id",
                    "Every public-proof packet MUST cite a workflow_bundle_ref.bundle_id from artifacts/qe/workflow_bundle_ids.yaml.",
                    row_ref=packet_id,
                )
            )
        elif bundle_id not in known_bundle_ids:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_unresolved_bundle_id",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' cites unknown bundle_id '{bundle_id}'",
                    "Register the workflow bundle in artifacts/qe/workflow_bundle_ids.yaml or fix the packet citation.",
                    row_ref=packet_id,
                    bundle_id=bundle_id,
                )
            )
        else:
            seen_bundle_ids.add(bundle_id)

        archetype_ref = payload.get("archetype_row_ref") or {}
        archetype_id = archetype_ref.get("archetype_row_id")
        if not archetype_id:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_missing_archetype_row",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' has no archetype_row_ref.archetype_row_id",
                    "Every public-proof packet MUST cite one archetype_row_id so reviewers can trace the corpus lineage.",
                    row_ref=packet_id,
                )
            )

        cutline_ref = payload.get("cutline_ref")
        if not cutline_ref:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_missing_cutline_ref",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' has no cutline_ref",
                    "Every public-proof packet MUST cite one cutline_ref so reviewers can trace the launch-wedge ownership.",
                    row_ref=packet_id,
                )
            )
        elif cutline_ref not in cutline_pairings:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_cutline_not_paired",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' cites cutline_ref '{cutline_ref}' that has no cutline_scoreboard_pairings row",
                    "Add a cutline_scoreboard_pairings row in workflow_bundle_ids.yaml or fix the packet citation.",
                    row_ref=packet_id,
                )
            )
        else:
            seen_cutline_refs.add(cutline_ref)

        scoreboard_family = payload.get("scoreboard_family_id")
        if not scoreboard_family:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_missing_scoreboard_family",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' has no scoreboard_family_id",
                    "Every public-proof packet MUST cite a scoreboard_family_id from workflow_bundle_ids.yaml.",
                    row_ref=packet_id,
                )
            )
        elif scoreboard_family_ids and scoreboard_family not in scoreboard_family_ids:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_unresolved_scoreboard_family",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' cites unknown scoreboard_family_id '{scoreboard_family}'",
                    "Register the scoreboard family in workflow_bundle_ids.yaml scoreboard_family_vocabulary or fix the packet citation.",
                    row_ref=packet_id,
                    scoreboard_family_id=scoreboard_family,
                )
            )
        else:
            seen_scoreboard_families.add(scoreboard_family)

        packet_shape = payload.get("packet_shape")
        if packet_shape and packet_shapes and packet_shape not in packet_shapes:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_unresolved_packet_shape",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' cites unknown packet_shape '{packet_shape}'",
                    "Register the packet shape in workflow_bundle_ids.yaml public_proof_packet_shape_vocabulary or fix the packet citation.",
                    row_ref=packet_id,
                )
            )

        environment_ref = payload.get("environment_ref") or {}
        exact_build_ref = environment_ref.get("exact_build_identity_ref")
        if not exact_build_ref:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_missing_exact_build_identity",
                    fixture_rel,
                    row_ref,
                    f"public-proof packet '{packet_id}' has no environment_ref.exact_build_identity_ref",
                    "Every public-proof packet MUST cite an exact-build identity so claim truth is anchored to a reproducible build.",
                    row_ref=packet_id,
                )
            )

        result_class = payload.get("result_class")
        active_reasons = payload.get("active_downgrade_reasons") or []
        reason_required_classes = {
            "narrow_claim_before_publish",
            "retest_pending",
            "fail_claim_blocked",
            "quarantined",
        }
        if result_class in reason_required_classes and not active_reasons:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_narrow_result_without_reason",
                    fixture_rel,
                    row_ref,
                    (
                        f"public-proof packet '{packet_id}' result_class='{result_class}' "
                        "but active_downgrade_reasons is empty"
                    ),
                    "narrow_claim_before_publish / retest_pending / fail_claim_blocked / quarantined packets MUST declare at least one active_downgrade_reason.",
                    row_ref=packet_id,
                )
            )
        if result_class == "pass_full_proof" and active_reasons:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.packet_full_proof_with_reasons",
                    fixture_rel,
                    row_ref,
                    (
                        f"public-proof packet '{packet_id}' claims pass_full_proof but "
                        f"declares active_downgrade_reasons={active_reasons}"
                    ),
                    "pass_full_proof packets MUST NOT carry active_downgrade_reasons.",
                    row_ref=packet_id,
                )
            )

        docs_match = payload.get("docs_help_version_match") or {}
        docs_state = docs_match.get("state")
        if (
            docs_state
            and docs_state not in {"exact_build_match", "compatible_minor_drift"}
            and result_class == "pass_full_proof"
        ):
            findings.append(
                make_finding(
                    "warning",
                    "public_proof_coverage.packet_docs_version_mismatch",
                    fixture_rel,
                    row_ref,
                    (
                        f"public-proof packet '{packet_id}' declares docs_help_version_match.state='{docs_state}' "
                        "while claiming pass_full_proof"
                    ),
                    "Narrow the packet's result_class or raise the docs/help version state to a matching class before publishing.",
                    row_ref=packet_id,
                    docs_state=docs_state,
                )
            )

    # Public-proof destination coverage: every family that publishes to
    # public_proof_packet SHOULD have at least one fixture of the matching
    # scoreboard family in the public-proof fixture set.
    for family in sorted(expects_public_proof):
        if not seen_scoreboard_families:
            findings.append(
                make_finding(
                    "warning",
                    "public_proof_coverage.family_expects_public_proof_no_fixture",
                    ASSURANCE_CLAIM_ROWS_REL,
                    item_ref(family, ASSURANCE_CLAIM_ROWS_REL),
                    (
                        f"assurance claim_subject_family '{family}' lists public_proof_packet "
                        "in publication_destinations but no public-proof packet fixtures were found"
                    ),
                    "Add at least one public-proof packet fixture that projects the family, or remove public_proof_packet from its publication_destinations list.",
                    row_ref=family,
                )
            )
    return findings


def validate_exact_build_coverage(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    assurance = repo.yaml(ASSURANCE_CLAIM_ROWS_REL)

    referenced_refs: set[str] = set()
    for row in manifest.get("claim_rows", []):
        for ref in row.get("exact_build_identity_refs") or []:
            if isinstance(ref, str):
                referenced_refs.add(ref)
    # Scan assurance rows for embedded exact-build references.
    for row in assurance.get("assurance_claims", []):
        for ref in row.get("exact_build_identity_refs") or []:
            if isinstance(ref, str):
                referenced_refs.add(ref)

    # Inspect public-proof packet fixtures to harvest the exact_build refs
    # they already cite.
    for fixture_path in list_public_proof_fixtures(repo):
        try:
            payload = load_json(fixture_path)
        except json.JSONDecodeError:
            continue
        env = payload.get("environment_ref") or {}
        ref = env.get("exact_build_identity_ref")
        if isinstance(ref, str):
            referenced_refs.add(ref)

    fixture_ids: dict[str, str] = {}
    for fixture_path in list_exact_build_fixtures(repo):
        fixture_rel = str(fixture_path.relative_to(repo.root))
        try:
            payload = load_json(fixture_path)
        except json.JSONDecodeError as exc:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.exact_build_fixture_not_valid_json",
                    fixture_rel,
                    fixture_rel,
                    f"exact-build fixture is not valid JSON: {exc}",
                    "Fix the JSON syntax so the validator can read the fixture.",
                )
            )
            continue
        identity_ref = payload.get("exact_build_identity_ref")
        if not identity_ref:
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.exact_build_fixture_missing_identity",
                    fixture_rel,
                    fixture_rel,
                    "exact-build fixture has no exact_build_identity_ref",
                    "Every exact-build fixture MUST declare an exact_build_identity_ref so downstream surfaces can cite it.",
                )
            )
            continue
        fixture_ids[identity_ref] = fixture_rel

    # Warn on claim_manifest-cited identities with no fixture anchor.
    for ref in sorted(referenced_refs):
        if (
            ref.startswith("exact_build_identity_ref.placeholder.")
            or ref.startswith("exact_build_identity.seed.")
        ):
            # Seed / placeholder identities intentionally precede real fixtures.
            continue
        if ref not in fixture_ids:
            findings.append(
                make_finding(
                    "warning",
                    "public_proof_coverage.exact_build_ref_without_fixture",
                    CLAIM_MANIFEST_REL,
                    CLAIM_MANIFEST_REL,
                    f"exact_build_identity_ref '{ref}' is cited but has no fixture anchor",
                    "Either land a fixture in fixtures/build/exact_build_examples/ that declares this identity, or retire the citation.",
                    row_ref=ref,
                )
            )
    return findings


def validate_destination_descriptors(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    descriptor = repo.yaml(DESTINATION_DESCRIPTOR_REL)
    # Smoke check: contract must name destination_descriptor schema.
    boundary_schema = descriptor.get("boundary_schema")
    if boundary_schema and not repo.path_exists(boundary_schema):
        findings.append(
            make_finding(
                "error",
                "public_proof_coverage.destination_boundary_schema_missing",
                DESTINATION_DESCRIPTOR_REL,
                DESTINATION_DESCRIPTOR_REL,
                f"destination_descriptor_seed points at missing boundary_schema '{boundary_schema}'",
                "Keep the destination-descriptor schema reference aligned with the real schema file.",
            )
        )
    # Check every required_product_field is a string (structural guard).
    required_fields = descriptor.get("required_product_fields") or []
    for field_name in required_fields:
        if not isinstance(field_name, str):
            findings.append(
                make_finding(
                    "error",
                    "public_proof_coverage.destination_required_field_malformed",
                    DESTINATION_DESCRIPTOR_REL,
                    DESTINATION_DESCRIPTOR_REL,
                    f"destination_descriptor required_product_fields contains non-string value: {field_name!r}",
                    "Every required_product_field MUST be a string.",
                )
            )
    return findings


def validate_known_limit_alignment(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    known_limit = repo.yaml(KNOWN_LIMIT_CLASSES_REL)
    schema_ref = known_limit.get("schema_ref")
    if schema_ref and not repo.path_exists(schema_ref):
        findings.append(
            make_finding(
                "error",
                "public_proof_coverage.known_limit_schema_missing",
                KNOWN_LIMIT_CLASSES_REL,
                KNOWN_LIMIT_CLASSES_REL,
                f"known_limit_classes points at missing schema_ref '{schema_ref}'",
                "Keep the known-limit schema reference aligned with the real schema file.",
            )
        )
    return findings


def validate_evidence_to_claim(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    assurance = repo.yaml(ASSURANCE_CLAIM_ROWS_REL)
    bundles = repo.yaml(WORKFLOW_BUNDLE_IDS_REL)

    claim_bundle_refs: set[str] = set()
    for row in manifest.get("claim_rows", []):
        for ref in row.get("launch_bundle_refs") or []:
            if isinstance(ref, str):
                claim_bundle_refs.add(ref)
    for row in assurance.get("assurance_claims", []):
        for ref in row.get("workflow_bundle_refs") or []:
            if isinstance(ref, str):
                claim_bundle_refs.add(ref)

    bundle_to_cutlines: dict[str, list[str]] = defaultdict(list)
    for row in bundles.get("workflow_bundles", []):
        bundle_id = row.get("bundle_id")
        if not isinstance(bundle_id, str):
            continue
        for cutline in row.get("cutline_refs") or []:
            if isinstance(cutline, str):
                bundle_to_cutlines[bundle_id].append(cutline)

    for fixture_path in list_public_proof_fixtures(repo):
        fixture_rel = str(fixture_path.relative_to(repo.root))
        try:
            payload = load_json(fixture_path)
        except json.JSONDecodeError:
            continue
        packet_id = payload.get("packet_id") or fixture_path.name
        row_ref = item_ref(packet_id, fixture_rel)
        bundle_id = (payload.get("workflow_bundle_ref") or {}).get("bundle_id")
        cutline_ref = payload.get("cutline_ref")
        if (
            isinstance(bundle_id, str)
            and bundle_id not in claim_bundle_refs
            and isinstance(cutline_ref, str)
            and cutline_ref not in {
                cutline
                for cutlines in bundle_to_cutlines.values()
                for cutline in cutlines
            }
        ):
            findings.append(
                make_finding(
                    "warning",
                    "public_proof_coverage.evidence_without_claim_binding",
                    fixture_rel,
                    row_ref,
                    (
                        f"public-proof packet '{packet_id}' cites bundle '{bundle_id}' "
                        f"and cutline '{cutline_ref}' that no claim or assurance row binds"
                    ),
                    "Add a claim_manifest_seed launch_bundle_ref, an assurance_claim_rows workflow_bundle_ref, or retire the orphan packet.",
                    row_ref=packet_id,
                )
            )
    return findings


def group_findings(findings: list[Finding]) -> dict[str, dict[str, int]]:
    grouped: dict[str, dict[str, int]] = defaultdict(lambda: {"error": 0, "warning": 0})
    for finding in findings:
        grouped[finding.check_id][finding.severity] += 1
    return grouped


def render_human_summary(
    findings: list[Finding],
    check_count: int,
) -> str:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    warning_count = sum(1 for finding in findings if finding.severity == "warning")
    status = "ok" if error_count == 0 else "failure"
    lines = [
        f"[public-proof-coverage] {status} ({check_count} checks, {error_count} errors, {warning_count} warnings)",
    ]
    if not findings:
        lines.append("[public-proof-coverage] no findings")
        return "\n".join(lines) + "\n"
    for finding in sorted(
        findings,
        key=lambda item: (item.severity != "error", item.check_id, item.owner_artifact_ref),
    ):
        lines.append(
            f"[{finding.severity.upper()}] {finding.check_id} :: {finding.message}"
        )
        lines.append(f"  artifact: {finding.artifact_ref}")
        lines.append(f"  owner:    {finding.owner_artifact_ref}")
        if finding.row_ref:
            lines.append(f"  row:      {finding.row_ref}")
        lines.append(f"  fix:      {finding.remediation}")
    return "\n".join(lines) + "\n"


def build_report(
    findings: list[Finding],
    check_ids: list[str],
    repo_root: Path,
    covered_families: list[str],
    uncovered_families: list[str],
    packet_summary: dict[str, Any],
) -> dict[str, Any]:
    grouped = group_findings(findings)
    return {
        "report_kind": "public_proof_coverage_report",
        "schema_version": 1,
        "generated_at": now_utc(),
        "repo_root": str(repo_root),
        "summary": {
            "check_count": len(check_ids),
            "error_count": sum(1 for finding in findings if finding.severity == "error"),
            "warning_count": sum(1 for finding in findings if finding.severity == "warning"),
            "contract_families_total": len(CONTRACT_FAMILIES),
            "contract_families_covered": len(covered_families),
            "contract_families_uncovered": len(uncovered_families),
        },
        "coverage": {
            "contract_families_covered": covered_families,
            "contract_families_uncovered": uncovered_families,
            "public_proof_packets": packet_summary,
        },
        "checks": [
            {
                "check_id": check_id,
                "error_count": grouped[check_id]["error"],
                "warning_count": grouped[check_id]["warning"],
                "status": "fail"
                if grouped[check_id]["error"]
                else ("warn" if grouped[check_id]["warning"] else "pass"),
            }
            for check_id in check_ids
        ],
        "findings": [finding.as_report() for finding in findings],
    }


def collect_packet_summary(repo: RepoView) -> dict[str, Any]:
    fixtures = list_public_proof_fixtures(repo)
    summary: dict[str, Any] = {
        "fixture_count": len(fixtures),
        "by_scoreboard_family": {},
        "by_bundle_id": {},
        "by_result_class": {},
    }
    by_family: Counter[str] = Counter()
    by_bundle: Counter[str] = Counter()
    by_result: Counter[str] = Counter()
    for fixture_path in fixtures:
        try:
            payload = load_json(fixture_path)
        except json.JSONDecodeError:
            continue
        family = payload.get("scoreboard_family_id") or "<missing>"
        bundle = (payload.get("workflow_bundle_ref") or {}).get("bundle_id") or "<missing>"
        result = payload.get("result_class") or "<missing>"
        by_family[family] += 1
        by_bundle[bundle] += 1
        by_result[result] += 1
    summary["by_scoreboard_family"] = dict(sorted(by_family.items()))
    summary["by_bundle_id"] = dict(sorted(by_bundle.items()))
    summary["by_result_class"] = dict(sorted(by_result.items()))
    return summary


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    repo = RepoView(repo_root)

    if not repo.exists(REQUIREMENT_REGISTER_REL):
        sys.stderr.write(
            f"[public-proof-coverage] error: missing {REQUIREMENT_REGISTER_REL}\n"
        )
        return 2
    if not repo.exists(CLAIM_MANIFEST_REL):
        sys.stderr.write(
            f"[public-proof-coverage] error: missing {CLAIM_MANIFEST_REL}\n"
        )
        return 2
    if not repo.exists(ASSURANCE_CLAIM_ROWS_REL):
        sys.stderr.write(
            f"[public-proof-coverage] error: missing {ASSURANCE_CLAIM_ROWS_REL}\n"
        )
        return 2

    requirement_ids = load_requirement_ids(repo)
    assurance_findings, by_family = validate_assurance_rows(repo)

    covered_families: list[str] = []
    uncovered_families: list[str] = []
    for family, admissible in CONTRACT_FAMILIES.items():
        if any(subject in by_family for subject in admissible):
            covered_families.append(family)
        else:
            uncovered_families.append(family)

    checks: list[tuple[str, list[Finding]]] = [
        ("requirement_coverage", validate_requirement_coverage(repo, requirement_ids)),
        ("claim_manifest_known_limits", validate_claim_manifest_known_limits(repo)),
        ("claim_manifest_docs_version", validate_claim_manifest_docs_version(repo)),
        ("assurance_rows", assurance_findings),
        ("contract_family_coverage", validate_contract_family_coverage(by_family)),
        ("public_proof_packets", validate_public_proof_packets(repo, by_family)),
        ("exact_build_coverage", validate_exact_build_coverage(repo)),
        ("destination_descriptors", validate_destination_descriptors(repo)),
        ("known_limit_alignment", validate_known_limit_alignment(repo)),
        ("evidence_to_claim_binding", validate_evidence_to_claim(repo)),
    ]

    all_findings = [finding for _, findings in checks for finding in findings]
    human = render_human_summary(all_findings, len(checks))
    sys.stdout.write(human)

    if args.report:
        check_ids = [check_id for check_id, _ in checks]
        report_path = repo.rel(args.report)
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(
                build_report(
                    all_findings,
                    check_ids,
                    repo_root,
                    sorted(covered_families),
                    sorted(uncovered_families),
                    collect_packet_summary(repo),
                ),
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )

    return 1 if any(finding.severity == "error" for finding in all_findings) else 0


if __name__ == "__main__":
    sys.exit(main())
