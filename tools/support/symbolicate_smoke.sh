#!/usr/bin/env bash
#
# Exact-build symbolication smoke runner.
#
# Resolves one synthetic crash envelope through the exact-build identity
# fixtures, validates the native and renderer sidecars, and emits a
# deterministic symbolication report. The path fails closed on any
# exact-build mismatch; it never guesses or falls back to a nearby build.
#
# Usage:
#   ./tools/support/symbolicate_smoke.sh [--crash-envelope PATH] [--manifest PATH] [--retention-seed PATH] [--out-dir PATH]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

CRASH_ENVELOPE="${REPO_ROOT}/fixtures/support/crash_fixture/crash_envelope.json"
MANIFEST="${REPO_ROOT}/fixtures/support/crash_fixture/symbolication_input_manifest.json"
RETENTION_SEED="${REPO_ROOT}/artifacts/support/crash_artifact_retention_seed.json"
OUT_DIR="${REPO_ROOT}/target/symbolication-smoke"

usage() {
  sed -n '2,15p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --crash-envelope)
      CRASH_ENVELOPE="${2:-}"
      shift 2
      ;;
    --crash-envelope=*)
      CRASH_ENVELOPE="${1#--crash-envelope=}"
      shift
      ;;
    --manifest)
      MANIFEST="${2:-}"
      shift 2
      ;;
    --manifest=*)
      MANIFEST="${1#--manifest=}"
      shift
      ;;
    --retention-seed)
      RETENTION_SEED="${2:-}"
      shift 2
      ;;
    --retention-seed=*)
      RETENTION_SEED="${1#--retention-seed=}"
      shift
      ;;
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --out-dir=*)
      OUT_DIR="${1#--out-dir=}"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "symbolicate_smoke: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

mkdir -p "${OUT_DIR}"
REPORT_PATH="${OUT_DIR}/symbolication_report.json"

python3 - "${REPO_ROOT}" "${CRASH_ENVELOPE}" "${MANIFEST}" "${RETENTION_SEED}" "${REPORT_PATH}" <<'PY'
import json
import sys
from pathlib import Path


repo_root = Path(sys.argv[1]).resolve()
crash_envelope_path = Path(sys.argv[2]).resolve()
manifest_path = Path(sys.argv[3]).resolve()
retention_seed_path = Path(sys.argv[4]).resolve()
report_path = Path(sys.argv[5]).resolve()


def load_json(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def rel(path: Path) -> str:
    try:
        return path.resolve().relative_to(repo_root).as_posix()
    except ValueError:
        return str(path.resolve())


def resolve_repo_path(path_str: str) -> Path:
    return (repo_root / path_str).resolve()


def get_path(data: dict, path: str):
    current = data
    for part in path.split("."):
        current = current[part]
    return current


def first_with_prefix(values, prefix):
    for value in values:
        if value.startswith(prefix):
            return value
    return None


def fail(message: str) -> None:
    print(f"symbolicate_smoke: error: {message}", file=sys.stderr)
    raise SystemExit(1)


crash_envelope = load_json(crash_envelope_path)
manifest = load_json(manifest_path)
retention_seed = load_json(retention_seed_path)

exact_build_records: dict[str, dict] = {}
for key, repo_rel in manifest["exact_build_record_paths"].items():
    exact_build_records[key] = load_json(resolve_repo_path(repo_rel))

runtime_identity = exact_build_records["runtime_identity"]
debug_symbols_identity = exact_build_records["debug_symbols_identity"]
source_map_identity = exact_build_records["source_map_identity"]
crash_symbols_archive_identity = exact_build_records["crash_symbols_archive_identity"]

if crash_envelope["primary_exact_build_identity_ref"] != runtime_identity["exact_build_identity_ref"]:
    fail(
        "crash envelope primary_exact_build_identity_ref does not match the runtime exact-build identity"
    )

for field_path in manifest["build_match_fields"]:
    envelope_value = get_path(crash_envelope["exact_build_snapshot"], field_path)
    runtime_value = get_path(runtime_identity, field_path)
    if envelope_value != runtime_value:
        fail(
            f"exact-build mismatch on {field_path}: envelope={envelope_value!r} runtime={runtime_value!r}"
        )

for sibling_key in (
    "debug_symbols_identity",
    "source_map_identity",
    "crash_symbols_archive_identity",
):
    sibling = exact_build_records[sibling_key]
    for field_path in manifest["build_match_fields"]:
        runtime_value = get_path(runtime_identity, field_path)
        sibling_value = get_path(sibling, field_path)
        if sibling_value != runtime_value:
            fail(
                f"exact-build mismatch on {field_path}: {sibling_key}={sibling_value!r} runtime={runtime_value!r}"
            )

if runtime_identity["evidence"]["split_symbols_ref"] != debug_symbols_identity["exact_build_identity_ref"]:
    fail("runtime exact-build identity does not point at the expected debug-symbols identity")

if runtime_identity["evidence"]["source_map_manifest_ref"] != source_map_identity["exact_build_identity_ref"]:
    fail("runtime exact-build identity does not point at the expected source-map identity")

crash_dump_manifest = load_json(resolve_repo_path(manifest["crash_dump_manifest_path"]))
if crash_dump_manifest["primary_exact_build_identity_ref"] != runtime_identity["exact_build_identity_ref"]:
    fail("crash dump manifest does not resolve to the same primary exact-build identity")

module_ids = [module["module_id"] for module in crash_envelope["modules"]]
if sorted(crash_dump_manifest["module_refs"]) != sorted(module_ids):
    fail("crash dump manifest module_refs do not match the crash envelope modules")

if crash_dump_manifest["support_bundle_ref"] != manifest["support_bundle_ref"]:
    fail("crash dump manifest support_bundle_ref does not match the symbolication manifest")

bindings_by_module_id = {binding["module_id"]: binding for binding in manifest["module_bindings"]}
module_results = []

for module in crash_envelope["modules"]:
    binding = bindings_by_module_id.get(module["module_id"])
    if binding is None:
        fail(f"no module binding exists for {module['module_id']}")
    if module["module_kind"] != binding["module_kind"]:
        fail(
            f"module kind mismatch for {module['module_id']}: envelope={module['module_kind']!r} binding={binding['module_kind']!r}"
        )

    expected_identity = exact_build_records[binding["expected_envelope_identity_key"]]
    if module["exact_build_identity_ref"] != expected_identity["exact_build_identity_ref"]:
        fail(
            f"module {module['module_id']} points at {module['exact_build_identity_ref']!r} instead of {expected_identity['exact_build_identity_ref']!r}"
        )

    symbolication_identity = exact_build_records[binding["symbolication_identity_key"]]
    module_build_id = get_path(module, binding["module_build_id_field"])
    matched_symbol_tag = first_with_prefix(
        symbolication_identity["propagation"]["symbol_tag_refs"],
        binding["required_symbol_tag_prefix"],
    )
    if matched_symbol_tag is None:
        fail(
            f"module {module['module_id']} could not find a symbol tag with prefix {binding['required_symbol_tag_prefix']!r}"
        )
    expected_symbol_tag = f"{binding['required_symbol_tag_prefix']}{module_build_id}"
    if matched_symbol_tag != expected_symbol_tag:
        fail(
            f"module {module['module_id']} symbol tag mismatch: expected {expected_symbol_tag!r} got {matched_symbol_tag!r}"
        )

    support_archive_identity_ref = None
    if binding["module_kind"] == "native_binary":
        runtime_symbol_tag = first_with_prefix(
            runtime_identity["propagation"]["symbol_tag_refs"],
            binding["required_symbol_tag_prefix"],
        )
        if runtime_symbol_tag != expected_symbol_tag:
            fail(
                f"runtime binary symbol tag mismatch for {module['module_id']}: expected {expected_symbol_tag!r} got {runtime_symbol_tag!r}"
            )

        archive_identity = exact_build_records[binding["archive_identity_key"]]
        archive_symbol_tag = first_with_prefix(
            archive_identity["propagation"]["symbol_tag_refs"],
            binding["required_archive_symbol_tag_prefix"],
        )
        expected_archive_tag = f"{binding['required_archive_symbol_tag_prefix']}{module_build_id}"
        if archive_symbol_tag != expected_archive_tag:
            fail(
                f"crash archive symbol tag mismatch for {module['module_id']}: expected {expected_archive_tag!r} got {archive_symbol_tag!r}"
            )
        support_archive_identity_ref = archive_identity["exact_build_identity_ref"]

    frame_count = binding["report_frame_count"]
    resolved_frame_summary = [
        f"{frame['frame_index']} {frame['symbol_hint']}"
        for frame in module["faulting_frames"][:frame_count]
    ]

    module_results.append(
        {
            "module_id": module["module_id"],
            "module_kind": module["module_kind"],
            "mapping_state": "exact",
            "runtime_identity_ref": runtime_identity["exact_build_identity_ref"],
            "symbolication_identity_ref": symbolication_identity["exact_build_identity_ref"],
            "support_archive_identity_ref": support_archive_identity_ref,
            "matched_symbol_tag": matched_symbol_tag,
            "resolved_frame_summary": resolved_frame_summary,
        }
    )

retention_rows_by_id = {row["artifact_class_id"]: row for row in retention_seed["rows"]}
retention_rows = []
for row_id in manifest["retention_row_ids"]:
    row = retention_rows_by_id.get(row_id)
    if row is None:
        fail(f"retention row {row_id!r} is missing from the retention seed")
    retention_rows.append(
        {
            "artifact_class_id": row["artifact_class_id"],
            "record_class_id": row["record_class_id"],
            "default_data_class": row["default_data_class"],
            "default_redaction_class": row["default_redaction_class"],
            "default_storage_mode": row["default_storage_mode"],
            "managed_retention_rule": row["managed_retention_rule"],
        }
    )

report = {
    "schema_version": 1,
    "record_kind": "symbolication_smoke_report",
    "fixture_id": manifest["fixture_id"],
    "symbolication_report_ref": manifest["symbolication_report_ref"],
    "generated_at": manifest["report_generated_at"],
    "crash_envelope_ref": crash_envelope["crash_envelope_ref"],
    "primary_exact_build_identity_ref": runtime_identity["exact_build_identity_ref"],
    "result_state": "exact_match",
    "build_match_fields_checked": manifest["build_match_fields"],
    "module_results": module_results,
    "crash_dump_ref": crash_dump_manifest["crash_dump_ref"],
    "support_bundle_ref": manifest["support_bundle_ref"],
    "release_evidence_packet_ref": manifest["release_evidence_packet_ref"],
    "claim_row_refs": manifest["claim_row_refs"],
    "retention_seed_ref": rel(retention_seed_path),
    "retention_rows": retention_rows,
    "notes": "Exact-build and module identity matched for both the native shell and renderer bundle; the smoke report stays redaction-safe and points back to the same support-bundle and release-evidence refs the wider supportability lane already uses.",
}

report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
print(f"symbolicate_smoke: exact-build match across {len(module_results)} modules")
print(f"symbolicate_smoke: wrote {rel(report_path)}")
PY
