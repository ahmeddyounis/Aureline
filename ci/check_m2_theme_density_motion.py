#!/usr/bin/env python3
"""Validate the M2 theme, density, motion, and visual-diff alpha lane."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/design/m2_theme_density_motion/manifest.yaml"
DEFAULT_VISUAL_MANIFEST_REL = "artifacts/design/m2_appearance_visual_diff_alpha/manifest.yaml"
DEFAULT_IMPORT_PROJECTION_REL = (
    "fixtures/design/m2_theme_density_motion/imported_theme_review_before_commit.yaml"
)
DEFAULT_POWER_SAVER_REL = "fixtures/design/m2_theme_density_motion/power_saver_motion_floor.yaml"

REQUIRED_THEMES = {
    "dark_reference",
    "light_parity",
    "high_contrast_dark",
    "high_contrast_light",
}
REQUIRED_DENSITIES = {"compact", "standard", "comfortable"}
REQUIRED_MOTION_POSTURES = {
    "motion_standard",
    "motion_reduced",
    "motion_low_motion",
    "motion_power_saver",
}
REQUIRED_FIXTURE_CASES = {
    "first_party_theme_manifest_full_modes",
    "appearance_session_atomic_checkpoint",
    "workspace_overlay_roundtrip_inert_unmapped",
    "imported_theme_review_before_commit",
    "os_forced_colors_reload_disclosed",
    "power_saver_motion_floor_preserves_attention",
    "launch_wedge_visual_diff_suite",
}
REQUIRED_MOTION_SURFACES = {
    "ai_assist_panel",
    "terminal_panel",
    "list_panel",
    "decorative_shell_chrome",
}
REQUIRED_GATED_SURFACES = {
    "trust_prompt_canvas",
    "onboarding_start_center",
    "notification_surface",
}
REQUIRED_RUNTIME_MARKERS = {
    "crates/aureline-ui/src/themes/session.rs": [
        "apply_checkpointed_changes",
        "revert_to_checkpoint",
        "AppearanceChangeSet",
    ],
    "crates/aureline-ui/src/themes/package.rs": [
        "first_party_theme_package_manifest",
        "supports_theme_class",
    ],
    "crates/aureline-ui/src/themes/import_review.rs": [
        "imported_theme_mapping_report_with_warnings",
        "PartialWithVisibleGaps",
    ],
    "crates/aureline-ui/src/themes/audit.rs": [
        "alpha_appearance_audit_manifest",
        "gate_requires_review",
    ],
}


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
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--visual-manifest", default=DEFAULT_VISUAL_MANIFEST_REL)
    parser.add_argument("--import-projection", default=DEFAULT_IMPORT_PROJECTION_REL)
    parser.add_argument("--power-saver", default=DEFAULT_POWER_SAVER_REL)
    parser.add_argument("--report", default=None)
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


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def validate_header(payload: dict[str, Any], label: str, kind: str, findings: list[Finding]) -> None:
    if payload.get("record_kind") != kind:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.record_kind.invalid",
                message=f"{label}.record_kind must be {kind}",
                remediation="Restore the lane record kind or update this validator.",
            )
        )
    if payload.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1",
                remediation="Update this validator in the same change that bumps schema_version.",
            )
        )


def validate_fixture_manifest(
    repo_root: Path, manifest: dict[str, Any], findings: list[Finding]
) -> None:
    validate_header(manifest, "fixture_manifest", "theme_density_motion_fixture_manifest", findings)
    required = set(
        ensure_str(item, "fixture_manifest.required_case_ids[]")
        for item in ensure_list(manifest.get("required_case_ids"), "fixture_manifest.required_case_ids")
    )
    missing_required = REQUIRED_FIXTURE_CASES - required
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.required_case_ids.missing",
                message="fixture manifest omits required M2 appearance cases",
                remediation="Add fixture rows for the missing acceptance cases.",
                details={"missing": sorted(missing_required)},
            )
        )

    cases = ensure_list(manifest.get("fixture_cases"), "fixture_manifest.fixture_cases")
    case_ids: set[str] = set()
    for idx, raw_case in enumerate(cases):
        case = ensure_dict(raw_case, f"fixture_manifest.fixture_cases[{idx}]")
        case_id = ensure_str(case.get("case_id"), f"fixture_manifest.fixture_cases[{idx}].case_id")
        case_ids.add(case_id)
        validate_path_ref(
            repo_root,
            ensure_str(case.get("fixture_ref"), f"fixture_manifest.fixture_cases[{idx}].fixture_ref"),
            "fixture_manifest.fixture_cases.fixture_ref",
            findings,
        )
        for optional_ref_key in ("runtime_consumer_ref", "source_mapping_report_ref"):
            ref = case.get(optional_ref_key)
            if ref is not None:
                validate_path_ref(
                    repo_root,
                    ensure_str(ref, f"fixture_manifest.fixture_cases[{idx}].{optional_ref_key}"),
                    f"fixture_manifest.fixture_cases.{optional_ref_key}",
                    findings,
                )
        if ensure_str(case.get("expected_result"), f"fixture_manifest.fixture_cases[{idx}].expected_result") != "pass":
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture_manifest.fixture_cases.expected_result",
                    message=f"{case_id} must expect pass",
                    remediation="Only protected passing cases can close this alpha lane.",
                    ref=case_id,
                )
            )
    missing_cases = REQUIRED_FIXTURE_CASES - case_ids
    if missing_cases:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture_manifest.fixture_cases.missing",
                message="fixture_cases does not include every required case id",
                remediation="Add the missing case rows.",
                details={"missing": sorted(missing_cases)},
            )
        )


def validate_visual_manifest(
    repo_root: Path, manifest: dict[str, Any], findings: list[Finding]
) -> None:
    validate_header(
        manifest,
        "visual_manifest",
        "appearance_visual_diff_alpha_manifest",
        findings,
    )
    for key in ("source_refs", "runtime_consumer_refs", "os_transition_evidence_refs", "import_review_refs"):
        for raw_ref in ensure_list(manifest.get(key), f"visual_manifest.{key}"):
            validate_path_ref(repo_root, ensure_str(raw_ref, f"visual_manifest.{key}[]"), f"visual_manifest.{key}", findings)

    themes = set(ensure_list(manifest.get("claimed_theme_classes"), "visual_manifest.claimed_theme_classes"))
    densities = set(ensure_list(manifest.get("claimed_density_classes"), "visual_manifest.claimed_density_classes"))
    postures = set(ensure_list(manifest.get("claimed_motion_postures"), "visual_manifest.claimed_motion_postures"))
    for label, actual, required in (
        ("themes", themes, REQUIRED_THEMES),
        ("densities", densities, REQUIRED_DENSITIES),
        ("motion_postures", postures, REQUIRED_MOTION_POSTURES),
    ):
        missing = required - actual
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"visual_manifest.claimed_{label}.missing",
                    message=f"visual manifest omits claimed {label}",
                    remediation="Add the missing claimed appearance rows.",
                    details={"missing": sorted(missing)},
                )
            )

    session_ref = ensure_str(manifest.get("appearance_session_ref"), "visual_manifest.appearance_session_ref")
    visual_cases = ensure_list(manifest.get("visual_diff_cases"), "visual_manifest.visual_diff_cases")
    visual_theme_coverage: set[str] = set()
    for idx, raw_case in enumerate(visual_cases):
        case = ensure_dict(raw_case, f"visual_manifest.visual_diff_cases[{idx}]")
        case_id = ensure_str(case.get("case_id"), f"visual_manifest.visual_diff_cases[{idx}].case_id")
        theme = ensure_str(case.get("theme_class"), f"visual_manifest.visual_diff_cases[{idx}].theme_class")
        visual_theme_coverage.add(theme)
        if ensure_str(case.get("appearance_session_ref"), f"visual_manifest.visual_diff_cases[{idx}].appearance_session_ref") != session_ref:
            findings.append(
                Finding(
                    severity="error",
                    check_id="visual_manifest.visual_diff_cases.session_mismatch",
                    message=f"{case_id} does not cite the manifest appearance session",
                    remediation="Tie all visual-diff cases to one appearance-session object.",
                    ref=case_id,
                )
            )
        for key in ("baseline_ref", "comparison_ref", "accessibility_audit_ref"):
            validate_path_ref(
                repo_root,
                ensure_str(case.get(key), f"visual_manifest.visual_diff_cases[{idx}].{key}"),
                f"visual_manifest.visual_diff_cases.{key}",
                findings,
            )
        if case.get("semantic_stability_result") != "stable" or case.get("minimum_contrast_result") != "pass":
            findings.append(
                Finding(
                    severity="error",
                    check_id="visual_manifest.visual_diff_cases.result_not_passing",
                    message=f"{case_id} must have stable semantics and passing contrast",
                    remediation="Refresh the audit row or narrow the claim.",
                    ref=case_id,
                )
            )
    missing_visual = REQUIRED_THEMES - visual_theme_coverage
    if missing_visual:
        findings.append(
            Finding(
                severity="error",
                check_id="visual_manifest.visual_diff_cases.theme_coverage_missing",
                message="visual diff cases do not cover all required theme classes",
                remediation="Add dark, light, and high-contrast visual-diff rows.",
                details={"missing": sorted(missing_visual)},
            )
        )

    density_coverage: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(manifest.get("density_audit_rows"), "visual_manifest.density_audit_rows")):
        row = ensure_dict(raw_row, f"visual_manifest.density_audit_rows[{idx}]")
        density_coverage.add(ensure_str(row.get("density_class"), f"visual_manifest.density_audit_rows[{idx}].density_class"))
        validate_path_ref(
            repo_root,
            ensure_str(row.get("fixture_ref"), f"visual_manifest.density_audit_rows[{idx}].fixture_ref"),
            "visual_manifest.density_audit_rows.fixture_ref",
            findings,
        )
        for key in ("affects_information_architecture", "affects_focus_visibility", "affects_state_conveyance"):
            if row.get(key) is not False:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"visual_manifest.density_audit_rows.{key}",
                        message=f"density row must keep {key}=false",
                        remediation="Density may affect spacing only; narrow or deny the claim.",
                        ref=ensure_str(row.get("row_id"), "density row_id"),
                    )
                )
    if REQUIRED_DENSITIES - density_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="visual_manifest.density_audit_rows.coverage_missing",
                message="density audit rows do not cover compact, standard, and comfortable",
                remediation="Add one audit row for each claimed density.",
                details={"missing": sorted(REQUIRED_DENSITIES - density_coverage)},
            )
        )

    motion_surfaces: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(manifest.get("motion_reduction_rows"), "visual_manifest.motion_reduction_rows")):
        row = ensure_dict(raw_row, f"visual_manifest.motion_reduction_rows[{idx}]")
        motion_surfaces.add(ensure_str(row.get("surface_class"), f"visual_manifest.motion_reduction_rows[{idx}].surface_class"))
        if row.get("critical_attention_cue_preserved") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="visual_manifest.motion_reduction_rows.cue_not_preserved",
                    message="motion-reduction row must preserve critical attention cues",
                    remediation="Add non-motion cue evidence or narrow the claim.",
                    ref=ensure_str(row.get("row_id"), "motion row_id"),
                )
            )
        validate_path_ref(
            repo_root,
            ensure_str(row.get("state_cue_ref"), f"visual_manifest.motion_reduction_rows[{idx}].state_cue_ref"),
            "visual_manifest.motion_reduction_rows.state_cue_ref",
            findings,
        )
    if REQUIRED_MOTION_SURFACES - motion_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="visual_manifest.motion_reduction_rows.coverage_missing",
                message="motion reduction rows do not cover AI, terminal, list, and decorative surfaces",
                remediation="Add power-saver/reduced-motion rows for each required surface.",
                details={"missing": sorted(REQUIRED_MOTION_SURFACES - motion_surfaces)},
            )
        )

    gated: set[str] = set()
    for idx, raw_gate in enumerate(ensure_list(manifest.get("safety_critical_change_gates"), "visual_manifest.safety_critical_change_gates")):
        gate = ensure_dict(raw_gate, f"visual_manifest.safety_critical_change_gates[{idx}]")
        gated.add(ensure_str(gate.get("surface_class"), f"visual_manifest.safety_critical_change_gates[{idx}].surface_class"))
        if gate.get("requires_visual_diff") is not True or gate.get("requires_accessibility_audit") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="visual_manifest.safety_critical_change_gates.review_required",
                    message="safety-critical gates must require visual diff and accessibility audit",
                    remediation="Set both review booleans to true.",
                    ref=ensure_str(gate.get("gate_id"), "gate_id"),
                )
            )
        validate_path_ref(
            repo_root,
            ensure_str(gate.get("evidence_ref"), f"visual_manifest.safety_critical_change_gates[{idx}].evidence_ref"),
            "visual_manifest.safety_critical_change_gates.evidence_ref",
            findings,
        )
    if REQUIRED_GATED_SURFACES - gated:
        findings.append(
            Finding(
                severity="error",
                check_id="visual_manifest.safety_critical_change_gates.coverage_missing",
                message="safety-critical visual gates are missing required surfaces",
                remediation="Gate trust, onboarding/import, and notification token changes.",
                details={"missing": sorted(REQUIRED_GATED_SURFACES - gated)},
            )
        )


def validate_import_projection(
    repo_root: Path, projection: dict[str, Any], findings: list[Finding]
) -> None:
    validate_header(projection, "import_projection", "imported_theme_review_projection", findings)
    source_ref = ensure_str(projection.get("source_mapping_report_ref"), "import_projection.source_mapping_report_ref")
    validate_path_ref(repo_root, source_ref, "import_projection.source_mapping_report_ref", findings)
    source = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(source_ref)), "source_mapping_report")
    source_summary = ensure_dict(source.get("mapping_summary"), "source_mapping_report.mapping_summary")
    review_summary = ensure_dict(projection.get("review_summary"), "import_projection.review_summary")
    for key in (
        "translated_slot_count",
        "substituted_with_fallback_count",
        "unsupported_slot_count",
        "unresolved_mapping_count",
        "blocked_honesty_count",
        "deprecated_replacement_count",
    ):
        if review_summary.get(key) != source_summary.get(key):
            findings.append(
                Finding(
                    severity="error",
                    check_id="import_projection.review_summary.count_mismatch",
                    message=f"review summary {key} does not match source mapping report",
                    remediation="Project counts directly from the audited mapping report.",
                    ref=source_ref,
                )
            )
    if review_summary.get("unresolved_mapping_count", 0) > 0 and review_summary.get("parity_claim_state") == "claimed_with_report":
        findings.append(
            Finding(
                severity="error",
                check_id="import_projection.parity_overclaim",
                message="import projection claims parity while unresolved rows remain",
                remediation="Keep parity partial until unresolved and unsupported rows are zero.",
            )
        )
    if review_summary.get("unresolved_slot_count_visible") is not True or review_summary.get("rollback_action_visible") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="import_projection.required_visibility_missing",
                message="unresolved slot count and rollback action must be visible before commit",
                remediation="Expose unresolved count and rollback action in the review projection.",
            )
        )


def validate_power_saver_fixture(fixture: dict[str, Any], findings: list[Finding]) -> None:
    validate_header(fixture, "power_saver", "power_saver_motion_floor_case", findings)
    precedence = ensure_dict(fixture.get("posture_precedence"), "power_saver.posture_precedence")
    if precedence.get("motion_power_saver", -1) <= precedence.get("motion_reduced", 999):
        findings.append(
            Finding(
                severity="error",
                check_id="power_saver.precedence.not_restrictive",
                message="power saver must be more restrictive than reduced motion",
                remediation="Raise motion_power_saver precedence above motion_reduced.",
            )
        )
    surfaces = set()
    for idx, raw_row in enumerate(ensure_list(fixture.get("surface_rows"), "power_saver.surface_rows")):
        row = ensure_dict(raw_row, f"power_saver.surface_rows[{idx}]")
        surfaces.add(ensure_str(row.get("surface_class"), f"power_saver.surface_rows[{idx}].surface_class"))
        cue = ensure_dict(row.get("critical_attention_cue"), f"power_saver.surface_rows[{idx}].critical_attention_cue")
        if cue.get("preserved") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="power_saver.surface_rows.cue_not_preserved",
                    message="critical attention cue must be preserved",
                    remediation="Add a non-motion cue or narrow the motion claim.",
                    ref=ensure_str(row.get("surface_class"), "surface_class"),
                )
            )
    if REQUIRED_MOTION_SURFACES - surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="power_saver.surface_rows.coverage_missing",
                message="power-saver fixture does not cover all required surfaces",
                remediation="Add AI, terminal, list, and decorative shell rows.",
                details={"missing": sorted(REQUIRED_MOTION_SURFACES - surfaces)},
            )
        )
    acceptance = ensure_dict(fixture.get("acceptance"), "power_saver.acceptance")
    for key in (
        "non_essential_motion_disabled_under_power_saver",
        "power_saver_not_less_restrictive_than_reduced_motion",
        "state_conveyance_preserved_without_animation",
        "focus_visibility_preserved",
    ):
        if acceptance.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"power_saver.acceptance.{key}",
                    message=f"power-saver acceptance flag must be true: {key}",
                    remediation="Fix the fixture or narrow the claim.",
                )
            )


def validate_runtime_markers(repo_root: Path, findings: list[Finding]) -> None:
    for rel, markers in REQUIRED_RUNTIME_MARKERS.items():
        path = repo_root / rel
        if not path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="runtime_markers.missing_file",
                    message=f"runtime consumer file is missing: {rel}",
                    remediation="Add the runtime consumer or update the lane contract.",
                    ref=rel,
                )
            )
            continue
        payload = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in payload:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="runtime_markers.missing_marker",
                        message=f"{rel} does not contain required marker {marker}",
                        remediation="Wire the runtime projection before claiming the lane.",
                        ref=rel,
                    )
                )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "validated_lane": "m2_theme_density_motion_alpha",
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    fixture_manifest = ensure_dict(
        render_yaml_as_json(repo_root / args.fixture_manifest),
        "fixture_manifest",
    )
    visual_manifest = ensure_dict(
        render_yaml_as_json(repo_root / args.visual_manifest),
        "visual_manifest",
    )
    import_projection = ensure_dict(
        render_yaml_as_json(repo_root / args.import_projection),
        "import_projection",
    )
    power_saver = ensure_dict(render_yaml_as_json(repo_root / args.power_saver), "power_saver")

    validate_fixture_manifest(repo_root, fixture_manifest, findings)
    validate_visual_manifest(repo_root, visual_manifest, findings)
    validate_import_projection(repo_root, import_projection, findings)
    validate_power_saver_fixture(power_saver, findings)
    validate_runtime_markers(repo_root, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for finding in errors:
            print(
                f"ERROR {finding.check_id}: {finding.message}",
                file=sys.stderr,
            )
        return 1
    print("m2 theme/density/motion appearance validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
