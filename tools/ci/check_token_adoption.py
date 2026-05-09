#!/usr/bin/env python3
"""Token-adoption audit for protected appearance surfaces.

This check is intentionally narrow: it ensures the protected shell surfaces keep
loading styling primitives from the shared token registries, rather than
silently swapping to surface-local literals.

The baseline is stored under `tests/golden/appearance/**` so that:
  - adding/removing required tokens produces a localized diff; and
  - drift is surfaced in CI before it becomes late-stage visual regressions.
"""

from __future__ import annotations

import argparse
import difflib
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

REPO_ROOT = Path(__file__).resolve().parents[2]


class AuditError(RuntimeError):
    """Raised when the audit fails."""


@dataclass(frozen=True)
class SurfaceBaseline:
    record_kind: str
    schema_version: int
    surface_id: str
    sources: tuple[str, ...]
    require_calls: dict[str, tuple[str, ...]]


REQUIRE_PATTERNS: dict[str, re.Pattern[str]] = {
    "require_color": re.compile(r'require_color\(\s*"([^"]+)"\s*\)'),
    "require_space_px": re.compile(r'require_space_px\(\s*"([^"]+)"\s*\)'),
    "require_size_px": re.compile(r'require_size_px\(\s*"([^"]+)"\s*\)'),
    "require_radius_px": re.compile(r'require_radius_px\(\s*"([^"]+)"\s*\)'),
    "require_stroke_px": re.compile(r'require_stroke_px\(\s*"([^"]+)"\s*\)'),
    "require_motion_ms": re.compile(r'require_motion_ms\(\s*"([^"]+)"\s*\)'),
}


def load_baseline(path: Path) -> SurfaceBaseline:
    if not path.exists():
        raise AuditError(f"missing baseline file: {path}")
    with path.open("r", encoding="utf-8") as fh:
        payload = json.load(fh)
    if not isinstance(payload, dict):
        raise AuditError(f"{path}: expected JSON object root")

    record_kind = payload.get("record_kind")
    schema_version = payload.get("schema_version")
    surface_id = payload.get("surface_id")
    sources = payload.get("sources")
    require_calls = payload.get("require_calls")

    if record_kind != "token_adoption_baseline_record":
        raise AuditError(f"{path}: record_kind must be token_adoption_baseline_record")
    if schema_version != 1:
        raise AuditError(f"{path}: schema_version must be 1")
    if not isinstance(surface_id, str) or not surface_id:
        raise AuditError(f"{path}: surface_id must be a non-empty string")
    if not isinstance(sources, list) or not all(isinstance(item, str) for item in sources):
        raise AuditError(f"{path}: sources must be a list of strings")
    if not isinstance(require_calls, dict):
        raise AuditError(f"{path}: require_calls must be a mapping")

    normalized: dict[str, tuple[str, ...]] = {}
    for key, value in require_calls.items():
        if key not in REQUIRE_PATTERNS:
            raise AuditError(f"{path}: unknown require_calls key {key!r}")
        if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
            raise AuditError(f"{path}: require_calls.{key} must be a list of strings")
        normalized[key] = tuple(value)

    return SurfaceBaseline(
        record_kind=record_kind,
        schema_version=schema_version,
        surface_id=surface_id,
        sources=tuple(sources),
        require_calls=normalized,
    )


def read_text(path: Path) -> str:
    with path.open("r", encoding="utf-8") as fh:
        return fh.read()


def extract_require_calls(*, repo_root: Path, sources: Iterable[str]) -> dict[str, list[str]]:
    observed: dict[str, set[str]] = {key: set() for key in REQUIRE_PATTERNS}
    for rel in sources:
        src_path = (repo_root / rel).resolve()
        if not src_path.exists():
            raise AuditError(f"missing source file referenced by baseline: {rel}")
        text = read_text(src_path)
        for key, pattern in REQUIRE_PATTERNS.items():
            for match in pattern.findall(text):
                observed[key].add(match)

    return {key: sorted(values) for key, values in observed.items()}


def unified_diff(label: str, expected: list[str], actual: list[str]) -> str:
    diff = difflib.unified_diff(
        expected,
        actual,
        fromfile=f"{label}:baseline",
        tofile=f"{label}:observed",
        lineterm="",
    )
    return "\n".join(diff)


def write_baseline(path: Path, baseline: SurfaceBaseline, observed: dict[str, list[str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "record_kind": baseline.record_kind,
        "schema_version": baseline.schema_version,
        "surface_id": baseline.surface_id,
        "sources": list(baseline.sources),
        "require_calls": observed,
    }
    with path.open("w", encoding="utf-8") as fh:
        json.dump(payload, fh, indent=2, sort_keys=True)
        fh.write("\n")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Audit token adoption on protected appearance surfaces."
    )
    parser.add_argument(
        "--baseline",
        default="tests/golden/appearance/shell_chrome/token_adoption_baseline.json",
        help="Path to the baseline JSON file (repo-relative).",
    )
    parser.add_argument(
        "--update",
        action="store_true",
        help="Rewrite the baseline from the currently observed require_* calls.",
    )
    args = parser.parse_args(argv)

    baseline_path = (REPO_ROOT / args.baseline).resolve()
    baseline = load_baseline(baseline_path)

    observed = extract_require_calls(repo_root=REPO_ROOT, sources=baseline.sources)

    if args.update:
        write_baseline(baseline_path, baseline, observed)
        print(f"[token-adoption] updated baseline: {baseline_path}")
        return 0

    failures: list[str] = []
    for key, expected_values in baseline.require_calls.items():
        expected = sorted(expected_values)
        actual = observed.get(key, [])
        if expected != actual:
            diff = unified_diff(key, expected, actual)
            failures.append(diff or f"{key}: baseline mismatch")

    if failures:
        joined = "\n\n".join(failures)
        print("[token-adoption] baseline mismatch detected:\n", file=sys.stderr)
        print(joined, file=sys.stderr)
        print(
            "\n[token-adoption] If this is intentional, run:\n"
            f"  python3 tools/ci/check_token_adoption.py --update --baseline {args.baseline}\n",
            file=sys.stderr,
        )
        return 1

    print(f"[token-adoption] ok: {baseline.surface_id}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))

