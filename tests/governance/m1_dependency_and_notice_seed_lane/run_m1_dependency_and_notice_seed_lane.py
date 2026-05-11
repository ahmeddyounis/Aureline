#!/usr/bin/env python3
"""Unattended M1 critical-dependency / import register seed validation lane.

Replays every row in ``artifacts/governance/critical_dependency_register.yaml``
against:

- ``schemas/governance/critical_dependency_register.schema.json`` — the
  seed envelope schema (vocabularies, required coverage, named consumers,
  required template-class / publication-target / release-notice-action
  coverage);
- ``schemas/governance/critical_dependency_register_entry.schema.json``
  — the row vocabulary; and
- the canonical landing page at
  ``docs/governance/m1_dependency_and_notice_seed.md`` so the seed
  cannot quietly point at a missing reviewer entry.

Per-row assertions:

- ``record_kind`` is ``critical_dependency_register_entry_record`` and
  ``critical_dependency_register_entry_schema_version`` is ``1``;
- ``register_entry_id`` is unique, non-empty, and matches the row
  schema's pattern;
- ``source_register``, ``template_class``, ``criticality_class``,
  ``protected_path_class``, ``license_class``,
  ``provenance_status_class``, ``admission_state_class``,
  ``release_notice_action_class``, and every ``publication_targets``
  entry are members of their closed vocabularies in the row schema;
- ``source_id`` matches the row schema's pattern AND agrees with
  ``source_register`` (``dep.*`` ↔ ``dependency_register``,
  ``import.*`` ↔ ``third_party_import_register``);
- ``source_id`` resolves in the named companion register on disk;
- ``owner_dri`` is a non-empty ``@handle``;
- ``fork_or_replace_trigger`` is non-empty (and required when
  ``protected_path_class = protected_path_critical``);
- ``failure_drill`` is a non-null object with a closed ``drill_id``;
- ``release_notice_action_class = hold_pending_first_admission`` is
  only valid when ``admission_state_class`` is
  ``selected_not_admitted`` or ``reserved_not_yet_imported``.

Template-class invariants the lane enforces independently of the row
schema (so the runner can emit precise actionable check_ids):

- ``runtime_dependency`` rows MUST include third_party_notice +
  spdx_sbom + provenance_statement in publication_targets and MUST
  use release_notice_action_class in {emit_third_party_notice_and_sbom_entries,
  hold_pending_first_admission};
- ``bundled_asset`` rows MUST include provenance_statement and MUST
  use release_notice_action_class in {emit_bundled_asset_notice_when_imported,
  hold_pending_first_admission};
- ``build_tooling`` rows MUST NOT include third_party_notice, MUST
  include provenance_statement, and MUST use
  release_notice_action_class = emit_build_tooling_provenance_record_only;
- ``host_runtime`` rows MUST publish publication_targets exactly equal
  to [provenance_statement] and MUST use release_notice_action_class
  = emit_host_runtime_environment_capture_only;
- ``mirrored_pack`` rows MUST include docs_pack_manifest +
  provenance_statement and MUST use release_notice_action_class =
  emit_docs_pack_manifest_attribution.

Cross-register drift assertions:

- every critical companion-register row (dependency or import) with
  criticality in {protected_path_release_critical,
  release_engineering_critical, benchmark_lab_required} MUST have a
  matching seed entry. The named failure drill exercises this
  ("add or update a critical dependency without register / update-
  notice changes -> automation flags the omission").

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_critical_dependency_register_seed``, ``status`` non-empty,
  ``owner_dri`` is a ``@handle``;
- ``overview_page``, ``row_schema_ref``, ``build_identity_ref``,
  ``validation_lane_ref`` resolve on disk;
- closed envelope vocabularies (source_register_class_vocabulary,
  template_class_vocabulary, criticality_class_vocabulary,
  protected_path_class_vocabulary, license_class_vocabulary,
  provenance_status_class_vocabulary, admission_state_class_vocabulary,
  publication_target_class_vocabulary,
  release_notice_action_class_vocabulary) AGREE with the row schema
  $defs of the same names;
- required template / publication-target / release-notice-action
  coverage lists are each satisfied by the seed entries;
- every named_runtime_consumer.consumer_ref resolves on disk and
  consumed_fields is non-empty;
- the draft-output paths declared in draft_output_refs both exist
  after a successful build_dependency_notice_seed.py --write (the
  lane runs it once for the on-disk capture).

``--force-drill <register_entry_id>:<drill_id>`` replays the named
drill on the named row and exits 0 only when the runner reproduces
the declared ``expected_check_id``. Drift in the unforced rows still
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
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/governance/critical_dependency_register.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/critical_dependency_register.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/critical_dependency_register_entry.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "dependency_and_notice_seed_validation_capture.json"
)

EXPECTED_RECORD_KIND = "critical_dependency_register_entry_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_critical_dependency_register_seed"

REGISTER_ENTRY_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")
SOURCE_ID_PATTERN = re.compile(r"^(dep|import)\.[a-z0-9]+(?:[._-][a-z0-9]+)*$")

CRITICAL_COMPANION_CRITICALITY_CLASSES = {
    "protected_path_release_critical",
    "release_engineering_critical",
    "benchmark_lab_required",
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
    register_entry_id: str
    template_class: str
    release_notice_action_class: str
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
            "'<register_entry_id>:<drill_id>'. The runner exits 0 only "
            "when the row's failure drill reproduces the exact "
            "expected_check_id."
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


def load_schema_enum(repo_root: Path, ref: str, defs_key: str) -> list[str]:
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
    row: dict[str, Any], forced_overrides: dict[str, Any]
) -> dict[str, Any]:
    row = copy.deepcopy(row)
    if not forced_overrides:
        return row

    if forced_overrides.get("clear_fork_or_replace_trigger"):
        row["fork_or_replace_trigger"] = ""

    if forced_overrides.get("clear_owner_dri"):
        row["owner_dri"] = ""

    if "rewrite_template_class" in forced_overrides:
        row["template_class"] = forced_overrides["rewrite_template_class"]

    if "rewrite_publication_targets" in forced_overrides:
        row["publication_targets"] = list(
            forced_overrides["rewrite_publication_targets"]
        )

    if "rewrite_release_notice_action_class" in forced_overrides:
        row["release_notice_action_class"] = forced_overrides[
            "rewrite_release_notice_action_class"
        ]

    if "rewrite_admission_state_class" in forced_overrides:
        row["admission_state_class"] = forced_overrides[
            "rewrite_admission_state_class"
        ]

    if "rewrite_protected_path_class" in forced_overrides:
        row["protected_path_class"] = forced_overrides[
            "rewrite_protected_path_class"
        ]

    if "rewrite_source_id" in forced_overrides:
        row["source_id"] = forced_overrides["rewrite_source_id"]

    return row


def validate_row(
    row: dict[str, Any],
    *,
    capability_label: str,
    template_class_vocab: set[str],
    criticality_class_vocab: set[str],
    protected_path_class_vocab: set[str],
    license_class_vocab: set[str],
    provenance_status_class_vocab: set[str],
    admission_state_class_vocab: set[str],
    publication_target_class_vocab: set[str],
    release_notice_action_class_vocab: set[str],
    source_register_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
) -> RowResult:
    entry_id = ensure_str(
        row.get("register_entry_id"),
        f"{capability_label}.register_entry_id",
    )
    template_class = (
        row.get("template_class")
        if isinstance(row.get("template_class"), str)
        else ""
    )
    action = (
        row.get("release_notice_action_class")
        if isinstance(row.get("release_notice_action_class"), str)
        else ""
    )

    result = RowResult(
        register_entry_id=entry_id,
        template_class=template_class,
        release_notice_action_class=action,
    )

    # --- discriminator and version pins ---------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "critical_dependency_register.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("critical_dependency_register_entry_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "critical_dependency_register.schema_version_wrong",
            (
                "critical_dependency_register_entry_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{row.get('critical_dependency_register_entry_schema_version')!r}"
            ),
        )

    # --- register_entry_id pattern --------------------------------------
    if not REGISTER_ENTRY_ID_PATTERN.match(entry_id):
        fail(
            result,
            "critical_dependency_register.register_entry_id_pattern_invalid",
            (
                f"register_entry_id {entry_id!r} does not match "
                f"{REGISTER_ENTRY_ID_PATTERN.pattern!r}"
            ),
        )

    # --- source_register / source_id ------------------------------------
    source_register = row.get("source_register")
    if source_register not in source_register_class_vocab:
        fail(
            result,
            "critical_dependency_register.source_register_class_unknown",
            (
                f"source_register {source_register!r} is not in the "
                "row schema's source_register_class enum"
            ),
        )

    source_id = row.get("source_id")
    if not isinstance(source_id, str) or not SOURCE_ID_PATTERN.match(source_id):
        fail(
            result,
            "critical_dependency_register.source_id_pattern_mismatch",
            (
                f"source_id {source_id!r} does not match "
                f"{SOURCE_ID_PATTERN.pattern!r}"
            ),
        )
    else:
        if source_register == "dependency_register" and not source_id.startswith(
            "dep."
        ):
            fail(
                result,
                "critical_dependency_register.source_id_prefix_must_be_dep",
                (
                    "source_register = dependency_register but source_id "
                    f"{source_id!r} does not start with 'dep.'"
                ),
            )
        if (
            source_register == "third_party_import_register"
            and not source_id.startswith("import.")
        ):
            fail(
                result,
                "critical_dependency_register.source_id_prefix_must_be_import",
                (
                    "source_register = third_party_import_register but "
                    f"source_id {source_id!r} does not start with 'import.'"
                ),
            )

    # --- name -----------------------------------------------------------
    name = row.get("name")
    if not isinstance(name, str) or not name.strip():
        fail(
            result,
            "critical_dependency_register.name_required",
            "name must be a non-empty string",
        )

    # --- template_class -------------------------------------------------
    if template_class not in template_class_vocab:
        fail(
            result,
            "critical_dependency_register.template_class_unknown",
            (
                f"template_class {template_class!r} is not in the row "
                "schema's template_class enum"
            ),
        )

    # --- criticality_class ---------------------------------------------
    criticality_class = row.get("criticality_class")
    if criticality_class not in criticality_class_vocab:
        fail(
            result,
            "critical_dependency_register.criticality_class_unknown",
            (
                f"criticality_class {criticality_class!r} is not in the "
                "row schema's criticality_class enum"
            ),
        )

    # --- protected_path_class ------------------------------------------
    protected_path_class = row.get("protected_path_class")
    if protected_path_class not in protected_path_class_vocab:
        fail(
            result,
            "critical_dependency_register.protected_path_class_unknown",
            (
                f"protected_path_class {protected_path_class!r} is not "
                "in the row schema's protected_path_class enum"
            ),
        )

    # --- license_class --------------------------------------------------
    license_class = row.get("license_class")
    if license_class not in license_class_vocab:
        fail(
            result,
            "critical_dependency_register.license_class_unknown",
            (
                f"license_class {license_class!r} is not in the row "
                "schema's license_class enum"
            ),
        )

    # --- provenance_status_class ---------------------------------------
    provenance_status_class = row.get("provenance_status_class")
    if provenance_status_class not in provenance_status_class_vocab:
        fail(
            result,
            "critical_dependency_register.provenance_status_class_unknown",
            (
                f"provenance_status_class {provenance_status_class!r} is "
                "not in the row schema's provenance_status_class enum"
            ),
        )

    # --- admission_state_class -----------------------------------------
    admission_state_class = row.get("admission_state_class")
    if admission_state_class not in admission_state_class_vocab:
        fail(
            result,
            "critical_dependency_register.admission_state_class_unknown",
            (
                f"admission_state_class {admission_state_class!r} is not "
                "in the row schema's admission_state_class enum"
            ),
        )

    # --- publication_targets ------------------------------------------
    publication_targets = row.get("publication_targets")
    if not isinstance(publication_targets, list) or not publication_targets:
        fail(
            result,
            "critical_dependency_register.publication_targets_required",
            "publication_targets must be a non-empty list",
        )
        targets_set: set[str] = set()
    else:
        targets_set = set(publication_targets)
        for target in publication_targets:
            if target not in publication_target_class_vocab:
                fail(
                    result,
                    "critical_dependency_register.publication_target_unknown",
                    (
                        f"publication_targets entry {target!r} is not in "
                        "the row schema's publication_target_class enum"
                    ),
                )

    # --- release_notice_action_class -----------------------------------
    if action not in release_notice_action_class_vocab:
        fail(
            result,
            "critical_dependency_register.release_notice_action_class_unknown",
            (
                f"release_notice_action_class {action!r} is not in the "
                "row schema's release_notice_action_class enum"
            ),
        )

    # --- template_class invariants -------------------------------------
    if template_class == "runtime_dependency":
        if "third_party_notice" not in targets_set:
            fail(
                result,
                "critical_dependency_register.runtime_dependency_publication_targets_must_include_third_party_notice",
                "runtime_dependency rows MUST include third_party_notice in publication_targets",
            )
        if "spdx_sbom" not in targets_set:
            fail(
                result,
                "critical_dependency_register.runtime_dependency_publication_targets_must_include_spdx_sbom",
                "runtime_dependency rows MUST include spdx_sbom in publication_targets",
            )
        if "provenance_statement" not in targets_set:
            fail(
                result,
                "critical_dependency_register.runtime_dependency_publication_targets_must_include_provenance_statement",
                "runtime_dependency rows MUST include provenance_statement in publication_targets",
            )
        if action not in {
            "emit_third_party_notice_and_sbom_entries",
            "hold_pending_first_admission",
        }:
            fail(
                result,
                "critical_dependency_register.runtime_dependency_release_notice_action_must_match_template",
                (
                    "runtime_dependency rows MUST use "
                    "release_notice_action_class = "
                    "emit_third_party_notice_and_sbom_entries or "
                    f"hold_pending_first_admission; got {action!r}"
                ),
            )
    elif template_class == "bundled_asset":
        if "provenance_statement" not in targets_set:
            fail(
                result,
                "critical_dependency_register.bundled_asset_publication_targets_must_include_provenance_statement",
                "bundled_asset rows MUST include provenance_statement in publication_targets",
            )
        if action not in {
            "emit_bundled_asset_notice_when_imported",
            "hold_pending_first_admission",
        }:
            fail(
                result,
                "critical_dependency_register.bundled_asset_release_notice_action_must_match_template",
                (
                    "bundled_asset rows MUST use release_notice_action_class "
                    "in {emit_bundled_asset_notice_when_imported, "
                    f"hold_pending_first_admission}}; got {action!r}"
                ),
            )
    elif template_class == "build_tooling":
        if "third_party_notice" in targets_set:
            fail(
                result,
                "critical_dependency_register.build_tooling_publication_targets_must_not_include_third_party_notice",
                "build_tooling rows MUST NOT include third_party_notice in publication_targets",
            )
        if "provenance_statement" not in targets_set:
            fail(
                result,
                "critical_dependency_register.build_tooling_publication_targets_must_include_provenance_statement",
                "build_tooling rows MUST include provenance_statement in publication_targets",
            )
        if action != "emit_build_tooling_provenance_record_only":
            fail(
                result,
                "critical_dependency_register.build_tooling_release_notice_action_must_match_template",
                (
                    "build_tooling rows MUST use release_notice_action_class "
                    f"= emit_build_tooling_provenance_record_only; got {action!r}"
                ),
            )
    elif template_class == "host_runtime":
        if targets_set != {"provenance_statement"}:
            fail(
                result,
                "critical_dependency_register.host_runtime_publication_targets_must_be_provenance_only",
                (
                    "host_runtime rows MUST publish exactly "
                    "[provenance_statement] in publication_targets; got "
                    f"{sorted(targets_set)!r}"
                ),
            )
        if action != "emit_host_runtime_environment_capture_only":
            fail(
                result,
                "critical_dependency_register.host_runtime_release_notice_action_must_match_template",
                (
                    "host_runtime rows MUST use release_notice_action_class "
                    f"= emit_host_runtime_environment_capture_only; got {action!r}"
                ),
            )
    elif template_class == "mirrored_pack":
        if "docs_pack_manifest" not in targets_set:
            fail(
                result,
                "critical_dependency_register.mirrored_pack_publication_targets_must_include_docs_pack_manifest",
                "mirrored_pack rows MUST include docs_pack_manifest in publication_targets",
            )
        if "provenance_statement" not in targets_set:
            fail(
                result,
                "critical_dependency_register.mirrored_pack_publication_targets_must_include_provenance_statement",
                "mirrored_pack rows MUST include provenance_statement in publication_targets",
            )
        if action != "emit_docs_pack_manifest_attribution":
            fail(
                result,
                "critical_dependency_register.mirrored_pack_release_notice_action_must_match_template",
                (
                    "mirrored_pack rows MUST use release_notice_action_class "
                    f"= emit_docs_pack_manifest_attribution; got {action!r}"
                ),
            )

    # --- owner_dri / fork_or_replace_trigger ---------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "critical_dependency_register.owner_dri_pattern_invalid",
            f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
        )

    fork_trigger = row.get("fork_or_replace_trigger")
    if not isinstance(fork_trigger, str) or not fork_trigger.strip():
        if protected_path_class == "protected_path_critical":
            fail(
                result,
                "critical_dependency_register.fork_or_replace_trigger_required_for_protected_path",
                (
                    "protected_path_class = protected_path_critical rows "
                    "MUST publish a non-empty fork_or_replace_trigger"
                ),
            )
        else:
            fail(
                result,
                "critical_dependency_register.fork_or_replace_trigger_required",
                "fork_or_replace_trigger must be a non-empty string",
            )

    # --- hold_pending_first_admission gating ---------------------------
    if action == "hold_pending_first_admission" and admission_state_class not in {
        "selected_not_admitted",
        "reserved_not_yet_imported",
    }:
        fail(
            result,
            "critical_dependency_register.hold_pending_first_admission_blocked_when_admitted",
            (
                "release_notice_action_class = hold_pending_first_admission "
                "is only valid when admission_state_class is "
                "selected_not_admitted or reserved_not_yet_imported; got "
                f"{admission_state_class!r}"
            ),
        )

    # --- evidence_refs / failure_drill ---------------------------------
    if not isinstance(row.get("evidence_refs"), list):
        fail(
            result,
            "critical_dependency_register.evidence_refs_must_be_list",
            "evidence_refs must be a list (may be empty)",
        )

    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "critical_dependency_register.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "critical_dependency_register.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "critical_dependency_register.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "critical_dependency_register.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if not isinstance(expected_check, str) or not expected_check.strip():
            fail(
                result,
                "critical_dependency_register.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "critical_dependency_register.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    result.diagnostics.update(
        {
            "register_entry_id": entry_id,
            "template_class": template_class,
            "release_notice_action_class": action,
            "publication_targets": sorted(targets_set),
            "criticality_class": criticality_class,
            "protected_path_class": protected_path_class,
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {entry_id} passes")

    return result


def load_companion_register(repo_root: Path, rel: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(render_yaml_as_json(repo_root / rel), rel)
    rows = ensure_list(payload.get("rows"), f"{rel}.rows")
    out: dict[str, dict[str, Any]] = {}
    for row in rows:
        if not isinstance(row, dict):
            continue
        row_id = row.get("id")
        if isinstance(row_id, str) and row_id.strip():
            out[row_id] = row
    return out


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix = ensure_dict(
        render_yaml_as_json(repo_root / matrix_rel), matrix_rel
    )

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_schema_version_wrong",
                message=(
                    f"matrix schema_version must be 1; got {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != EXPECTED_MATRIX_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be {EXPECTED_MATRIX_ID!r}; got "
                    f"{matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    owner_dri = ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    if not OWNER_DRI_PATTERN.match(owner_dri):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_owner_dri_pattern_invalid",
                message=f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}",
                remediation="Use an @handle for the owner DRI.",
            )
        )

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer landing page or fix the path.",
                ref=overview_page,
            )
        )

    for key in ("row_schema_ref", "build_identity_ref"):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"critical_dependency_register.envelope_{key}_missing",
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
                check_id="critical_dependency_register.envelope_validation_lane_ref_missing",
                message=(
                    f"validation_lane_ref base does not exist: "
                    f"{validation_lane_ref}"
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

    source_register_class_vocab = load_vocab("source_register_class_vocabulary")
    template_class_vocab = load_vocab("template_class_vocabulary")
    criticality_class_vocab = load_vocab("criticality_class_vocabulary")
    protected_path_class_vocab = load_vocab("protected_path_class_vocabulary")
    license_class_vocab = load_vocab("license_class_vocabulary")
    provenance_status_class_vocab = load_vocab("provenance_status_class_vocabulary")
    admission_state_class_vocab = load_vocab("admission_state_class_vocabulary")
    publication_target_class_vocab = load_vocab(
        "publication_target_class_vocabulary"
    )
    release_notice_action_class_vocab = load_vocab(
        "release_notice_action_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")

    required_template_class_coverage = load_vocab("required_template_class_coverage")
    required_publication_target_coverage = load_vocab(
        "required_publication_target_coverage"
    )
    required_release_notice_action_coverage = load_vocab(
        "required_release_notice_action_coverage"
    )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )

    # Closed-vocabulary agreement with the row schema.
    schema_vocab_pairs = [
        ("source_register_class_vocabulary", "source_register_class", source_register_class_vocab),
        ("template_class_vocabulary", "template_class", template_class_vocab),
        ("criticality_class_vocabulary", "criticality_class", criticality_class_vocab),
        ("protected_path_class_vocabulary", "protected_path_class", protected_path_class_vocab),
        ("license_class_vocabulary", "license_class", license_class_vocab),
        ("provenance_status_class_vocabulary", "provenance_status_class", provenance_status_class_vocab),
        ("admission_state_class_vocabulary", "admission_state_class", admission_state_class_vocab),
        ("publication_target_class_vocabulary", "publication_target_class", publication_target_class_vocab),
        ("release_notice_action_class_vocabulary", "release_notice_action_class", release_notice_action_class_vocab),
    ]
    for envelope_key, defs_key, envelope_set in schema_vocab_pairs:
        schema_enum = set(load_schema_enum(repo_root, row_schema_ref, defs_key))
        if not schema_enum:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.envelope_row_schema_defs_missing",
                    message=(
                        f"row schema {row_schema_ref} is missing $defs.{defs_key}.enum"
                    ),
                    remediation="Restore the row schema $defs entry.",
                    ref=row_schema_ref,
                )
            )
            continue
        diff = envelope_set.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"critical_dependency_register.envelope_{envelope_key}_disagrees_with_row_schema",
                    message=(
                        f"matrix.{envelope_key} disagrees with "
                        f"{row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(envelope_set - schema_enum)}; "
                        f"schema-only: {sorted(schema_enum - envelope_set)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the "
                        "row schema; the schema is canonical."
                    ),
                )
            )

    # --- named runtime consumers --------------------------------------
    consumers = ensure_list(
        matrix.get("named_runtime_consumers"),
        "matrix.named_runtime_consumers",
    )
    if not consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_named_runtime_consumers_empty",
                message="named_runtime_consumers must declare at least one consumer",
                remediation="Add at least one named runtime consumer that reads the seed.",
            )
        )
    for idx, consumer in enumerate(consumers):
        consumer = ensure_dict(
            consumer, f"matrix.named_runtime_consumers[{idx}]"
        )
        ensure_str(
            consumer.get("consumer_id"),
            f"matrix.named_runtime_consumers[{idx}].consumer_id",
        )
        consumer_ref = ensure_str(
            consumer.get("consumer_ref"),
            f"matrix.named_runtime_consumers[{idx}].consumer_ref",
        )
        if not artifact_ref_exists(repo_root, consumer_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.named_runtime_consumer_ref_missing",
                    message=(
                        f"named_runtime_consumers[{idx}].consumer_ref does "
                        f"not exist: {consumer_ref}"
                    ),
                    remediation=(
                        "Fix the path or land the referenced consumer "
                        "before claiming it as live."
                    ),
                    ref=consumer_ref,
                )
            )
        consumed_fields = consumer.get("consumed_fields")
        if not isinstance(consumed_fields, list) or not consumed_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.named_runtime_consumer_consumed_fields_empty",
                    message=(
                        f"named_runtime_consumers[{idx}].consumed_fields "
                        "must be a non-empty list"
                    ),
                    remediation=(
                        "Name at least one field the consumer reads so "
                        "the consumer cannot regress to mentioned-but-"
                        "unread."
                    ),
                )
            )

    # --- companion-register paths -------------------------------------
    companion = ensure_dict(
        matrix.get("companion_registers"), "matrix.companion_registers"
    )
    dep_rel = ensure_str(
        companion.get("dependency_register"),
        "matrix.companion_registers.dependency_register",
    )
    imp_rel = ensure_str(
        companion.get("third_party_import_register"),
        "matrix.companion_registers.third_party_import_register",
    )
    seed_rel = ensure_str(
        companion.get("release_notice_seed"),
        "matrix.companion_registers.release_notice_seed",
    )
    for label, rel in (
        ("dependency_register", dep_rel),
        ("third_party_import_register", imp_rel),
        ("release_notice_seed", seed_rel),
    ):
        if not artifact_ref_exists(repo_root, rel):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"critical_dependency_register.companion_register_missing.{label}",
                    message=f"companion register {label} does not exist: {rel}",
                    remediation="Fix the path or land the companion register.",
                    ref=rel,
                )
            )

    dep_rows = load_companion_register(repo_root, dep_rel)
    imp_rows = load_companion_register(repo_root, imp_rel)

    # --force-drill plumbing ---------------------------------------------
    forced_entry_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<register_entry_id>:<drill_id>'"
            )
        forced_entry_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_entry_id = forced_entry_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one register row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_template_classes: set[str] = set()
    seen_publication_targets: set[str] = set()
    seen_actions: set[str] = set()
    seed_source_ids_by_register: dict[str, set[str]] = {
        "dependency_register": set(),
        "third_party_import_register": set(),
    }
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        entry_id_local = ensure_str(
            raw_row.get("register_entry_id"),
            f"matrix.entries[{idx}].register_entry_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        drill_local: dict[str, Any] | None = None
        if (
            forced_entry_id is not None
            and entry_id_local == forced_entry_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted register_entry_id "
                    f"{forced_entry_id!r} but the row has no failure_drill"
                )
            drill_id_local = drill_local.get("drill_id")
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does not "
                    f"match the row's failure_drill.drill_id "
                    f"{drill_id_local!r}"
                )
            forced_input_local = drill_local.get("forced_input")
            if not isinstance(forced_input_local, dict):
                raise SystemExit(
                    f"failure_drill.forced_input must be an object on row "
                    f"{forced_entry_id!r}"
                )
            applied_overrides = forced_input_local
            replay_row_payload = apply_forced_overrides(
                raw_row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            capability_label=entry_id_local,
            template_class_vocab=template_class_vocab,
            criticality_class_vocab=criticality_class_vocab,
            protected_path_class_vocab=protected_path_class_vocab,
            license_class_vocab=license_class_vocab,
            provenance_status_class_vocab=provenance_status_class_vocab,
            admission_state_class_vocab=admission_state_class_vocab,
            publication_target_class_vocab=publication_target_class_vocab,
            release_notice_action_class_vocab=release_notice_action_class_vocab,
            source_register_class_vocab=source_register_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.register_entry_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.entries_duplicate_register_entry_id",
                    message=(
                        f"duplicate register_entry_id: {result.register_entry_id}"
                    ),
                    remediation="register_entry_ids must be unique.",
                    ref=result.register_entry_id,
                )
            )
        seen_ids.add(result.register_entry_id)

        # Source-id resolution against the companion register. Use the
        # raw row's source_id (not the forced-replay payload) so the
        # companion-register critical-coverage check still treats the
        # row as present in the seed under --force-drill; the drill
        # exercises an unrelated drift in the row's other fields.
        src_reg_raw = raw_row.get("source_register")
        src_id_raw = raw_row.get("source_id")
        if (
            isinstance(src_reg_raw, str)
            and src_reg_raw in seed_source_ids_by_register
            and isinstance(src_id_raw, str)
        ):
            seed_source_ids_by_register[src_reg_raw].add(src_id_raw)

        # Companion-register resolution per-row is only reported when
        # this row is not under --force-drill (so the drill verdict is
        # clean) AND the replay payload's source fields still agree.
        if not (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and applied_overrides
        ):
            src_reg = replay_row_payload.get("source_register")
            src_id = replay_row_payload.get("source_id")
            if (
                isinstance(src_reg, str)
                and src_reg in seed_source_ids_by_register
                and isinstance(src_id, str)
            ):
                ref = f"{matrix_rel}#{result.register_entry_id}"
                if src_reg == "dependency_register" and src_id not in dep_rows:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="critical_dependency_register.source_id_not_found_in_companion_register",
                            message=(
                                f"source_id {src_id!r} does not resolve in "
                                f"{dep_rel}"
                            ),
                            remediation=(
                                "Fix source_id or add the missing upstream "
                                "row in the companion dependency register."
                            ),
                            ref=ref,
                        )
                    )
                elif (
                    src_reg == "third_party_import_register"
                    and src_id not in imp_rows
                ):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="critical_dependency_register.source_id_not_found_in_companion_register",
                            message=(
                                f"source_id {src_id!r} does not resolve in "
                                f"{imp_rel}"
                            ),
                            remediation=(
                                "Fix source_id or add the missing upstream "
                                "row in the companion import register."
                            ),
                            ref=ref,
                        )
                    )

        if isinstance(raw_row.get("template_class"), str):
            seen_template_classes.add(raw_row["template_class"])
        if isinstance(raw_row.get("publication_targets"), list):
            for t in raw_row["publication_targets"]:
                if isinstance(t, str):
                    seen_publication_targets.add(t)
        if isinstance(raw_row.get("release_notice_action_class"), str):
            seen_actions.add(raw_row["release_notice_action_class"])

        if (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and applied_overrides
            and isinstance(drill_local, dict)
        ):
            expected_check = ensure_str(
                drill_local.get("expected_check_id"),
                f"{forced_entry_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "register_entry_id": forced_entry_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- companion-register critical-coverage ---------------------------
    for dep_id, dep_row in dep_rows.items():
        criticality = dep_row.get("criticality")
        if (
            isinstance(criticality, str)
            and criticality in CRITICAL_COMPANION_CRITICALITY_CLASSES
            and dep_id not in seed_source_ids_by_register["dependency_register"]
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.companion_dependency_missing_seed_entry",
                    message=(
                        f"critical dependency row {dep_id!r} (criticality "
                        f"{criticality!r}) has no matching seed entry in "
                        f"{matrix_rel}"
                    ),
                    remediation=(
                        "Add a critical_dependency_register row that cites "
                        f"source_register=dependency_register and source_id={dep_id}, "
                        "or reduce the upstream row's criticality with an "
                        "explicit decision row in the dependency register."
                    ),
                    ref=f"{dep_rel}#{dep_id}",
                )
            )
    for imp_id, imp_row in imp_rows.items():
        criticality = imp_row.get("criticality")
        if (
            isinstance(criticality, str)
            and criticality in CRITICAL_COMPANION_CRITICALITY_CLASSES
            and imp_id
            not in seed_source_ids_by_register["third_party_import_register"]
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.companion_import_missing_seed_entry",
                    message=(
                        f"critical import row {imp_id!r} (criticality "
                        f"{criticality!r}) has no matching seed entry in "
                        f"{matrix_rel}"
                    ),
                    remediation=(
                        "Add a critical_dependency_register row that cites "
                        f"source_register=third_party_import_register and source_id={imp_id}, "
                        "or reduce the upstream row's criticality with an "
                        "explicit decision row in the import register."
                    ),
                    ref=f"{imp_rel}#{imp_id}",
                )
            )

    # --- required coverage ---------------------------------------------
    missing_templates = required_template_class_coverage - seen_template_classes
    if missing_templates:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.coverage_missing_required_template_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    f"template_class: {sorted(required_template_class_coverage)};"
                    f" missing: {sorted(missing_templates)}"
                ),
                remediation=(
                    "Add the missing rows so the canonical "
                    "runtime_dependency / bundled_asset / build_tooling / "
                    "host_runtime / mirrored_pack set is covered."
                ),
            )
        )

    missing_targets = (
        required_publication_target_coverage - seen_publication_targets
    )
    if missing_targets:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.coverage_missing_required_publication_targets",
                message=(
                    "matrix must exercise each required publication target "
                    f"{sorted(required_publication_target_coverage)};"
                    f" missing: {sorted(missing_targets)}"
                ),
                remediation=(
                    "Add a row whose publication_targets includes each "
                    "missing target so the draft pipeline emits the section."
                ),
            )
        )

    missing_actions = required_release_notice_action_coverage - seen_actions
    if missing_actions:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.coverage_missing_required_release_notice_actions",
                message=(
                    "matrix must exercise each required release-notice action "
                    f"{sorted(required_release_notice_action_coverage)};"
                    f" missing: {sorted(missing_actions)}"
                ),
                remediation=(
                    "Add a row whose release_notice_action_class is each "
                    "missing action so the draft pipeline emits every "
                    "required path."
                ),
            )
        )

    # --- draft outputs --------------------------------------------------
    draft_refs = matrix.get("draft_output_refs")
    if not isinstance(draft_refs, dict):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.envelope_draft_output_refs_missing",
                message="draft_output_refs must be a mapping/object",
                remediation="Declare draft_notice_markdown and draft_notice_json paths.",
            )
        )
    else:
        for key in ("draft_notice_markdown", "draft_notice_json"):
            value = draft_refs.get(key)
            if not isinstance(value, str) or not value.strip():
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"critical_dependency_register.envelope_draft_output_refs_{key}_missing",
                        message=f"draft_output_refs.{key} must be non-empty",
                        remediation="Declare the draft output path.",
                    )
                )
                continue
            if not artifact_ref_exists(repo_root, value):
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"critical_dependency_register.envelope_draft_output_refs_{key}_not_on_disk",
                        message=(
                            f"draft_output_refs.{key} does not exist on disk: {value}; "
                            "run tools/governance/build_dependency_notice_seed.py "
                            "to regenerate the draft outputs"
                        ),
                        remediation=(
                            "Run python3 tools/governance/build_dependency_notice_seed.py "
                            "--repo-root . to regenerate the draft notice."
                        ),
                        ref=value,
                    )
                )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
    for result in row_results:
        if (
            forced_entry_id is not None
            and result.register_entry_id == forced_entry_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id",
                        "critical_dependency_register.row_failed_check",
                    ),
                    message=f"{result.register_entry_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the critical-dependency "
                        "register contract or fix the drift in the seed; "
                        "failures are reported with the precise actionable "
                        "check_id."
                    ),
                    ref=result.register_entry_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    if forced_replay_record is not None and forced_replay_record["reproduced"]:
        status = "FORCE_DRILL_REPRODUCED"
    elif forced_replay_record is not None:
        status = "FORCE_DRILL_FAILED_TO_REPRODUCE"
    else:
        status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "dependency_and_notice_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_dependency_and_notice_seed_lane/"
            "run_m1_dependency_and_notice_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_template_class_coverage": sorted(
            required_template_class_coverage
        ),
        "observed_template_classes": sorted(seen_template_classes),
        "required_publication_target_coverage": sorted(
            required_publication_target_coverage
        ),
        "observed_publication_targets": sorted(seen_publication_targets),
        "required_release_notice_action_coverage": sorted(
            required_release_notice_action_coverage
        ),
        "observed_release_notice_actions": sorted(seen_actions),
        "observed_source_ids": {
            k: sorted(v) for k, v in seed_source_ids_by_register.items()
        },
        "rows": [
            {
                "register_entry_id": r.register_entry_id,
                "template_class": r.template_class,
                "release_notice_action_class": r.release_notice_action_class,
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
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "dependency-and-notice-seed"
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
                f" on {forced_replay_record['register_entry_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['register_entry_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[dependency-and-notice-seed] interrupted", file=sys.stderr)
        sys.exit(130)
