#!/usr/bin/env python3
"""Validate the M3 beta public benchmark packet and public copy gate.

The gate is the headless consumer for the beta benchmark publication
packet. It keeps the release doc and partner-facing packet aligned with
the current benchmark evidence: methodology-only packets may publish
methodology, corpus, hardware, threshold, comparability, freshness, and
known-limit language, but they block wider public benchmark claims until
the council notes and packet say those claims are publishable.
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


DEFAULT_PACKET_REL = "artifacts/benchmarks/m3/publication_packet/packet.md"
DEFAULT_COUNCIL_NOTES_REL = "artifacts/benchmarks/m3/benchmark_council_notes.md"
DEFAULT_RELEASE_DOC_REL = "docs/release/m3/public_benchmark_beta.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/benchmarks/m3/publication_packet/captures/"
    "public_benchmark_beta_validation_capture.json"
)
DEFAULT_PUBLIC_PROOF_INDEX_REL = "artifacts/milestones/m3/public_proof_index.md"
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_SLO_REL = "artifacts/governance/evidence_freshness_slos.yaml"
DEFAULT_TRIGGER_REL = "artifacts/governance/evidence_rerun_triggers.yaml"

PACKET_BEGIN = "<!-- BEGIN canonical:public_benchmark_beta_packet -->"
PACKET_END = "<!-- END canonical:public_benchmark_beta_packet -->"
NOTES_BEGIN = "<!-- BEGIN canonical:benchmark_council_review_notes -->"
NOTES_END = "<!-- END canonical:benchmark_council_review_notes -->"
PUBLIC_PROOF_BEGIN = "<!-- BEGIN canonical:public_proof_index -->"
PUBLIC_PROOF_END = "<!-- END canonical:public_proof_index -->"

STALE_AFTER_RE = re.compile(r"^P(\d+)D$")
GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')

REQUIRED_RERUN_TRIGGERS = {
    "reference_hardware_image_changed",
    "corpus_or_fixture_revision_changed",
    "protected_metrics_or_fitness_catalog_changed",
    "exact_build_identity_chain_changed",
    "claim_row_or_channel_binding_changed",
}

REQUIRED_FALSE_POSTURES = {
    "public_head_to_head_comparison",
    "replacement_or_certified_performance_claim",
    "numeric_performance_win",
}

REQUIRED_TRUE_POSTURES = {
    "methodology_disclosure",
    "known_limits_disclosure",
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
        if self.ref is None:
            payload.pop("ref")
        if not self.details:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--council-notes", default=DEFAULT_COUNCIL_NOTES_REL)
    parser.add_argument("--release-doc", default=DEFAULT_RELEASE_DOC_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--public-proof-index", default=DEFAULT_PUBLIC_PROOF_INDEX_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument("--slo-catalog", default=DEFAULT_SLO_REL)
    parser.add_argument("--rerun-trigger-catalog", default=DEFAULT_TRIGGER_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the checked-in validation capture would change.",
    )
    return parser.parse_args()


def render_yaml_as_json(text: str, label: str) -> Any:
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
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
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
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {label}: {exc}") from exc


def load_yaml(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    return render_yaml_as_json(path.read_text(encoding="utf-8"), str(path))


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def extract_yaml_block(text: str, begin: str, end: str, label: str) -> dict[str, Any]:
    if begin not in text or end not in text:
        raise SystemExit(f"{label}: missing canonical sentinels")
    block = text.split(begin, 1)[1].split(end, 1)[0]
    if "```yaml" not in block:
        raise SystemExit(f"{label}: canonical block must contain a YAML fence")
    body = block.split("```yaml", 1)[1].split("```", 1)[0]
    if not body.strip():
        raise SystemExit(f"{label}: canonical YAML block is empty")
    return ensure_dict(render_yaml_as_json(body, label), label)


def ref_path(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def ref_exists(repo_root: Path, ref: str) -> bool:
    path = ref_path(ref)
    return bool(path) and (repo_root / path).exists()


def parse_stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_RE.match(value)
    if not match:
        raise SystemExit(f"{label} must use P<N>D duration syntax, got {value!r}")
    return int(match.group(1))


def parse_instant(value: str, label: str) -> dt.datetime | None:
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None
    if parsed.tzinfo is None:
        return parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    details: dict[str, Any] | None = None,
    severity: str = "error",
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def index_proof_class_ceilings(slo_catalog: dict[str, Any]) -> dict[str, int]:
    out: dict[str, int] = {}
    for raw in ensure_list(slo_catalog.get("proof_classes"), "slo.proof_classes"):
        row = ensure_dict(raw, "slo.proof_classes[]")
        proof_class = ensure_str(row.get("proof_class_id"), "proof_class_id")
        max_stale_after = ensure_str(row.get("max_stale_after"), "max_stale_after")
        out[proof_class] = parse_stale_after_days(
            max_stale_after,
            f"{proof_class}.max_stale_after",
        )
    return out


def index_propagation_profiles(slo_catalog: dict[str, Any]) -> set[str]:
    return {
        ensure_str(row.get("profile_id"), "profile_id")
        for row in ensure_list(
            slo_catalog.get("stale_propagation_profiles"),
            "slo.stale_propagation_profiles",
        )
        if isinstance(row, dict)
    }


def index_trigger_ids(trigger_catalog: dict[str, Any]) -> set[str]:
    return {
        ensure_str(row.get("trigger_id"), "trigger_id")
        for row in ensure_list(trigger_catalog.get("trigger_rows"), "trigger_rows")
        if isinstance(row, dict)
    }


def index_public_proof_rows(index_text: str) -> dict[str, dict[str, Any]]:
    index = extract_yaml_block(
        index_text,
        PUBLIC_PROOF_BEGIN,
        PUBLIC_PROOF_END,
        "public_proof_index",
    )
    return {
        ensure_str(row.get("row_id"), "public_proof.rows[].row_id"): row
        for row in ensure_list(index.get("rows"), "public_proof.rows")
        if isinstance(row, dict)
    }


def index_claim_manifest_rows(manifest: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        ensure_str(row.get("row_id"), "claim_manifest.rows[].row_id"): row
        for row in ensure_list(manifest.get("rows"), "claim_manifest.rows")
        if isinstance(row, dict)
    }


def validate_refs(
    repo_root: Path,
    refs: list[str],
    findings: list[Finding],
    label: str,
) -> None:
    for ref in refs:
        if not ref_exists(repo_root, ref):
            add_finding(
                findings,
                f"{label}.ref_missing",
                f"{label} references a missing artifact: {ref}",
                "Seed the referenced artifact or remove the stale ref.",
                ref=ref,
            )


def validate_packet(
    repo_root: Path,
    packet: dict[str, Any],
    *,
    packet_rel: str,
    capture_rel: str,
    proof_rows: dict[str, dict[str, Any]],
    manifest_rows: dict[str, dict[str, Any]],
    proof_class_ceilings: dict[str, int],
    propagation_profiles: set[str],
    trigger_ids: set[str],
    findings: list[Finding],
) -> dict[str, Any]:
    if packet.get("schema_version") != 1:
        add_finding(
            findings,
            "packet.schema_version",
            f"packet schema_version must be 1, got {packet.get('schema_version')!r}",
            "Update the validator in the same change as any packet schema bump.",
            ref=packet_rel,
        )

    packet_kind = ensure_str(packet.get("packet_kind"), "packet.packet_kind")
    if packet_kind != "m3_public_benchmark_beta_packet":
        add_finding(
            findings,
            "packet.packet_kind",
            f"unexpected packet_kind {packet_kind!r}",
            "Use m3_public_benchmark_beta_packet for this beta lane.",
            ref=packet_rel,
        )

    result_status = ensure_str(packet.get("result_status"), "packet.result_status")
    if result_status != "methodology_only":
        add_finding(
            findings,
            "packet.result_status",
            f"result_status must be methodology_only, got {result_status!r}",
            "Downgrade to methodology_only or land current comparable benchmark proof first.",
            ref=packet_rel,
        )

    claim_family = ensure_str(packet.get("claim_family"), "packet.claim_family")
    if claim_family != "benchmark_publication":
        add_finding(
            findings,
            "packet.claim_family",
            f"claim_family must be benchmark_publication, got {claim_family!r}",
            "Bind this packet to the benchmark_publication claim family.",
            ref=packet_rel,
        )

    proof_row = proof_rows.get("m3_public_proof:benchmark_publication")
    if proof_row is None:
        add_finding(
            findings,
            "packet.public_proof_row_missing",
            "public-proof index is missing the benchmark_publication row",
            "Add or restore the m3_public_proof:benchmark_publication row.",
            ref=DEFAULT_PUBLIC_PROOF_INDEX_REL,
        )
    else:
        canonical_ref = ensure_str(
            proof_row.get("canonical_packet_ref"),
            "benchmark_publication.canonical_packet_ref",
        )
        if canonical_ref != packet_rel:
            add_finding(
                findings,
                "packet.public_proof_row_mismatch",
                (
                    "public-proof benchmark row points at "
                    f"{canonical_ref!r}, expected {packet_rel!r}"
                ),
                "Update the public-proof index so the beta packet is the canonical packet.",
                ref=DEFAULT_PUBLIC_PROOF_INDEX_REL,
            )

    coverage = ensure_dict(packet.get("coverage"), "packet.coverage")
    claim_refs = [
        ensure_str(ref, "packet.coverage.claim_row_refs[]")
        for ref in ensure_list(coverage.get("claim_row_refs"), "packet.coverage.claim_row_refs")
    ]
    if "m3_claim_row:canonical.benchmark.publication_truth" not in claim_refs:
        add_finding(
            findings,
            "packet.claim_row_missing",
            "packet does not cite the canonical M3 benchmark publication claim row",
            "Add m3_claim_row:canonical.benchmark.publication_truth to coverage.claim_row_refs.",
            ref=packet_rel,
        )
    for claim_ref in claim_refs:
        if claim_ref not in manifest_rows:
            add_finding(
                findings,
                "packet.claim_row_unknown",
                f"packet cites unknown claim manifest row: {claim_ref}",
                "Use a row_id from artifacts/release/m3/claim_manifest.json.",
                ref=claim_ref,
            )

    freshness = ensure_dict(packet.get("freshness"), "packet.freshness")
    captured_at = ensure_str(freshness.get("captured_at"), "packet.freshness.captured_at")
    if parse_instant(captured_at, "packet.freshness.captured_at") is None:
        add_finding(
            findings,
            "packet.freshness.captured_at_invalid",
            f"freshness.captured_at is not ISO-8601: {captured_at!r}",
            "Use an ISO-8601 UTC timestamp.",
            ref=packet_rel,
        )
    stale_after = ensure_str(freshness.get("stale_after"), "packet.freshness.stale_after")
    stale_days = parse_stale_after_days(stale_after, "packet.freshness.stale_after")
    proof_class = ensure_str(freshness.get("proof_class_id"), "packet.freshness.proof_class_id")
    if proof_class != "benchmark_publication_proof":
        add_finding(
            findings,
            "packet.freshness.proof_class",
            f"proof_class_id must be benchmark_publication_proof, got {proof_class!r}",
            "Use the benchmark publication proof class so the SLO ceiling applies.",
            ref=packet_rel,
        )
    ceiling = proof_class_ceilings.get(proof_class)
    if ceiling is None:
        add_finding(
            findings,
            "packet.freshness.unknown_proof_class",
            f"unknown proof_class_id {proof_class!r}",
            "Use a proof class from artifacts/governance/evidence_freshness_slos.yaml.",
            ref=proof_class,
        )
    elif stale_days > ceiling:
        add_finding(
            findings,
            "packet.freshness.exceeds_ceiling",
            f"stale_after {stale_after} exceeds {proof_class} ceiling P{ceiling}D",
            "Use a freshness window no wider than the proof class ceiling.",
            ref=packet_rel,
        )
    propagation = ensure_str(
        freshness.get("stale_propagation_profile"),
        "packet.freshness.stale_propagation_profile",
    )
    if propagation not in propagation_profiles:
        add_finding(
            findings,
            "packet.freshness.unknown_propagation_profile",
            f"unknown stale_propagation_profile {propagation!r}",
            "Use a profile from artifacts/governance/evidence_freshness_slos.yaml.",
            ref=propagation,
        )

    refs_to_check: list[str] = []
    corpus_pins = ensure_list(packet.get("corpus_pins"), "packet.corpus_pins")
    if not corpus_pins:
        add_finding(
            findings,
            "packet.corpus.empty",
            "packet must include at least one corpus pin",
            "Pin the fixture register rows and corpus refs used by the beta packet.",
            ref=packet_rel,
        )
    for idx, raw in enumerate(corpus_pins):
        row = ensure_dict(raw, f"packet.corpus_pins[{idx}]")
        ensure_str(row.get("fixture_register_row_ref"), "fixture_register_row_ref")
        corpus_refs = ensure_list(row.get("corpus_refs"), "corpus_refs")
        if not corpus_refs:
            add_finding(
                findings,
                "packet.corpus.refs_empty",
                "corpus pin has no corpus_refs",
                "Add at least one corpus ref for every fixture register row.",
                ref=packet_rel,
            )
        refs_to_check.append(ensure_str(row.get("fixture_packet_ref"), "fixture_packet_ref"))

    hardware = ensure_dict(packet.get("reference_hardware_pins"), "packet.reference_hardware_pins")
    for key in ("manifest_ref", "lab_image_manifest_ref"):
        refs_to_check.append(ensure_str(hardware.get(key), f"reference_hardware_pins.{key}"))
    if not ensure_list(hardware.get("hardware_definition_refs"), "hardware_definition_refs"):
        add_finding(
            findings,
            "packet.hardware.missing_rows",
            "packet must name at least one hardware_definition_ref",
            "Pin the reference hardware row used by the packet.",
            ref=packet_rel,
        )
    if not ensure_list(hardware.get("display_class_refs"), "display_class_refs"):
        add_finding(
            findings,
            "packet.hardware.missing_display",
            "packet must name at least one display_class_ref",
            "Pin the display class used by the packet.",
            ref=packet_rel,
        )

    thresholds = ensure_dict(packet.get("threshold_pins"), "packet.threshold_pins")
    for key in (
        "protected_fitness_catalog_ref",
        "dashboard_snapshot_ref",
        "protected_metrics_ref",
        "canonical_fitness_catalog_ref",
    ):
        refs_to_check.append(ensure_str(thresholds.get(key), f"threshold_pins.{key}"))
    ensure_str(thresholds.get("threshold_owner_dri"), "threshold_owner_dri")
    ensure_str(thresholds.get("threshold_review_state"), "threshold_review_state")

    comparability = ensure_dict(packet.get("comparability"), "packet.comparability")
    comparability_class = ensure_str(
        comparability.get("comparability_class"),
        "packet.comparability.comparability_class",
    )
    if result_status == "methodology_only" and comparability_class != "not_yet_comparable":
        add_finding(
            findings,
            "packet.comparability.too_wide",
            (
                "methodology-only packet must use comparability_class "
                f"not_yet_comparable, got {comparability_class!r}"
            ),
            "Narrow comparability or land current full-corpus comparable results first.",
            ref=packet_rel,
        )
    ensure_str(comparability.get("freshness_state"), "packet.comparability.freshness_state")
    ensure_str(comparability.get("comparability_note"), "packet.comparability.comparability_note")

    postures: dict[str, bool] = {}
    for raw in ensure_list(packet.get("publication_postures"), "packet.publication_postures"):
        row = ensure_dict(raw, "publication_postures[]")
        posture_id = ensure_str(row.get("posture_id"), "posture_id")
        postures[posture_id] = ensure_bool(row.get("admitted"), f"{posture_id}.admitted")
        ensure_str(row.get("reason"), f"{posture_id}.reason")
    for posture in REQUIRED_FALSE_POSTURES:
        if postures.get(posture) is not False:
            add_finding(
                findings,
                "packet.posture_must_be_blocked",
                f"posture {posture!r} must be present with admitted=false",
                "Block wider benchmark language while the packet remains methodology-only.",
                ref=posture,
            )
    for posture in REQUIRED_TRUE_POSTURES:
        if postures.get(posture) is not True:
            add_finding(
                findings,
                "packet.posture_must_be_publishable",
                f"posture {posture!r} must be present with admitted=true",
                "Allow methodology and known-limit disclosure explicitly.",
                ref=posture,
            )

    rerun_refs = [
        ensure_str(ref, "packet.rerun_trigger_refs[]")
        for ref in ensure_list(packet.get("rerun_trigger_refs"), "packet.rerun_trigger_refs")
    ]
    for trigger in rerun_refs:
        if trigger not in trigger_ids:
            add_finding(
                findings,
                "packet.rerun_trigger_unknown",
                f"unknown rerun trigger {trigger!r}",
                "Use trigger ids from artifacts/governance/evidence_rerun_triggers.yaml.",
                ref=trigger,
            )
    missing_triggers = sorted(REQUIRED_RERUN_TRIGGERS - set(rerun_refs))
    if missing_triggers:
        add_finding(
            findings,
            "packet.rerun_trigger_missing_required",
            f"packet is missing required rerun triggers: {missing_triggers}",
            "Add the missing triggers so freshness expires on material benchmark input changes.",
            ref=packet_rel,
        )

    artifact_links = ensure_dict(packet.get("artifact_links"), "packet.artifact_links")
    for key in ("exact_build_identity_refs", "source_anchor_refs"):
        for ref in ensure_list(artifact_links.get(key), f"artifact_links.{key}"):
            refs_to_check.append(ensure_str(ref, f"artifact_links.{key}[]"))
    refs_to_check.extend(
        ensure_str(ref, "governed_public_surfaces[].surface_ref")
        for ref in [
            ensure_dict(row, "governed_public_surfaces[]").get("surface_ref")
            for row in ensure_list(
                packet.get("governed_public_surfaces"),
                "packet.governed_public_surfaces",
            )
        ]
    )
    validate_refs(repo_root, refs_to_check, findings, "packet")

    latest = ensure_dict(packet.get("latest_capture"), "packet.latest_capture")
    report_ref = ensure_str(latest.get("report_ref"), "packet.latest_capture.report_ref")
    if report_ref != capture_rel:
        add_finding(
            findings,
            "packet.latest_capture.report_ref",
            f"latest_capture.report_ref must be {capture_rel!r}, got {report_ref!r}",
            "Pin latest_capture.report_ref to the validator capture path.",
            ref=packet_rel,
        )

    return {
        "packet_id": packet.get("packet_id"),
        "result_status": result_status,
        "claim_family": claim_family,
        "freshness_class": freshness.get("freshness_class"),
        "comparability_class": comparability_class,
        "publication_postures": postures,
        "rerun_trigger_refs": rerun_refs,
    }


def validate_council_notes(
    notes: dict[str, Any],
    *,
    packet_rel: str,
    findings: list[Finding],
) -> dict[str, Any]:
    if notes.get("schema_version") != 1:
        add_finding(
            findings,
            "council_notes.schema_version",
            "benchmark council notes schema_version must be 1",
            "Update the validator alongside any schema bump.",
            ref=DEFAULT_COUNCIL_NOTES_REL,
        )
    packet_ref = ensure_str(notes.get("packet_ref"), "council_notes.packet_ref")
    if packet_ref != packet_rel:
        add_finding(
            findings,
            "council_notes.packet_ref",
            f"council notes packet_ref must be {packet_rel!r}, got {packet_ref!r}",
            "Bind the notes to the current beta benchmark packet.",
            ref=DEFAULT_COUNCIL_NOTES_REL,
        )
    decision = ensure_str(notes.get("review_decision"), "council_notes.review_decision")
    if decision != "keep_methodology_only":
        add_finding(
            findings,
            "council_notes.decision_too_wide",
            f"review_decision must be keep_methodology_only, got {decision!r}",
            "Keep public copy narrow until rerun and comparability blockers close.",
            ref=DEFAULT_COUNCIL_NOTES_REL,
        )

    publishable = ensure_list(notes.get("publishable_claims"), "publishable_claims")
    internal = ensure_list(notes.get("internal_only_items"), "internal_only_items")
    reruns = ensure_list(notes.get("rerun_before_promotion"), "rerun_before_promotion")
    blocked = ensure_list(notes.get("blocked_public_claims"), "blocked_public_claims")
    for list_name, value in (
        ("publishable_claims", publishable),
        ("internal_only_items", internal),
        ("rerun_before_promotion", reruns),
        ("blocked_public_claims", blocked),
    ):
        if not value:
            add_finding(
                findings,
                f"council_notes.{list_name}.empty",
                f"council notes must include at least one {list_name} row",
                "Record the council disposition so public copy can be gated mechanically.",
                ref=DEFAULT_COUNCIL_NOTES_REL,
            )

    return {
        "review_decision": decision,
        "publishable_claim_ids": [
            ensure_str(ensure_dict(row, "publishable_claims[]").get("claim_id"), "claim_id")
            for row in publishable
            if isinstance(row, dict)
        ],
        "internal_only_item_ids": [
            ensure_str(ensure_dict(row, "internal_only_items[]").get("item_id"), "item_id")
            for row in internal
            if isinstance(row, dict)
        ],
        "rerun_ids": [
            ensure_str(ensure_dict(row, "rerun_before_promotion[]").get("rerun_id"), "rerun_id")
            for row in reruns
            if isinstance(row, dict)
        ],
        "blocked_claim_ids": [
            ensure_str(ensure_dict(row, "blocked_public_claims[]").get("claim_id"), "claim_id")
            for row in blocked
            if isinstance(row, dict)
        ],
    }


def validate_governed_surfaces(
    repo_root: Path,
    packet: dict[str, Any],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    results: list[dict[str, Any]] = []
    surfaces = ensure_list(
        packet.get("governed_public_surfaces"),
        "packet.governed_public_surfaces",
    )
    for raw in surfaces:
        row = ensure_dict(raw, "governed_public_surfaces[]")
        surface_ref = ensure_str(row.get("surface_ref"), "surface_ref")
        path = repo_root / ref_path(surface_ref)
        required_tokens = [
            ensure_str(token, "required_tokens[]")
            for token in ensure_list(row.get("required_tokens"), "required_tokens")
        ]
        blocked_phrases = [
            ensure_str(token, "blocked_phrases[]")
            for token in ensure_list(row.get("blocked_phrases"), "blocked_phrases")
        ]
        status = "pass"
        if not path.exists():
            add_finding(
                findings,
                "surface.missing",
                f"governed public benchmark surface is missing: {surface_ref}",
                "Seed the release doc or partner packet before publication.",
                ref=surface_ref,
            )
            status = "fail"
            results.append({"surface_ref": surface_ref, "status": status})
            continue
        body = path.read_text(encoding="utf-8")
        lower_body = body.lower()
        missing = [token for token in required_tokens if token not in body]
        if missing:
            add_finding(
                findings,
                "surface.required_token_missing",
                f"{surface_ref} is missing required public benchmark tokens: {missing}",
                "Render the bounded benchmark packet refs and posture tokens on the surface.",
                ref=surface_ref,
                details={"missing_tokens": missing},
            )
            status = "fail"
        blocked_seen = [
            phrase for phrase in blocked_phrases if phrase.lower() in lower_body
        ]
        if blocked_seen:
            add_finding(
                findings,
                "surface.blocked_phrase_present",
                f"{surface_ref} contains unsupported public benchmark wording",
                "Remove the wider benchmark claim or refresh the packet and council notes first.",
                ref=surface_ref,
                details={"blocked_phrases": blocked_seen},
            )
            status = "fail"
        results.append(
            {
                "surface_ref": surface_ref,
                "surface_kind": row.get("surface_kind"),
                "status": status,
                "required_token_count": len(required_tokens),
                "blocked_phrase_count": len(blocked_phrases),
            }
        )
    return results


def normalized(text: str) -> str:
    return GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def write_capture(path: Path, payload: dict[str, Any], check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text = path.read_text(encoding="utf-8") if path.exists() else None
    changed = old_text is None or normalized(old_text) != normalized(new_text)
    if not check_only:
        path.write_text(new_text, encoding="utf-8")
    return changed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    packet_path = repo_root / args.packet
    notes_path = repo_root / args.council_notes
    if not packet_path.exists():
        raise SystemExit(f"missing public benchmark packet: {args.packet}")
    if not notes_path.exists():
        raise SystemExit(f"missing benchmark council notes: {args.council_notes}")

    packet = extract_yaml_block(
        packet_path.read_text(encoding="utf-8"),
        PACKET_BEGIN,
        PACKET_END,
        args.packet,
    )
    notes = extract_yaml_block(
        notes_path.read_text(encoding="utf-8"),
        NOTES_BEGIN,
        NOTES_END,
        args.council_notes,
    )

    proof_rows = index_public_proof_rows(
        (repo_root / args.public_proof_index).read_text(encoding="utf-8")
    )
    manifest_rows = index_claim_manifest_rows(
        ensure_dict(load_json(repo_root / args.claim_manifest), "claim_manifest")
    )
    slo_catalog = ensure_dict(load_yaml(repo_root / args.slo_catalog), "slo_catalog")
    trigger_catalog = ensure_dict(
        load_yaml(repo_root / args.rerun_trigger_catalog),
        "rerun_trigger_catalog",
    )

    findings: list[Finding] = []
    packet_derived = validate_packet(
        repo_root,
        packet,
        packet_rel=args.packet,
        capture_rel=args.capture,
        proof_rows=proof_rows,
        manifest_rows=manifest_rows,
        proof_class_ceilings=index_proof_class_ceilings(slo_catalog),
        propagation_profiles=index_propagation_profiles(slo_catalog),
        trigger_ids=index_trigger_ids(trigger_catalog),
        findings=findings,
    )
    council_derived = validate_council_notes(
        notes,
        packet_rel=args.packet,
        findings=findings,
    )
    surface_results = validate_governed_surfaces(repo_root, packet, findings)

    generated_at = now_iso_z()
    payload = {
        "schema_version": 1,
        "check_id": "m3_public_benchmark_beta",
        "generated_at": generated_at,
        "packet_ref": args.packet,
        "council_notes_ref": args.council_notes,
        "release_doc_ref": args.release_doc,
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "derived": {
            "packet": packet_derived,
            "council_notes": council_derived,
            "governed_public_surfaces": surface_results,
        },
        "findings": [finding.as_report() for finding in findings],
    }

    changed = write_capture(repo_root / args.capture, payload, args.check)
    if args.check and changed:
        add_finding(
            findings,
            "capture.stale",
            "checked-in public benchmark validation capture is stale",
            "Run `python3 ci/check_m3_public_benchmark_beta.py --repo-root .` and commit the capture.",
            ref=args.capture,
        )
        payload["status"] = "fail"
        payload["finding_counts"] = {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        }
        payload["findings"] = [finding.as_report() for finding in findings]
        write_capture(repo_root / args.capture, payload, False)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m3-public-benchmark-beta] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- capture: {args.capture}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m3-public-benchmark-beta] {prefix} {finding.check_id}: "
            f"{finding.message}{ref}"
        )
        print(f"[m3-public-benchmark-beta]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-public-benchmark-beta] interrupted", file=sys.stderr)
        sys.exit(130)
