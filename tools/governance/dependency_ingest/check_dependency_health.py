#!/usr/bin/env python3
"""Validate dependency-governance artifacts remain internally consistent.

This is an offline integrity check: it parses the canonical registers and
verifies stable IDs, cross-references, and minimum required fields. It does not
fetch upstream metadata and does not rewrite the canonical registers.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_CONFIG_REL = "ci/check_dependency_health.yml"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def render_yaml_as_json(path: Path) -> Any:
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
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
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
    return value


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--config", default=DEFAULT_CONFIG_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


def item_ref(path_rel: str, row_id: str) -> str:
    return f"{path_rel}#{row_id}"


def load_config(repo_root: Path, config_rel: str) -> dict[str, Any]:
    config_path = repo_root / config_rel
    config = ensure_dict(render_yaml_as_json(config_path), "config")
    schema_version = config.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        raise SystemExit("config.schema_version must be a positive integer")
    ensure_str(config.get("config_id"), "config.config_id")
    return config


def validate_unique_ids(
    findings: list[Finding],
    rows: list[dict[str, Any]],
    id_field: str,
    artifact_rel: str,
) -> set[str]:
    seen: set[str] = set()
    for idx, row in enumerate(rows):
        row_id = row.get(id_field)
        if not isinstance(row_id, str) or not row_id.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.id_missing",
                    message=f"row is missing required id field {id_field!r}",
                    remediation=f"Set {id_field} to a stable, non-empty identifier string.",
                    row_ref=f"{artifact_rel}#row[{idx}]",
                )
            )
            continue
        if row_id in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row.id_duplicate",
                    message=f"duplicate row id {row_id!r} in {artifact_rel}",
                    remediation="Ensure row ids are stable and unique within the register.",
                    row_ref=item_ref(artifact_rel, row_id),
                )
            )
            continue
        seen.add(row_id)
    return seen


def require_fields(
    findings: list[Finding],
    row: dict[str, Any],
    required: dict[str, str],
    artifact_rel: str,
    row_id: str,
) -> None:
    for field_name, kind in required.items():
        value = row.get(field_name)
        if value is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"row.required_field_missing.{field_name}",
                    message=f"row is missing required field {field_name!r}",
                    remediation=f"Populate {field_name} per the dependency review policy.",
                    row_ref=item_ref(artifact_rel, row_id),
                )
            )
            continue
        if kind == "str":
            if not isinstance(value, str) or not value.strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"row.required_field_invalid.{field_name}",
                        message=f"row field {field_name!r} must be a non-empty string",
                        remediation=f"Set {field_name} to a non-empty string.",
                        row_ref=item_ref(artifact_rel, row_id),
                    )
                )
        elif kind == "bool":
            if not isinstance(value, bool):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"row.required_field_invalid.{field_name}",
                        message=f"row field {field_name!r} must be a boolean",
                        remediation=f"Set {field_name} to true/false.",
                        row_ref=item_ref(artifact_rel, row_id),
                    )
                )
        elif kind == "list":
            if not isinstance(value, list):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"row.required_field_invalid.{field_name}",
                        message=f"row field {field_name!r} must be a list",
                        remediation=f"Set {field_name} to a YAML list (possibly empty if allowed).",
                        row_ref=item_ref(artifact_rel, row_id),
                    )
                )
        elif kind == "dict":
            if not isinstance(value, dict):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"row.required_field_invalid.{field_name}",
                        message=f"row field {field_name!r} must be a mapping/object",
                        remediation=f"Set {field_name} to a YAML mapping/object.",
                        row_ref=item_ref(artifact_rel, row_id),
                    )
                )
        else:
            raise SystemExit(f"unknown required-field kind {kind!r}")


def validate_automation_refresh(
    findings: list[Finding],
    row: dict[str, Any],
    artifact_rel: str,
    row_id: str,
) -> None:
    refresh = row.get("automation_refresh")
    if refresh is None:
        findings.append(
            Finding(
                severity="error",
                check_id="row.automation_refresh_missing",
                message="row is missing automation_refresh",
                remediation="Add automation_refresh with mode, evidence_sources, machine_refresh_fields, manual_fields, and stale_threshold_profile.",
                row_ref=item_ref(artifact_rel, row_id),
            )
        )
        return
    if not isinstance(refresh, dict):
        findings.append(
            Finding(
                severity="error",
                check_id="row.automation_refresh_invalid",
                message="row automation_refresh must be an object",
                remediation="Set automation_refresh to a YAML mapping/object.",
                row_ref=item_ref(artifact_rel, row_id),
            )
        )
        return

    required = {
        "mode": "str",
        "stale_threshold_profile": "str",
        "evidence_sources": "list",
        "machine_refresh_fields": "list",
        "manual_fields": "list",
    }
    require_fields(findings, refresh, required, artifact_rel, row_id)


def validate_dependency_register(
    findings: list[Finding],
    dep: dict[str, Any],
    dep_rel: str,
) -> dict[str, dict[str, Any]]:
    schema_version = dep.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id="dependency_register.schema_version_invalid",
                message="dependency register schema_version must be a positive integer",
                remediation="Set schema_version: 1 (or bump with a migration record).",
                row_ref=dep_rel,
            )
        )

    rows = ensure_list(dep.get("rows"), "dependency_register.rows")
    validate_unique_ids(findings, rows, "id", dep_rel)
    rows_by_id: dict[str, dict[str, Any]] = {}

    required_fields = {
        "id": "str",
        "name": "str",
        "dependency_kind": "str",
        "version_track": "str",
        "version_ref": "str",
        "admission_state": "str",
        "owner_dri": "str",
        "owning_packages": "list",
        "owning_lanes": "list",
        "protected_path": "bool",
        "build_vs_buy_reference_required": "bool",
        "build_vs_buy_refs": "list",
        "selection_basis_refs": "list",
        "license_class": "str",
        "provenance_status": "str",
        "health_status": "str",
        "criticality": "str",
        "update_cadence_class": "str",
        "fork_or_replace_trigger": "str",
        "release_notice_class": "str",
        "sbom_inclusion_class": "str",
        "automation_refresh": "dict",
    }

    for row in rows:
        row = ensure_dict(row, "dependency_register.rows[]")
        row_id = row.get("id")
        if not isinstance(row_id, str) or not row_id.strip():
            continue
        rows_by_id[row_id] = row
        require_fields(findings, row, required_fields, dep_rel, row_id)
        validate_automation_refresh(findings, row, dep_rel, row_id)

    return rows_by_id


def validate_import_register(
    findings: list[Finding],
    imp: dict[str, Any],
    imp_rel: str,
) -> dict[str, dict[str, Any]]:
    schema_version = imp.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id="third_party_import_register.schema_version_invalid",
                message="third-party import register schema_version must be a positive integer",
                remediation="Set schema_version: 1 (or bump with a migration record).",
                row_ref=imp_rel,
            )
        )

    rows = ensure_list(imp.get("rows"), "third_party_import_register.rows")
    validate_unique_ids(findings, rows, "id", imp_rel)
    rows_by_id: dict[str, dict[str, Any]] = {}

    required_fields = {
        "id": "str",
        "name": "str",
        "import_kind": "str",
        "admission_state": "str",
        "local_path_ref": "str",
        "owner_dri": "str",
        "owning_packages": "list",
        "owning_lanes": "list",
        "protected_path": "bool",
        "license_class": "str",
        "provenance_status": "str",
        "health_status": "str",
        "criticality": "str",
        "local_modifications": "str",
        "update_cadence_class": "str",
        "fork_or_replace_trigger": "str",
        "release_notice_class": "str",
        "sbom_inclusion_class": "str",
        "automation_refresh": "dict",
    }

    for row in rows:
        row = ensure_dict(row, "third_party_import_register.rows[]")
        row_id = row.get("id")
        if not isinstance(row_id, str) or not row_id.strip():
            continue
        rows_by_id[row_id] = row
        require_fields(findings, row, required_fields, imp_rel, row_id)
        validate_automation_refresh(findings, row, imp_rel, row_id)

        # source_dependency_id may be null, but if present it must be a string.
        src_dep = row.get("source_dependency_id")
        if src_dep is not None and (not isinstance(src_dep, str) or not src_dep.strip()):
            findings.append(
                Finding(
                    severity="error",
                    check_id="import_row.source_dependency_id_invalid",
                    message="import row source_dependency_id must be null or a non-empty string",
                    remediation="Set source_dependency_id to a dependency row id or null.",
                    row_ref=item_ref(imp_rel, row_id),
                )
            )

    return rows_by_id


def validate_release_notice_seed(
    findings: list[Finding],
    seed: dict[str, Any],
    seed_rel: str,
    dep_ids: set[str],
    import_ids: set[str],
    enforce_coverage: bool,
) -> None:
    schema_version = seed.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id="release_notice_seed.schema_version_invalid",
                message="release notice seed schema_version must be a positive integer",
                remediation="Set schema_version: 1 (or bump with a migration record).",
                row_ref=seed_rel,
            )
        )

    rows = ensure_list(seed.get("rows"), "release_notice_seed.rows")
    seen: set[tuple[str, str]] = set()

    required_fields = {
        "source_register": "str",
        "source_id": "str",
        "template_class": "str",
        "render_gate_class": "str",
        "publication_targets": "list",
        "machine_renderable": "bool",
        "attribution_text_source": "str",
    }

    covered_deps: set[str] = set()
    covered_imports: set[str] = set()

    for idx, row in enumerate(rows):
        row = ensure_dict(row, f"release_notice_seed.rows[{idx}]")
        src_reg = row.get("source_register")
        src_id = row.get("source_id")
        if not isinstance(src_reg, str) or not isinstance(src_id, str):
            continue
        key = (src_reg, src_id)
        if key in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notice_seed.duplicate_source_key",
                    message=f"duplicate release notice seed row for ({src_reg}, {src_id})",
                    remediation="Keep a single seed row per stable source id.",
                    row_ref=f"{seed_rel}#row[{idx}]",
                )
            )
        else:
            seen.add(key)

        # Validate required fields with type checks.
        for field_name, kind in required_fields.items():
            value = row.get(field_name)
            if value is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"release_notice_seed.required_field_missing.{field_name}",
                        message=f"release notice seed row is missing {field_name!r}",
                        remediation=f"Populate {field_name} (seed rows are stable contract inputs).",
                        row_ref=f"{seed_rel}#row[{idx}]",
                    )
                )
                continue
            if kind == "str":
                if not isinstance(value, str) or not value.strip():
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"release_notice_seed.required_field_invalid.{field_name}",
                            message=f"release notice seed field {field_name!r} must be a non-empty string",
                            remediation=f"Set {field_name} to a non-empty string.",
                            row_ref=f"{seed_rel}#row[{idx}]",
                        )
                    )
            elif kind == "bool":
                if not isinstance(value, bool):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"release_notice_seed.required_field_invalid.{field_name}",
                            message=f"release notice seed field {field_name!r} must be a boolean",
                            remediation=f"Set {field_name} to true/false.",
                            row_ref=f"{seed_rel}#row[{idx}]",
                        )
                    )
            elif kind == "list":
                if not isinstance(value, list):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=f"release_notice_seed.required_field_invalid.{field_name}",
                            message=f"release notice seed field {field_name!r} must be a list",
                            remediation=f"Set {field_name} to a YAML list.",
                            row_ref=f"{seed_rel}#row[{idx}]",
                        )
                    )
            else:
                raise SystemExit(f"unknown kind {kind!r} for release notice seed field {field_name}")

        if src_reg == "dependency_register":
            if src_id not in dep_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="release_notice_seed.unknown_dependency_source_id",
                        message=f"release notice seed references unknown dependency id: {src_id}",
                        remediation="Fix source_id or add the missing dependency row.",
                        row_ref=f"{seed_rel}#row[{idx}]",
                    )
                )
            else:
                covered_deps.add(src_id)
        elif src_reg == "third_party_import_register":
            if src_id not in import_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="release_notice_seed.unknown_import_source_id",
                        message=f"release notice seed references unknown import id: {src_id}",
                        remediation="Fix source_id or add the missing import row.",
                        row_ref=f"{seed_rel}#row[{idx}]",
                    )
                )
            else:
                covered_imports.add(src_id)
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notice_seed.unknown_source_register",
                    message=f"release notice seed row uses unknown source_register: {src_reg!r}",
                    remediation="Set source_register to dependency_register or third_party_import_register.",
                    row_ref=f"{seed_rel}#row[{idx}]",
                )
            )

    if enforce_coverage:
        missing_deps = sorted(dep_ids - covered_deps)
        missing_imports = sorted(import_ids - covered_imports)
        for dep_id in missing_deps:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notice_seed.missing_dependency_coverage",
                    message=f"dependency row is missing release notice seed coverage: {dep_id}",
                    remediation="Add a release_notice_seed row keyed by this dependency id.",
                    row_ref=item_ref(seed_rel, dep_id),
                )
            )
        for imp_id in missing_imports:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_notice_seed.missing_import_coverage",
                    message=f"import row is missing release notice seed coverage: {imp_id}",
                    remediation="Add a release_notice_seed row keyed by this import id.",
                    row_ref=item_ref(seed_rel, imp_id),
                )
            )


def validate_upstream_scorecard(
    findings: list[Finding],
    scorecard: dict[str, Any],
    score_rel: str,
    dep_ids: set[str],
    dep_rows: dict[str, dict[str, Any]],
    criticality_require_scorecard: set[str],
    enforce_coverage: bool,
) -> None:
    schema_version = scorecard.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id="upstream_health_scorecard.schema_version_invalid",
                message="upstream health scorecard schema_version must be a positive integer",
                remediation="Set schema_version: 1 (or bump with a migration record).",
                row_ref=score_rel,
            )
        )

    rows = ensure_list(scorecard.get("rows"), "upstream_health_scorecard.rows")
    seen = validate_unique_ids(findings, rows, "dependency_id", score_rel)

    required_fields = {
        "dependency_id": "str",
        "dependency_name": "str",
        "criticality": "str",
        "cadence_class": "str",
        "assessment_state": "str",
        "last_reviewed_on": "str",
        "sponsor_dri": "str",
        "evidence_refs": "list",
        "dimension_notes": "dict",
        "escalation_triggers": "list",
        "required_follow_up": "str",
    }

    for row in rows:
        row = ensure_dict(row, "upstream_health_scorecard.rows[]")
        dep_id = row.get("dependency_id")
        if not isinstance(dep_id, str) or not dep_id.strip():
            continue
        require_fields(findings, row, required_fields, score_rel, dep_id)
        if dep_id not in dep_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="upstream_health_scorecard.unknown_dependency_id",
                    message=f"upstream health scorecard references unknown dependency id: {dep_id}",
                    remediation="Fix dependency_id or add the missing dependency row.",
                    row_ref=item_ref(score_rel, dep_id),
                )
            )

    if enforce_coverage:
        for dep_id, dep_row in dep_rows.items():
            if dep_row.get("criticality") in criticality_require_scorecard and dep_id not in seen:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="upstream_health_scorecard.missing_critical_dependency",
                        message=f"critical dependency is missing upstream health scorecard coverage: {dep_id}",
                        remediation="Add an upstream_health_scorecard row for this dependency_id.",
                        row_ref=item_ref(score_rel, dep_id),
                    )
                )


def validate_cross_register_links(
    findings: list[Finding],
    import_rows: dict[str, dict[str, Any]],
    imp_rel: str,
    dep_ids: set[str],
) -> None:
    for row_id, row in import_rows.items():
        src_dep = row.get("source_dependency_id")
        if src_dep is None:
            continue
        if not isinstance(src_dep, str) or not src_dep.strip():
            continue
        if src_dep not in dep_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="third_party_import_register.unknown_source_dependency_id",
                    message=f"import row references unknown source_dependency_id: {src_dep}",
                    remediation="Fix source_dependency_id or add the missing dependency row.",
                    row_ref=item_ref(imp_rel, row_id),
                )
            )


def render_human_summary(findings: list[Finding]) -> str:
    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    lines = []
    lines.append(f"[dependency-health] findings: {len(errors)} error(s), {len(warnings)} warning(s)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN "
        ref = f" ({finding.row_ref})" if finding.row_ref else ""
        lines.append(f"{prefix} {finding.check_id}{ref}: {finding.message}")
        lines.append(f"      remediation: {finding.remediation}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    config_rel = args.config

    config = load_config(repo_root, config_rel)
    dep_rel = ensure_str(config.get("dependency_register_path"), "config.dependency_register_path")
    imp_rel = ensure_str(
        config.get("third_party_import_register_path"), "config.third_party_import_register_path"
    )
    seed_rel = ensure_str(config.get("release_notice_seed_path"), "config.release_notice_seed_path")
    score_rel = ensure_str(config.get("upstream_health_scorecard_path"), "config.upstream_health_scorecard_path")

    enforce_seed = bool(config.get("enforce_release_notice_seed_coverage", True))
    enforce_scorecard = bool(config.get("enforce_upstream_scorecard_coverage", True))
    require_scorecard_classes = config.get("criticality_require_scorecard", [])
    if not isinstance(require_scorecard_classes, list):
        raise SystemExit("config.criticality_require_scorecard must be a list")
    criticality_require_scorecard = {c for c in require_scorecard_classes if isinstance(c, str) and c.strip()}

    findings: list[Finding] = []
    dep = ensure_dict(render_yaml_as_json(repo_root / dep_rel), dep_rel)
    imp = ensure_dict(render_yaml_as_json(repo_root / imp_rel), imp_rel)
    seed = ensure_dict(render_yaml_as_json(repo_root / seed_rel), seed_rel)
    scorecard = ensure_dict(render_yaml_as_json(repo_root / score_rel), score_rel)

    dep_rows = validate_dependency_register(findings, dep, dep_rel)
    import_rows = validate_import_register(findings, imp, imp_rel)
    validate_cross_register_links(findings, import_rows, imp_rel, set(dep_rows.keys()))

    validate_release_notice_seed(
        findings,
        seed,
        seed_rel,
        set(dep_rows.keys()),
        set(import_rows.keys()),
        enforce_seed,
    )
    validate_upstream_scorecard(
        findings,
        scorecard,
        score_rel,
        set(dep_rows.keys()),
        dep_rows,
        criticality_require_scorecard,
        enforce_scorecard,
    )

    sys.stdout.write(render_human_summary(findings))

    report_payload = {
        "schema_version": 1,
        "generated_at_utc": now_utc(),
        "config": {"path": config_rel},
        "findings": [finding.as_report() for finding in findings],
    }

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(report_payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(f.severity == "error" for f in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())

