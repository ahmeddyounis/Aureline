#!/usr/bin/env python3
"""Unattended M1 internal-boundary manifest validation lane.

Replays every row in
``artifacts/governance/m1_open_local_capability_matrix.yaml`` against the
boundary manifest schema and the canonical truth sources the matrix
joins:

- ``artifacts/governance/deployment_profiles.yaml`` — the frozen
  deployment-profile vocabulary the matrix's ``deployment_profiles``
  arrays MUST resolve against;
- ``artifacts/governance/residual_dependencies.yaml`` — the canonical
  residual-dependency ledger; the matrix's
  ``residual_dependencies[].dependency_class`` values MUST be a subset
  of the ledger's ``dependency_class_vocabulary``, posture values MUST
  resolve against the ledger's ``posture_class_vocabulary``, and the
  matrix MUST NOT relax a stricter posture (forbidden /
  not_applicable_structural) declared on the ledger.

Per-row assertions:

- ``row_id`` is unique and namespaced under ``boundary_row:``;
- ``surface_family``, ``boundary_class``, ``manifest_status``, every
  ``carries_truth_fields`` entry, and every ``failure_drill.drill_id``
  are members of the matrix's closed vocabularies;
- ``deployment_profiles`` is non-empty and every entry resolves
  against the deployment-profile register;
- ``local_only`` rows declare ``residual_dependencies: []`` (the
  ``local_only`` invariant means no residual surface);
- ``local_core_continuity`` is a non-empty string on every row;
- every ``residual_dependencies`` entry declares a ``per_profile_posture``
  covering every frozen deployment profile, with posture values in the
  closed vocabulary;
- a row MUST NOT relax a ledger posture of ``forbidden`` or
  ``not_applicable_structural`` to anything else; the ledger is canonical;
- every row pins exactly one named failure drill drawn from
  ``failure_drill_id_vocabulary`` with a typed ``expected_check_id``,
  actionable owner, and next-action sentence; and
- the matrix covers every surface family in
  ``required_surface_family_coverage``.

Cross-artifact assertions:

- every named external ref (overview page, schemas, registers, ledgers,
  consumer refs) exists on disk;
- the named runtime consumer (``consumer_bindings.named_runtime_consumer``)
  declares a non-empty ``consumed_fields`` list whose entries are members
  of ``truth_field_vocabulary``.

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill <row_id>:<drill_id>``
replays the named drill on the named row and exits 0 only when the
runner reproduces the declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix and
ledger files are parsed via Ruby/Psych so this script does not require
a third-party Python YAML dependency.
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


DEFAULT_MATRIX_REL = "artifacts/governance/m1_open_local_capability_matrix.yaml"
DEFAULT_SCHEMA_REL = "schemas/governance/boundary_manifest.schema.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

ROW_ID_PREFIX = "boundary_row:"


# ---- finding / result types -----------------------------------------------


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


@dataclass
class RowResult:
    row_id: str
    surface_family: str
    boundary_class: str
    manifest_status: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


# ---- shell helpers --------------------------------------------------------


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Boundary-manifest matrix YAML path, repo-relative.",
    )
    parser.add_argument(
        "--schema",
        default=DEFAULT_SCHEMA_REL,
        help="Boundary-manifest schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build identity record to embed in the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<row_id>:<drill_id>'. The runner exits 0 only when the "
            "row's failure drill reproduces the exact expected_check_id."
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
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [Time, Date, DateTime], aliases: false); "
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
    return value.strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


# ---- canonical-source loaders --------------------------------------------


def load_ledger_dependency_classes(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    items = ensure_list(
        payload.get("dependency_class_vocabulary"),
        f"{ref}.dependency_class_vocabulary",
    )
    return {ensure_str(item, f"{ref}.dependency_class_vocabulary[]") for item in items}


def load_ledger_posture_classes(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    items = ensure_list(
        payload.get("posture_class_vocabulary"),
        f"{ref}.posture_class_vocabulary",
    )
    return {ensure_str(item, f"{ref}.posture_class_vocabulary[]") for item in items}


def load_ledger_postures(
    repo_root: Path, ref: str
) -> dict[str, dict[str, str]]:
    """Return {dependency_class: {profile_id: posture}} from the ledger."""
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(payload.get("ledger_rows"), f"{ref}.ledger_rows")
    out: dict[str, dict[str, str]] = {}
    for idx, row in enumerate(rows):
        row = ensure_dict(row, f"{ref}.ledger_rows[{idx}]")
        dep_class = ensure_str(
            row.get("dependency_class"),
            f"{ref}.ledger_rows[{idx}].dependency_class",
        )
        postures = ensure_dict(
            row.get("per_profile_posture"),
            f"{ref}.ledger_rows[{idx}].per_profile_posture",
        )
        out[dep_class] = {
            ensure_str(profile, f"{ref}.ledger_rows[{idx}].per_profile_posture key"):
                ensure_str(
                    postures[profile],
                    f"{ref}.ledger_rows[{idx}].per_profile_posture.{profile}",
                )
            for profile in postures
        }
    return out


def load_profile_register(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    items = ensure_list(
        payload.get("deployment_profile_vocabulary"),
        f"{ref}.deployment_profile_vocabulary",
    )
    return {ensure_str(item, f"{ref}.deployment_profile_vocabulary[]") for item in items}


# ---- per-row replay -------------------------------------------------------


# Ledger postures that this matrix is forbidden from relaxing. If the
# ledger declares one of these for a (dependency_class, profile), the
# matrix MUST honor it verbatim.
LEDGER_LOCKED_POSTURES = {"forbidden", "not_applicable_structural"}


def replay_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    boundary_class_vocab: set[str],
    deployment_profile_vocab: set[str],
    residual_dependency_class_vocab: set[str],
    posture_class_vocab: set[str],
    surface_family_vocab: set[str],
    truth_field_vocab: set[str],
    manifest_status_vocab: set[str],
    failure_drill_id_vocab: set[str],
    ledger_postures: dict[str, dict[str, str]],
    forced_overrides: dict[str, Any] | None = None,
) -> RowResult:
    forced_overrides = forced_overrides or {}
    row_id = ensure_str(row.get("row_id"), "row.row_id")
    surface_family = ensure_str(row.get("surface_family"), f"{row_id}.surface_family")

    boundary_class = ensure_str(
        row.get("boundary_class"), f"{row_id}.boundary_class"
    )
    if "rewrite_boundary_class" in forced_overrides:
        boundary_class = ensure_str(
            forced_overrides["rewrite_boundary_class"],
            f"{row_id}.failure_drill.forced_input.rewrite_boundary_class",
        )

    manifest_status = ensure_str(
        row.get("manifest_status"), f"{row_id}.manifest_status"
    )

    deployment_profiles = list(
        ensure_list(
            row.get("deployment_profiles"),
            f"{row_id}.deployment_profiles",
        )
    )

    local_core_continuity = row.get("local_core_continuity")
    if forced_overrides.get("clear_local_core_continuity"):
        local_core_continuity = ""

    residual_dependencies = list(
        ensure_list(
            row.get("residual_dependencies"),
            f"{row_id}.residual_dependencies",
        )
    )

    # Forced overrides on residual dependencies.
    inject = forced_overrides.get("inject_residual_dependency")
    if isinstance(inject, dict):
        residual_dependencies.append(inject)
    rewrite_class = forced_overrides.get("rewrite_residual_dependency_class")
    if isinstance(rewrite_class, dict):
        old_class = rewrite_class.get("old_class")
        new_class = rewrite_class.get("new_class")
        if isinstance(old_class, str) and isinstance(new_class, str):
            residual_dependencies = [
                ({**dep, "dependency_class": new_class}
                 if isinstance(dep, dict) and dep.get("dependency_class") == old_class
                 else dep)
                for dep in residual_dependencies
            ]
    rewrite_posture = forced_overrides.get("rewrite_residual_dependency_posture")
    if isinstance(rewrite_posture, dict):
        dep_class = rewrite_posture.get("dependency_class")
        profile = rewrite_posture.get("profile")
        new_posture = rewrite_posture.get("new_posture")
        if (
            isinstance(dep_class, str)
            and isinstance(profile, str)
            and isinstance(new_posture, str)
        ):
            new_deps = []
            for dep in residual_dependencies:
                if (
                    isinstance(dep, dict)
                    and dep.get("dependency_class") == dep_class
                ):
                    postures = dict(
                        ensure_dict(
                            dep.get("per_profile_posture"),
                            f"{row_id}.residual_dependencies[].per_profile_posture",
                        )
                    )
                    postures[profile] = new_posture
                    new_deps.append({**dep, "per_profile_posture": postures})
                else:
                    new_deps.append(dep)
            residual_dependencies = new_deps

    carries_truth_fields = list(
        ensure_list(
            row.get("carries_truth_fields"),
            f"{row_id}.carries_truth_fields",
        )
    )
    rewrite_truth = forced_overrides.get("rewrite_truth_field")
    if isinstance(rewrite_truth, dict):
        old_field = rewrite_truth.get("old_field")
        new_field = rewrite_truth.get("new_field")
        if isinstance(old_field, str) and isinstance(new_field, str):
            carries_truth_fields = [
                new_field if f == old_field else f for f in carries_truth_fields
            ]

    result = RowResult(
        row_id=row_id,
        surface_family=surface_family,
        boundary_class=boundary_class,
        manifest_status=manifest_status,
    )

    # row_id pattern
    if not row_id.startswith(ROW_ID_PREFIX):
        fail(
            result,
            "boundary_manifest.row_id_unprefixed",
            f"row_id '{row_id}' must start with '{ROW_ID_PREFIX}'",
        )

    # Closed-vocabulary checks.
    if surface_family not in surface_family_vocab:
        fail(
            result,
            "boundary_manifest.surface_family_unknown",
            (
                f"surface_family '{surface_family}' is not in "
                "surface_family_vocabulary"
            ),
        )
    else:
        pass_(result, f"surface_family '{surface_family}' is in vocabulary")

    if boundary_class not in boundary_class_vocab:
        fail(
            result,
            "boundary_manifest.boundary_class_unknown",
            f"boundary_class '{boundary_class}' is not in boundary_class_vocabulary",
        )
    else:
        pass_(result, f"boundary_class '{boundary_class}' is in vocabulary")

    if manifest_status not in manifest_status_vocab:
        fail(
            result,
            "boundary_manifest.manifest_status_unknown",
            (
                f"manifest_status '{manifest_status}' is not in "
                "manifest_status_vocabulary"
            ),
        )

    if not deployment_profiles:
        fail(
            result,
            "boundary_manifest.deployment_profiles_empty",
            "deployment_profiles must declare at least one profile id",
        )
    for token in deployment_profiles:
        if token not in deployment_profile_vocab:
            fail(
                result,
                "boundary_manifest.deployment_profile_unknown",
                (
                    f"deployment_profiles contains '{token}' which is not in "
                    "deployment_profile_vocabulary (deployment_profiles.yaml)"
                ),
            )

    # local_core_continuity must be a non-empty string.
    if not isinstance(local_core_continuity, str) or not local_core_continuity.strip():
        fail(
            result,
            "boundary_manifest.local_core_continuity_must_be_present",
            "local_core_continuity must be a non-empty string on every row",
        )
    else:
        pass_(result, "local_core_continuity is present")

    # local_only rows MUST have empty residual_dependencies.
    if boundary_class == "local_only" and residual_dependencies:
        fail(
            result,
            "boundary_manifest.local_only_must_have_no_residual_dependencies",
            (
                "local_only rows MUST declare residual_dependencies: []; "
                "introduce a residual dependency by reclassifying to "
                "provider_linked, managed, or mirrored first"
            ),
        )
    elif boundary_class == "local_only":
        pass_(result, "local_only row has no residual_dependencies")

    # carries_truth_fields non-empty and in vocab.
    if not carries_truth_fields:
        fail(
            result,
            "boundary_manifest.carries_truth_fields_empty",
            "carries_truth_fields must declare at least one truth field",
        )
    for token in carries_truth_fields:
        if token not in truth_field_vocab:
            fail(
                result,
                "boundary_manifest.truth_field_unknown",
                (
                    f"carries_truth_fields contains '{token}' which is not "
                    "in truth_field_vocabulary"
                ),
            )

    # Residual-dependency entries.
    for idx, dep in enumerate(residual_dependencies):
        dep = ensure_dict(dep, f"{row_id}.residual_dependencies[{idx}]")
        dep_class = ensure_str(
            dep.get("dependency_class"),
            f"{row_id}.residual_dependencies[{idx}].dependency_class",
        )
        if dep_class not in residual_dependency_class_vocab:
            fail(
                result,
                "boundary_manifest.residual_dependency_class_unknown",
                (
                    f"residual_dependencies[{idx}].dependency_class "
                    f"'{dep_class}' is not in "
                    "residual_dependency_class_vocabulary"
                ),
            )
        postures = ensure_dict(
            dep.get("per_profile_posture"),
            f"{row_id}.residual_dependencies[{idx}].per_profile_posture",
        )
        missing_profiles = sorted(deployment_profile_vocab - set(postures.keys()))
        if missing_profiles:
            fail(
                result,
                "boundary_manifest.per_profile_posture_incomplete",
                (
                    f"residual_dependencies[{idx}].per_profile_posture must "
                    "declare a posture for every deployment profile; "
                    f"missing: {missing_profiles}"
                ),
            )
        for profile, posture in postures.items():
            posture = ensure_str(
                posture,
                f"{row_id}.residual_dependencies[{idx}].per_profile_posture.{profile}",
            )
            if posture not in posture_class_vocab:
                fail(
                    result,
                    "boundary_manifest.posture_class_unknown",
                    (
                        f"residual_dependencies[{idx}].per_profile_posture."
                        f"{profile} '{posture}' is not in "
                        "posture_class_vocabulary"
                    ),
                )
            # Anti-relax check against the ledger.
            ledger_posture = ledger_postures.get(dep_class, {}).get(profile)
            if (
                ledger_posture in LEDGER_LOCKED_POSTURES
                and posture != ledger_posture
            ):
                fail(
                    result,
                    "boundary_manifest.posture_relaxes_ledger_forbidden",
                    (
                        f"residual_dependencies[{idx}] ({dep_class}/"
                        f"{profile}): matrix posture '{posture}' relaxes "
                        f"ledger posture '{ledger_posture}'; the ledger is "
                        "canonical for forbidden / not_applicable_structural"
                    ),
                )
        absence_impact = dep.get("absence_impact")
        if not isinstance(absence_impact, str) or not absence_impact.strip():
            fail(
                result,
                "boundary_manifest.absence_impact_must_be_present",
                (
                    f"residual_dependencies[{idx}].absence_impact must be a "
                    "non-empty string"
                ),
            )

    # Failure drill structure.
    failure_drill = ensure_dict(
        row.get("failure_drill"), f"{row_id}.failure_drill"
    )
    drill_id = ensure_str(
        failure_drill.get("drill_id"), f"{row_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "boundary_manifest.failure_drill_id_unknown",
            (
                f"failure_drill.drill_id '{drill_id}' is not in "
                "failure_drill_id_vocabulary"
            ),
        )
    ensure_str(
        failure_drill.get("expected_check_id"),
        f"{row_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        failure_drill.get("actionable_owner_ref"),
        f"{row_id}.failure_drill.actionable_owner_ref",
    )
    ensure_str(
        failure_drill.get("next_action"),
        f"{row_id}.failure_drill.next_action",
    )
    forced_input = ensure_dict(
        failure_drill.get("forced_input"),
        f"{row_id}.failure_drill.forced_input",
    )
    if not forced_input:
        fail(
            result,
            "boundary_manifest.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )

    result.diagnostics.update(
        {
            "row_id": row_id,
            "surface_family": surface_family,
            "boundary_class": boundary_class,
            "manifest_status": manifest_status,
            "deployment_profiles": deployment_profiles,
            "residual_dependency_classes": [
                d.get("dependency_class") if isinstance(d, dict) else None
                for d in residual_dependencies
            ],
            "carries_truth_fields": carries_truth_fields,
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": failure_drill.get("expected_check_id"),
            },
            "forced_overrides_applied": forced_overrides,
        }
    )

    return result


# ---- main -----------------------------------------------------------------


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=(
                    f"matrix schema_version must be the integer 1, got "
                    f"{schema_version!r}"
                ),
                remediation=(
                    "Bump the runner together with the schema if the matrix "
                    "shape changes."
                ),
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    overview_page = ensure_str(matrix.get("overview_page"), "matrix.overview_page")
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer-facing landing page or fix the path.",
                ref=overview_page,
            )
        )

    schema_rel = ensure_str(
        matrix.get("manifest_schema_ref"), "matrix.manifest_schema_ref"
    )
    if not artifact_ref_exists(repo_root, schema_rel):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.manifest_schema_ref.missing",
                message=f"manifest_schema_ref does not exist: {schema_rel}",
                remediation="Fix the path or land the schema file.",
                ref=schema_rel,
            )
        )

    # Required external refs the matrix anchors.
    for key in (
        "boundary_manifest_strawman_ref",
        "open_paid_boundary_rows_ref",
        "deployment_profile_register_ref",
        "residual_dependency_ledger_ref",
        "capability_inventory_seed_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{key}.missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or land the referenced artifact.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
                message=(
                    f"validation_lane_ref base does not exist: "
                    f"{validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    # Closed vocabularies.
    boundary_class_vocab = {
        ensure_str(item, "matrix.boundary_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("boundary_class_vocabulary"),
            "matrix.boundary_class_vocabulary",
        )
    }
    deployment_profile_vocab = {
        ensure_str(item, "matrix.deployment_profile_vocabulary[]")
        for item in ensure_list(
            matrix.get("deployment_profile_vocabulary"),
            "matrix.deployment_profile_vocabulary",
        )
    }
    residual_dependency_class_vocab = {
        ensure_str(item, "matrix.residual_dependency_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("residual_dependency_class_vocabulary"),
            "matrix.residual_dependency_class_vocabulary",
        )
    }
    posture_class_vocab = {
        ensure_str(item, "matrix.posture_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("posture_class_vocabulary"),
            "matrix.posture_class_vocabulary",
        )
    }
    surface_family_vocab = {
        ensure_str(item, "matrix.surface_family_vocabulary[]")
        for item in ensure_list(
            matrix.get("surface_family_vocabulary"),
            "matrix.surface_family_vocabulary",
        )
    }
    truth_field_vocab = {
        ensure_str(item, "matrix.truth_field_vocabulary[]")
        for item in ensure_list(
            matrix.get("truth_field_vocabulary"),
            "matrix.truth_field_vocabulary",
        )
    }
    manifest_status_vocab = {
        ensure_str(item, "matrix.manifest_status_vocabulary[]")
        for item in ensure_list(
            matrix.get("manifest_status_vocabulary"),
            "matrix.manifest_status_vocabulary",
        )
    }
    failure_drill_id_vocab = {
        ensure_str(item, "matrix.failure_drill_id_vocabulary[]")
        for item in ensure_list(
            matrix.get("failure_drill_id_vocabulary"),
            "matrix.failure_drill_id_vocabulary",
        )
    }
    required_surface_family_coverage = {
        ensure_str(item, "matrix.required_surface_family_coverage[]")
        for item in ensure_list(
            matrix.get("required_surface_family_coverage"),
            "matrix.required_surface_family_coverage",
        )
    }

    # Cross-file agreement: the matrix's deployment-profile vocabulary
    # MUST match the canonical register one-for-one.
    register_profiles = load_profile_register(
        repo_root,
        ensure_str(
            matrix.get("deployment_profile_register_ref"),
            "matrix.deployment_profile_register_ref",
        ),
    )
    if deployment_profile_vocab != register_profiles:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.deployment_profile_vocabulary.mismatch",
                message=(
                    "matrix.deployment_profile_vocabulary does not match "
                    "artifacts/governance/deployment_profiles.yaml verbatim; "
                    f"matrix-only: {sorted(deployment_profile_vocab - register_profiles)}; "
                    f"register-only: {sorted(register_profiles - deployment_profile_vocab)}"
                ),
                remediation=(
                    "Update the matrix vocabulary in lock-step with the "
                    "canonical deployment-profile register; never fork the "
                    "vocabulary here."
                ),
            )
        )

    # Matrix dependency-class vocab MUST be a subset of the ledger vocab.
    ledger_dep_classes = load_ledger_dependency_classes(
        repo_root,
        ensure_str(
            matrix.get("residual_dependency_ledger_ref"),
            "matrix.residual_dependency_ledger_ref",
        ),
    )
    unknown_dep_classes = residual_dependency_class_vocab - ledger_dep_classes
    if unknown_dep_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.residual_dependency_class_vocabulary.unknown",
                message=(
                    "matrix.residual_dependency_class_vocabulary contains "
                    "values absent from the canonical residual-dependency "
                    f"ledger: {sorted(unknown_dep_classes)}"
                ),
                remediation=(
                    "Add new dependency classes to "
                    "artifacts/governance/residual_dependencies.yaml first; "
                    "the matrix is a consumer, not the source of truth."
                ),
            )
        )

    # Posture vocab MUST match the ledger verbatim.
    ledger_posture_vocab = load_ledger_posture_classes(
        repo_root,
        ensure_str(
            matrix.get("residual_dependency_ledger_ref"),
            "matrix.residual_dependency_ledger_ref",
        ),
    )
    if posture_class_vocab != ledger_posture_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.posture_class_vocabulary.mismatch",
                message=(
                    "matrix.posture_class_vocabulary does not match the "
                    "ledger verbatim; "
                    f"matrix-only: {sorted(posture_class_vocab - ledger_posture_vocab)}; "
                    f"ledger-only: {sorted(ledger_posture_vocab - posture_class_vocab)}"
                ),
                remediation=(
                    "Keep posture vocabularies in lock-step with the "
                    "residual-dependency ledger."
                ),
            )
        )

    ledger_postures = load_ledger_postures(
        repo_root,
        ensure_str(
            matrix.get("residual_dependency_ledger_ref"),
            "matrix.residual_dependency_ledger_ref",
        ),
    )

    # Consumer bindings.
    consumer_bindings = ensure_dict(
        matrix.get("consumer_bindings"), "matrix.consumer_bindings"
    )
    named_consumer = ensure_dict(
        consumer_bindings.get("named_runtime_consumer"),
        "matrix.consumer_bindings.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        "matrix.consumer_bindings.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.missing",
                message=(
                    "named_runtime_consumer.consumer_ref does not exist: "
                    f"{consumer_ref}"
                ),
                remediation=(
                    "Point at a real downstream consumer or seed the surface "
                    "before claiming a runtime consumer exists."
                ),
                ref=consumer_ref,
            )
        )
    consumed_fields = ensure_list(
        named_consumer.get("consumed_fields"),
        "matrix.consumer_bindings.named_runtime_consumer.consumed_fields",
    )
    if not consumed_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.consumed_fields_empty",
                message=(
                    "named_runtime_consumer.consumed_fields must declare at "
                    "least one truth field"
                ),
                remediation=(
                    "Name the truth fields the runtime consumer reads from "
                    "the matrix."
                ),
            )
        )
    for token in consumed_fields:
        token = ensure_str(
            token,
            "matrix.consumer_bindings.named_runtime_consumer.consumed_fields[]",
        )
        if token not in truth_field_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.named_runtime_consumer.consumed_field_unknown",
                    message=(
                        f"named_runtime_consumer.consumed_fields contains "
                        f"'{token}' which is not in truth_field_vocabulary"
                    ),
                    remediation=(
                        "Add the truth field to the schema vocabulary first; "
                        "consumers are not allowed to invent fields."
                    ),
                )
            )

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least one capability row",
                remediation="Seed the required rows.",
            )
        )

    # Resolve --force-drill if requested.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<row_id>:<drill_id>'"
            )
        # Row ids contain ':' (e.g. 'boundary_row:shell.frame_and_status'),
        # so split on the rightmost ':' to recover the drill suffix.
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_surface_families: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")

        forced_overrides: dict[str, Any] = {}
        if forced_row_id is not None and ensure_str(
            row.get("row_id"), "row.row_id"
        ) == forced_row_id:
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            drill_id = ensure_str(
                failure_drill.get("drill_id"),
                f"{forced_row_id}.failure_drill.drill_id",
            )
            if drill_id != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id '{forced_drill_id}' does not "
                    f"match the row's failure_drill.drill_id '{drill_id}'"
                )
            forced_overrides = ensure_dict(
                failure_drill.get("forced_input"),
                f"{forced_row_id}.failure_drill.forced_input",
            )

        result = replay_row(
            row,
            repo_root=repo_root,
            boundary_class_vocab=boundary_class_vocab,
            deployment_profile_vocab=deployment_profile_vocab,
            residual_dependency_class_vocab=residual_dependency_class_vocab,
            posture_class_vocab=posture_class_vocab,
            surface_family_vocab=surface_family_vocab,
            truth_field_vocab=truth_field_vocab,
            manifest_status_vocab=manifest_status_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            ledger_postures=ledger_postures,
            forced_overrides=forced_overrides,
        )
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate row_id: {result.row_id}",
                    remediation="row_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)
        seen_surface_families.add(result.surface_family)

        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_overrides
        ):
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            expected_check = ensure_str(
                failure_drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_families = required_surface_family_coverage - seen_surface_families
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_surface_families",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(required_surface_family_coverage)}; missing: "
                    f"{sorted(missing_families)}"
                ),
                remediation=(
                    "Add the missing rows so every required surface family "
                    "is exercised."
                ),
            )
        )

    # Promote per-row failures into findings, but skip them under
    # force-drill mode for the targeted row so the runner's exit can
    # reflect the reproduce verdict rather than failing on the
    # reproduced check.
    for result in row_results:
        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get("check_id", "matrix.row.failed_check"),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the canonical sources or fix "
                        "the drift in the matrix; failures are reported with "
                        "the precise actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "boundary_manifest_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "schema_ref": args.schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_boundary_manifest_lane/"
            "run_m1_boundary_manifest_lane.py --repo-root ."
        ),
        "status": status,
        "required_surface_family_coverage": sorted(required_surface_family_coverage),
        "observed_surface_families": sorted(seen_surface_families),
        "rows": [
            {
                "row_id": r.row_id,
                "surface_family": r.surface_family,
                "boundary_class": r.boundary_class,
                "manifest_status": r.manifest_status,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "boundary-manifest"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']} "
                f"on {forced_replay_record['row_id']} reproduced "
                f"{forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on "
            f"{forced_replay_record['row_id']} did NOT reproduce "
            f"{forced_replay_record['expected_check_id']}; "
            f"observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[boundary-manifest] interrupted", file=sys.stderr)
        sys.exit(130)
