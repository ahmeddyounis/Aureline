#!/usr/bin/env python3
"""Validate the frozen beta protected-fitness release-candidate packet.

This is the release-gating consumer for the checked-in beta record:

  - artifacts/release/protected_fitness_packet_beta.yaml

The beta packet reuses the alpha review packet verbatim as its base
evidence (base_packet_ref) and layers release-candidate thresholds on
top. This gate validates the beta record against the canonical protected
fitness-function catalog and the reused base packet, enforcing:

  - one release-candidate bar per catalog-linked protected function;
  - a metric over its release-candidate bar without an active waiver may
    not render as passing or waived (it must degrade);
  - an over-bar metric held open by an active waiver keeps its waiver
    record visible and does not render as a clean pass; and
  - an expired waiver degrades the protected function instead of holding
    it open as passing, warning, or waived.

The Rust consumer is aureline_support::fitness::FitnessPacketBeta; this
gate mirrors its release-candidate rules so CI catches drift without a
cargo build.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_PACKET_REL = "artifacts/release/protected_fitness_packet_beta.yaml"
DEFAULT_CATALOG_REL = "artifacts/bench/fitness_function_catalog.yaml"

BETA_RECORD_KIND = "protected_fitness_release_candidate_packet"
BASE_RECORD_KIND = "protected_fitness_review_packet"
EXPECTED_BASE_PACKET_REF = "artifacts/release/protected_fitness_packet_alpha.yaml"

RELEASE_CANDIDATE_COMPARATORS = {
    "measured_at_or_below_bar",
    "ratio_at_or_above_floor",
    "boolean_must_hold",
    "provisional_bar_pending_council",
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--catalog", default=DEFAULT_CATALOG_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Optional path to write a machine-readable validation capture.",
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
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(File.read(ARGV[0]),"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
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


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def opt_str(value: Any) -> str | None:
    if value is None:
        return None
    if isinstance(value, str):
        return value.strip() or None
    return str(value)


def require_str(
    value: Any,
    check_id: str,
    ref: str,
    message: str,
    findings: list[Finding],
) -> None:
    if not isinstance(value, str) or not value.strip():
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=message,
                remediation="Set the field to a non-empty string.",
                ref=ref,
            )
        )


def index_catalog_rows(catalog: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(catalog.get("rows"), "catalog.rows")
    index: dict[str, dict[str, Any]] = {}
    for raw in rows:
        row = ensure_dict(raw, "catalog.rows[]")
        row_id = opt_str(row.get("id"))
        if row_id:
            index[row_id] = row
    return index


def validate_beta_packet(
    repo_root: Path,
    packet_path: Path,
    catalog_path: Path,
    findings: list[Finding],
) -> None:
    packet = ensure_dict(render_yaml_as_json(packet_path), "beta_packet")
    packet_id = opt_str(packet.get("packet_id")) or "<missing packet_id>"

    if packet.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fitness_packet_beta.schema_version",
                message="beta packet schema_version must be 1",
                remediation="Update the validator in the same change that bumps the schema.",
                ref=packet_id,
            )
        )
    if opt_str(packet.get("record_kind")) != BETA_RECORD_KIND:
        findings.append(
            Finding(
                severity="error",
                check_id="fitness_packet_beta.record_kind",
                message=f"beta packet record_kind must be {BETA_RECORD_KIND}",
                remediation="Regenerate the beta packet with the correct record kind.",
                ref=packet_id,
            )
        )
    require_str(
        packet.get("owner_dri"),
        "fitness_packet_beta.owner_dri",
        packet_id,
        "beta packet owner_dri must be a non-empty string",
        findings,
    )
    require_str(
        packet.get("candidate_stage"),
        "fitness_packet_beta.candidate_stage",
        packet_id,
        "beta packet candidate_stage must be a non-empty string",
        findings,
    )
    base_packet_ref = opt_str(packet.get("base_packet_ref"))
    if base_packet_ref != EXPECTED_BASE_PACKET_REF:
        findings.append(
            Finding(
                severity="error",
                check_id="fitness_packet_beta.base_packet_ref",
                message="beta packet base_packet_ref must name the checked-in protected fitness packet",
                remediation=f"Set base_packet_ref to {EXPECTED_BASE_PACKET_REF}.",
                ref=packet_id,
            )
        )
        return

    base_path = repo_root / base_packet_ref
    base = ensure_dict(render_yaml_as_json(base_path), "base_packet")
    if opt_str(base.get("record_kind")) != BASE_RECORD_KIND:
        findings.append(
            Finding(
                severity="error",
                check_id="fitness_packet_beta.base_record_kind",
                message=f"base packet record_kind must be {BASE_RECORD_KIND}",
                remediation="Point base_packet_ref at the protected fitness review packet.",
                ref=base_packet_ref,
            )
        )
    if base.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="fitness_packet_beta.base_schema_version",
                message="base packet schema_version must be 1",
                remediation="Refresh the base packet in the same change set.",
                ref=base_packet_ref,
            )
        )

    catalog = ensure_dict(render_yaml_as_json(catalog_path), "catalog")
    catalog_index = index_catalog_rows(catalog)

    base_rows = {
        opt_str(ensure_dict(row, "base.protected_function_rows[]").get("protected_function_ref")): row
        for row in ensure_list(
            base.get("protected_function_rows"),
            "base.protected_function_rows",
        )
    }

    thresholds = ensure_list(
        packet.get("release_candidate_thresholds"),
        "beta_packet.release_candidate_thresholds",
    )
    threshold_refs = {
        opt_str(ensure_dict(t, "release_candidate_thresholds[]").get("protected_function_ref"))
        for t in thresholds
    }

    # Coverage: every catalog-linked base row needs a release-candidate bar.
    for ref, row in base_rows.items():
        if ref is None:
            continue
        if opt_str(ensure_dict(row, "base row").get("catalog_row_ref")) and ref not in threshold_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.coverage",
                    message=f"{ref} is catalog-linked but carries no release-candidate threshold",
                    remediation="Add a release_candidate_thresholds row for this protected function.",
                    ref=ref,
                )
            )

    seen: set[str] = set()
    for raw in thresholds:
        threshold = ensure_dict(raw, "release_candidate_thresholds[]")
        ref = opt_str(threshold.get("protected_function_ref")) or "<missing protected_function_ref>"
        if ref in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.duplicate",
                    message=f"duplicate release-candidate threshold for {ref}",
                    remediation="Use one threshold row per protected function.",
                    ref=ref,
                )
            )
        seen.add(ref)

        comparator = opt_str(threshold.get("comparator"))
        if comparator not in RELEASE_CANDIDATE_COMPARATORS:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.comparator",
                    message=f"{ref} comparator {comparator!r} is not in the release-candidate comparator vocabulary",
                    remediation="Use one of: " + ", ".join(sorted(RELEASE_CANDIDATE_COMPARATORS)),
                    ref=ref,
                )
            )
        require_str(
            threshold.get("release_candidate_bar"),
            "release_candidate_thresholds.release_candidate_bar",
            ref,
            f"{ref} must name a release_candidate_bar",
            findings,
        )
        require_str(
            threshold.get("measured_value"),
            "release_candidate_thresholds.measured_value",
            ref,
            f"{ref} must name a measured_value",
            findings,
        )

        base_row = base_rows.get(ref)
        if base_row is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.unknown_function",
                    message=f"{ref} does not resolve to a base protected_function_row",
                    remediation="Bind the threshold to a protected_function_ref present in the base packet.",
                    ref=ref,
                )
            )
            continue

        threshold_authority = opt_str(threshold.get("waiver_authority_ref"))
        catalog_ref = opt_str(threshold.get("catalog_row_ref"))
        if catalog_ref is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.catalog_row_ref",
                    message=f"{ref} must name a catalog_row_ref",
                    remediation="Name the catalog row whose bar this threshold mirrors.",
                    ref=ref,
                )
            )
        else:
            catalog_row = catalog_index.get(catalog_ref)
            if catalog_row is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="release_candidate_thresholds.catalog_row_ref",
                        message=f"{ref} catalog_row_ref {catalog_ref} does not resolve in the catalog",
                        remediation="Use an id from artifacts/bench/fitness_function_catalog.yaml#rows.",
                        ref=ref,
                    )
                )
            else:
                if opt_str(catalog_row.get("threshold_mode")) != opt_str(threshold.get("threshold_mode")):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="release_candidate_thresholds.threshold_mode",
                            message=f"{ref} threshold_mode does not match the catalog threshold_mode",
                            remediation="Copy the catalog row threshold_mode onto the release-candidate threshold.",
                            ref=ref,
                        )
                    )
                if opt_str(catalog_row.get("waiver_authority")) != threshold_authority:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="release_candidate_thresholds.waiver_authority_ref",
                            message=f"{ref} waiver_authority_ref does not match the catalog waiver_authority",
                            remediation="Copy the catalog row waiver_authority onto the release-candidate threshold.",
                            ref=ref,
                        )
                    )
            if opt_str(base_row.get("catalog_row_ref")) != catalog_ref:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="release_candidate_thresholds.catalog_row_ref",
                        message=f"{ref} catalog_row_ref does not match the base row catalog_row_ref",
                        remediation="Align the threshold catalog_row_ref with the base row.",
                        ref=ref,
                    )
                )

        if threshold_authority != opt_str(base_row.get("waiver_authority_ref")):
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.waiver_authority_ref",
                    message=f"{ref} waiver_authority_ref does not match the base row waiver_authority_ref",
                    remediation="Align the threshold waiver_authority_ref with the base row.",
                    ref=ref,
                )
            )

        waiver = ensure_dict(base_row.get("waiver", {}), f"{ref}.waiver")
        waiver_state = opt_str(waiver.get("waiver_state"))
        current_result = opt_str(base_row.get("current_result"))
        within_bar = bool(threshold.get("within_release_candidate_bar"))

        if not within_bar:
            if waiver_state == "active_waiver":
                if not opt_str(waiver.get("waiver_record_ref")):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="release_candidate_thresholds.active_waiver_visibility",
                            message=f"{ref} is over its bar under an active waiver but carries no visible waiver_record_ref",
                            remediation="Attach the active waiver record so the held release stays visible.",
                            ref=ref,
                        )
                    )
                if current_result == "passing":
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="release_candidate_thresholds.active_waiver_visibility",
                            message=f"{ref} renders passing while an active waiver holds it over the bar",
                            remediation="Render the waived state instead of passing.",
                            ref=ref,
                        )
                    )
            elif current_result in {"passing", "waived"}:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="release_candidate_thresholds.over_threshold_without_active_waiver",
                        message=f"{ref} is over its release-candidate bar without an active waiver but renders {current_result}",
                        remediation="Degrade the protected function or attach an active waiver.",
                        ref=ref,
                    )
                )

        if waiver_state == "expired_waiver" and current_result in {"passing", "warning", "waived"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="release_candidate_thresholds.expired_waiver_degrades",
                    message=f"{ref} has an expired waiver but still renders {current_result}",
                    remediation="Degrade the protected function once its waiver expires.",
                    ref=ref,
                )
            )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "record_kind": "protected_fitness_release_candidate_validation_capture",
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "generated_at": dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for f in findings if f.severity == "error"),
            "warnings": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet_path = repo_root / args.packet
    catalog_path = repo_root / args.catalog

    findings: list[Finding] = []
    validate_beta_packet(repo_root, packet_path, catalog_path, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    print("beta protected-fitness release-candidate packet validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
