#!/usr/bin/env python3
"""Validate and render the beta third-party import and notice lane."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_MANIFEST_REL = "artifacts/release/m3/third_party_import_manifest.json"
DEFAULT_NOTICE_REPORT_REL = "artifacts/release/m3/notice_generation_report.md"
DEFAULT_RED_RISK_REVIEW_REL = "artifacts/security/m3/red_risk_dependency_review.md"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/third_party_imports/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/captures/third_party_imports_validation_capture.json"
)
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/m3/third_party_imports/manifest.yaml"
DEFAULT_DEPENDENCY_REGISTER_REL = "artifacts/governance/dependency_register.yaml"
DEFAULT_IMPORT_REGISTER_REL = "artifacts/governance/third_party_import_register.yaml"
DEFAULT_NOTICE_SEED_REL = "artifacts/governance/release_notice_seed.yaml"
DEFAULT_HEALTH_REGISTER_REL = "artifacts/governance/critical_upstream_health_register.yaml"
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"

EXPECTED_RECORD_KIND = "third_party_import_manifest"
EXPECTED_SUPPORT_RECORD_KIND = "third_party_import_support_projection"
EXPECTED_CAPTURE_RECORD_KIND = "third_party_import_validation_capture"

OWNER_PATTERN = re.compile(r"^@[A-Za-z0-9_-]+$")

OPAQUE_PREFIXES = (
    "artifact_node:",
    "build-id:",
    "check:",
    "claim_row:",
    "compat_row:",
    "dep.",
    "import.",
    "policy:",
    "release_candidate:",
    "schema:",
    "support_projection:",
)

ALLOWED_PROMOTION_EFFECTS = {
    "blocks_stable_promotion_until_scored_review",
    "narrows_beta_claim_until_first_import_review",
    "blocks_binary_distribution_until_import_terms_verified",
    "requires_architecture_review_before_widening",
}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST_REL)
    parser.add_argument("--notice-report", default=DEFAULT_NOTICE_REPORT_REL)
    parser.add_argument("--red-risk-review", default=DEFAULT_RED_RISK_REVIEW_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--dependency-register", default=DEFAULT_DEPENDENCY_REGISTER_REL)
    parser.add_argument("--third-party-import-register", default=DEFAULT_IMPORT_REGISTER_REL)
    parser.add_argument("--notice-seed", default=DEFAULT_NOTICE_SEED_REL)
    parser.add_argument("--health-register", default=DEFAULT_HEALTH_REGISTER_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument("--generated-at")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated reports or projections would change.",
    )
    return parser.parse_args()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def load_yaml(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
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
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def parse_date_from_instant(value: str, label: str) -> dt.date:
    if value.endswith("Z"):
        value = value[:-1] + "+00:00"
    try:
        return dt.datetime.fromisoformat(value).date()
    except ValueError:
        try:
            return dt.date.fromisoformat(value)
        except ValueError as exc:
            raise SystemExit(f"{label} must be an ISO date or instant: {value!r}") from exc


def validate_repo_ref(
    repo_root: Path,
    ref: Any,
    findings: list[Finding],
    check_id: str,
    owner: str,
    generated_refs: set[str] | None = None,
) -> None:
    if not isinstance(ref, str) or not ref:
        findings.append(
            Finding(
                "error",
                check_id,
                "reference must be a non-empty string",
                "Provide a stable repo-relative or opaque ref.",
                owner,
            )
        )
        return
    if generated_refs and strip_fragment(ref) in generated_refs:
        return
    if is_repo_ref(ref) and not (repo_root / strip_fragment(ref)).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                "Add the missing artifact or correct the reference.",
                owner,
            )
        )


def row_key(source_register: str, source_id: str) -> str:
    return f"{source_register}:{source_id}"


def source_version(row: dict[str, Any]) -> str:
    for field in ("version_ref", "upstream_version", "local_path_ref"):
        value = row.get(field)
        if isinstance(value, str) and value:
            return value
    return "unversioned_source"


def index_source_rows(
    dependency_register: dict[str, Any],
    import_register: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for row in ensure_list(dependency_register.get("rows"), "dependency_register.rows"):
        if isinstance(row, dict) and isinstance(row.get("id"), str):
            indexed[row_key("dependency_register", row["id"])] = row
    for row in ensure_list(import_register.get("rows"), "third_party_import_register.rows"):
        if isinstance(row, dict) and isinstance(row.get("id"), str):
            indexed[row_key("third_party_import_register", row["id"])] = row
    return indexed


def index_notice_seed(notice_seed: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for row in ensure_list(notice_seed.get("rows"), "release_notice_seed.rows"):
        if (
            isinstance(row, dict)
            and isinstance(row.get("source_register"), str)
            and isinstance(row.get("source_id"), str)
        ):
            rows[row_key(row["source_register"], row["source_id"])] = row
    return rows


def protected_source_keys(source_rows: dict[str, dict[str, Any]]) -> set[str]:
    return {
        key
        for key, row in source_rows.items()
        if row.get("protected_path") is True
        or row.get("criticality") == "protected_path_release_critical"
    }


def red_risk_rows(health_register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for row in ensure_list(health_register.get("rows"), "health_register.rows"):
        if isinstance(row, dict) and row.get("risk_state") == "red":
            dependency_id = row.get("dependency_id")
            if isinstance(dependency_id, str):
                rows[dependency_id] = row
    return rows


def validate_manifest(
    *,
    repo_root: Path,
    manifest: dict[str, Any],
    manifest_rel: str,
    notice_report_rel: str,
    red_risk_review_rel: str,
    support_projection_rel: str,
    generated_at: str,
    source_rows: dict[str, dict[str, Any]],
    notice_seed_rows: dict[str, dict[str, Any]],
    health_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    generated_refs = {notice_report_rel, red_risk_review_rel, support_projection_rel}
    if manifest.get("schema_version") != 1:
        findings.append(
            Finding(
                "error",
                "manifest.schema_version",
                "schema_version must be 1",
                "Keep the manifest on schema version 1 until a governed migration exists.",
                manifest_rel,
            )
        )
    if manifest.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                f"record_kind must be {EXPECTED_RECORD_KIND}",
                "Use the third-party import manifest record discriminator.",
                manifest_rel,
            )
        )

    for label, ref in ensure_dict(
        manifest.get("source_contract_refs"), "manifest.source_contract_refs"
    ).items():
        validate_repo_ref(
            repo_root,
            ref,
            findings,
            "manifest.source_contract_refs.missing",
            f"source_contract_refs.{label}",
            generated_refs,
        )

    notice_generation = ensure_dict(
        manifest.get("notice_generation"), "manifest.notice_generation"
    )
    if notice_generation.get("source_of_truth_ref") != manifest_rel:
        findings.append(
            Finding(
                "error",
                "manifest.notice_generation.source_of_truth_ref",
                "notice generation must name the checked manifest as source of truth",
                "Set notice_generation.source_of_truth_ref to the manifest path.",
                manifest_rel,
            )
        )
    for field, expected in (
        ("report_ref", notice_report_rel),
        ("support_projection_ref", support_projection_rel),
    ):
        if notice_generation.get(field) != expected:
            findings.append(
                Finding(
                    "error",
                    f"manifest.notice_generation.{field}",
                    f"notice_generation.{field} must be {expected}",
                    "Keep the manifest and generator arguments aligned.",
                    manifest_rel,
                )
            )

    red_risk = ensure_dict(manifest.get("red_risk_review"), "manifest.red_risk_review")
    if red_risk.get("review_packet_ref") != red_risk_review_rel:
        findings.append(
            Finding(
                "error",
                "manifest.red_risk_review.review_packet_ref",
                f"red_risk_review.review_packet_ref must be {red_risk_review_rel}",
                "Keep the manifest and generated review packet aligned.",
                manifest_rel,
            )
        )

    rows = [row for row in ensure_list(manifest.get("rows"), "manifest.rows") if isinstance(row, dict)]
    rows_by_id: dict[str, dict[str, Any]] = {}
    rows_by_source: dict[str, dict[str, Any]] = {}
    for index, row in enumerate(rows):
        row_id = row.get("row_id")
        row_ref = f"{manifest_rel}#{row_id or f'row[{index}]'}"
        if not isinstance(row_id, str) or not row_id:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.row_id",
                    "row_id must be a non-empty string",
                    "Set a stable row_id for the manifest row.",
                    row_ref,
                )
            )
            continue
        if row_id in rows_by_id:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.duplicate_row_id",
                    f"duplicate row_id {row_id}",
                    "Give every manifest row a unique id.",
                    row_ref,
                )
            )
        rows_by_id[row_id] = row

        missing = [
            field
            for field in (
                "source_register",
                "source_id",
                "name",
                "source_class",
                "source",
                "license_class",
                "upstream_version",
                "owner_dri",
                "protected_path",
                "criticality_class",
                "admission_state_class",
                "provenance_status_class",
                "artifact_node_refs",
                "notice",
            )
            if field not in row
        ]
        if missing:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.required_fields",
                    "manifest row is missing required fields",
                    "Populate source, license, version, owner, artifact, and notice fields.",
                    row_ref,
                    {"missing": missing},
                )
            )

        source_register = row.get("source_register")
        source_id = row.get("source_id")
        if not isinstance(source_register, str) or not isinstance(source_id, str):
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_key",
                    "source_register and source_id must be strings",
                    "Set the source key to a companion-register row.",
                    row_ref,
                )
            )
            continue
        key = row_key(source_register, source_id)
        if key in rows_by_source:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.duplicate_source",
                    f"duplicate source binding {key}",
                    "Represent each source-register row once in the manifest.",
                    row_ref,
                )
            )
        rows_by_source[key] = row

        source = source_rows.get(key)
        if source is None:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_unresolved",
                    f"source row does not resolve: {key}",
                    "Correct source_register/source_id or add the companion-register row.",
                    row_ref,
                )
            )
            continue

        expected_pairs = {
            "license_class": source.get("license_class"),
            "upstream_version": source_version(source),
            "owner_dri": source.get("owner_dri"),
            "protected_path": source.get("protected_path"),
            "criticality_class": source.get("criticality"),
            "admission_state_class": source.get("admission_state"),
            "provenance_status_class": source.get("provenance_status"),
            "update_cadence_class": source.get("update_cadence_class"),
        }
        mismatched = {
            field: {"expected": expected, "actual": row.get(field)}
            for field, expected in expected_pairs.items()
            if expected is not None and row.get(field) != expected
        }
        if mismatched:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_projection_mismatch",
                    "manifest row diverges from the companion source register",
                    "Refresh the manifest row from the source register or update both together.",
                    row_ref,
                    mismatched,
                )
            )

        owner = row.get("owner_dri")
        if not isinstance(owner, str) or not OWNER_PATTERN.fullmatch(owner):
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.owner_dri",
                    "owner_dri must be an @handle",
                    "Set owner_dri to the named dependency/import owner.",
                    row_ref,
                )
            )

        source_object = ensure_dict(row.get("source"), f"{row_ref}.source")
        if not source_object.get("source_kind"):
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source.source_kind",
                    "source.source_kind must be populated",
                    "State whether the source is a registry dependency, host runtime, or reserved import.",
                    row_ref,
                )
            )
        for source_ref in source_object.get("source_refs", []) or []:
            validate_repo_ref(
                repo_root,
                source_ref,
                findings,
                "manifest.rows.source.source_refs.missing",
                row_ref,
                generated_refs,
            )

        for artifact_ref in row.get("artifact_node_refs", []) or []:
            if not isinstance(artifact_ref, str) or not artifact_ref.startswith("artifact_node:"):
                findings.append(
                    Finding(
                        "error",
                        "manifest.rows.artifact_node_refs.invalid",
                        "artifact_node_refs must contain artifact_node refs",
                        "Bind the row to the beta artifact graph node(s) it affects.",
                        row_ref,
                    )
                )

        notice = ensure_dict(row.get("notice"), f"{row_ref}.notice")
        seed = notice_seed_rows.get(key)
        if seed is None:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.notice_seed_missing",
                    f"release notice seed has no row for {key}",
                    "Add the release-notice seed row or remove the notice projection.",
                    row_ref,
                )
            )
        else:
            notice_pairs = {
                "template_class": seed.get("template_class"),
                "render_gate_class": seed.get("render_gate_class"),
                "publication_targets": seed.get("publication_targets"),
                "machine_renderable": seed.get("machine_renderable"),
                "attribution_text_source": seed.get("attribution_text_source"),
            }
            notice_mismatches = {
                field: {"expected": expected, "actual": notice.get(field)}
                for field, expected in notice_pairs.items()
                if notice.get(field) != expected
            }
            if notice_mismatches:
                findings.append(
                    Finding(
                        "error",
                        "manifest.rows.notice_projection_mismatch",
                        "notice projection diverges from release_notice_seed",
                        "Refresh the manifest notice binding from the notice seed.",
                        row_ref,
                        notice_mismatches,
                    )
                )
        validate_repo_ref(
            repo_root,
            notice.get("source_ref"),
            findings,
            "manifest.rows.notice.source_ref.missing",
            row_ref,
            generated_refs,
        )

    missing_protected = sorted(protected_source_keys(source_rows) - set(rows_by_source))
    if missing_protected:
        findings.append(
            Finding(
                "error",
                "manifest.coverage.protected_path_source_missing",
                "protected-path third-party sources are missing from the manifest",
                "Add manifest rows for every protected-path dependency or import source.",
                manifest_rel,
                {"missing": missing_protected},
            )
        )

    missing_notice_seed = sorted(set(notice_seed_rows) - set(rows_by_source))
    if missing_notice_seed:
        findings.append(
            Finding(
                "error",
                "manifest.coverage.notice_seed_missing",
                "notice seed rows are missing from the manifest",
                "Keep notice generation on the manifest by adding the missing source rows.",
                manifest_rel,
                {"missing": missing_notice_seed},
            )
        )

    review_rows = [
        row
        for row in ensure_list(
            manifest.get("red_risk_dependency_reviews"),
            "manifest.red_risk_dependency_reviews",
        )
        if isinstance(row, dict)
    ]
    review_by_id = {
        row.get("dependency_id"): row
        for row in review_rows
        if isinstance(row.get("dependency_id"), str)
    }
    for dependency_id, source in health_rows.items():
        review = review_by_id.get(dependency_id)
        if review is None:
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.review_missing",
                    f"red-risk dependency review is missing: {dependency_id}",
                    "Add a review row that carries owner, due date, and promotion decision effect.",
                    manifest_rel,
                )
            )
            continue
        review_ref = f"{manifest_rel}#red_risk_dependency_reviews.{dependency_id}"
        if review.get("risk_state") != source.get("risk_state"):
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.risk_state_mismatch",
                    "red-risk review must mirror the source health register risk_state",
                    "Refresh the red-risk review from the health register.",
                    review_ref,
                )
            )
        if review.get("owner_dri") != source.get("owner_dri"):
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.owner_mismatch",
                    "red-risk review owner must mirror the source health register",
                    "Refresh owner_dri or update both registers in one change.",
                    review_ref,
                )
            )
        effect = review.get("promotion_decision_effect")
        if effect not in ALLOWED_PROMOTION_EFFECTS:
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.promotion_decision_effect",
                    "red-risk review must declare a promotion decision effect",
                    "Use a controlled promotion decision effect so release review can act on it.",
                    review_ref,
                )
            )
        if not review.get("promotion_decision_refs"):
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.promotion_decision_refs",
                    "red-risk review must cite promotion decision refs",
                    "Cite the artifact graph, release center, or shiproom decision packet.",
                    review_ref,
                )
            )
        due = review.get("next_review_due")
        if not isinstance(due, str) or not due:
            findings.append(
                Finding(
                    "error",
                    "manifest.red_risk.next_review_due",
                    "red-risk review must publish next_review_due",
                    "Set next_review_due to a date inside the review window.",
                    review_ref,
                )
            )
        else:
            try:
                due_date = dt.date.fromisoformat(due)
            except ValueError:
                findings.append(
                    Finding(
                        "error",
                        "manifest.red_risk.next_review_due_format",
                        "next_review_due must be a YYYY-MM-DD date",
                        "Use an ISO date so the gate can compare freshness.",
                        review_ref,
                    )
                )
            else:
                if due_date < parse_date_from_instant(generated_at, "generated_at"):
                    findings.append(
                        Finding(
                            "error",
                            "manifest.red_risk.review_stale",
                            "red-risk review is past next_review_due",
                            "Refresh the review before using it for promotion decisions.",
                            review_ref,
                            {"next_review_due": due, "generated_at": generated_at},
                        )
                    )
        for row_id in review.get("manifest_row_refs", []) or []:
            if row_id not in rows_by_id:
                findings.append(
                    Finding(
                        "error",
                        "manifest.red_risk.manifest_row_ref_unknown",
                        f"manifest row ref does not resolve: {row_id}",
                        "Point manifest_row_refs at existing manifest rows.",
                        review_ref,
                    )
                )


def notice_target_counts(rows: list[dict[str, Any]]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        notice = ensure_dict(row.get("notice"), f"{row.get('row_id')}.notice")
        for target in notice.get("publication_targets", []) or []:
            counts[target] = counts.get(target, 0) + 1
    return dict(sorted(counts.items()))


def render_notice_report(manifest: dict[str, Any], manifest_rel: str) -> str:
    rows = ensure_list(manifest.get("rows"), "manifest.rows")
    counts = notice_target_counts([row for row in rows if isinstance(row, dict)])
    lines = [
        "# Third-party Notice Generation Report",
        "",
        f"Generated from `{manifest_rel}`.",
        "",
        "| Field | Value |",
        "|---|---|",
        f"| Manifest id | `{manifest.get('manifest_id')}` |",
        f"| Release candidate | `{manifest.get('release_candidate_ref')}` |",
        f"| As of | `{manifest.get('as_of')}` |",
        f"| Source rows | {len(rows)} |",
        f"| Protected-path rows | {sum(1 for row in rows if isinstance(row, dict) and row.get('protected_path') is True)} |",
        f"| Red-risk reviews | {len(manifest.get('red_risk_dependency_reviews', []))} |",
        "",
        "## Target Coverage",
        "",
        "| Publication target | Row count |",
        "|---|---:|",
    ]
    for target, count in counts.items():
        lines.append(f"| `{target}` | {count} |")
    lines.extend(["", "## Generated Notice Inputs", ""])

    for target in (
        "third_party_notice",
        "spdx_sbom",
        "cyclonedx_sbom",
        "provenance_statement",
        "docs_pack_manifest",
    ):
        target_rows = [
            row
            for row in rows
            if isinstance(row, dict)
            and target in ensure_dict(row.get("notice"), f"{row.get('row_id')}.notice").get(
                "publication_targets", []
            )
        ]
        lines.extend(
            [
                f"### `{target}`",
                "",
                "| Source id | Name | Version | License class | Owner | Gate |",
                "|---|---|---|---|---|---|",
            ]
        )
        for row in target_rows:
            notice = ensure_dict(row.get("notice"), f"{row.get('row_id')}.notice")
            lines.append(
                "| "
                f"`{row.get('source_id')}` | "
                f"{row.get('name')} | "
                f"`{row.get('upstream_version')}` | "
                f"`{row.get('license_class')}` | "
                f"`{row.get('owner_dri')}` | "
                f"`{notice.get('render_gate_class')}` |"
            )
        lines.append("")

    held_rows = [
        row
        for row in rows
        if isinstance(row, dict)
        and row.get("admission_state_class")
        in {"selected_not_admitted", "reserved_not_yet_imported"}
    ]
    lines.extend(
        [
            "## Held Until Admission Or First Import",
            "",
            "| Source id | Admission state | Notice action | Promotion effect |",
            "|---|---|---|---|",
        ]
    )
    for row in held_rows:
        notice = ensure_dict(row.get("notice"), f"{row.get('row_id')}.notice")
        lines.append(
            "| "
            f"`{row.get('source_id')}` | "
            f"`{row.get('admission_state_class')}` | "
            f"`{notice.get('generation_action')}` | "
            f"{row.get('promotion_effect')} |"
        )

    lines.extend(
        [
            "",
            "## Generation Rule",
            "",
            "Notice, SBOM, docs-pack, and provenance rows are rendered from "
            f"`{manifest_rel}`. Hand-maintained notice lists are not admitted "
            "for the beta release family.",
            "",
        ]
    )
    return "\n".join(lines)


def render_red_risk_review(manifest: dict[str, Any], review_rel: str) -> str:
    reviews = ensure_list(
        manifest.get("red_risk_dependency_reviews"),
        "manifest.red_risk_dependency_reviews",
    )
    lines = [
        "# Red-risk Dependency Review",
        "",
        f"Generated from `{manifest.get('manifest_id')}` into `{review_rel}`.",
        "",
        "| Field | Value |",
        "|---|---|",
        f"| Release candidate | `{manifest.get('release_candidate_ref')}` |",
        f"| As of | `{manifest.get('as_of')}` |",
        f"| Review rows | {len(reviews)} |",
        "",
        "## Promotion Impact",
        "",
        "| Dependency | Owner | Health status | Next review | Promotion effect | Manifest rows |",
        "|---|---|---|---|---|---|",
    ]
    for row in reviews:
        if not isinstance(row, dict):
            continue
        manifest_rows = ", ".join(f"`{item}`" for item in row.get("manifest_row_refs", []))
        lines.append(
            "| "
            f"`{row.get('dependency_id')}` | "
            f"`{row.get('owner_dri')}` | "
            f"`{row.get('health_status')}` | "
            f"`{row.get('next_review_due')}` | "
            f"`{row.get('promotion_decision_effect')}` | "
            f"{manifest_rows} |"
        )

    lines.extend(["", "## Review Notes", ""])
    for row in reviews:
        if not isinstance(row, dict):
            continue
        lines.extend(
            [
                f"### `{row.get('dependency_id')}`",
                "",
                f"- Risk state: `{row.get('risk_state')}`",
                f"- Review gaps: {', '.join(f'`{item}`' for item in row.get('review_gap_classes', []))}",
                f"- Fork / replace / escalate trigger: {row.get('fork_replace_escalate_trigger')}",
                f"- Required follow-up: {row.get('required_follow_up')}",
                f"- Decision refs: {', '.join(f'`{item}`' for item in row.get('promotion_decision_refs', []))}",
                "",
            ]
        )
    lines.extend(
        [
            "## Release Rule",
            "",
            "A red-risk protected-path dependency cannot be treated as green "
            "while owner, backup-owner, license, or upstream-health evidence "
            "is missing. Promotion must either refresh the review, narrow the "
            "claim, or carry an explicit mitigation.",
            "",
        ]
    )
    return "\n".join(lines)


def build_support_projection(
    manifest: dict[str, Any],
    *,
    manifest_rel: str,
    notice_report_rel: str,
    red_risk_review_rel: str,
    support_projection_rel: str,
    generated_at: str,
) -> dict[str, Any]:
    rows = [row for row in ensure_list(manifest.get("rows"), "manifest.rows") if isinstance(row, dict)]
    reviews = [
        row
        for row in ensure_list(
            manifest.get("red_risk_dependency_reviews"),
            "manifest.red_risk_dependency_reviews",
        )
        if isinstance(row, dict)
    ]
    return {
        "schema_version": 1,
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "projection_id": "support_projection:beta.third_party_imports",
        "generated_at": generated_at,
        "manifest_ref": manifest_rel,
        "notice_report_ref": notice_report_rel,
        "red_risk_review_ref": red_risk_review_rel,
        "support_projection_ref": support_projection_rel,
        "manifest_id": manifest.get("manifest_id"),
        "release_candidate_ref": manifest.get("release_candidate_ref"),
        "summary": {
            "row_count": len(rows),
            "protected_path_row_count": sum(1 for row in rows if row.get("protected_path") is True),
            "notice_target_counts": notice_target_counts(rows),
            "red_risk_review_count": len(reviews),
            "promotion_blocking_red_risk_count": sum(
                1
                for row in reviews
                if row.get("promotion_decision_effect")
                in {
                    "blocks_stable_promotion_until_scored_review",
                    "blocks_binary_distribution_until_import_terms_verified",
                }
            ),
            "manual_notice_list_allowed": False,
            "raw_license_text_included": False,
        },
        "rows": [
            {
                "row_id": row.get("row_id"),
                "source_register": row.get("source_register"),
                "source_id": row.get("source_id"),
                "name": row.get("name"),
                "source_class": row.get("source_class"),
                "license_class": row.get("license_class"),
                "upstream_version": row.get("upstream_version"),
                "owner_dri": row.get("owner_dri"),
                "protected_path": row.get("protected_path"),
                "criticality_class": row.get("criticality_class"),
                "publication_targets": ensure_dict(
                    row.get("notice"), f"{row.get('row_id')}.notice"
                ).get("publication_targets", []),
                "artifact_node_refs": row.get("artifact_node_refs", []),
                "promotion_effect": row.get("promotion_effect"),
            }
            for row in rows
        ],
        "red_risk_reviews": [
            {
                "dependency_id": row.get("dependency_id"),
                "owner_dri": row.get("owner_dri"),
                "health_status": row.get("health_status"),
                "next_review_due": row.get("next_review_due"),
                "promotion_decision_effect": row.get("promotion_decision_effect"),
                "manifest_row_refs": row.get("manifest_row_refs", []),
            }
            for row in reviews
        ],
    }


def validate_fixtures(
    repo_root: Path,
    fixture_manifest_rel: str,
    manifest: dict[str, Any],
    projection: dict[str, Any],
    *,
    manifest_rel: str,
    support_projection_rel: str,
    findings: list[Finding],
) -> list[dict[str, Any]]:
    fixture_manifest = ensure_dict(load_yaml(repo_root / fixture_manifest_rel), fixture_manifest_rel)
    results: list[dict[str, Any]] = []
    if fixture_manifest.get("manifest_ref") != manifest_rel:
        findings.append(
            Finding(
                "error",
                "fixtures.manifest_ref",
                "fixture manifest must point at the third-party import manifest",
                "Keep fixture manifest_ref aligned with the validator arguments.",
                fixture_manifest_rel,
            )
        )
    if fixture_manifest.get("projection_ref") != support_projection_rel:
        findings.append(
            Finding(
                "error",
                "fixtures.projection_ref",
                "fixture manifest must point at the support projection",
                "Keep fixture projection_ref aligned with the validator arguments.",
                fixture_manifest_rel,
            )
        )

    manifest_source_ids = {
        row.get("source_id")
        for row in ensure_list(manifest.get("rows"), "manifest.rows")
        if isinstance(row, dict)
    }
    projection_source_ids = {
        row.get("source_id")
        for row in ensure_list(projection.get("rows"), "projection.rows")
        if isinstance(row, dict)
    }
    target_counts = ensure_dict(projection.get("summary"), "projection.summary").get(
        "notice_target_counts", {}
    )
    review_ids = {
        row.get("dependency_id")
        for row in ensure_list(projection.get("red_risk_reviews"), "projection.red_risk_reviews")
        if isinstance(row, dict)
    }

    for raw_case in ensure_list(fixture_manifest.get("cases"), "fixtures.cases"):
        case = ensure_dict(raw_case, "fixtures.cases[]")
        case_ref = ensure_str(case.get("case_ref"), "fixtures.cases[].case_ref")
        validate_repo_ref(
            repo_root,
            case_ref,
            findings,
            "fixtures.case_ref.missing",
            str(case.get("case_id")),
            {support_projection_rel},
        )
        if not (repo_root / case_ref).exists():
            continue
        payload = ensure_dict(load_json(repo_root / case_ref), case_ref)
        case_findings: list[str] = []
        if payload.get("manifest_id") != manifest.get("manifest_id"):
            case_findings.append("manifest_id_mismatch")
        if payload.get("support_projection_ref") != support_projection_rel:
            case_findings.append("support_projection_ref_mismatch")
        if payload.get("manual_notice_list_allowed") is not False:
            case_findings.append("manual_notice_list_not_refused")
        for source_id in payload.get("required_source_ids", []) or []:
            if source_id not in manifest_source_ids:
                case_findings.append(f"manifest_missing_source:{source_id}")
            if source_id not in projection_source_ids:
                case_findings.append(f"projection_missing_source:{source_id}")
        for target in payload.get("required_notice_targets", []) or []:
            if target not in target_counts:
                case_findings.append(f"notice_target_missing:{target}")
        for dependency_id in payload.get("required_red_risk_dependency_ids", []) or []:
            if dependency_id not in review_ids:
                case_findings.append(f"red_risk_review_missing:{dependency_id}")

        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "case_id": payload.get("case_id", case.get("case_id")),
                "case_ref": case_ref,
                "status": status,
                "findings": case_findings,
            }
        )
        for item in case_findings:
            findings.append(
                Finding(
                    "error",
                    "fixtures.case.failed",
                    f"fixture case failed: {item}",
                    "Update the manifest, support projection, or fixture expectation.",
                    case_ref,
                )
            )
    return results


def build_capture(
    *,
    manifest: dict[str, Any],
    manifest_rel: str,
    notice_report_rel: str,
    red_risk_review_rel: str,
    support_projection_rel: str,
    generated_at: str,
    fixture_results: list[dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    rows = [row for row in ensure_list(manifest.get("rows"), "manifest.rows") if isinstance(row, dict)]
    return {
        "schema_version": 1,
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "generated_at": generated_at,
        "status": "passed" if error_count == 0 else "failed",
        "manifest_ref": manifest_rel,
        "notice_report_ref": notice_report_rel,
        "red_risk_review_ref": red_risk_review_rel,
        "support_projection_ref": support_projection_rel,
        "manifest_id": manifest.get("manifest_id"),
        "release_candidate_ref": manifest.get("release_candidate_ref"),
        "summary": {
            "row_count": len(rows),
            "protected_path_row_count": sum(1 for row in rows if row.get("protected_path") is True),
            "red_risk_review_count": len(manifest.get("red_risk_dependency_reviews", [])),
            "fixture_case_count": len(fixture_results),
            "finding_count": len(findings),
            "error_count": error_count,
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    manifest_rel = args.manifest
    notice_report_rel = args.notice_report
    red_risk_review_rel = args.red_risk_review
    support_projection_rel = args.support_projection
    capture_rel = args.capture

    manifest = ensure_dict(load_json(repo_root / manifest_rel), manifest_rel)
    dependency_register = ensure_dict(
        load_yaml(repo_root / args.dependency_register), args.dependency_register
    )
    import_register = ensure_dict(
        load_yaml(repo_root / args.third_party_import_register),
        args.third_party_import_register,
    )
    notice_seed = ensure_dict(load_yaml(repo_root / args.notice_seed), args.notice_seed)
    health_register = ensure_dict(
        load_yaml(repo_root / args.health_register), args.health_register
    )

    generated_at = args.generated_at or ensure_str(manifest.get("as_of"), "manifest.as_of")
    source_rows = index_source_rows(dependency_register, import_register)
    notice_seed_rows = index_notice_seed(notice_seed)
    health_rows = red_risk_rows(health_register)
    findings: list[Finding] = []

    validate_manifest(
        repo_root=repo_root,
        manifest=manifest,
        manifest_rel=manifest_rel,
        notice_report_rel=notice_report_rel,
        red_risk_review_rel=red_risk_review_rel,
        support_projection_rel=support_projection_rel,
        generated_at=generated_at,
        source_rows=source_rows,
        notice_seed_rows=notice_seed_rows,
        health_rows=health_rows,
        findings=findings,
    )

    notice_report = render_notice_report(manifest, manifest_rel)
    red_risk_review = render_red_risk_review(manifest, red_risk_review_rel)
    support_projection = build_support_projection(
        manifest,
        manifest_rel=manifest_rel,
        notice_report_rel=notice_report_rel,
        red_risk_review_rel=red_risk_review_rel,
        support_projection_rel=support_projection_rel,
        generated_at=generated_at,
    )
    fixture_results = validate_fixtures(
        repo_root,
        args.fixture_manifest,
        manifest,
        support_projection,
        manifest_rel=manifest_rel,
        support_projection_rel=support_projection_rel,
        findings=findings,
    )
    capture = build_capture(
        manifest=manifest,
        manifest_rel=manifest_rel,
        notice_report_rel=notice_report_rel,
        red_risk_review_rel=red_risk_review_rel,
        support_projection_rel=support_projection_rel,
        generated_at=generated_at,
        fixture_results=fixture_results,
        findings=findings,
    )

    notice_ok = write_or_check(repo_root / notice_report_rel, notice_report, args.check)
    review_ok = write_or_check(repo_root / red_risk_review_rel, red_risk_review, args.check)
    projection_ok = write_or_check(
        repo_root / support_projection_rel, render_json(support_projection), args.check
    )
    capture_ok = write_or_check(repo_root / capture_rel, render_json(capture), args.check)

    error_count = sum(1 for finding in findings if finding.severity == "error")
    if args.check and not (notice_ok and review_ok and projection_ok and capture_ok):
        return 1
    if error_count:
        print(render_json(capture), file=sys.stderr)
        return 1

    print(
        "third-party import manifest validated "
        f"({len(manifest.get('rows', []))} rows, "
        f"{len(manifest.get('red_risk_dependency_reviews', []))} red-risk reviews)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

