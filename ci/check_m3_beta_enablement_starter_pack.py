#!/usr/bin/env python3
"""Validate the M3 beta enablement starter pack.

This is the first consumer for the four checked-in artifacts that make
the M3 extension-author, design-partner, and community starter pack
real:

  - artifacts/milestones/m3/beta_enablement_starter_pack.yaml
  - docs/extensions/m3/beta_starter_pack.md
  - docs/partners/m3/design_partner_beta_pack.md
  - docs/community/m3/issue_rfc_routing_beta.md

The validator parses the canonical YAML, cross-checks every cohort,
surface, and issue-class ref against the upstream canonical sources
(claimed-surface register, cohort guardrails, issue routing), confirms
each entrypoint doc cites the right lane vocabulary, and writes a
machine-readable capture so downstream consumers can downgrade stale
rows without reviewer interpretation.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_PACK_REL = "artifacts/milestones/m3/beta_enablement_starter_pack.yaml"
DEFAULT_REGISTER_REL = "artifacts/milestones/m3/claimed_surface_register.json"
DEFAULT_COHORTS_REL = "artifacts/milestones/m3/cohort_guardrails.yaml"
DEFAULT_ISSUE_ROUTING_REL = "artifacts/governance/issue_routing.yaml"
DEFAULT_CAPTURE_REL = (
    "artifacts/milestones/m3/captures/"
    "beta_enablement_starter_pack_validation_capture.json"
)

REQUIRED_LANE_IDS = {
    "starter_pack_lane:extension_author",
    "starter_pack_lane:design_partner",
    "starter_pack_lane:community",
}

PATH_LIKE_SUFFIXES = (
    ".yaml",
    ".yml",
    ".json",
    ".md",
    ".toml",
    ".rs",
    ".py",
    ".mmd",
)
ID_PREFIXES = (
    "archetype_row:",
    "beta_archetype:",
    "beta_surface:",
    "cohort:",
    "compat_row:",
    "issue_class:",
    "m3_cutline:",
    "m4_unlock:",
    "starter_pack_lane:",
)


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--pack", default=DEFAULT_PACK_REL)
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--cohorts", default=DEFAULT_COHORTS_REL)
    parser.add_argument("--issue-routing", default=DEFAULT_ISSUE_ROUTING_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail when the on-disk validation capture would change after "
            "regeneration. Use this in CI to keep the capture committed."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
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
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(
            f"{label} must be a YYYY-MM-DD date, got {value!r}"
        ) from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_refs(
    repo_root: Path,
    refs: list[Any],
    label: str,
    findings: list[Finding],
) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation=(
                        "Replace the empty or non-string ref with a "
                        "repo-relative artifact path or stable row id."
                    ),
                )
            )
            continue
        ref = raw_ref.strip()
        if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_ref",
                    message=f"{label}[{idx}] does not resolve: {ref}",
                    remediation=(
                        "Fix the path or seed the referenced artifact so the "
                        "starter pack lane stays inspectable."
                    ),
                    ref=ref,
                )
            )


def collect_register_surface_ids(register: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        register.get("claimed_surfaces"), "register.claimed_surfaces"
    )
    return {
        ensure_str(
            row.get("surface_id"),
            f"register.claimed_surfaces[{idx}].surface_id",
        )
        for idx, row in enumerate(rows)
    }


def collect_cohort_ids(cohorts: dict[str, Any]) -> set[str]:
    rows = ensure_list(cohorts.get("cohorts"), "cohorts.cohorts")
    return {
        ensure_str(row.get("cohort_id"), f"cohorts.cohorts[{idx}].cohort_id")
        for idx, row in enumerate(rows)
    }


def collect_issue_class_ids(issue_routing: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        issue_routing.get("issue_classes"), "issue_routing.issue_classes"
    )
    return {
        ensure_str(
            row.get("id"), f"issue_routing.issue_classes[{idx}].id"
        )
        for idx, row in enumerate(rows)
    }


def collect_route_class_ids(issue_routing: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        issue_routing.get("route_classes"), "issue_routing.route_classes"
    )
    return {
        ensure_str(
            row.get("id"), f"issue_routing.route_classes[{idx}].id"
        )
        for idx, row in enumerate(rows)
    }


def collect_transition_ids(issue_routing: dict[str, Any]) -> set[str]:
    rows = issue_routing.get("disclosure_transitions", [])
    if not isinstance(rows, list):
        return set()
    out: set[str] = set()
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            continue
        value = row.get("id")
        if isinstance(value, str) and value.strip():
            out.add(value.strip())
    return out


def validate_header(
    repo_root: Path,
    pack: dict[str, Any],
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(
        pack.get("schema_version"), "pack.schema_version"
    )
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.schema_version.unsupported",
                message=f"schema_version must be 1, got {schema_version}",
                remediation=(
                    "Bump the validator in the same change that bumps "
                    "schema_version."
                ),
            )
        )

    parse_iso_date(
        ensure_str(pack.get("as_of"), "pack.as_of"), "pack.as_of"
    )
    ensure_str(pack.get("owner"), "pack.owner")
    ensure_str(pack.get("starter_pack_id"), "pack.starter_pack_id")
    ensure_str(pack.get("milestone_id"), "pack.milestone_id")
    if ensure_str(pack.get("scope_state"), "pack.scope_state") != "frozen":
        findings.append(
            Finding(
                severity="error",
                check_id="pack.scope_state.not_frozen",
                message="pack.scope_state must be frozen",
                remediation=(
                    "Set scope_state to frozen once the starter pack is "
                    "locked for the beta train."
                ),
            )
        )

    entrypoints = ensure_dict(
        pack.get("human_entrypoint_refs"), "pack.human_entrypoint_refs"
    )
    required_keys = {"extension_author", "design_partner", "community"}
    missing = required_keys - set(entrypoints.keys())
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.human_entrypoint_refs.missing",
                message=(
                    "human_entrypoint_refs is missing required audience "
                    "keys"
                ),
                remediation=(
                    "Name an entrypoint doc for extension_author, "
                    "design_partner, and community in the same change set."
                ),
                details={"missing": sorted(missing)},
            )
        )
    for key, value in entrypoints.items():
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.human_entrypoint_refs.invalid",
                    message=(
                        f"human_entrypoint_refs[{key!r}] must be a "
                        "non-empty string"
                    ),
                    remediation="Set the entry doc path or fix the typo.",
                    ref=key,
                )
            )
            continue
        if not artifact_ref_exists(repo_root, value):
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.human_entrypoint_refs.missing_file",
                    message=(
                        f"human_entrypoint_refs[{key!r}] does not exist: "
                        f"{value}"
                    ),
                    remediation=(
                        "Author the entrypoint doc before running the "
                        "validator."
                    ),
                    ref=value,
                )
            )

    upstream = ensure_dict(
        pack.get("upstream_canonical_refs"), "pack.upstream_canonical_refs"
    )
    validate_path_refs(
        repo_root,
        list(upstream.values()),
        "pack.upstream_canonical_refs",
        findings,
    )

    validator_block = ensure_dict(pack.get("validator"), "pack.validator")
    validate_path_refs(
        repo_root,
        [validator_block.get("script_ref")],
        "pack.validator.script_ref",
        findings,
    )


def validate_lane(
    repo_root: Path,
    lane: dict[str, Any],
    idx: int,
    register_surface_ids: set[str],
    cohort_ids: set[str],
    issue_class_ids: set[str],
    transition_ids: set[str],
    findings: list[Finding],
) -> dict[str, Any]:
    label = f"pack.starter_pack_lanes[{idx}]"
    lane_id = ensure_str(lane.get("lane_id"), f"{label}.lane_id")
    ensure_str(lane.get("title"), f"{label}.title")
    ensure_str(lane.get("audience"), f"{label}.audience")
    entrypoint = ensure_str(
        lane.get("human_entrypoint_ref"),
        f"{label}.human_entrypoint_ref",
    )
    if not artifact_ref_exists(repo_root, entrypoint):
        findings.append(
            Finding(
                severity="error",
                check_id="pack.lane.human_entrypoint_ref.missing",
                message=(
                    f"{lane_id} human_entrypoint_ref does not exist: "
                    f"{entrypoint}"
                ),
                remediation=(
                    "Author the entrypoint doc or fix the path so the lane "
                    "is inspectable."
                ),
                ref=entrypoint,
            )
        )

    cohort_keys = (
        "primary_cohort_refs",
        "secondary_cohort_refs",
    )
    cited_cohorts: list[str] = []
    for key in cohort_keys:
        if key not in lane:
            continue
        refs = ensure_list(lane.get(key), f"{label}.{key}")
        for ref_idx, raw_ref in enumerate(refs):
            ref = ensure_str(raw_ref, f"{label}.{key}[{ref_idx}]")
            cited_cohorts.append(ref)
            if ref not in cohort_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="pack.lane.cohort_ref.unknown",
                        message=(
                            f"{lane_id} {key}[{ref_idx}] cites unknown "
                            f"cohort_id: {ref}"
                        ),
                        remediation=(
                            "Use a cohort_id from cohort_guardrails.yaml or "
                            "add the cohort there in the same change set."
                        ),
                        ref=ref,
                    )
                )

    if lane.get("audience") in {
        "extension_authors",
        "design_partners_and_managed_pilots",
    }:
        if not lane.get("primary_cohort_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.lane.primary_cohort_refs.empty",
                    message=(
                        f"{lane_id} must name at least one primary cohort"
                    ),
                    remediation=(
                        "Bind the lane to a cohort_id from "
                        "cohort_guardrails.yaml."
                    ),
                    ref=lane_id,
                )
            )

    surface_keys = ("primary_surface_refs", "secondary_surface_refs")
    cited_surfaces: list[str] = []
    for key in surface_keys:
        if key not in lane:
            continue
        refs = ensure_list(lane.get(key), f"{label}.{key}")
        for ref_idx, raw_ref in enumerate(refs):
            ref = ensure_str(raw_ref, f"{label}.{key}[{ref_idx}]")
            cited_surfaces.append(ref)
            if ref not in register_surface_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="pack.lane.surface_ref.unknown",
                        message=(
                            f"{lane_id} {key}[{ref_idx}] cites unknown "
                            f"surface_id: {ref}"
                        ),
                        remediation=(
                            "Use a beta_surface: id from "
                            "claimed_surface_register.json, or add it "
                            "first."
                        ),
                        ref=ref,
                    )
                )

    issue_refs = lane.get("issue_routing_class_refs")
    if issue_refs is None:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.lane.issue_routing_class_refs.missing",
                message=(
                    f"{lane_id} must name at least one issue_routing "
                    "class"
                ),
                remediation=(
                    "Cite one or more issue_classes[].id from "
                    "artifacts/governance/issue_routing.yaml."
                ),
                ref=lane_id,
            )
        )
    else:
        refs = ensure_list(
            issue_refs, f"{label}.issue_routing_class_refs"
        )
        if not refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.lane.issue_routing_class_refs.empty",
                    message=(
                        f"{lane_id} must name at least one issue_routing "
                        "class"
                    ),
                    remediation=(
                        "Cite one or more issue_classes[].id from "
                        "artifacts/governance/issue_routing.yaml."
                    ),
                    ref=lane_id,
                )
            )
        for ref_idx, raw_ref in enumerate(refs):
            ref = ensure_str(
                raw_ref, f"{label}.issue_routing_class_refs[{ref_idx}]"
            )
            if ref not in issue_class_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="pack.lane.issue_routing_class_refs.unknown",
                        message=(
                            f"{lane_id} cites unknown issue_class id: {ref}"
                        ),
                        remediation=(
                            "Use an issue_classes[].id from "
                            "artifacts/governance/issue_routing.yaml."
                        ),
                        ref=ref,
                    )
                )

    transition_refs = lane.get("private_to_public_transition_refs", [])
    if not isinstance(transition_refs, list):
        transition_refs = []
    for ref_idx, raw_ref in enumerate(transition_refs):
        ref = ensure_str(
            raw_ref,
            f"{label}.private_to_public_transition_refs[{ref_idx}]",
        )
        if transition_ids and ref not in transition_ids:
            findings.append(
                Finding(
                    severity="warning",
                    check_id="pack.lane.disclosure_transition.unknown",
                    message=(
                        f"{lane_id} cites unknown disclosure transition: "
                        f"{ref}"
                    ),
                    remediation=(
                        "Use a disclosure_transitions[].id from "
                        "artifacts/governance/issue_routing.yaml or remove "
                        "the ref."
                    ),
                    ref=ref,
                )
            )

    path_ref_keys = (
        "sdk_overview_refs",
        "compatibility_refs",
        "sample_pack_refs",
        "revocation_and_support_refs",
        "known_limits_refs",
        "rollback_and_support_refs",
        "cohort_guardrail_refs",
        "upstream_evidence_refs",
        "routing_matrix_refs",
        "contributor_entry_refs",
        "truth_vocabulary_refs",
        "escalation_refs",
    )
    for key in path_ref_keys:
        if key not in lane:
            continue
        refs = ensure_list(lane.get(key), f"{label}.{key}")
        validate_path_refs(repo_root, refs, f"{label}.{key}", findings)

    single_path_keys = (
        "scorecard_ref",
        "managed_pilot_scorecard_ref",
        "cohort_scorecard_index_ref",
        "review_packet_template_ref",
    )
    for key in single_path_keys:
        value = lane.get(key)
        if value is None:
            continue
        validate_path_refs(repo_root, [value], f"{label}.{key}", findings)

    if lane.get("audience") == "extension_authors":
        for key in (
            "sdk_overview_refs",
            "compatibility_refs",
            "sample_pack_refs",
            "revocation_and_support_refs",
        ):
            value = lane.get(key)
            if not isinstance(value, list) or not value:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"pack.lane.{key}.empty",
                        message=(
                            f"{lane_id} must name at least one ref in {key}"
                        ),
                        remediation=(
                            "Add the SDK, compatibility, sample pack, or "
                            "revocation/support refs the lane is meant to "
                            "carry."
                        ),
                        ref=lane_id,
                    )
                )

    if lane.get("audience") == "design_partners_and_managed_pilots":
        for key in (
            "scorecard_ref",
            "managed_pilot_scorecard_ref",
            "cohort_scorecard_index_ref",
            "known_limits_refs",
            "rollback_and_support_refs",
            "compatibility_refs",
        ):
            value = lane.get(key)
            if value is None or (isinstance(value, list) and not value):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"pack.lane.{key}.empty",
                        message=(
                            f"{lane_id} must name {key} for design-partner "
                            "scope"
                        ),
                        remediation=(
                            "Bind the lane to the scorecard, known limits, "
                            "rollback / support, and compatibility refs the "
                            "partner pack consumes."
                        ),
                        ref=lane_id,
                    )
                )

    if lane.get("audience") == "community_contributors":
        for key in (
            "routing_matrix_refs",
            "contributor_entry_refs",
            "truth_vocabulary_refs",
        ):
            value = lane.get(key)
            if not isinstance(value, list) or not value:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"pack.lane.{key}.empty",
                        message=(
                            f"{lane_id} must name at least one ref in {key}"
                        ),
                        remediation=(
                            "Cite the routing matrix, CONTRIBUTING/SECURITY "
                            "entry points, and M3 truth vocabulary."
                        ),
                        ref=lane_id,
                    )
                )

    return {
        "lane_id": lane_id,
        "entrypoint_ref": entrypoint,
        "audience": lane.get("audience"),
        "cited_cohorts": cited_cohorts,
        "cited_surfaces": cited_surfaces,
        "issue_routing_class_refs": list(
            lane.get("issue_routing_class_refs") or []
        ),
    }


def validate_entrypoint_doc(
    repo_root: Path,
    lane: dict[str, Any],
    findings: list[Finding],
) -> None:
    entrypoint = lane.get("entrypoint_ref")
    if not entrypoint:
        return
    doc_path = repo_root / entrypoint
    if not doc_path.exists():
        return  # already reported by validate_lane
    body = doc_path.read_text(encoding="utf-8")
    lane_id = lane["lane_id"]
    required = list(lane.get("cited_cohorts") or [])
    required.extend(lane.get("cited_surfaces") or [])
    required.extend(lane.get("issue_routing_class_refs") or [])
    for ref in required:
        if ref not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.entrypoint_doc.missing_ref",
                    message=(
                        f"{lane_id} entrypoint doc is missing required ref: "
                        f"{ref}"
                    ),
                    remediation=(
                        "Cite the cohort, surface, or issue_class id in the "
                        "entrypoint doc so reviewers can resolve the lane "
                        "by name."
                    ),
                    ref=entrypoint,
                )
            )
    audience = lane.get("audience")
    if audience == "extension_authors":
        required_phrases = [
            "docs/extensions/sdk_publication_contract.md",
            "artifacts/compat/m3/compatibility_report.md",
            "docs/extensions/extension_lifecycle_and_quarantine_sequence.md",
            "ci/check_m3_beta_enablement_starter_pack.py",
        ]
    elif audience == "design_partners_and_managed_pilots":
        required_phrases = [
            "artifacts/milestones/m3/cohorts/design_partner_scorecard.md",
            "artifacts/milestones/m3/cohorts/managed_pilot_scorecard.md",
            "docs/release/update_and_rollback_contract.md",
            "ci/check_m3_beta_enablement_starter_pack.py",
        ]
    elif audience == "community_contributors":
        required_phrases = [
            "docs/governance/issue_routing_matrix.md",
            "artifacts/governance/issue_routing.yaml",
            "CONTRIBUTING.md",
            "SECURITY.md",
            "ci/check_m3_beta_enablement_starter_pack.py",
        ]
    else:
        required_phrases = []
    for phrase in required_phrases:
        if phrase not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.entrypoint_doc.missing_phrase",
                    message=(
                        f"{lane_id} entrypoint doc is missing required "
                        f"phrase: {phrase!r}"
                    ),
                    remediation=(
                        "Update the doc so it carries the canonical cross-"
                        "references the validator enforces."
                    ),
                    ref=entrypoint,
                )
            )


def validate_change_control(
    pack: dict[str, Any],
    findings: list[Finding],
) -> None:
    change_control = ensure_dict(
        pack.get("change_control"), "pack.change_control"
    )
    addition = set(
        ensure_list(
            change_control.get("addition_requires_refs"),
            "pack.change_control.addition_requires_refs",
        )
    )
    required = {
        "artifacts/milestones/m3/beta_enablement_starter_pack.yaml",
        "artifacts/milestones/m3/claimed_surface_register.json",
        "artifacts/milestones/m3/cohort_guardrails.yaml",
        "docs/milestones/m3/beta_admission_matrix.md",
        "artifacts/milestones/m3/dependency_graph.mmd",
        "docs/extensions/m3/beta_starter_pack.md",
        "docs/partners/m3/design_partner_beta_pack.md",
        "docs/community/m3/issue_rfc_routing_beta.md",
        "artifacts/milestones/m3/open_project_beta_packet.md",
        "docs/governance/m3/standards_interchange_matrix.md",
    }
    missing = required - addition
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.change_control.missing_addition_refs",
                message=(
                    "change_control.addition_requires_refs is missing "
                    "required artifacts"
                ),
                remediation=(
                    "Add the starter-pack YAML, claimed-surface register, "
                    "cohort guardrails, beta admission matrix, dependency "
                    "graph, the three entrypoint docs, and the open-project "
                    "beta packet/standards publication docs."
                ),
                details={"missing": sorted(missing)},
            )
        )


def write_capture(
    repo_root: Path,
    capture_rel: str,
    pack_rel: str,
    lane_summaries: list[dict[str, Any]],
    findings: list[Finding],
    generated_at: str,
    check_only: bool,
) -> bool:
    capture_path = repo_root / capture_rel
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m3_beta_enablement_starter_pack",
        "generated_at": generated_at,
        "pack_ref": pack_rel,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(
                1 for f in findings if f.severity == "warning"
            ),
        },
        "findings": [f.as_report() for f in findings],
        "lane_summaries": lane_summaries,
    }
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text: str | None = None
    if capture_path.exists():
        old_text = capture_path.read_text(encoding="utf-8")
    changed = old_text is None or _normalize(old_text) != _normalize(
        new_text
    )
    if not check_only:
        capture_path.write_text(new_text, encoding="utf-8")
    return changed


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub(
        '"generated_at": "__generated_at__"', text
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    pack = ensure_dict(
        render_yaml_as_json(repo_root / args.pack), "pack"
    )
    register = ensure_dict(
        load_json(repo_root / args.register), "register"
    )
    cohorts = ensure_dict(
        render_yaml_as_json(repo_root / args.cohorts), "cohorts"
    )
    issue_routing = ensure_dict(
        render_yaml_as_json(repo_root / args.issue_routing),
        "issue_routing",
    )

    register_surface_ids = collect_register_surface_ids(register)
    cohort_ids = collect_cohort_ids(cohorts)
    issue_class_ids = collect_issue_class_ids(issue_routing)
    transition_ids = collect_transition_ids(issue_routing)

    findings: list[Finding] = []
    validate_header(repo_root, pack, findings)

    lanes = ensure_list(
        pack.get("starter_pack_lanes"), "pack.starter_pack_lanes"
    )
    lane_summaries: list[dict[str, Any]] = []
    seen_lane_ids: set[str] = set()
    for idx, raw_lane in enumerate(lanes):
        lane = ensure_dict(raw_lane, f"pack.starter_pack_lanes[{idx}]")
        summary = validate_lane(
            repo_root,
            lane,
            idx,
            register_surface_ids,
            cohort_ids,
            issue_class_ids,
            transition_ids,
            findings,
        )
        if summary["lane_id"] in seen_lane_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="pack.starter_pack_lanes.duplicate_lane_id",
                    message=f"duplicate lane_id: {summary['lane_id']}",
                    remediation="Use one row per starter-pack lane.",
                    ref=summary["lane_id"],
                )
            )
        seen_lane_ids.add(summary["lane_id"])
        lane_summaries.append(summary)

    missing_required = REQUIRED_LANE_IDS - seen_lane_ids
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="pack.starter_pack_lanes.missing_required",
                message=(
                    "starter_pack_lanes is missing required lanes: "
                    f"{sorted(missing_required)}"
                ),
                remediation=(
                    "Land the extension-author, design-partner, and "
                    "community lanes in the same change set."
                ),
                details={"missing": sorted(missing_required)},
            )
        )

    for summary in lane_summaries:
        validate_entrypoint_doc(repo_root, summary, findings)

    validate_change_control(pack, findings)

    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )
    changed = write_capture(
        repo_root,
        args.capture,
        args.pack,
        lane_summaries,
        findings,
        generated_at,
        check_only=args.check,
    )
    if args.check and changed:
        print(
            "ERROR validation capture is stale on disk; re-run without "
            "--check and commit the regenerated capture.",
            file=sys.stderr,
        )
        return 1

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(
                f"ERROR [{item.check_id}]{ref}: {item.message}",
                file=sys.stderr,
            )
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    warnings = [item for item in findings if item.severity == "warning"]
    for item in warnings:
        ref = f" ({item.ref})" if item.ref else ""
        print(
            f"WARNING [{item.check_id}]{ref}: {item.message}",
            file=sys.stderr,
        )

    print("m3 beta enablement starter pack validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
