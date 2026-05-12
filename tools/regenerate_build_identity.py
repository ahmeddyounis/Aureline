#!/usr/bin/env python3
"""Regenerate the checked-in build identity artifact from Cargo output."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
BUILD_IDENTITY_PATH = REPO_ROOT / "artifacts/build/build_identity.json"
BUILD_INFO_PACKAGE = "aureline-build-info"
BUILD_IDENTITY_FILENAME = "build_identity.json"


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def repo_relative(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def run_cargo_build() -> str:
    command = [
        "cargo",
        "build",
        "-p",
        BUILD_INFO_PACKAGE,
        "--message-format=json",
    ]
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stderr)
        sys.stderr.write(completed.stdout)
        print(
            f"failed to run {' '.join(command)}",
            file=sys.stderr,
        )
        raise SystemExit(completed.returncode)

    if completed.stderr:
        sys.stderr.write(completed.stderr)

    return completed.stdout


def find_out_dir(cargo_stdout: str) -> Path:
    out_dir: Path | None = None
    for raw_line in cargo_stdout.splitlines():
        if not raw_line.strip():
            continue
        try:
            message = json.loads(raw_line)
        except json.JSONDecodeError:
            continue
        if message.get("reason") != "build-script-executed":
            continue
        package_id = str(message.get("package_id", ""))
        if BUILD_INFO_PACKAGE not in package_id:
            continue
        raw_out_dir = message.get("out_dir")
        if isinstance(raw_out_dir, str) and raw_out_dir:
            out_dir = Path(raw_out_dir)

    if out_dir is not None:
        return out_dir

    candidates = list(
        (REPO_ROOT / "target").glob(
            f"**/build/{BUILD_INFO_PACKAGE}-*/out/{BUILD_IDENTITY_FILENAME}"
        )
    )
    if candidates:
        newest = max(candidates, key=lambda path: path.stat().st_mtime)
        return newest.parent

    print(
        f"cargo output did not report an OUT_DIR for {BUILD_INFO_PACKAGE}",
        file=sys.stderr,
    )
    raise SystemExit(1)


def preserve_schema_version(payload: dict[str, Any]) -> None:
    if not BUILD_IDENTITY_PATH.exists():
        return
    current = load_json(BUILD_IDENTITY_PATH)
    if not isinstance(current, dict):
        return
    schema_version = current.get("schema_version")
    if isinstance(schema_version, int):
        payload["schema_version"] = schema_version


def main() -> int:
    cargo_stdout = run_cargo_build()
    out_dir = find_out_dir(cargo_stdout)
    source_path = out_dir / BUILD_IDENTITY_FILENAME
    if not source_path.exists():
        print(
            f"missing generated build identity artifact: {repo_relative(source_path)}",
            file=sys.stderr,
        )
        return 1

    payload = load_json(source_path)
    if not isinstance(payload, dict):
        print(
            f"generated build identity must be a JSON object: {repo_relative(source_path)}",
            file=sys.stderr,
        )
        return 1

    preserve_schema_version(payload)
    BUILD_IDENTITY_PATH.parent.mkdir(parents=True, exist_ok=True)
    BUILD_IDENTITY_PATH.write_text(
        json.dumps(payload, indent=2) + "\n",
        encoding="utf-8",
    )
    print(
        "regenerated "
        f"{repo_relative(BUILD_IDENTITY_PATH)} from {repo_relative(source_path)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
