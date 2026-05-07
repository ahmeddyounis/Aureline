#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

import yaml
from jsonschema import Draft202012Validator


def repo_root() -> pathlib.Path:
    return pathlib.Path(__file__).resolve().parents[1]


def load_yaml(path: pathlib.Path) -> Any:
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def load_schema(root: pathlib.Path) -> Draft202012Validator:
    schema_path = root / "schemas/release/silent_deployment_result.schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    return Draft202012Validator(schema)


def iter_fixture_paths(root: pathlib.Path) -> list[pathlib.Path]:
    fixture_dir = root / "fixtures/release/silent_deployment_cases"
    return sorted(fixture_dir.glob("*.yaml"))


def validate_schema(validator: Draft202012Validator, path: pathlib.Path) -> list[str]:
    payload = load_yaml(path)
    errors = sorted(validator.iter_errors(payload), key=lambda e: list(e.path))
    messages: list[str] = []
    for err in errors:
        loc = "/".join(map(str, err.path))
        prefix = f"{path}:{loc}" if loc else str(path)
        messages.append(f"{prefix}: {err.message}")
    return messages


def load_id_index(root: pathlib.Path, rel: str, *, list_key: str, id_key: str) -> set[str]:
    payload = load_yaml(root / rel)
    if not isinstance(payload, dict):
        return set()
    rows = payload.get(list_key) or []
    ids: set[str] = set()
    if isinstance(rows, list):
        for row in rows:
            if isinstance(row, dict):
                value = row.get(id_key)
                if isinstance(value, str) and value:
                    ids.add(value)
    return ids


def ensure_ref_set(
    *,
    ref: Any,
    known: set[str],
    label: str,
    source_path: pathlib.Path,
    errors: list[str],
) -> None:
    if ref is None:
        return
    if isinstance(ref, str):
        if ref not in known:
            errors.append(f"{source_path}: unknown {label} '{ref}'")
        return
    if not isinstance(ref, list):
        errors.append(f"{source_path}: {label} must be a string, array, or null")
        return
    for entry in ref:
        if not isinstance(entry, str) or not entry:
            errors.append(f"{source_path}: {label} contains non-string entry")
            continue
        if entry not in known:
            errors.append(f"{source_path}: unknown {label} '{entry}'")


def validate_cross_refs(
    *,
    root: pathlib.Path,
    fixture_path: pathlib.Path,
    payload: dict[str, Any],
    known_cards: set[str],
    known_state_roots: set[str],
    known_reports: set[str],
) -> list[str]:
    errors: list[str] = []

    ensure_ref_set(
        ref=payload.get("install_profile_card_refs"),
        known=known_cards,
        label="install_profile_card_ref",
        source_path=fixture_path,
        errors=errors,
    )
    ensure_ref_set(
        ref=payload.get("state_root_refs"),
        known=known_state_roots,
        label="state_root_ref",
        source_path=fixture_path,
        errors=errors,
    )
    ensure_ref_set(
        ref=payload.get("managed_package_report_refs"),
        known=known_reports,
        label="managed_package_report_ref",
        source_path=fixture_path,
        errors=errors,
    )

    results = payload.get("results") or []
    if not isinstance(results, list):
        errors.append(f"{fixture_path}: results must be an array")
        return errors

    packet_reports = payload.get("managed_package_report_refs") or []
    if packet_reports is None:
        packet_reports_set: set[str] = set()
    elif isinstance(packet_reports, list):
        packet_reports_set = {item for item in packet_reports if isinstance(item, str)}
    else:
        packet_reports_set = set()

    for idx, record in enumerate(results):
        if not isinstance(record, dict):
            errors.append(f"{fixture_path}: results[{idx}] must be an object")
            continue
        card_ref = record.get("install_profile_card_ref")
        if isinstance(card_ref, str) and card_ref and card_ref not in known_cards:
            errors.append(f"{fixture_path}: results[{idx}].install_profile_card_ref '{card_ref}' does not resolve")
        state_refs = record.get("state_root_refs")
        if state_refs is not None:
            ensure_ref_set(
                ref=state_refs,
                known=known_state_roots,
                label="state_root_ref",
                source_path=fixture_path,
                errors=errors,
            )
        report_ref = record.get("managed_package_report_ref")
        if isinstance(report_ref, str) and report_ref:
            if report_ref not in known_reports:
                errors.append(
                    f"{fixture_path}: results[{idx}].managed_package_report_ref '{report_ref}' does not resolve"
                )
            if report_ref not in packet_reports_set:
                errors.append(
                    f"{fixture_path}: results[{idx}].managed_package_report_ref '{report_ref}' is missing from managed_package_report_refs"
                )

    return errors


def validate_required_coverage(fixture_payloads: list[dict[str, Any]]) -> list[str]:
    scenarios = {payload.get("scenario_class") for payload in fixture_payloads}
    required = {
        "side_by_side_stable_preview",
        "portable_spill_detected",
        "customer_managed_mirror",
        "policy_pinned_lane",
        "blocked_rollback",
        "uninstall_preservation",
    }
    missing = sorted(required - scenarios)
    if missing:
        return ["fixtures/release/silent_deployment_cases: missing scenario coverage: " + ", ".join(missing)]
    return []


def main() -> int:
    root = repo_root()
    validator = load_schema(root)
    fixture_paths = iter_fixture_paths(root)
    if not fixture_paths:
        print("[validate-silent-deployment-cases] error: no fixtures found", file=sys.stderr)
        return 1

    known_cards = load_id_index(
        root,
        "artifacts/release/install_topology_matrix.yaml",
        list_key="install_profile_cards",
        id_key="id",
    )
    known_state_roots = load_id_index(
        root,
        "artifacts/release/state_root_map.yaml",
        list_key="state_roots",
        id_key="id",
    )
    report_payload = load_yaml(root / "artifacts/release/managed_package_report_seed.yaml")
    known_reports: set[str] = set()
    if isinstance(report_payload, dict):
        for row in report_payload.get("fixtures") or []:
            if isinstance(row, dict):
                report_id = row.get("report_id")
                if isinstance(report_id, str) and report_id:
                    known_reports.add(report_id)

    all_errors: list[str] = []
    fixture_payloads: list[dict[str, Any]] = []
    for path in fixture_paths:
        all_errors.extend(validate_schema(validator, path))
        payload = load_yaml(path)
        if isinstance(payload, dict):
            fixture_payloads.append(payload)
            all_errors.extend(
                validate_cross_refs(
                    root=root,
                    fixture_path=path,
                    payload=payload,
                    known_cards=known_cards,
                    known_state_roots=known_state_roots,
                    known_reports=known_reports,
                )
            )
        else:
            all_errors.append(f"{path}: fixture must be a YAML mapping")

    all_errors.extend(validate_required_coverage(fixture_payloads))

    if all_errors:
        for message in all_errors:
            print(f"[validate-silent-deployment-cases] error: {message}", file=sys.stderr)
        return 1

    print("[validate-silent-deployment-cases] OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
