#!/usr/bin/env python3
"""Validate and render the external alpha migration-parity artifacts."""

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


DEFAULT_SCOREBOARD_REL = "artifacts/migration/m2_parity_scoreboard.yaml"
DEFAULT_TAXONOMY_REL = "artifacts/migration/import_gap_taxonomy.yaml"
DEFAULT_DIAGNOSTICS_DOC_REL = "docs/migration/import_diagnostics_packet.md"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/migration/parity_alpha_cases/manifest.yaml"
DEFAULT_ALPHA_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"
DEFAULT_ALPHA_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_KNOWN_LIMITS_REL = "artifacts/feedback/external_alpha_known_limits.md"

REQUIRED_PARITY_STATES = {
    "native_parity",
    "bridged_parity",
    "lossy_mapping",
    "unsupported_items",
    "manual_follow_up",
}

ALLOWED_OUTCOME_STATES = {
    "imported",
    "mapped",
    "skipped",
    "manual_review",
    "bridge_required",
    "unsupported",
}

NON_NATIVE_STATES = REQUIRED_PARITY_STATES - {"native_parity"}

REQUIRED_RETAINED_FIELDS = {
    "migration_session_ref",
    "outcome_packet_ref",
    "migration_report_ref",
    "support_export_ref",
    "export_packet_ref",
}

REQUIRED_REVISIT_SURFACES = {
    "migration_center_history",
    "support_export",
    "issue_template",
}

REQUIRED_ISSUE_FIELDS = {
    "source_tool",
    "source_version",
    "alpha_wedge_ref",
    "migration_session_ref",
    "outcome_packet_ref",
    "migration_report_ref",
    "taxonomy_gap_refs",
    "parity_scoreboard_row_ref",
    "support_export_ref",
    "known_limit_refs",
}

REQUIRED_ACCEPTANCE_STATES = REQUIRED_PARITY_STATES | {
    "retained_import_diagnostics",
    "known_gap_issue_template_binding",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "alpha_wedge:",
    "archetype_row:",
    "fixture_register:",
    "import_gap:",
    "importer-outcome-packet:",
    "known_limit:",
    "launch_bundle:",
    "migration-report:",
    "migration-session:",
    "migration_gap_row:",
    "migration_parity_row:",
    "migration_scorecard:",
    "scoreboard_row:",
    "support:",
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
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--taxonomy", default=DEFAULT_TAXONOMY_REL)
    parser.add_argument("--diagnostics-doc", default=DEFAULT_DIAGNOSTICS_DOC_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--alpha-matrix", default=DEFAULT_ALPHA_MATRIX_REL)
    parser.add_argument("--alpha-scoreboard", default=DEFAULT_ALPHA_SCOREBOARD_REL)
    parser.add_argument("--known-limits", default=DEFAULT_KNOWN_LIMITS_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-retained-diagnostics",
        action="store_true",
        help="Print the export-safe retained diagnostics projection after validation.",
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
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
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


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def parse_iso_date(value: str, label: str, findings: list[Finding], ref: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_date",
                message=f"{label} must be a YYYY-MM-DD date, got {value!r}",
                remediation="Use an ISO date without time.",
                ref=ref,
            )
        )


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of", findings, label)
    ensure_str(payload.get("owner"), f"{label}.owner")


def collect_alpha_wedge_ids(matrix: dict[str, Any]) -> set[str]:
    ids = {"alpha_wedge:shared"}
    for idx, raw_row in enumerate(ensure_list(matrix.get("wedge_rows"), "alpha_matrix.wedge_rows")):
        row = ensure_dict(raw_row, f"alpha_matrix.wedge_rows[{idx}]")
        ids.add(ensure_str(row.get("wedge_id"), f"alpha_matrix.wedge_rows[{idx}].wedge_id"))
    return ids


def collect_alpha_scoreboard_row_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "alpha_scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"alpha_scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"alpha_scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def known_limit_ids(markdown: str) -> set[str]:
    return set(re.findall(r"`(known_limit:[^`]+)`", markdown))


def list_of_strings(value: Any, label: str) -> list[str]:
    return [ensure_str(item, f"{label}[]") for item in ensure_list(value, label)]


def validate_refs(
    repo_root: Path,
    refs: list[str],
    label: str,
    findings: list[Finding],
) -> None:
    for ref in refs:
        validate_path_ref(repo_root, ref, label, findings)


def validate_scoreboard(
    repo_root: Path,
    scoreboard: dict[str, Any],
    taxonomy_ids: set[str],
    alpha_wedge_ids: set[str],
    alpha_scoreboard_ids: set[str],
    known_limits: set[str],
    findings: list[Finding],
) -> tuple[dict[str, dict[str, Any]], set[str]]:
    validate_header(scoreboard, "scoreboard", findings)
    validate_refs(repo_root, list_of_strings(scoreboard.get("source_contract_refs"), "scoreboard.source_contract_refs"), "scoreboard.source_contract_refs", findings)
    validate_path_ref(repo_root, ensure_str(scoreboard.get("human_entrypoint_ref"), "scoreboard.human_entrypoint_ref"), "scoreboard.human_entrypoint_ref", findings)
    validate_path_ref(repo_root, ensure_str(scoreboard.get("gap_taxonomy_ref"), "scoreboard.gap_taxonomy_ref"), "scoreboard.gap_taxonomy_ref", findings)

    parity_vocab = set(list_of_strings(scoreboard.get("parity_state_vocabulary"), "scoreboard.parity_state_vocabulary"))
    missing_states = REQUIRED_PARITY_STATES - parity_vocab
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.parity_state_vocabulary.missing",
                message="scoreboard parity vocabulary omits required acceptance states",
                remediation="Add native, bridged, lossy, unsupported, and manual-follow-up states.",
                details={"missing": sorted(missing_states)},
            )
        )

    alpha_scoreboard_row_ref = ensure_str(scoreboard.get("alpha_scoreboard_row_ref"), "scoreboard.alpha_scoreboard_row_ref")
    if alpha_scoreboard_row_ref not in alpha_scoreboard_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.alpha_scoreboard_row_ref.unknown",
                message=f"scoreboard cites unknown alpha scoreboard row: {alpha_scoreboard_row_ref}",
                remediation="Point the migration parity packet at an existing alpha exit-gate row.",
                ref=alpha_scoreboard_row_ref,
            )
        )

    rows_by_id: dict[str, dict[str, Any]] = {}
    seen_states: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id")
        if row_id in rows_by_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.duplicate",
                    message=f"duplicate scoreboard row id: {row_id}",
                    remediation="Use one stable row id per migration parity decision.",
                    ref=row_id,
                )
            )
        rows_by_id[row_id] = row

        parity_state = ensure_str(row.get("parity_state"), f"{row_id}.parity_state")
        seen_states.add(parity_state)
        if parity_state not in REQUIRED_PARITY_STATES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.invalid_parity_state",
                    message=f"{row_id} uses unknown parity state {parity_state}",
                    remediation="Use the closed parity state vocabulary.",
                    ref=row_id,
                )
            )

        category_score = ensure_int(row.get("category_score"), f"{row_id}.category_score")
        if category_score < 0 or category_score > 100:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.category_score.range",
                    message=f"{row_id} category_score must be in 0..100",
                    remediation="Clamp the score to the category parity range.",
                    ref=row_id,
                )
            )

        for wedge_ref in list_of_strings(row.get("alpha_wedge_refs"), f"{row_id}.alpha_wedge_refs"):
            if wedge_ref not in alpha_wedge_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.unknown_alpha_wedge",
                        message=f"{row_id} cites unknown alpha wedge: {wedge_ref}",
                        remediation="Use a wedge id from the alpha launch-wedge matrix.",
                        ref=row_id,
                    )
                )

        row_alpha_ref = ensure_str(row.get("alpha_scoreboard_row_ref"), f"{row_id}.alpha_scoreboard_row_ref")
        if row_alpha_ref not in alpha_scoreboard_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.unknown_alpha_scoreboard_row",
                    message=f"{row_id} cites unknown alpha scoreboard row: {row_alpha_ref}",
                    remediation="Use a row id from the alpha exit-gate scoreboard.",
                    ref=row_id,
                )
            )

        for outcome_state in list_of_strings(row.get("importer_outcome_states"), f"{row_id}.importer_outcome_states"):
            if outcome_state not in ALLOWED_OUTCOME_STATES:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.invalid_outcome_state",
                        message=f"{row_id} uses unknown importer outcome state: {outcome_state}",
                        remediation="Use the migration-center importer outcome vocabulary.",
                        ref=row_id,
                    )
                )

        taxonomy_refs = list_of_strings(row.get("taxonomy_gap_refs"), f"{row_id}.taxonomy_gap_refs")
        if parity_state in NON_NATIVE_STATES and not taxonomy_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.non_native_without_gap_refs",
                    message=f"{row_id} is non-native but has no taxonomy gap refs",
                    remediation="Attach non-native rows to import-gap taxonomy rows.",
                    ref=row_id,
                )
            )
        for gap_ref in taxonomy_refs:
            if gap_ref not in taxonomy_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.unknown_taxonomy_gap",
                        message=f"{row_id} cites unknown taxonomy gap: {gap_ref}",
                        remediation="Add the gap to artifacts/migration/import_gap_taxonomy.yaml or correct the row.",
                        ref=row_id,
                    )
                )

        row_known_limits = list_of_strings(row.get("known_limit_refs"), f"{row_id}.known_limit_refs")
        if parity_state in NON_NATIVE_STATES and not row_known_limits:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.non_native_without_known_limits",
                    message=f"{row_id} is non-native but has no known-limit refs",
                    remediation="Attach known-limit ids so docs and issue templates inherit the same caveat.",
                    ref=row_id,
                )
            )
        for known_limit in row_known_limits:
            if known_limit not in known_limits:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.unknown_known_limit",
                        message=f"{row_id} cites unknown known-limit id: {known_limit}",
                        remediation="Add the known-limit id to the external alpha known-limits packet.",
                        ref=row_id,
                    )
                )

        issue_refs = list_of_strings(row.get("issue_template_refs"), f"{row_id}.issue_template_refs")
        if parity_state in NON_NATIVE_STATES and not issue_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.non_native_without_issue_refs",
                    message=f"{row_id} is non-native but has no issue-template refs",
                    remediation="Attach docs/governance or design-partner taxonomy refs for migration issue filing.",
                    ref=row_id,
                )
            )
        validate_refs(repo_root, issue_refs, f"{row_id}.issue_template_refs", findings)

        validate_refs(repo_root, list_of_strings(row.get("evidence_refs"), f"{row_id}.evidence_refs"), f"{row_id}.evidence_refs", findings)
        validate_refs(repo_root, list_of_strings(row.get("protected_fixture_refs"), f"{row_id}.protected_fixture_refs"), f"{row_id}.protected_fixture_refs", findings)
        validate_retained_diagnostics(repo_root, row, row_id, findings)

    missing_row_states = REQUIRED_PARITY_STATES - seen_states
    if missing_row_states:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.rows.missing_required_states",
                message="scoreboard rows do not cover all required parity states",
                remediation="Seed at least one row for each required acceptance state.",
                details={"missing": sorted(missing_row_states)},
            )
        )
    return rows_by_id, seen_states


def validate_retained_diagnostics(repo_root: Path, row: dict[str, Any], row_id: str, findings: list[Finding]) -> None:
    retained = ensure_dict(row.get("retained_import_diagnostics"), f"{row_id}.retained_import_diagnostics")
    for field_name in REQUIRED_RETAINED_FIELDS:
        value = ensure_str(retained.get(field_name), f"{row_id}.retained_import_diagnostics.{field_name}")
        validate_path_ref(repo_root, value, f"{row_id}.retained_import_diagnostics.{field_name}", findings)

    revisit_refs = set(list_of_strings(retained.get("revisit_surface_refs"), f"{row_id}.retained_import_diagnostics.revisit_surface_refs"))
    missing = REQUIRED_REVISIT_SURFACES - revisit_refs
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.rows.retained_diagnostics.missing_revisit_surface",
                message=f"{row_id} retained diagnostics omit required revisit surfaces",
                remediation="Include migration center history, support export, and issue template revisit surfaces.",
                ref=row_id,
                details={"missing": sorted(missing)},
            )
        )


def validate_taxonomy(
    repo_root: Path,
    taxonomy: dict[str, Any],
    alpha_wedge_ids: set[str],
    known_limits: set[str],
    findings: list[Finding],
) -> set[str]:
    validate_header(taxonomy, "taxonomy", findings)
    validate_path_ref(repo_root, ensure_str(taxonomy.get("scoreboard_ref"), "taxonomy.scoreboard_ref"), "taxonomy.scoreboard_ref", findings)
    validate_path_ref(repo_root, ensure_str(taxonomy.get("diagnostics_packet_ref"), "taxonomy.diagnostics_packet_ref"), "taxonomy.diagnostics_packet_ref", findings)

    taxonomy_ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(taxonomy.get("taxonomy_rows"), "taxonomy.taxonomy_rows")):
        row = ensure_dict(raw_row, f"taxonomy.taxonomy_rows[{idx}]")
        gap_id = ensure_str(row.get("gap_id"), f"taxonomy.taxonomy_rows[{idx}].gap_id")
        if gap_id in taxonomy_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.duplicate",
                    message=f"duplicate taxonomy gap id: {gap_id}",
                    remediation="Use one stable id per import gap.",
                    ref=gap_id,
                )
            )
        taxonomy_ids.add(gap_id)
        ensure_str(row.get("title"), f"{gap_id}.title")
        ensure_str(row.get("gap_class"), f"{gap_id}.gap_class")
        ensure_str(row.get("severity"), f"{gap_id}.severity")

        parity_states = set(list_of_strings(row.get("parity_states"), f"{gap_id}.parity_states"))
        invalid_states = parity_states - REQUIRED_PARITY_STATES
        if invalid_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.invalid_parity_state",
                    message=f"{gap_id} cites unknown parity states",
                    remediation="Use the scoreboard parity state vocabulary.",
                    ref=gap_id,
                    details={"invalid": sorted(invalid_states)},
                )
            )
        if not parity_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.parity_states.empty",
                    message=f"{gap_id} must cite at least one parity state",
                    remediation="Bind the gap to the parity states it can produce.",
                    ref=gap_id,
                )
            )

        for wedge_ref in list_of_strings(row.get("alpha_wedge_refs"), f"{gap_id}.alpha_wedge_refs"):
            if wedge_ref not in alpha_wedge_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="taxonomy.rows.unknown_alpha_wedge",
                        message=f"{gap_id} cites unknown alpha wedge: {wedge_ref}",
                        remediation="Use claimed alpha wedge ids from the matrix.",
                        ref=gap_id,
                    )
                )

        row_known_limits = list_of_strings(row.get("known_limit_refs"), f"{gap_id}.known_limit_refs")
        if not row_known_limits:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.known_limit_refs.empty",
                    message=f"{gap_id} must carry at least one known-limit ref",
                    remediation="Attach the known-limit id that narrows public and support copy.",
                    ref=gap_id,
                )
            )
        for known_limit in row_known_limits:
            if known_limit not in known_limits:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="taxonomy.rows.unknown_known_limit",
                        message=f"{gap_id} cites unknown known-limit id: {known_limit}",
                        remediation="Add the known-limit id to the external alpha known-limits packet.",
                        ref=gap_id,
                    )
                )

        for label in ("docs_help_refs", "scoreboard_row_refs"):
            refs = list_of_strings(row.get(label), f"{gap_id}.{label}")
            if not refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"taxonomy.rows.{label}.empty",
                        message=f"{gap_id} must carry {label}",
                        remediation="Attach the downstream refs that consume this gap.",
                        ref=gap_id,
                    )
                )
            validate_refs(repo_root, refs, f"{gap_id}.{label}", findings)

        if not list_of_strings(row.get("support_export_refs"), f"{gap_id}.support_export_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.support_export_refs.empty",
                    message=f"{gap_id} must carry support export refs",
                    remediation="Attach support export refs so support can reconstruct the gap.",
                    ref=gap_id,
                )
            )

        binding = ensure_dict(row.get("issue_template_binding"), f"{gap_id}.issue_template_binding")
        template_refs = list_of_strings(binding.get("template_refs"), f"{gap_id}.issue_template_binding.template_refs")
        if not template_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.issue_template_binding.template_refs.empty",
                    message=f"{gap_id} must cite issue-template refs",
                    remediation="Attach docs/governance and design-partner taxonomy refs.",
                    ref=gap_id,
                )
            )
        validate_refs(repo_root, template_refs, f"{gap_id}.issue_template_binding.template_refs", findings)
        required_fields = set(list_of_strings(binding.get("required_fields"), f"{gap_id}.issue_template_binding.required_fields"))
        missing_fields = REQUIRED_ISSUE_FIELDS - required_fields
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.rows.issue_template_binding.required_fields.missing",
                    message=f"{gap_id} issue binding omits required fields",
                    remediation="Add the fields needed to preserve migration, report, support, and known-limit refs.",
                    ref=gap_id,
                    details={"missing": sorted(missing_fields)},
                )
            )
    return taxonomy_ids


def validate_fixture_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    rows_by_id: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    validate_header(manifest, "fixture_manifest", findings)
    seen_states: set[str] = set()
    for idx, raw_case in enumerate(ensure_list(manifest.get("cases"), "fixture_manifest.cases")):
        case = ensure_dict(raw_case, f"fixture_manifest.cases[{idx}]")
        case_id = ensure_str(case.get("case_id"), f"fixture_manifest.cases[{idx}].case_id")
        state = ensure_str(case.get("exercises_parity_state"), f"fixture_manifest.cases[{idx}].exercises_parity_state")
        seen_states.add(state)
        row_ref = ensure_str(case.get("scoreboard_row_ref"), f"fixture_manifest.cases[{idx}].scoreboard_row_ref")
        if row_ref not in rows_by_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_manifest.cases.unknown_scoreboard_row",
                    message=f"case {case_id} cites unknown scoreboard row: {row_ref}",
                    remediation="Point the case at a seeded parity scoreboard row.",
                    ref=case_id,
                )
            )
        if case.get("expected_retained_diagnostics") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_manifest.cases.expected_retained_diagnostics",
                    message=f"case {case_id} must require retained diagnostics",
                    remediation="Set expected_retained_diagnostics: true.",
                    ref=case_id,
                )
            )
        validate_refs(repo_root, list_of_strings(case.get("source_fixture_refs"), f"{case_id}.source_fixture_refs"), f"{case_id}.source_fixture_refs", findings)

    missing = REQUIRED_ACCEPTANCE_STATES - seen_states
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.cases.missing_acceptance_states",
                message="protected fixture manifest does not cover all acceptance states",
                remediation="Add fixture cases for every parity state plus retained diagnostics and issue-template binding.",
                details={"missing": sorted(missing)},
            )
        )


def validate_docs_packet(text: str, scoreboard_rel: str, taxonomy_rel: str, findings: list[Finding]) -> None:
    required_markers = REQUIRED_PARITY_STATES | {
        scoreboard_rel,
        taxonomy_rel,
        "retained_import_diagnostics",
        "migration_session_ref",
        "outcome_packet_ref",
        "support_export_ref",
        "issue_template",
    }
    missing = sorted(marker for marker in required_markers if marker not in text)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="diagnostics_doc.missing_required_markers",
                message="diagnostics packet does not describe every required state and retained field",
                remediation="Update the diagnostics packet to cover the scoreboard, taxonomy, states, and retained refs.",
                ref=DEFAULT_DIAGNOSTICS_DOC_REL,
                details={"missing": missing},
            )
        )


def write_report(path: Path, scoreboard_rel: str, rows_by_id: dict[str, dict[str, Any]], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "scoreboard_ref": scoreboard_rel,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_rows": sorted(rows_by_id),
            "required_parity_states": sorted(REQUIRED_PARITY_STATES),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def render_retained_diagnostics(rows_by_id: dict[str, dict[str, Any]]) -> None:
    print("[migration-parity-alpha] retained diagnostics projection")
    print("row_id\tparity_state\tmigration_report_ref\tsupport_export_ref")
    for row_id in sorted(rows_by_id):
        row = rows_by_id[row_id]
        retained = ensure_dict(row.get("retained_import_diagnostics"), f"{row_id}.retained_import_diagnostics")
        print(
            "\t".join(
                [
                    row_id,
                    ensure_str(row.get("parity_state"), f"{row_id}.parity_state"),
                    ensure_str(retained.get("migration_report_ref"), f"{row_id}.migration_report_ref"),
                    ensure_str(retained.get("support_export_ref"), f"{row_id}.support_export_ref"),
                ]
            )
        )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    scoreboard_rel = str(args.scoreboard)
    taxonomy_rel = str(args.taxonomy)
    report_rel = str(args.report) if args.report else None

    scoreboard = ensure_dict(render_yaml_as_json(repo_root / scoreboard_rel), "scoreboard")
    taxonomy = ensure_dict(render_yaml_as_json(repo_root / taxonomy_rel), "taxonomy")
    fixture_manifest = ensure_dict(render_yaml_as_json(repo_root / args.fixture_manifest), "fixture_manifest")
    alpha_matrix = ensure_dict(render_yaml_as_json(repo_root / args.alpha_matrix), "alpha_matrix")
    alpha_scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.alpha_scoreboard), "alpha_scoreboard")
    diagnostics_text = (repo_root / args.diagnostics_doc).read_text(encoding="utf-8")
    known_limits_text = (repo_root / args.known_limits).read_text(encoding="utf-8")

    findings: list[Finding] = []
    alpha_wedge_ids = collect_alpha_wedge_ids(alpha_matrix)
    alpha_scoreboard_ids = collect_alpha_scoreboard_row_ids(alpha_scoreboard)
    known_limits = known_limit_ids(known_limits_text)
    taxonomy_ids = validate_taxonomy(repo_root, taxonomy, alpha_wedge_ids, known_limits, findings)
    rows_by_id, _ = validate_scoreboard(
        repo_root=repo_root,
        scoreboard=scoreboard,
        taxonomy_ids=taxonomy_ids,
        alpha_wedge_ids=alpha_wedge_ids,
        alpha_scoreboard_ids=alpha_scoreboard_ids,
        known_limits=known_limits,
        findings=findings,
    )
    validate_fixture_manifest(repo_root, fixture_manifest, rows_by_id, findings)
    validate_docs_packet(diagnostics_text, scoreboard_rel, taxonomy_rel, findings)

    if report_rel:
        write_report(repo_root / report_rel, scoreboard_rel, rows_by_id, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[migration-parity-alpha] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[migration-parity-alpha] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[migration-parity-alpha]   remediation: {finding.remediation}")
    if args.render_retained_diagnostics and not errors:
        render_retained_diagnostics(rows_by_id)
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[migration-parity-alpha] interrupted", file=sys.stderr)
        sys.exit(130)
