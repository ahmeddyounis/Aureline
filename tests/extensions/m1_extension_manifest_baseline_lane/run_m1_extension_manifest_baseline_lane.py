#!/usr/bin/env python3
"""Unattended M1 extension manifest-baseline validation lane.

Replays every row in
``fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml``
against the M1 extension-manifest baseline schema:

- ``schemas/extensions/m1_extension_manifest.schema.json`` — the
  extension_manifest_baseline_record, the
  effective_permission_baseline_record, and the
  manifest_install_decision_record vocabulary.

Per-row assertions:

- ``row_id`` is unique and namespaced under ``extension_manifest:``;
- the row's ``manifest_baseline``, ``effective_permission_baseline``,
  and ``install_decision`` blocks carry the matching ``record_kind``
  discriminator and the pinned schema version;
- closed vocabularies are honored (``publisher_trust_tier_class``,
  ``publisher_lifecycle_state_class``,
  ``extension_lifecycle_state_class``,
  ``manifest_origin_source_class``, ``host_contract_family_class``,
  ``permission_scope_class``, ``effective_permission_diff_class``,
  ``manifest_scope_completeness_class``, ``install_decision_class``,
  ``install_decision_reason_class``);
- the manifest-baseline structural invariants hold:
  ``publisher_identity_required``,
  ``publisher_display_label_required``,
  ``publisher_signing_key_required``, ``origin_source_label_required``,
  ``extension_identity_unnamespaced``,
  ``manifest_baseline_id_unprefixed``,
  ``declared_permission_rationale_required``,
  ``declared_permission_scope_target_required``;
- the effective-permission baseline rules hold:
  every ``effective_permissions`` entry's ``(scope_class, scope_target)``
  pair is in the manifest's declared set
  (``effective_scope_not_in_declared_set``);
  ``widening_attempted_blocked_count`` equals the number of
  ``declared_vs_effective_diff`` entries with diff_class
  ``widening_attempted_blocked``
  (``widening_attempted_blocked_count_mismatch``);
- the install-decision precedence rules hold:
  ``anonymous_publisher_install_must_be_denied``,
  ``quarantined_publisher_install_must_be_denied``,
  ``unknown_origin_install_must_be_denied``,
  ``incomplete_manifest_install_must_be_denied``,
  ``unverified_publisher_install_must_be_review_only``,
  ``step_up_required_install_must_be_admit_with_step_up``,
  ``effective_permission_widening_attempted``;
- the matrix covers every entry in
  ``required_publisher_trust_tier_coverage`` and
  ``required_install_decision_coverage``;
- every named external schema, build identity, and consumer ref exists
  on disk.

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill <row_id>:<drill_id>``
replays the named drill on the named row and exits 0 only when the
runner reproduces the declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix and
fixture files are parsed via Ruby/Psych so this script does not require
a third-party Python YAML dependency.
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


DEFAULT_MATRIX_REL = (
    "fixtures/extensions/m1_extension_manifest_baseline_rows/m1_rows.yaml"
)
DEFAULT_SCHEMA_REL = "schemas/extensions/m1_extension_manifest.schema.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "extension_manifest_baseline_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

ROW_ID_PREFIX = "extension_manifest:"
MANIFEST_BASELINE_ID_PREFIX = "manifest_baseline:"

EXPECTED_SCHEMA_VERSION = 1


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
    publisher_trust_tier_class: str
    install_decision_class: str
    install_decision_reason_class: str
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
        help="Seed-row matrix YAML path, repo-relative.",
    )
    parser.add_argument(
        "--schema",
        default=DEFAULT_SCHEMA_REL,
        help="M1 extension-manifest baseline schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build-identity record to embed in the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<row_id>:<drill_id>'. The runner exits 0 only when the "
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
    schema = json.loads((repo_root / ref).read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def apply_forced_overrides(
    row: dict[str, Any], forced_overrides: dict[str, Any]
) -> dict[str, Any]:
    """Return a deep-copied row with the forced overrides applied."""
    row = copy.deepcopy(row)
    if not forced_overrides:
        return row

    manifest = row["manifest_baseline"]
    effective = row["effective_permission_baseline"]
    decision = row["install_decision"]

    if forced_overrides.get("clear_publisher_identity_ref"):
        manifest["publisher_identity_ref"] = ""

    if forced_overrides.get("clear_first_declared_permission_rationale"):
        if manifest.get("declared_permissions"):
            manifest["declared_permissions"][0]["rationale_label"] = ""

    inject = forced_overrides.get("inject_widening_attempted_blocked_diff")
    if isinstance(inject, dict):
        effective.setdefault("declared_vs_effective_diff", []).append(
            {
                "scope_class": inject.get("scope_class"),
                "scope_target": inject.get("scope_target"),
                "diff_class": "widening_attempted_blocked",
                "narrowing_reason_label": (
                    "declared scope did not include this scope; widening blocked"
                ),
            }
        )
        effective["widening_attempted_blocked_count"] = (
            int(effective.get("widening_attempted_blocked_count", 0)) + 1
        )

    if "rewrite_install_decision_class" in forced_overrides:
        decision["install_decision_class"] = forced_overrides[
            "rewrite_install_decision_class"
        ]
    if "rewrite_install_decision_reason_class" in forced_overrides:
        decision["install_decision_reason_class"] = forced_overrides[
            "rewrite_install_decision_reason_class"
        ]

    return row


def validate_manifest_baseline(
    record: dict[str, Any],
    *,
    row_id: str,
    publisher_trust_tier_class_vocab: set[str],
    publisher_lifecycle_state_class_vocab: set[str],
    extension_lifecycle_state_class_vocab: set[str],
    manifest_origin_source_class_vocab: set[str],
    host_contract_family_class_vocab: set[str],
    permission_scope_class_vocab: set[str],
    manifest_scope_completeness_class_vocab: set[str],
    result: RowResult,
) -> None:
    if record.get("record_kind") != "extension_manifest_baseline_record":
        fail(
            result,
            "manifest_baseline.record_kind_wrong",
            (
                "manifest_baseline.record_kind must be"
                " 'extension_manifest_baseline_record'; "
                f"got {record.get('record_kind')!r}"
            ),
        )
    if (
        record.get("extension_manifest_baseline_schema_version")
        != EXPECTED_SCHEMA_VERSION
    ):
        fail(
            result,
            "manifest_baseline.schema_version_wrong",
            (
                "manifest_baseline.extension_manifest_baseline_schema_version"
                f" must be {EXPECTED_SCHEMA_VERSION}; got"
                f" {record.get('extension_manifest_baseline_schema_version')!r}"
            ),
        )

    manifest_baseline_id = ensure_str(
        record.get("manifest_baseline_id"),
        f"{row_id}.manifest_baseline.manifest_baseline_id",
    )
    if not manifest_baseline_id.startswith(MANIFEST_BASELINE_ID_PREFIX):
        fail(
            result,
            "manifest_baseline.manifest_baseline_id_unprefixed",
            (
                f"manifest_baseline_id '{manifest_baseline_id}' must start with"
                f" '{MANIFEST_BASELINE_ID_PREFIX}'"
            ),
        )

    extension_identity = ensure_str(
        record.get("extension_identity"),
        f"{row_id}.manifest_baseline.extension_identity",
    )
    if "/" not in extension_identity:
        fail(
            result,
            "manifest_baseline.extension_identity_unnamespaced",
            (
                f"extension_identity '{extension_identity}' must be of the form"
                " 'publisher_id/extension_id'"
            ),
        )

    publisher_identity_ref = record.get("publisher_identity_ref")
    if not isinstance(publisher_identity_ref, str) or not publisher_identity_ref.strip():
        fail(
            result,
            "manifest_baseline.publisher_identity_required",
            (
                "publisher_identity_ref MUST be a non-empty opaque ref;"
                " anonymous or ambient publisher privilege is not acceptable"
            ),
        )
    else:
        pass_(
            result,
            f"publisher_identity_ref present ('{publisher_identity_ref}')",
        )

    if not isinstance(record.get("publisher_display_label"), str) or not record[
        "publisher_display_label"
    ].strip():
        fail(
            result,
            "manifest_baseline.publisher_display_label_required",
            "publisher_display_label MUST be a non-empty string",
        )

    if not isinstance(record.get("publisher_signing_key_ref"), str) or not record[
        "publisher_signing_key_ref"
    ].strip():
        fail(
            result,
            "manifest_baseline.publisher_signing_key_required",
            "publisher_signing_key_ref MUST be a non-empty opaque ref",
        )

    if not isinstance(record.get("origin_source_label"), str) or not record[
        "origin_source_label"
    ].strip():
        fail(
            result,
            "manifest_baseline.origin_source_label_required",
            "origin_source_label MUST be a non-empty string",
        )

    publisher_trust_tier = record.get("publisher_trust_tier_class")
    if publisher_trust_tier not in publisher_trust_tier_class_vocab:
        fail(
            result,
            "manifest_baseline.publisher_trust_tier_class_unknown",
            (
                f"publisher_trust_tier_class '{publisher_trust_tier}' is not in"
                " publisher_trust_tier_class_vocabulary"
            ),
        )

    publisher_lifecycle = record.get("publisher_lifecycle_state_class")
    if publisher_lifecycle not in publisher_lifecycle_state_class_vocab:
        fail(
            result,
            "manifest_baseline.publisher_lifecycle_state_class_unknown",
            (
                f"publisher_lifecycle_state_class '{publisher_lifecycle}' is not"
                " in publisher_lifecycle_state_class_vocabulary"
            ),
        )

    extension_lifecycle = record.get("extension_lifecycle_state_class")
    if extension_lifecycle not in extension_lifecycle_state_class_vocab:
        fail(
            result,
            "manifest_baseline.extension_lifecycle_state_class_unknown",
            (
                f"extension_lifecycle_state_class '{extension_lifecycle}' is not"
                " in extension_lifecycle_state_class_vocabulary"
            ),
        )

    origin_class = record.get("manifest_origin_source_class")
    if origin_class not in manifest_origin_source_class_vocab:
        fail(
            result,
            "manifest_baseline.manifest_origin_source_class_unknown",
            (
                f"manifest_origin_source_class '{origin_class}' is not in"
                " manifest_origin_source_class_vocabulary"
            ),
        )

    host_class = record.get("host_contract_family_class")
    if host_class not in host_contract_family_class_vocab:
        fail(
            result,
            "manifest_baseline.host_contract_family_class_unknown",
            (
                f"host_contract_family_class '{host_class}' is not in"
                " host_contract_family_class_vocabulary"
            ),
        )

    completeness = record.get("manifest_scope_completeness_class")
    if completeness not in manifest_scope_completeness_class_vocab:
        fail(
            result,
            "manifest_baseline.manifest_scope_completeness_class_unknown",
            (
                f"manifest_scope_completeness_class '{completeness}' is not in"
                " manifest_scope_completeness_class_vocabulary"
            ),
        )

    declared_permissions = ensure_list(
        record.get("declared_permissions", []),
        f"{row_id}.manifest_baseline.declared_permissions",
    )
    for idx, perm in enumerate(declared_permissions):
        perm = ensure_dict(
            perm, f"{row_id}.manifest_baseline.declared_permissions[{idx}]"
        )
        scope_class = perm.get("scope_class")
        if scope_class not in permission_scope_class_vocab:
            fail(
                result,
                "manifest_baseline.permission_scope_class_unknown",
                (
                    f"declared_permissions[{idx}].scope_class '{scope_class}'"
                    " is not in permission_scope_class_vocabulary"
                ),
            )
        scope_target = perm.get("scope_target")
        if not isinstance(scope_target, str) or not scope_target.strip():
            fail(
                result,
                "manifest_baseline.declared_permission_scope_target_required",
                (
                    f"declared_permissions[{idx}].scope_target MUST be a"
                    " non-empty string"
                ),
            )
        rationale = perm.get("rationale_label")
        if not isinstance(rationale, str) or not rationale.strip():
            fail(
                result,
                "manifest_baseline.declared_permission_rationale_required",
                (
                    f"declared_permissions[{idx}] (scope_class={scope_class!r},"
                    f" scope_target={scope_target!r}) is missing a non-empty"
                    " rationale_label"
                ),
            )


def validate_effective_permission_baseline(
    record: dict[str, Any],
    *,
    row_id: str,
    declared_pairs: set[tuple[str, str]],
    permission_scope_class_vocab: set[str],
    effective_permission_diff_class_vocab: set[str],
    result: RowResult,
) -> None:
    if record.get("record_kind") != "effective_permission_baseline_record":
        fail(
            result,
            "manifest_baseline.effective_record_kind_wrong",
            (
                "effective_permission_baseline.record_kind must be"
                " 'effective_permission_baseline_record'; "
                f"got {record.get('record_kind')!r}"
            ),
        )

    if (
        record.get("extension_manifest_baseline_schema_version")
        != EXPECTED_SCHEMA_VERSION
    ):
        fail(
            result,
            "manifest_baseline.effective_schema_version_wrong",
            (
                "effective_permission_baseline."
                "extension_manifest_baseline_schema_version must be"
                f" {EXPECTED_SCHEMA_VERSION}; got"
                f" {record.get('extension_manifest_baseline_schema_version')!r}"
            ),
        )

    effective_permissions = ensure_list(
        record.get("effective_permissions", []),
        f"{row_id}.effective_permission_baseline.effective_permissions",
    )
    for idx, perm in enumerate(effective_permissions):
        perm = ensure_dict(
            perm,
            f"{row_id}.effective_permission_baseline.effective_permissions[{idx}]",
        )
        scope_class = perm.get("scope_class")
        scope_target = perm.get("scope_target")
        if scope_class not in permission_scope_class_vocab:
            fail(
                result,
                "manifest_baseline.effective_permission_scope_class_unknown",
                (
                    f"effective_permissions[{idx}].scope_class '{scope_class}'"
                    " is not in permission_scope_class_vocabulary"
                ),
            )
        if (scope_class, scope_target) not in declared_pairs:
            fail(
                result,
                "manifest_baseline.effective_scope_not_in_declared_set",
                (
                    f"effective_permissions[{idx}] (scope_class={scope_class!r},"
                    f" scope_target={scope_target!r}) is not in the manifest's"
                    " declared_permissions set"
                ),
            )

    diff = ensure_list(
        record.get("declared_vs_effective_diff", []),
        f"{row_id}.effective_permission_baseline.declared_vs_effective_diff",
    )
    observed_widening = 0
    for idx, entry in enumerate(diff):
        entry = ensure_dict(
            entry,
            f"{row_id}.effective_permission_baseline.declared_vs_effective_diff[{idx}]",
        )
        diff_class = entry.get("diff_class")
        if diff_class not in effective_permission_diff_class_vocab:
            fail(
                result,
                "manifest_baseline.effective_permission_diff_class_unknown",
                (
                    f"declared_vs_effective_diff[{idx}].diff_class"
                    f" '{diff_class}' is not in"
                    " effective_permission_diff_class_vocabulary"
                ),
            )
        if diff_class == "widening_attempted_blocked":
            observed_widening += 1
        narrowing_reason_label = entry.get("narrowing_reason_label")
        if (
            not isinstance(narrowing_reason_label, str)
            or not narrowing_reason_label.strip()
        ):
            fail(
                result,
                "manifest_baseline.effective_diff_reason_label_required",
                (
                    f"declared_vs_effective_diff[{idx}].narrowing_reason_label"
                    " MUST be a non-empty string"
                ),
            )

    declared_count = record.get("widening_attempted_blocked_count")
    if not isinstance(declared_count, int) or declared_count < 0:
        fail(
            result,
            "manifest_baseline.widening_attempted_blocked_count_invalid",
            (
                "widening_attempted_blocked_count MUST be a non-negative integer"
            ),
        )
    elif declared_count != observed_widening:
        fail(
            result,
            "manifest_baseline.widening_attempted_blocked_count_mismatch",
            (
                "widening_attempted_blocked_count"
                f" ({declared_count}) does not match the number of"
                " declared_vs_effective_diff entries with diff_class"
                f" widening_attempted_blocked ({observed_widening})"
            ),
        )

    summary_freshness_class = record.get("summary_freshness_class")
    if summary_freshness_class not in {
        "authoritative_live",
        "warm_cached",
        "degraded_cached",
        "stale",
        "unverified",
    }:
        fail(
            result,
            "manifest_baseline.summary_freshness_class_unknown",
            (
                f"summary_freshness_class '{summary_freshness_class}' is not in"
                " the closed freshness vocabulary"
            ),
        )


def validate_install_decision(
    record: dict[str, Any],
    *,
    row_id: str,
    manifest: dict[str, Any],
    effective: dict[str, Any],
    install_decision_class_vocab: set[str],
    install_decision_reason_class_vocab: set[str],
    result: RowResult,
) -> None:
    if record.get("record_kind") != "manifest_install_decision_record":
        fail(
            result,
            "manifest_baseline.install_decision_record_kind_wrong",
            (
                "install_decision.record_kind must be"
                " 'manifest_install_decision_record'; "
                f"got {record.get('record_kind')!r}"
            ),
        )

    if (
        record.get("extension_manifest_baseline_schema_version")
        != EXPECTED_SCHEMA_VERSION
    ):
        fail(
            result,
            "manifest_baseline.install_decision_schema_version_wrong",
            (
                "install_decision.extension_manifest_baseline_schema_version"
                f" must be {EXPECTED_SCHEMA_VERSION}; got"
                f" {record.get('extension_manifest_baseline_schema_version')!r}"
            ),
        )

    decision_class = record.get("install_decision_class")
    reason_class = record.get("install_decision_reason_class")

    if decision_class not in install_decision_class_vocab:
        fail(
            result,
            "manifest_baseline.install_decision_class_unknown",
            (
                f"install_decision_class '{decision_class}' is not in"
                " install_decision_class_vocabulary"
            ),
        )
    if reason_class not in install_decision_reason_class_vocab:
        fail(
            result,
            "manifest_baseline.install_decision_reason_class_unknown",
            (
                f"install_decision_reason_class '{reason_class}' is not in"
                " install_decision_reason_class_vocabulary"
            ),
        )

    summary = record.get("decision_summary")
    if not isinstance(summary, str) or not summary.strip():
        fail(
            result,
            "manifest_baseline.install_decision_summary_required",
            "decision_summary MUST be a non-empty string",
        )

    publisher_trust_tier = manifest.get("publisher_trust_tier_class")
    publisher_lifecycle = manifest.get("publisher_lifecycle_state_class")
    extension_lifecycle = manifest.get("extension_lifecycle_state_class")
    origin_class = manifest.get("manifest_origin_source_class")
    completeness = manifest.get("manifest_scope_completeness_class")
    widening_count = effective.get("widening_attempted_blocked_count", 0)
    diff_entries = effective.get("declared_vs_effective_diff", []) or []
    has_step_up = any(
        isinstance(d, dict) and d.get("diff_class") == "step_up_required"
        for d in diff_entries
    )

    # Precedence rules (must match the Rust decide_manifest_install
    # contract).
    if publisher_trust_tier == "anonymous_publisher_class":
        if decision_class != "denied" or reason_class != "publisher_anonymous":
            fail(
                result,
                "manifest_baseline.anonymous_publisher_install_must_be_denied",
                (
                    "anonymous_publisher_class MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class = publisher_anonymous; "
                    f"got class={decision_class!r}, reason={reason_class!r}"
                ),
            )
    elif (
        publisher_trust_tier == "quarantined_publisher"
        or publisher_lifecycle == "quarantined"
    ):
        if decision_class != "denied" or reason_class != "publisher_quarantined":
            fail(
                result,
                "manifest_baseline.quarantined_publisher_install_must_be_denied",
                (
                    "quarantined publisher MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class = publisher_quarantined; "
                    f"got class={decision_class!r}, reason={reason_class!r}"
                ),
            )
    elif publisher_lifecycle == "retired":
        if (
            decision_class != "denied"
            or reason_class != "publisher_lifecycle_retired"
        ):
            fail(
                result,
                "manifest_baseline.retired_publisher_install_must_be_denied",
                (
                    "retired publisher MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class = publisher_lifecycle_retired"
                ),
            )
    elif extension_lifecycle in {"retired", "quarantined"}:
        if (
            decision_class != "denied"
            or reason_class != "extension_lifecycle_retired"
        ):
            fail(
                result,
                "manifest_baseline.retired_extension_install_must_be_denied",
                (
                    "retired or quarantined extension MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class = extension_lifecycle_retired"
                ),
            )
    elif origin_class == "unknown_source_class":
        if (
            decision_class != "denied"
            or reason_class != "manifest_origin_unknown"
        ):
            fail(
                result,
                "manifest_baseline.unknown_origin_install_must_be_denied",
                (
                    "unknown_source_class origin MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class = manifest_origin_unknown"
                ),
            )
    elif widening_count > 0:
        if (
            decision_class != "denied"
            or reason_class != "effective_permission_widening_attempted"
        ):
            fail(
                result,
                "manifest_baseline.effective_permission_widening_attempted",
                (
                    "widening_attempted_blocked_count"
                    f" ({widening_count}) > 0 MUST be paired with"
                    " install_decision_class = denied and"
                    " install_decision_reason_class ="
                    " effective_permission_widening_attempted"
                ),
            )
    elif completeness != "complete":
        if decision_class != "denied":
            fail(
                result,
                "manifest_baseline.incomplete_manifest_install_must_be_denied",
                (
                    f"manifest_scope_completeness_class = {completeness!r}"
                    " (anything other than complete) MUST be paired with"
                    " install_decision_class = denied"
                ),
            )
    elif has_step_up:
        if (
            decision_class != "admit_with_step_up"
            or reason_class != "step_up_required_by_policy_pack"
        ):
            fail(
                result,
                "manifest_baseline.step_up_required_install_must_be_admit_with_step_up",
                (
                    "a declared_vs_effective_diff entry with diff_class"
                    " step_up_required MUST be paired with"
                    " install_decision_class = admit_with_step_up and"
                    " install_decision_reason_class ="
                    " step_up_required_by_policy_pack"
                ),
            )
    elif publisher_trust_tier == "unverified_publisher":
        if (
            decision_class != "review_only"
            or reason_class != "review_only_unverified_publisher"
        ):
            fail(
                result,
                "manifest_baseline.unverified_publisher_install_must_be_review_only",
                (
                    "unverified_publisher MUST be paired with"
                    " install_decision_class = review_only and"
                    " install_decision_reason_class ="
                    " review_only_unverified_publisher"
                ),
            )
    else:
        if decision_class != "admit" or reason_class != "admitted_no_violation":
            fail(
                result,
                "manifest_baseline.admittable_manifest_install_must_be_admit",
                (
                    "no denial / step-up / review trigger fires; install"
                    " MUST be admit / admitted_no_violation; got"
                    f" class={decision_class!r}, reason={reason_class!r}"
                ),
            )


def validate_row(
    row: dict[str, Any],
    *,
    publisher_trust_tier_class_vocab: set[str],
    publisher_lifecycle_state_class_vocab: set[str],
    extension_lifecycle_state_class_vocab: set[str],
    manifest_origin_source_class_vocab: set[str],
    host_contract_family_class_vocab: set[str],
    permission_scope_class_vocab: set[str],
    effective_permission_diff_class_vocab: set[str],
    manifest_scope_completeness_class_vocab: set[str],
    install_decision_class_vocab: set[str],
    install_decision_reason_class_vocab: set[str],
) -> RowResult:
    row_id = ensure_str(row.get("row_id"), "row.row_id")
    manifest = ensure_dict(
        row.get("manifest_baseline"), f"{row_id}.manifest_baseline"
    )
    effective = ensure_dict(
        row.get("effective_permission_baseline"),
        f"{row_id}.effective_permission_baseline",
    )
    decision = ensure_dict(
        row.get("install_decision"), f"{row_id}.install_decision"
    )

    publisher_trust_tier_class = ensure_str(
        manifest.get("publisher_trust_tier_class"),
        f"{row_id}.manifest_baseline.publisher_trust_tier_class",
    )
    install_decision_class = ensure_str(
        decision.get("install_decision_class"),
        f"{row_id}.install_decision.install_decision_class",
    )
    install_decision_reason_class = ensure_str(
        decision.get("install_decision_reason_class"),
        f"{row_id}.install_decision.install_decision_reason_class",
    )

    result = RowResult(
        row_id=row_id,
        publisher_trust_tier_class=publisher_trust_tier_class,
        install_decision_class=install_decision_class,
        install_decision_reason_class=install_decision_reason_class,
    )

    if not row_id.startswith(ROW_ID_PREFIX):
        fail(
            result,
            "manifest_baseline.row_id_unprefixed",
            f"row_id '{row_id}' must start with '{ROW_ID_PREFIX}'",
        )

    declared_permissions = ensure_list(
        manifest.get("declared_permissions", []),
        f"{row_id}.manifest_baseline.declared_permissions",
    )
    declared_pairs: set[tuple[str, str]] = set()
    for perm in declared_permissions:
        if isinstance(perm, dict):
            sc = perm.get("scope_class")
            st = perm.get("scope_target")
            if isinstance(sc, str) and isinstance(st, str):
                declared_pairs.add((sc, st))

    validate_manifest_baseline(
        manifest,
        row_id=row_id,
        publisher_trust_tier_class_vocab=publisher_trust_tier_class_vocab,
        publisher_lifecycle_state_class_vocab=publisher_lifecycle_state_class_vocab,
        extension_lifecycle_state_class_vocab=extension_lifecycle_state_class_vocab,
        manifest_origin_source_class_vocab=manifest_origin_source_class_vocab,
        host_contract_family_class_vocab=host_contract_family_class_vocab,
        permission_scope_class_vocab=permission_scope_class_vocab,
        manifest_scope_completeness_class_vocab=manifest_scope_completeness_class_vocab,
        result=result,
    )
    validate_effective_permission_baseline(
        effective,
        row_id=row_id,
        declared_pairs=declared_pairs,
        permission_scope_class_vocab=permission_scope_class_vocab,
        effective_permission_diff_class_vocab=effective_permission_diff_class_vocab,
        result=result,
    )
    validate_install_decision(
        decision,
        row_id=row_id,
        manifest=manifest,
        effective=effective,
        install_decision_class_vocab=install_decision_class_vocab,
        install_decision_reason_class_vocab=install_decision_reason_class_vocab,
        result=result,
    )

    # Failure-drill shape.
    drill = ensure_dict(row.get("failure_drill"), f"{row_id}.failure_drill")
    ensure_str(drill.get("drill_id"), f"{row_id}.failure_drill.drill_id")
    ensure_str(
        drill.get("expected_check_id"),
        f"{row_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        drill.get("actionable_owner_ref"),
        f"{row_id}.failure_drill.actionable_owner_ref",
    )
    ensure_str(drill.get("next_action"), f"{row_id}.failure_drill.next_action")
    forced_input = ensure_dict(
        drill.get("forced_input"), f"{row_id}.failure_drill.forced_input"
    )
    if not forced_input:
        fail(
            result,
            "manifest_baseline.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )

    result.diagnostics.update(
        {
            "row_id": row_id,
            "publisher_trust_tier_class": publisher_trust_tier_class,
            "install_decision_class": install_decision_class,
            "install_decision_reason_class": install_decision_reason_class,
            "declared_permission_count": len(declared_permissions),
            "declared_permission_pairs": [
                {"scope_class": sc, "scope_target": st} for sc, st in sorted(declared_pairs)
            ],
            "widening_attempted_blocked_count": effective.get(
                "widening_attempted_blocked_count", 0
            ),
            "applied_policy_pack_refs": effective.get("applied_policy_pack_refs", []),
            "failure_drill": {
                "drill_id": drill.get("drill_id"),
                "expected_check_id": drill.get("expected_check_id"),
            },
        }
    )

    return result


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=(
                    f"matrix schema_version must be the integer 1, got "
                    f"{schema_version!r}"
                ),
                remediation=(
                    "Bump the runner together with the matrix when its shape"
                    " changes."
                ),
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation=(
                    "Create the reviewer-facing landing page or fix the path."
                ),
                ref=overview_page,
            )
        )

    for key in (
        "manifest_schema_ref",
        "adr_seed_schema_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{key}.missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation=(
                        "Fix the path or land the referenced artifact."
                    ),
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
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

    publisher_trust_tier_class_vocab = load_vocab(
        "publisher_trust_tier_class_vocabulary"
    )
    publisher_lifecycle_state_class_vocab = load_vocab(
        "publisher_lifecycle_state_class_vocabulary"
    )
    extension_lifecycle_state_class_vocab = load_vocab(
        "extension_lifecycle_state_class_vocabulary"
    )
    manifest_origin_source_class_vocab = load_vocab(
        "manifest_origin_source_class_vocabulary"
    )
    host_contract_family_class_vocab = load_vocab(
        "host_contract_family_class_vocabulary"
    )
    permission_scope_class_vocab = load_vocab(
        "permission_scope_class_vocabulary"
    )
    effective_permission_diff_class_vocab = load_vocab(
        "effective_permission_diff_class_vocabulary"
    )
    manifest_scope_completeness_class_vocab = load_vocab(
        "manifest_scope_completeness_class_vocabulary"
    )
    install_decision_class_vocab = load_vocab(
        "install_decision_class_vocabulary"
    )
    install_decision_reason_class_vocab = load_vocab(
        "install_decision_reason_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")

    required_publisher_trust_tier_coverage = load_vocab(
        "required_publisher_trust_tier_coverage"
    )
    required_install_decision_coverage = load_vocab(
        "required_install_decision_coverage"
    )

    schema_rel = ensure_str(
        matrix.get("manifest_schema_ref"), "matrix.manifest_schema_ref"
    )

    def assert_vocab_matches_schema(
        matrix_vocab: set[str], defs_key: str, name: str
    ) -> None:
        if not (repo_root / schema_rel).exists():
            return
        schema_enum = set(load_schema_enums(repo_root, schema_rel, defs_key))
        if not schema_enum:
            return
        diff = matrix_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{name}.mismatch_schema",
                    message=(
                        f"matrix.{name} disagrees with"
                        f" {schema_rel}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(matrix_vocab - schema_enum)};"
                        f" schema-only: {sorted(schema_enum - matrix_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix's closed vocabulary in lock-step"
                        f" with {schema_rel}; the schema is canonical."
                    ),
                )
            )

    assert_vocab_matches_schema(
        publisher_trust_tier_class_vocab,
        "publisher_trust_tier_class",
        "publisher_trust_tier_class_vocabulary",
    )
    assert_vocab_matches_schema(
        publisher_lifecycle_state_class_vocab,
        "publisher_lifecycle_state_class",
        "publisher_lifecycle_state_class_vocabulary",
    )
    assert_vocab_matches_schema(
        extension_lifecycle_state_class_vocab,
        "extension_lifecycle_state_class",
        "extension_lifecycle_state_class_vocabulary",
    )
    assert_vocab_matches_schema(
        manifest_origin_source_class_vocab,
        "manifest_origin_source_class",
        "manifest_origin_source_class_vocabulary",
    )
    assert_vocab_matches_schema(
        host_contract_family_class_vocab,
        "host_contract_family_class",
        "host_contract_family_class_vocabulary",
    )
    assert_vocab_matches_schema(
        permission_scope_class_vocab,
        "permission_scope_class",
        "permission_scope_class_vocabulary",
    )
    assert_vocab_matches_schema(
        effective_permission_diff_class_vocab,
        "effective_permission_diff_class",
        "effective_permission_diff_class_vocabulary",
    )
    assert_vocab_matches_schema(
        manifest_scope_completeness_class_vocab,
        "manifest_scope_completeness_class",
        "manifest_scope_completeness_class_vocabulary",
    )
    assert_vocab_matches_schema(
        install_decision_class_vocab,
        "install_decision_class",
        "install_decision_class_vocabulary",
    )
    assert_vocab_matches_schema(
        install_decision_reason_class_vocab,
        "install_decision_reason_class",
        "install_decision_reason_class_vocabulary",
    )

    # Consumer bindings.
    consumer_bindings = ensure_dict(
        matrix.get("consumer_bindings"), "matrix.consumer_bindings"
    )
    named_consumer = ensure_dict(
        consumer_bindings.get("named_runtime_consumer"),
        "matrix.consumer_bindings.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        "matrix.consumer_bindings.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.missing",
                message=(
                    "named_runtime_consumer.consumer_ref does not exist: "
                    f"{consumer_ref}"
                ),
                remediation=(
                    "Point at a real downstream consumer or seed the surface"
                    " before claiming a runtime consumer exists."
                ),
                ref=consumer_ref,
            )
        )
    consumed_fields = ensure_list(
        named_consumer.get("consumed_fields"),
        "matrix.consumer_bindings.named_runtime_consumer.consumed_fields",
    )
    if not consumed_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.consumed_fields_empty",
                message=(
                    "named_runtime_consumer.consumed_fields must declare at"
                    " least one truth field"
                ),
                remediation=(
                    "Name the truth fields the runtime consumer reads from"
                    " the matrix."
                ),
            )
        )

    rust_consumer = consumer_bindings.get("rust_validator_consumer")
    if isinstance(rust_consumer, dict):
        rust_ref = ensure_str(
            rust_consumer.get("consumer_ref"),
            "matrix.consumer_bindings.rust_validator_consumer.consumer_ref",
        )
        if not artifact_ref_exists(repo_root, rust_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rust_validator_consumer.missing",
                    message=(
                        "rust_validator_consumer.consumer_ref does not exist: "
                        f"{rust_ref}"
                    ),
                    remediation=(
                        "Point at the Rust crate module that consumes the"
                        " manifest baseline."
                    ),
                    ref=rust_ref,
                )
            )

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least one capability row",
                remediation="Seed the required rows.",
            )
        )

    # Resolve --force-drill if requested.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<row_id>:<drill_id>'"
            )
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_trust_tiers: set[str] = set()
    seen_install_decisions: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")
        original_row = copy.deepcopy(row)
        row_id_local = ensure_str(row.get("row_id"), "row.row_id")
        drill = ensure_dict(
            row.get("failure_drill"), f"{row_id_local}.failure_drill"
        )
        drill_id_local = ensure_str(
            drill.get("drill_id"), f"{row_id_local}.failure_drill.drill_id"
        )

        if drill_id_local not in failure_drill_id_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.failure_drill_id_unknown",
                    message=(
                        f"{row_id_local}: failure_drill.drill_id"
                        f" '{drill_id_local}' is not in"
                        " failure_drill_id_vocabulary"
                    ),
                    remediation=(
                        "Add the drill id to failure_drill_id_vocabulary or"
                        " rename the drill."
                    ),
                    ref=row_id_local,
                )
            )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = row
        if forced_row_id is not None and row_id_local == forced_row_id:
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id '{forced_drill_id}' does not"
                    f" match the row's failure_drill.drill_id"
                    f" '{drill_id_local}'"
                )
            applied_overrides = ensure_dict(
                drill.get("forced_input"),
                f"{row_id_local}.failure_drill.forced_input",
            )
            replay_row_payload = apply_forced_overrides(
                row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            publisher_trust_tier_class_vocab=publisher_trust_tier_class_vocab,
            publisher_lifecycle_state_class_vocab=publisher_lifecycle_state_class_vocab,
            extension_lifecycle_state_class_vocab=extension_lifecycle_state_class_vocab,
            manifest_origin_source_class_vocab=manifest_origin_source_class_vocab,
            host_contract_family_class_vocab=host_contract_family_class_vocab,
            permission_scope_class_vocab=permission_scope_class_vocab,
            effective_permission_diff_class_vocab=effective_permission_diff_class_vocab,
            manifest_scope_completeness_class_vocab=manifest_scope_completeness_class_vocab,
            install_decision_class_vocab=install_decision_class_vocab,
            install_decision_reason_class_vocab=install_decision_reason_class_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate row_id: {result.row_id}",
                    remediation="row_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)

        # Coverage is computed from the original row (not the drill-forced
        # replay), so drills cannot accidentally satisfy or violate coverage.
        expected_coverage = ensure_dict(
            original_row.get("expected_coverage"),
            f"{result.row_id}.expected_coverage",
        )
        seen_trust_tiers.add(
            ensure_str(
                expected_coverage.get("publisher_trust_tier_class"),
                f"{result.row_id}.expected_coverage.publisher_trust_tier_class",
            )
        )
        seen_install_decisions.add(
            ensure_str(
                expected_coverage.get("install_decision_class"),
                f"{result.row_id}.expected_coverage.install_decision_class",
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

    missing_trust_tiers = required_publisher_trust_tier_coverage - seen_trust_tiers
    if missing_trust_tiers:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_publisher_trust_tiers",
                message=(
                    "matrix must seed at least one row for each required"
                    f" publisher_trust_tier_class:"
                    f" {sorted(required_publisher_trust_tier_coverage)};"
                    f" missing: {sorted(missing_trust_tiers)}"
                ),
                remediation=(
                    "Add the missing rows so every required publisher trust"
                    " tier is exercised."
                ),
            )
        )

    missing_install_decisions = (
        required_install_decision_coverage - seen_install_decisions
    )
    if missing_install_decisions:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_install_decisions",
                message=(
                    "matrix must seed at least one row for each required"
                    f" install_decision_class:"
                    f" {sorted(required_install_decision_coverage)}; missing:"
                    f" {sorted(missing_install_decisions)}"
                ),
                remediation=(
                    "Add the missing rows so every required install decision"
                    " class is exercised."
                ),
            )
        )

    # Promote per-row failures into findings. Skip the targeted row under
    # --force-drill so the runner's exit can reflect the reproduce verdict.
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
                        "check_id", "matrix.row.failed_check"
                    ),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the manifest baseline schema"
                        " or fix the drift in the matrix; failures are"
                        " reported with the precise actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "extension_manifest_baseline_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "manifest_schema_ref": args.schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/extensions/m1_extension_manifest_baseline_lane/"
            "run_m1_extension_manifest_baseline_lane.py --repo-root ."
        ),
        "status": status,
        "required_publisher_trust_tier_coverage": sorted(
            required_publisher_trust_tier_coverage
        ),
        "observed_publisher_trust_tiers": sorted(seen_trust_tiers),
        "required_install_decision_coverage": sorted(
            required_install_decision_coverage
        ),
        "observed_install_decisions": sorted(seen_install_decisions),
        "rows": [
            {
                "row_id": r.row_id,
                "publisher_trust_tier_class": r.publisher_trust_tier_class,
                "install_decision_class": r.install_decision_class,
                "install_decision_reason_class": r.install_decision_reason_class,
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

    label = "extension-manifest-baseline"
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
        print("[extension-manifest-baseline] interrupted", file=sys.stderr)
        sys.exit(130)
