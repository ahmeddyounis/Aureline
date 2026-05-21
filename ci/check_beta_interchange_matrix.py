#!/usr/bin/env python3
"""Validate the beta standards/interchange matrix.

The canonical cross-milestone register at
``artifacts/governance/standards_matrix.yaml`` already records every industry
standard Aureline reuses, mirrors, extends, or declines, and two markdown pages
project it for reviewers. Until now that posture was consumable only as YAML plus
markdown docs -- there was no typed model and no CI gate, so it could not be
enforced like the other gated surfaces.

This gate promotes the beta interchange posture into a typed, gated machine
matrix:

  - regenerates the frozen machine matrix at
    ``artifacts/governance/standards_interchange_matrix_beta.json`` from the
    canonical register (``--check`` fails if the checked-in matrix is stale);
  - asserts every row carries a support posture, an import expectation, and an
    export expectation drawn from the closed register vocabularies, so a row
    that lacks a posture is a hard failure rather than a silent pass;
  - cross-checks the human-view markdown matrices -- the beta publication view
    and the architecture summary -- against the machine matrix so the docs and
    the machine matrix cannot drift; and
  - runs negative drills proving a missing posture and a drifted docs row are
    both rejected.

The typed Rust consumer
(``aureline_governance::interchange_matrix::current_standards_interchange_matrix``)
reads the same frozen machine matrix, so this gate and
``cargo test -p aureline-governance`` agree without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/governance/standards_matrix.yaml"
DEFAULT_MATRIX_REL = "artifacts/governance/standards_interchange_matrix_beta.json"
DEFAULT_BETA_DOC_REL = "docs/governance/m3/standards_interchange_matrix.md"
DEFAULT_ARCH_DOC_REL = "docs/architecture/standards_interchange_matrix.md"
DEFAULT_REPORT_REL = (
    "artifacts/governance/captures/standards_interchange_matrix_beta_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
MATRIX_RECORD_KIND = "standards_interchange_matrix"
MATRIX_ID = "standards_interchange_matrix:governance.beta"
ROW_ID_PATTERN = re.compile(r"^standard\.[a-z0-9_]+$")

# Closed vocabularies, mirrored from the register. The gate asserts the register
# declares exactly these so the schema, the Rust enums, and this gate cannot
# silently diverge from the register's vocabulary.
SUPPORT_POSTURE_CLASSES = (
    "standard_shaped_import_and_export",
    "standard_shaped_export_only",
    "standard_shaped_import_only",
    "custom_but_mirrorable",
    "custom_with_bridge_planned",
    "standard_deferred_placeholder",
    "standard_declined_with_rationale",
)
IMPORT_EXPECTATION_CLASSES = (
    "required",
    "supported",
    "best_effort",
    "none_planned",
    "deferred_to_later_milestone",
)
EXPORT_EXPECTATION_CLASSES = (
    "required",
    "supported",
    "best_effort",
    "placeholder_stub_only",
    "none_planned",
    "deferred_to_later_milestone",
)
DEVIATION_POLICY_CLASSES = (
    "no_deviation_permitted",
    "narrow_with_adr",
    "extend_with_adr",
    "bridge_with_adr",
    "temporarily_diverge_with_adr",
    "no_standard_currently_adopted",
    "not_yet_committed_pending_standard_maturity",
)

# The seven support postures partition into three tiers. Standard-shaped rows
# are live interoperability claims; bridge rows are custom contracts with a
# mirrorable/planned bridge (not a conformance claim); placeholder rows are
# reserved or declined (not a live claim) and may carry no evidence beyond the
# reserved-seat note, so the evidence-minimum rule is relaxed for them.
STANDARD_SHAPED_POSTURES = frozenset(
    {
        "standard_shaped_import_and_export",
        "standard_shaped_export_only",
        "standard_shaped_import_only",
    }
)
BRIDGE_POSTURES = frozenset({"custom_but_mirrorable", "custom_with_bridge_planned"})
PLACEHOLDER_POSTURES = frozenset(
    {"standard_deferred_placeholder", "standard_declined_with_rationale"}
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--beta-doc", default=DEFAULT_BETA_DOC_REL)
    parser.add_argument("--arch-doc", default=DEFAULT_ARCH_DOC_REL)
    parser.add_argument("--report", default=None, help="Optional JSON validation capture path.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the checked-in machine matrix would change instead of rewriting it.",
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
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {label}: {exc}") from exc


def render_yaml_file_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def normalize_generated_at(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def project_row(raw: dict[str, Any]) -> dict[str, Any]:
    row = ensure_dict(raw, "register.rows[]")
    preferred = ensure_dict(row.get("preferred_standard"), "register.row.preferred_standard")
    owner = ensure_dict(row.get("named_surface_owner"), "register.row.named_surface_owner")
    evidence_paths = [
        ensure_str(item, "register.row.evidence_paths[]")
        for item in ensure_list(row.get("evidence_paths", []), "register.row.evidence_paths")
    ]
    return {
        "row_id": ensure_str(row.get("id"), "register.row.id"),
        "standard_surface": ensure_str(row.get("name"), "register.row.name"),
        "domain": ensure_str(row.get("domain"), "register.row.domain"),
        "support_posture": ensure_str(row.get("support_class"), "register.row.support_class"),
        "import_expectation": ensure_str(
            row.get("import_expectation"), "register.row.import_expectation"
        ),
        "export_expectation": ensure_str(
            row.get("export_expectation"), "register.row.export_expectation"
        ),
        "version_range": ensure_str(
            preferred.get("version_range"), "register.row.preferred_standard.version_range"
        ),
        "deviation_policy": ensure_str(
            row.get("deviation_policy"), "register.row.deviation_policy"
        ),
        "owner_lane": ensure_str(owner.get("lane"), "register.row.named_surface_owner.lane"),
        "owner_dri": ensure_str(owner.get("dri"), "register.row.named_surface_owner.dri"),
        "compatibility_window_class": ensure_str(
            row.get("compatibility_window_class"),
            "register.row.compatibility_window_class",
        ),
        "evidence_paths": evidence_paths,
    }


def build_matrix(register: dict[str, Any], register_rel: str, generated_at: str) -> dict[str, Any]:
    rows = [project_row(raw) for raw in ensure_list(register.get("rows"), "register.rows")]
    rows.sort(key=lambda row: row["row_id"])
    summary = {
        "total_rows": len(rows),
        "standard_shaped_rows": sum(
            1 for row in rows if row["support_posture"] in STANDARD_SHAPED_POSTURES
        ),
        "bridge_rows": sum(1 for row in rows if row["support_posture"] in BRIDGE_POSTURES),
        "deferred_or_declined_rows": sum(
            1 for row in rows if row["support_posture"] in PLACEHOLDER_POSTURES
        ),
        "domains_covered": len({row["domain"] for row in rows}),
    }
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": MATRIX_RECORD_KIND,
        "matrix_id": MATRIX_ID,
        "generated_at": generated_at,
        "source_register_ref": register_rel,
        "human_view_refs": [DEFAULT_BETA_DOC_REL, DEFAULT_ARCH_DOC_REL],
        "support_posture_classes": list(SUPPORT_POSTURE_CLASSES),
        "import_expectation_classes": list(IMPORT_EXPECTATION_CLASSES),
        "export_expectation_classes": list(EXPORT_EXPECTATION_CLASSES),
        "deviation_policy_classes": list(DEVIATION_POLICY_CLASSES),
        "rows": rows,
        "summary": summary,
    }


def check_register_vocabularies(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    expected = {
        "support_class_classes": list(SUPPORT_POSTURE_CLASSES),
        "import_expectation_classes": list(IMPORT_EXPECTATION_CLASSES),
        "export_expectation_classes": list(EXPORT_EXPECTATION_CLASSES),
        "deviation_policy_classes": list(DEVIATION_POLICY_CLASSES),
    }
    for key, value in expected.items():
        if list(register.get(key, [])) != value:
            findings.append(
                Finding(
                    "error",
                    "register.vocabulary_drift",
                    f"register {key} no longer matches the gate vocabulary; update both together",
                    key,
                )
            )
    return findings


def validate_matrix(matrix: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    matrix_id = str(matrix.get("matrix_id", "<matrix>"))

    if matrix.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("error", "matrix.schema_version", "matrix schema_version must be 1", matrix_id)
        )
    if matrix.get("record_kind") != MATRIX_RECORD_KIND:
        findings.append(
            Finding("error", "matrix.record_kind", "matrix record_kind is not supported", matrix_id)
        )
    for key, expected in (
        ("support_posture_classes", list(SUPPORT_POSTURE_CLASSES)),
        ("import_expectation_classes", list(IMPORT_EXPECTATION_CLASSES)),
        ("export_expectation_classes", list(EXPORT_EXPECTATION_CLASSES)),
        ("deviation_policy_classes", list(DEVIATION_POLICY_CLASSES)),
    ):
        if list(matrix.get(key, [])) != expected:
            findings.append(
                Finding("error", "matrix.vocabulary", f"matrix.{key} is not the closed vocabulary", key)
            )

    rows = matrix.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(
            Finding("error", "matrix.rows_empty", "matrix must enumerate at least one row", matrix_id)
        )
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "matrix.rows[]")
        findings.extend(validate_row(row))
        row_id = str(row.get("row_id", "<row>"))
        if row_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "row ids must be unique", row_id))
        seen.add(row_id)

    findings.extend(validate_summary(matrix, rows))
    return findings


def validate_row(row: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    row_id = str(row.get("row_id", "<row>"))

    if not ROW_ID_PATTERN.match(row_id):
        findings.append(
            Finding("error", "row.row_id_pattern", "row_id must match standard.<token>", row_id)
        )

    for field in ("standard_surface", "version_range", "owner_lane", "owner_dri", "domain"):
        value = row.get(field)
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding("error", "row.empty_field", f"row {field} must be a non-empty string", row_id)
            )

    posture = row.get("support_posture")
    if posture not in SUPPORT_POSTURE_CLASSES:
        findings.append(
            Finding(
                "error",
                "row.support_posture_invalid",
                "every claimed row must carry a support posture from the closed vocabulary",
                row_id,
            )
        )
    if row.get("import_expectation") not in IMPORT_EXPECTATION_CLASSES:
        findings.append(
            Finding("error", "row.import_invalid", "row import_expectation is outside the vocabulary", row_id)
        )
    if row.get("export_expectation") not in EXPORT_EXPECTATION_CLASSES:
        findings.append(
            Finding("error", "row.export_invalid", "row export_expectation is outside the vocabulary", row_id)
        )
    if row.get("deviation_policy") not in DEVIATION_POLICY_CLASSES:
        findings.append(
            Finding("error", "row.deviation_invalid", "row deviation_policy is outside the vocabulary", row_id)
        )

    evidence = row.get("evidence_paths")
    if not isinstance(evidence, list):
        findings.append(
            Finding("error", "row.evidence_type", "row evidence_paths must be a list", row_id)
        )
    elif posture not in PLACEHOLDER_POSTURES and not evidence:
        findings.append(
            Finding(
                "error",
                "row.evidence_missing",
                "a non-placeholder row must list at least one evidence path",
                row_id,
            )
        )

    return findings


def validate_summary(matrix: dict[str, Any], rows: list[Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = matrix.get("summary")
    if not isinstance(summary, dict):
        findings.append(
            Finding("error", "summary.missing", "matrix must carry a summary block", str(matrix.get("matrix_id")))
        )
        return findings
    expected = {
        "total_rows": len(rows),
        "standard_shaped_rows": sum(
            1 for r in rows if r.get("support_posture") in STANDARD_SHAPED_POSTURES
        ),
        "bridge_rows": sum(1 for r in rows if r.get("support_posture") in BRIDGE_POSTURES),
        "deferred_or_declined_rows": sum(
            1 for r in rows if r.get("support_posture") in PLACEHOLDER_POSTURES
        ),
        "domains_covered": len({r.get("domain") for r in rows}),
    }
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(
                Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key)
            )
    return findings


def strip_cell(cell: str) -> str:
    return cell.strip().strip("`").strip()


def extract_table(text: str, header_cells: tuple[str, ...]) -> list[list[str]]:
    """Returns the data rows of the first markdown table whose header carries
    every requested header cell, as lists of stripped cells."""
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if not line.lstrip().startswith("|"):
            continue
        header = [strip_cell(cell) for cell in line.strip().strip("|").split("|")]
        if not all(any(want == cell for cell in header) for want in header_cells):
            continue
        rows: list[list[str]] = []
        for follow in lines[index + 1 :]:
            stripped = follow.strip()
            if not stripped.startswith("|"):
                break
            cells = [strip_cell(cell) for cell in stripped.strip("|").split("|")]
            if all(set(cell) <= {"-", ":"} for cell in cells if cell):
                continue  # separator row
            rows.append(cells)
        return rows
    raise SystemExit(f"could not find a markdown table with header cells {header_cells}")


def parse_beta_doc(text: str) -> dict[str, dict[str, str]]:
    rows = extract_table(
        text, ("Row id", "Beta support posture", "Import", "Export")
    )
    parsed: dict[str, dict[str, str]] = {}
    for cells in rows:
        if len(cells) < 5:
            raise SystemExit(f"beta matrix row has too few cells: {cells}")
        parsed[cells[0]] = {
            "support_posture": cells[2],
            "import_expectation": cells[3],
            "export_expectation": cells[4],
        }
    return parsed


def parse_arch_row_ids(text: str) -> set[str]:
    rows = extract_table(text, ("Row id", "Support class", "Import", "Export"))
    return {cells[0] for cells in rows if cells and cells[0]}


def cross_check_docs(
    matrix: dict[str, Any], beta_rows: dict[str, dict[str, str]], arch_ids: set[str]
) -> list[Finding]:
    findings: list[Finding] = []
    matrix_rows = {str(row["row_id"]): row for row in matrix.get("rows", [])}
    matrix_ids = set(matrix_rows)

    for missing in sorted(matrix_ids - set(beta_rows)):
        findings.append(
            Finding("error", "docs.beta_row_missing", "machine-matrix row absent from the beta docs", missing)
        )
    for extra in sorted(set(beta_rows) - matrix_ids):
        findings.append(
            Finding("error", "docs.beta_row_extra", "beta docs row absent from the machine matrix", extra)
        )
    for missing in sorted(matrix_ids - arch_ids):
        findings.append(
            Finding("error", "docs.arch_row_missing", "machine-matrix row absent from the architecture docs", missing)
        )
    for extra in sorted(arch_ids - matrix_ids):
        findings.append(
            Finding("error", "docs.arch_row_extra", "architecture docs row absent from the machine matrix", extra)
        )

    for row_id in sorted(matrix_ids & set(beta_rows)):
        machine = matrix_rows[row_id]
        doc = beta_rows[row_id]
        for field in ("support_posture", "import_expectation", "export_expectation"):
            if str(machine.get(field)) != doc.get(field):
                findings.append(
                    Finding(
                        "error",
                        f"docs.{field}_drift",
                        f"beta docs {field} {doc.get(field)!r} disagrees with machine matrix "
                        f"{machine.get(field)!r}",
                        row_id,
                    )
                )
    return findings


def run_negative_drills(
    matrix: dict[str, Any], beta_rows: dict[str, dict[str, str]], arch_ids: set[str]
) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    # A row that lacks a posture must be rejected.
    mutated = copy.deepcopy(matrix)
    mutated["rows"][0]["support_posture"] = ""
    passed = "row.support_posture_invalid" in {f.check_id for f in validate_matrix(mutated)}
    results.append(
        {
            "drill_id": "missing_posture_rejected",
            "expected_check_id": "row.support_posture_invalid",
            "status": "passed" if passed else "failed",
        }
    )
    if not passed:
        findings.append(
            Finding(
                "error",
                "negative_drill.not_rejected",
                "negative drill missing_posture_rejected did not fire",
                "missing_posture_rejected",
            )
        )

    # A drifted beta-docs posture must be rejected.
    drifted = copy.deepcopy(beta_rows)
    first_id = sorted(drifted)[0]
    drifted[first_id]["support_posture"] = "standard_declined_with_rationale"
    check_ids = {f.check_id for f in cross_check_docs(matrix, drifted, arch_ids)}
    passed = "docs.support_posture_drift" in check_ids
    results.append(
        {
            "drill_id": "docs_drift_rejected",
            "expected_check_id": "docs.support_posture_drift",
            "status": "passed" if passed else "failed",
        }
    )
    if not passed:
        findings.append(
            Finding(
                "error",
                "negative_drill.not_rejected",
                "negative drill docs_drift_rejected did not fire",
                "docs_drift_rejected",
            )
        )

    return results, findings


def write_report(
    path: Path,
    matrix: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    matrix_changed: bool,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "standards_interchange_matrix_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "matrix_id": matrix.get("matrix_id"),
        "matrix_changed": matrix_changed,
        "summary": matrix.get("summary"),
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(
        render_yaml_file_as_json(repo_root / args.register), "register"
    )
    beta_doc_path = repo_root / args.beta_doc
    arch_doc_path = repo_root / args.arch_doc
    if not beta_doc_path.exists():
        raise SystemExit(f"missing beta matrix doc: {beta_doc_path}")
    if not arch_doc_path.exists():
        raise SystemExit(f"missing architecture matrix doc: {arch_doc_path}")

    generated_at = generated_at_now()
    matrix = build_matrix(register, args.register, generated_at)

    matrix_path = repo_root / args.matrix
    new_text = json_text(matrix)
    existing = matrix_path.read_text(encoding="utf-8") if matrix_path.exists() else None
    matrix_changed = existing is None or normalize_generated_at(existing) != normalize_generated_at(
        new_text
    )
    if not args.check:
        matrix_path.parent.mkdir(parents=True, exist_ok=True)
        matrix_path.write_text(new_text, encoding="utf-8")

    beta_rows = parse_beta_doc(beta_doc_path.read_text(encoding="utf-8"))
    arch_ids = parse_arch_row_ids(arch_doc_path.read_text(encoding="utf-8"))

    findings = check_register_vocabularies(register)
    findings.extend(validate_matrix(matrix))
    findings.extend(cross_check_docs(matrix, beta_rows, arch_ids))
    drill_results, drill_findings = run_negative_drills(matrix, beta_rows, arch_ids)
    findings.extend(drill_findings)

    if args.check and matrix_changed:
        findings.append(
            Finding(
                "error",
                "matrix.stale",
                "checked-in machine matrix is stale; regenerate and commit it",
                args.matrix,
            )
        )

    report_rel = args.report
    if args.check and report_rel is None:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, matrix, findings, drill_results, matrix_changed)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    print(
        "beta standards/interchange matrix validated "
        f"({matrix['summary']['total_rows']} rows, "
        f"{matrix['summary']['standard_shaped_rows']} standard-shaped across "
        f"{matrix['summary']['domains_covered']} domains, "
        f"{len(drill_results)} negative drills)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
