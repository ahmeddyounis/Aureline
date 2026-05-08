#!/usr/bin/env python3
"""Standalone entry point for schema/example drift validation."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
CI_DIR = SCRIPT_DIR / "ci"
if str(CI_DIR) not in sys.path:
    sys.path.insert(0, str(CI_DIR))

from schema_example_drift_validation import render_human_summary, validate_schema_example_drift  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--source-map",
        default=None,
        help="Override the default source-map path (repo-relative).",
    )
    parser.add_argument(
        "--scenario",
        default=None,
        help="Optional YAML/JSON scenario that overrides changed_files and/or file contents for a deterministic failing example.",
    )
    return parser.parse_args()


def load_scenario(repo_root: Path, scenario: str | None) -> dict | None:
    if scenario is None:
        return None
    scenario_path = Path(scenario)
    path = scenario_path if scenario_path.is_absolute() else repo_root / scenario_path
    if not path.exists():
        raise SystemExit(f"scenario file does not exist: {path}")

    if path.suffix.lower() == ".json":
        return json.loads(path.read_text(encoding="utf-8"))

    try:
        import yaml  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(f"python PyYAML is required to parse scenario YAML: {exc}") from exc
    payload = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise SystemExit(f"scenario file must contain a mapping/object: {path}")
    return payload


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    scenario = load_scenario(repo_root, args.scenario)

    findings, analysis = validate_schema_example_drift(
        repo_root,
        source_map_path=args.source_map,
        scenario=scenario,
    )
    sys.stdout.write(render_human_summary(findings, analysis))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())

