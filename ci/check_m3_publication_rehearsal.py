#!/usr/bin/env python3
"""Validate the M3 publication and clean-room rebuild rehearsal packets.

This validator is the first consumer for the three artifacts added by
the M3 publication-dry-run / clean-room rebuild rehearsal change set:

  - artifacts/benchmarks/m3/publication_dry_run/packet.md
  - artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md
  - artifacts/milestones/m3/proof_consumption_walkthrough.md

It parses the canonical YAML block embedded in each packet, cross-checks
the rows against the M3 public-proof index, the cross-milestone
evidence-freshness SLO catalog, and the cross-milestone rerun-trigger
catalog, then writes machine-readable validation captures so downstream
surfaces (docs, support exports, partner letters) can downgrade stale
rows without re-reading prose.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_BENCH_PACKET_REL = "artifacts/benchmarks/m3/publication_dry_run/packet.md"
DEFAULT_REBUILD_PACKET_REL = (
    "artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md"
)
DEFAULT_WALKTHROUGH_REL = "artifacts/milestones/m3/proof_consumption_walkthrough.md"
DEFAULT_INDEX_REL = "artifacts/milestones/m3/public_proof_index.md"
DEFAULT_SLO_REL = "artifacts/governance/evidence_freshness_slos.yaml"
DEFAULT_TRIGGER_REL = "artifacts/governance/evidence_rerun_triggers.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_BENCH_CAPTURE_REL = (
    "artifacts/benchmarks/m3/publication_dry_run/captures/"
    "publication_dry_run_validation_capture.json"
)
DEFAULT_REBUILD_CAPTURE_REL = (
    "artifacts/release/m3/clean_room_rebuild_rehearsal/captures/"
    "clean_room_rebuild_rehearsal_validation_capture.json"
)

BENCH_BLOCK_BEGIN = "<!-- BEGIN canonical:benchmark_publication_dry_run -->"
BENCH_BLOCK_END = "<!-- END canonical:benchmark_publication_dry_run -->"
REBUILD_BLOCK_BEGIN = "<!-- BEGIN canonical:clean_room_rebuild_rehearsal -->"
REBUILD_BLOCK_END = "<!-- END canonical:clean_room_rebuild_rehearsal -->"

STALE_AFTER_PATTERN = re.compile(r"^P(\d+)D$")

REQUIRED_BENCH_REBUILD_TRIGGERS: set[str] = {
    "exact_build_identity_chain_changed",
    "claim_row_or_channel_binding_changed",
    "schema_or_packet_header_contract_changed",
}

REQUIRED_BENCH_PUBLICATION_TRIGGERS: set[str] = {
    "reference_hardware_image_changed",
    "corpus_or_fixture_revision_changed",
    "protected_metrics_or_fitness_catalog_changed",
    "exact_build_identity_chain_changed",
}

REQUIRED_WALKTHROUGH_ENTRYPOINTS: list[str] = [
    "artifacts/milestones/m3/public_proof_index.md",
    "docs/governance/m3/publication_shelf_life_policy.md",
    "artifacts/milestones/m3/review_packet_template.md",
    "artifacts/release/m3/claim_manifest.md",
    "artifacts/benchmarks/m3/publication_dry_run/packet.md",
    "artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md",
    "artifacts/build/build_identity.json",
    "artifacts/governance/evidence_freshness_slos.yaml",
    "artifacts/governance/evidence_rerun_triggers.yaml",
]

REQUIRED_WALKTHROUGH_PATHS: list[str] = [
    "Partner reviewer path",
    "Docs writer path",
    "Support reader path",
    "Acceptance evidence",
    "Refresh trigger",
    "Failure drill",
    "ci/check_m3_publication_rehearsal.py",
]


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
    parser.add_argument("--bench-packet", default=DEFAULT_BENCH_PACKET_REL)
    parser.add_argument("--rebuild-packet", default=DEFAULT_REBUILD_PACKET_REL)
    parser.add_argument("--walkthrough", default=DEFAULT_WALKTHROUGH_REL)
    parser.add_argument("--public-proof-index", default=DEFAULT_INDEX_REL)
    parser.add_argument("--slo-catalog", default=DEFAULT_SLO_REL)
    parser.add_argument("--rerun-trigger-catalog", default=DEFAULT_TRIGGER_REL)
    parser.add_argument("--build-identity", default=DEFAULT_BUILD_IDENTITY_REL)
    parser.add_argument("--bench-capture", default=DEFAULT_BENCH_CAPTURE_REL)
    parser.add_argument("--rebuild-capture", default=DEFAULT_REBUILD_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail when the on-disk validation capture would change after "
            "regeneration. Use this in CI to keep the capture committed."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(STDIN.read,"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML for {label}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {label}: {exc}"
        ) from exc


def render_yaml_file_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_PATTERN.match(value)
    if not match:
        raise SystemExit(f"{label} must be of the form P<N>D, got {value!r}")
    return int(match.group(1))


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(
            f"{label} must be a YYYY-MM-DD date, got {value!r}"
        ) from exc


def extract_canonical_block(
    packet_text: str, begin: str, end: str, label: str
) -> str:
    if begin not in packet_text:
        raise SystemExit(f"{label}: missing BEGIN canonical sentinel")
    if end not in packet_text:
        raise SystemExit(f"{label}: missing END canonical sentinel")
    after_begin = packet_text.split(begin, 1)[1]
    block = after_begin.split(end, 1)[0]
    if "```yaml" not in block:
        raise SystemExit(
            f"{label}: canonical block must wrap one ```yaml ... ``` fence"
        )
    yaml_body = block.split("```yaml", 1)[1].split("```", 1)[0]
    if not yaml_body.strip():
        raise SystemExit(f"{label}: canonical YAML block is empty")
    return yaml_body


def ref_exists(repo_root: Path, ref: str) -> bool:
    target = ref.split("#", 1)[0].strip()
    return bool(target) and (repo_root / target).exists()


def index_proof_class_ceilings(slo_catalog: dict[str, Any]) -> dict[str, int]:
    rows = ensure_list(
        slo_catalog.get("proof_classes"), "slo_catalog.proof_classes"
    )
    out: dict[str, int] = {}
    for row in rows:
        proof_class_id = ensure_str(
            row.get("proof_class_id"),
            "slo_catalog.proof_classes[].proof_class_id",
        )
        max_stale_after = ensure_str(
            row.get("max_stale_after"),
            f"slo_catalog.proof_classes[{proof_class_id}].max_stale_after",
        )
        out[proof_class_id] = stale_after_days(
            max_stale_after,
            f"slo_catalog.proof_classes[{proof_class_id}].max_stale_after",
        )
    return out


def index_stale_propagation_profiles(slo_catalog: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        slo_catalog.get("stale_propagation_profiles"),
        "slo_catalog.stale_propagation_profiles",
    )
    return {
        ensure_str(
            row.get("profile_id"),
            "slo_catalog.stale_propagation_profiles[].profile_id",
        )
        for row in rows
    }


def index_trigger_ids(trigger_catalog: dict[str, Any]) -> set[str]:
    rows = ensure_list(
        trigger_catalog.get("trigger_rows"),
        "trigger_catalog.trigger_rows",
    )
    return {
        ensure_str(
            row.get("trigger_id"),
            "trigger_catalog.trigger_rows[].trigger_id",
        )
        for row in rows
    }


def index_public_proof_rows(public_proof_index_text: str) -> dict[str, Any]:
    begin = "<!-- BEGIN canonical:public_proof_index -->"
    end = "<!-- END canonical:public_proof_index -->"
    if begin not in public_proof_index_text or end not in public_proof_index_text:
        raise SystemExit(
            "public-proof index canonical sentinels not found; refresh the "
            "index first"
        )
    body = public_proof_index_text.split(begin, 1)[1].split(end, 1)[0]
    yaml_body = body.split("```yaml", 1)[1].split("```", 1)[0]
    index_payload = ensure_dict(
        render_yaml_as_json(yaml_body, "public_proof_index"),
        "public_proof_index",
    )
    rows = ensure_list(index_payload.get("rows"), "public_proof_index.rows")
    by_row_id: dict[str, Any] = {}
    for row in rows:
        row_id = ensure_str(
            row.get("row_id"), "public_proof_index.rows[].row_id"
        )
        by_row_id[row_id] = row
    return by_row_id


def validate_header(
    repo_root: Path,
    packet: dict[str, Any],
    label: str,
    findings: list[Finding],
) -> None:
    schema_version = packet.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=(
                    f"{label}: schema_version must be 1, got {schema_version!r}"
                ),
                remediation=(
                    "Bump the validator in the same change that bumps "
                    "schema_version."
                ),
            )
        )
    ensure_str(packet.get("packet_family"), f"{label}.packet_family")
    ensure_str(packet.get("packet_id"), f"{label}.packet_id")
    ensure_str(packet.get("evidence_id"), f"{label}.evidence_id")
    ensure_str(packet.get("title"), f"{label}.title")
    milestone_id = ensure_str(packet.get("milestone_id"), f"{label}.milestone_id")
    if milestone_id != "m3":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.milestone_id.unexpected",
                message=(
                    f"{label}: milestone_id must be 'm3', got {milestone_id!r}"
                ),
                remediation="Pin the milestone id to 'm3'.",
            )
        )
    release_channel_scope = ensure_str(
        packet.get("release_channel_scope"), f"{label}.release_channel_scope"
    )
    if release_channel_scope != "beta":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.release_channel_scope.unexpected",
                message=(
                    f"{label}: release_channel_scope must be 'beta', got "
                    f"{release_channel_scope!r}"
                ),
                remediation="Pin the release channel scope to 'beta'.",
            )
        )
    as_of = ensure_str(packet.get("as_of"), f"{label}.as_of")
    _ = parse_iso_date(as_of, f"{label}.as_of")

    ownership = ensure_dict(packet.get("ownership"), f"{label}.ownership")
    ensure_str(ownership.get("owner_dri"), f"{label}.ownership.owner_dri")
    ensure_str(
        ownership.get("evidence_owner"), f"{label}.ownership.evidence_owner"
    )


def validate_freshness(
    packet: dict[str, Any],
    label: str,
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    expected_proof_class: str,
    findings: list[Finding],
) -> int:
    freshness = ensure_dict(packet.get("freshness"), f"{label}.freshness")
    captured_at = ensure_str(
        freshness.get("captured_at"), f"{label}.freshness.captured_at"
    )
    try:
        dt.datetime.fromisoformat(captured_at.replace("Z", "+00:00"))
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.freshness.captured_at.invalid",
                message=(
                    f"{label}: freshness.captured_at must be ISO-8601, got "
                    f"{captured_at!r}"
                ),
                remediation="Use an ISO-8601 timestamp (UTC Z preferred).",
            )
        )

    stale_after = ensure_str(
        freshness.get("stale_after"), f"{label}.freshness.stale_after"
    )
    stale_after_d = stale_after_days(
        stale_after, f"{label}.freshness.stale_after"
    )
    proof_class_id = ensure_str(
        freshness.get("proof_class_id"), f"{label}.freshness.proof_class_id"
    )
    if proof_class_id != expected_proof_class:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.freshness.proof_class_id.unexpected",
                message=(
                    f"{label}: proof_class_id must be "
                    f"{expected_proof_class!r}, got {proof_class_id!r}"
                ),
                remediation=(
                    f"Pin the proof class to {expected_proof_class!r} so the "
                    "freshness ceiling matches the index row."
                ),
            )
        )
    if proof_class_id in proof_class_ceilings:
        ceiling = proof_class_ceilings[proof_class_id]
        if stale_after_d > ceiling:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.freshness.exceeds_ceiling",
                    message=(
                        f"{label}: stale_after {stale_after} exceeds "
                        f"proof_class_id {proof_class_id} ceiling P{ceiling}D"
                    ),
                    remediation=(
                        "Use a stale_after no wider than the proof class "
                        "ceiling."
                    ),
                )
            )
    else:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.freshness.proof_class_id.unknown",
                message=(
                    f"{label}: proof_class_id {proof_class_id!r} is not in "
                    "the SLO catalog"
                ),
                remediation=(
                    "Use a proof_class_id from "
                    "artifacts/governance/evidence_freshness_slos.yaml."
                ),
            )
        )

    propagation = ensure_str(
        freshness.get("stale_propagation_profile"),
        f"{label}.freshness.stale_propagation_profile",
    )
    if propagation not in propagation_profiles:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.freshness.stale_propagation_profile.unknown",
                message=(
                    f"{label}: stale_propagation_profile {propagation!r} is "
                    "not in the SLO catalog"
                ),
                remediation=(
                    "Use a profile_id from "
                    "artifacts/governance/evidence_freshness_slos.yaml#/"
                    "stale_propagation_profiles."
                ),
            )
        )

    return stale_after_d


def validate_refs(
    repo_root: Path,
    refs: list[str],
    check_id: str,
    label: str,
    findings: list[Finding],
) -> None:
    for ref in refs:
        if not ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=check_id,
                    message=f"{label}: referenced artifact does not exist: {ref}",
                    remediation=(
                        "Seed the artifact (or remove the ref) so the packet "
                        "stays inspectable."
                    ),
                    ref=ref,
                )
            )


def validate_rerun_triggers(
    packet: dict[str, Any],
    label: str,
    trigger_ids: set[str],
    required_subset: set[str],
    findings: list[Finding],
) -> list[str]:
    rerun = ensure_list(
        packet.get("rerun_trigger_refs"), f"{label}.rerun_trigger_refs"
    )
    if not rerun:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.rerun_trigger_refs.empty",
                message=f"{label}: must name at least one rerun trigger",
                remediation=(
                    "Cite at least one trigger id from "
                    "artifacts/governance/evidence_rerun_triggers.yaml."
                ),
            )
        )
    cited: list[str] = []
    for trigger_ref in rerun:
        trigger_ref = ensure_str(
            trigger_ref, f"{label}.rerun_trigger_refs[]"
        )
        cited.append(trigger_ref)
        if trigger_ref not in trigger_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.rerun_trigger_refs.unknown",
                    message=(
                        f"{label}: cites unknown rerun trigger {trigger_ref}"
                    ),
                    remediation=(
                        "Use a trigger_id from "
                        "artifacts/governance/evidence_rerun_triggers.yaml."
                    ),
                    ref=trigger_ref,
                )
            )
    missing_required = required_subset - set(cited)
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.rerun_trigger_refs.missing_required",
                message=(
                    f"{label}: missing required rerun triggers: "
                    f"{sorted(missing_required)}"
                ),
                remediation=(
                    "Add the missing trigger ids so the packet expires when "
                    "the underlying inputs change."
                ),
            )
        )
    return cited


def validate_latest_capture(
    repo_root: Path,
    packet: dict[str, Any],
    label: str,
    capture_rel: str,
    findings: list[Finding],
) -> None:
    latest = ensure_dict(
        packet.get("latest_capture"), f"{label}.latest_capture"
    )
    captured_at = ensure_str(
        latest.get("captured_at"), f"{label}.latest_capture.captured_at"
    )
    try:
        dt.datetime.fromisoformat(captured_at.replace("Z", "+00:00"))
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.latest_capture.captured_at.invalid",
                message=(
                    f"{label}: latest_capture.captured_at must be ISO-8601, "
                    f"got {captured_at!r}"
                ),
                remediation="Use an ISO-8601 timestamp (UTC Z preferred).",
            )
        )
    ensure_str(latest.get("command"), f"{label}.latest_capture.command")
    report_ref = ensure_str(
        latest.get("report_ref"), f"{label}.latest_capture.report_ref"
    )
    if report_ref != capture_rel:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.latest_capture.report_ref.mismatch",
                message=(
                    f"{label}: latest_capture.report_ref must equal "
                    f"{capture_rel!r}, got {report_ref!r}"
                ),
                remediation=(
                    "Pin the capture report ref to the validator's default "
                    "output path."
                ),
            )
        )


def validate_bench_packet(
    repo_root: Path,
    packet: dict[str, Any],
    public_proof_rows: dict[str, Any],
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    trigger_ids: set[str],
    capture_rel: str,
    findings: list[Finding],
) -> dict[str, Any]:
    label = "bench_packet"
    validate_header(repo_root, packet, label, findings)

    packet_family = packet.get("packet_family")
    if packet_family != "benchmark_publication_pack":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.packet_family.unexpected",
                message=(
                    f"{label}: packet_family must be "
                    "'benchmark_publication_pack', got "
                    f"{packet_family!r}"
                ),
                remediation=(
                    "Use the 'benchmark_publication_pack' packet family so "
                    "the SLO catalog selectors apply."
                ),
            )
        )

    coverage = ensure_dict(packet.get("coverage"), f"{label}.coverage")
    index_row_ref = ensure_str(
        coverage.get("public_proof_index_row_ref"),
        f"{label}.coverage.public_proof_index_row_ref",
    )
    if index_row_ref not in public_proof_rows:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.public_proof_index_row_ref.unknown",
                message=(
                    f"{label}: public_proof_index_row_ref {index_row_ref!r} "
                    "is not in the public-proof index"
                ),
                remediation=(
                    "Refresh the public-proof index or fix the row ref."
                ),
            )
        )
    elif index_row_ref != "m3_public_proof:benchmark_publication":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.public_proof_index_row_ref.unexpected",
                message=(
                    f"{label}: public_proof_index_row_ref must be "
                    "'m3_public_proof:benchmark_publication', got "
                    f"{index_row_ref!r}"
                ),
                remediation=(
                    "Bind the benchmark dry run to the "
                    "m3_public_proof:benchmark_publication row."
                ),
            )
        )

    claim_rows = ensure_list(
        coverage.get("claim_row_refs"), f"{label}.coverage.claim_row_refs"
    )
    if not claim_rows:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.claim_row_refs.empty",
                message=(
                    f"{label}: claim_row_refs must name at least one M3 claim row"
                ),
                remediation=(
                    "Bind the dry run to the canonical benchmark-publication "
                    "claim row."
                ),
            )
        )

    validate_freshness(
        packet,
        label,
        proof_class_ceilings,
        propagation_profiles,
        expected_proof_class="benchmark_publication_proof",
        findings=findings,
    )

    refs_to_check: list[str] = []
    corpus_pins = ensure_list(
        packet.get("corpus_pins"), f"{label}.corpus_pins"
    )
    if not corpus_pins:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.corpus_pins.empty",
                message=f"{label}: corpus_pins must name at least one fixture row",
                remediation=(
                    "Pin at least one fixture register row so the corpus is "
                    "inspectable."
                ),
            )
        )
    for idx, raw in enumerate(corpus_pins):
        row = ensure_dict(raw, f"{label}.corpus_pins[{idx}]")
        ensure_str(
            row.get("fixture_register_row_ref"),
            f"{label}.corpus_pins[{idx}].fixture_register_row_ref",
        )
        corpus_refs = ensure_list(
            row.get("corpus_refs"),
            f"{label}.corpus_pins[{idx}].corpus_refs",
        )
        for cref in corpus_refs:
            ensure_str(
                cref, f"{label}.corpus_pins[{idx}].corpus_refs[]"
            )
        fixture_packet_ref = ensure_str(
            row.get("fixture_packet_ref"),
            f"{label}.corpus_pins[{idx}].fixture_packet_ref",
        )
        refs_to_check.append(fixture_packet_ref)

    hw_pins = ensure_dict(
        packet.get("reference_hardware_pins"),
        f"{label}.reference_hardware_pins",
    )
    manifest_ref = ensure_str(
        hw_pins.get("manifest_ref"),
        f"{label}.reference_hardware_pins.manifest_ref",
    )
    refs_to_check.append(manifest_ref)
    lab_image_ref = ensure_str(
        hw_pins.get("lab_image_manifest_ref"),
        f"{label}.reference_hardware_pins.lab_image_manifest_ref",
    )
    refs_to_check.append(lab_image_ref)
    display_classes = ensure_list(
        hw_pins.get("display_class_refs"),
        f"{label}.reference_hardware_pins.display_class_refs",
    )
    if not display_classes:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.reference_hardware_pins.display_class_refs.empty",
                message=(
                    f"{label}: reference_hardware_pins.display_class_refs "
                    "must name at least one display class id"
                ),
                remediation=(
                    "Pin at least one display class from the reference "
                    "hardware manifest."
                ),
            )
        )

    threshold_pins = ensure_dict(
        packet.get("threshold_pins"), f"{label}.threshold_pins"
    )
    for field_name in (
        "protected_fitness_catalog_ref",
        "dashboard_snapshot_ref",
        "protected_metrics_ref",
        "canonical_fitness_catalog_ref",
    ):
        refs_to_check.append(
            ensure_str(
                threshold_pins.get(field_name),
                f"{label}.threshold_pins.{field_name}",
            )
        )
    ensure_str(
        threshold_pins.get("threshold_owner_dri"),
        f"{label}.threshold_pins.threshold_owner_dri",
    )
    ensure_str(
        threshold_pins.get("threshold_decision_forum_ref"),
        f"{label}.threshold_pins.threshold_decision_forum_ref",
    )
    ensure_str(
        threshold_pins.get("threshold_owning_lane"),
        f"{label}.threshold_pins.threshold_owning_lane",
    )

    artifact_links = ensure_dict(
        packet.get("artifact_links"), f"{label}.artifact_links"
    )
    build_identity_refs = ensure_list(
        artifact_links.get("exact_build_identity_refs"),
        f"{label}.artifact_links.exact_build_identity_refs",
    )
    if not build_identity_refs:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.artifact_links.exact_build_identity_refs.empty",
                message=(
                    f"{label}: artifact_links.exact_build_identity_refs must "
                    "name at least one build identity"
                ),
                remediation=(
                    "Bind the packet to "
                    "artifacts/build/build_identity.json."
                ),
            )
        )
    for bref in build_identity_refs:
        refs_to_check.append(
            ensure_str(
                bref,
                f"{label}.artifact_links.exact_build_identity_refs[]",
            )
        )
    refs_to_check.append(
        ensure_str(
            artifact_links.get("fixture_register_ref"),
            f"{label}.artifact_links.fixture_register_ref",
        )
    )
    for sref in ensure_list(
        artifact_links.get("source_anchor_refs"),
        f"{label}.artifact_links.source_anchor_refs",
    ):
        refs_to_check.append(
            ensure_str(
                sref,
                f"{label}.artifact_links.source_anchor_refs[]",
            )
        )

    known_limits = ensure_list(
        artifact_links.get("known_limit_refs"),
        f"{label}.artifact_links.known_limit_refs",
    )
    if not known_limits:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.artifact_links.known_limit_refs.empty",
                message=(
                    f"{label}: artifact_links.known_limit_refs must name at "
                    "least one rehearsal-specific limit"
                ),
                remediation=(
                    "Attach the rehearsal-specific known limits so no public "
                    "head-to-head claim is smuggled in."
                ),
            )
        )

    postures = ensure_list(
        packet.get("publication_postures"), f"{label}.publication_postures"
    )
    seen_postures: dict[str, bool] = {}
    for idx, raw in enumerate(postures):
        row = ensure_dict(raw, f"{label}.publication_postures[{idx}]")
        posture_id = ensure_str(
            row.get("posture_id"),
            f"{label}.publication_postures[{idx}].posture_id",
        )
        admitted = ensure_bool(
            row.get("admitted"),
            f"{label}.publication_postures[{idx}].admitted",
        )
        ensure_str(
            row.get("reason"),
            f"{label}.publication_postures[{idx}].reason",
        )
        seen_postures[posture_id] = admitted

    for forbidden in (
        "public_head_to_head_comparison",
        "certified_archetype_marketing",
    ):
        if seen_postures.get(forbidden, False) is True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.publication_postures.forbidden_admitted",
                    message=(
                        f"{label}: posture {forbidden!r} must not be admitted "
                        "by a methodology-only dry run"
                    ),
                    remediation=(
                        "Set admitted=false for public head-to-head and "
                        "certified-archetype marketing postures."
                    ),
                    ref=forbidden,
                )
            )
        elif forbidden not in seen_postures:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.publication_postures.missing_forbidden",
                    message=(
                        f"{label}: posture {forbidden!r} must be declared "
                        "with admitted=false"
                    ),
                    remediation=(
                        "Declare the posture row and explicitly refuse it."
                    ),
                    ref=forbidden,
                )
            )

    result_status = packet.get("result_status")
    if result_status != "methodology_only":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.result_status.unexpected",
                message=(
                    f"{label}: result_status must be 'methodology_only', got "
                    f"{result_status!r}"
                ),
                remediation=(
                    "Pin result_status to 'methodology_only' for the dry run."
                ),
            )
        )

    rerun = validate_rerun_triggers(
        packet,
        label,
        trigger_ids,
        REQUIRED_BENCH_PUBLICATION_TRIGGERS,
        findings,
    )

    validate_refs(
        repo_root,
        refs_to_check,
        check_id=f"{label}.refs.missing",
        label=label,
        findings=findings,
    )

    validate_latest_capture(
        repo_root, packet, label, capture_rel, findings
    )

    return {
        "packet_id": packet.get("packet_id"),
        "claim_rows": claim_rows,
        "rerun_triggers": rerun,
        "publication_postures": seen_postures,
    }


def validate_rebuild_packet(
    repo_root: Path,
    packet: dict[str, Any],
    public_proof_rows: dict[str, Any],
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    trigger_ids: set[str],
    capture_rel: str,
    build_identity_rel: str,
    findings: list[Finding],
) -> dict[str, Any]:
    label = "rebuild_packet"
    validate_header(repo_root, packet, label, findings)

    packet_family = packet.get("packet_family")
    if packet_family != "clean_room_rebuild_rehearsal":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.packet_family.unexpected",
                message=(
                    f"{label}: packet_family must be "
                    "'clean_room_rebuild_rehearsal', got "
                    f"{packet_family!r}"
                ),
                remediation=(
                    "Use the 'clean_room_rebuild_rehearsal' packet family."
                ),
            )
        )

    coverage = ensure_dict(packet.get("coverage"), f"{label}.coverage")
    index_row_ref = ensure_str(
        coverage.get("public_proof_index_row_ref"),
        f"{label}.coverage.public_proof_index_row_ref",
    )
    if index_row_ref not in public_proof_rows:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.public_proof_index_row_ref.unknown",
                message=(
                    f"{label}: public_proof_index_row_ref {index_row_ref!r} "
                    "is not in the public-proof index"
                ),
                remediation=(
                    "Refresh the public-proof index or fix the row ref."
                ),
            )
        )
    elif index_row_ref != "m3_public_proof:exact_build_identity":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.public_proof_index_row_ref.unexpected",
                message=(
                    f"{label}: public_proof_index_row_ref must be "
                    "'m3_public_proof:exact_build_identity', got "
                    f"{index_row_ref!r}"
                ),
                remediation=(
                    "Bind the rehearsal to the "
                    "m3_public_proof:exact_build_identity row."
                ),
            )
        )

    claim_rows = ensure_list(
        coverage.get("claim_row_refs"), f"{label}.coverage.claim_row_refs"
    )
    if not claim_rows:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.coverage.claim_row_refs.empty",
                message=(
                    f"{label}: claim_row_refs must name at least one M3 claim row"
                ),
                remediation=(
                    "Bind the rehearsal to the canonical exact-build "
                    "identity claim row."
                ),
            )
        )

    validate_freshness(
        packet,
        label,
        proof_class_ceilings,
        propagation_profiles,
        expected_proof_class="docs_claim_truth_proof",
        findings=findings,
    )

    refs_to_check: list[str] = []
    rebuild_lane = ensure_dict(
        packet.get("rebuild_lane"), f"{label}.rebuild_lane"
    )
    for field_name in (
        "lane_doc_ref",
        "baseline_doc_ref",
        "alpha_dry_run_ref",
        "provenance_capture_seed_ref",
    ):
        refs_to_check.append(
            ensure_str(
                rebuild_lane.get(field_name),
                f"{label}.rebuild_lane.{field_name}",
            )
        )
    ensure_str(
        rebuild_lane.get("command"), f"{label}.rebuild_lane.command"
    )
    ensure_str(
        rebuild_lane.get("offline_command"),
        f"{label}.rebuild_lane.offline_command",
    )

    exact_build = ensure_dict(
        packet.get("exact_build_identity"), f"{label}.exact_build_identity"
    )
    shared_axes_ref = ensure_str(
        exact_build.get("shared_axes_ref"),
        f"{label}.exact_build_identity.shared_axes_ref",
    )
    refs_to_check.append(shared_axes_ref)
    if shared_axes_ref != build_identity_rel:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.exact_build_identity.shared_axes_ref.mismatch",
                message=(
                    f"{label}: exact_build_identity.shared_axes_ref must "
                    f"equal {build_identity_rel!r}, got {shared_axes_ref!r}"
                ),
                remediation=(
                    "Pin the shared-axes ref to the canonical build identity."
                ),
            )
        )
    ensure_str(
        exact_build.get("comparison_basis"),
        f"{label}.exact_build_identity.comparison_basis",
    )
    byte_identity_claimed = ensure_bool(
        exact_build.get("byte_identity_claimed"),
        f"{label}.exact_build_identity.byte_identity_claimed",
    )
    if byte_identity_claimed is not False:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.exact_build_identity.byte_identity_claimed.unexpected",
                message=(
                    f"{label}: byte_identity_claimed must be false in the "
                    "rehearsal; byte-identical binaries are not yet claimed."
                ),
                remediation=(
                    "Set byte_identity_claimed=false in the canonical block."
                ),
            )
        )
    symbol_metadata_claimed = ensure_bool(
        exact_build.get("symbol_metadata_preservation_claimed"),
        f"{label}.exact_build_identity.symbol_metadata_preservation_claimed",
    )
    if symbol_metadata_claimed is not True:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    f"{label}.exact_build_identity"
                    ".symbol_metadata_preservation_claimed.unexpected"
                ),
                message=(
                    f"{label}: symbol_metadata_preservation_claimed must be "
                    "true; the rehearsal proves identity preservation."
                ),
                remediation=(
                    "Set symbol_metadata_preservation_claimed=true and keep "
                    "the rule string in the canonical block."
                ),
            )
        )
    ensure_str(
        exact_build.get("symbol_metadata_preservation_rule"),
        f"{label}.exact_build_identity.symbol_metadata_preservation_rule",
    )

    families = ensure_list(
        packet.get("artifact_families"), f"{label}.artifact_families"
    )
    if not families:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.artifact_families.empty",
                message=(
                    f"{label}: artifact_families must name at least one "
                    "rebuild artifact family"
                ),
                remediation=(
                    "Declare each artifact family the rebuild lane emits."
                ),
            )
        )
    seen_family_refs: set[str] = set()
    for idx, raw in enumerate(families):
        row = ensure_dict(raw, f"{label}.artifact_families[{idx}]")
        family_ref = ensure_str(
            row.get("artifact_family_ref"),
            f"{label}.artifact_families[{idx}].artifact_family_ref",
        )
        if family_ref in seen_family_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.artifact_families.duplicate",
                    message=(
                        f"{label}: duplicate artifact_family_ref "
                        f"{family_ref!r}"
                    ),
                    remediation="Artifact family refs must be unique.",
                    ref=family_ref,
                )
            )
        seen_family_refs.add(family_ref)
        ensure_str(
            row.get("artifact_id"),
            f"{label}.artifact_families[{idx}].artifact_id",
        )
        ensure_str(
            row.get("publishability_class"),
            f"{label}.artifact_families[{idx}].publishability_class",
        )
        ensure_str(
            row.get("artifact_graph_seed_ref"),
            f"{label}.artifact_families[{idx}].artifact_graph_seed_ref",
        )
        ensure_str(
            row.get("comparison_basis"),
            f"{label}.artifact_families[{idx}].comparison_basis",
        )

    required_families = {
        "workspace_build_identity.primary",
        "cleanroom_artifact_digest_manifest.primary",
        "workspace_sbom_stub.primary",
        "workspace_provenance_summary.primary",
        "cleanroom_input_manifest.primary",
        "cleanroom_provenance_capture.primary",
    }
    missing_families = required_families - seen_family_refs
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.artifact_families.missing_required",
                message=(
                    f"{label}: missing required artifact families: "
                    f"{sorted(missing_families)}"
                ),
                remediation=(
                    "Declare every clean-room rebuild lane artifact family."
                ),
            )
        )

    artifact_links = ensure_dict(
        packet.get("artifact_links"), f"{label}.artifact_links"
    )
    for bref in ensure_list(
        artifact_links.get("exact_build_identity_refs"),
        f"{label}.artifact_links.exact_build_identity_refs",
    ):
        refs_to_check.append(
            ensure_str(
                bref,
                f"{label}.artifact_links.exact_build_identity_refs[]",
            )
        )
    for sref in ensure_list(
        artifact_links.get("source_anchor_refs"),
        f"{label}.artifact_links.source_anchor_refs",
    ):
        refs_to_check.append(
            ensure_str(
                sref,
                f"{label}.artifact_links.source_anchor_refs[]",
            )
        )
    known_limits = ensure_list(
        artifact_links.get("known_limit_refs"),
        f"{label}.artifact_links.known_limit_refs",
    )
    if not known_limits:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.artifact_links.known_limit_refs.empty",
                message=(
                    f"{label}: artifact_links.known_limit_refs must name at "
                    "least one rehearsal-specific limit"
                ),
                remediation=(
                    "Attach the rehearsal-specific known limits so no "
                    "release-grade signing claim is smuggled in."
                ),
            )
        )

    result_status = packet.get("result_status")
    if result_status != "rehearsal_only":
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.result_status.unexpected",
                message=(
                    f"{label}: result_status must be 'rehearsal_only', got "
                    f"{result_status!r}"
                ),
                remediation=(
                    "Pin result_status to 'rehearsal_only' for the rehearsal."
                ),
            )
        )

    rerun = validate_rerun_triggers(
        packet,
        label,
        trigger_ids,
        REQUIRED_BENCH_REBUILD_TRIGGERS,
        findings,
    )

    validate_refs(
        repo_root,
        refs_to_check,
        check_id=f"{label}.refs.missing",
        label=label,
        findings=findings,
    )

    validate_latest_capture(
        repo_root, packet, label, capture_rel, findings
    )

    return {
        "packet_id": packet.get("packet_id"),
        "claim_rows": claim_rows,
        "rerun_triggers": rerun,
        "artifact_families": sorted(seen_family_refs),
        "byte_identity_claimed": byte_identity_claimed,
        "symbol_metadata_preservation_claimed": symbol_metadata_claimed,
    }


def validate_walkthrough(
    repo_root: Path,
    walkthrough_rel: str,
    findings: list[Finding],
) -> None:
    path = repo_root / walkthrough_rel
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="walkthrough.missing",
                message=(
                    f"proof-consumption walkthrough does not exist: "
                    f"{walkthrough_rel}"
                ),
                remediation="Seed the walkthrough.",
                ref=walkthrough_rel,
            )
        )
        return
    body = path.read_text(encoding="utf-8")
    for entrypoint in REQUIRED_WALKTHROUGH_ENTRYPOINTS:
        if entrypoint not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="walkthrough.missing_entrypoint",
                    message=(
                        "proof-consumption walkthrough is missing required "
                        f"entrypoint: {entrypoint}"
                    ),
                    remediation=(
                        "Cite the canonical entrypoint so partner / docs / "
                        "support readers can resolve it."
                    ),
                    ref=entrypoint,
                )
            )
        elif not ref_exists(repo_root, entrypoint):
            findings.append(
                Finding(
                    severity="error",
                    check_id="walkthrough.entrypoint_missing_on_disk",
                    message=(
                        "proof-consumption walkthrough cites entrypoint that "
                        f"does not exist on disk: {entrypoint}"
                    ),
                    remediation=(
                        "Seed the artifact or update the walkthrough."
                    ),
                    ref=entrypoint,
                )
            )
    for required_phrase in REQUIRED_WALKTHROUGH_PATHS:
        if required_phrase not in body:
            findings.append(
                Finding(
                    severity="error",
                    check_id="walkthrough.missing_phrase",
                    message=(
                        "proof-consumption walkthrough is missing required "
                        f"phrase: {required_phrase!r}"
                    ),
                    remediation=(
                        "Update the walkthrough so it carries each reader "
                        "path and the validator command."
                    ),
                )
            )


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def write_capture(
    capture_path: Path,
    check_id: str,
    packet_rel: str,
    derived: dict[str, Any],
    findings: list[Finding],
    generated_at: str,
    check_only: bool,
) -> bool:
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": check_id,
        "generated_at": generated_at,
        "packet_ref": packet_rel,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
        "derived": derived,
    }
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text: str | None = None
    if capture_path.exists():
        old_text = capture_path.read_text(encoding="utf-8")
    changed = old_text is None or _normalize(old_text) != _normalize(new_text)
    if not check_only:
        capture_path.write_text(new_text, encoding="utf-8")
    return changed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    bench_path = repo_root / args.bench_packet
    rebuild_path = repo_root / args.rebuild_packet
    if not bench_path.exists():
        raise SystemExit(f"missing benchmark dry-run packet: {args.bench_packet}")
    if not rebuild_path.exists():
        raise SystemExit(
            f"missing clean-room rebuild rehearsal packet: {args.rebuild_packet}"
        )

    bench_block = extract_canonical_block(
        bench_path.read_text(encoding="utf-8"),
        BENCH_BLOCK_BEGIN,
        BENCH_BLOCK_END,
        label=args.bench_packet,
    )
    rebuild_block = extract_canonical_block(
        rebuild_path.read_text(encoding="utf-8"),
        REBUILD_BLOCK_BEGIN,
        REBUILD_BLOCK_END,
        label=args.rebuild_packet,
    )

    bench_packet = ensure_dict(
        render_yaml_as_json(bench_block, args.bench_packet),
        f"{args.bench_packet}.canonical_block",
    )
    rebuild_packet = ensure_dict(
        render_yaml_as_json(rebuild_block, args.rebuild_packet),
        f"{args.rebuild_packet}.canonical_block",
    )

    slo_catalog = ensure_dict(
        render_yaml_file_as_json(repo_root / args.slo_catalog),
        "slo_catalog",
    )
    trigger_catalog = ensure_dict(
        render_yaml_file_as_json(repo_root / args.rerun_trigger_catalog),
        "trigger_catalog",
    )
    public_proof_index_path = repo_root / args.public_proof_index
    if not public_proof_index_path.exists():
        raise SystemExit(
            f"missing public-proof index: {args.public_proof_index}"
        )
    public_proof_rows = index_public_proof_rows(
        public_proof_index_path.read_text(encoding="utf-8")
    )

    proof_class_ceilings = index_proof_class_ceilings(slo_catalog)
    propagation_profiles = index_stale_propagation_profiles(slo_catalog)
    trigger_ids = index_trigger_ids(trigger_catalog)

    bench_findings: list[Finding] = []
    rebuild_findings: list[Finding] = []
    walkthrough_findings: list[Finding] = []

    bench_derived = validate_bench_packet(
        repo_root,
        bench_packet,
        public_proof_rows,
        proof_class_ceilings,
        propagation_profiles,
        trigger_ids,
        args.bench_capture,
        bench_findings,
    )
    rebuild_derived = validate_rebuild_packet(
        repo_root,
        rebuild_packet,
        public_proof_rows,
        proof_class_ceilings,
        propagation_profiles,
        trigger_ids,
        args.rebuild_capture,
        args.build_identity,
        rebuild_findings,
    )
    validate_walkthrough(repo_root, args.walkthrough, walkthrough_findings)

    generated_at = (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    all_findings = bench_findings + rebuild_findings + walkthrough_findings

    bench_capture_changed = write_capture(
        capture_path=repo_root / args.bench_capture,
        check_id="m3_benchmark_publication_dry_run",
        packet_rel=args.bench_packet,
        derived=bench_derived,
        findings=bench_findings,
        generated_at=generated_at,
        check_only=args.check,
    )
    rebuild_capture_changed = write_capture(
        capture_path=repo_root / args.rebuild_capture,
        check_id="m3_clean_room_rebuild_rehearsal",
        packet_rel=args.rebuild_packet,
        derived=rebuild_derived,
        findings=rebuild_findings + walkthrough_findings,
        generated_at=generated_at,
        check_only=args.check,
    )

    if args.check and (bench_capture_changed or rebuild_capture_changed):
        stale_finding = Finding(
            severity="error",
            check_id="capture.stale",
            message=(
                "checked-in publication-rehearsal capture is stale relative "
                "to the canonical packets, walkthrough, index, or SLO/trigger "
                "catalogs"
            ),
            remediation=(
                "Run `python3 ci/check_m3_publication_rehearsal.py "
                "--repo-root .` and commit the regenerated captures."
            ),
            details={
                "bench_capture_changed": bench_capture_changed,
                "rebuild_capture_changed": rebuild_capture_changed,
            },
        )
        all_findings.append(stale_finding)
        # Re-run to persist the up-to-date capture so the operator can commit.
        write_capture(
            capture_path=repo_root / args.bench_capture,
            check_id="m3_benchmark_publication_dry_run",
            packet_rel=args.bench_packet,
            derived=bench_derived,
            findings=bench_findings,
            generated_at=generated_at,
            check_only=False,
        )
        write_capture(
            capture_path=repo_root / args.rebuild_capture,
            check_id="m3_clean_room_rebuild_rehearsal",
            packet_rel=args.rebuild_packet,
            derived=rebuild_derived,
            findings=rebuild_findings + walkthrough_findings,
            generated_at=generated_at,
            check_only=False,
        )

    errors = [f for f in all_findings if f.severity == "error"]
    warnings = [f for f in all_findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m3-publication-rehearsal] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings)"
    )
    for finding in all_findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m3-publication-rehearsal] {prefix} {finding.check_id}: "
            f"{finding.message}{ref_suffix}"
        )
        print(
            f"[m3-publication-rehearsal]   remediation: {finding.remediation}"
        )
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-publication-rehearsal] interrupted", file=sys.stderr)
        sys.exit(130)
