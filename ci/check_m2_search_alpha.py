#!/usr/bin/env python3
"""Validate the external alpha search review packet and fixtures."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REVIEW_REL = "docs/review/m2_search_alpha_review.md"
DEFAULT_PACKET_REL = "artifacts/milestones/m2/search_alpha_validation.md"
DEFAULT_KEYBOARD_FIXTURE_REL = "fixtures/accessibility/m2_search_keyboard/search_keyboard_parity.yaml"
DEFAULT_RUNTIME_REL = "crates/aureline-shell/src/search/alpha_validation.rs"
DEFAULT_TEST_REL = "crates/aureline-shell/tests/search_alpha_validation.rs"
DEFAULT_KNOWN_LIMITS_REL = "artifacts/milestones/m2/known_limits_alpha.yaml"
DEFAULT_KNOWN_LIMITS_MD_REL = "artifacts/feedback/external_alpha_known_limits.md"

KNOWN_LIMIT_ID = "known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only"

REQUIRED_RUNTIME_MARKERS = {
    "SearchAlphaValidationPacket",
    "materialize_search_alpha_validation_packet",
    "passes_acceptance",
    "SearchAlphaKeyboardReview",
    "SearchAlphaDiscoverabilityReview",
}

REQUIRED_REVIEW_MARKERS = {
    "quick open",
    "symbol search",
    "Why this result?",
    "fixtures/search/ranking_reason_alpha/",
    "fixtures/accessibility/m2_search_keyboard/search_keyboard_parity.yaml",
    KNOWN_LIMIT_ID,
}

REQUIRED_PACKET_MARKERS = {
    "accepted_with_known_limits",
    "crates/aureline-shell/src/search/alpha_validation.rs",
    "crates/aureline-shell/tests/search_alpha_validation.rs",
    "ci/check_m2_search_alpha.py",
    KNOWN_LIMIT_ID,
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
    parser.add_argument("--review", default=DEFAULT_REVIEW_REL)
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--keyboard-fixture", default=DEFAULT_KEYBOARD_FIXTURE_REL)
    parser.add_argument("--runtime", default=DEFAULT_RUNTIME_REL)
    parser.add_argument("--test", default=DEFAULT_TEST_REL)
    parser.add_argument("--known-limits", default=DEFAULT_KNOWN_LIMITS_REL)
    parser.add_argument("--known-limits-md", default=DEFAULT_KNOWN_LIMITS_MD_REL)
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


def ensure_path(repo_root: Path, rel: str, label: str, findings: list[Finding]) -> Path:
    path = repo_root / rel
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing",
                message=f"{label} does not exist: {rel}",
                remediation="Create the missing alpha search artifact.",
                ref=rel,
            )
        )
    return path


def check_markers(
    path: Path,
    rel: str,
    markers: set[str],
    label: str,
    findings: list[Finding],
) -> None:
    if not path.exists():
        return
    text = path.read_text(encoding="utf-8")
    missing = sorted(marker for marker in markers if marker not in text)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_markers",
                message=f"{label} is missing required search alpha markers.",
                remediation="Update the artifact so reviewers can trace search, ranking, keyboard, and known-limit evidence.",
                ref=rel,
                details={"missing": missing},
            )
        )


def validate_keyboard_fixture(repo_root: Path, rel: str, findings: list[Finding]) -> None:
    path = ensure_path(repo_root, rel, "keyboard_fixture", findings)
    if not path.exists():
        return
    fixture = render_yaml_as_json(path)
    if not isinstance(fixture, dict):
        raise SystemExit(f"{rel} must be a YAML mapping")

    if fixture.get("fixture_id") != "a11y.search_alpha.keyboard_parity":
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.fixture_id",
                message="search keyboard fixture has the wrong fixture_id.",
                remediation="Use the canonical a11y.search_alpha.keyboard_parity fixture id.",
                ref=rel,
            )
        )

    required_surfaces = set(fixture.get("required_surface_ids") or [])
    missing_surfaces = {"quick_open", "symbol_search"} - required_surfaces
    if missing_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.required_surfaces",
                message="search keyboard fixture is missing required surfaces.",
                remediation="Require both quick_open and symbol_search in the fixture.",
                ref=rel,
                details={"missing": sorted(missing_surfaces)},
            )
        )

    if fixture.get("required_keyboard_surface_id") != "palette.command_diagnostics":
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.keyboard_surface",
                message="fixture must cite the palette diagnostics keyboard route.",
                remediation="Point required_keyboard_surface_id at palette.command_diagnostics.",
                ref=rel,
            )
        )

    if fixture.get("required_known_limit_ref") != KNOWN_LIMIT_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.known_limit",
                message="fixture does not cite the search alpha known limit.",
                remediation="Attach the canonical known-limit id to the fixture.",
                ref=rel,
            )
        )

    acceptance = fixture.get("acceptance")
    if not isinstance(acceptance, dict):
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.acceptance",
                message="fixture must carry an acceptance mapping.",
                remediation="Add acceptance booleans and discoverability row-kind requirements.",
                ref=rel,
            )
        )
        return

    for key in (
        "result_ids_reused_from_card_contract",
        "ranking_reason_cards_keyboard_reachable",
        "planner_result_id_contract_required",
    ):
        if acceptance.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"keyboard_fixture.acceptance.{key}",
                    message=f"acceptance.{key} must be true.",
                    remediation="Set the acceptance field to true only after the Rust packet consumes that path.",
                    ref=rel,
                )
            )

    row_kinds = set(acceptance.get("discoverability_row_kinds_required") or [])
    missing_kinds = {"command", "symbol", "file"} - row_kinds
    if missing_kinds:
        findings.append(
            Finding(
                severity="error",
                check_id="keyboard_fixture.discoverability_row_kinds",
                message="fixture must require command, symbol, and file discoverability rows.",
                remediation="Add the missing row-kind requirements.",
                ref=rel,
                details={"missing": sorted(missing_kinds)},
            )
        )


def validate_known_limits(
    repo_root: Path,
    yaml_rel: str,
    markdown_rel: str,
    findings: list[Finding],
) -> None:
    yaml_path = ensure_path(repo_root, yaml_rel, "known_limits_yaml", findings)
    md_path = ensure_path(repo_root, markdown_rel, "known_limits_markdown", findings)

    if yaml_path.exists():
        payload = render_yaml_as_json(yaml_path)
        rows = payload.get("known_limits") if isinstance(payload, dict) else None
        ids = {
            row.get("known_limit_id")
            for row in rows or []
            if isinstance(row, dict)
        }
        if KNOWN_LIMIT_ID not in ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="known_limits_yaml.search_limit_missing",
                    message="machine-readable known-limits packet lacks the search alpha limit.",
                    remediation="Add the search alpha known-limit row.",
                    ref=yaml_rel,
                )
            )

    if md_path.exists() and KNOWN_LIMIT_ID not in md_path.read_text(encoding="utf-8"):
        findings.append(
            Finding(
                severity="error",
                check_id="known_limits_markdown.search_limit_missing",
                message="markdown known-limits packet lacks the search alpha limit.",
                remediation="Add the same known-limit id to the markdown companion.",
                ref=markdown_rel,
            )
        )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "record_kind": "search_alpha_validation_capture",
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "known_limit_ref": KNOWN_LIMIT_ID,
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
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    review_path = ensure_path(repo_root, args.review, "review_packet", findings)
    packet_path = ensure_path(repo_root, args.packet, "validation_packet", findings)
    runtime_path = ensure_path(repo_root, args.runtime, "runtime_consumer", findings)
    test_path = ensure_path(repo_root, args.test, "runtime_test", findings)

    check_markers(review_path, args.review, REQUIRED_REVIEW_MARKERS, "review_packet", findings)
    check_markers(packet_path, args.packet, REQUIRED_PACKET_MARKERS, "validation_packet", findings)
    check_markers(runtime_path, args.runtime, REQUIRED_RUNTIME_MARKERS, "runtime_consumer", findings)
    check_markers(
        test_path,
        args.test,
        {"search_alpha_validation_combines_ranking_discoverability_and_keyboard_routes", KNOWN_LIMIT_ID},
        "runtime_test",
        findings,
    )
    validate_keyboard_fixture(repo_root, args.keyboard_fixture, findings)
    validate_known_limits(repo_root, args.known_limits, args.known_limits_md, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[search-alpha] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[search-alpha] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[search-alpha]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[search-alpha] interrupted", file=sys.stderr)
        sys.exit(130)
