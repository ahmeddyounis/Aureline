#!/usr/bin/env python3
"""Generate the SDK conformance packet and bridge-compatibility scorecard.

The packet aggregates the extension validator-suite report, the SDK v1
starter-pack fixture (sample-pack outcome), the lifecycle/deprecation
metadata packet, the bridge matrix, and a docs-freshness sweep into one
checked-in artifact per claimed beta SDK/runtime line. Output paths and
schema:

  - schemas/extensions/sdk_conformance_packet.schema.json
  - schemas/extensions/bridge_compatibility_scorecard.schema.json
  - artifacts/extensions/m3/sdk_conformance_packet.json (machine)
  - artifacts/extensions/m3/sdk_conformance_packet.md   (reviewer prose)
  - artifacts/extensions/m3/bridge_compatibility_scorecard.json

The generator is deterministic. Drift between a fixture's declared
expectations and the underlying source rows fails closed and is
reported as a typed non_green_reason on the packet record.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any


PACKET_SCHEMA_VERSION = 1
SCORECARD_SCHEMA_VERSION = 1
PACKET_RECORD_KIND = "sdk_conformance_packet"
SCORECARD_RECORD_KIND = "bridge_compatibility_scorecard"

PACKET_ID_PREFIX = "sdk_conformance_packet:"
SCORECARD_ID_PREFIX = "bridge_compatibility_scorecard:"

SCORECARD_CLASS_VOCABULARY = [
    "native_green",
    "bridge_amber",
    "shimmed_amber",
    "partial_amber",
    "unsupported_red",
]

BRIDGE_STATE_TO_SCORECARD = {
    "native": "native_green",
    "bridge": "bridge_amber",
    "shimmed": "shimmed_amber",
    "partial": "partial_amber",
    "unsupported": "unsupported_red",
}

BRIDGE_STATE_TO_NATIVE_CHECK = {
    "native": "native_supported",
    "bridge": "native_not_applicable",
    "shimmed": "native_not_applicable",
    "partial": "native_not_applicable",
    "unsupported": "native_unsupported",
}

BRIDGE_STATE_TO_BRIDGE_CHECK = {
    "native": "bridge_not_applicable",
    "bridge": "bridge_translated_with_caveats",
    "shimmed": "bridge_shimmed_static_only",
    "partial": "bridge_partial_subset_documented",
    "unsupported": "bridge_unsupported",
}

FRESHNESS_CHECK_CLASSES = {
    "sdk_line_semver_token",
    "lifecycle_metadata_packet_ref",
    "bridge_matrix_id",
    "bridge_matrix_path",
    "versioning_policy_ref",
    "deprecation_packet_template_ref",
    "conformance_kit_report_ref",
    "sample_pack_starter_pack_ref",
    "consuming_surface_ref",
}

DECISION_READY = "ready_for_authors"
DECISION_PARTIAL = "partially_ready_preview_surfaces_only"
DECISION_REFUSED = "refused_inconsistent_input"

REASON_READY = "all_claimed_surfaces_available_in_beta"
REASON_PARTIAL = "some_claimed_surfaces_preview_in_beta"
REASON_VALIDATOR_FAIL = "validator_suite_failed"
REASON_SAMPLE_PACK_REFUSED = "sample_pack_refused"
REASON_LIFECYCLE_FAIL = "lifecycle_metadata_packet_failed"
REASON_FRESHNESS_DRIFT = "docs_freshness_drift_detected"
REASON_BRIDGE_WIDENS = "bridge_matrix_widens_native_claim"
REASON_BRIDGE_MISSING = "bridge_matrix_missing_required_state"
REASON_LIFECYCLE_MISSING = "lifecycle_metadata_missing_required_surface"

REFUSED_REASONS = {
    REASON_VALIDATOR_FAIL,
    REASON_SAMPLE_PACK_REFUSED,
    REASON_LIFECYCLE_FAIL,
    REASON_FRESHNESS_DRIFT,
    REASON_BRIDGE_WIDENS,
    REASON_BRIDGE_MISSING,
    REASON_LIFECYCLE_MISSING,
}

SEMVER_RE = re.compile(
    r"^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$"
)
SDK_LINE_RE = re.compile(r"^aureline\.sdk\.[a-z][a-z0-9-]*[a-z0-9]$")


@dataclass
class FreshnessFinding:
    doc_ref: str
    check_class: str
    required_token: str
    status: str
    message: str
    fix: str | None = None

    def as_record(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["fix"] is None:
            payload.pop("fix")
        return payload


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def load_yaml(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    proc = subprocess.run(
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
    if proc.returncode != 0:
        stderr = proc.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"YAML→JSON parse failed for {path}: {exc}") from exc


def non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def derive_freshness_findings(
    *,
    docs_dir: Path,
    repo_root: Path,
    required_tokens: list[dict[str, Any]],
) -> list[FreshnessFinding]:
    findings: list[FreshnessFinding] = []
    for entry in required_tokens:
        doc_rel = entry.get("doc")
        check_class = entry.get("check_class")
        token = entry.get("token")
        if not non_empty_string(doc_rel):
            raise SystemExit("docs_freshness entry requires non-empty doc")
        if check_class not in FRESHNESS_CHECK_CLASSES:
            raise SystemExit(
                f"docs_freshness entry uses unknown check_class: {check_class!r}"
            )
        if not non_empty_string(token):
            raise SystemExit("docs_freshness entry requires non-empty token")
        doc_path = (repo_root / doc_rel).resolve()
        if not doc_path.exists():
            findings.append(
                FreshnessFinding(
                    doc_ref=doc_rel,
                    check_class=check_class,
                    required_token=token,
                    status="cite_missing",
                    message=f"doc {doc_rel} does not exist",
                    fix=f"Land the missing doc and cite {token}.",
                )
            )
            continue
        body = doc_path.read_text(encoding="utf-8")
        if token in body:
            findings.append(
                FreshnessFinding(
                    doc_ref=doc_rel,
                    check_class=check_class,
                    required_token=token,
                    status="cite_present",
                    message=f"doc cites {token}",
                )
            )
        else:
            findings.append(
                FreshnessFinding(
                    doc_ref=doc_rel,
                    check_class=check_class,
                    required_token=token,
                    status="drifted",
                    message=f"doc {doc_rel} does not cite required token {token}",
                    fix=f"Update {doc_rel} to cite {token}.",
                )
            )
    return findings


def summarize_validator_report(report: dict[str, Any]) -> dict[str, Any]:
    if report.get("record_kind") != "extension_conformance_suite_report":
        raise SystemExit(
            "validator report must be an extension_conformance_suite_report"
        )
    case_results = report.get("case_results") or []
    matched = sum(
        1
        for case in case_results
        if isinstance(case, dict) and case.get("matched_expectation") is True
    )
    return {
        "suite_result_class": report.get("suite_result_class"),
        "case_count": int(report.get("case_count") or 0),
        "matched_expectation_count": matched,
        "aggregate_blocker_count": int(report.get("aggregate_blocker_count") or 0),
        "required_scenario_classes": sorted(
            report.get("required_scenario_classes") or []
        ),
        "observed_scenario_classes": sorted(
            report.get("observed_scenario_classes") or []
        ),
        "unexpected_results": list(report.get("unexpected_results") or []),
    }


def summarize_sample_pack(
    fixture_payload: dict[str, Any],
    *,
    declared_expectations: dict[str, Any],
) -> dict[str, Any]:
    fixture = fixture_payload.get("__fixture__") or {}
    input_block = fixture_payload.get("input") or {}
    api_surfaces = input_block.get("claimed_api_surfaces") or []
    samples = input_block.get("sample_pack_entries") or []
    guides = input_block.get("authoring_guides") or []
    wasm_runnable_classes = {
        "wasm_component_minimal",
        "wasm_component_capability_negotiated",
    }
    external_host_runnable_classes = {
        "external_host_supervised_minimal",
        "external_host_supervised_capability_negotiated",
    }
    runnable_validation_classes = {
        "must_compile_in_ci",
        "must_validate_in_ci",
        "must_run_in_ci",
    }
    wasm_count = sum(
        1
        for sample in samples
        if isinstance(sample, dict)
        and sample.get("sample_entry_class") in wasm_runnable_classes
        and sample.get("sample_validation_class") in runnable_validation_classes
    )
    external_host_count = sum(
        1
        for sample in samples
        if isinstance(sample, dict)
        and sample.get("sample_entry_class") in external_host_runnable_classes
        and sample.get("sample_validation_class") in runnable_validation_classes
    )
    available_in_beta = sum(
        1
        for surface in api_surfaces
        if isinstance(surface, dict)
        and surface.get("availability_class") == "available_in_beta"
    )
    preview_in_beta = sum(
        1
        for surface in api_surfaces
        if isinstance(surface, dict)
        and surface.get("availability_class") == "preview_in_beta"
    )
    summary = {
        "starter_pack_id": input_block.get("starter_pack_id"),
        "decision_class": fixture.get("expected_decision_class"),
        "reason_class": fixture.get("expected_reason_class"),
        "claimed_api_surface_count": len(api_surfaces),
        "available_in_beta_surface_count": available_in_beta,
        "preview_in_beta_surface_count": preview_in_beta,
        "wasm_sample_count": wasm_count,
        "external_host_sample_count": external_host_count,
        "authoring_guide_count": len(guides),
    }
    for field, expected in declared_expectations.items():
        actual = summary.get(field)
        if expected is not None and actual != expected:
            raise SystemExit(
                f"sample-pack expectation drift on {field}: "
                f"expected {expected!r}, observed {actual!r}"
            )
    return summary


def summarize_lifecycle_packet(
    packet: dict[str, Any],
    *,
    lifecycle_report: dict[str, Any] | None,
) -> dict[str, Any]:
    decision_class = "ready_for_authors"
    reason_class = "all_rows_governed"
    if lifecycle_report is not None:
        result_class = lifecycle_report.get("result_class")
        if result_class != "pass":
            decision_class = "refused_inconsistent_input"
            reason_class = "lifecycle_metadata_packet_failed"
    return {
        "packet_id": packet.get("packet_id"),
        "policy_ref": packet.get("policy_ref"),
        "row_count": int(packet.get("row_count") or 0),
        "deprecated_row_count": int(packet.get("deprecated_row_count") or 0),
        "beta_or_stable_row_count": int(packet.get("beta_or_stable_row_count") or 0),
        "decision_class": decision_class,
        "reason_class": reason_class,
    }


def project_scorecard_lane(row: dict[str, Any]) -> dict[str, Any]:
    bridge_window = row.get("bridge_window") or {}
    bridge_state = bridge_window.get("bridge_state_class")
    if bridge_state not in BRIDGE_STATE_TO_SCORECARD:
        raise SystemExit(
            f"unknown bridge_state_class {bridge_state!r} on row {row.get('row_id')!r}"
        )
    downgrade = row.get("downgrade_behavior") or {}
    scorecard_class = BRIDGE_STATE_TO_SCORECARD[bridge_state]
    native_check = BRIDGE_STATE_TO_NATIVE_CHECK[bridge_state]
    bridge_check = BRIDGE_STATE_TO_BRIDGE_CHECK[bridge_state]
    non_green_reasons: list[str] = []
    if bridge_state == "bridge":
        non_green_reasons.append("bridge_translated_subset_not_native_parity")
    elif bridge_state == "shimmed":
        non_green_reasons.append("static_asset_shim_no_runtime_parity")
    elif bridge_state == "partial":
        non_green_reasons.append("partial_subset_documented_no_full_parity")
    elif bridge_state == "unsupported":
        non_green_reasons.append("foreign_runtime_unsupported_in_beta_lane")
    downgrade_summary = " ".join(
        line.strip()
        for line in (downgrade.get("state_preservation_note") or "").splitlines()
        if line.strip()
    ) or (downgrade.get("contract_rule") or "").strip().replace("\n", " ") or (
        "downgrade behavior follows the bridge matrix contract."
    )
    return {
        "row_id": row.get("row_id"),
        "lane_label": row.get("claimed_lane") or row.get("row_id"),
        "package_or_lane_id": row.get("package_or_lane_id"),
        "support_class": row.get("support_class"),
        "bridge_state_class": bridge_state,
        "compatibility_label": bridge_window.get("compatibility_label"),
        "parity_claim_class": bridge_window.get("parity_claim_class"),
        "scorecard_class": scorecard_class,
        "native_runtime_check_class": native_check,
        "compatibility_bridge_check_class": bridge_check,
        "supported_artifact_classes": list(
            bridge_window.get("supported_artifact_classes") or []
        ),
        "known_limits": list(bridge_window.get("known_limits") or []),
        "non_green_reasons": non_green_reasons,
        "downgrade_behavior_summary": downgrade_summary,
        "lifecycle_row_refs": list(row.get("lifecycle_row_refs") or []),
        "evidence_refs": list(row.get("evidence_refs") or []),
        "consuming_surface_refs": list(row.get("marketplace_surface_refs") or [])
        + list(row.get("sdk_doc_refs") or [])
        + list(row.get("release_packet_refs") or [])
        + list(row.get("support_export_refs") or []),
    }


def project_bridge_lane_summary(lane: dict[str, Any]) -> dict[str, Any]:
    return {
        "row_id": lane["row_id"],
        "lane_label": lane["lane_label"],
        "package_or_lane_id": lane["package_or_lane_id"],
        "bridge_state_class": lane["bridge_state_class"],
        "compatibility_label": lane["compatibility_label"],
        "parity_claim_class": lane["parity_claim_class"],
        "scorecard_class": lane["scorecard_class"],
        "support_class": lane["support_class"],
        "native_runtime_check_class": lane["native_runtime_check_class"],
        "compatibility_bridge_check_class": lane["compatibility_bridge_check_class"],
        "known_limits": lane["known_limits"],
        "non_green_reasons": lane["non_green_reasons"],
        "lifecycle_row_refs": lane["lifecycle_row_refs"],
        "consuming_surface_refs": lane["consuming_surface_refs"],
    }


def collect_non_green_reasons(
    *,
    decision_class: str,
    reason_class: str,
    freshness_findings: list[FreshnessFinding],
    validator_summary: dict[str, Any],
    sample_pack_summary: dict[str, Any],
    lifecycle_summary: dict[str, Any],
    bridge_lane_summaries: list[dict[str, Any]],
) -> list[str]:
    reasons: list[str] = []
    if validator_summary.get("suite_result_class") != "pass":
        reasons.append(REASON_VALIDATOR_FAIL)
    if sample_pack_summary.get("decision_class") not in {
        DECISION_READY,
        DECISION_PARTIAL,
    }:
        reasons.append(REASON_SAMPLE_PACK_REFUSED)
    if lifecycle_summary.get("decision_class") != "ready_for_authors":
        reasons.append(REASON_LIFECYCLE_FAIL)
    if any(finding.status == "drifted" for finding in freshness_findings):
        reasons.append(REASON_FRESHNESS_DRIFT)
    if any(finding.status == "cite_missing" for finding in freshness_findings):
        reasons.append(REASON_FRESHNESS_DRIFT)
    observed_states = {lane["bridge_state_class"] for lane in bridge_lane_summaries}
    required_states = {"native", "bridge", "shimmed", "unsupported"}
    if not required_states.issubset(observed_states):
        reasons.append(REASON_BRIDGE_MISSING)
    for lane in bridge_lane_summaries:
        if lane["bridge_state_class"] == "native" and lane["scorecard_class"] != "native_green":
            reasons.append(REASON_BRIDGE_WIDENS)
            break
    seen: list[str] = []
    for reason in reasons:
        if reason not in seen:
            seen.append(reason)
    if reason_class in REFUSED_REASONS and reason_class not in seen:
        seen.insert(0, reason_class)
    return seen


def derive_decision(
    *,
    sample_pack_summary: dict[str, Any],
    non_green_reasons: list[str],
) -> tuple[str, str]:
    if non_green_reasons:
        return DECISION_REFUSED, non_green_reasons[0]
    sample_decision = sample_pack_summary.get("decision_class")
    sample_reason = sample_pack_summary.get("reason_class")
    if sample_decision == DECISION_READY:
        return DECISION_READY, REASON_READY
    if sample_decision == DECISION_PARTIAL:
        return DECISION_PARTIAL, REASON_PARTIAL
    return DECISION_REFUSED, sample_reason or REASON_SAMPLE_PACK_REFUSED


def render_packet_markdown(packet: dict[str, Any]) -> str:
    out: list[str] = []
    out.append(f"# SDK conformance packet for `{packet['sdk_line_id']}` @ `{packet['sdk_line_semver']}`")
    out.append("")
    out.append(
        f"- **Packet id:** `{packet['packet_id']}`"
    )
    out.append(f"- **Release channel scope:** `{packet['release_channel_scope']}`")
    out.append(f"- **As of:** `{packet['as_of']}`")
    out.append(f"- **Generated at:** `{packet['generated_at']}`")
    out.append(f"- **Decision class:** `{packet['decision_class']}`")
    out.append(f"- **Reason class:** `{packet['reason_class']}`")
    out.append(
        f"- **Validator report:** [`{packet['validator_report_ref']}`](../../../{packet['validator_report_ref']})"
    )
    out.append(
        f"- **Sample-pack record:** [`{packet['sample_pack_record_ref']}`](../../../{packet['sample_pack_record_ref']})"
    )
    out.append(
        f"- **Lifecycle metadata packet:** [`{packet['lifecycle_metadata_packet_ref']}`](../../../{packet['lifecycle_metadata_packet_ref']})"
    )
    out.append(
        f"- **Bridge matrix:** [`{packet['bridge_matrix_ref']}`](../../../{packet['bridge_matrix_ref']})"
    )
    out.append(
        f"- **Bridge-compatibility scorecard:** [`{packet['bridge_compatibility_scorecard_ref']}`](../../../{packet['bridge_compatibility_scorecard_ref']})"
    )
    out.append("")
    out.append("## Validator suite summary")
    out.append("")
    vs = packet["validator_result_summary"]
    out.append(
        f"- Result: `{vs['suite_result_class']}` ({vs['matched_expectation_count']}/{vs['case_count']} cases matched expectations, "
        f"{vs['aggregate_blocker_count']} aggregate blockers)"
    )
    out.append(
        "- Required scenarios: " + ", ".join(f"`{cls}`" for cls in vs["required_scenario_classes"])
    )
    out.append(
        "- Observed scenarios: " + ", ".join(f"`{cls}`" for cls in vs["observed_scenario_classes"])
    )
    if vs["unexpected_results"]:
        out.append("- Unexpected results:")
        for line in vs["unexpected_results"]:
            out.append(f"  - {line}")
    out.append("")
    out.append("## Sample-pack summary")
    out.append("")
    sp = packet["sample_pack_summary"]
    out.append(f"- Starter pack: `{sp['starter_pack_id']}`")
    out.append(f"- Decision: `{sp['decision_class']}` / reason `{sp['reason_class']}`")
    out.append(
        f"- Surfaces: {sp['claimed_api_surface_count']} claimed, "
        f"{sp['available_in_beta_surface_count']} available, "
        f"{sp['preview_in_beta_surface_count']} preview"
    )
    out.append(
        f"- Samples: {sp['wasm_sample_count']} wasm runnable, "
        f"{sp['external_host_sample_count']} external-host runnable"
    )
    out.append(f"- Authoring guides: {sp['authoring_guide_count']}")
    out.append("")
    out.append("## Lifecycle metadata summary")
    out.append("")
    lc = packet["lifecycle_metadata_summary"]
    out.append(f"- Packet: `{lc['packet_id']}`")
    out.append(f"- Policy: [`{lc['policy_ref']}`](../../../{lc['policy_ref']})")
    out.append(
        f"- Rows: {lc['row_count']} total, "
        f"{lc['beta_or_stable_row_count']} governed beta/stable, "
        f"{lc['deprecated_row_count']} deprecated"
    )
    out.append(f"- Decision: `{lc['decision_class']}` / reason `{lc['reason_class']}`")
    out.append("")
    out.append("## Bridge-compatibility scorecard")
    out.append("")
    out.append("| Lane | State | Scorecard | Native | Bridge |")
    out.append("|---|---|---|---|---|")
    for lane in packet["bridge_lane_summaries"]:
        out.append(
            f"| `{lane['lane_label']}` | `{lane['bridge_state_class']}` "
            f"| `{lane['scorecard_class']}` | `{lane['native_runtime_check_class']}` "
            f"| `{lane['compatibility_bridge_check_class']}` |"
        )
    out.append("")
    out.append("### Lane caveats and non-green reasons")
    out.append("")
    for lane in packet["bridge_lane_summaries"]:
        if not lane["known_limits"] and not lane["non_green_reasons"]:
            continue
        out.append(f"- `{lane['row_id']}`")
        if lane["non_green_reasons"]:
            out.append(
                "  - Non-green reasons: "
                + ", ".join(f"`{reason}`" for reason in lane["non_green_reasons"])
            )
        if lane["known_limits"]:
            out.append("  - Known limits:")
            for limit in lane["known_limits"]:
                out.append(f"    - {limit}")
    out.append("")
    out.append("## Docs freshness findings")
    out.append("")
    if not packet["docs_freshness_findings"]:
        out.append("- No docs registered for freshness checks.")
    else:
        out.append("| Doc | Token | Check | Status |")
        out.append("|---|---|---|---|")
        for finding in packet["docs_freshness_findings"]:
            out.append(
                f"| `{finding['doc_ref']}` | `{finding['required_token']}` "
                f"| `{finding['check_class']}` | `{finding['status']}` |"
            )
    out.append("")
    if packet["non_green_reasons"]:
        out.append("## Non-green reasons")
        out.append("")
        for reason in packet["non_green_reasons"]:
            out.append(f"- `{reason}`")
        out.append("")
    if packet["known_caveats"]:
        out.append("## Known caveats")
        out.append("")
        for caveat in packet["known_caveats"]:
            out.append(f"- {caveat}")
        out.append("")
    if packet["consuming_surfaces"]:
        out.append("## Consuming surfaces")
        out.append("")
        for surface in packet["consuming_surfaces"]:
            out.append(f"- `{surface}`")
        out.append("")
    return "\n".join(out)


def build_packet(
    fixture: dict[str, Any],
    *,
    repo_root: Path,
) -> tuple[dict[str, Any], dict[str, Any], str]:
    sdk_line_id = fixture.get("sdk_line_id")
    sdk_line_semver = fixture.get("sdk_line_semver")
    if not (isinstance(sdk_line_id, str) and SDK_LINE_RE.match(sdk_line_id)):
        raise SystemExit(f"sdk_line_id must match {SDK_LINE_RE.pattern}")
    if not (isinstance(sdk_line_semver, str) and SEMVER_RE.match(sdk_line_semver)):
        raise SystemExit(f"sdk_line_semver must follow semver")
    packet_id = fixture.get("packet_id")
    if not (isinstance(packet_id, str) and packet_id.startswith(PACKET_ID_PREFIX)):
        raise SystemExit(f"packet_id must be prefixed {PACKET_ID_PREFIX}")
    scorecard_id = fixture.get("scorecard_id")
    if not (isinstance(scorecard_id, str) and scorecard_id.startswith(SCORECARD_ID_PREFIX)):
        raise SystemExit(f"scorecard_id must be prefixed {SCORECARD_ID_PREFIX}")
    release_channel_scope = fixture.get("release_channel_scope", "beta")
    if release_channel_scope not in {"beta", "stable"}:
        raise SystemExit("release_channel_scope must be beta or stable")
    as_of = fixture.get("as_of")
    generated_at = fixture.get("generated_at")
    if not non_empty_string(as_of) or not non_empty_string(generated_at):
        raise SystemExit("as_of and generated_at are required")
    inputs = fixture.get("inputs") or {}
    validator_path = repo_root / inputs.get("validator_report_ref", "")
    sample_pack_path = repo_root / inputs.get("sample_pack_record_ref", "")
    lifecycle_path = repo_root / inputs.get("lifecycle_metadata_packet_ref", "")
    bridge_path = repo_root / inputs.get("bridge_matrix_ref", "")
    lifecycle_report_rel = inputs.get("lifecycle_metadata_report_ref")
    lifecycle_report_path = (
        repo_root / lifecycle_report_rel if lifecycle_report_rel else None
    )

    validator_report = load_json(validator_path)
    validator_summary = summarize_validator_report(validator_report)

    sample_pack_payload = load_json(sample_pack_path)
    sample_pack_summary = summarize_sample_pack(
        sample_pack_payload,
        declared_expectations=fixture.get("sample_pack_expectations") or {},
    )

    lifecycle_packet = load_json(lifecycle_path)
    lifecycle_report = (
        load_json(lifecycle_report_path) if lifecycle_report_path else None
    )
    lifecycle_summary = summarize_lifecycle_packet(
        lifecycle_packet, lifecycle_report=lifecycle_report
    )

    bridge_matrix = load_yaml(bridge_path)
    rows = bridge_matrix.get("rows") or []
    if not rows:
        raise SystemExit(f"bridge matrix {bridge_path} has no rows")
    scorecard_lanes = [project_scorecard_lane(row) for row in rows]
    bridge_lane_summaries = [project_bridge_lane_summary(lane) for lane in scorecard_lanes]

    freshness_findings = derive_freshness_findings(
        docs_dir=repo_root / "docs",
        repo_root=repo_root,
        required_tokens=fixture.get("docs_freshness") or [],
    )

    non_green_reasons = collect_non_green_reasons(
        decision_class=DECISION_READY,
        reason_class=REASON_READY,
        freshness_findings=freshness_findings,
        validator_summary=validator_summary,
        sample_pack_summary=sample_pack_summary,
        lifecycle_summary=lifecycle_summary,
        bridge_lane_summaries=bridge_lane_summaries,
    )

    declared_decision = fixture.get("declared_decision_class")
    declared_reason = fixture.get("declared_reason_class")
    decision_class, reason_class = derive_decision(
        sample_pack_summary=sample_pack_summary,
        non_green_reasons=non_green_reasons,
    )
    if declared_decision is not None and declared_decision != decision_class:
        raise SystemExit(
            f"declared decision_class {declared_decision!r} does not match derived {decision_class!r}"
        )
    if declared_reason is not None and declared_reason != reason_class:
        raise SystemExit(
            f"declared reason_class {declared_reason!r} does not match derived {reason_class!r}"
        )

    scorecard = {
        "record_kind": SCORECARD_RECORD_KIND,
        "bridge_compatibility_scorecard_schema_version": SCORECARD_SCHEMA_VERSION,
        "scorecard_id": scorecard_id,
        "packet_ref": fixture.get("packet_artifact_ref"),
        "matrix_ref": inputs.get("bridge_matrix_ref"),
        "matrix_revision": int(bridge_matrix.get("matrix_revision") or 1),
        "release_channel_scope": release_channel_scope,
        "as_of": as_of,
        "generated_at": generated_at,
        "owner": fixture.get("owner") or bridge_matrix.get("owner") or "@ahmeddyounis",
        "scorecard_class_vocabulary": list(SCORECARD_CLASS_VOCABULARY),
        "lanes": scorecard_lanes,
        "redaction_class": "metadata_safe_default",
    }

    packet = {
        "record_kind": PACKET_RECORD_KIND,
        "sdk_conformance_packet_schema_version": PACKET_SCHEMA_VERSION,
        "packet_id": packet_id,
        "sdk_line_id": sdk_line_id,
        "sdk_line_semver": sdk_line_semver,
        "release_channel_scope": release_channel_scope,
        "as_of": as_of,
        "generated_at": generated_at,
        "owner": fixture.get("owner") or "@ahmeddyounis",
        "decision_class": decision_class,
        "reason_class": reason_class,
        "validator_report_ref": inputs.get("validator_report_ref"),
        "validator_result_summary": validator_summary,
        "sample_pack_record_ref": inputs.get("sample_pack_record_ref"),
        "sample_pack_summary": sample_pack_summary,
        "lifecycle_metadata_packet_ref": inputs.get("lifecycle_metadata_packet_ref"),
        "lifecycle_metadata_summary": lifecycle_summary,
        "bridge_matrix_ref": inputs.get("bridge_matrix_ref"),
        "bridge_compatibility_scorecard_ref": fixture.get("scorecard_artifact_ref"),
        "docs_freshness_findings": [finding.as_record() for finding in freshness_findings],
        "bridge_lane_summaries": bridge_lane_summaries,
        "non_green_reasons": non_green_reasons,
        "known_caveats": list(fixture.get("known_caveats") or []),
        "consuming_surfaces": list(fixture.get("consuming_surfaces") or []),
        "redaction_class": "metadata_safe_default",
    }

    markdown = render_packet_markdown(packet)
    return packet, scorecard, markdown


def write_or_check(
    path: Path,
    *,
    rendered: str,
    check: bool,
    label: str,
) -> bool:
    if check:
        if not path.exists():
            print(f"[sdk-conformance-packet] missing {label}: {path}", file=sys.stderr)
            return False
        existing = path.read_text(encoding="utf-8")
        if existing != rendered:
            print(
                f"[sdk-conformance-packet] {label} drift detected: {path}",
                file=sys.stderr,
            )
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(rendered, encoding="utf-8")
    return True


def command_generate(args: argparse.Namespace) -> int:
    repo_root = Path(args.repo_root).resolve()
    fixture_path = Path(args.fixture).resolve()
    fixture = load_json(fixture_path)
    packet, scorecard, markdown = build_packet(fixture, repo_root=repo_root)
    rendered_packet = render_json(packet)
    rendered_scorecard = render_json(scorecard)
    rendered_markdown = markdown if markdown.endswith("\n") else markdown + "\n"

    ok = True
    if args.packet_json:
        ok &= write_or_check(
            Path(args.packet_json).resolve(),
            rendered=rendered_packet,
            check=args.check,
            label="packet JSON",
        )
    elif not args.check:
        sys.stdout.write(rendered_packet)
    if args.packet_md:
        ok &= write_or_check(
            Path(args.packet_md).resolve(),
            rendered=rendered_markdown,
            check=args.check,
            label="packet Markdown",
        )
    if args.scorecard_json:
        ok &= write_or_check(
            Path(args.scorecard_json).resolve(),
            rendered=rendered_scorecard,
            check=args.check,
            label="scorecard JSON",
        )

    if packet["decision_class"] == DECISION_REFUSED:
        print(
            "[sdk-conformance-packet] decision_class=refused_inconsistent_input "
            f"reason_class={packet['reason_class']}",
            file=sys.stderr,
        )
        for reason in packet["non_green_reasons"]:
            print(f"  - non_green_reason: {reason}", file=sys.stderr)

    return 0 if ok else 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    sub = parser.add_subparsers(dest="command", required=True)

    gen = sub.add_parser("generate", help="generate one SDK conformance packet")
    gen.add_argument("--fixture", required=True, help="packet input fixture (JSON)")
    gen.add_argument("--packet-json", help="output path for packet JSON")
    gen.add_argument("--packet-md", help="output path for packet Markdown")
    gen.add_argument("--scorecard-json", help="output path for bridge scorecard JSON")
    gen.add_argument(
        "--check",
        action="store_true",
        help="fail when an existing output does not match the generated bytes",
    )
    gen.set_defaults(func=command_generate)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
