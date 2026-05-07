#!/usr/bin/env python3
"""Validate reopen-after-eviction storage fixtures against their boundary schema.

This repository prefers a lightweight toolchain. The fixtures are YAML, but the
schema is JSON Schema draft 2020-12. To avoid adding Python YAML dependencies,
this validator parses YAML via Ruby/Psych (which ships on macOS and the standard
CI images) and validates the resulting JSON against the schema via `jsonschema`.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
SCHEMA_PATH = REPO_ROOT / "schemas/storage/reopen_after_eviction_packet.schema.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/storage/reopen_after_eviction_cases"


def load_yaml_via_ruby(path: Path) -> object:
    payload = subprocess.run(
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
    if payload.returncode != 0:
        stderr = payload.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    return json.loads(payload.stdout)


def main() -> int:
    try:
        import jsonschema
    except Exception as exc:  # pragma: no cover
        print(f"[validate-storage-fixtures] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    validator = jsonschema.Draft202012Validator(schema)

    fixture_paths = sorted(FIXTURE_DIR.glob("*.yaml"))
    if not fixture_paths:
        print(f"[validate-storage-fixtures] error: no fixtures found under {FIXTURE_DIR}", file=sys.stderr)
        return 2

    failures = 0
    for fixture_path in fixture_paths:
        instance = load_yaml_via_ruby(fixture_path)
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        if not errors:
            continue
        failures += 1
        rel = fixture_path.relative_to(REPO_ROOT)
        print(f"[validate-storage-fixtures] {rel}: {len(errors)} error(s)")
        for err in errors[:12]:
            loc = "/".join(str(part) for part in err.path) or "(root)"
            print(f"  - {loc}: {err.message}")

    if failures:
        print(f"[validate-storage-fixtures] FAIL ({failures} fixture(s) invalid)", file=sys.stderr)
        return 1

    print("[validate-storage-fixtures] OK (all reopen-after-eviction fixtures validate)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

