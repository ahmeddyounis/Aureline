#!/usr/bin/env python3
"""Build a draft release-notice / report artifact from the critical-dependency register.

This is the first executable consumer of
``artifacts/governance/critical_dependency_register.yaml``. It reads the
seed, resolves each row's ``source_id`` against the companion register
(``dependency_register.yaml`` for ``dep.*`` ids and
``third_party_import_register.yaml`` for ``import.*`` ids), and emits:

- a markdown draft at the seed's ``draft_output_refs.draft_notice_markdown``
  path (default ``artifacts/governance/build/dependency_notice_draft.md``)
  with one section per release-notice action class so reviewers can read
  the release narrative without learning the YAML, and
- a JSON sidecar at ``draft_output_refs.draft_notice_json`` so downstream
  automation (release packets, SBOM generators, provenance writers) can
  read the same payload without re-parsing markdown.

Drift modes
-----------

The tool fails closed when:

- a seed row's ``source_id`` does not resolve in the named companion register;
- a companion register declares a critical row (protected-path,
  release-engineering-critical, benchmark-lab-required) that has no
  matching seed entry (the spec's failure drill: adding a critical
  dependency without register / update-notice changes MUST be flagged);
- the seed's vocabulary disagrees with the row schema $defs;
- a row violates the template / publication-target / release-notice-action
  invariants the row schema declares;
- ``--check`` is passed and the on-disk draft outputs do not match what
  the tool would re-emit from the seed.

Run modes
---------

- (default) ``--write``: regenerate the draft outputs.
- ``--check``: compare the on-disk draft outputs against what the tool
  would emit; exit non-zero on mismatch. Suitable for CI / pre-commit.
- ``--force-drill omit:<register_entry_id>``: simulate omitting a seed
  row to prove the gate flags the omission.
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


DEFAULT_SEED_REL = "artifacts/governance/critical_dependency_register.yaml"
DEFAULT_ROW_SCHEMA_REL = (
    "schemas/governance/critical_dependency_register_entry.schema.json"
)
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/governance/critical_dependency_register.schema.json"
)

EXPECTED_RECORD_KIND = "critical_dependency_register_entry_record"
EXPECTED_MATRIX_ID = "m1_critical_dependency_register_seed"
EXPECTED_ROW_SCHEMA_VERSION = 1

REGISTER_ENTRY_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")
SOURCE_ID_PATTERN = re.compile(r"^(dep|import)\.[a-z0-9]+(?:[._-][a-z0-9]+)*$")

CRITICAL_DEP_CRITICALITY_CLASSES = {
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


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


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
    findings: list[Finding],
    seed_rel: str,
    template_class_vocab: set[str],
    criticality_class_vocab: set[str],
    protected_path_class_vocab: set[str],
    license_class_vocab: set[str],
    provenance_status_class_vocab: set[str],
    admission_state_class_vocab: set[str],
    publication_target_class_vocab: set[str],
    release_notice_action_class_vocab: set[str],
    source_register_class_vocab: set[str],
) -> None:
    entry_id = row.get("register_entry_id") or "<missing>"
    ref = f"{seed_rel}#{entry_id}"

    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.record_kind_wrong",
                message=(
                    f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                    f"{row.get('record_kind')!r}"
                ),
                remediation="Restore the discriminator on the row.",
                ref=ref,
            )
        )
    if (
        row.get("critical_dependency_register_entry_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.schema_version_wrong",
                message=(
                    "critical_dependency_register_entry_schema_version "
                    f"must be {EXPECTED_ROW_SCHEMA_VERSION}; got "
                    f"{row.get('critical_dependency_register_entry_schema_version')!r}"
                ),
                remediation="Bump runner together with the row schema.",
                ref=ref,
            )
        )

    if not isinstance(entry_id, str) or not REGISTER_ENTRY_ID_PATTERN.match(
        entry_id
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.register_entry_id_pattern_invalid",
                message=(
                    f"register_entry_id {entry_id!r} does not match "
                    f"{REGISTER_ENTRY_ID_PATTERN.pattern!r}"
                ),
                remediation="Use lowercase dot/underscore/hyphen segments only.",
                ref=ref,
            )
        )

    source_register = row.get("source_register")
    if source_register not in source_register_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.source_register_class_unknown",
                message=(
                    f"source_register {source_register!r} is not in the "
                    "row schema's source_register_class enum"
                ),
                remediation="Set source_register to dependency_register or third_party_import_register.",
                ref=ref,
            )
        )

    source_id = row.get("source_id")
    if not isinstance(source_id, str) or not SOURCE_ID_PATTERN.match(source_id):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.source_id_pattern_mismatch",
                message=(
                    f"source_id {source_id!r} does not match "
                    f"{SOURCE_ID_PATTERN.pattern!r}"
                ),
                remediation="source_id must start with 'dep.' (dependency_register) or 'import.' (third_party_import_register).",
                ref=ref,
            )
        )
    else:
        if source_register == "dependency_register" and not source_id.startswith(
            "dep."
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.source_id_prefix_must_be_dep",
                    message=(
                        "source_register = dependency_register but source_id "
                        f"{source_id!r} does not start with 'dep.'"
                    ),
                    remediation="Use a 'dep.*' source_id when source_register is dependency_register.",
                    ref=ref,
                )
            )
        if (
            source_register == "third_party_import_register"
            and not source_id.startswith("import.")
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.source_id_prefix_must_be_import",
                    message=(
                        "source_register = third_party_import_register but "
                        f"source_id {source_id!r} does not start with 'import.'"
                    ),
                    remediation="Use an 'import.*' source_id when source_register is third_party_import_register.",
                    ref=ref,
                )
            )

    name = row.get("name")
    if not isinstance(name, str) or not name.strip():
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.name_required",
                message="name must be a non-empty string",
                remediation="Restore the row's human-readable name.",
                ref=ref,
            )
        )

    template_class = row.get("template_class")
    if template_class not in template_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.template_class_unknown",
                message=(
                    f"template_class {template_class!r} is not in the row "
                    "schema's template_class enum"
                ),
                remediation="Use one of runtime_dependency / bundled_asset / build_tooling / host_runtime / mirrored_pack.",
                ref=ref,
            )
        )

    criticality_class = row.get("criticality_class")
    if criticality_class not in criticality_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.criticality_class_unknown",
                message=(
                    f"criticality_class {criticality_class!r} is not in the "
                    "row schema's criticality_class enum"
                ),
                remediation="Use a closed criticality vocabulary member.",
                ref=ref,
            )
        )

    protected_path_class = row.get("protected_path_class")
    if protected_path_class not in protected_path_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.protected_path_class_unknown",
                message=(
                    f"protected_path_class {protected_path_class!r} is not "
                    "in the row schema's protected_path_class enum"
                ),
                remediation="Use protected_path_critical / non_protected_repo_operations / non_protected_docs_pack.",
                ref=ref,
            )
        )

    license_class = row.get("license_class")
    if license_class not in license_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.license_class_unknown",
                message=(
                    f"license_class {license_class!r} is not in the row "
                    "schema's license_class enum"
                ),
                remediation="Use a closed license vocabulary member.",
                ref=ref,
            )
        )

    provenance_status_class = row.get("provenance_status_class")
    if provenance_status_class not in provenance_status_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.provenance_status_class_unknown",
                message=(
                    f"provenance_status_class {provenance_status_class!r} is "
                    "not in the row schema's provenance_status_class enum"
                ),
                remediation="Use a closed provenance-status vocabulary member.",
                ref=ref,
            )
        )

    admission_state_class = row.get("admission_state_class")
    if admission_state_class not in admission_state_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.admission_state_class_unknown",
                message=(
                    f"admission_state_class {admission_state_class!r} is not "
                    "in the row schema's admission_state_class enum"
                ),
                remediation="Use a closed admission-state vocabulary member.",
                ref=ref,
            )
        )

    publication_targets = row.get("publication_targets")
    if not isinstance(publication_targets, list) or not publication_targets:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.publication_targets_required",
                message="publication_targets must be a non-empty list",
                remediation="Restore at least one publication target on the row.",
                ref=ref,
            )
        )
        publication_targets = []
    else:
        for target in publication_targets:
            if target not in publication_target_class_vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="critical_dependency_register.publication_target_unknown",
                        message=(
                            f"publication_targets entry {target!r} is not in "
                            "the row schema's publication_target_class enum"
                        ),
                        remediation="Use third_party_notice / spdx_sbom / cyclonedx_sbom / provenance_statement / docs_pack_manifest only.",
                        ref=ref,
                    )
                )

    action = row.get("release_notice_action_class")
    if action not in release_notice_action_class_vocab:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.release_notice_action_class_unknown",
                message=(
                    f"release_notice_action_class {action!r} is not in the "
                    "row schema's release_notice_action_class enum"
                ),
                remediation="Use a closed release-notice-action vocabulary member.",
                ref=ref,
            )
        )

    targets_set = set(publication_targets)

    if template_class == "runtime_dependency":
        if "third_party_notice" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.runtime_dependency_publication_targets_must_include_third_party_notice",
                    message=(
                        "runtime_dependency rows MUST include third_party_notice "
                        "in publication_targets"
                    ),
                    remediation="Add third_party_notice to publication_targets.",
                    ref=ref,
                )
            )
        if "spdx_sbom" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.runtime_dependency_publication_targets_must_include_spdx_sbom",
                    message=(
                        "runtime_dependency rows MUST include spdx_sbom in "
                        "publication_targets"
                    ),
                    remediation="Add spdx_sbom to publication_targets.",
                    ref=ref,
                )
            )
        if "provenance_statement" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.runtime_dependency_publication_targets_must_include_provenance_statement",
                    message=(
                        "runtime_dependency rows MUST include provenance_statement "
                        "in publication_targets"
                    ),
                    remediation="Add provenance_statement to publication_targets.",
                    ref=ref,
                )
            )
        if action not in {
            "emit_third_party_notice_and_sbom_entries",
            "hold_pending_first_admission",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.runtime_dependency_release_notice_action_must_match_template",
                    message=(
                        "runtime_dependency rows MUST use "
                        "release_notice_action_class = "
                        "emit_third_party_notice_and_sbom_entries or "
                        f"hold_pending_first_admission; got {action!r}"
                    ),
                    remediation="Restore the matching action for runtime_dependency rows.",
                    ref=ref,
                )
            )
    elif template_class == "bundled_asset":
        if "provenance_statement" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.bundled_asset_publication_targets_must_include_provenance_statement",
                    message=(
                        "bundled_asset rows MUST include provenance_statement "
                        "in publication_targets"
                    ),
                    remediation="Add provenance_statement to publication_targets.",
                    ref=ref,
                )
            )
        if action not in {
            "emit_bundled_asset_notice_when_imported",
            "hold_pending_first_admission",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.bundled_asset_release_notice_action_must_match_template",
                    message=(
                        "bundled_asset rows MUST use "
                        "release_notice_action_class = "
                        "emit_bundled_asset_notice_when_imported or "
                        f"hold_pending_first_admission; got {action!r}"
                    ),
                    remediation="Restore the matching action for bundled_asset rows.",
                    ref=ref,
                )
            )
    elif template_class == "build_tooling":
        if "third_party_notice" in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.build_tooling_publication_targets_must_not_include_third_party_notice",
                    message=(
                        "build_tooling rows MUST NOT include third_party_notice "
                        "in publication_targets; build tooling is never redistributed"
                    ),
                    remediation="Drop third_party_notice; build tooling only contributes provenance + SBOM.",
                    ref=ref,
                )
            )
        if "provenance_statement" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.build_tooling_publication_targets_must_include_provenance_statement",
                    message=(
                        "build_tooling rows MUST include provenance_statement "
                        "in publication_targets"
                    ),
                    remediation="Add provenance_statement to publication_targets.",
                    ref=ref,
                )
            )
        if action != "emit_build_tooling_provenance_record_only":
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.build_tooling_release_notice_action_must_match_template",
                    message=(
                        "build_tooling rows MUST use release_notice_action_class "
                        f"= emit_build_tooling_provenance_record_only; got {action!r}"
                    ),
                    remediation="Restore emit_build_tooling_provenance_record_only.",
                    ref=ref,
                )
            )
    elif template_class == "host_runtime":
        if targets_set != {"provenance_statement"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.host_runtime_publication_targets_must_be_provenance_only",
                    message=(
                        "host_runtime rows MUST publish exactly "
                        "[provenance_statement] in publication_targets; got "
                        f"{sorted(targets_set)!r}"
                    ),
                    remediation="Restore publication_targets to [provenance_statement] only; host runtimes are never redistributed and never contribute SBOM entries.",
                    ref=ref,
                )
            )
        if action != "emit_host_runtime_environment_capture_only":
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.host_runtime_release_notice_action_must_match_template",
                    message=(
                        "host_runtime rows MUST use release_notice_action_class "
                        f"= emit_host_runtime_environment_capture_only; got {action!r}"
                    ),
                    remediation="Restore emit_host_runtime_environment_capture_only.",
                    ref=ref,
                )
            )
    elif template_class == "mirrored_pack":
        if "docs_pack_manifest" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.mirrored_pack_publication_targets_must_include_docs_pack_manifest",
                    message=(
                        "mirrored_pack rows MUST include docs_pack_manifest in "
                        "publication_targets"
                    ),
                    remediation="Add docs_pack_manifest to publication_targets.",
                    ref=ref,
                )
            )
        if "provenance_statement" not in targets_set:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.mirrored_pack_publication_targets_must_include_provenance_statement",
                    message=(
                        "mirrored_pack rows MUST include provenance_statement "
                        "in publication_targets"
                    ),
                    remediation="Add provenance_statement to publication_targets.",
                    ref=ref,
                )
            )
        if action != "emit_docs_pack_manifest_attribution":
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.mirrored_pack_release_notice_action_must_match_template",
                    message=(
                        "mirrored_pack rows MUST use release_notice_action_class "
                        f"= emit_docs_pack_manifest_attribution; got {action!r}"
                    ),
                    remediation="Restore emit_docs_pack_manifest_attribution.",
                    ref=ref,
                )
            )

    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not OWNER_DRI_PATTERN.match(owner_dri):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.owner_dri_pattern_invalid",
                message=(
                    f"owner_dri {owner_dri!r} must match {OWNER_DRI_PATTERN.pattern!r}"
                ),
                remediation="Use an @handle for the owner DRI.",
                ref=ref,
            )
        )

    fork_trigger = row.get("fork_or_replace_trigger")
    if not isinstance(fork_trigger, str) or not fork_trigger.strip():
        if protected_path_class == "protected_path_critical":
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.fork_or_replace_trigger_required_for_protected_path",
                    message=(
                        "protected_path_class = protected_path_critical rows "
                        "MUST publish a non-empty fork_or_replace_trigger"
                    ),
                    remediation="Restore the protected-path row's fork_or_replace_trigger so the seed cannot anonymise a release-critical upstream choice.",
                    ref=ref,
                )
            )
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.fork_or_replace_trigger_required",
                    message="fork_or_replace_trigger must be a non-empty string",
                    remediation="Restore the row's fork_or_replace_trigger.",
                    ref=ref,
                )
            )

    if action == "hold_pending_first_admission" and admission_state_class not in {
        "selected_not_admitted",
        "reserved_not_yet_imported",
    }:
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.hold_pending_first_admission_blocked_when_admitted",
                message=(
                    "release_notice_action_class = hold_pending_first_admission "
                    "is only valid when admission_state_class is "
                    "selected_not_admitted or reserved_not_yet_imported; got "
                    f"{admission_state_class!r}"
                ),
                remediation="Either set admission_state_class to a pre-admission state or pick a non-held action.",
                ref=ref,
            )
        )

    evidence_refs = row.get("evidence_refs")
    if not isinstance(evidence_refs, list):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.evidence_refs_must_be_list",
                message="evidence_refs must be a list (may be empty)",
                remediation="Set evidence_refs to a YAML list.",
                ref=ref,
            )
        )

    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        findings.append(
            Finding(
                severity="error",
                check_id="critical_dependency_register.failure_drill_required",
                message="failure_drill must be a non-null object on every row",
                remediation="Restore the row's named failure drill.",
                ref=ref,
            )
        )


def load_companion_register(repo_root: Path, rel: str) -> dict[str, dict[str, Any]]:
    path = repo_root / rel
    payload = ensure_dict(render_yaml_as_json(path), rel)
    rows = ensure_list(payload.get("rows"), f"{rel}.rows")
    out: dict[str, dict[str, Any]] = {}
    for row in rows:
        row = ensure_dict(row, f"{rel}.rows[]")
        row_id = row.get("id")
        if isinstance(row_id, str) and row_id.strip():
            out[row_id] = row
    return out


def build_draft_payload(
    matrix: dict[str, Any],
    repo_root: Path,
) -> dict[str, Any]:
    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    entries: list[dict[str, Any]] = []
    grouped: dict[str, list[dict[str, Any]]] = {}
    for row in rows:
        if not isinstance(row, dict):
            continue
        entry = {
            "register_entry_id": row.get("register_entry_id"),
            "source_register": row.get("source_register"),
            "source_id": row.get("source_id"),
            "name": row.get("name"),
            "template_class": row.get("template_class"),
            "criticality_class": row.get("criticality_class"),
            "protected_path_class": row.get("protected_path_class"),
            "license_class": row.get("license_class"),
            "provenance_status_class": row.get("provenance_status_class"),
            "admission_state_class": row.get("admission_state_class"),
            "publication_targets": list(row.get("publication_targets", [])),
            "release_notice_action_class": row.get("release_notice_action_class"),
            "owner_dri": row.get("owner_dri"),
            "fork_or_replace_trigger": (row.get("fork_or_replace_trigger") or "").strip(),
        }
        entries.append(entry)
        grouped.setdefault(entry["release_notice_action_class"] or "", []).append(
            entry
        )
    return {
        "draft_kind": "critical_dependency_register_notice_draft",
        "schema_version": 1,
        "matrix_id": matrix.get("matrix_id"),
        "matrix_status": matrix.get("status"),
        "matrix_as_of": matrix.get("as_of"),
        "owner_dri": matrix.get("owner_dri"),
        "overview_page": matrix.get("overview_page"),
        "row_schema_ref": matrix.get("row_schema_ref"),
        "companion_registers": matrix.get("companion_registers"),
        "draft_output_refs": matrix.get("draft_output_refs"),
        "build_identity_ref": matrix.get("build_identity_ref"),
        "validation_lane_ref": matrix.get("validation_lane_ref"),
        "entries": entries,
        "entries_by_action": grouped,
    }


SECTION_TITLES = {
    "emit_third_party_notice_and_sbom_entries": "Third-party notice + SBOM entries",
    "emit_bundled_asset_notice_when_imported": "Bundled-asset notice (when imported)",
    "emit_build_tooling_provenance_record_only": "Build-tooling provenance records (no third-party notice)",
    "emit_host_runtime_environment_capture_only": "Host-runtime environment capture (provenance only)",
    "emit_docs_pack_manifest_attribution": "Docs-pack manifest attribution",
    "hold_pending_first_admission": "Held pending first admission",
}


SECTION_ORDER = [
    "emit_third_party_notice_and_sbom_entries",
    "emit_bundled_asset_notice_when_imported",
    "emit_build_tooling_provenance_record_only",
    "emit_host_runtime_environment_capture_only",
    "emit_docs_pack_manifest_attribution",
    "hold_pending_first_admission",
]


def render_draft_markdown(payload: dict[str, Any]) -> str:
    matrix_id = payload.get("matrix_id") or ""
    owner = payload.get("owner_dri") or ""
    as_of = payload.get("matrix_as_of") or ""
    overview = payload.get("overview_page") or ""

    lines: list[str] = []
    lines.append("# Critical-dependency register notice draft")
    lines.append("")
    lines.append(
        "Generated by `tools/governance/build_dependency_notice_seed.py` from "
        f"`{payload.get('companion_registers', {}).get('release_notice_seed', '')}` and "
        f"`artifacts/governance/critical_dependency_register.yaml` (`{matrix_id}`, owner {owner}, as of {as_of}). "
        "Reviewer entry point: "
        f"[`{overview}`](../../../{overview})."
    )
    lines.append("")
    lines.append(
        "This draft is deterministic and is regenerated from the register; "
        "edit the seed, not this file. Each section below corresponds to one "
        "`release_notice_action_class` and represents one path through the "
        "release-notice / SBOM / provenance / docs-pack-manifest pipeline."
    )
    lines.append("")

    grouped = payload.get("entries_by_action", {})
    for action in SECTION_ORDER:
        rows = grouped.get(action, [])
        if not rows:
            continue
        lines.append(f"## {SECTION_TITLES.get(action, action)}")
        lines.append("")
        lines.append(
            f"`release_notice_action_class = {action}` "
            f"({len(rows)} row{'s' if len(rows) != 1 else ''})."
        )
        lines.append("")
        lines.append(
            "| `register_entry_id` | `name` | `source_id` | `template_class` | `publication_targets` | `admission_state_class` | `owner_dri` |"
        )
        lines.append(
            "| --- | --- | --- | --- | --- | --- | --- |"
        )
        for r in sorted(rows, key=lambda r: r.get("register_entry_id") or ""):
            targets = ", ".join(r.get("publication_targets") or [])
            lines.append(
                "| `{eid}` | {name} | `{sid}` | `{tpl}` | `{tgt}` | `{adm}` | `{own}` |".format(
                    eid=r.get("register_entry_id") or "",
                    name=r.get("name") or "",
                    sid=r.get("source_id") or "",
                    tpl=r.get("template_class") or "",
                    tgt=targets,
                    adm=r.get("admission_state_class") or "",
                    own=r.get("owner_dri") or "",
                )
            )
        lines.append("")

    lines.append("## Provenance trail")
    lines.append("")
    lines.append(
        "- Build-identity record: "
        f"`{payload.get('build_identity_ref', '')}`"
    )
    lines.append(
        "- Validation lane: "
        f"`{payload.get('validation_lane_ref', '')}`"
    )
    lines.append(
        "- Companion dependency register: "
        f"`{payload.get('companion_registers', {}).get('dependency_register', '')}`"
    )
    lines.append(
        "- Companion third-party import register: "
        f"`{payload.get('companion_registers', {}).get('third_party_import_register', '')}`"
    )
    lines.append(
        "- Companion release-notice seed: "
        f"`{payload.get('companion_registers', {}).get('release_notice_seed', '')}`"
    )
    lines.append("")
    return "\n".join(lines) + "\n"


def cross_register_check(
    matrix: dict[str, Any],
    repo_root: Path,
    findings: list[Finding],
    seed_rel: str,
    omit_entry_id: str | None,
) -> None:
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

    dep_rows = load_companion_register(repo_root, dep_rel)
    imp_rows = load_companion_register(repo_root, imp_rel)

    seed_source_ids_by_register: dict[str, set[str]] = {
        "dependency_register": set(),
        "third_party_import_register": set(),
    }
    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    for row in rows:
        if not isinstance(row, dict):
            continue
        entry_id = row.get("register_entry_id") or "<missing>"
        if omit_entry_id is not None and entry_id == omit_entry_id:
            continue
        src_reg = row.get("source_register")
        src_id = row.get("source_id")
        ref = f"{seed_rel}#{entry_id}"
        if src_reg == "dependency_register":
            seed_source_ids_by_register["dependency_register"].add(src_id)
            if isinstance(src_id, str) and src_id not in dep_rows:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="critical_dependency_register.source_id_not_found_in_companion_register",
                        message=(
                            f"source_id {src_id!r} does not resolve in "
                            f"{dep_rel}"
                        ),
                        remediation="Fix source_id or add the missing upstream row in the companion dependency register.",
                        ref=ref,
                    )
                )
        elif src_reg == "third_party_import_register":
            seed_source_ids_by_register["third_party_import_register"].add(src_id)
            if isinstance(src_id, str) and src_id not in imp_rows:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="critical_dependency_register.source_id_not_found_in_companion_register",
                        message=(
                            f"source_id {src_id!r} does not resolve in "
                            f"{imp_rel}"
                        ),
                        remediation="Fix source_id or add the missing upstream row in the companion import register.",
                        ref=ref,
                    )
                )

    for dep_id, dep_row in dep_rows.items():
        criticality = dep_row.get("criticality")
        if (
            isinstance(criticality, str)
            and criticality in CRITICAL_DEP_CRITICALITY_CLASSES
            and dep_id not in seed_source_ids_by_register["dependency_register"]
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="critical_dependency_register.companion_dependency_missing_seed_entry",
                    message=(
                        f"critical dependency row {dep_id!r} (criticality "
                        f"{criticality!r}) has no matching seed entry in "
                        f"{seed_rel}; the M1 seed must surface every critical "
                        "upstream choice so the release-notice draft pipeline "
                        "can reason about it"
                    ),
                    remediation=(
                        "Add a critical_dependency_register row that cites "
                        f"source_register=dependency_register and source_id={dep_id}, "
                        "or reduce the upstream row's criticality with an explicit "
                        "decision row in the dependency register."
                    ),
                    ref=f"{dep_rel}#{dep_id}",
                )
            )

    for imp_id, imp_row in imp_rows.items():
        criticality = imp_row.get("criticality")
        if (
            isinstance(criticality, str)
            and criticality in CRITICAL_DEP_CRITICALITY_CLASSES
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
                        f"{seed_rel}; the M1 seed must surface every critical "
                        "import so the release-notice draft pipeline can "
                        "reason about it"
                    ),
                    remediation=(
                        "Add a critical_dependency_register row that cites "
                        f"source_register=third_party_import_register and source_id={imp_id}, "
                        "or reduce the upstream row's criticality with an explicit "
                        "decision row in the import register."
                    ),
                    ref=f"{imp_rel}#{imp_id}",
                )
            )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--seed",
        default=DEFAULT_SEED_REL,
        help="Critical-dependency register seed YAML path, repo-relative.",
    )
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "--write",
        action="store_true",
        help="Regenerate the draft outputs (default).",
    )
    group.add_argument(
        "--check",
        action="store_true",
        help="Compare on-disk drafts to what the tool would emit; do not write.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Simulate omitting a seed row to prove the omission gate fires. "
            "Form: 'omit:<register_entry_id>'."
        ),
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    seed_rel = args.seed
    matrix = ensure_dict(
        render_yaml_as_json(repo_root / seed_rel), seed_rel
    )

    if matrix.get("matrix_id") != EXPECTED_MATRIX_ID:
        raise SystemExit(
            f"seed matrix_id must be {EXPECTED_MATRIX_ID!r}; got "
            f"{matrix.get('matrix_id')!r}"
        )

    row_schema_ref = ensure_str(matrix.get("row_schema_ref"), "matrix.row_schema_ref")
    template_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "template_class")
    )
    criticality_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "criticality_class")
    )
    protected_path_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "protected_path_class")
    )
    license_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "license_class")
    )
    provenance_status_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "provenance_status_class")
    )
    admission_state_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "admission_state_class")
    )
    publication_target_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "publication_target_class")
    )
    release_notice_action_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "release_notice_action_class")
    )
    source_register_class_vocab = set(
        load_schema_enum(repo_root, row_schema_ref, "source_register_class")
    )

    omit_entry_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill or not args.force_drill.startswith("omit:"):
            raise SystemExit(
                "--force-drill must be of the form 'omit:<register_entry_id>'"
            )
        omit_entry_id = args.force_drill.split(":", 1)[1].strip()

    findings: list[Finding] = []
    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    effective_matrix = copy.deepcopy(matrix)
    if omit_entry_id is not None:
        effective_matrix["entries"] = [
            r
            for r in rows
            if isinstance(r, dict)
            and r.get("register_entry_id") != omit_entry_id
        ]

    for raw_row in effective_matrix.get("entries", []):
        row = ensure_dict(raw_row, "matrix.entries[]")
        validate_row(
            row,
            findings=findings,
            seed_rel=seed_rel,
            template_class_vocab=template_class_vocab,
            criticality_class_vocab=criticality_class_vocab,
            protected_path_class_vocab=protected_path_class_vocab,
            license_class_vocab=license_class_vocab,
            provenance_status_class_vocab=provenance_status_class_vocab,
            admission_state_class_vocab=admission_state_class_vocab,
            publication_target_class_vocab=publication_target_class_vocab,
            release_notice_action_class_vocab=release_notice_action_class_vocab,
            source_register_class_vocab=source_register_class_vocab,
        )

    cross_register_check(
        effective_matrix, repo_root, findings, seed_rel, omit_entry_id
    )

    draft_payload = build_draft_payload(effective_matrix, repo_root)
    draft_markdown = render_draft_markdown(draft_payload)

    draft_refs = ensure_dict(
        matrix.get("draft_output_refs"), "matrix.draft_output_refs"
    )
    md_rel = ensure_str(
        draft_refs.get("draft_notice_markdown"),
        "matrix.draft_output_refs.draft_notice_markdown",
    )
    json_rel = ensure_str(
        draft_refs.get("draft_notice_json"),
        "matrix.draft_output_refs.draft_notice_json",
    )
    md_path = repo_root / md_rel
    json_path = repo_root / json_rel

    mode = "check" if args.check else "write"

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for finding in findings:
            prefix = "ERROR" if finding.severity == "error" else "WARN"
            ref_suffix = f" [{finding.ref}]" if finding.ref else ""
            print(
                f"[build-dependency-notice-seed] {prefix} {finding.check_id}: "
                f"{finding.message}{ref_suffix}"
            )
            print(
                f"[build-dependency-notice-seed]   remediation: "
                f"{finding.remediation}"
            )
        return 1

    new_json = json.dumps(draft_payload, indent=2, sort_keys=True) + "\n"
    if mode == "check":
        ok = True
        if not md_path.exists() or md_path.read_text(encoding="utf-8") != draft_markdown:
            print(
                f"[build-dependency-notice-seed] DRIFT in {md_rel}; "
                "re-run without --check to refresh."
            )
            ok = False
        if not json_path.exists() or json_path.read_text(encoding="utf-8") != new_json:
            print(
                f"[build-dependency-notice-seed] DRIFT in {json_rel}; "
                "re-run without --check to refresh."
            )
            ok = False
        if not ok:
            return 1
        print(
            f"[build-dependency-notice-seed] OK ({mode}) — draft outputs match seed."
        )
        return 0

    md_path.parent.mkdir(parents=True, exist_ok=True)
    md_path.write_text(draft_markdown, encoding="utf-8")
    json_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(new_json, encoding="utf-8")
    print(
        f"[build-dependency-notice-seed] WROTE {md_rel} and {json_rel} "
        f"(generated at {now_iso_z()})"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[build-dependency-notice-seed] interrupted", file=sys.stderr)
        sys.exit(130)
