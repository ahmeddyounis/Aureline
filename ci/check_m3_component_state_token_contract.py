#!/usr/bin/env python3
"""Validate beta component-state and token-conformance fixtures."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REQUIRED_STATES = {
    "empty",
    "loading",
    "pending",
    "degraded",
    "blocked",
    "error",
    "completed",
}
REQUIRED_CUE_FAMILIES = {"lifecycle", "route", "readiness", "policy"}
REQUIRED_CUE_KINDS = {"badge", "notice"}
REQUIRED_SURFACES = {
    "shell_chrome",
    "start_center",
    "command_palette",
    "search_surface",
    "dialog_sheet",
    "trust_prompt",
    "notification_envelope",
    "help_about_row",
    "settings_root",
    "activity_center_row",
}
REQUIRED_THEMES = {
    "dark_reference",
    "light_parity",
    "high_contrast_dark",
    "high_contrast_light",
}
REQUIRED_DENSITIES = {"compact", "standard", "comfortable"}
REQUIRED_POSTURES = {
    "motion_standard",
    "motion_reduced",
    "motion_low_motion",
    "motion_power_saver",
    "motion_critical_hot_path",
}


@dataclass
class Finding:
    check_id: str
    message: str
    ref: str | None = None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--registry",
        default="fixtures/ux/m3/state_semantics/component_state_registry.json",
    )
    parser.add_argument(
        "--screenshot-diff",
        default="fixtures/ux/m3/state_semantics/screenshot_diff_matrix.json",
    )
    parser.add_argument(
        "--screenshot-artifact",
        default="artifacts/ux/m3/component_state_screenshot_diff/packet.json",
    )
    parser.add_argument(
        "--token-conformance",
        default="fixtures/ux/m3/state_semantics/token_conformance_report.json",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"failed to parse JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_registry(repo_root: Path, registry: dict[str, Any], findings: list[Finding]) -> None:
    if registry.get("record_kind") != "component_state_registry_beta_record":
        findings.append(Finding("registry.record_kind", "registry record kind is invalid"))
    if registry.get("schema_version") != 1:
        findings.append(Finding("registry.schema_version", "registry schema version must be 1"))

    for key in ("source_refs", "runtime_consumer_refs"):
        for raw_ref in ensure_list(registry.get(key), f"registry.{key}"):
            if not isinstance(raw_ref, str) or not ref_exists(repo_root, raw_ref):
                findings.append(Finding(f"registry.{key}.missing", "registry ref does not resolve", str(raw_ref)))

    state_rows = [
        ensure_dict(row, "component_state_families[]")
        for row in ensure_list(registry.get("component_state_families"), "component_state_families")
    ]
    states = {row.get("state_class") for row in state_rows}
    missing_states = REQUIRED_STATES - states
    if missing_states:
        findings.append(Finding("registry.states.missing", f"missing required states: {sorted(missing_states)}"))
    for row in state_rows:
        state = str(row.get("state_class"))
        if row.get("hue_only_allowed") is not False:
            findings.append(Finding("registry.state.hue_only_allowed", "state permits hue-only meaning", state))
        if row.get("hover_only_critical_action_allowed") is not False:
            findings.append(Finding("registry.state.hover_only", "state permits hover-only critical action", state))
        if state == "blocked" and row.get("spinner_only_allowed") is not False:
            findings.append(Finding("registry.state.spinner_only_blocked", "blocked permits spinner-only treatment", state))
        if not ensure_list(row.get("required_non_color_cues"), f"{state}.required_non_color_cues"):
            findings.append(Finding("registry.state.non_color_cues", "state has no non-color cues", state))

    cue_rows = [
        ensure_dict(row, "cue_families[]")
        for row in ensure_list(registry.get("cue_families"), "cue_families")
    ]
    cue_pairs = {(row.get("family_class"), row.get("cue_kind")) for row in cue_rows}
    for family in REQUIRED_CUE_FAMILIES:
        for kind in REQUIRED_CUE_KINDS:
            if (family, kind) not in cue_pairs:
                findings.append(Finding("registry.cue_family.missing", f"missing {family}.{kind}"))
    for row in cue_rows:
        cues = set(ensure_list(row.get("required_non_color_cues"), "cue.required_non_color_cues"))
        if row.get("requires_text_shape_fallback") is not True or not {"label_text", "shape"}.issubset(cues):
            findings.append(Finding("registry.cue_family.text_shape", "cue family lacks text and shape fallback", str(row.get("family_class"))))
        tokens = ensure_list(row.get("vocabulary_tokens"), "cue.vocabulary_tokens")
        token_ids = {ensure_dict(token, "cue token").get("token") for token in tokens}
        if row.get("honesty_fallback_token") not in token_ids:
            findings.append(Finding("registry.cue_family.fallback", "fallback token is not in vocabulary", str(row.get("family_class"))))

    surface_rows = [
        ensure_dict(row, "launch_surface_consumers[]")
        for row in ensure_list(registry.get("launch_surface_consumers"), "launch_surface_consumers")
    ]
    surfaces = {row.get("surface_class") for row in surface_rows}
    missing_surfaces = REQUIRED_SURFACES - surfaces
    if missing_surfaces:
        findings.append(Finding("registry.surfaces.missing", f"missing surfaces: {sorted(missing_surfaces)}"))
    for row in surface_rows:
        surface = str(row.get("surface_class"))
        for raw_ref in ensure_list(row.get("consumer_refs"), f"{surface}.consumer_refs"):
            if not isinstance(raw_ref, str) or not ref_exists(repo_root, raw_ref):
                findings.append(Finding("registry.surface.ref_missing", "surface consumer ref does not resolve", str(raw_ref)))
        if row.get("launch_priority") == "out_of_scope" and not row.get("waiver_ref"):
            findings.append(Finding("registry.surface.waiver_missing", "out-of-scope surface lacks waiver", surface))
        if row.get("launch_priority") != "out_of_scope":
            if not set(ensure_list(row.get("consumes_component_states"), f"{surface}.states")).issuperset(REQUIRED_STATES):
                findings.append(Finding("registry.surface.states", "surface does not consume all canonical states", surface))
            if not set(ensure_list(row.get("consumes_cue_families"), f"{surface}.cues")).issuperset(REQUIRED_CUE_FAMILIES):
                findings.append(Finding("registry.surface.cues", "surface does not consume all cue families", surface))
            for key in ("critical_actions_keyboard_reachable", "focus_visibility_preserved", "no_color_only_state_meaning"):
                if row.get(key) is not True:
                    findings.append(Finding(f"registry.surface.{key}", f"surface has {key}=false", surface))


def validate_screenshot_packet(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != "component_state_screenshot_diff_packet":
        findings.append(Finding("screenshot.record_kind", "screenshot packet record kind is invalid"))
    if packet.get("gate_state") != "pass" or packet.get("findings") != []:
        findings.append(Finding("screenshot.gate", "screenshot packet must pass with no findings"))
    rows = [
        ensure_dict(row, "screenshot.rows[]")
        for row in ensure_list(packet.get("rows"), "screenshot.rows")
    ]
    surfaces = {row.get("surface_class") for row in rows}
    states = {row.get("state_class") for row in rows}
    themes = {row.get("theme_class") for row in rows}
    densities = {row.get("density_class") for row in rows}
    postures = {row.get("motion_posture") for row in rows}
    for label, actual, required in (
        ("surface", surfaces, REQUIRED_SURFACES),
        ("state", states, REQUIRED_STATES),
        ("theme", themes, REQUIRED_THEMES),
        ("density", densities, REQUIRED_DENSITIES),
        ("motion", postures, REQUIRED_POSTURES),
    ):
        missing = required - actual
        if missing:
            findings.append(Finding(f"screenshot.{label}.missing", f"missing {label} coverage: {sorted(missing)}"))
    for row in rows:
        row_id = str(row.get("row_id"))
        checks = {
            "hover_only_critical_actions_absent": "hover-only critical action present",
            "focus_visibility_present": "focus visibility missing",
            "spinner_only_blocked_state_absent": "spinner-only blocked state present",
            "color_only_state_meaning_absent": "color-only state meaning present",
            "semantic_stability_passed": "semantic stability failed",
        }
        for key, message in checks.items():
            if row.get(key) is not True:
                findings.append(Finding(f"screenshot.{key}", message, row_id))
        if not ensure_list(row.get("required_non_color_cues"), f"{row_id}.required_non_color_cues"):
            findings.append(Finding("screenshot.non_color_cues", "row has no non-color cues", row_id))
        for ref_key in (
            "baseline_capture_ref",
            "comparison_capture_ref",
            "diff_artifact_ref",
            "keyboard_journey_ref",
            "assistive_technology_ref",
            "token_conformance_ref",
        ):
            ref = row.get(ref_key)
            if not isinstance(ref, str) or not ref_exists(repo_root, ref):
                findings.append(Finding(f"screenshot.{ref_key}.missing", "screenshot row reference does not resolve", str(ref)))


def validate_token_conformance(repo_root: Path, packet: dict[str, Any], findings: list[Finding]) -> None:
    if packet.get("record_kind") != "token_conformance_packet_record":
        findings.append(Finding("token.record_kind", "token packet record kind is invalid"))
    if packet.get("gate_state") != "pass" or packet.get("findings") != []:
        findings.append(Finding("token.gate", "token packet must pass with no findings"))
    if packet.get("raw_private_material_excluded") is not True:
        findings.append(Finding("token.raw_private_material", "token packet must exclude raw private material"))
    for ref_key in ("registry_ref", "screenshot_diff_packet_ref", "shell_token_state_audit_ref"):
        ref = packet.get(ref_key)
        if not isinstance(ref, str) or not ref_exists(repo_root, ref):
            findings.append(Finding(f"token.{ref_key}.missing", "token packet reference does not resolve", str(ref)))
    rows = [
        ensure_dict(row, "token.rows[]")
        for row in ensure_list(packet.get("rows"), "token.rows")
    ]
    surfaces = {row.get("surface_class") for row in rows}
    missing = REQUIRED_SURFACES - surfaces
    if missing:
        findings.append(Finding("token.surface.missing", f"missing token rows: {sorted(missing)}"))
    for row in rows:
        surface = str(row.get("surface_class"))
        for raw_ref in ensure_list(row.get("source_refs"), f"{surface}.source_refs"):
            if not isinstance(raw_ref, str) or not ref_exists(repo_root, raw_ref):
                findings.append(Finding("token.source_ref.missing", "token source ref does not resolve", str(raw_ref)))
        if row.get("launch_priority") != "out_of_scope":
            if not set(ensure_list(row.get("required_component_states"), f"{surface}.states")).issuperset(REQUIRED_STATES):
                findings.append(Finding("token.states", "row does not require all canonical states", surface))
            if not set(ensure_list(row.get("required_cue_families"), f"{surface}.cues")).issuperset(REQUIRED_CUE_FAMILIES):
                findings.append(Finding("token.cues", "row does not require all cue families", surface))
            if row.get("raw_color_literals_forbidden") is not True:
                findings.append(Finding("token.raw_color", "row allows raw colors", surface))
            if row.get("local_token_forks_forbidden") is not True:
                findings.append(Finding("token.local_fork", "row allows local token forks", surface))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    registry = ensure_dict(load_json(repo_root / args.registry), args.registry)
    screenshot = ensure_dict(load_json(repo_root / args.screenshot_diff), args.screenshot_diff)
    screenshot_artifact = ensure_dict(load_json(repo_root / args.screenshot_artifact), args.screenshot_artifact)
    token = ensure_dict(load_json(repo_root / args.token_conformance), args.token_conformance)

    validate_registry(repo_root, registry, findings)
    validate_screenshot_packet(repo_root, screenshot, findings)
    validate_screenshot_packet(repo_root, screenshot_artifact, findings)
    if screenshot != screenshot_artifact:
        findings.append(Finding("screenshot.artifact_drift", "fixture screenshot matrix and release packet differ"))
    validate_token_conformance(repo_root, token, findings)

    if findings:
        for finding in findings:
            ref = f" ({finding.ref})" if finding.ref else ""
            print(f"ERROR {finding.check_id}: {finding.message}{ref}", file=sys.stderr)
        return 1
    print("component-state and token beta contract validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
