#!/usr/bin/env python3
"""Reject Cargo target names that create non-portable rustc object paths."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


# rustc can repeat a target name in a codegen-unit filename. Keeping targets at
# or below 80 characters leaves ample room below the 255-byte filename limit
# used by macOS and many Linux filesystems.
MAX_TARGET_NAME_LENGTH = 80


def audit(repo_root: Path) -> tuple[int, list[str]]:
    failures: list[str] = []
    protected = 0

    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--locked",
            "--no-deps",
            "--format-version",
            "1",
        ],
        cwd=repo_root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip()
        return 0, [f"cargo metadata failed: {detail}"]

    metadata = json.loads(result.stdout)
    for package in metadata["packages"]:
        for target in package["targets"]:
            name = target["name"]
            source = Path(target["src_path"])
            if len(name) > MAX_TARGET_NAME_LENGTH:
                kinds = ",".join(target["kind"])
                failures.append(
                    f"{package['name']} {kinds} target {name!r} is {len(name)} "
                    f"characters; maximum is {MAX_TARGET_NAME_LENGTH}"
                )
            elif len(source.stem) > MAX_TARGET_NAME_LENGTH:
                protected += 1

    return protected, failures


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Check Cargo target names for portable rustc object-path lengths."
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
        f"{protected} long target sources use portable explicit target names"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
