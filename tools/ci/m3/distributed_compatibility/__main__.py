#!/usr/bin/env python3
"""Generate beta distributed compatibility manifests and skew-harness reports."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_COMPAT_REPORT_REL = "artifacts/compat/m3/compatibility_report.json"
DEFAULT_SKEW_WINDOWS_REL = "artifacts/compat/skew_windows.yaml"
DEFAULT_VERSION_SKEW_REGISTER_REL = "artifacts/compat/version_skew_register.yaml"
DEFAULT_OUTPUT_DIR_REL = "artifacts/compat/m3/distributed_manifests"
DEFAULT_HARNESS_MANIFEST_REL = "fixtures/release/m3/skew_harness/manifest.yaml"
DEFAULT_RELEASE_PACKET_REL = "artifacts/release/m3/distributed_compatibility/release_packet.json"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/distributed_compatibility/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/compat/m3/captures/distributed_compatibility_validation_capture.json"
)
DEFAULT_DOC_REL = "docs/release/m3/distributed_compatibility_beta.md"
DEFAULT_MANIFEST_SCHEMA_REL = "schemas/release/distributed_compatibility_manifest.schema.json"
DEFAULT_HARNESS_SCHEMA_REL = "schemas/release/distributed_skew_harness_case.schema.json"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_MANIFEST_RECORD_KIND = "distributed_compatibility_manifest"
EXPECTED_INDEX_RECORD_KIND = "distributed_compatibility_manifest_index"
EXPECTED_RELEASE_PACKET_RECORD_KIND = "distributed_compatibility_release_packet"
EXPECTED_SUPPORT_EXPORT_RECORD_KIND = "distributed_compatibility_support_export"
EXPECTED_HARNESS_CASE_RECORD_KIND = "distributed_skew_harness_case"
EXPECTED_HARNESS_REPORT_RECORD_KIND = "distributed_skew_harness_report"

REQUIRED_FAMILIES = ("client_helper", "client_extension", "schema", "provider")

FAMILY_LABELS = {
    "client_helper": "Client/helper and remote-agent boundaries",
    "client_extension": "Client/extension host and SDK boundaries",
    "schema": "Schema, state, command, and event producer/consumer boundaries",
    "provider": "Provider API, service family, and browser-handoff boundaries",
}

FAMILY_ROW_SCOPES = {
    "client_helper": {"helper"},
    "client_extension": {"extension"},
    "schema": {"schema"},
    "provider": {"provider"},
}

DEFAULT_BOUNDARY_FIELDS_BY_WINDOW_CLASS = {
    "coordinated_artifact_set_only": [
        "coordinated_artifact_set_ref",
        "contract_version",
        "schema_epoch",
    ],
    "declared_adjacent_window": [
        "client_version",
        "agent_version",
        "min_protocol",
        "max_protocol",
        "toolchain_manifest_epoch",
    ],
    "published_sdk_support_window": [
        "sdk_range",
        "wit_abi_version",
        "permission_vocabulary_version",
    ],
    "same_schema_epoch_additive_only": [
        "schema_family",
        "schema_epoch",
        "additive_field_posture",
    ],
    "current_plus_previous_minor_or_lts": [
        "api_version",
        "tenancy_region_epoch",
        "capability_policy_ref",
    ],
}

DEFAULT_REPAIR_HINTS_BY_POSTURE = {
    "fail_closed": ["upgrade_component", "import_compatible_bundle"],
    "read_only": ["use_cached_read_only_mode", "upgrade_component"],
    "degraded": ["upgrade_component", "use_file_or_review_only_mode"],
    "explicitly_unsupported": ["upgrade_component", "contact_admin_or_support"],
}

PATH_LIKE_SUFFIXES = (".json", ".yaml", ".yml", ".md", ".py", ".rs")
OPAQUE_PREFIXES = (
    "beta_surface:",
    "build-id:",
    "claim_manifest:",
    "cohort:",
    "compat_report:",
    "compat_row:",
    "distributed_compat:",
    "distributed_compat_row:",
    "release_packet:",
    "skew_case:",
    "skew_harness:",
    "skew_register:",
    "skew_window:",
    "support_export:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--compat-report", default=DEFAULT_COMPAT_REPORT_REL)
    parser.add_argument("--skew-windows", default=DEFAULT_SKEW_WINDOWS_REL)
    parser.add_argument("--version-skew-register", default=DEFAULT_VERSION_SKEW_REGISTER_REL)
    parser.add_argument("--output-dir", default=DEFAULT_OUTPUT_DIR_REL)
    parser.add_argument("--harness-manifest", default=DEFAULT_HARNESS_MANIFEST_REL)
    parser.add_argument("--release-packet", default=DEFAULT_RELEASE_PACKET_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--manifest-schema", default=DEFAULT_MANIFEST_SCHEMA_REL)
    parser.add_argument("--harness-schema", default=DEFAULT_HARNESS_SCHEMA_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated artifacts or validation capture would change.",
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
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def normalize_generated_at(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def write_if_changed(path: Path, content: str, check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    existing: str | None = None
    if path.exists():
        existing = path.read_text(encoding="utf-8")
    changed = existing is None or normalize_generated_at(existing) != normalize_generated_at(content)
    if not check_only:
        path.write_text(content, encoding="utf-8")
    return changed


def rel_for(output_dir_rel: str, file_name: str) -> str:
    return f"{output_dir_rel.rstrip('/')}/{file_name}"


def index_skew_windows(payload: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("skew_window_id"), "skew_windows.declarations[].skew_window_id"): row
        for row in ensure_list(payload.get("declarations"), "skew_windows.declarations")
        if isinstance(row, dict)
    }


def index_register(payload: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("register_id"), "version_skew_register.register[].register_id"): row
        for row in ensure_list(payload.get("register"), "version_skew_register.register")
        if isinstance(row, dict)
    }


def skew_cases(register_entry: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    cases: dict[str, list[dict[str, Any]]] = {}
    for status in ("supported", "best_effort", "untested", "unsupported"):
        entries: list[dict[str, Any]] = []
        for raw_case in ensure_list(
            register_entry.get(status, []),
            f"{register_entry.get('register_id')}.{status}",
        ):
            case = ensure_dict(raw_case, f"{register_entry.get('register_id')}.{status}[]")
            entries.append(
                {
                    "skew_case_ref": ensure_str(
                        case.get("skew_case_id"),
                        f"{register_entry.get('register_id')}.{status}.skew_case_id",
                    ),
                    "status": status,
                    "combination_label": ensure_str(
                        case.get("combination_label"),
                        f"{register_entry.get('register_id')}.{status}.combination_label",
                    ),
                    "window_class": ensure_str(
                        case.get("window_class"),
                        f"{register_entry.get('register_id')}.{status}.window_class",
                    ),
                    "outside_window_posture": ensure_str(
                        case.get("outside_window_posture"),
                        f"{register_entry.get('register_id')}.{status}.outside_window_posture",
                    ),
                    "notes": case.get("notes"),
                }
            )
        cases[status] = entries
    return cases


def skew_case_status_lookup(register_entry: dict[str, Any]) -> dict[str, str]:
    lookup: dict[str, str] = {}
    for status, entries in skew_cases(register_entry).items():
        for entry in entries:
            lookup[entry["skew_case_ref"]] = status
    return lookup


def looks_like_repo_ref(ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    if not clean or clean.startswith(OPAQUE_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def repo_ref_exists(repo_root: Path, ref: str) -> bool:
    return (repo_root / ref.split("#", 1)[0]).exists()


def validate_repo_ref(
    repo_root: Path,
    ref: str,
    findings: list[Finding],
    check_id: str,
    owner: str,
) -> None:
    if looks_like_repo_ref(ref) and not repo_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                "Add the missing artifact or correct the generated reference.",
                owner,
            )
        )


def inline_declaration_for(row: dict[str, Any]) -> dict[str, Any]:
    skew_window = ensure_dict(row.get("skew_window"), "row.skew_window")
    downgrade = ensure_dict(row.get("downgrade_behavior"), "row.downgrade_behavior")
    window_class = ensure_str(skew_window.get("window_class"), "row.skew_window.window_class")
    posture = ensure_str(
        downgrade.get("out_of_window_posture"),
        "row.downgrade_behavior.out_of_window_posture",
    )
    return {
        "boundary_family": "inline_report_window",
        "supported_window": {
            "window_class": window_class,
            "summary": ensure_str(skew_window.get("summary"), "row.skew_window.summary"),
        },
        "upgrade_order": {
            "declared_order": ["refer_to_compatibility_report_row"],
            "notes": "Inline report window; no wider rolling-upgrade order is claimed.",
        },
        "rollback_order": {
            "declared_order": ["refer_to_compatibility_report_row"],
            "notes": "Inline report window; rollback follows the row's downgrade behavior.",
        },
        "downgrade_behavior": {
            "support_class": ensure_str(
                downgrade.get("support_class"),
                "row.downgrade_behavior.support_class",
            ),
            "state_preservation_note": ensure_str(
                downgrade.get("state_preservation_note"),
                "row.downgrade_behavior.state_preservation_note",
            ),
        },
        "unsupported_state_behavior": {
            "state_class": "unsupported",
            "out_of_window_posture": posture,
            "contract_rule": ensure_str(
                downgrade.get("contract_rule"),
                "row.downgrade_behavior.contract_rule",
            ),
        },
        "reserved_boundary_fields": DEFAULT_BOUNDARY_FIELDS_BY_WINDOW_CLASS.get(
            window_class, ["compatibility_row_ref", "version_skew_register_ref"]
        ),
        "default_repair_hints": DEFAULT_REPAIR_HINTS_BY_POSTURE.get(
            posture, ["contact_admin_or_support"]
        ),
        "downstream_consumers": ["compatibility_report", "support_bundle"],
    }


def declaration_for_row(
    row: dict[str, Any],
    skew_window_index: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    skew_window = ensure_dict(row.get("skew_window"), "row.skew_window")
    skew_window_id = skew_window.get("skew_window_id")
    if isinstance(skew_window_id, str) and skew_window_id:
        return skew_window_index.get(skew_window_id) or inline_declaration_for(row)
    return inline_declaration_for(row)


def slug_from_ref(ref: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", ref.split(":", 1)[-1].lower()).strip("_")


def row_to_manifest_entry(
    row: dict[str, Any],
    family: str,
    skew_window_index: dict[str, dict[str, Any]],
    skew_register_index: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    row_id = ensure_str(row.get("row_id"), "compat_report.rows[].row_id")
    skew_window = ensure_dict(row.get("skew_window"), f"{row_id}.skew_window")
    register_ref = ensure_str(
        skew_window.get("version_skew_register_ref"),
        f"{row_id}.version_skew_register_ref",
    )
    register_entry = ensure_dict(
        skew_register_index.get(register_ref),
        f"version_skew_register.{register_ref}",
    )
    declaration = declaration_for_row(row, skew_window_index)
    unsupported_state = ensure_dict(
        declaration.get("unsupported_state_behavior"),
        f"{row_id}.unsupported_state_behavior",
    )
    downgrade = ensure_dict(
        declaration.get("downgrade_behavior"),
        f"{row_id}.downgrade_behavior",
    )
    upgrade = ensure_dict(declaration.get("upgrade_order"), f"{row_id}.upgrade_order")
    rollback = ensure_dict(declaration.get("rollback_order"), f"{row_id}.rollback_order")
    cases = skew_cases(register_entry)
    return {
        "manifest_row_id": f"distributed_compat_row:{family}.{slug_from_ref(row_id)}",
        "compatibility_report_row_ref": ensure_str(
            row.get("report_row_id"), f"{row_id}.report_row_id"
        ),
        "compatibility_row_ref": row_id,
        "row_scope": ensure_str(row.get("row_scope"), f"{row_id}.row_scope"),
        "artifact_or_protocol_boundary_label": ensure_str(
            row.get("artifact_or_protocol_boundary_label"),
            f"{row_id}.artifact_or_protocol_boundary_label",
        ),
        "claimed_surface": ensure_str(row.get("claimed_surface"), f"{row_id}.claimed_surface"),
        "support_class": ensure_dict(row.get("support_class"), f"{row_id}.support_class"),
        "client_scope": ensure_dict(row.get("client_scope"), f"{row_id}.client_scope"),
        "skew_window": {
            "skew_window_id": skew_window.get("skew_window_id"),
            "window_class": ensure_str(
                skew_window.get("window_class"),
                f"{row_id}.skew_window.window_class",
            ),
            "summary": ensure_str(
                skew_window.get("summary"),
                f"{row_id}.skew_window.summary",
            ),
            "version_skew_register_ref": register_ref,
            "current_skew_case_ref": ensure_str(
                skew_window.get("current_skew_case_ref"),
                f"{row_id}.skew_window.current_skew_case_ref",
            ),
        },
        "boundary_family": ensure_str(
            declaration.get("boundary_family"), f"{row_id}.boundary_family"
        ),
        "negotiation_fields": [
            ensure_str(value, f"{row_id}.reserved_boundary_fields[]")
            for value in ensure_list(
                declaration.get("reserved_boundary_fields"),
                f"{row_id}.reserved_boundary_fields",
            )
        ],
        "upgrade_order": {
            "declared_order": ensure_list(
                upgrade.get("declared_order"), f"{row_id}.upgrade_order.declared_order"
            ),
            "notes": ensure_str(upgrade.get("notes"), f"{row_id}.upgrade_order.notes"),
        },
        "rollback_order": {
            "declared_order": ensure_list(
                rollback.get("declared_order"),
                f"{row_id}.rollback_order.declared_order",
            ),
            "notes": ensure_str(rollback.get("notes"), f"{row_id}.rollback_order.notes"),
        },
        "downgrade_behavior": {
            "support_class": ensure_str(
                downgrade.get("support_class"), f"{row_id}.downgrade.support_class"
            ),
            "state_preservation_note": ensure_str(
                downgrade.get("state_preservation_note"),
                f"{row_id}.downgrade.state_preservation_note",
            ),
        },
        "unsupported_state_behavior": {
            "state_class": ensure_str(
                unsupported_state.get("state_class"),
                f"{row_id}.unsupported_state_behavior.state_class",
            ),
            "out_of_window_posture": ensure_str(
                unsupported_state.get("out_of_window_posture"),
                f"{row_id}.unsupported_state_behavior.out_of_window_posture",
            ),
            "contract_rule": ensure_str(
                unsupported_state.get("contract_rule"),
                f"{row_id}.unsupported_state_behavior.contract_rule",
            ),
        },
        "repair_hints": [
            ensure_str(value, f"{row_id}.default_repair_hints[]")
            for value in ensure_list(
                declaration.get("default_repair_hints"),
                f"{row_id}.default_repair_hints",
            )
        ],
        "skew_cases": cases,
        "evidence_refs": ensure_list(row.get("evidence_refs"), f"{row_id}.evidence_refs"),
        "notes": ensure_str(row.get("notes"), f"{row_id}.notes"),
    }


def select_family_rows(report: dict[str, Any], family: str) -> list[dict[str, Any]]:
    scopes = FAMILY_ROW_SCOPES[family]
    rows = [
        ensure_dict(row, "compat_report.rows[]")
        for row in ensure_list(report.get("rows"), "compat_report.rows")
        if isinstance(row, dict) and row.get("row_scope") in scopes
    ]
    return sorted(rows, key=lambda row: str(row.get("row_id")))


def compose_manifest(
    *,
    report: dict[str, Any],
    family: str,
    output_dir_rel: str,
    harness_case_refs: list[str],
    skew_window_index: dict[str, dict[str, Any]],
    skew_register_index: dict[str, dict[str, Any]],
    generated_at: str,
    args: argparse.Namespace,
) -> dict[str, Any]:
    rows = [
        row_to_manifest_entry(row, family, skew_window_index, skew_register_index)
        for row in select_family_rows(report, family)
    ]
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_MANIFEST_RECORD_KIND,
        "manifest_id": f"distributed_compat:{family}.beta",
        "manifest_family": family,
        "family_label": FAMILY_LABELS[family],
        "manifest_revision": 1,
        "release_channel_scope": ensure_str(
            report.get("release_channel_scope"), "compat_report.release_channel_scope"
        ),
        "as_of": ensure_str(report.get("as_of"), "compat_report.as_of"),
        "generated_at": generated_at,
        "source_refs": {
            "compatibility_report": args.compat_report,
            "skew_windows": args.skew_windows,
            "version_skew_register": args.version_skew_register,
            "skew_harness_manifest": args.harness_manifest,
            "manifest_schema": args.manifest_schema,
        },
        "consuming_surfaces": [
            args.release_packet,
            args.support_projection,
            args.doc,
            "docs/help/m3/release_truth_surfaces.md",
        ],
        "rows": rows,
        "harness_case_refs": harness_case_refs,
        "release_packet_ref": args.release_packet,
        "support_export_projection_ref": args.support_projection,
        "notes": (
            "Generated from the governed beta compatibility report and skew-window "
            "register; do not widen row support in this manifest by hand."
        ),
    }


def load_harness_cases(repo_root: Path, manifest_rel: str) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    manifest = ensure_dict(render_yaml_as_json(repo_root / manifest_rel), "skew_harness_manifest")
    cases: list[dict[str, Any]] = []
    for idx, ref in enumerate(ensure_list(manifest.get("case_refs"), "skew_harness.case_refs")):
        case_ref = ensure_str(ref, f"skew_harness.case_refs[{idx}]")
        case = ensure_dict(load_json(repo_root / case_ref), case_ref)
        case["_case_ref"] = case_ref
        cases.append(case)
    return manifest, cases


def manifest_ref_for_family(output_dir_rel: str, family: str) -> str:
    return rel_for(output_dir_rel, f"{family}.json")


def expected_manifest_refs(output_dir_rel: str) -> dict[str, str]:
    return {
        family: manifest_ref_for_family(output_dir_rel, family)
        for family in REQUIRED_FAMILIES
    }


def validate_case_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    findings: list[Finding],
    case_ref: str,
) -> None:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "harness_case.schema",
                f"{case_ref}:{path}: {error.message}",
                "Update the skew-harness case so it conforms to the schema.",
                case_ref,
            )
        )


def validate_generated_manifest_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    findings: list[Finding],
    manifest_ref: str,
) -> None:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "distributed_manifest.schema",
                f"{manifest_ref}:{path}: {error.message}",
                "Update the generated manifest or schema so they agree.",
                manifest_ref,
            )
        )


def validate_manifest_rows(manifest: dict[str, Any], findings: list[Finding]) -> None:
    family = ensure_str(manifest.get("manifest_family"), "manifest.manifest_family")
    rows = ensure_list(manifest.get("rows"), f"{family}.rows")
    if not rows:
        findings.append(
            Finding(
                "error",
                "manifest.family.empty",
                f"{family} manifest has no compatibility rows",
                "Seed at least one beta compatibility row for this required family.",
                family,
            )
        )
    for row in rows:
        item = ensure_dict(row, f"{family}.rows[]")
        skew_cases_payload = ensure_dict(item.get("skew_cases"), f"{family}.skew_cases")
        for status in ("supported", "unsupported"):
            if not ensure_list(skew_cases_payload.get(status), f"{family}.{status}"):
                findings.append(
                    Finding(
                        "error",
                        "manifest.skew_case_bucket.empty",
                        f"{item.get('compatibility_row_ref')} has no {status} skew case",
                        "Add the missing skew-register bucket before claiming this distributed row.",
                        str(item.get("compatibility_row_ref")),
                    )
                )
        if not ensure_list(item.get("negotiation_fields"), f"{family}.negotiation_fields"):
            findings.append(
                Finding(
                    "error",
                    "manifest.negotiation_fields.empty",
                    f"{item.get('compatibility_row_ref')} does not declare negotiation fields",
                    "Bind the row to reserved boundary fields from the skew-window declaration.",
                    str(item.get("compatibility_row_ref")),
                )
            )


def validate_harness(
    *,
    repo_root: Path,
    harness_manifest: dict[str, Any],
    harness_cases: list[dict[str, Any]],
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    skew_register_index: dict[str, dict[str, Any]],
    harness_schema: dict[str, Any],
    findings: list[Finding],
) -> None:
    declared_families = {
        ensure_str(value, "skew_harness.required_families[]")
        for value in ensure_list(
            harness_manifest.get("required_families"),
            "skew_harness.required_families",
        )
    }
    missing_families = sorted(set(REQUIRED_FAMILIES) - declared_families)
    if missing_families:
        findings.append(
            Finding(
                "error",
                "harness.required_family.missing",
                "skew harness manifest does not declare every required family",
                "Add the missing family to required_families and seed at least one supported and unsupported case.",
                details={"missing": missing_families},
            )
        )

    actual_case_ids = {
        ensure_str(case.get("case_id"), f"{case.get('_case_ref')}.case_id")
        for case in harness_cases
    }
    expected_case_ids = {
        ensure_str(value, "skew_harness.expected_case_ids[]")
        for value in ensure_list(
            harness_manifest.get("expected_case_ids"),
            "skew_harness.expected_case_ids",
        )
    }
    if actual_case_ids != expected_case_ids:
        findings.append(
            Finding(
                "error",
                "harness.case_ids.mismatch",
                "expected_case_ids must exactly match loaded cases",
                "Update the harness manifest or case refs so the protected set is explicit.",
                details={"actual": sorted(actual_case_ids), "expected": sorted(expected_case_ids)},
            )
        )

    coverage = {
        family: {"supported": 0, "unsupported": 0}
        for family in REQUIRED_FAMILIES
    }
    rows_by_family = {
        family: {
            ensure_str(row.get("compatibility_row_ref"), "row.compatibility_row_ref"): row
            for row in ensure_list(manifest.get("rows"), f"{family}.rows")
            if isinstance(row, dict)
        }
        for family, manifest in manifests.items()
    }

    for case in harness_cases:
        case_ref = ensure_str(case.get("_case_ref"), "case._case_ref")
        schema_payload = {key: value for key, value in case.items() if key != "_case_ref"}
        validate_case_schema(schema_payload, harness_schema, findings, case_ref)

        for ref in ensure_list(case.get("evidence_refs", []), f"{case_ref}.evidence_refs"):
            validate_repo_ref(
                repo_root,
                ensure_str(ref, f"{case_ref}.evidence_refs[]"),
                findings,
                "harness_case.evidence_ref.missing",
                case_ref,
            )

        family = ensure_str(case.get("family"), f"{case_ref}.family")
        if family not in REQUIRED_FAMILIES:
            findings.append(
                Finding(
                    "error",
                    "harness_case.family.invalid",
                    f"{case_ref} declares unknown family {family!r}",
                    "Use one of the required distributed manifest families.",
                    case_ref,
                )
            )
            continue
        expected_manifest_ref = manifest_refs[family]
        if case.get("manifest_ref") != expected_manifest_ref:
            findings.append(
                Finding(
                    "error",
                    "harness_case.manifest_ref.mismatch",
                    f"{case_ref} must cite {expected_manifest_ref}",
                    "Bind each case to the generated manifest for its family.",
                    case_ref,
                )
            )
        compat_ref = ensure_str(
            case.get("compatibility_row_ref"), f"{case_ref}.compatibility_row_ref"
        )
        family_rows = rows_by_family.get(family, {})
        manifest_row = family_rows.get(compat_ref)
        if manifest_row is None:
            findings.append(
                Finding(
                    "error",
                    "harness_case.compatibility_row_ref.unknown",
                    f"{case_ref} cites a row not present in the {family} manifest: {compat_ref}",
                    "Use a compatibility_row_ref emitted by the generated family manifest.",
                    case_ref,
                )
            )
            continue

        register_ref = ensure_str(
            ensure_dict(manifest_row.get("skew_window"), f"{compat_ref}.skew_window").get(
                "version_skew_register_ref"
            ),
            f"{compat_ref}.version_skew_register_ref",
        )
        register_entry = ensure_dict(
            skew_register_index.get(register_ref),
            f"version_skew_register.{register_ref}",
        )
        skew_case_ref = ensure_str(case.get("skew_case_ref"), f"{case_ref}.skew_case_ref")
        status_lookup = skew_case_status_lookup(register_entry)
        actual_status = status_lookup.get(skew_case_ref)
        if actual_status is None:
            findings.append(
                Finding(
                    "error",
                    "harness_case.skew_case_ref.unknown",
                    f"{case_ref} cites unknown skew case {skew_case_ref}",
                    "Use a skew_case_ref from the row's version-skew register.",
                    case_ref,
                )
            )
            continue
        expected_status = ensure_str(
            case.get("expected_status"), f"{case_ref}.expected_status"
        )
        if actual_status != expected_status:
            findings.append(
                Finding(
                    "error",
                    "harness_case.expected_status.mismatch",
                    f"{case_ref} expected {expected_status} but register class is {actual_status}",
                    "Align expected_status with the source version-skew register.",
                    case_ref,
                )
            )
        if expected_status in coverage[family]:
            coverage[family][expected_status] += 1
        if expected_status == "unsupported" and bool(case.get("mutation_allowed")):
            findings.append(
                Finding(
                    "error",
                    "harness_case.unsupported_allows_mutation",
                    f"{case_ref} is unsupported but allows mutation",
                    "Unsupported skew cases must fail closed, read-only, or degraded without mutation.",
                    case_ref,
                )
            )
        if not ensure_list(case.get("expected_repair_hints"), f"{case_ref}.expected_repair_hints"):
            findings.append(
                Finding(
                    "error",
                    "harness_case.repair_hints.empty",
                    f"{case_ref} has no expected repair hints",
                    "Name at least one truthful repair or continuation path.",
                    case_ref,
                )
            )

    for family, states in coverage.items():
        for status, count in states.items():
            if count == 0:
                findings.append(
                    Finding(
                        "error",
                        "harness.coverage.missing_status",
                        f"{family} has no {status} harness case",
                        "Add a protected skew-harness case for this family and status.",
                        family,
                    )
                )


def summarize_manifest(manifest: dict[str, Any], manifest_ref: str) -> dict[str, Any]:
    rows = ensure_list(manifest.get("rows"), "manifest.rows")
    supported_case_refs: list[str] = []
    unsupported_case_refs: list[str] = []
    out_of_window_postures: set[str] = set()
    effective_support_classes: set[str] = set()
    for row in rows:
        item = ensure_dict(row, "manifest.rows[]")
        cases = ensure_dict(item.get("skew_cases"), "row.skew_cases")
        supported_case_refs.extend(
            case["skew_case_ref"]
            for case in ensure_list(cases.get("supported"), "row.supported")
            if isinstance(case, dict)
        )
        unsupported_case_refs.extend(
            case["skew_case_ref"]
            for case in ensure_list(cases.get("unsupported"), "row.unsupported")
            if isinstance(case, dict)
        )
        support = ensure_dict(item.get("support_class"), "row.support_class")
        effective_support_classes.add(
            ensure_str(support.get("effective"), "row.support_class.effective")
        )
        unsupported_state = ensure_dict(
            item.get("unsupported_state_behavior"), "row.unsupported_state_behavior"
        )
        out_of_window_postures.add(
            ensure_str(
                unsupported_state.get("out_of_window_posture"),
                "row.unsupported_state_behavior.out_of_window_posture",
            )
        )
    return {
        "manifest_family": manifest["manifest_family"],
        "manifest_ref": manifest_ref,
        "manifest_id": manifest["manifest_id"],
        "row_count": len(rows),
        "compatibility_row_refs": [
            row["compatibility_row_ref"]
            for row in rows
            if isinstance(row, dict)
        ],
        "supported_case_refs": sorted(set(supported_case_refs)),
        "unsupported_case_refs": sorted(set(unsupported_case_refs)),
        "effective_support_classes": sorted(effective_support_classes),
        "out_of_window_postures": sorted(out_of_window_postures),
        "harness_case_refs": manifest["harness_case_refs"],
    }


def build_index(
    *,
    report: dict[str, Any],
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    harness_report_ref: str,
    generated_at: str,
    args: argparse.Namespace,
) -> dict[str, Any]:
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_INDEX_RECORD_KIND,
        "index_id": "distributed_compat:index.beta",
        "index_revision": 1,
        "release_channel_scope": report["release_channel_scope"],
        "as_of": report["as_of"],
        "generated_at": generated_at,
        "source_refs": {
            "compatibility_report": args.compat_report,
            "skew_windows": args.skew_windows,
            "version_skew_register": args.version_skew_register,
            "skew_harness_manifest": args.harness_manifest,
        },
        "required_families": list(REQUIRED_FAMILIES),
        "manifest_refs": [
            {
                "manifest_family": family,
                "manifest_ref": manifest_refs[family],
                "manifest_id": manifests[family]["manifest_id"],
                "row_count": len(manifests[family]["rows"]),
            }
            for family in REQUIRED_FAMILIES
        ],
        "harness_report_ref": harness_report_ref,
        "release_packet_ref": args.release_packet,
        "support_export_projection_ref": args.support_projection,
        "partner_doc_ref": args.doc,
    }


def build_harness_report(
    *,
    harness_manifest: dict[str, Any],
    harness_cases: list[dict[str, Any]],
    generated_at: str,
    args: argparse.Namespace,
    findings: list[Finding],
) -> dict[str, Any]:
    case_results = []
    for case in harness_cases:
        expected_status = ensure_str(case.get("expected_status"), f"{case.get('_case_ref')}.expected_status")
        expected_outcome = ensure_str(case.get("expected_outcome"), f"{case.get('_case_ref')}.expected_outcome")
        passed = not any(
            finding.ref == case.get("_case_ref") and finding.severity == "error"
            for finding in findings
        )
        case_results.append(
            {
                "case_id": case["case_id"],
                "case_ref": case["_case_ref"],
                "family": case["family"],
                "compatibility_row_ref": case["compatibility_row_ref"],
                "skew_case_ref": case["skew_case_ref"],
                "expected_status": expected_status,
                "expected_outcome": expected_outcome,
                "mutation_allowed": bool(case["mutation_allowed"]),
                "result": "pass" if passed else "fail",
            }
        )
    summary = {
        "total_cases": len(case_results),
        "passed_cases": sum(1 for result in case_results if result["result"] == "pass"),
        "failed_cases": sum(1 for result in case_results if result["result"] == "fail"),
        "supported_case_count": sum(
            1 for result in case_results if result["expected_status"] == "supported"
        ),
        "unsupported_case_count": sum(
            1 for result in case_results if result["expected_status"] == "unsupported"
        ),
        "mutation_blocked_case_count": sum(
            1 for result in case_results if not result["mutation_allowed"]
        ),
    }
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_HARNESS_REPORT_RECORD_KIND,
        "harness_id": ensure_str(harness_manifest.get("harness_id"), "harness.harness_id"),
        "generated_at": generated_at,
        "manifest_ref": args.harness_manifest,
        "source_manifest_index_ref": rel_for(args.output_dir, "manifest_index.json"),
        "summary": summary,
        "case_results": case_results,
    }


def build_release_packet(
    *,
    report: dict[str, Any],
    index_ref: str,
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    harness_report: dict[str, Any],
    generated_at: str,
    args: argparse.Namespace,
) -> dict[str, Any]:
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_RELEASE_PACKET_RECORD_KIND,
        "packet_id": "release_packet:distributed_compatibility.beta",
        "packet_revision": 1,
        "release_channel_scope": report["release_channel_scope"],
        "as_of": report["as_of"],
        "generated_at": generated_at,
        "source_index_ref": index_ref,
        "source_manifest_refs": [
            manifest_refs[family]
            for family in REQUIRED_FAMILIES
        ],
        "compatibility_report_ref": args.compat_report,
        "skew_harness_report_ref": rel_for(args.output_dir, "skew_harness_report.json"),
        "support_export_projection_ref": args.support_projection,
        "partner_doc_ref": args.doc,
        "private_notes_allowed": False,
        "claim_policy": {
            "stale_or_degraded_proof_policy": "downgrade_claim",
            "unsupported_window_policy": "fail_closed_or_read_only_by_row",
            "source_of_truth": "generated_distributed_manifest",
        },
        "families": [
            summarize_manifest(manifests[family], manifest_refs[family])
            for family in REQUIRED_FAMILIES
        ],
        "harness_summary": harness_report["summary"],
    }


def build_support_projection(
    *,
    release_packet: dict[str, Any],
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    generated_at: str,
    args: argparse.Namespace,
) -> dict[str, Any]:
    support_rows = []
    for family in REQUIRED_FAMILIES:
        manifest = manifests[family]
        manifest_ref = manifest_refs[family]
        for row in ensure_list(manifest.get("rows"), f"{family}.rows"):
            item = ensure_dict(row, f"{family}.rows[]")
            support = ensure_dict(item.get("support_class"), f"{family}.support_class")
            skew_window = ensure_dict(item.get("skew_window"), f"{family}.skew_window")
            unsupported_state = ensure_dict(
                item.get("unsupported_state_behavior"),
                f"{family}.unsupported_state_behavior",
            )
            skew_cases_payload = ensure_dict(item.get("skew_cases"), f"{family}.skew_cases")
            support_rows.append(
                {
                    "support_row_id": (
                        "support_export:distributed_compatibility."
                        f"{family}.{slug_from_ref(item['compatibility_row_ref'])}"
                    ),
                    "manifest_ref": manifest_ref,
                    "manifest_row_id": item["manifest_row_id"],
                    "manifest_family": family,
                    "compatibility_row_ref": item["compatibility_row_ref"],
                    "compatibility_report_row_ref": item["compatibility_report_row_ref"],
                    "support_class_effective": support["effective"],
                    "skew_window_class": skew_window["window_class"],
                    "skew_window_ref": skew_window["skew_window_id"],
                    "current_skew_case_ref": skew_window["current_skew_case_ref"],
                    "unsupported_case_refs": [
                        case["skew_case_ref"]
                        for case in ensure_list(
                            skew_cases_payload.get("unsupported"),
                            f"{family}.unsupported",
                        )
                        if isinstance(case, dict)
                    ],
                    "out_of_window_posture": unsupported_state["out_of_window_posture"],
                    "repair_hints": item["repair_hints"],
                    "release_packet_ref": args.release_packet,
                }
            )
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_SUPPORT_EXPORT_RECORD_KIND,
        "export_id": "support_export:distributed_compatibility.beta",
        "generated_at": generated_at,
        "source_index_ref": release_packet["source_index_ref"],
        "release_packet_ref": args.release_packet,
        "redaction_class": "metadata_safe_default",
        "raw_private_material_excluded": True,
        "manifest_families": [
            {
                "manifest_family": family,
                "manifest_ref": manifest_refs[family],
                "manifest_id": manifests[family]["manifest_id"],
                "row_count": len(manifests[family]["rows"]),
            }
            for family in REQUIRED_FAMILIES
        ],
        "harness_summary": release_packet["harness_summary"],
        "support_rows": support_rows,
    }


def validate_doc(
    repo_root: Path,
    doc_rel: str,
    required_refs: list[str],
    findings: list[Finding],
) -> None:
    path = repo_root / doc_rel
    if not path.exists():
        findings.append(
            Finding(
                "error",
                "doc.missing",
                f"release doc does not exist: {doc_rel}",
                "Add the partner/release doc that consumes the generated manifests.",
                doc_rel,
            )
        )
        return
    text = path.read_text(encoding="utf-8")
    required_phrases = [
        "Distributed compatibility beta manifests",
        "skew harness",
        "support export",
        "release packet",
    ]
    for phrase in required_phrases:
        if phrase not in text:
            findings.append(
                Finding(
                    "error",
                    "doc.required_phrase.missing",
                    f"release doc is missing required phrase: {phrase}",
                    "Update the doc so reviewers can see what generated data it consumes.",
                    doc_rel,
                )
            )
    for ref in required_refs:
        if ref not in text:
            findings.append(
                Finding(
                    "error",
                    "doc.generated_ref.missing",
                    f"release doc does not cite generated artifact: {ref}",
                    "Cite the generated manifest, release packet, and support projection refs verbatim.",
                    doc_rel,
                )
            )


def build_capture(
    *,
    generated_at: str,
    args: argparse.Namespace,
    findings: list[Finding],
    outputs: dict[str, bool],
    harness_report: dict[str, Any],
    release_packet: dict[str, Any],
    support_projection: dict[str, Any],
) -> dict[str, Any]:
    errors = sum(1 for finding in findings if finding.severity == "error")
    warnings = sum(1 for finding in findings if finding.severity == "warning")
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "distributed_compatibility_validation_capture",
        "generated_at": generated_at,
        "status": "pass" if errors == 0 else "fail",
        "source_refs": {
            "compatibility_report": args.compat_report,
            "skew_windows": args.skew_windows,
            "version_skew_register": args.version_skew_register,
            "skew_harness_manifest": args.harness_manifest,
        },
        "output_refs": {
            "manifest_index": rel_for(args.output_dir, "manifest_index.json"),
            "distributed_manifest_dir": args.output_dir,
            "skew_harness_report": rel_for(args.output_dir, "skew_harness_report.json"),
            "release_packet": args.release_packet,
            "support_projection": args.support_projection,
            "partner_doc": args.doc,
        },
        "summary": {
            "errors": errors,
            "warnings": warnings,
            "changed_outputs": sorted(ref for ref, changed in outputs.items() if changed),
            "harness_total_cases": harness_report["summary"]["total_cases"],
            "harness_supported_case_count": harness_report["summary"]["supported_case_count"],
            "harness_unsupported_case_count": harness_report["summary"]["unsupported_case_count"],
            "release_packet_family_count": len(release_packet["families"]),
            "support_row_count": len(support_projection["support_rows"]),
        },
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    generated_at = generated_at_now()
    findings: list[Finding] = []

    compat_report = ensure_dict(
        load_json(repo_root / args.compat_report),
        "compat_report",
    )
    skew_windows = ensure_dict(
        render_yaml_as_json(repo_root / args.skew_windows),
        "skew_windows",
    )
    skew_register = ensure_dict(
        render_yaml_as_json(repo_root / args.version_skew_register),
        "version_skew_register",
    )
    harness_manifest, harness_cases = load_harness_cases(repo_root, args.harness_manifest)
    manifest_schema = ensure_dict(
        load_json(repo_root / args.manifest_schema),
        "manifest_schema",
    )
    harness_schema = ensure_dict(
        load_json(repo_root / args.harness_schema),
        "harness_schema",
    )

    skew_window_index = index_skew_windows(skew_windows)
    skew_register_index = index_register(skew_register)
    manifest_refs = expected_manifest_refs(args.output_dir)
    cases_by_family: dict[str, list[str]] = {family: [] for family in REQUIRED_FAMILIES}
    for case in harness_cases:
        family = str(case.get("family", ""))
        if family in cases_by_family:
            cases_by_family[family].append(str(case["_case_ref"]))

    manifests = {
        family: compose_manifest(
            report=compat_report,
            family=family,
            output_dir_rel=args.output_dir,
            harness_case_refs=sorted(cases_by_family[family]),
            skew_window_index=skew_window_index,
            skew_register_index=skew_register_index,
            generated_at=generated_at,
            args=args,
        )
        for family in REQUIRED_FAMILIES
    }

    for family, manifest in manifests.items():
        validate_generated_manifest_schema(
            manifest,
            manifest_schema,
            findings,
            manifest_refs[family],
        )
        validate_manifest_rows(manifest, findings)
        for ref in ensure_list(manifest.get("consuming_surfaces"), f"{family}.consuming_surfaces"):
            if ref in {args.release_packet, args.support_projection}:
                continue
            validate_repo_ref(
                repo_root,
                ensure_str(ref, f"{family}.consuming_surfaces[]"),
                findings,
                "manifest.consuming_surface.missing",
                manifest_refs[family],
            )

    validate_harness(
        repo_root=repo_root,
        harness_manifest=harness_manifest,
        harness_cases=harness_cases,
        manifests=manifests,
        manifest_refs=manifest_refs,
        skew_register_index=skew_register_index,
        harness_schema=harness_schema,
        findings=findings,
    )

    harness_report = build_harness_report(
        harness_manifest=harness_manifest,
        harness_cases=harness_cases,
        generated_at=generated_at,
        args=args,
        findings=findings,
    )
    index_ref = rel_for(args.output_dir, "manifest_index.json")
    index = build_index(
        report=compat_report,
        manifests=manifests,
        manifest_refs=manifest_refs,
        harness_report_ref=rel_for(args.output_dir, "skew_harness_report.json"),
        generated_at=generated_at,
        args=args,
    )
    release_packet = build_release_packet(
        report=compat_report,
        index_ref=index_ref,
        manifests=manifests,
        manifest_refs=manifest_refs,
        harness_report=harness_report,
        generated_at=generated_at,
        args=args,
    )
    support_projection = build_support_projection(
        release_packet=release_packet,
        manifests=manifests,
        manifest_refs=manifest_refs,
        generated_at=generated_at,
        args=args,
    )

    validate_doc(
        repo_root,
        args.doc,
        [
            index_ref,
            args.release_packet,
            args.support_projection,
            rel_for(args.output_dir, "skew_harness_report.json"),
            *[manifest_refs[family] for family in REQUIRED_FAMILIES],
        ],
        findings,
    )

    output_contents: dict[str, str] = {
        index_ref: json_text(index),
        rel_for(args.output_dir, "skew_harness_report.json"): json_text(harness_report),
        args.release_packet: json_text(release_packet),
        args.support_projection: json_text(support_projection),
    }
    for family in REQUIRED_FAMILIES:
        output_contents[manifest_refs[family]] = json_text(manifests[family])

    changed: dict[str, bool] = {}
    for ref, content in output_contents.items():
        changed[ref] = write_if_changed(repo_root / ref, content, args.check)

    if args.check and any(changed.values()):
        findings.append(
            Finding(
                "error",
                "generated_artifacts.stale",
                "checked-in distributed compatibility artifacts are stale",
                "Run `python3 -m tools.ci.m3.distributed_compatibility --repo-root .` and commit the regenerated files.",
                details={"changed": sorted(ref for ref, value in changed.items() if value)},
            )
        )

    capture = build_capture(
        generated_at=generated_at,
        args=args,
        findings=findings,
        outputs=changed,
        harness_report=harness_report,
        release_packet=release_packet,
        support_projection=support_projection,
    )
    capture_changed = write_if_changed(repo_root / args.capture, json_text(capture), args.check)
    if args.check and capture_changed:
        findings.append(
            Finding(
                "error",
                "validation_capture.stale",
                "checked-in validation capture is stale",
                "Run the distributed compatibility generator and commit the refreshed capture.",
                args.capture,
            )
        )

    errors = [finding for finding in findings if finding.severity == "error"]
    if errors:
        for error in errors:
            ref = f" ({error.ref})" if error.ref else ""
            print(f"ERROR [{error.check_id}]{ref}: {error.message}", file=sys.stderr)
            print(f"  remediation: {error.remediation}", file=sys.stderr)
        return 1

    print(
        "distributed compatibility manifests validated "
        f"({len(support_projection['support_rows'])} support rows, "
        f"{harness_report['summary']['total_cases']} harness cases)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
