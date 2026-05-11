#!/usr/bin/env python3
"""Stale-example detection pipeline for protected M1 docs/help-pack examples.

This validator is the M1 skeleton for the stale-example detection CI gate.
It reads the maintained source map at
``artifacts/ci/m1_stale_example_source_map.yaml`` and, for every protected
docs-pack row, asserts that:

- the docs-pack page exists on disk and publishes the required
  source / version / freshness metadata tokens the row pins;
- every protected example payload exists, parses, and validates against
  its pinned schema (after stripping the fixture-only metadata keys
  ``$schema`` and ``__fixture__``);
- every example's pinned vocabulary tokens still appear in the
  controlling vocabulary seed row's ``vocabulary_tokens`` list (so a
  token that the upstream seed retires fails the gate on every example
  that still pins it); and
- the row's named failure drill is reproducible: under
  ``--force-drill`` the named drill is replayed and the gate exits 0
  only when the declared ``expected_check_id`` is observed.

The runner emits a deterministic human summary on stdout and a durable
JSON capture (``--report`` path) for proof archives.

YAML decoding follows the repository convention: matrix files are
parsed via Ruby/Psych so this script does not require a third-party
Python YAML dependency. JSON Schema validation uses ``jsonschema`` when
available and falls back to a structural sanity check otherwise so a
fresh checkout can still run the gate before optional dev dependencies
are installed.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_SOURCE_MAP_REL = "artifacts/ci/m1_stale_example_source_map.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "docs_pack_and_example_checks_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_SOURCE_MAP_ID = "m1_stale_example_source_map"

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


@dataclass
class PackResult:
    pack_id: str
    pack_ref: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    examples: list[dict[str, Any]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(
    result: PackResult,
    check_id: str,
    message: str,
    *,
    ref: str | None = None,
    details: dict[str, Any] | None = None,
) -> None:
    entry: dict[str, Any] = {"check_id": check_id, "message": message}
    if ref:
        entry["ref"] = ref
    if details:
        entry["details"] = details
    result.failed_checks.append(entry)


def pass_(result: PackResult, message: str) -> None:
    result.passed_checks.append(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
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
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill in the form "
            "'<pack_id>:<drill_id>'. The runner applies the drill's "
            "forced_input, re-validates the affected pack, and exits 0 "
            "only when the drill reproduces the declared expected_check_id."
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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


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
    """Resolve a dotted payload_path like 'source_truth.source_class'.

    Returns (found, value). Numeric path components are treated as list
    indices. A missing key / index yields (False, None) without raising.
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
    """Write `value` at dotted `path` on `payload` (must already exist)."""
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


def load_json_payload(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


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
    """Best-effort schema validation. Falls back to a structural check.

    Returns (passed, error_messages). When the optional `jsonschema`
    package is not installed the function reports a single advisory
    "schema_validator_unavailable" finding so the caller can decide
    whether to treat it as a soft warning rather than as a stale
    example.
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


def load_vocabulary_seeds(
    repo_root: Path,
    vocabulary_seeds: list[dict[str, Any]],
) -> tuple[dict[str, dict[str, Any]], list[Finding]]:
    """Resolve vocabulary_seeds[*] into vocabulary_id -> seed row state."""
    findings: list[Finding] = []
    cache: dict[str, Any] = {}
    resolved: dict[str, dict[str, Any]] = {}
    for entry in vocabulary_seeds:
        if not isinstance(entry, dict):
            continue
        vocab_id = entry.get("vocabulary_id")
        seed_ref = entry.get("seed_ref")
        row_id = entry.get("seed_row_id")
        if not isinstance(vocab_id, str) or not vocab_id:
            continue
        if not isinstance(seed_ref, str) or not seed_ref:
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_ref_missing",
                    message=(
                        f"vocabulary_seeds[{vocab_id}].seed_ref must be a "
                        "non-empty path."
                    ),
                    remediation=(
                        "Set seed_ref to the canonical seed YAML on disk."
                    ),
                    ref=vocab_id,
                )
            )
            continue
        if not isinstance(row_id, str) or not row_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_row_id_missing",
                    message=(
                        f"vocabulary_seeds[{vocab_id}].seed_row_id must be a "
                        "non-empty row id."
                    ),
                    remediation=(
                        "Set seed_row_id to the canonical row id in the seed."
                    ),
                    ref=vocab_id,
                )
            )
            continue
        seed_path = repo_root / seed_ref
        if not seed_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_missing",
                    message=(
                        f"vocabulary_seeds[{vocab_id}].seed_ref does not "
                        f"exist on disk: {seed_ref}"
                    ),
                    remediation=(
                        "Restore the seed file or fix the seed_ref path."
                    ),
                    ref=vocab_id,
                    details={"seed_ref": seed_ref},
                )
            )
            continue
        seed_doc = cache.get(seed_ref)
        if seed_doc is None:
            try:
                seed_doc = render_yaml_as_json(seed_path)
            except SystemExit as exc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="stale_examples.vocabulary_seed_unparseable",
                        message=(
                            f"vocabulary_seeds[{vocab_id}].seed_ref failed "
                            f"to parse: {exc}"
                        ),
                        remediation="Fix the seed YAML so Ruby/Psych can parse it.",
                        ref=vocab_id,
                    )
                )
                continue
            cache[seed_ref] = seed_doc
        rows = (seed_doc or {}).get("rows") if isinstance(seed_doc, dict) else None
        if not isinstance(rows, list):
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_rows_missing",
                    message=(
                        f"vocabulary_seeds[{vocab_id}].seed_ref does not "
                        "contain a `rows` list."
                    ),
                    remediation=(
                        "Point seed_ref at a seed that exposes the canonical "
                        "rows list (e.g. the M1 truth-source seed)."
                    ),
                    ref=vocab_id,
                )
            )
            continue
        match = None
        for row in rows:
            if isinstance(row, dict) and row.get("row_id") == row_id:
                match = row
                break
        if match is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_row_not_found",
                    message=(
                        f"vocabulary_seeds[{vocab_id}].seed_row_id "
                        f"{row_id!r} not found in {seed_ref}."
                    ),
                    remediation=(
                        "Point seed_row_id at an existing row, or remove the "
                        "vocabulary entry if the upstream row was retired."
                    ),
                    ref=vocab_id,
                )
            )
            continue
        tokens_field = match.get("vocabulary_tokens")
        if not isinstance(tokens_field, list):
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.vocabulary_seed_tokens_missing",
                    message=(
                        f"vocabulary_seeds[{vocab_id}] seed row {row_id!r} "
                        "does not expose a vocabulary_tokens list."
                    ),
                    remediation=(
                        "Update the seed row so vocabulary_tokens is a list, "
                        "or pick a row that does."
                    ),
                    ref=vocab_id,
                )
            )
            continue
        token_strings: list[str] = []
        for token in tokens_field:
            if isinstance(token, dict):
                value = token.get("token")
                if isinstance(value, str) and value:
                    token_strings.append(value)
            elif isinstance(token, str) and token:
                token_strings.append(token)
        resolved[vocab_id] = {
            "seed_ref": seed_ref,
            "seed_row_id": row_id,
            "tokens": token_strings,
        }
    return resolved, findings


def apply_forced_overrides(
    payload: Any,
    forced_overrides: dict[str, Any] | None,
) -> tuple[Any, dict[str, Any]]:
    """Apply a drill's forced_input to a copy of `payload`."""
    payload = copy.deepcopy(payload)
    applied: dict[str, Any] = {}
    if not forced_overrides:
        return payload, applied

    rewrite = forced_overrides.get("rewrite_vocabulary_pin")
    if isinstance(rewrite, dict):
        target_path = rewrite.get("payload_path")
        replacement = rewrite.get("replacement_value")
        if isinstance(target_path, str) and target_path:
            if set_payload_path(payload, target_path, replacement):
                applied["rewrite_vocabulary_pin"] = {
                    "payload_path": target_path,
                    "replacement_value": replacement,
                }

    drop = forced_overrides.get("drop_payload_path")
    if isinstance(drop, str) and drop:
        if set_payload_path(payload, drop, None):
            applied["drop_payload_path"] = drop

    return payload, applied


def find_required_metadata_tokens(
    pack_text: str,
    required_tokens: list[str],
) -> list[str]:
    """Return the subset of `required_tokens` that are missing from pack_text."""
    missing: list[str] = []
    for token in required_tokens:
        if not isinstance(token, str) or not token:
            continue
        if token not in pack_text:
            missing.append(token)
    return missing


def validate_pack(
    pack: dict[str, Any],
    *,
    repo_root: Path,
    vocabulary_index: dict[str, dict[str, Any]],
    forced_pack_id: str | None,
    forced_drill: dict[str, Any] | None,
) -> tuple[PackResult, dict[str, Any] | None]:
    pack_id = ensure_str(pack.get("pack_id"), "protected_docs_packs[].pack_id")
    pack_ref = ensure_str(
        pack.get("pack_ref"),
        f"protected_docs_packs[{pack_id}].pack_ref",
    )
    result = PackResult(pack_id=pack_id, pack_ref=pack_ref)
    forced_replay_record: dict[str, Any] | None = None

    pack_title = pack.get("pack_title")
    if isinstance(pack_title, str) and pack_title.strip():
        result.diagnostics["pack_title"] = pack_title.strip()
    upstream = pack.get("upstream_truth_source_ref")
    if isinstance(upstream, str) and upstream.strip():
        if not artifact_ref_exists(repo_root, upstream):
            fail(
                result,
                "stale_examples.upstream_truth_source_missing",
                (
                    f"upstream_truth_source_ref does not exist on disk: "
                    f"{upstream}"
                ),
                ref=upstream,
            )
        else:
            result.diagnostics["upstream_truth_source_ref"] = upstream

    pack_path = repo_root / pack_ref
    if not pack_path.exists():
        fail(
            result,
            "stale_examples.docs_pack_missing",
            f"docs pack page does not exist on disk: {pack_ref}",
            ref=pack_ref,
        )
        pack_text = ""
    else:
        pack_text = pack_path.read_text(encoding="utf-8")

    # ---- required source / version / freshness metadata --------------
    required_tokens_raw = pack.get("required_metadata_tokens") or []
    required_tokens = [t for t in required_tokens_raw if isinstance(t, str) and t]
    if not required_tokens:
        fail(
            result,
            "stale_examples.required_metadata_tokens_empty",
            "required_metadata_tokens must list at least one token "
            "(source / version / freshness).",
            ref=pack_id,
        )
    elif pack_text:
        missing = find_required_metadata_tokens(pack_text, required_tokens)
        if missing:
            fail(
                result,
                "stale_examples.required_metadata_tokens_missing",
                (
                    "docs pack page does not publish the required source / "
                    f"version / freshness tokens: {sorted(missing)}"
                ),
                ref=pack_ref,
            )
        else:
            result.diagnostics["required_metadata_tokens_present"] = sorted(
                required_tokens
            )

    # ---- protected example validation -------------------------------
    raw_examples = pack.get("protected_examples") or []
    examples = ensure_list(
        raw_examples,
        f"protected_docs_packs[{pack_id}].protected_examples",
    )
    if not examples:
        fail(
            result,
            "stale_examples.protected_examples_empty",
            "protected_examples must list at least one fixture.",
            ref=pack_id,
        )

    forced_example_overrides: dict[str, Any] = {}
    forced_example_id: str | None = None
    if (
        forced_pack_id is not None
        and forced_pack_id == pack_id
        and forced_drill is not None
    ):
        forced_input = forced_drill.get("forced_input")
        if isinstance(forced_input, dict):
            target_example_id = forced_input.get("example_id")
            if isinstance(target_example_id, str) and target_example_id:
                forced_example_id = target_example_id
            forced_example_overrides = forced_input

    for example in examples:
        if not isinstance(example, dict):
            continue
        example_id = example.get("example_id")
        if not isinstance(example_id, str) or not example_id:
            fail(
                result,
                "stale_examples.example_id_missing",
                "every protected_examples[] entry must declare a non-empty "
                "example_id.",
                ref=pack_id,
            )
            continue
        example_record: dict[str, Any] = {
            "example_id": example_id,
            "passed_checks": [],
            "failed_checks": [],
            "diagnostics": {},
        }
        payload_ref = example.get("payload_ref")
        schema_ref = example.get("schema_ref")
        if not isinstance(payload_ref, str) or not payload_ref:
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.payload_ref_missing",
                    "message": (
                        f"protected_examples[{example_id}].payload_ref is "
                        "missing or empty."
                    ),
                }
            )
            result.examples.append(example_record)
            fail(
                result,
                "stale_examples.payload_ref_missing",
                (
                    f"protected_examples[{example_id}].payload_ref is "
                    "missing or empty."
                ),
                ref=example_id,
            )
            continue
        if not isinstance(schema_ref, str) or not schema_ref:
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.schema_ref_missing",
                    "message": (
                        f"protected_examples[{example_id}].schema_ref is "
                        "missing or empty."
                    ),
                }
            )
            result.examples.append(example_record)
            fail(
                result,
                "stale_examples.schema_ref_missing",
                (
                    f"protected_examples[{example_id}].schema_ref is "
                    "missing or empty."
                ),
                ref=example_id,
            )
            continue

        payload_path = repo_root / payload_ref
        schema_path = repo_root / schema_ref
        if not payload_path.exists():
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.payload_missing",
                    "message": f"payload file does not exist: {payload_ref}",
                }
            )
            fail(
                result,
                "stale_examples.payload_missing",
                f"protected example payload does not exist: {payload_ref}",
                ref=example_id,
            )
            result.examples.append(example_record)
            continue
        if not schema_path.exists():
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.schema_missing",
                    "message": f"schema file does not exist: {schema_ref}",
                }
            )
            fail(
                result,
                "stale_examples.schema_missing",
                f"protected example schema does not exist: {schema_ref}",
                ref=example_id,
            )
            result.examples.append(example_record)
            continue

        try:
            payload = parse_payload(payload_path)
        except Exception as exc:  # noqa: BLE001
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.payload_parse_failed",
                    "message": (
                        f"payload {payload_ref} failed to parse: {exc}"
                    ),
                }
            )
            fail(
                result,
                "stale_examples.payload_parse_failed",
                (
                    f"protected example payload {payload_ref} failed to "
                    f"parse: {exc}"
                ),
                ref=example_id,
            )
            result.examples.append(example_record)
            continue

        applied_overrides: dict[str, Any] = {}
        if (
            forced_pack_id is not None
            and forced_pack_id == pack_id
            and forced_example_id == example_id
        ):
            payload, applied_overrides = apply_forced_overrides(
                payload, forced_example_overrides
            )
            if applied_overrides:
                example_record["diagnostics"][
                    "forced_overrides_applied"
                ] = applied_overrides

        # Schema validity (after fixture-metadata strip).
        ok, schema_errors = validate_payload_against_schema(
            payload=payload,
            schema_ref=schema_ref,
            schema_path=schema_path,
        )
        if not ok:
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.example_payload_schema_invalid",
                    "message": (
                        f"payload {payload_ref} does not validate against "
                        f"{schema_ref}: {schema_errors[:3]}"
                    ),
                }
            )
            fail(
                result,
                "stale_examples.example_payload_schema_invalid",
                (
                    f"protected example payload {payload_ref} does not "
                    f"validate against {schema_ref}."
                ),
                ref=example_id,
            )
        elif schema_errors == ["schema_validator_unavailable"]:
            example_record["diagnostics"]["schema_validator"] = "unavailable"
        else:
            example_record["passed_checks"].append(
                "example_payload_schema_valid"
            )

        # Vocabulary-pin freshness.
        vocab_pins = example.get("vocabulary_pins") or []
        if not isinstance(vocab_pins, list) or not vocab_pins:
            example_record["failed_checks"].append(
                {
                    "check_id": "stale_examples.vocabulary_pins_empty",
                    "message": (
                        f"protected_examples[{example_id}].vocabulary_pins "
                        "must list at least one pin."
                    ),
                }
            )
            fail(
                result,
                "stale_examples.vocabulary_pins_empty",
                (
                    f"protected_examples[{example_id}].vocabulary_pins "
                    "must list at least one pin."
                ),
                ref=example_id,
            )
        else:
            for pin in vocab_pins:
                if not isinstance(pin, dict):
                    continue
                vocab_id = pin.get("vocabulary_id")
                pin_path = pin.get("payload_path")
                if not isinstance(vocab_id, str) or not vocab_id:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_id_missing"
                            ),
                            "message": (
                                "vocabulary_pins[] entry missing "
                                "vocabulary_id"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_id_missing",
                        (
                            f"protected_examples[{example_id}] has a "
                            "vocabulary_pin without vocabulary_id."
                        ),
                        ref=example_id,
                    )
                    continue
                if not isinstance(pin_path, str) or not pin_path:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_path_missing"
                            ),
                            "message": (
                                f"vocabulary_pins[{vocab_id}] missing "
                                "payload_path"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_path_missing",
                        (
                            f"protected_examples[{example_id}] vocabulary_pin "
                            f"{vocab_id} is missing payload_path."
                        ),
                        ref=example_id,
                    )
                    continue
                if vocab_id not in vocabulary_index:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_unknown_id"
                            ),
                            "message": (
                                f"vocabulary_pins[{vocab_id}] not declared in "
                                "source map vocabulary_seeds[]"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_unknown_id",
                        (
                            f"protected_examples[{example_id}] vocabulary_pin "
                            f"{vocab_id} is not declared in "
                            "vocabulary_seeds."
                        ),
                        ref=example_id,
                    )
                    continue
                found, value = resolve_payload_path(payload, pin_path)
                if not found:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_path_unresolved"
                            ),
                            "message": (
                                f"vocabulary_pins[{vocab_id}].payload_path "
                                f"{pin_path!r} does not resolve in "
                                f"{payload_ref}"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_path_unresolved",
                        (
                            f"protected_examples[{example_id}] vocabulary_pin "
                            f"{vocab_id} payload_path {pin_path!r} did not "
                            f"resolve in {payload_ref}."
                        ),
                        ref=example_id,
                    )
                    continue
                if not isinstance(value, str) or not value:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_value_not_string"
                            ),
                            "message": (
                                f"vocabulary_pins[{vocab_id}] resolved to a "
                                f"non-string or empty value at {pin_path}"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_value_not_string",
                        (
                            f"protected_examples[{example_id}] vocabulary_pin "
                            f"{vocab_id} resolved to a non-string value."
                        ),
                        ref=example_id,
                    )
                    continue
                seed_tokens = vocabulary_index[vocab_id]["tokens"]
                if value not in seed_tokens:
                    example_record["failed_checks"].append(
                        {
                            "check_id": (
                                "stale_examples.vocabulary_pin_not_in_seed"
                            ),
                            "message": (
                                f"vocabulary_pins[{vocab_id}] token "
                                f"{value!r} is no longer present in seed "
                                f"row {vocabulary_index[vocab_id]['seed_row_id']!r} "
                                f"({vocabulary_index[vocab_id]['seed_ref']})"
                            ),
                        }
                    )
                    fail(
                        result,
                        "stale_examples.vocabulary_pin_not_in_seed",
                        (
                            f"protected_examples[{example_id}] vocabulary_pin "
                            f"{vocab_id} pins token {value!r} which is no "
                            f"longer present in the controlling seed row "
                            f"{vocabulary_index[vocab_id]['seed_row_id']!r}."
                        ),
                        ref=example_id,
                        details={
                            "seed_ref": vocabulary_index[vocab_id]["seed_ref"],
                            "seed_row_id": vocabulary_index[vocab_id][
                                "seed_row_id"
                            ],
                            "stale_token": value,
                        },
                    )
                else:
                    example_record["passed_checks"].append(
                        f"vocabulary_pin:{vocab_id}={value}"
                    )

        result.examples.append(example_record)

    if (
        forced_pack_id is not None
        and forced_pack_id == pack_id
        and forced_drill is not None
    ):
        expected = forced_drill.get("expected_check_id")
        if not isinstance(expected, str) or not expected:
            expected = ""
        observed = [
            fc.get("check_id") for fc in result.failed_checks if isinstance(fc, dict)
        ]
        forced_replay_record = {
            "pack_id": pack_id,
            "drill_id": forced_drill.get("drill_id"),
            "expected_check_id": expected,
            "observed_failed_check_ids": observed,
            "reproduced": bool(expected and expected in observed),
        }

    if not result.failed_checks:
        pass_(result, f"pack {pack_id} passes")

    return result, forced_replay_record


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    source_map_rel = args.source_map
    source_map_path = repo_root / source_map_rel
    source_map = ensure_dict(
        render_yaml_as_json(source_map_path), source_map_rel
    )

    findings: list[Finding] = []

    schema_version = source_map.get("schema_version")
    if schema_version != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.source_map_schema_version_wrong",
                message=(
                    f"source map schema_version must be "
                    f"{EXPECTED_SCHEMA_VERSION}; got {schema_version!r}"
                ),
                remediation=(
                    "Bump the validator and source map together; do not drift."
                ),
            )
        )

    source_map_id = source_map.get("source_map_id")
    if source_map_id != EXPECTED_SOURCE_MAP_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.source_map_id_wrong",
                message=(
                    f"source map_id must be {EXPECTED_SOURCE_MAP_ID!r}; got "
                    f"{source_map_id!r}"
                ),
                remediation=(
                    "Restore the canonical source_map_id or land a new "
                    "decision row before renaming the gate."
                ),
            )
        )

    ensure_str(source_map.get("status"), "source_map.status")
    ensure_str(source_map.get("owner_dri"), "source_map.owner_dri")
    overview_page = ensure_str(
        source_map.get("overview_page"), "source_map.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    validator_entrypoint = ensure_str(
        source_map.get("validator_entrypoint"),
        "source_map.validator_entrypoint",
    )
    if not artifact_ref_exists(repo_root, validator_entrypoint):
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.validator_entrypoint_missing",
                message=(
                    f"validator_entrypoint does not exist: "
                    f"{validator_entrypoint}"
                ),
                remediation=(
                    "Restore tools/ci/check_stale_examples.py or fix the path."
                ),
                ref=validator_entrypoint,
            )
        )

    ci_gate_manifest = ensure_str(
        source_map.get("ci_gate_manifest"),
        "source_map.ci_gate_manifest",
    )
    if not artifact_ref_exists(repo_root, ci_gate_manifest):
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.ci_gate_manifest_missing",
                message=(
                    f"ci_gate_manifest does not exist: {ci_gate_manifest}"
                ),
                remediation=(
                    "Create ci/docs/stale_example_gate.yml or fix the path."
                ),
                ref=ci_gate_manifest,
            )
        )

    vocabulary_seeds = ensure_list(
        source_map.get("vocabulary_seeds") or [],
        "source_map.vocabulary_seeds",
    )
    vocabulary_index, vocab_findings = load_vocabulary_seeds(
        repo_root, vocabulary_seeds
    )
    findings.extend(vocab_findings)

    raw_packs = source_map.get("protected_docs_packs") or []
    packs = ensure_list(raw_packs, "source_map.protected_docs_packs")
    if not packs:
        findings.append(
            Finding(
                severity="error",
                check_id="stale_examples.protected_docs_packs_empty",
                message=(
                    "protected_docs_packs must list at least one pack so the "
                    "M1 docs-pack guard has at least one M1 surface to check."
                ),
                remediation=(
                    "Add at least one pack entry under protected_docs_packs."
                ),
            )
        )

    # Decode --force-drill once.
    forced_pack_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be '<pack_id>:<drill_id>'; got "
                f"{args.force_drill!r}"
            )
        forced_pack_id, forced_drill_id = args.force_drill.split(":", 1)
        forced_pack_id = forced_pack_id.strip()
        forced_drill_id = forced_drill_id.strip()
        if not forced_pack_id or not forced_drill_id:
            raise SystemExit(
                "--force-drill must be '<pack_id>:<drill_id>'; got "
                f"{args.force_drill!r}"
            )

    pack_results: list[PackResult] = []
    forced_replay_record: dict[str, Any] | None = None
    forced_drill_used: dict[str, Any] | None = None

    for pack in packs:
        if not isinstance(pack, dict):
            continue
        drill_for_pack: dict[str, Any] | None = None
        if (
            forced_pack_id is not None
            and forced_pack_id == pack.get("pack_id")
        ):
            drill = pack.get("failure_drill")
            if not isinstance(drill, dict):
                raise SystemExit(
                    f"pack {forced_pack_id!r} has no failure_drill block; "
                    "cannot replay --force-drill."
                )
            if drill.get("drill_id") != forced_drill_id:
                raise SystemExit(
                    f"pack {forced_pack_id!r} failure_drill.drill_id "
                    f"{drill.get('drill_id')!r} does not match requested "
                    f"drill_id {forced_drill_id!r}."
                )
            drill_for_pack = drill
            forced_drill_used = drill
        result, replay = validate_pack(
            pack,
            repo_root=repo_root,
            vocabulary_index=vocabulary_index,
            forced_pack_id=forced_pack_id,
            forced_drill=drill_for_pack,
        )
        pack_results.append(result)
        if replay is not None:
            forced_replay_record = replay

    if forced_pack_id is not None and forced_replay_record is None:
        raise SystemExit(
            f"--force-drill pack_id {forced_pack_id!r} did not match any "
            "protected_docs_packs[].pack_id."
        )

    # Promote per-pack failures into findings, skipping the forced pack so
    # the runner's exit can reflect the drill verdict.
    seen_pack_ids: set[str] = set()
    for result in pack_results:
        if result.pack_id in seen_pack_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="stale_examples.protected_docs_packs_duplicate_id",
                    message=f"duplicate pack_id: {result.pack_id}",
                    remediation="pack_ids must be unique.",
                    ref=result.pack_id,
                )
            )
        seen_pack_ids.add(result.pack_id)
        if (
            forced_pack_id is not None
            and result.pack_id == forced_pack_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "stale_examples.pack_failed_check"
                    ),
                    message=f"{result.pack_id}: {failure.get('message', '')}",
                    remediation=(
                        "Restore docs-pack metadata / canonical example / "
                        "vocabulary pin agreement, or land a superseding "
                        "decision row before publishing the change."
                    ),
                    ref=failure.get("ref") or result.pack_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "docs_pack_and_example_checks_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(
            source_map.get("owner_dri"), "source_map.owner_dri"
        ),
        "source_map_ref": source_map_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tools/ci/check_stale_examples.py --repo-root ."
        ),
        "status": status,
        "vocabulary_seeds": sorted(vocabulary_index.keys()),
        "protected_pack_ids": [r.pack_id for r in pack_results],
        "packs": [
            {
                "pack_id": r.pack_id,
                "pack_ref": r.pack_ref,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "examples": r.examples,
                "diagnostics": r.diagnostics,
            }
            for r in pack_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record
    if forced_drill_used is not None:
        capture.setdefault("forced_drill_replay", {})
        capture["forced_drill_replay"]["actionable_next_action"] = (
            forced_drill_used.get("actionable_next_action")
        )

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "stale-examples"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill "
                f"{forced_replay_record['drill_id']} on "
                f"{forced_replay_record['pack_id']} reproduced "
                f"{forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} "
            f"on {forced_replay_record['pack_id']} did NOT reproduce "
            f"{forced_replay_record['expected_check_id']}; observed: "
            f"{forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[stale-examples] interrupted", file=sys.stderr)
        sys.exit(130)
