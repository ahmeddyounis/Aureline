#!/usr/bin/env python3
"""Validate the M3 public-proof artifact index and emit its validation capture.

This is the first consumer for the three artifacts in the M3 public-proof
review batch:

  - artifacts/milestones/m3/public_proof_index.md
  - artifacts/milestones/m3/review_packet_template.md
  - docs/governance/m3/publication_shelf_life_policy.md

The validator parses the canonical YAML block embedded in the public-proof
index, cross-checks the rows against the M3 claim manifest, the
cross-milestone evidence-freshness SLO catalog, and the cross-milestone
rerun-trigger catalog, then writes a machine-readable validation capture
plus a derived downgrade matrix so downstream surfaces can downgrade
stale rows without re-reading prose.
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


DEFAULT_INDEX_REL = "artifacts/milestones/m3/public_proof_index.md"
DEFAULT_TEMPLATE_REL = "artifacts/milestones/m3/review_packet_template.md"
DEFAULT_POLICY_REL = "docs/governance/m3/publication_shelf_life_policy.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/milestones/m3/captures/public_proof_index_validation_capture.json"
)

CANONICAL_BLOCK_BEGIN = "<!-- BEGIN canonical:public_proof_index -->"
CANONICAL_BLOCK_END = "<!-- END canonical:public_proof_index -->"

REQUIRED_TEMPLATE_PHRASES = [
    "evidence_packet_header",
    "packet_family: m3_public_proof_review_packet",
    "Claim-row pass / fail state",
    "Waivers",
    "Owner signoff",
    "Freshness and rerun triggers",
    "schemas/governance/evidence_packet_header.schema.json",
    "artifacts/governance/evidence_rerun_triggers.yaml",
]

REQUIRED_POLICY_PHRASES = [
    "M3 publication shelf-life policy",
    "Freshness windows per claim family",
    "Rerun-trigger ids per claim family",
    "Automatic downgrade behavior",
    "Failure drill",
    "artifacts/governance/evidence_freshness_slos.yaml",
    "artifacts/governance/evidence_rerun_triggers.yaml",
    "artifacts/milestones/m3/public_proof_index.md",
]

STALE_AFTER_PATTERN = re.compile(r"^P(\d+)D$")


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
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
    parser.add_argument("--template", default=DEFAULT_TEMPLATE_REL)
    parser.add_argument("--policy", default=DEFAULT_POLICY_REL)
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


def render_yaml_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(STDIN.read,"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {label}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {label}: {exc}"
        ) from exc


def render_yaml_file_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(
            f"{label} must be a YYYY-MM-DD date, got {value!r}"
        ) from exc


def stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_PATTERN.match(value)
    if not match:
        raise SystemExit(
            f"{label} must be of the form P<N>D, got {value!r}"
        )
    return int(match.group(1))


def extract_canonical_block(index_text: str) -> str:
    if CANONICAL_BLOCK_BEGIN not in index_text:
        raise SystemExit(
            "public-proof index is missing the BEGIN canonical sentinel"
        )
    if CANONICAL_BLOCK_END not in index_text:
        raise SystemExit(
            "public-proof index is missing the END canonical sentinel"
        )
    after_begin = index_text.split(CANONICAL_BLOCK_BEGIN, 1)[1]
    block = after_begin.split(CANONICAL_BLOCK_END, 1)[0]
    if "```yaml" not in block or "```" not in block.split("```yaml", 1)[1]:
        raise SystemExit(
            "public-proof index canonical block must wrap one ```yaml ... ``` fence"
        )
    yaml_body = block.split("```yaml", 1)[1].split("```", 1)[0]
    if not yaml_body.strip():
        raise SystemExit("public-proof index canonical YAML block is empty")
    return yaml_body


def is_under_any_root(ref: str, roots: list[str]) -> bool:
    target = ref.split("#", 1)[0].strip()
    if not target:
        return False
    for root in roots:
        root_path = root.rstrip("/").strip()
        if not root_path:
            continue
        if target == root_path or target.startswith(root_path + "/"):
            return True
    return False


def ref_exists(repo_root: Path, ref: str) -> bool:
    target = ref.split("#", 1)[0].strip()
    return bool(target) and (repo_root / target).exists()


def index_proof_class_ceilings(slo_catalog: dict[str, Any]) -> dict[str, int]:
    rows = ensure_list(slo_catalog.get("proof_classes"), "slo_catalog.proof_classes")
    out: dict[str, int] = {}
    for row in rows:
        proof_class_id = ensure_str(
            row.get("proof_class_id"), "slo_catalog.proof_classes[].proof_class_id"
        )
        max_stale_after = ensure_str(
            row.get("max_stale_after"),
            f"slo_catalog.proof_classes[{proof_class_id}].max_stale_after",
        )
        out[proof_class_id] = stale_after_days(
            max_stale_after,
            f"slo_catalog.proof_classes[{proof_class_id}].max_stale_after",
        )
    return out


def index_stale_propagation_profiles(slo_catalog: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        slo_catalog.get("stale_propagation_profiles"),
        "slo_catalog.stale_propagation_profiles",
    )
    return {
        ensure_str(
            row.get("profile_id"),
            "slo_catalog.stale_propagation_profiles[].profile_id",
        )
        for row in rows
    }


def index_trigger_ids(trigger_catalog: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        trigger_catalog.get("trigger_rows"),
        "trigger_catalog.trigger_rows",
    )
    return {
        ensure_str(row.get("trigger_id"), "trigger_catalog.trigger_rows[].trigger_id")
        for row in rows
    }


def index_claim_families(claim_manifest: dict[str, Any]) -> set[str]:
    rows = ensure_list(claim_manifest.get("rows"), "claim_manifest.rows")
    return {
        ensure_str(row.get("claim_family"), "claim_manifest.rows[].claim_family")
        for row in rows
    }


def validate_storage_roots(
    repo_root: Path,
    index: dict[str, Any],
    findings: list[Finding],
) -> list[str]:
    roots = ensure_list(index.get("storage_roots"), "index.storage_roots")
    refs: list[str] = []
    for idx, raw in enumerate(roots):
        row = ensure_dict(raw, f"index.storage_roots[{idx}]")
        root_id = ensure_str(row.get("root_id"), f"index.storage_roots[{idx}].root_id")
        root_ref = ensure_str(
            row.get("root_ref"), f"index.storage_roots[{idx}].root_ref"
        )
        refs.append(root_ref)
        if not ref_exists(repo_root, root_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.storage_roots.missing",
                    message=(
                        f"storage root {root_id} does not exist: {root_ref}"
                    ),
                    remediation=(
                        "Create the directory (or fix the path) so evidence "
                        "has a governed sink."
                    ),
                    ref=root_ref,
                )
            )
    if not refs:
        findings.append(
            Finding(
                severity="error",
                check_id="index.storage_roots.empty",
                message="storage_roots must declare at least one root",
                remediation=(
                    "Declare the M3 public-proof evidence roots so packets "
                    "have governed sinks."
                ),
            )
        )
    return refs


def validate_header(
    repo_root: Path,
    index: dict[str, Any],
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(
        index.get("schema_version"), "index.schema_version"
    )
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="index.schema_version.unsupported",
                message=(
                    f"schema_version must be 1, got {schema_version}"
                ),
                remediation=(
                    "Bump the validator in the same change that bumps "
                    "schema_version."
                ),
            )
        )

    as_of = ensure_str(index.get("as_of"), "index.as_of")
    _ = parse_iso_date(as_of, "index.as_of")
    _ = ensure_str(index.get("owner"), "index.owner")
    _ = ensure_str(index.get("index_id"), "index.index_id")
    _ = ensure_str(index.get("milestone_id"), "index.milestone_id")
    _ = ensure_str(
        index.get("release_channel_scope"), "index.release_channel_scope"
    )

    for ref_field in (
        "human_entrypoint_ref",
        "review_packet_template_ref",
        "publication_shelf_life_policy_ref",
        "validator_ref",
        "validation_capture_ref",
        "claim_manifest_source",
        "slo_catalog_source",
        "rerun_trigger_catalog_source",
    ):
        ref_value = ensure_str(index.get(ref_field), f"index.{ref_field}")
        if ref_field == "validation_capture_ref":
            continue  # The validator writes this, so it may not exist yet.
        if not ref_exists(repo_root, ref_value):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"index.{ref_field}.missing",
                    message=f"{ref_field} does not exist: {ref_value}",
                    remediation="Fix the path or seed the referenced artifact.",
                    ref=ref_value,
                )
            )


def validate_rows(
    repo_root: Path,
    index: dict[str, Any],
    storage_roots: list[str],
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    trigger_ids: set[str],
    claim_families: set[str],
    findings: list[Finding],
) -> tuple[list[dict[str, Any]], set[str]]:
    rows = ensure_list(index.get("rows"), "index.rows")
    seen_families: set[str] = set()
    seen_row_ids: set[str] = set()
    derived: list[dict[str, Any]] = []

    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"index.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"index.rows[{idx}].row_id")
        if row_id in seen_row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.duplicate_row_id",
                    message=f"duplicate row_id: {row_id}",
                    remediation="Row ids must be stable and unique.",
                    ref=row_id,
                )
            )
        seen_row_ids.add(row_id)

        claim_family = ensure_str(
            row.get("claim_family"), f"index.rows[{idx}].claim_family"
        )
        if claim_family in seen_families:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.duplicate_claim_family",
                    message=(
                        f"claim_family {claim_family} is bound to more than "
                        "one public-proof row"
                    ),
                    remediation=(
                        "Bind each claim family to exactly one public-proof "
                        "row so downstream surfaces resolve to one packet."
                    ),
                    ref=row_id,
                )
            )
        seen_families.add(claim_family)

        if claim_family not in claim_families:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.unknown_claim_family",
                    message=(
                        f"row {row_id} cites unknown claim_family "
                        f"{claim_family}"
                    ),
                    remediation=(
                        "Use a claim_family that appears in the M3 claim "
                        "manifest, or refresh the manifest first."
                    ),
                    ref=row_id,
                )
            )

        ensure_str(row.get("title"), f"index.rows[{idx}].title")
        ensure_str(row.get("owner_dri"), f"index.rows[{idx}].owner_dri")
        ensure_str(
            row.get("visibility_class"), f"index.rows[{idx}].visibility_class"
        )

        canonical_packet_ref = ensure_str(
            row.get("canonical_packet_ref"),
            f"index.rows[{idx}].canonical_packet_ref",
        )
        if not ref_exists(repo_root, canonical_packet_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.canonical_packet.missing",
                    message=(
                        f"row {row_id} canonical_packet_ref does not exist: "
                        f"{canonical_packet_ref}"
                    ),
                    remediation=(
                        "Seed the packet or fix the path so reviewers can "
                        "resolve the row to one canonical artifact."
                    ),
                    ref=canonical_packet_ref,
                )
            )

        signoff_packet_ref = ensure_str(
            row.get("signoff_packet_ref"),
            f"index.rows[{idx}].signoff_packet_ref",
        )
        if not ref_exists(repo_root, signoff_packet_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.signoff_packet.missing",
                    message=(
                        f"row {row_id} signoff_packet_ref does not exist: "
                        f"{signoff_packet_ref}"
                    ),
                    remediation=(
                        "Point at the review-packet template or a filled "
                        "instance so signoff has a standard form."
                    ),
                    ref=signoff_packet_ref,
                )
            )

        proof_class_id = ensure_str(
            row.get("proof_class_id"), f"index.rows[{idx}].proof_class_id"
        )
        if proof_class_id not in proof_class_ceilings:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.unknown_proof_class",
                    message=(
                        f"row {row_id} cites unknown proof_class_id "
                        f"{proof_class_id}"
                    ),
                    remediation=(
                        "Use a proof_class_id from "
                        "artifacts/governance/evidence_freshness_slos.yaml."
                    ),
                    ref=row_id,
                )
            )
            class_ceiling_days: int | None = None
        else:
            class_ceiling_days = proof_class_ceilings[proof_class_id]

        freshness = ensure_dict(
            row.get("freshness"), f"index.rows[{idx}].freshness"
        )
        stale_after = ensure_str(
            freshness.get("stale_after"),
            f"index.rows[{idx}].freshness.stale_after",
        )
        stale_after_d = stale_after_days(
            stale_after, f"index.rows[{idx}].freshness.stale_after"
        )
        if class_ceiling_days is not None and stale_after_d > class_ceiling_days:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.freshness.exceeds_ceiling",
                    message=(
                        f"row {row_id} stale_after {stale_after} exceeds "
                        f"proof_class_id {proof_class_id} ceiling "
                        f"P{class_ceiling_days}D"
                    ),
                    remediation=(
                        "Use a stale_after no wider than the proof class "
                        "ceiling, or pick a different proof class."
                    ),
                    ref=row_id,
                )
            )

        ensure_str(
            freshness.get("freshness_class"),
            f"index.rows[{idx}].freshness.freshness_class",
        )
        propagation_profile = ensure_str(
            freshness.get("stale_propagation_profile"),
            f"index.rows[{idx}].freshness.stale_propagation_profile",
        )
        if propagation_profile not in propagation_profiles:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.unknown_stale_propagation_profile",
                    message=(
                        f"row {row_id} cites unknown stale_propagation_"
                        f"profile {propagation_profile}"
                    ),
                    remediation=(
                        "Use a profile_id from "
                        "artifacts/governance/evidence_freshness_slos.yaml#/"
                        "stale_propagation_profiles."
                    ),
                    ref=row_id,
                )
            )

        current_outputs = ensure_list(
            row.get("current_outputs"), f"index.rows[{idx}].current_outputs"
        )
        if not current_outputs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.current_outputs.empty",
                    message=f"row {row_id} must name at least one current output",
                    remediation=(
                        "Add a current output so the row resolves to a "
                        "machine-readable artifact."
                    ),
                    ref=row_id,
                )
            )
        for output_idx, output_ref in enumerate(current_outputs):
            output_ref = ensure_str(
                output_ref,
                f"index.rows[{idx}].current_outputs[{output_idx}]",
            )
            if not ref_exists(repo_root, output_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.current_outputs.missing",
                        message=(
                            f"row {row_id} current output does not exist: "
                            f"{output_ref}"
                        ),
                        remediation=(
                            "Seed the artifact (or remove the ref) so the "
                            "row points at current evidence."
                        ),
                        ref=output_ref,
                    )
                )
            elif not is_under_any_root(output_ref, storage_roots):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.current_outputs.outside_roots",
                        message=(
                            f"row {row_id} current output is outside declared "
                            f"storage roots: {output_ref}"
                        ),
                        remediation=(
                            "Store evidence under a governed root or declare "
                            "a new storage root."
                        ),
                        ref=output_ref,
                    )
                )

        supporting = row.get("supporting_evidence_refs", [])
        supporting = ensure_list(
            supporting, f"index.rows[{idx}].supporting_evidence_refs"
        )
        for ref_idx, supporting_ref in enumerate(supporting):
            supporting_ref = ensure_str(
                supporting_ref,
                f"index.rows[{idx}].supporting_evidence_refs[{ref_idx}]",
            )
            if not ref_exists(repo_root, supporting_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.supporting_evidence.missing",
                        message=(
                            f"row {row_id} supporting evidence ref does not "
                            f"exist: {supporting_ref}"
                        ),
                        remediation=(
                            "Seed the artifact or remove the ref so the row "
                            "stays inspectable."
                        ),
                        ref=supporting_ref,
                    )
                )

        exact_build = ensure_str(
            row.get("exact_build_identity_ref"),
            f"index.rows[{idx}].exact_build_identity_ref",
        )
        if not ref_exists(repo_root, exact_build):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.exact_build_identity.missing",
                    message=(
                        f"row {row_id} exact_build_identity_ref does not "
                        f"exist: {exact_build}"
                    ),
                    remediation=(
                        "Point at the checked-in build identity artifact so "
                        "every public-proof row joins one build identity."
                    ),
                    ref=exact_build,
                )
            )

        rerun = ensure_list(
            row.get("rerun_trigger_refs"),
            f"index.rows[{idx}].rerun_trigger_refs",
        )
        if not rerun:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.rerun_triggers.empty",
                    message=(
                        f"row {row_id} must name at least one rerun trigger"
                    ),
                    remediation=(
                        "Cite at least one trigger id from "
                        "artifacts/governance/evidence_rerun_triggers.yaml."
                    ),
                    ref=row_id,
                )
            )
        for trigger_ref in rerun:
            trigger_ref = ensure_str(
                trigger_ref,
                f"index.rows[{idx}].rerun_trigger_refs[]",
            )
            if trigger_ref not in trigger_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.rerun_triggers.unknown",
                        message=(
                            f"row {row_id} cites unknown rerun trigger "
                            f"{trigger_ref}"
                        ),
                        remediation=(
                            "Use a trigger_id from "
                            "artifacts/governance/evidence_rerun_triggers."
                            "yaml."
                        ),
                        ref=trigger_ref,
                    )
                )

        latest_capture = row.get("latest_capture")
        if latest_capture is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.rows.latest_capture.missing",
                    message=(
                        f"row {row_id} must register a latest_capture block"
                    ),
                    remediation=(
                        "Register the latest capture command and report ref, "
                        "or mark the row planned-only with an explicit note."
                    ),
                    ref=row_id,
                )
            )
        else:
            capture_block = ensure_dict(
                latest_capture, f"index.rows[{idx}].latest_capture"
            )
            captured_at = ensure_str(
                capture_block.get("captured_at"),
                f"index.rows[{idx}].latest_capture.captured_at",
            )
            try:
                dt.datetime.fromisoformat(
                    captured_at.replace("Z", "+00:00")
                )
            except ValueError:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.latest_capture.captured_at.invalid",
                        message=(
                            f"row {row_id} latest_capture.captured_at must be "
                            f"ISO-8601, got {captured_at!r}"
                        ),
                        remediation=(
                            "Use an ISO-8601 timestamp (UTC Z preferred)."
                        ),
                        ref=row_id,
                    )
                )
            ensure_str(
                capture_block.get("command"),
                f"index.rows[{idx}].latest_capture.command",
            )
            report_ref = ensure_str(
                capture_block.get("report_ref"),
                f"index.rows[{idx}].latest_capture.report_ref",
            )
            if not ref_exists(repo_root, report_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.latest_capture.report_ref.missing",
                        message=(
                            f"row {row_id} latest_capture.report_ref does not "
                            f"exist: {report_ref}"
                        ),
                        remediation=(
                            "Re-run the lane and check in the capture under a "
                            "governed storage root."
                        ),
                        ref=report_ref,
                    )
                )
            elif not is_under_any_root(report_ref, storage_roots):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.rows.latest_capture.report_ref.outside_roots",
                        message=(
                            f"row {row_id} latest_capture.report_ref is "
                            f"outside declared storage roots: {report_ref}"
                        ),
                        remediation=(
                            "Store the capture under a governed storage root."
                        ),
                        ref=report_ref,
                    )
                )

        derived.append(
            {
                "row_id": row_id,
                "claim_family": claim_family,
                "owner_dri": ensure_str(
                    row.get("owner_dri"), f"index.rows[{idx}].owner_dri"
                ),
                "proof_class_id": proof_class_id,
                "stale_after": stale_after,
                "stale_after_days": stale_after_d,
                "stale_propagation_profile": propagation_profile,
                "canonical_packet_ref": canonical_packet_ref,
                "current_outputs": list(current_outputs),
                "rerun_trigger_refs": list(rerun),
            }
        )

    required_families = set(
        ensure_str(v, "index.required_claim_families[]")
        for v in ensure_list(
            index.get("required_claim_families"), "index.required_claim_families"
        )
    )
    missing_required = required_families - seen_families
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="index.required_claim_families.missing",
                message=(
                    "required_claim_families is missing rows for: "
                    f"{sorted(missing_required)}"
                ),
                remediation=(
                    "Add a public-proof row for each required claim family "
                    "or update the required list in the same change set."
                ),
            )
        )

    extra_required = required_families - claim_families
    if extra_required:
        findings.append(
            Finding(
                severity="error",
                check_id="index.required_claim_families.unknown",
                message=(
                    "required_claim_families includes families not in the M3 "
                    f"claim manifest: {sorted(extra_required)}"
                ),
                remediation=(
                    "Drop the entry, fix the spelling, or refresh the claim "
                    "manifest first."
                ),
            )
        )

    manifest_uncovered = claim_families - seen_families
    if manifest_uncovered:
        findings.append(
            Finding(
                severity="error",
                check_id="index.rows.manifest_uncovered",
                message=(
                    "claim families present in the M3 claim manifest are not "
                    f"covered by any public-proof row: {sorted(manifest_uncovered)}"
                ),
                remediation=(
                    "Add a public-proof row for each missing family in the "
                    "same change set that introduced it to the manifest."
                ),
            )
        )

    return derived, seen_families


def validate_template(
    repo_root: Path,
    template_rel: str,
    findings: list[Finding],
) -> None:
    template_path = repo_root / template_rel
    if not template_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="template.missing",
                message=f"review packet template does not exist: {template_rel}",
                remediation="Seed the review packet template.",
                ref=template_rel,
            )
        )
        return
    body = template_path.read_text(encoding="utf-8")
    for phrase in REQUIRED_TEMPLATE_PHRASES:
        if phrase not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="template.missing_phrase",
                    message=(
                        f"review packet template is missing required phrase: "
                        f"{phrase!r}"
                    ),
                    remediation=(
                        "Update the template so it carries the standard "
                        "header, claim-row table, waiver, signoff, and "
                        "freshness sections."
                    ),
                    ref=template_rel,
                )
            )


def validate_policy(
    repo_root: Path,
    policy_rel: str,
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    trigger_ids: set[str],
    derived_rows: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    policy_path = repo_root / policy_rel
    if not policy_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="policy.missing",
                message=(
                    f"publication shelf-life policy does not exist: {policy_rel}"
                ),
                remediation="Seed the policy document.",
                ref=policy_rel,
            )
        )
        return
    body = policy_path.read_text(encoding="utf-8")
    for phrase in REQUIRED_POLICY_PHRASES:
        if phrase not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="policy.missing_phrase",
                    message=(
                        f"publication shelf-life policy is missing required "
                        f"phrase: {phrase!r}"
                    ),
                    remediation=(
                        "Update the policy so it carries scope, per-family "
                        "windows, rerun-trigger sets, downgrade behavior, and "
                        "the failure drill."
                    ),
                    ref=policy_rel,
                )
            )

    for row in derived_rows:
        family = row["claim_family"]
        if row["proof_class_id"] not in proof_class_ceilings:
            continue
        if family not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="policy.missing_family",
                    message=(
                        f"publication shelf-life policy does not name claim "
                        f"family {family!r}"
                    ),
                    remediation=(
                        "Add a row for the family or refresh the policy in "
                        "the same change set."
                    ),
                    ref=family,
                )
            )
        for trigger_id in row["rerun_trigger_refs"]:
            if trigger_id not in body:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="policy.missing_trigger",
                        message=(
                            "publication shelf-life policy does not cite "
                            f"required rerun trigger {trigger_id!r} for family "
                            f"{family!r}"
                        ),
                        remediation=(
                            "Add the trigger to the policy table for the "
                            "matching family."
                        ),
                        ref=trigger_id,
                    )
                )
        if row["stale_propagation_profile"] not in propagation_profiles:
            continue
        if row["stale_propagation_profile"] not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="policy.missing_stale_propagation_profile",
                    message=(
                        "publication shelf-life policy does not cite stale "
                        f"propagation profile "
                        f"{row['stale_propagation_profile']!r} for family "
                        f"{family!r}"
                    ),
                    remediation=(
                        "Cite the profile id in the policy table so downstream "
                        "surfaces know which downgrade behavior applies."
                    ),
                    ref=row["stale_propagation_profile"],
                )
            )


def write_capture(
    repo_root: Path,
    capture_rel: str,
    index_rel: str,
    derived_rows: list[dict[str, Any]],
    findings: list[Finding],
    generated_at: str,
    check_only: bool,
) -> bool:
    capture_path = repo_root / capture_rel
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m3_public_proof_index",
        "generated_at": generated_at,
        "index_ref": index_rel,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
        "downgrade_matrix": derived_rows,
    }
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text: str | None = None
    if capture_path.exists():
        old_text = capture_path.read_text(encoding="utf-8")
    changed = old_text is None or _normalize(old_text) != _normalize(new_text)
    if not check_only:
        capture_path.write_text(new_text, encoding="utf-8")
    return changed


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    index_path = repo_root / args.index
    if not index_path.exists():
        raise SystemExit(f"missing public-proof index: {args.index}")
    canonical_block = extract_canonical_block(
        index_path.read_text(encoding="utf-8")
    )
    index_payload = ensure_dict(
        render_yaml_as_json(canonical_block, args.index),
        f"{args.index}.canonical_block",
    )

    findings: list[Finding] = []
    validate_header(repo_root, index_payload, findings)
    storage_roots = validate_storage_roots(repo_root, index_payload, findings)

    slo_catalog = ensure_dict(
        render_yaml_file_as_json(
            repo_root / ensure_str(
                index_payload.get("slo_catalog_source"),
                "index.slo_catalog_source",
            )
        ),
        "slo_catalog",
    )
    trigger_catalog = ensure_dict(
        render_yaml_file_as_json(
            repo_root / ensure_str(
                index_payload.get("rerun_trigger_catalog_source"),
                "index.rerun_trigger_catalog_source",
            )
        ),
        "trigger_catalog",
    )
    claim_manifest = ensure_dict(
        load_json(
            repo_root / ensure_str(
                index_payload.get("claim_manifest_source"),
                "index.claim_manifest_source",
            )
        ),
        "claim_manifest",
    )

    proof_class_ceilings = index_proof_class_ceilings(slo_catalog)
    propagation_profiles = index_stale_propagation_profiles(slo_catalog)
    trigger_ids = index_trigger_ids(trigger_catalog)
    claim_families = index_claim_families(claim_manifest)

    derived_rows, _seen_families = validate_rows(
        repo_root=repo_root,
        index=index_payload,
        storage_roots=storage_roots,
        proof_class_ceilings=proof_class_ceilings,
        propagation_profiles=propagation_profiles,
        trigger_ids=trigger_ids,
        claim_families=claim_families,
        findings=findings,
    )

    validate_template(repo_root, args.template, findings)
    validate_policy(
        repo_root,
        args.policy,
        proof_class_ceilings,
        propagation_profiles,
        trigger_ids,
        derived_rows,
        findings,
    )

    generated_at = (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    capture_changed = write_capture(
        repo_root=repo_root,
        capture_rel=args.capture,
        index_rel=args.index,
        derived_rows=derived_rows,
        findings=findings,
        generated_at=generated_at,
        check_only=args.check,
    )

    if args.check and capture_changed:
        findings.append(
            Finding(
                severity="error",
                check_id="capture.stale",
                message=(
                    "checked-in public-proof index capture is stale relative to "
                    "the canonical index, template, or policy"
                ),
                remediation=(
                    "Run `python3 ci/check_m3_public_proof_index.py "
                    "--repo-root .` and commit the regenerated capture."
                ),
                details={"capture_changed": capture_changed},
            )
        )
        write_capture(
            repo_root=repo_root,
            capture_rel=args.capture,
            index_rel=args.index,
            derived_rows=derived_rows,
            findings=findings,
            generated_at=generated_at,
            check_only=False,
        )

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m3-public-proof-index] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings)"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m3-public-proof-index] {prefix} {finding.check_id}: "
            f"{finding.message}{ref_suffix}"
        )
        print(
            f"[m3-public-proof-index]   remediation: {finding.remediation}"
        )
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-public-proof-index] interrupted", file=sys.stderr)
        sys.exit(130)
