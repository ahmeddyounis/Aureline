#!/usr/bin/env python3
"""Unattended token / state / reduced-motion audit for protected M1 surfaces.

Walks every fixture row in ``fixtures/ux/reduced_motion_cases/*.yaml`` and,
for the protected M1 shell, Start Center, search palette, and trust surfaces,
verifies that:

- the canonical source files still exist;
- every required ``require_*`` token id still appears as a literal call in
  the union of the surface's source files (so theme/density/contrast
  switching keeps reaching the surface);
- every required ``ComponentStates::*`` symbol still appears in the union of
  surface sources (so focus-visible, selected, hover, etc. keep mapping to
  one shared visual treatment instead of color-only or per-surface styling);
- every referenced motion-preset fixture (``fixtures/design/motion_cases/*``)
  still exists and declares a non-empty ``reduced_motion_fallbacks`` list
  with ``preserves_state_conveyance: true`` and at least one fallback that
  preserves focus visibility, so reduced-motion postures keep getting a
  state-preserving substitute instead of disappearing motion;
- the surface's source files do not match any of the prohibited literal
  patterns (e.g. raw ``ColorRgba { r: ... }`` constructions, hand-coded
  ``Duration::from_millis(...)`` durations) that would bypass the shared
  token / motion contracts.

The runner emits a durable, machine-readable JSON capture
(``--report``) and exits non-zero if any protected walk regresses. A
``--force-drill`` mode replays the named failure drill from a fixture
(dropping a required token, state symbol, or motion-preset ref from the
"observed" set) and asserts the audit reports the expected ``check_id``
for that drill — proving the lane fails loudly on real regressions
instead of silently passing on a partial walk.

YAML decoding follows the existing repo convention: parse via Ruby/Psych
(already required by other CI checks) so this runner stays free of
Python-side YAML dependencies.
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


DEFAULT_CASES_DIR_REL = "fixtures/ux/reduced_motion_cases"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

REQUIRE_KINDS = (
    "require_color",
    "require_motion_ms",
    "require_space_px",
    "require_size_px",
    "require_radius_px",
    "require_stroke_px",
)

REQUIRED_SURFACE_COVERAGE = {
    "shell_chrome",
    "start_center",
    "search_palette",
    "trust_surface",
}

CHECK_ID_REQUIRED_TOKEN_MISSING = "token_state_audit.required_token.missing"
CHECK_ID_REQUIRED_STATE_MISSING = "token_state_audit.required_state.missing"
CHECK_ID_REQUIRED_MOTION_PRESET_MISSING = (
    "token_state_audit.required_motion_preset.missing"
)
CHECK_ID_MOTION_PRESET_INVALID = "token_state_audit.required_motion_preset.invalid"
CHECK_ID_MOTION_MODULE_MISSING = "token_state_audit.required_motion_module.missing"
CHECK_ID_PROHIBITED_PATTERN = "token_state_audit.prohibited_pattern.matched"
CHECK_ID_SOURCE_MISSING = "token_state_audit.source.missing"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    surface_id: str | None = None
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        for key in ("ref", "surface_id"):
            if payload[key] is None:
                payload.pop(key)
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--cases-dir",
        default=DEFAULT_CASES_DIR_REL,
        help="Directory holding token_state_audit_case_record YAML files.",
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
            "Replay the named failure drill (drill_id) from one of the case "
            "files. The runner injects the forced input and verifies the "
            "expected check_id is reported, then exits 0 only if the drill "
            "reproduced exactly as declared."
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
    return value.strip()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


@dataclass(frozen=True)
class ProhibitedPattern:
    pattern_id: str
    description: str
    pattern: re.Pattern[str]


@dataclass
class CaseRecord:
    case_id: str
    surface_id: str
    surface_family: str
    sources: list[str]
    required_token_calls: dict[str, list[str]]
    required_state_symbols: list[str]
    required_motion_preset_refs: list[str]
    required_motion_module_ref: str
    prohibited_patterns: list[ProhibitedPattern]
    failure_drill: dict[str, Any]
    case_path: str


def load_case(path: Path, repo_root: Path) -> CaseRecord:
    raw = ensure_dict(render_yaml_as_json(path), str(path.relative_to(repo_root)))
    if raw.get("record_kind") != "token_state_audit_case_record":
        raise SystemExit(f"{path}: record_kind must be token_state_audit_case_record")
    schema_version = raw.get("schema_version")
    if schema_version != 1:
        raise SystemExit(f"{path}: schema_version must be 1, got {schema_version!r}")
    case_id = ensure_str(raw.get("case_id"), f"{path}.case_id")
    surface_id = ensure_str(raw.get("surface_id"), f"{path}.surface_id")
    surface_family = ensure_str(raw.get("surface_family"), f"{path}.surface_family")

    sources_raw = ensure_list(raw.get("sources"), f"{path}.sources")
    sources = [ensure_str(item, f"{path}.sources[]") for item in sources_raw]
    if not sources:
        raise SystemExit(f"{path}: sources must declare at least one repo path")

    required_token_calls_raw = ensure_dict(
        raw.get("required_token_calls", {}), f"{path}.required_token_calls"
    )
    required_token_calls: dict[str, list[str]] = {}
    for kind, value in required_token_calls_raw.items():
        if kind not in REQUIRE_KINDS:
            raise SystemExit(
                f"{path}: required_token_calls.{kind} is not a known require_* kind"
            )
        items = ensure_list(value, f"{path}.required_token_calls.{kind}")
        required_token_calls[kind] = [
            ensure_str(item, f"{path}.required_token_calls.{kind}[]") for item in items
        ]

    required_state_symbols_raw = raw.get("required_state_symbols", [])
    required_state_symbols = [
        ensure_str(item, f"{path}.required_state_symbols[]")
        for item in ensure_list(required_state_symbols_raw, f"{path}.required_state_symbols")
    ]

    required_motion_preset_refs_raw = raw.get("required_motion_preset_refs", [])
    required_motion_preset_refs = [
        ensure_str(item, f"{path}.required_motion_preset_refs[]")
        for item in ensure_list(
            required_motion_preset_refs_raw, f"{path}.required_motion_preset_refs"
        )
    ]

    required_motion_module_ref = ensure_str(
        raw.get("required_motion_module_ref"),
        f"{path}.required_motion_module_ref",
    )

    prohibited_raw = ensure_list(
        raw.get("prohibited_patterns", []), f"{path}.prohibited_patterns"
    )
    prohibited_patterns: list[ProhibitedPattern] = []
    for idx, row in enumerate(prohibited_raw):
        row = ensure_dict(row, f"{path}.prohibited_patterns[{idx}]")
        pattern_id = ensure_str(
            row.get("id"), f"{path}.prohibited_patterns[{idx}].id"
        )
        description = ensure_str(
            row.get("description"),
            f"{path}.prohibited_patterns[{idx}].description",
        )
        pattern_str = ensure_str(
            row.get("pattern"), f"{path}.prohibited_patterns[{idx}].pattern"
        )
        try:
            compiled = re.compile(pattern_str)
        except re.error as exc:
            raise SystemExit(
                f"{path}.prohibited_patterns[{idx}].pattern is not a valid regex: {exc}"
            ) from exc
        prohibited_patterns.append(
            ProhibitedPattern(pattern_id=pattern_id, description=description, pattern=compiled)
        )

    failure_drill = ensure_dict(raw.get("failure_drill"), f"{path}.failure_drill")
    ensure_str(failure_drill.get("drill_id"), f"{path}.failure_drill.drill_id")
    ensure_dict(failure_drill.get("forced_input"), f"{path}.failure_drill.forced_input")
    ensure_list(
        failure_drill.get("expected_findings"),
        f"{path}.failure_drill.expected_findings",
    )

    return CaseRecord(
        case_id=case_id,
        surface_id=surface_id,
        surface_family=surface_family,
        sources=sources,
        required_token_calls=required_token_calls,
        required_state_symbols=required_state_symbols,
        required_motion_preset_refs=required_motion_preset_refs,
        required_motion_module_ref=required_motion_module_ref,
        prohibited_patterns=prohibited_patterns,
        failure_drill=failure_drill,
        case_path=str(path.relative_to(repo_root)),
    )


def read_text(path: Path) -> str:
    with path.open("r", encoding="utf-8") as fh:
        return fh.read()


def collect_observed_token_calls(
    repo_root: Path, sources: list[str], findings: list[Finding], surface_id: str
) -> dict[str, set[str]]:
    observed: dict[str, set[str]] = {kind: set() for kind in REQUIRE_KINDS}
    for rel in sources:
        path = repo_root / rel
        if not path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_SOURCE_MISSING,
                    message=f"surface source does not exist: {rel}",
                    remediation="Restore the source path or update the case fixture if the surface moved.",
                    surface_id=surface_id,
                    ref=rel,
                )
            )
            continue
        text = read_text(path)
        for kind in REQUIRE_KINDS:
            pattern = re.compile(rf'{kind}\(\s*"([^"]+)"\s*\)')
            for match in pattern.findall(text):
                observed[kind].add(match)
    return observed


def collect_observed_state_symbols(
    repo_root: Path, sources: list[str], symbols: list[str]
) -> set[str]:
    observed: set[str] = set()
    if not symbols:
        return observed
    for rel in sources:
        path = repo_root / rel
        if not path.exists():
            continue
        text = read_text(path)
        for symbol in symbols:
            literal = re.escape(symbol)
            if re.search(rf'(?<![A-Za-z0-9_]){literal}(?![A-Za-z0-9_])', text):
                observed.add(symbol)
    return observed


def collect_motion_preset_module_reuse(
    repo_root: Path, sources: list[str], required_motion_module_ref: str
) -> bool:
    """Return True if at least one source still pulls from the shared motion module."""
    target = required_motion_module_ref.split("/")
    if len(target) < 2:
        return False
    expected_crate_path = "::".join(target[1:]).replace(".rs", "").replace("/src/", "::")
    # We accept either a `use aureline_ui::motion::` import or a direct
    # `aureline_ui::motion::` qualified reference. The shell renderer uses
    # the first form today.
    indicators = (
        "aureline_ui::motion::",
        "use aureline_ui::motion",
        "crate::motion::",
        "super::motion::",
        "self::motion::",
    )
    for rel in sources:
        path = repo_root / rel
        if not path.exists():
            continue
        text = read_text(path)
        if any(token in text for token in indicators):
            return True
    return False


def validate_motion_preset_fixture(
    repo_root: Path, ref: str
) -> tuple[bool, str | None, dict[str, Any]]:
    """Returns (ok, error_message, summary_dict)."""
    path = repo_root / ref
    if not path.exists():
        return False, f"motion preset fixture does not exist: {ref}", {}
    payload = render_yaml_as_json(path)
    if not isinstance(payload, dict):
        return False, f"{ref}: expected mapping", {}
    fallbacks_raw = payload.get("reduced_motion_fallbacks")
    if not isinstance(fallbacks_raw, list) or not fallbacks_raw:
        return (
            False,
            f"{ref}: reduced_motion_fallbacks must be a non-empty list",
            {},
        )
    has_state_preserving = False
    has_focus_preserving = False
    postures: list[str] = []
    for fallback in fallbacks_raw:
        if not isinstance(fallback, dict):
            continue
        posture = fallback.get("posture_class")
        if isinstance(posture, str):
            postures.append(posture)
        if fallback.get("preserves_state_conveyance") is True:
            has_state_preserving = True
        if fallback.get("preserves_focus_visibility") is True:
            has_focus_preserving = True
    summary = {
        "postures": sorted(set(postures)),
        "preserves_state_conveyance": has_state_preserving,
        "preserves_focus_visibility": has_focus_preserving,
    }
    if not has_state_preserving:
        return (
            False,
            (
                f"{ref}: at least one reduced_motion_fallbacks entry must declare "
                "preserves_state_conveyance: true"
            ),
            summary,
        )
    if not has_focus_preserving:
        return (
            False,
            (
                f"{ref}: at least one reduced_motion_fallbacks entry must declare "
                "preserves_focus_visibility: true"
            ),
            summary,
        )
    return True, None, summary


def scan_prohibited_patterns(
    repo_root: Path,
    sources: list[str],
    patterns: list[ProhibitedPattern],
    findings: list[Finding],
    surface_id: str,
) -> None:
    if not patterns:
        return
    for rel in sources:
        path = repo_root / rel
        if not path.exists():
            continue
        text = read_text(path)
        for pattern in patterns:
            for line_idx, line in enumerate(text.splitlines(), start=1):
                if pattern.pattern.search(line):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id=CHECK_ID_PROHIBITED_PATTERN,
                            message=(
                                f"{rel}:{line_idx} matches prohibited pattern "
                                f"'{pattern.pattern_id}': {pattern.description}"
                            ),
                            remediation=(
                                "Replace the literal with a TokenRegistry-backed "
                                "lookup or a shared motion-preset duration so the "
                                "shared design-system contract still reaches this "
                                "surface."
                            ),
                            surface_id=surface_id,
                            ref=f"{rel}:{line_idx}",
                            details={
                                "pattern_id": pattern.pattern_id,
                                "match_line": line.strip(),
                            },
                        )
                    )


def apply_forced_drop(
    drop_kind: str,
    forced_value: str,
    observed_tokens: dict[str, set[str]],
    observed_states: set[str],
    motion_preset_refs: list[str],
) -> tuple[dict[str, set[str]], set[str], list[str]]:
    """Apply the failure-drill forced input, returning the mutated observed sets."""
    new_tokens = {kind: set(values) for kind, values in observed_tokens.items()}
    new_states = set(observed_states)
    new_presets = list(motion_preset_refs)
    if drop_kind == "drop_required_token":
        kind = forced_value.get("kind")
        token = forced_value.get("token")
        if kind in new_tokens and isinstance(token, str):
            new_tokens[kind].discard(token)
    elif drop_kind == "drop_required_state_symbol":
        symbol = forced_value.get("symbol")
        if isinstance(symbol, str):
            new_states.discard(symbol)
    elif drop_kind == "drop_required_motion_preset_ref":
        ref = forced_value.get("ref")
        if isinstance(ref, str) and ref in new_presets:
            new_presets.remove(ref)
    else:
        raise SystemExit(f"unknown forced_input drop kind: {drop_kind}")
    return new_tokens, new_states, new_presets


def evaluate_case(
    case: CaseRecord,
    repo_root: Path,
    forced_drill: dict[str, Any] | None,
) -> tuple[list[Finding], dict[str, Any]]:
    findings: list[Finding] = []
    observed_tokens = collect_observed_token_calls(
        repo_root, case.sources, findings, case.surface_id
    )
    observed_states = collect_observed_state_symbols(
        repo_root, case.sources, case.required_state_symbols
    )
    motion_preset_refs = list(case.required_motion_preset_refs)

    forced_input_summary: dict[str, Any] = {}
    if forced_drill is not None:
        forced_input = ensure_dict(
            forced_drill.get("forced_input"),
            f"{case.case_id}.failure_drill.forced_input",
        )
        if not forced_input:
            raise SystemExit(
                f"{case.case_id}: failure_drill.forced_input must declare a drop_* directive"
            )
        drop_kind = next(iter(forced_input))
        forced_input_summary = {
            "drop_kind": drop_kind,
            "forced_value": forced_input[drop_kind],
        }
        observed_tokens, observed_states, motion_preset_refs = apply_forced_drop(
            drop_kind, forced_input[drop_kind], observed_tokens, observed_states, motion_preset_refs
        )

    # 1) Required token-call presence.
    for kind, expected in case.required_token_calls.items():
        seen = observed_tokens.get(kind, set())
        for token in expected:
            if token not in seen:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_REQUIRED_TOKEN_MISSING,
                        message=(
                            f"surface '{case.surface_id}' is missing a "
                            f"required {kind}(\"{token}\") call across "
                            f"{case.sources}"
                        ),
                        remediation=(
                            "Restore the TokenRegistry::"
                            f"{kind}(\"{token}\") call so theme/contrast/"
                            "density switching keeps reaching the surface."
                        ),
                        surface_id=case.surface_id,
                        ref=token,
                        details={"kind": kind, "token": token, "sources": case.sources},
                    )
                )

    # 2) Required component-state symbols.
    for symbol in case.required_state_symbols:
        if symbol not in observed_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_REQUIRED_STATE_MISSING,
                    message=(
                        f"surface '{case.surface_id}' is missing the "
                        f"required component-state symbol '{symbol}' "
                        f"across {case.sources}"
                    ),
                    remediation=(
                        "Reuse the shared aureline_ui::components state "
                        "vocabulary so focus/selection/hover keep mapping "
                        "to one shared visual treatment."
                    ),
                    surface_id=case.surface_id,
                    ref=symbol,
                    details={"symbol": symbol, "sources": case.sources},
                )
            )

    # 3) Required motion module re-use.
    if not collect_motion_preset_module_reuse(
        repo_root, case.sources, case.required_motion_module_ref
    ):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_MOTION_MODULE_MISSING,
                message=(
                    f"surface '{case.surface_id}' no longer pulls from the "
                    f"shared motion module ({case.required_motion_module_ref})"
                ),
                remediation=(
                    "Re-import aureline_ui::motion (or the equivalent "
                    "shared motion entry) so reduced-motion fallbacks "
                    "reach the surface."
                ),
                surface_id=case.surface_id,
                ref=case.required_motion_module_ref,
            )
        )

    # 4) Required motion-preset fixtures (and their fallback declarations).
    motion_summaries: dict[str, Any] = {}
    for ref in motion_preset_refs:
        ok, error, summary = validate_motion_preset_fixture(repo_root, ref)
        motion_summaries[ref] = summary
        if not ok:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_MOTION_PRESET_INVALID,
                    message=error or f"{ref}: motion preset fixture invalid",
                    remediation=(
                        "Restore the reduced_motion_fallbacks block on the "
                        "preset fixture so the surface still has a "
                        "state-preserving substitute under reduced motion."
                    ),
                    surface_id=case.surface_id,
                    ref=ref,
                )
            )
    for required_ref in case.required_motion_preset_refs:
        if required_ref not in motion_preset_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_REQUIRED_MOTION_PRESET_MISSING,
                    message=(
                        f"surface '{case.surface_id}' is missing required "
                        f"motion preset reference: {required_ref}"
                    ),
                    remediation=(
                        "Restore the motion-preset reference so the surface "
                        "keeps consulting the shared reduced-motion contract."
                    ),
                    surface_id=case.surface_id,
                    ref=required_ref,
                )
            )

    # 5) Prohibited literal patterns in surface source.
    scan_prohibited_patterns(
        repo_root, case.sources, case.prohibited_patterns, findings, case.surface_id
    )

    diagnostics = {
        "observed_token_calls": {kind: sorted(values) for kind, values in observed_tokens.items()},
        "observed_state_symbols": sorted(observed_states),
        "motion_preset_summaries": motion_summaries,
        "motion_module_ref": case.required_motion_module_ref,
        "forced_input": forced_input_summary,
    }
    return findings, diagnostics


def expected_findings_match(
    findings: list[Finding], expected_rows: list[dict[str, Any]]
) -> tuple[bool, list[str]]:
    """Returns (ok, missing_descriptions)."""
    missing: list[str] = []
    for expected in expected_rows:
        check_id = expected.get("check_id") if isinstance(expected, dict) else None
        message_contains = (
            expected.get("message_contains") if isinstance(expected, dict) else None
        )
        matched = False
        for finding in findings:
            if finding.check_id != check_id:
                continue
            if isinstance(message_contains, str) and message_contains:
                if message_contains not in finding.message:
                    continue
            matched = True
            break
        if not matched:
            missing.append(
                f"check_id={check_id!r}, message_contains={message_contains!r}"
            )
    return len(missing) == 0, missing


def find_drill_case(cases: list[CaseRecord], drill_id: str) -> CaseRecord | None:
    for case in cases:
        if case.failure_drill.get("drill_id") == drill_id:
            return case
    return None


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    cases_dir = repo_root / args.cases_dir
    if not cases_dir.is_dir():
        raise SystemExit(f"cases dir not found: {args.cases_dir}")

    case_paths = sorted(p for p in cases_dir.glob("*.yaml"))
    if not case_paths:
        raise SystemExit(f"no token_state_audit cases found in {args.cases_dir}")

    cases = [load_case(p, repo_root) for p in case_paths]

    seen_ids: set[str] = set()
    seen_surfaces: set[str] = set()
    for case in cases:
        if case.case_id in seen_ids:
            raise SystemExit(f"duplicate case_id: {case.case_id}")
        seen_ids.add(case.case_id)
        seen_surfaces.add(case.surface_id)

    coverage_findings: list[Finding] = []
    missing_surfaces = REQUIRED_SURFACE_COVERAGE - seen_surfaces
    if missing_surfaces:
        coverage_findings.append(
            Finding(
                severity="error",
                check_id="token_state_audit.coverage.missing_required_surfaces",
                message=(
                    "cases must seed at least one row each for "
                    f"{sorted(REQUIRED_SURFACE_COVERAGE)}; missing: "
                    f"{sorted(missing_surfaces)}"
                ),
                remediation=(
                    "Add the missing surface case so the protected walk covers "
                    "shell, Start Center, search, and trust surfaces."
                ),
            )
        )

    drill_mode = args.force_drill is not None
    drill_case: CaseRecord | None = None
    if drill_mode:
        drill_case = find_drill_case(cases, args.force_drill)
        if drill_case is None:
            raise SystemExit(
                f"--force-drill: no case declares drill_id '{args.force_drill}'"
            )

    case_results: list[dict[str, Any]] = []
    all_findings: list[Finding] = list(coverage_findings)

    for case in cases:
        is_drill_case = drill_mode and drill_case is not None and case.case_id == drill_case.case_id
        forced_drill = case.failure_drill if is_drill_case else None
        findings, diagnostics = evaluate_case(case, repo_root, forced_drill)
        passed_checks = sum(
            1
            for kind, expected in case.required_token_calls.items()
            for token in expected
        ) - sum(
            1 for f in findings if f.check_id == CHECK_ID_REQUIRED_TOKEN_MISSING
        )
        case_record: dict[str, Any] = {
            "case_id": case.case_id,
            "surface_id": case.surface_id,
            "surface_family": case.surface_family,
            "case_path": case.case_path,
            "sources": case.sources,
            "diagnostics": diagnostics,
            "passed_required_token_checks": max(passed_checks, 0),
            "finding_count": len(findings),
            "findings": [f.as_report() for f in findings],
        }
        if is_drill_case:
            expected_rows = case.failure_drill.get("expected_findings", [])
            ok, missing = expected_findings_match(findings, expected_rows)
            case_record["failure_drill"] = {
                "drill_id": case.failure_drill.get("drill_id"),
                "expected_findings": expected_rows,
                "expected_findings_observed": ok,
                "missing_expected_findings": missing,
            }
            if not ok:
                all_findings.append(
                    Finding(
                        severity="error",
                        check_id="token_state_audit.failure_drill.expected_finding_missing",
                        message=(
                            f"failure drill {case.failure_drill.get('drill_id')!r} did "
                            "not surface the expected findings: "
                            f"{missing}"
                        ),
                        remediation=(
                            "Either restore the audit logic to detect this "
                            "violation, or update the case fixture if the "
                            "regression class genuinely changed."
                        ),
                        surface_id=case.surface_id,
                        ref=case.failure_drill.get("drill_id"),
                    )
                )
        else:
            all_findings.extend(findings)
        case_results.append(case_record)

    errors = [f for f in all_findings if f.severity == "error"]
    if drill_mode:
        # In drill mode we EXPECT the drill case to fail loudly (those
        # findings are the proof). The lane is healthy iff:
        #   - the drill case surfaced exactly the expected findings, and
        #   - no other case regressed under the drill mutation.
        non_drill_failures = [
            f for f in errors
            if f.check_id == "token_state_audit.failure_drill.expected_finding_missing"
        ]
        status = "PASS" if not non_drill_failures else "FAIL"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "token_motion_audit_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": "@ahmeddyounis",
        "cases_dir_ref": args.cases_dir,
        "case_count": len(cases),
        "drill_mode": drill_mode,
        "drill_id": args.force_drill,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/ux/token_state_audit/run_token_state_audit.py "
            "--repo-root ."
            + (f" --force-drill {args.force_drill}" if drill_mode else "")
        ),
        "required_surface_coverage": sorted(REQUIRED_SURFACE_COVERAGE),
        "observed_surface_ids": sorted(seen_surfaces),
        "status": status,
        "cases": case_results,
        "finding_counts": {
            "error": sum(1 for f in all_findings if f.severity == "error"),
            "warning": sum(1 for f in all_findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in all_findings],
    }

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(
        f"[token-state-audit] {status} "
        f"({len(cases)} cases, {len(errors)} errors, "
        f"{sum(1 for f in all_findings if f.severity == 'warning')} warnings) — "
        f"capture: {args.report}"
    )
    for finding in all_findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        surface_suffix = (
            f" {{{finding.surface_id}}}" if finding.surface_id else ""
        )
        print(
            f"[token-state-audit] {prefix}{surface_suffix} "
            f"{finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(f"[token-state-audit]   remediation: {finding.remediation}")

    return 0 if status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[token-state-audit] interrupted", file=sys.stderr)
        sys.exit(130)
