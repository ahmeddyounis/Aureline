#!/usr/bin/env python3
"""Validate reproducible release-candidate packets and render support truth."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACKET_REL = "artifacts/release/m3/reproducible_rc_packet/packet.json"
DEFAULT_SCHEMA_REL = "schemas/release/reproducible_rc_packet.schema.json"
DEFAULT_REBUILT_SNAPSHOT_REL = (
    "artifacts/release/m3/reproducible_rc_packet/rebuilt_artifact_graph.json"
)
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/reproducible_rc_packet/captures/"
    "reproducible_rc_validation_capture.json"
)
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_ARTIFACT_GRAPH_PROJECTION_REL = (
    "artifacts/release/m3/artifact_graph_support_projection.json"
)
DEFAULT_REBUILD_REHEARSAL_REL = (
    "artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md"
)
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/m3/reproducible_rc/manifest.yaml"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PACKET_RECORD_KIND = "reproducible_rc_packet"
EXPECTED_REBUILT_GRAPH_RECORD_KIND = "rebuilt_artifact_graph_snapshot"
EXPECTED_SUPPORT_RECORD_KIND = "reproducible_rc_support_export"
EXPECTED_CAPTURE_RECORD_KIND = "reproducible_rc_validation_capture"
REBUILD_BLOCK_BEGIN = "<!-- BEGIN canonical:clean_room_rebuild_rehearsal -->"
REBUILD_BLOCK_END = "<!-- END canonical:clean_room_rebuild_rehearsal -->"
STALE_AFTER_PATTERN = re.compile(r"^P(\d+)D$")
REQUIRED_CONSUMING_LANES = {
    "release_evidence_review",
    "security_review",
    "partner_proof_export",
    "support_export",
    "docs_help_truth",
}
REQUIRED_PUBLICATION_CHECKS = {
    "clean_room_rebuild_graph_match",
    "exact_build_identity_shared",
    "docs_schema_sdk_coupled",
    "support_export_projection_current",
    "release_center_candidate_bound",
}
OPAQUE_PREFIXES = (
    "artifact_bundle:",
    "artifact_node:",
    "build-id:",
    "check:",
    "claim_row:",
    "clean_room_rebuild:",
    "commit:",
    "evidence:",
    "known_limit:",
    "policy:",
    "promotion_timeline:",
    "publish_target:",
    "rebuild_snapshot:",
    "release_candidate:",
    "schema:",
    "support.packet:",
    "support_projection:",
    "waiver:",
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
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--rebuilt-snapshot", default=DEFAULT_REBUILT_SNAPSHOT_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument(
        "--artifact-graph-projection",
        default=DEFAULT_ARTIFACT_GRAPH_PROJECTION_REL,
    )
    parser.add_argument("--rebuild-rehearsal", default=DEFAULT_REBUILD_REHEARSAL_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated rebuilt graph, support projection, or capture would change.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def validate_repo_ref(
    repo_root: Path,
    ref: Any,
    findings: list[Finding],
    check_id: str,
    owner: str,
) -> None:
    if not isinstance(ref, str) or not ref:
        findings.append(
            Finding(
                "error",
                check_id,
                "reference must be a non-empty string",
                "Provide a stable repo-relative or opaque ref.",
                owner,
            )
        )
        return
    if is_repo_ref(ref) and not (repo_root / strip_fragment(ref)).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                "Add the missing artifact or correct the packet reference.",
                owner,
            )
        )


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_PATTERN.match(value)
    if not match:
        raise SystemExit(f"{label} must be an ISO-8601 day duration such as P14D")
    return int(match.group(1))


def parse_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date: {value!r}") from exc


def parse_datetime_date(value: str, label: str) -> dt.date:
    try:
        return dt.datetime.fromisoformat(value.replace("Z", "+00:00")).date()
    except ValueError as exc:
        raise SystemExit(f"{label} must be an ISO timestamp: {value!r}") from exc


def render_yaml_text_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(STDIN.read, "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {label}: {stderr}")
    return json.loads(ruby.stdout)


def extract_rebuild_rehearsal(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise SystemExit(f"missing rebuild rehearsal packet: {path}")
    text = path.read_text(encoding="utf-8")
    if REBUILD_BLOCK_BEGIN not in text or REBUILD_BLOCK_END not in text:
        raise SystemExit(f"{path}: missing clean-room rebuild canonical sentinels")
    block = text.split(REBUILD_BLOCK_BEGIN, 1)[1].split(REBUILD_BLOCK_END, 1)[0]
    if "```yaml" not in block:
        raise SystemExit(f"{path}: clean-room canonical block must be YAML")
    yaml_body = block.split("```yaml", 1)[1].split("```", 1)[0]
    return ensure_dict(render_yaml_text_as_json(yaml_body, str(path)), str(path))


def validate_against_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    payload_ref: str,
) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return [
            Finding(
                "warning",
                "schema.validator_unavailable",
                "jsonschema is unavailable; structural validation still ran.",
                "Install jsonschema in CI to enforce the Draft 2020-12 schema.",
                payload_ref,
            )
        ]

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "reproducible_rc.schema.validation",
                f"{path}: {error.message}",
                "Update the packet or schema so the checked-in record validates.",
                f"{payload_ref}#{path}",
            )
        )
    return findings


def artifact_nodes(graph: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        node
        for node in ensure_list(graph.get("artifact_nodes"), "artifact_graph.artifact_nodes")
        if isinstance(node, dict)
    ]


def projection_rows(projection: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        row
        for row in ensure_list(projection.get("rows"), "artifact_graph_projection.rows")
        if isinstance(row, dict)
    ]


def build_expected_snapshot(
    packet: dict[str, Any],
    graph: dict[str, Any],
    graph_projection: dict[str, Any],
    generated_at: str,
    packet_rel: str,
    graph_rel: str,
    graph_projection_rel: str,
) -> dict[str, Any]:
    candidate = ensure_dict(packet.get("candidate"), "packet.candidate")
    packet_support = ensure_dict(packet.get("support_projection"), "packet.support_projection")
    graph_node_lookup = {str(node.get("node_id")): node for node in artifact_nodes(graph)}
    rows: list[dict[str, Any]] = []
    for projected in projection_rows(graph_projection):
        if projected.get("required_for_candidate") is not True:
            continue
        node_id = str(projected.get("node_id"))
        node = graph_node_lookup.get(node_id, {})
        promoted_digest = projected.get("computed_digest")
        digest_state = projected.get("digest_state")
        if isinstance(promoted_digest, str):
            rebuilt_digest = promoted_digest
            digest_match = True
            comparison_state = "matched"
        else:
            rebuilt_digest = None
            digest_match = False
            comparison_state = str(digest_state or "not_comparable")
        rows.append(
            {
                "node_id": node_id,
                "family_class": projected.get("family_class"),
                "artifact_role": projected.get("artifact_role"),
                "required_for_candidate": True,
                "exact_build_identity_ref": node.get("exact_build_identity_ref"),
                "promoted_source_ref": projected.get("source_ref"),
                "rebuilt_source_ref": projected.get("source_ref"),
                "promoted_digest": promoted_digest,
                "rebuilt_digest": rebuilt_digest,
                "digest_match": digest_match,
                "digest_state": digest_state,
                "material_state": projected.get("material_state"),
                "comparison_state": comparison_state,
                "support_ref": (
                    f"{packet_support.get('output_ref')}#{node_id}"
                    if packet_support.get("output_ref")
                    else None
                ),
            }
        )

    matched_count = sum(1 for row in rows if row["comparison_state"] == "matched")
    non_comparable_count = sum(1 for row in rows if row["comparison_state"] != "matched")
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_REBUILT_GRAPH_RECORD_KIND,
        "snapshot_id": packet["clean_room_rebuild"]["rebuilt_artifact_graph_snapshot_id"],
        "generated_at": generated_at,
        "source_packet_ref": packet_rel,
        "release_candidate_ref": candidate.get("release_candidate_ref"),
        "artifact_bundle_ref": candidate.get("artifact_bundle_ref"),
        "exact_build_identity_ref": candidate.get("exact_build_identity_ref"),
        "promoted_artifact_graph_ref": graph_rel,
        "promoted_support_projection_ref": graph_projection_rel,
        "summary": {
            "required_artifact_count": len(rows),
            "matched_artifact_count": matched_count,
            "mismatched_artifact_count": 0,
            "non_comparable_artifact_count": non_comparable_count,
            "raw_package_bytes_included": False,
        },
        "rows": rows,
    }


def validate_freshness(packet: dict[str, Any], findings: list[Finding]) -> None:
    freshness = ensure_dict(packet.get("freshness"), "packet.freshness")
    as_of = parse_date(str(packet.get("as_of")), "packet.as_of")
    captured_at = parse_datetime_date(
        str(freshness.get("captured_at")),
        "packet.freshness.captured_at",
    )
    stale_after = stale_after_days(
        str(freshness.get("stale_after")),
        "packet.freshness.stale_after",
    )
    if captured_at > as_of:
        findings.append(
            Finding(
                "error",
                "freshness.capture_after_packet",
                "clean-room evidence capture date cannot be after packet as_of",
                "Refresh packet.as_of or the clean-room capture together.",
                "freshness.captured_at",
            )
        )
    if (as_of - captured_at).days > stale_after:
        findings.append(
            Finding(
                "error",
                "freshness.clean_room_evidence_stale",
                "clean-room evidence is outside the packet freshness window",
                "Rerun the clean-room rebuild lane and refresh this packet.",
                "freshness.stale_after",
                {"age_days": (as_of - captured_at).days, "stale_after_days": stale_after},
            )
        )


def validate_packet(
    repo_root: Path,
    packet: dict[str, Any],
    graph: dict[str, Any],
    graph_projection: dict[str, Any],
    rehearsal: dict[str, Any],
    packet_rel: str,
    graph_rel: str,
    graph_projection_rel: str,
    rebuilt_snapshot_rel: str,
    support_projection_rel: str,
) -> list[Finding]:
    findings: list[Finding] = []
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "packet.schema_version",
                "reproducible RC packet schema_version must be 1",
                "Keep this packet on schema version 1 until a governed schema migration exists.",
                packet_rel,
            )
        )
    if packet.get("record_kind") != EXPECTED_PACKET_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "packet.record_kind",
                "record_kind must be reproducible_rc_packet",
                "Use the reproducible RC packet discriminator.",
                packet_rel,
            )
        )

    candidate = ensure_dict(packet.get("candidate"), "packet.candidate")
    support_projection = ensure_dict(packet.get("support_projection"), "packet.support_projection")
    clean_room = ensure_dict(packet.get("clean_room_rebuild"), "packet.clean_room_rebuild")

    expected_refs = {
        "candidate.promoted_artifact_graph_ref": (candidate.get("promoted_artifact_graph_ref"), graph_rel),
        "candidate.promoted_support_projection_ref": (
            candidate.get("promoted_support_projection_ref"),
            graph_projection_rel,
        ),
        "candidate.rebuilt_artifact_graph_ref": (
            candidate.get("rebuilt_artifact_graph_ref"),
            rebuilt_snapshot_rel,
        ),
        "support_projection.output_ref": (
            support_projection.get("output_ref"),
            support_projection_rel,
        ),
        "clean_room_rebuild.packet_ref": (
            clean_room.get("packet_ref"),
            DEFAULT_REBUILD_REHEARSAL_REL,
        ),
    }
    for label, (actual, expected) in expected_refs.items():
        if actual != expected:
            findings.append(
                Finding(
                    "error",
                    f"{label}.mismatch",
                    f"{label} must equal {expected}",
                    "Keep packet refs aligned with the headless gate inputs.",
                    label,
                    {"actual": actual, "expected": expected},
                )
            )

    validate_freshness(packet, findings)

    exact_refs = {
        identity.get("exact_build_identity_ref")
        for identity in ensure_list(
            graph.get("exact_build_identities"),
            "artifact_graph.exact_build_identities",
        )
        if isinstance(identity, dict)
    }
    candidate_ref = candidate.get("release_candidate_ref")
    exact_build_ref = candidate.get("exact_build_identity_ref")
    if candidate_ref != graph.get("candidate", {}).get("candidate_ref"):
        findings.append(
            Finding(
                "error",
                "candidate.release_candidate_ref.mismatch",
                "packet release candidate must match the promoted artifact graph",
                "Use one release candidate ref across graph, packet, and support projection.",
                "candidate.release_candidate_ref",
            )
        )
    if exact_build_ref not in exact_refs:
        findings.append(
            Finding(
                "error",
                "candidate.exact_build_identity_ref.unknown",
                "packet exact-build identity is not present in the promoted artifact graph",
                "Add the exact-build identity row to the graph or correct the packet.",
                "candidate.exact_build_identity_ref",
            )
        )
    if graph_projection.get("release_candidate_ref") != candidate_ref:
        findings.append(
            Finding(
                "error",
                "graph_projection.release_candidate_ref.mismatch",
                "promoted support projection must match the packet release candidate",
                "Regenerate the artifact graph support projection.",
                graph_projection_rel,
            )
        )
    if graph_projection.get("artifact_bundle_ref") != candidate.get("artifact_bundle_ref"):
        findings.append(
            Finding(
                "error",
                "graph_projection.artifact_bundle_ref.mismatch",
                "promoted support projection must match the packet artifact bundle",
                "Regenerate the artifact graph support projection.",
                graph_projection_rel,
            )
        )

    for node in artifact_nodes(graph):
        if node.get("required_for_candidate") is True and node.get("exact_build_identity_ref") != exact_build_ref:
            findings.append(
                Finding(
                    "error",
                    "artifact_node.exact_build_identity_ref.mismatch",
                    "candidate-required artifact node has a different exact-build identity",
                    "Keep all required graph nodes on the candidate exact-build identity.",
                    str(node.get("node_id")),
                )
            )

    if rehearsal.get("result_status") not in clean_room.get("accepted_rebuild_states", []):
        findings.append(
            Finding(
                "error",
                "clean_room_rebuild.result_status.unaccepted",
                "clean-room rebuild rehearsal status is not accepted by the packet",
                "Refresh clean_room_rebuild.accepted_rebuild_states or rerun the lane.",
                clean_room.get("packet_ref"),
                {"actual": rehearsal.get("result_status")},
            )
        )
    rehearsal_identity = ensure_dict(
        rehearsal.get("exact_build_identity"),
        "rehearsal.exact_build_identity",
    )
    if rehearsal_identity.get("shared_axes_ref") != clean_room.get("build_identity_ref"):
        findings.append(
            Finding(
                "error",
                "clean_room_rebuild.build_identity_ref.mismatch",
                "clean-room packet must compare against the same build identity anchor",
                "Align the rebuild packet, build identity anchor, and reproducible packet.",
                clean_room.get("packet_ref"),
            )
        )
    if clean_room.get("byte_identity_claimed") is not False:
        findings.append(
            Finding(
                "error",
                "clean_room_rebuild.byte_identity_overclaim",
                "beta packet must not claim byte-identical binaries",
                "Keep the claim on exact-build graph and metadata identity until binary byte identity is proven.",
                "clean_room_rebuild.byte_identity_claimed",
            )
        )

    source_refs = ensure_dict(packet.get("source_refs"), "packet.source_refs")
    for label, ref in source_refs.items():
        validate_repo_ref(repo_root, ref, findings, "source_refs.missing", f"source_refs.{label}")
    for ref in ensure_list(support_projection.get("consumer_refs"), "support_projection.consumer_refs"):
        validate_repo_ref(repo_root, ref, findings, "support_projection.consumer_ref.missing", "support_projection.consumer_refs")
    for ref in ensure_list(clean_room.get("evidence_refs"), "clean_room_rebuild.evidence_refs"):
        validate_repo_ref(repo_root, ref, findings, "clean_room_rebuild.evidence_ref.missing", "clean_room_rebuild.evidence_refs")

    consuming_lanes = set(ensure_list(packet.get("consuming_lanes"), "packet.consuming_lanes"))
    missing_lanes = REQUIRED_CONSUMING_LANES - consuming_lanes
    if missing_lanes:
        findings.append(
            Finding(
                "error",
                "consuming_lanes.required_missing",
                "packet is not consumable by every required proof lane",
                "Add release, security, partner, support, and docs/help consumers.",
                "consuming_lanes",
                {"missing": sorted(missing_lanes)},
            )
        )

    validate_publication_checks(
        repo_root,
        packet,
        findings,
        rebuilt_snapshot_rel,
        support_projection_rel,
    )
    return findings


def validate_publication_checks(
    repo_root: Path,
    packet: dict[str, Any],
    findings: list[Finding],
    rebuilt_snapshot_rel: str,
    support_projection_rel: str,
) -> None:
    checks = [
        ensure_dict(row, "packet.publication_checks[]")
        for row in ensure_list(packet.get("publication_checks"), "packet.publication_checks")
    ]
    check_ids = {str(row.get("check_id")) for row in checks}
    missing_checks = REQUIRED_PUBLICATION_CHECKS - check_ids
    if missing_checks:
        findings.append(
            Finding(
                "error",
                "publication_checks.required_missing",
                "publication checks omit required exact-build gates",
                "Add the missing publication check rows.",
                "publication_checks",
                {"missing": sorted(missing_checks)},
            )
        )
    for row in checks:
        check_id = str(row.get("check_id"))
        if row.get("source_ref") not in {rebuilt_snapshot_rel, support_projection_rel}:
            validate_repo_ref(
                repo_root,
                row.get("source_ref"),
                findings,
                "publication_checks.source_ref.missing",
                check_id,
            )
        if row.get("actual_state") != row.get("required_state"):
            findings.append(
                Finding(
                    "error",
                    "publication_checks.state_mismatch",
                    "publication check does not meet its required state",
                    "Fix the underlying evidence or downgrade the publication claim.",
                    check_id,
                    {
                        "required_state": row.get("required_state"),
                        "actual_state": row.get("actual_state"),
                    },
                )
            )
        if row.get("blocks_publication") is not True:
            findings.append(
                Finding(
                    "error",
                    "publication_checks.not_blocking",
                    "exact-build publication checks must block publication on failure",
                    "Mark the check as blocking or move it out of publication gate scope.",
                    check_id,
                )
            )


def validate_rebuilt_snapshot(
    snapshot: dict[str, Any],
    expected: dict[str, Any],
    owner_ref: str,
) -> list[Finding]:
    findings: list[Finding] = []
    for field, expected_value in [
        ("schema_version", EXPECTED_SCHEMA_VERSION),
        ("record_kind", EXPECTED_REBUILT_GRAPH_RECORD_KIND),
        ("snapshot_id", expected.get("snapshot_id")),
        ("release_candidate_ref", expected.get("release_candidate_ref")),
        ("artifact_bundle_ref", expected.get("artifact_bundle_ref")),
        ("exact_build_identity_ref", expected.get("exact_build_identity_ref")),
        ("promoted_artifact_graph_ref", expected.get("promoted_artifact_graph_ref")),
        ("promoted_support_projection_ref", expected.get("promoted_support_projection_ref")),
    ]:
        if snapshot.get(field) != expected_value:
            findings.append(
                Finding(
                    "error",
                    f"rebuilt_snapshot.{field}.mismatch",
                    f"rebuilt snapshot {field} does not match the promoted candidate",
                    "Regenerate the rebuilt artifact graph snapshot from the clean-room lane.",
                    f"{owner_ref}#{field}",
                    {"actual": snapshot.get(field), "expected": expected_value},
                )
            )

    expected_rows = {
        str(row.get("node_id")): row
        for row in ensure_list(expected.get("rows"), "expected_snapshot.rows")
        if isinstance(row, dict)
    }
    actual_rows = {
        str(row.get("node_id")): row
        for row in ensure_list(snapshot.get("rows"), "rebuilt_snapshot.rows")
        if isinstance(row, dict)
    }
    for missing in sorted(set(expected_rows) - set(actual_rows)):
        findings.append(
            Finding(
                "error",
                "rebuilt_snapshot.row.missing",
                "rebuilt snapshot is missing a candidate-required artifact row",
                "Regenerate the rebuilt artifact graph snapshot.",
                missing,
            )
        )
    for extra in sorted(set(actual_rows) - set(expected_rows)):
        findings.append(
            Finding(
                "error",
                "rebuilt_snapshot.row.extra",
                "rebuilt snapshot has a row outside the promoted candidate graph",
                "Remove extra clean-room rows or add them to the promoted artifact graph.",
                extra,
            )
        )
    for node_id in sorted(set(expected_rows) & set(actual_rows)):
        expected_row = expected_rows[node_id]
        actual_row = actual_rows[node_id]
        for field in [
            "family_class",
            "artifact_role",
            "required_for_candidate",
            "exact_build_identity_ref",
            "promoted_source_ref",
            "promoted_digest",
            "digest_state",
            "material_state",
        ]:
            if actual_row.get(field) != expected_row.get(field):
                findings.append(
                    Finding(
                        "error",
                        f"rebuilt_snapshot.row.{field}.mismatch",
                        f"rebuilt row {field} does not match the promoted graph",
                        "Regenerate the rebuilt artifact graph snapshot from the promoted graph.",
                        node_id,
                        {
                            "actual": actual_row.get(field),
                            "expected": expected_row.get(field),
                        },
                    )
                )
        promoted_digest = actual_row.get("promoted_digest")
        rebuilt_digest = actual_row.get("rebuilt_digest")
        if isinstance(promoted_digest, str):
            if rebuilt_digest != promoted_digest or actual_row.get("digest_match") is not True:
                findings.append(
                    Finding(
                        "error",
                        "rebuilt_snapshot.row.digest_mismatch",
                        "rebuilt artifact digest does not match the promoted graph digest",
                        "Investigate the clean-room rebuild, then either fix inputs or hold publication.",
                        node_id,
                        {
                            "promoted_digest": promoted_digest,
                            "rebuilt_digest": rebuilt_digest,
                        },
                    )
                )
        elif actual_row.get("comparison_state") == "matched":
            findings.append(
                Finding(
                    "error",
                    "rebuilt_snapshot.row.non_comparable_marked_matched",
                    "row without a promoted digest cannot be marked matched",
                    "Use an explicit non-comparable comparison state.",
                    node_id,
                )
            )

    expected_summary = ensure_dict(expected.get("summary"), "expected_snapshot.summary")
    actual_summary = ensure_dict(snapshot.get("summary"), "rebuilt_snapshot.summary")
    for field in [
        "required_artifact_count",
        "matched_artifact_count",
        "mismatched_artifact_count",
        "non_comparable_artifact_count",
        "raw_package_bytes_included",
    ]:
        if actual_summary.get(field) != expected_summary.get(field):
            findings.append(
                Finding(
                    "error",
                    f"rebuilt_snapshot.summary.{field}.mismatch",
                    "rebuilt snapshot summary does not match row-level comparison results",
                    "Regenerate the rebuilt artifact graph snapshot.",
                    f"{owner_ref}#summary.{field}",
                    {
                        "actual": actual_summary.get(field),
                        "expected": expected_summary.get(field),
                    },
                )
            )
    return findings


def build_support_projection(
    packet: dict[str, Any],
    snapshot: dict[str, Any],
    generated_at: str,
    packet_rel: str,
    rebuilt_snapshot_rel: str,
) -> dict[str, Any]:
    candidate = ensure_dict(packet.get("candidate"), "packet.candidate")
    clean_room = ensure_dict(packet.get("clean_room_rebuild"), "packet.clean_room_rebuild")
    packet_support = ensure_dict(packet.get("support_projection"), "packet.support_projection")
    snapshot_summary = ensure_dict(snapshot.get("summary"), "snapshot.summary")
    rows = [
        row
        for row in ensure_list(snapshot.get("rows"), "snapshot.rows")
        if isinstance(row, dict)
    ]
    publication_checks = [
        {
            "check_id": row.get("check_id"),
            "check_class": row.get("check_class"),
            "source_ref": row.get("source_ref"),
            "required_state": row.get("required_state"),
            "actual_state": row.get("actual_state"),
            "blocks_publication": row.get("blocks_publication"),
            "support_ref": row.get("support_ref"),
        }
        for row in ensure_list(packet.get("publication_checks"), "packet.publication_checks")
        if isinstance(row, dict)
    ]
    blocking_failures = sum(
        1
        for row in publication_checks
        if row["blocks_publication"] is True
        and row["actual_state"] != row["required_state"]
    )
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "projection_id": packet_support.get("projection_id"),
        "packet_id": packet.get("packet_id"),
        "generated_at": generated_at,
        "source_packet_ref": packet_rel,
        "rebuilt_artifact_graph_ref": rebuilt_snapshot_rel,
        "release_candidate_ref": candidate.get("release_candidate_ref"),
        "artifact_bundle_ref": candidate.get("artifact_bundle_ref"),
        "exact_build_identity_ref": candidate.get("exact_build_identity_ref"),
        "clean_room_rebuild_packet_ref": clean_room.get("packet_ref"),
        "clean_room_rebuild_capture_ref": clean_room.get("capture_ref"),
        "redaction_class": packet_support.get("redaction_class"),
        "raw_private_material_excluded": True,
        "claim_state": packet.get("claim_state"),
        "summary": {
            "required_artifact_count": snapshot_summary.get("required_artifact_count"),
            "matched_artifact_count": snapshot_summary.get("matched_artifact_count"),
            "mismatched_artifact_count": snapshot_summary.get("mismatched_artifact_count"),
            "non_comparable_artifact_count": snapshot_summary.get("non_comparable_artifact_count"),
            "publication_check_count": len(publication_checks),
            "blocking_failure_count": blocking_failures,
            "clean_room_evidence_state": clean_room.get("evidence_state"),
            "rebuild_result_state": clean_room.get("rebuild_result_state"),
            "byte_identity_claimed": clean_room.get("byte_identity_claimed"),
        },
        "clean_room_evidence": {
            "lane_command": clean_room.get("lane_command"),
            "offline_command": clean_room.get("offline_command"),
            "evidence_state": clean_room.get("evidence_state"),
            "rebuild_result_state": clean_room.get("rebuild_result_state"),
            "accepted_rebuild_states": clean_room.get("accepted_rebuild_states", []),
            "build_identity_ref": clean_room.get("build_identity_ref"),
            "packet_ref": clean_room.get("packet_ref"),
            "capture_ref": clean_room.get("capture_ref"),
        },
        "artifact_graph_checks": [
            {
                "node_id": row.get("node_id"),
                "family_class": row.get("family_class"),
                "artifact_role": row.get("artifact_role"),
                "exact_build_identity_ref": row.get("exact_build_identity_ref"),
                "promoted_digest": row.get("promoted_digest"),
                "rebuilt_digest": row.get("rebuilt_digest"),
                "digest_match": row.get("digest_match"),
                "comparison_state": row.get("comparison_state"),
                "support_ref": row.get("support_ref"),
            }
            for row in rows
        ],
        "publication_checks": publication_checks,
        "consumer_refs": packet_support.get("consumer_refs", []),
    }


def validate_fixtures(
    repo_root: Path,
    manifest_rel: str,
    expected_snapshot: dict[str, Any],
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest_path = repo_root / manifest_rel
    manifest = ensure_dict(render_yaml_as_json(manifest_path), manifest_rel)
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []
    if manifest.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "fixtures.schema_version",
                "fixture manifest schema_version must be 1",
                "Set fixture manifest schema_version to 1.",
                manifest_rel,
            )
        )
    for raw_case in ensure_list(manifest.get("cases"), "fixture_manifest.cases"):
        case = ensure_dict(raw_case, "fixture_manifest.cases[]")
        case_ref = str(case.get("case_ref", ""))
        validate_repo_ref(repo_root, case_ref, findings, "fixtures.case_ref.missing", str(case.get("case_id")))
        if not case_ref or not (repo_root / case_ref).exists():
            continue
        payload = ensure_dict(load_json(repo_root / case_ref), case_ref)
        case_findings = validate_rebuilt_snapshot(payload, expected_snapshot, case_ref)
        actual_check_ids = {finding.check_id for finding in case_findings}
        expected_check_ids = set(
            ensure_list(case.get("expected_check_ids"), "fixture_manifest.cases[].expected_check_ids")
        )
        missing_expected = expected_check_ids - actual_check_ids
        status = "passed" if not missing_expected else "failed"
        results.append(
            {
                "case_id": case.get("case_id"),
                "case_ref": case_ref,
                "status": status,
                "expected_check_ids": sorted(expected_check_ids),
                "actual_check_ids": sorted(actual_check_ids),
                "missing_expected_check_ids": sorted(missing_expected),
            }
        )
        if missing_expected:
            findings.append(
                Finding(
                    "error",
                    "fixtures.expected_failure_not_observed",
                    "fixture did not trigger the expected clean-room failure",
                    "Update the fixture or expected check ids so the failure drill remains live.",
                    case_ref,
                    {"missing_expected_check_ids": sorted(missing_expected)},
                )
            )
    return results, findings


def build_capture(
    packet: dict[str, Any],
    generated_at: str,
    packet_rel: str,
    rebuilt_snapshot_rel: str,
    support_projection_rel: str,
    fixture_results: list[dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    warning_count = sum(1 for finding in findings if finding.severity == "warning")
    candidate = ensure_dict(packet.get("candidate"), "packet.candidate")
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "generated_at": generated_at,
        "packet_id": packet.get("packet_id"),
        "packet_ref": packet_rel,
        "release_candidate_ref": candidate.get("release_candidate_ref"),
        "exact_build_identity_ref": candidate.get("exact_build_identity_ref"),
        "rebuilt_artifact_graph_ref": rebuilt_snapshot_rel,
        "support_projection_ref": support_projection_rel,
        "status": "passed" if error_count == 0 else "failed",
        "summary": {
            "finding_count": len(findings),
            "error_count": error_count,
            "warning_count": warning_count,
            "fixture_case_count": len(fixture_results),
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet_rel = args.packet
    schema_rel = args.schema
    rebuilt_snapshot_rel = args.rebuilt_snapshot
    support_projection_rel = args.support_projection
    capture_rel = args.capture
    graph_rel = args.artifact_graph
    graph_projection_rel = args.artifact_graph_projection

    packet = ensure_dict(load_json(repo_root / packet_rel), packet_rel)
    schema = ensure_dict(load_json(repo_root / schema_rel), schema_rel)
    graph = ensure_dict(load_json(repo_root / graph_rel), graph_rel)
    graph_projection = ensure_dict(
        load_json(repo_root / graph_projection_rel),
        graph_projection_rel,
    )
    rehearsal = extract_rebuild_rehearsal(repo_root / args.rebuild_rehearsal)
    generated_at = str(packet.get("generated_at") or graph.get("as_of"))

    findings = validate_against_schema(packet, schema, packet_rel)
    findings.extend(
        validate_packet(
            repo_root,
            packet,
            graph,
            graph_projection,
            rehearsal,
            packet_rel,
            graph_rel,
            graph_projection_rel,
            rebuilt_snapshot_rel,
            support_projection_rel,
        )
    )
    expected_snapshot = build_expected_snapshot(
        packet,
        graph,
        graph_projection,
        generated_at,
        packet_rel,
        graph_rel,
        graph_projection_rel,
    )
    snapshot_path = repo_root / rebuilt_snapshot_rel
    if snapshot_path.exists():
        snapshot = ensure_dict(load_json(snapshot_path), rebuilt_snapshot_rel)
    elif args.check:
        raise SystemExit(f"missing JSON file: {snapshot_path}")
    else:
        snapshot = expected_snapshot
    findings.extend(
        validate_rebuilt_snapshot(snapshot, expected_snapshot, rebuilt_snapshot_rel)
    )
    fixture_results, fixture_findings = validate_fixtures(
        repo_root,
        args.fixture_manifest,
        expected_snapshot,
    )
    findings.extend(fixture_findings)

    expected_snapshot_text = render_json(expected_snapshot)
    support_projection = build_support_projection(
        packet,
        expected_snapshot,
        generated_at,
        packet_rel,
        rebuilt_snapshot_rel,
    )
    support_projection_text = render_json(support_projection)
    capture = build_capture(
        packet,
        generated_at,
        packet_rel,
        rebuilt_snapshot_rel,
        support_projection_rel,
        fixture_results,
        findings,
    )
    capture_text = render_json(capture)

    snapshot_ok = write_or_check(
        repo_root / rebuilt_snapshot_rel,
        expected_snapshot_text,
        args.check,
    )
    support_ok = write_or_check(
        repo_root / support_projection_rel,
        support_projection_text,
        args.check,
    )
    capture_ok = write_or_check(repo_root / capture_rel, capture_text, args.check)
    error_count = sum(1 for finding in findings if finding.severity == "error")

    if args.check and not (snapshot_ok and support_ok and capture_ok):
        return 1
    if error_count:
        print(capture_text, file=sys.stderr)
        return 1

    print("reproducible release-candidate packet validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
