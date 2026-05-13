#!/usr/bin/env python3
"""Validate the alpha token/state semantics and badge-family registry."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REGISTRY_REL = "artifacts/design/state_badge_families_alpha.yaml"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/design/m2_state_semantics/manifest.yaml"

REQUIRED_STATE_CLASSES = {
    "empty",
    "loading",
    "pending",
    "degraded",
    "blocked",
    "error",
    "completed",
    "focus_visible",
    "selection",
    "active_target",
    "trust_restricted",
    "policy_locked",
    "readiness_ready",
    "readiness_partial",
}

REQUIRED_BADGE_FAMILIES = {
    "lifecycle",
    "route",
    "support_class",
    "readiness",
    "policy",
    "trust",
    "docs_help",
    "package_marketplace",
    "support_export",
    "theme_package",
}

REQUIRED_NOTICE_FAMILIES = {
    "info",
    "warning",
    "degraded",
    "blocked",
    "restricted",
    "success",
}

REQUIRED_SURFACES = {
    "shell_chrome",
    "editor_canvas",
    "command_palette_and_search",
    "docs_help_canvas",
    "package_marketplace_canvas",
    "support_export",
    "trust_prompt_canvas",
    "extension_embedded_canvas",
}

REQUIRED_CASES = {
    "command_palette_loading_pending_distinct",
    "shell_editor_degraded_blocked_trust_policy",
    "docs_help_badge_projection",
    "package_marketplace_support_policy_badges",
    "trust_notice_families",
    "extension_embedded_inheritance_gap",
}

REQUIRED_ACCEPTANCE_STATES = {
    "loading_pending_distinct",
    "trust_policy_non_hue_only",
    "badge_family_cross_surface",
    "embedded_inheritance_gap",
}

TEXT_SHAPE_CUES = {"label_text", "shape"}


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
    parser.add_argument("--registry", default=DEFAULT_REGISTRY_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
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


def render_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"failed to parse JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
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


def collect_token_names(repo_root: Path) -> set[str]:
    tokens: set[str] = set()
    for theme_rel in (
        "fixtures/design/themes/dark.json",
        "fixtures/design/themes/light.json",
        "fixtures/design/themes/hc-dark.json",
        "fixtures/design/themes/hc-light.json",
    ):
        theme = ensure_dict(render_json(repo_root / theme_rel), theme_rel)
        semantic_tokens = ensure_dict(theme.get("semantic_tokens"), f"{theme_rel}.semantic_tokens")
        tokens.update(str(key) for key in semantic_tokens)

    domains = ensure_dict(
        render_yaml_as_json(repo_root / "artifacts/design/semantic_token_domains.yaml"),
        "semantic_token_domains",
    )
    for raw_domain in ensure_list(domains.get("domains"), "semantic_token_domains.domains"):
        domain = ensure_dict(raw_domain, "semantic_token_domains.domains[]")
        for raw_token in domain.get("tokens") or []:
            token = ensure_dict(raw_token, "semantic_token_domains.tokens[]")
            name = token.get("token_name")
            if isinstance(name, str):
                tokens.add(name)
    return tokens


def validate_token_refs(
    token_refs: dict[str, Any],
    known_tokens: set[str],
    ref: str,
    findings: list[Finding],
) -> None:
    for slot in ("foreground", "border", "fill"):
        value = ensure_str(token_refs.get(slot), f"{ref}.token_refs.{slot}")
        if value not in known_tokens:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.unknown_token_ref",
                    message=f"{ref} references unknown semantic token {value}",
                    remediation="Use an existing token from the semantic token or theme ledgers.",
                    ref=ref,
                    details={"slot": slot, "token": value},
                )
            )


def validate_header(repo_root: Path, registry: dict[str, Any], findings: list[Finding]) -> None:
    if registry.get("record_kind") != "state_badge_family_alpha_registry":
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.record_kind.invalid",
                message="registry record_kind must be state_badge_family_alpha_registry",
                remediation="Restore the registry record_kind or update the validator in the same change.",
            )
        )
    if registry.get("state_badge_family_schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.schema_version.unsupported",
                message="state_badge_family_schema_version must be 1",
                remediation="Update the validator in the same change that bumps the schema.",
            )
        )
    for raw_ref in ensure_list(registry.get("source_of_truth_refs"), "registry.source_of_truth_refs"):
        validate_path_ref(repo_root, ensure_str(raw_ref, "registry.source_of_truth_refs[]"), "registry.source_of_truth_refs", findings)
    for raw_ref in ensure_list(registry.get("runtime_consumer_refs"), "registry.runtime_consumer_refs"):
        validate_path_ref(repo_root, ensure_str(raw_ref, "registry.runtime_consumer_refs[]"), "registry.runtime_consumer_refs", findings)


def validate_component_states(
    registry: dict[str, Any],
    known_tokens: set[str],
    findings: list[Finding],
) -> set[str]:
    rows = ensure_list(registry.get("component_state_families"), "registry.component_state_families")
    state_classes: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"registry.component_state_families[{idx}]")
        state_class = ensure_str(row.get("state_class"), f"component_state_families[{idx}].state_class")
        if state_class in state_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.component_state.duplicate",
                    message=f"duplicate component state class {state_class}",
                    remediation="Each component state class must be unique.",
                    ref=state_class,
                )
            )
        state_classes.add(state_class)
        cues = ensure_list(row.get("required_non_color_cues"), f"{state_class}.required_non_color_cues")
        if not cues:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.component_state.non_color_cues_missing",
                    message=f"{state_class} has no non-color cues",
                    remediation="Add label, icon, border, shape, progress, or lock/shield cues.",
                    ref=state_class,
                )
            )
        if row.get("hue_only_allowed") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.component_state.hue_only_allowed",
                    message=f"{state_class} permits hue-only state signaling",
                    remediation="Set hue_only_allowed to false and add non-color cues.",
                    ref=state_class,
                )
            )
        validate_token_refs(ensure_dict(row.get("token_refs"), f"{state_class}.token_refs"), known_tokens, state_class, findings)

    missing = REQUIRED_STATE_CLASSES - state_classes
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.component_state.required_missing",
                message="registry is missing required component state classes",
                remediation="Add rows for each required component-state class.",
                details={"missing": sorted(missing)},
            )
        )
    return state_classes


def validate_badge_families(
    registry: dict[str, Any],
    known_tokens: set[str],
    state_classes: set[str],
    findings: list[Finding],
) -> set[str]:
    rows = ensure_list(registry.get("badge_families"), "registry.badge_families")
    family_classes: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"registry.badge_families[{idx}]")
        family_class = ensure_str(row.get("badge_family_class"), f"badge_families[{idx}].badge_family_class")
        family_classes.add(family_class)
        cues = set(ensure_list(row.get("required_non_color_cues"), f"{family_class}.required_non_color_cues"))
        if row.get("requires_text_shape_fallback") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.badge_family.text_shape_fallback_not_required",
                    message=f"{family_class} does not require text-plus-shape fallback",
                    remediation="Set requires_text_shape_fallback to true for all badge families.",
                    ref=family_class,
                )
            )
        if not TEXT_SHAPE_CUES.issubset(cues):
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.badge_family.text_shape_fallback_missing",
                    message=f"{family_class} does not include label_text and shape cues",
                    remediation="Add label_text and shape to required_non_color_cues.",
                    ref=family_class,
                )
            )
        tokens = ensure_list(row.get("vocabulary_tokens"), f"{family_class}.vocabulary_tokens")
        token_ids: set[str] = set()
        for raw_token in tokens:
            token = ensure_dict(raw_token, f"{family_class}.vocabulary_tokens[]")
            token_id = ensure_str(token.get("token"), f"{family_class}.vocabulary_tokens[].token")
            token_ids.add(token_id)
            state_ref = ensure_str(token.get("state_class_ref"), f"{family_class}.{token_id}.state_class_ref")
            if state_ref not in state_classes:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="state_semantics.badge_family.unknown_state_ref",
                        message=f"{family_class}.{token_id} references unknown state class {state_ref}",
                        remediation="Use a state_class_ref published by component_state_families.",
                        ref=f"{family_class}.{token_id}",
                    )
                )
            validate_token_refs(ensure_dict(token.get("token_refs"), f"{family_class}.{token_id}.token_refs"), known_tokens, f"{family_class}.{token_id}", findings)
        fallback = ensure_str(row.get("honesty_fallback_token"), f"{family_class}.honesty_fallback_token")
        if fallback not in token_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.badge_family.fallback_missing",
                    message=f"{family_class} fallback token {fallback} is not in the family vocabulary",
                    remediation="Add the fallback token to vocabulary_tokens or choose an existing token.",
                    ref=family_class,
                )
            )
    missing = REQUIRED_BADGE_FAMILIES - family_classes
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.badge_family.required_missing",
                message="registry is missing required badge families",
                remediation="Add rows for each required badge family.",
                details={"missing": sorted(missing)},
            )
        )
    return family_classes


def validate_notice_families(
    registry: dict[str, Any],
    known_tokens: set[str],
    state_classes: set[str],
    findings: list[Finding],
) -> set[str]:
    rows = ensure_list(registry.get("notice_families"), "registry.notice_families")
    family_classes: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"registry.notice_families[{idx}]")
        family_class = ensure_str(row.get("notice_family_class"), f"notice_families[{idx}].notice_family_class")
        family_classes.add(family_class)
        cues = ensure_list(row.get("required_non_color_cues"), f"{family_class}.required_non_color_cues")
        if not cues:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.notice_family.non_color_cues_missing",
                    message=f"{family_class} has no non-color cues",
                    remediation="Add label, icon, border, shape, progress, or lock/shield cues.",
                    ref=family_class,
                )
            )
        state_ref = ensure_str(row.get("state_class_ref"), f"{family_class}.state_class_ref")
        if state_ref not in state_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.notice_family.unknown_state_ref",
                    message=f"{family_class} references unknown state class {state_ref}",
                    remediation="Use a state_class_ref published by component_state_families.",
                    ref=family_class,
                )
            )
        validate_token_refs(ensure_dict(row.get("token_refs"), f"{family_class}.token_refs"), known_tokens, family_class, findings)
    missing = REQUIRED_NOTICE_FAMILIES - family_classes
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.notice_family.required_missing",
                message="registry is missing required notice families",
                remediation="Add rows for each required notice family.",
                details={"missing": sorted(missing)},
            )
        )
    return family_classes


def validate_surface_consumers(
    repo_root: Path,
    registry: dict[str, Any],
    state_classes: set[str],
    badge_families: set[str],
    notice_families: set[str],
    findings: list[Finding],
) -> set[str]:
    rows = ensure_list(registry.get("surface_consumers"), "registry.surface_consumers")
    surfaces: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"registry.surface_consumers[{idx}]")
        surface = ensure_str(row.get("surface_class"), f"surface_consumers[{idx}].surface_class")
        surfaces.add(surface)
        validate_path_ref(repo_root, ensure_str(row.get("first_consumer_ref"), f"{surface}.first_consumer_ref"), "surface_consumers.first_consumer_ref", findings)
        for state in ensure_list(row.get("consumes_component_state_classes"), f"{surface}.consumes_component_state_classes"):
            state = ensure_str(state, f"{surface}.consumes_component_state_classes[]")
            if state not in state_classes:
                findings.append(Finding("error", "state_semantics.surface.unknown_state", f"{surface} consumes unknown state {state}", "Use a published component state class.", surface))
        for family in ensure_list(row.get("consumes_badge_family_classes"), f"{surface}.consumes_badge_family_classes"):
            family = ensure_str(family, f"{surface}.consumes_badge_family_classes[]")
            if family not in badge_families:
                findings.append(Finding("error", "state_semantics.surface.unknown_badge_family", f"{surface} consumes unknown badge family {family}", "Use a published badge family.", surface))
        for family in ensure_list(row.get("consumes_notice_family_classes"), f"{surface}.consumes_notice_family_classes"):
            family = ensure_str(family, f"{surface}.consumes_notice_family_classes[]")
            if family not in notice_families:
                findings.append(Finding("error", "state_semantics.surface.unknown_notice_family", f"{surface} consumes unknown notice family {family}", "Use a published notice family.", surface))
    missing = REQUIRED_SURFACES - surfaces
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.surface.required_missing",
                message="registry is missing required surface consumers",
                remediation="Add surface_consumers rows for each required surface.",
                details={"missing": sorted(missing)},
            )
        )
    if "extension_embedded_canvas" in surfaces:
        extension_row = next(
            row for row in rows if ensure_dict(row, "surface").get("surface_class") == "extension_embedded_canvas"
        )
        posture = ensure_str(extension_row.get("inheritance_posture"), "extension_embedded_canvas.inheritance_posture")
        if posture != "declares_inheritance_gap_when_not_aligned":
            findings.append(
                Finding(
                    severity="error",
                    check_id="state_semantics.surface.extension_gap_not_declared",
                    message="extension_embedded_canvas must declare inheritance gaps when not aligned",
                    remediation="Set inheritance_posture to declares_inheritance_gap_when_not_aligned.",
                    ref="extension_embedded_canvas",
                )
            )
    return surfaces


def validate_fixtures(
    repo_root: Path,
    manifest: dict[str, Any],
    state_classes: set[str],
    badge_families: set[str],
    notice_families: set[str],
    surfaces: set[str],
    findings: list[Finding],
) -> set[str]:
    if manifest.get("record_kind") != "state_semantics_fixture_manifest":
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.fixture_manifest.record_kind.invalid",
                message="fixture manifest record_kind must be state_semantics_fixture_manifest",
                remediation="Restore the fixture manifest record kind.",
            )
        )
    cases = ensure_list(manifest.get("fixture_cases"), "fixture_manifest.fixture_cases")
    case_ids: set[str] = set()
    acceptance_states: set[str] = set()
    for raw_case in cases:
        case = ensure_dict(raw_case, "fixture_manifest.fixture_cases[]")
        case_id = ensure_str(case.get("case_id"), "fixture.case_id")
        case_ids.add(case_id)
        fixture_ref = ensure_str(case.get("fixture_ref"), f"{case_id}.fixture_ref")
        validate_path_ref(repo_root, fixture_ref, "fixture.fixture_ref", findings)
        surface = ensure_str(case.get("surface_class"), f"{case_id}.surface_class")
        if surface not in surfaces:
            findings.append(Finding("error", "state_semantics.fixture.unknown_surface", f"{case_id} references unknown surface {surface}", "Use a surface published by surface_consumers.", case_id))
        for state in ensure_list(case.get("exercises_component_state_classes"), f"{case_id}.exercises_component_state_classes"):
            state = ensure_str(state, f"{case_id}.exercises_component_state_classes[]")
            if state not in state_classes:
                findings.append(Finding("error", "state_semantics.fixture.unknown_state", f"{case_id} references unknown state {state}", "Use a published component state class.", case_id))
        for family in ensure_list(case.get("exercises_badge_family_classes"), f"{case_id}.exercises_badge_family_classes"):
            family = ensure_str(family, f"{case_id}.exercises_badge_family_classes[]")
            if family not in badge_families:
                findings.append(Finding("error", "state_semantics.fixture.unknown_badge_family", f"{case_id} references unknown badge family {family}", "Use a published badge family.", case_id))
        for family in ensure_list(case.get("exercises_notice_family_classes"), f"{case_id}.exercises_notice_family_classes"):
            family = ensure_str(family, f"{case_id}.exercises_notice_family_classes[]")
            if family not in notice_families:
                findings.append(Finding("error", "state_semantics.fixture.unknown_notice_family", f"{case_id} references unknown notice family {family}", "Use a published notice family.", case_id))
        fixture_payload = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(fixture_ref)), fixture_ref)
        validate_fixture_payload(fixture_payload, case_id, state_classes, badge_families, notice_families, findings)
        acceptance_states.update(acceptance_for_case(case_id))
    missing_cases = REQUIRED_CASES - case_ids
    if missing_cases:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.fixture.required_case_missing",
                message="fixture manifest is missing required cases",
                remediation="Add fixture cases for each protected state semantics path.",
                details={"missing": sorted(missing_cases)},
            )
        )
    missing_states = REQUIRED_ACCEPTANCE_STATES - acceptance_states
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="state_semantics.fixture.acceptance_state_missing",
                message="fixtures do not cover all required acceptance states",
                remediation="Add fixture coverage for loading/pending distinction, trust/policy non-hue cues, cross-surface badge families, and inheritance gaps.",
                details={"missing": sorted(missing_states)},
            )
        )
    return case_ids


def acceptance_for_case(case_id: str) -> set[str]:
    mapping = {
        "command_palette_loading_pending_distinct": {"loading_pending_distinct"},
        "shell_editor_degraded_blocked_trust_policy": {"trust_policy_non_hue_only"},
        "docs_help_badge_projection": {"badge_family_cross_surface"},
        "package_marketplace_support_policy_badges": {"badge_family_cross_surface", "trust_policy_non_hue_only"},
        "trust_notice_families": {"trust_policy_non_hue_only"},
        "extension_embedded_inheritance_gap": {"embedded_inheritance_gap"},
    }
    return mapping.get(case_id, set())


def validate_fixture_payload(
    fixture: dict[str, Any],
    case_id: str,
    state_classes: set[str],
    badge_families: set[str],
    notice_families: set[str],
    findings: list[Finding],
) -> None:
    if fixture.get("record_kind") != "state_semantics_fixture_case":
        findings.append(Finding("error", "state_semantics.fixture.record_kind.invalid", f"{case_id} fixture has invalid record_kind", "Restore record_kind to state_semantics_fixture_case.", case_id))
    for raw_state in ensure_list(fixture.get("states"), f"{case_id}.states"):
        state = ensure_dict(raw_state, f"{case_id}.states[]")
        state_class = ensure_str(state.get("state_class"), f"{case_id}.states[].state_class")
        if state_class not in state_classes:
            findings.append(Finding("error", "state_semantics.fixture_payload.unknown_state", f"{case_id} references unknown state {state_class}", "Use a published component state class.", case_id))
        if not ensure_list(state.get("required_visible_cues"), f"{case_id}.{state_class}.required_visible_cues"):
            findings.append(Finding("error", "state_semantics.fixture_payload.visible_cues_missing", f"{case_id}.{state_class} has no required visible cues", "Add persistent non-color cues.", case_id))
    for raw_badge in ensure_list(fixture.get("badge_tokens"), f"{case_id}.badge_tokens"):
        badge = ensure_dict(raw_badge, f"{case_id}.badge_tokens[]")
        family = ensure_str(badge.get("family"), f"{case_id}.badge_tokens[].family")
        if family not in badge_families:
            findings.append(Finding("error", "state_semantics.fixture_payload.unknown_badge_family", f"{case_id} references unknown badge family {family}", "Use a published badge family.", case_id))
        ensure_str(badge.get("token"), f"{case_id}.{family}.token")
    for raw_notice in ensure_list(fixture.get("notice_tokens"), f"{case_id}.notice_tokens"):
        notice = ensure_dict(raw_notice, f"{case_id}.notice_tokens[]")
        family = ensure_str(notice.get("family"), f"{case_id}.notice_tokens[].family")
        if family not in notice_families:
            findings.append(Finding("error", "state_semantics.fixture_payload.unknown_notice_family", f"{case_id} references unknown notice family {family}", "Use a published notice family.", case_id))


def write_report(path: Path, registry_rel: str, case_ids: set[str], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "registry_ref": registry_rel,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_fixture_cases": sorted(case_ids),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    registry_rel = str(args.registry)
    registry = ensure_dict(render_yaml_as_json(repo_root / registry_rel), "registry")
    manifest = ensure_dict(render_yaml_as_json(repo_root / args.fixture_manifest), "fixture_manifest")
    known_tokens = collect_token_names(repo_root)

    findings: list[Finding] = []
    validate_header(repo_root, registry, findings)
    state_classes = validate_component_states(registry, known_tokens, findings)
    badge_families = validate_badge_families(registry, known_tokens, state_classes, findings)
    notice_families = validate_notice_families(registry, known_tokens, state_classes, findings)
    surfaces = validate_surface_consumers(
        repo_root, registry, state_classes, badge_families, notice_families, findings
    )
    case_ids = validate_fixtures(
        repo_root, manifest, state_classes, badge_families, notice_families, surfaces, findings
    )

    if args.report:
        write_report(repo_root / args.report, registry_rel, case_ids, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[state-semantics] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[state-semantics] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[state-semantics]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[state-semantics] interrupted", file=sys.stderr)
        sys.exit(130)
