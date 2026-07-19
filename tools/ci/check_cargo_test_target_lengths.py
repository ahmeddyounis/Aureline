#!/usr/bin/env python3
"""Reject integration-test target names that create non-portable rustc paths."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


# rustc can repeat the crate name in a codegen-unit filename. Keeping integration
# targets at or below 80 characters leaves ample room below the 255-byte filename
# limit used by macOS and many Linux filesystems.
MAX_TARGET_NAME_LENGTH = 80
TEST_TABLE = re.compile(
    r"^\[\[test\]\]\s*(.*?)(?=^\[\[|^\[(?!\[)|\Z)",
    flags=re.MULTILINE | re.DOTALL,
)
FIELD = re.compile(r'^\s*(name|path)\s*=\s*"([^"]+)"\s*$', re.MULTILINE)


def explicit_tests(manifest: Path) -> dict[str, str]:
    """Return explicit integration-test paths mapped to their Cargo target names."""

    text = manifest.read_text(encoding="utf-8")
    targets: dict[str, str] = {}
    for table in TEST_TABLE.findall(text):
        fields = dict(FIELD.findall(table))
        name = fields.get("name")
        path = fields.get("path")
        if name is not None and path is not None:
            targets[path] = name
    return targets


def audit(repo_root: Path) -> tuple[int, list[str]]:
    failures: list[str] = []
    protected = 0

    for tests_dir in sorted(repo_root.glob("crates/*/tests")):
        manifest = tests_dir.parent / "Cargo.toml"
        targets = explicit_tests(manifest)

        for path, name in sorted(targets.items()):
            if len(name) > MAX_TARGET_NAME_LENGTH:
                failures.append(
                    f"{manifest.relative_to(repo_root)}: explicit test target {name!r} "
                    f"is {len(name)} characters; maximum is {MAX_TARGET_NAME_LENGTH}"
                )

        for source in sorted(tests_dir.glob("*.rs")):
            stem = source.stem
            if len(stem) <= MAX_TARGET_NAME_LENGTH:
                continue

            rel_to_crate = source.relative_to(tests_dir.parent).as_posix()
            target_name = targets.get(rel_to_crate)
            if target_name is None:
                failures.append(
                    f"{source.relative_to(repo_root)}: inferred Cargo target is "
                    f"{len(stem)} characters; add an explicit [[test]] with a short name"
                )
                continue
            protected += 1

    return protected, failures


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Check Cargo integration-test target names for portable path lengths."
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path(__file__).resolve().parents[2],
        help="Repository root (defaults to the root containing this script).",
    )
    args = parser.parse_args(argv)

    repo_root = args.repo_root.resolve()
    protected, failures = audit(repo_root)
    if failures:
        for failure in failures:
            print(f"[cargo-target-lengths] error: {failure}", file=sys.stderr)
        return 1

    print(
        "[cargo-target-lengths] PASS: "
        f"{protected} long integration-test sources use portable explicit target names"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
