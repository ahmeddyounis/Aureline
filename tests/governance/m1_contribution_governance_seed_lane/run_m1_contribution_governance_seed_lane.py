#!/usr/bin/env python3
"""Unattended M1 contribution-governance seed validation lane.

Replays every row in
``artifacts/governance/contribution_governance_seed.yaml`` against:

- ``schemas/governance/contribution_governance_seed.schema.json`` —
  the seed envelope schema (vocabularies, required coverage, row
  list);
- ``schemas/governance/contribution_governance_seed_row.schema.json``
  — the row vocabulary;
- the row's canonical artifact (resolved on disk, scanned for the
  declared ``canonical_artifact_marker`` literal substring);
- every supporting artifact (resolved on disk);
- the row's canonical envelope example under
  ``fixtures/governance/m1_contribution_governance_examples/``; and
- the named runtime consumer.

Per-row assertions:

- ``control_id`` is unique and matches its ``control_class`` through
  the closed prefix table (signoff_dco -> signoff., license_metadata
  -> license., third_party_import_record -> import_record.,
  public_interface_versioning -> versioning.,
  deprecation_packet_template -> deprecation.,
  repo_hygiene_scaffold -> repo_hygiene.);
- closed vocabularies are honoured (``control_class``,
  ``artifact_kind_class``, ``enforcement_class``,
  ``lifecycle_state_class``,
  ``contribution_governance_consumer_class``);
- the envelope's closed vocabularies agree with the row schema's
  ``$defs`` (no drift);
- the row's ``artifact_kind_class`` agrees with ``control_class``
  through the envelope's
  ``control_class_to_artifact_kind_class`` closed map;
- the row's ``enforcement_class`` agrees with ``control_class``
  through the envelope's
  ``control_class_to_enforcement_class`` closed map;
- the row's ``canonical_artifact_ref`` resolves on disk and its text
  contains the row's ``canonical_artifact_marker``;
- every ``supporting_artifact_ref`` resolves on disk;
- ``named_runtime_consumer.consumer_ref`` resolves on disk,
  ``consumer_class`` is in the closed vocabulary, and
  ``consumed_fields`` is non-empty;
- the canonical envelope example carries
  ``registry_example_kind: contribution_governance_seed_envelope_example``,
  the row's ``control_id``, the row's ``canonical_artifact_ref``, the
  row's ``control_class``, the row's ``artifact_kind_class``, the
  row's ``enforcement_class``, and the row's ``lifecycle_state_class``;
- the row's failure drill is listed in
  ``failure_drill_id_vocabulary``;
- the matrix covers every entry in ``required_control_class_coverage``.

``--force-drill <control_id>:<drill_id>`` replays the named drill on
the named row and exits 0 only when the runner reproduces the
declared ``expected_check_id``. Drift in the unforced rows still
fails the lane.

YAML decoding follows the repository convention: matrix and fixture
files are parsed via Ruby/Psych so this script does not require a
third-party Python YAML dependency.
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


DEFAULT_MATRIX_REL = "artifacts/governance/contribution_governance_seed.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/contribution_governance_seed.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/contribution_governance_seed_row.schema.json"
)
DEFAULT_IMPORT_RECORD_REL = "artifacts/governance/import_record_seed.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "contribution_governance_seed_validation_capture.json"
)

EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_RECORD_KIND = "m1_contribution_governance_seed_row"
EXPECTED_REGISTRY_EXAMPLE_KIND = "contribution_governance_seed_envelope_example"

# Closed mapping control_class -> required control_id prefix.
CONTROL_TO_PREFIX = {
    "signoff_dco": "signoff.",
    "license_metadata": "license.",
    "third_party_import_record": "import_record.",
    "public_interface_versioning": "versioning.",
    "deprecation_packet_template": "deprecation.",
    "repo_hygiene_scaffold": "repo_hygiene.",
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


@dataclass
class RowResult:
    row_id: str
    control_class: str
    artifact_kind_class: str
    enforcement_class: str
    lifecycle_state_class: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Seed YAML path, repo-relative.",
    )
    parser.add_argument(
        "--envelope-schema",
        default=DEFAULT_ENVELOPE_SCHEMA_REL,
        help="Envelope schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--row-schema",
        default=DEFAULT_ROW_SCHEMA_REL,
        help="Row schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--import-record-register",
        default=DEFAULT_IMPORT_RECORD_REL,
        help="Import-record register YAML path, repo-relative.",
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
            "Replay a named failure drill on a named row in the form "
            "'<control_id>:<drill_id>'. The runner exits 0 only when the "
            "row's failure drill reproduces the exact expected_check_id."
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
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_schema_enums(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    """Best-effort enum lookup from a schema's $defs."""
    schema_path = repo_root / ref
    if not schema_path.exists():
        return []
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def apply_forced_overrides(
    row: dict[str, Any],
    example: dict[str, Any] | None,
    forced_overrides: dict[str, Any],
) -> tuple[dict[str, Any], dict[str, Any] | None]:
    row = copy.deepcopy(row)
    example = copy.deepcopy(example) if example is not None else None

    if not forced_overrides:
        return row, example

    if "rewrite_enforcement_class" in forced_overrides:
        row["enforcement_class"] = forced_overrides["rewrite_enforcement_class"]

    if "rewrite_artifact_kind_class" in forced_overrides:
        row["artifact_kind_class"] = forced_overrides[
            "rewrite_artifact_kind_class"
        ]

    if "rewrite_lifecycle_state_class" in forced_overrides:
        row["lifecycle_state_class"] = forced_overrides[
            "rewrite_lifecycle_state_class"
        ]

    if forced_overrides.get("clear_canonical_artifact_marker"):
        row["canonical_artifact_marker"] = "__drill_cleared_marker_must_not_appear__"

    if (
        "rewrite_example_pinned_canonical_artifact_ref" in forced_overrides
        and example is not None
    ):
        example["pinned_canonical_artifact_ref"] = forced_overrides[
            "rewrite_example_pinned_canonical_artifact_ref"
        ]

    if forced_overrides.get("clear_named_runtime_consumer_consumed_fields"):
        consumer = row.get("named_runtime_consumer")
        if isinstance(consumer, dict):
            consumer["consumed_fields"] = []

    return row, example


def validate_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    row_id_value: str,
    control_class_vocab: set[str],
    artifact_kind_class_vocab: set[str],
    enforcement_class_vocab: set[str],
    lifecycle_state_class_vocab: set[str],
    consumer_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
    control_class_to_artifact_kind_class: dict[str, str],
    control_class_to_enforcement_class: dict[str, str],
    example_override: dict[str, Any] | None,
) -> RowResult:
    control_id = ensure_str(row.get("control_id"), f"{row_id_value}.control_id")
    control_class = ensure_str(
        row.get("control_class"), f"{row_id_value}.control_class"
    )
    artifact_kind_class = ensure_str(
        row.get("artifact_kind_class"),
        f"{row_id_value}.artifact_kind_class",
    )
    enforcement_class = ensure_str(
        row.get("enforcement_class"),
        f"{row_id_value}.enforcement_class",
    )
    lifecycle_state_class = ensure_str(
        row.get("lifecycle_state_class"),
        f"{row_id_value}.lifecycle_state_class",
    )

    result = RowResult(
        row_id=control_id,
        control_class=control_class,
        artifact_kind_class=artifact_kind_class,
        enforcement_class=enforcement_class,
        lifecycle_state_class=lifecycle_state_class,
    )

    # --- discriminator and version pins -----------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "contribution_governance.row.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("contribution_governance_seed_row_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "contribution_governance.row.schema_version_wrong",
            (
                "contribution_governance_seed_row_schema_version must be"
                f" {EXPECTED_ROW_SCHEMA_VERSION}; got"
                f" {row.get('contribution_governance_seed_row_schema_version')!r}"
            ),
        )

    # --- closed vocabularies ---------------------------------------------
    if control_class not in control_class_vocab:
        fail(
            result,
            "contribution_governance.row.control_class_unknown",
            (
                f"control_class {control_class!r} is not in"
                " control_class_vocabulary"
            ),
        )
    if artifact_kind_class not in artifact_kind_class_vocab:
        fail(
            result,
            "contribution_governance.row.artifact_kind_class_unknown",
            (
                f"artifact_kind_class {artifact_kind_class!r} is not"
                " in artifact_kind_class_vocabulary"
            ),
        )
    if enforcement_class not in enforcement_class_vocab:
        fail(
            result,
            "contribution_governance.row.enforcement_class_unknown",
            (
                f"enforcement_class {enforcement_class!r} is not in"
                " enforcement_class_vocabulary"
            ),
        )
    if lifecycle_state_class not in lifecycle_state_class_vocab:
        fail(
            result,
            "contribution_governance.row.lifecycle_state_class_unknown",
            (
                f"lifecycle_state_class {lifecycle_state_class!r} is not"
                " in lifecycle_state_class_vocabulary"
            ),
        )

    # --- control_id prefix matches control_class -------------------------
    prefix = CONTROL_TO_PREFIX.get(control_class)
    if prefix and not control_id.startswith(prefix):
        fail(
            result,
            "contribution_governance.row.control_id_prefix_mismatch",
            (
                f"control_id {control_id!r} must start with {prefix!r}"
                f" for control_class {control_class!r}"
            ),
        )

    # --- closed maps agreement ------------------------------------------
    expected_artifact_kind = control_class_to_artifact_kind_class.get(control_class)
    if (
        expected_artifact_kind is not None
        and artifact_kind_class != expected_artifact_kind
    ):
        fail(
            result,
            "contribution_governance.artifact_kind_class_disagrees_with_control_class",
            (
                f"artifact_kind_class {artifact_kind_class!r} disagrees"
                f" with control_class {control_class!r} which requires"
                f" {expected_artifact_kind!r} (see"
                " control_class_to_artifact_kind_class)"
            ),
        )

    expected_enforcement = control_class_to_enforcement_class.get(control_class)
    if (
        expected_enforcement is not None
        and enforcement_class != expected_enforcement
    ):
        fail(
            result,
            "contribution_governance.enforcement_class_disagrees_with_control_class",
            (
                f"enforcement_class {enforcement_class!r} disagrees with"
                f" control_class {control_class!r} which requires"
                f" {expected_enforcement!r} (see"
                " control_class_to_enforcement_class)"
            ),
        )

    # --- canonical artifact resolves and carries the marker --------------
    canonical_artifact_ref = ensure_str(
        row.get("canonical_artifact_ref"),
        f"{control_id}.canonical_artifact_ref",
    )
    canonical_artifact_marker = ensure_str(
        row.get("canonical_artifact_marker"),
        f"{control_id}.canonical_artifact_marker",
    )
    canonical_artifact_path = repo_root / canonical_artifact_ref
    if not canonical_artifact_path.exists():
        fail(
            result,
            "contribution_governance.canonical_artifact_ref_missing",
            (
                f"canonical_artifact_ref does not exist on disk: "
                f"{canonical_artifact_ref}"
            ),
        )
    else:
        try:
            canonical_text = canonical_artifact_path.read_text(
                encoding="utf-8", errors="replace"
            )
        except OSError as exc:
            fail(
                result,
                "contribution_governance.canonical_artifact_ref_unreadable",
                (
                    f"canonical_artifact_ref {canonical_artifact_ref}"
                    f" could not be read: {exc}"
                ),
            )
            canonical_text = ""
        if canonical_artifact_marker not in canonical_text:
            fail(
                result,
                "contribution_governance.canonical_artifact_marker_missing",
                (
                    "canonical_artifact_marker"
                    f" {canonical_artifact_marker!r} was not found in"
                    f" {canonical_artifact_ref}; the seed cannot prove"
                    " the row points at the right document"
                ),
            )

    # --- supporting artifacts resolve ------------------------------------
    supporting = ensure_list(
        row.get("supporting_artifact_refs", []),
        f"{control_id}.supporting_artifact_refs",
    )
    for idx, sup in enumerate(supporting):
        sup_ref = ensure_str(sup, f"{control_id}.supporting_artifact_refs[{idx}]")
        if not artifact_ref_exists(repo_root, sup_ref):
            fail(
                result,
                "contribution_governance.supporting_artifact_ref_missing",
                (
                    f"supporting_artifact_refs[{idx}] does not exist on"
                    f" disk: {sup_ref}"
                ),
            )

    # --- named runtime consumer -----------------------------------------
    named_consumer = ensure_dict(
        row.get("named_runtime_consumer"),
        f"{control_id}.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        f"{control_id}.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        fail(
            result,
            "contribution_governance.named_runtime_consumer_missing",
            (
                "named_runtime_consumer.consumer_ref does not exist:"
                f" {consumer_ref}"
            ),
        )
    consumer_class = ensure_str(
        named_consumer.get("consumer_class"),
        f"{control_id}.named_runtime_consumer.consumer_class",
    )
    if consumer_class not in consumer_class_vocab:
        fail(
            result,
            "contribution_governance.named_runtime_consumer_consumer_class_unknown",
            (
                f"named_runtime_consumer.consumer_class {consumer_class!r}"
                " is not in contribution_governance_consumer_class_vocabulary"
            ),
        )
    consumed_fields = named_consumer.get("consumed_fields")
    if not isinstance(consumed_fields, list) or not consumed_fields:
        fail(
            result,
            "contribution_governance.named_runtime_consumer_consumed_fields_empty",
            (
                "named_runtime_consumer.consumed_fields must declare at"
                " least one field"
            ),
        )

    # --- example payload --------------------------------------------------
    example_ref = ensure_str(
        row.get("example_payload_ref"), f"{control_id}.example_payload_ref"
    )
    example_path = repo_root / example_ref
    if example_override is not None:
        example_doc: dict[str, Any] | None = example_override
    elif not example_path.exists():
        fail(
            result,
            "contribution_governance.example_payload_missing",
            f"example_payload_ref does not exist: {example_ref}",
        )
        example_doc = None
    else:
        try:
            example_doc = json.loads(example_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            fail(
                result,
                "contribution_governance.example_payload_invalid_json",
                (
                    f"example_payload_ref {example_ref} is not valid"
                    f" JSON: {exc}"
                ),
            )
            example_doc = None

    if example_doc is not None:
        if (
            example_doc.get("registry_example_kind")
            != EXPECTED_REGISTRY_EXAMPLE_KIND
        ):
            fail(
                result,
                "contribution_governance.example_payload_kind_wrong",
                (
                    "example_payload.registry_example_kind must be"
                    f" {EXPECTED_REGISTRY_EXAMPLE_KIND!r}; got"
                    f" {example_doc.get('registry_example_kind')!r}"
                ),
            )
        if (
            example_doc.get("schema_registry_row_control_id")
            != control_id
        ):
            fail(
                result,
                "contribution_governance.example_payload_control_id_mismatch",
                (
                    "example_payload.schema_registry_row_control_id must"
                    f" match the row's control_id {control_id!r}; got"
                    f" {example_doc.get('schema_registry_row_control_id')!r}"
                ),
            )
        if (
            example_doc.get("pinned_canonical_artifact_ref")
            != canonical_artifact_ref
        ):
            fail(
                result,
                "contribution_governance.example_payload_pinned_canonical_artifact_ref_mismatch",
                (
                    "example_payload.pinned_canonical_artifact_ref must"
                    f" equal the row's canonical_artifact_ref"
                    f" {canonical_artifact_ref!r}; got"
                    f" {example_doc.get('pinned_canonical_artifact_ref')!r}"
                ),
            )
        if example_doc.get("pinned_control_class") != control_class:
            fail(
                result,
                "contribution_governance.example_payload_control_class_mismatch",
                (
                    "example_payload.pinned_control_class must equal the"
                    f" row's control_class {control_class!r}; got"
                    f" {example_doc.get('pinned_control_class')!r}"
                ),
            )
        if (
            example_doc.get("pinned_artifact_kind_class") != artifact_kind_class
        ):
            fail(
                result,
                "contribution_governance.example_payload_artifact_kind_class_mismatch",
                (
                    "example_payload.pinned_artifact_kind_class must"
                    f" equal the row's artifact_kind_class"
                    f" {artifact_kind_class!r}; got"
                    f" {example_doc.get('pinned_artifact_kind_class')!r}"
                ),
            )
        if example_doc.get("pinned_enforcement_class") != enforcement_class:
            fail(
                result,
                "contribution_governance.example_payload_enforcement_class_mismatch",
                (
                    "example_payload.pinned_enforcement_class must equal"
                    f" the row's enforcement_class {enforcement_class!r};"
                    f" got {example_doc.get('pinned_enforcement_class')!r}"
                ),
            )
        if (
            example_doc.get("pinned_lifecycle_state_class")
            != lifecycle_state_class
        ):
            fail(
                result,
                "contribution_governance.example_payload_lifecycle_state_class_mismatch",
                (
                    "example_payload.pinned_lifecycle_state_class must"
                    f" equal the row's lifecycle_state_class"
                    f" {lifecycle_state_class!r}; got"
                    f" {example_doc.get('pinned_lifecycle_state_class')!r}"
                ),
            )

    # --- failure-drill shape ---------------------------------------------
    drill = ensure_dict(row.get("failure_drill"), f"{control_id}.failure_drill")
    drill_id = ensure_str(
        drill.get("drill_id"), f"{control_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "contribution_governance.failure_drill_id_unknown",
            (
                f"failure_drill.drill_id {drill_id!r} is not in"
                " failure_drill_id_vocabulary"
            ),
        )
    forced_input = ensure_dict(
        drill.get("forced_input"),
        f"{control_id}.failure_drill.forced_input",
    )
    if not forced_input:
        fail(
            result,
            "contribution_governance.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )
    ensure_str(
        drill.get("expected_check_id"),
        f"{control_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        drill.get("actionable_next_action"),
        f"{control_id}.failure_drill.actionable_next_action",
    )

    result.diagnostics.update(
        {
            "control_id": control_id,
            "control_class": control_class,
            "artifact_kind_class": artifact_kind_class,
            "enforcement_class": enforcement_class,
            "lifecycle_state_class": lifecycle_state_class,
            "canonical_artifact_ref": canonical_artifact_ref,
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": drill.get("expected_check_id"),
            },
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {control_id} passes")

    return result


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix = ensure_dict(render_yaml_as_json(repo_root / matrix_rel), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="contribution_governance.envelope_schema_version_wrong",
                message=(
                    f"matrix schema_version must be 1; got {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != "m1_contribution_governance_seed":
        findings.append(
            Finding(
                severity="error",
                check_id="contribution_governance.envelope_matrix_id_wrong",
                message=(
                    "matrix_id must be 'm1_contribution_governance_seed'; got"
                    f" {matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="contribution_governance.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    for key in (
        "row_schema_ref",
        "import_record_register_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"contribution_governance.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation="Fix the path or land the referenced artifact.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="contribution_governance.envelope_validation_lane_ref_missing",
                message=(
                    f"validation_lane_ref base does not exist:"
                    f" {validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    def load_vocab(key: str) -> set[str]:
        return {
            ensure_str(item, f"matrix.{key}[]")
            for item in ensure_list(matrix.get(key), f"matrix.{key}")
        }

    control_class_vocab = load_vocab("control_class_vocabulary")
    artifact_kind_class_vocab = load_vocab("artifact_kind_class_vocabulary")
    enforcement_class_vocab = load_vocab("enforcement_class_vocabulary")
    lifecycle_state_class_vocab = load_vocab("lifecycle_state_class_vocabulary")
    consumer_class_vocab = load_vocab(
        "contribution_governance_consumer_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_control_class_coverage = load_vocab(
        "required_control_class_coverage"
    )

    control_class_to_artifact_kind_class_raw = ensure_dict(
        matrix.get("control_class_to_artifact_kind_class"),
        "matrix.control_class_to_artifact_kind_class",
    )
    control_class_to_artifact_kind_class: dict[str, str] = {
        ensure_str(
            k, "matrix.control_class_to_artifact_kind_class.key"
        ): ensure_str(
            v, f"matrix.control_class_to_artifact_kind_class.{k}"
        )
        for k, v in control_class_to_artifact_kind_class_raw.items()
    }
    control_class_to_enforcement_class_raw = ensure_dict(
        matrix.get("control_class_to_enforcement_class"),
        "matrix.control_class_to_enforcement_class",
    )
    control_class_to_enforcement_class: dict[str, str] = {
        ensure_str(
            k, "matrix.control_class_to_enforcement_class.key"
        ): ensure_str(
            v, f"matrix.control_class_to_enforcement_class.{k}"
        )
        for k, v in control_class_to_enforcement_class_raw.items()
    }

    # Sanity: required maps must cover every required control class so the
    # row check can never be silently relaxed by deleting a map entry.
    for required in required_control_class_coverage:
        if required not in control_class_to_artifact_kind_class:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "contribution_governance."
                        "control_class_to_artifact_kind_class_missing_required"
                    ),
                    message=(
                        "control_class_to_artifact_kind_class is missing"
                        f" required control_class {required!r}"
                    ),
                    remediation=(
                        "Add the closed-map entry so the row check"
                        " cannot silently relax."
                    ),
                )
            )
        if required not in control_class_to_enforcement_class:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "contribution_governance."
                        "control_class_to_enforcement_class_missing_required"
                    ),
                    message=(
                        "control_class_to_enforcement_class is missing"
                        f" required control_class {required!r}"
                    ),
                    remediation=(
                        "Add the closed-map entry so the row check"
                        " cannot silently relax."
                    ),
                )
            )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )

    def assert_vocab_matches_schema(
        matrix_vocab: set[str], defs_key: str, name: str
    ) -> None:
        schema_enum = set(
            load_schema_enums(repo_root, row_schema_ref, defs_key)
        )
        if not schema_enum:
            return
        diff = matrix_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        f"contribution_governance.envelope_{name}_disagrees_with_row_schema"
                    ),
                    message=(
                        f"matrix.{name} disagrees with"
                        f" {row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(matrix_vocab - schema_enum)};"
                        f" schema-only: {sorted(schema_enum - matrix_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the"
                        " row schema; the schema is canonical."
                    ),
                )
            )

    assert_vocab_matches_schema(
        control_class_vocab, "control_class", "control_class_vocabulary"
    )
    assert_vocab_matches_schema(
        artifact_kind_class_vocab,
        "artifact_kind_class",
        "artifact_kind_class_vocabulary",
    )
    assert_vocab_matches_schema(
        enforcement_class_vocab,
        "enforcement_class",
        "enforcement_class_vocabulary",
    )
    assert_vocab_matches_schema(
        lifecycle_state_class_vocab,
        "lifecycle_state_class",
        "lifecycle_state_class_vocabulary",
    )
    assert_vocab_matches_schema(
        consumer_class_vocab,
        "contribution_governance_consumer_class",
        "contribution_governance_consumer_class_vocabulary",
    )

    # --force-drill plumbing.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<control_id>:<drill_id>'"
            )
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="contribution_governance.envelope_rows_empty",
                message="matrix.rows must declare at least one row",
                remediation="Seed the required rows.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_control_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        row_id_local = ensure_str(
            raw_row.get("control_id"), f"matrix.rows[{idx}].control_id"
        )
        original_row = copy.deepcopy(raw_row)
        drill = ensure_dict(
            raw_row.get("failure_drill"),
            f"{row_id_local}.failure_drill",
        )
        drill_id_local = ensure_str(
            drill.get("drill_id"),
            f"{row_id_local}.failure_drill.drill_id",
        )

        example_override: dict[str, Any] | None = None
        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        if forced_row_id is not None and row_id_local == forced_row_id:
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does not"
                    f" match the row's failure_drill.drill_id"
                    f" {drill_id_local!r}"
                )
            applied_overrides = ensure_dict(
                drill.get("forced_input"),
                f"{row_id_local}.failure_drill.forced_input",
            )
            # Pre-load the example so apply_forced_overrides can rewrite it.
            example_rel = raw_row.get("example_payload_ref")
            if (
                isinstance(example_rel, str)
                and (repo_root / example_rel).exists()
            ):
                try:
                    example_override = json.loads(
                        (repo_root / example_rel).read_text(encoding="utf-8")
                    )
                except json.JSONDecodeError:
                    example_override = None
            replay_row_payload, example_override = apply_forced_overrides(
                raw_row, example_override, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            repo_root=repo_root,
            row_id_value=row_id_local,
            control_class_vocab=control_class_vocab,
            artifact_kind_class_vocab=artifact_kind_class_vocab,
            enforcement_class_vocab=enforcement_class_vocab,
            lifecycle_state_class_vocab=lifecycle_state_class_vocab,
            consumer_class_vocab=consumer_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            control_class_to_artifact_kind_class=control_class_to_artifact_kind_class,
            control_class_to_enforcement_class=control_class_to_enforcement_class,
            example_override=example_override,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="contribution_governance.rows_duplicate_id",
                    message=f"duplicate control_id: {result.row_id}",
                    remediation="control_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)

        seen_control_classes.add(
            ensure_str(
                original_row.get("control_class"),
                f"{result.row_id}.control_class",
            )
        )

        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and applied_overrides
        ):
            expected_check = ensure_str(
                drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_controls = required_control_class_coverage - seen_control_classes
    if missing_controls:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "contribution_governance."
                    "coverage_missing_required_control_classes"
                ),
                message=(
                    "matrix must seed at least one row for each required"
                    f" control_class: {sorted(required_control_class_coverage)};"
                    f" missing: {sorted(missing_controls)}"
                ),
                remediation=(
                    "Add the missing rows so every required control is"
                    " exercised."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit reflects the drill verdict.
    for result in row_results:
        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "contribution_governance.row_failed_check"
                    ),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the contribution-governance"
                        " contract or fix the drift in the seed; failures"
                        " are reported with the precise actionable"
                        " check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "contribution_governance_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "import_record_register_ref": args.import_record_register,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_contribution_governance_seed_lane/"
            "run_m1_contribution_governance_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_control_class_coverage": sorted(required_control_class_coverage),
        "observed_control_classes": sorted(seen_control_classes),
        "rows": [
            {
                "row_id": r.row_id,
                "control_class": r.control_class,
                "artifact_kind_class": r.artifact_kind_class,
                "enforcement_class": r.enforcement_class,
                "lifecycle_state_class": r.lifecycle_state_class,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "contribution-governance-seed"
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
                f"[{label}] forced drill {forced_replay_record['drill_id']}"
                f" on {forced_replay_record['row_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['row_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[contribution-governance-seed] interrupted", file=sys.stderr)
        sys.exit(130)
