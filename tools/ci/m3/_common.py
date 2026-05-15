"""Shared helpers for the M3 docs-and-public-truth gates.

These helpers parse the source map, the M3 claim manifest, and the M3
compatibility report so the freshness gate and the stale-example
checker read upstream truth through one routine. The helpers are kept
deliberately small; both gates remain runnable on a fresh checkout
without third-party Python dependencies (YAML is decoded via Ruby /
Psych, JSON Schema validation degrades to a structural sanity check
when ``jsonschema`` is not installed).
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_SOURCE_MAP_REL = "artifacts/ci/m3_docs_truth_source_map.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_SOURCE_MAP_ID = "m3_docs_truth_source_map"

FIXTURE_METADATA_KEYS = {"$schema", "__fixture__"}


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


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(
            f"failed to parse YAML at {path} via Ruby/Psych: {stderr}"
        )
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def load_json_payload(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


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


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except Exception as exc:  # noqa: BLE001
        raise SystemExit(f"{label} is not an ISO date: {value!r} ({exc})")


def strip_fixture_metadata(value: Any) -> Any:
    if isinstance(value, dict):
        stripped: dict[str, Any] = {}
        for key, item in value.items():
            if key in FIXTURE_METADATA_KEYS:
                continue
            stripped[key] = strip_fixture_metadata(item)
        return stripped
    if isinstance(value, list):
        return [strip_fixture_metadata(item) for item in value]
    return value


def resolve_payload_path(payload: Any, path: str) -> tuple[bool, Any]:
    """Resolve a dotted ``payload_path`` like ``source_truth.source_class``.

    Numeric components are list indices. A missing key / index yields
    ``(False, None)`` instead of raising so callers can report a
    targeted finding.
    """
    if not path:
        return True, payload
    cursor: Any = payload
    for raw_segment in path.split("."):
        segment = raw_segment.strip()
        if not segment:
            return False, None
        if isinstance(cursor, list):
            try:
                index = int(segment)
            except ValueError:
                return False, None
            if index < 0 or index >= len(cursor):
                return False, None
            cursor = cursor[index]
            continue
        if not isinstance(cursor, dict) or segment not in cursor:
            return False, None
        cursor = cursor[segment]
    return True, cursor


def set_payload_path(payload: Any, path: str, value: Any) -> bool:
    """Write ``value`` at dotted ``path``. Returns False if the path is
    not already present on the payload."""
    segments = [s.strip() for s in path.split(".") if s.strip()]
    if not segments:
        return False
    cursor: Any = payload
    for segment in segments[:-1]:
        if isinstance(cursor, list):
            try:
                index = int(segment)
            except ValueError:
                return False
            if index < 0 or index >= len(cursor):
                return False
            cursor = cursor[index]
            continue
        if not isinstance(cursor, dict) or segment not in cursor:
            return False
        cursor = cursor[segment]
    last = segments[-1]
    if isinstance(cursor, list):
        try:
            index = int(last)
        except ValueError:
            return False
        if index < 0 or index >= len(cursor):
            return False
        cursor[index] = value
        return True
    if isinstance(cursor, dict) and last in cursor:
        cursor[last] = value
        return True
    return False


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def parse_payload(path: Path) -> Any:
    if path.suffix.lower() == ".json":
        return load_json_payload(path)
    if path.suffix.lower() in {".yaml", ".yml"}:
        return render_yaml_as_json(path)
    raise SystemExit(
        f"protected payload must be .json/.yaml/.yml: {path}"
    )


def validate_payload_against_schema(
    *,
    payload: Any,
    schema_ref: str,
    schema_path: Path,
) -> tuple[bool, list[str]]:
    """Best-effort schema validation.

    Falls back to ``schema_validator_unavailable`` when ``jsonschema``
    is not importable so a fresh checkout can still run the gate.
    """
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except Exception:  # noqa: BLE001
        return True, ["schema_validator_unavailable"]

    try:
        schema = load_json_payload(schema_path)
    except Exception as exc:  # noqa: BLE001
        return False, [f"failed_to_parse_schema:{schema_ref}:{exc}"]

    try:
        validator = Draft202012Validator(schema)
    except Exception as exc:  # noqa: BLE001
        return False, [f"failed_to_build_validator:{schema_ref}:{exc}"]

    stripped = strip_fixture_metadata(payload)
    errors = sorted(validator.iter_errors(stripped), key=lambda e: list(e.path))
    if not errors:
        return True, []
    messages: list[str] = []
    for err in errors[:25]:
        loc = ".".join(str(p) for p in err.path) or "<root>"
        messages.append(f"{loc}: {str(err.message)[:240]}")
    if len(errors) > 25:
        messages.append(f"... ({len(errors) - 25} additional errors truncated)")
    return False, messages


def load_source_map(repo_root: Path, rel: str) -> dict[str, Any]:
    """Parse the M3 source map and assert schema_version and id."""
    source_map_path = repo_root / rel
    source_map = ensure_dict(
        render_yaml_as_json(source_map_path), rel
    )
    schema_version = source_map.get("schema_version")
    if schema_version != EXPECTED_SCHEMA_VERSION:
        raise SystemExit(
            f"source map schema_version must be "
            f"{EXPECTED_SCHEMA_VERSION}; got {schema_version!r}"
        )
    source_map_id = source_map.get("source_map_id")
    if source_map_id != EXPECTED_SOURCE_MAP_ID:
        raise SystemExit(
            f"source map source_map_id must be "
            f"{EXPECTED_SOURCE_MAP_ID!r}; got {source_map_id!r}"
        )
    ensure_str(source_map.get("status"), "source_map.status")
    ensure_str(source_map.get("owner_dri"), "source_map.owner_dri")
    return source_map


def load_claim_manifest(repo_root: Path, ref: str) -> dict[str, Any]:
    manifest = ensure_dict(
        load_json_payload(repo_root / ref), f"claim_manifest@{ref}"
    )
    if manifest.get("record_kind") != "m3_claim_manifest":
        raise SystemExit(
            f"{ref} is not an m3_claim_manifest record; got "
            f"record_kind={manifest.get('record_kind')!r}"
        )
    return manifest


def load_compatibility_report(repo_root: Path, ref: str) -> dict[str, Any]:
    report = ensure_dict(
        load_json_payload(repo_root / ref),
        f"compatibility_report@{ref}",
    )
    if report.get("record_kind") != "compatibility_report":
        raise SystemExit(
            f"{ref} is not a compatibility_report record; got "
            f"record_kind={report.get('record_kind')!r}"
        )
    return report


def index_manifest_rows(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(manifest.get("rows"), "claim_manifest.rows")
    return {
        ensure_str(row.get("row_id"), "claim_manifest.row.row_id"): row
        for row in rows
    }


def index_compat_rows(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(report.get("rows"), "compatibility_report.rows")
    return {
        ensure_str(row.get("row_id"), "compatibility_report.row.row_id"): row
        for row in rows
    }


_GENERATED_AT_RE = re.compile(
    r'"generated_at":\s*"[^"]*"|"captured_at":\s*"[^"]*"|'
    r'\*\*Generated at:\*\*\s*`[^`]*`'
)


def normalize_generated_text(text: str) -> str:
    return _GENERATED_AT_RE.sub("__generated_at__", text)


def write_if_changed(path: Path, content: str, check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    existing: str | None = None
    if path.exists():
        existing = path.read_text(encoding="utf-8")
    changed = (
        existing is None
        or normalize_generated_text(existing) != normalize_generated_text(content)
    )
    if not check_only:
        path.write_text(content, encoding="utf-8")
    return changed


def base_argument_parser(description: str) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--source-map",
        default=DEFAULT_SOURCE_MAP_REL,
        help="Source map YAML path, repo-relative.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build-identity record path the capture embeds.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail if the on-disk generated artifacts would change "
            "after regeneration. Use this in CI."
        ),
    )
    parser.add_argument(
        "--today",
        default=None,
        help=(
            "Override today's ISO date (YYYY-MM-DD). Used for "
            "reproducible review-window checks in tests."
        ),
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay the named drill from source_map.failure_drills[]. "
            "The gate exits 0 only when the drill reproduces the "
            "declared expected_check_id."
        ),
    )
    return parser


def resolve_today(today_arg: str | None) -> dt.date:
    if today_arg:
        return parse_iso_date(today_arg, "--today")
    return dt.datetime.now(dt.timezone.utc).date()
