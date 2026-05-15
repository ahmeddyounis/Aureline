#!/usr/bin/env python3
"""Generate and validate the M3 governed claim manifest.

The script reads the M3 claim-manifest matrix and the canonical
upstream seeds (claim-manifest seed, claimed-surface register,
cohort/archetype scorecard register, M3 compatibility report, cohort
guardrails, version-skew register) and writes:

  - artifacts/release/m3/claim_manifest.json -- machine-readable
    record conforming to schemas/release/m3_claim_manifest.schema.json;
  - artifacts/release/m3/claim_manifest.md   -- reviewer-facing
    rendering with the same row truth; and
  - artifacts/release/m3/captures/claim_manifest_validation_capture.json
    -- checked-in validation capture for downstream reviewers.

The manifest is generated from the matrix plus the canonical seeds so
docs, Help/About, service health, support exports, release packets,
and partner packets all read one machine-derived public-truth row
shape.
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


DEFAULT_MATRIX_REL = "artifacts/release/m3/claim_manifest_matrix.yaml"
DEFAULT_MANIFEST_JSON_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_MANIFEST_MD_REL = "artifacts/release/m3/claim_manifest.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/captures/claim_manifest_validation_capture.json"
)
DEFAULT_SCHEMA_REL = "schemas/release/m3_claim_manifest.schema.json"


VALID_ROW_KINDS = {
    "canonical_claim_family",
    "beta_surface_binding",
    "beta_archetype_binding",
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
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--manifest-json", default=DEFAULT_MANIFEST_JSON_REL)
    parser.add_argument("--manifest-md", default=DEFAULT_MANIFEST_MD_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail if the on-disk manifest or capture would change after "
            "regeneration. Use this in CI to keep the checked-in artifacts "
            "fresh against the matrix and upstream seeds."
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


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


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


def index_seed_claim_rows(seed: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(seed.get("claim_rows"), "claim_manifest_seed.claim_rows")
    return {ensure_str(r.get("claim_row_id"), "claim_row.claim_row_id"): r for r in rows}


def index_claimed_surfaces(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    surfaces = ensure_list(
        register.get("claimed_surfaces"),
        "claimed_surface_register.claimed_surfaces",
    )
    return {ensure_str(s.get("surface_id"), "claimed_surface.surface_id"): s for s in surfaces}


def index_claimed_archetypes(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(
        register.get("claimed_archetype_rows"),
        "claimed_surface_register.claimed_archetype_rows",
    )
    return {
        ensure_str(r.get("archetype_surface_id"), "archetype_row.archetype_surface_id"): r
        for r in rows
    }


def index_scorecards(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(
        register.get("archetype_rows"), "scorecard_register.archetype_rows"
    ) + ensure_list(
        register.get("cohort_rows"), "scorecard_register.cohort_rows"
    )
    return {ensure_str(r.get("scorecard_id"), "scorecard.scorecard_id"): r for r in rows}


def index_compat_rows(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(report.get("rows"), "compatibility_report.rows")
    return {ensure_str(r.get("row_id"), "compat_row.row_id"): r for r in rows}


def index_cohorts(guardrails: dict[str, Any]) -> set[str]:
    cohorts = ensure_list(guardrails.get("cohorts"), "cohort_guardrails.cohorts")
    return {ensure_str(c.get("cohort_id"), "cohort.cohort_id") for c in cohorts}


def index_skew_register(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(register.get("register"), "version_skew_register.register")
    return {ensure_str(r.get("register_id"), "skew_register.register_id"): r for r in rows}


def map_claim_family_to_kind(family: str) -> str:
    """Return the manifest_row.claim_family value for a seed claim_family."""

    return family


def derive_lifecycle_label(claim_rows: list[dict[str, Any]]) -> str:
    """Derive a single display lifecycle label from inherited claim rows.

    M3 publishes everything as a beta-class manifest. If any inherited
    claim row is `preview`, the manifest row shows `preview`; otherwise
    `beta`. The display label is independent of the canonical claim
    row's own lifecycle_state because the manifest is the M3 beta
    publication.
    """

    for row in claim_rows:
        lifecycle = ensure_str(
            ensure_dict(row.get("lifecycle_support"), f"{row.get('claim_row_id')}.lifecycle_support").get("lifecycle_state"),
            f"{row.get('claim_row_id')}.lifecycle_support.lifecycle_state",
        )
        if lifecycle == "preview":
            return "preview"
    return "beta"


def collect_requirement_ids(claim_rows: list[dict[str, Any]]) -> list[str]:
    ids: list[str] = []
    for row in claim_rows:
        for ref in ensure_list(
            row.get("requirement_ids"),
            f"{row.get('claim_row_id')}.requirement_ids",
        ):
            sref = ensure_str(ref, f"{row.get('claim_row_id')}.requirement_ids[]")
            if sref not in ids:
                ids.append(sref)
    return ids


def collect_evidence_links(claim_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    links: list[dict[str, Any]] = []
    seen: set[str] = set()
    for row in claim_rows:
        for link in ensure_list(
            row.get("evidence_links"),
            f"{row.get('claim_row_id')}.evidence_links",
        ):
            link = ensure_dict(link, f"{row.get('claim_row_id')}.evidence_links[]")
            evid = ensure_str(
                link.get("claim_evidence_id"),
                "claim_evidence_id",
            )
            if evid in seen:
                continue
            seen.add(evid)
            links.append(
                {
                    "claim_evidence_id": evid,
                    "evidence_kind": ensure_str(
                        link.get("evidence_kind"), "evidence_kind"
                    ),
                    "packet_ref": ensure_str(link.get("packet_ref"), "packet_ref"),
                    "freshness_expectation": ensure_str(
                        link.get("freshness_expectation"),
                        "freshness_expectation",
                    ),
                    "minimum_result_status": ensure_str(
                        link.get("minimum_result_status"),
                        "minimum_result_status",
                    ),
                    "link_requirement": ensure_str(
                        link.get("link_requirement"), "link_requirement"
                    ),
                }
            )
    return links


def collect_ref_union(claim_rows: list[dict[str, Any]], field: str) -> list[str]:
    result: list[str] = []
    for row in claim_rows:
        for ref in ensure_list(row.get(field, []), f"{row.get('claim_row_id')}.{field}"):
            sref = ensure_str(ref, f"{row.get('claim_row_id')}.{field}[]")
            if sref not in result:
                result.append(sref)
    return result


def collect_channel_projections(claim_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Build a stable per-channel projection summary from the inherited rows.

    For each of the eleven canonical channels, the strictest binding
    among the inherited rows wins so the manifest row never widens a
    canonical binding.
    """

    channels = [
        "docs_site",
        "migration_notes",
        "help_about",
        "service_health",
        "support_export",
        "release_packet",
        "release_notes",
        "cli_help",
        "evaluation_artifact",
        "marketplace_discovery",
        "public_proof_packet",
    ]
    binding_strictness = {
        "required": 3,
        "must_not_claim": 3,
        "optional": 2,
        "not_applicable": 1,
    }
    projection_strictness = {
        "verbatim_claim": 1,
        "narrowed_summary": 2,
        "status_badge_only": 3,
        "packet_reference_only": 4,
        "known_limit_link_only": 5,
        "repair_only": 6,
    }
    result: list[dict[str, Any]] = []
    for channel in channels:
        winning: dict[str, Any] | None = None
        winning_score = (-1, -1)
        for row in claim_rows:
            bindings = ensure_dict(
                row.get("channel_bindings"),
                f"{row.get('claim_row_id')}.channel_bindings",
            )
            binding = ensure_dict(
                bindings.get(channel),
                f"{row.get('claim_row_id')}.channel_bindings.{channel}",
            )
            status = ensure_str(binding.get("binding_status"), "binding_status")
            projection = ensure_str(binding.get("projection_kind"), "projection_kind")
            score = (
                binding_strictness.get(status, 0),
                projection_strictness.get(projection, 0),
            )
            if score > winning_score:
                winning_score = score
                winning = binding
        if winning is None:
            continue
        result.append(
            {
                "channel_id": channel,
                "binding_status": ensure_str(
                    winning.get("binding_status"), f"{channel}.binding_status"
                ),
                "projection_kind": ensure_str(
                    winning.get("projection_kind"), f"{channel}.projection_kind"
                ),
                "copy_field": ensure_str(
                    winning.get("copy_field"), f"{channel}.copy_field"
                ),
                "surface_ref": ensure_str(
                    winning.get("surface_ref"), f"{channel}.surface_ref"
                ),
            }
        )
    return result


def derive_canonical_claim_state(claim_rows: list[dict[str, Any]]) -> dict[str, Any]:
    """Compose canonical-row-derived posture and downgrade reasons.

    The strictest effective posture among inherited rows wins so the
    manifest row never publishes a wider posture than its canonical
    sources.
    """

    posture_strictness = {
        "claim_bearing": 1,
        "experimental": 2,
        "limited": 3,
        "replacement_grade": 4,
        "policy_disabled": 5,
        "seed_only": 6,
        "withdrawn": 7,
    }
    declared = "claim_bearing"
    effective = "claim_bearing"
    reasons: list[str] = []
    for row in claim_rows:
        row_declared = ensure_str(
            row.get("declared_claim_posture"),
            f"{row.get('claim_row_id')}.declared_claim_posture",
        )
        row_effective = ensure_str(
            row.get("effective_claim_posture"),
            f"{row.get('claim_row_id')}.effective_claim_posture",
        )
        if posture_strictness.get(row_declared, 0) > posture_strictness.get(declared, 0):
            declared = row_declared
        if posture_strictness.get(row_effective, 0) > posture_strictness.get(effective, 0):
            effective = row_effective
        for reason in ensure_list(
            row.get("active_downgrade_reasons", []),
            f"{row.get('claim_row_id')}.active_downgrade_reasons",
        ):
            sreason = ensure_str(reason, "active_downgrade_reason")
            if sreason not in reasons:
                reasons.append(sreason)
    return {
        "declared": declared,
        "effective": effective,
        "active_downgrade_reasons": reasons,
    }


def compose_row(
    repo_root: Path,
    raw_row: dict[str, Any],
    seed_index: dict[str, dict[str, Any]],
    surface_index: dict[str, dict[str, Any]],
    archetype_index: dict[str, dict[str, Any]],
    scorecard_index: dict[str, dict[str, Any]],
    compat_index: dict[str, dict[str, Any]],
    cohort_ids: set[str],
    skew_register_index: dict[str, dict[str, Any]],
    matrix_as_of: str,
    findings: list[Finding],
) -> dict[str, Any] | None:
    row_id = ensure_str(raw_row.get("row_id"), "matrix.row.row_id")
    row_kind = ensure_str(raw_row.get("row_kind"), f"{row_id}.row_kind")
    if row_kind not in VALID_ROW_KINDS:
        findings.append(
            Finding(
                severity="error",
                check_id="row_kind.invalid",
                message=f"{row_id} declares unknown row_kind {row_kind!r}",
                remediation="Use one of " + ", ".join(sorted(VALID_ROW_KINDS)),
                ref=row_id,
            )
        )
        return None

    # Resolve canonical claim rows.
    canonical_refs: list[str] = []
    if row_kind == "canonical_claim_family":
        canonical_refs = [
            ensure_str(raw_row.get("claim_row_ref"), f"{row_id}.claim_row_ref")
        ]
    else:
        canonical_refs = [
            ensure_str(ref, f"{row_id}.inherits_claim_row_refs[]")
            for ref in ensure_list(
                raw_row.get("inherits_claim_row_refs"),
                f"{row_id}.inherits_claim_row_refs",
            )
        ]
    if not canonical_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="row_references_no_canonical_claim",
                message=f"{row_id} does not bind to any canonical claim_row",
                remediation=(
                    "Set claim_row_ref (for canonical_claim_family rows) or "
                    "inherits_claim_row_refs (for beta_surface_binding / "
                    "beta_archetype_binding rows)."
                ),
                ref=row_id,
            )
        )
        return None

    canonical_rows: list[dict[str, Any]] = []
    for cref in canonical_refs:
        seed_row = seed_index.get(cref)
        if seed_row is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_canonical_claim",
                    message=f"{row_id} cites unknown canonical claim_row: {cref}",
                    remediation=(
                        "Use a claim_row_id from "
                        "artifacts/governance/claim_manifest_seed.yaml#claim_rows."
                    ),
                    ref=row_id,
                )
            )
            continue
        canonical_rows.append(seed_row)
    if not canonical_rows:
        return None

    # Resolve cohort refs.
    primary_cohort_refs = [
        ensure_str(ref, f"{row_id}.primary_cohort_refs[]")
        for ref in ensure_list(
            raw_row.get("primary_cohort_refs"),
            f"{row_id}.primary_cohort_refs",
        )
    ]
    for cohort in primary_cohort_refs:
        if cohort not in cohort_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_cohort",
                    message=f"{row_id} cites unknown cohort_id: {cohort}",
                    remediation=(
                        "Use a cohort_id from "
                        "artifacts/milestones/m3/cohort_guardrails.yaml#cohorts."
                    ),
                    ref=row_id,
                )
            )

    # Resolve compat refs.
    declared_compat_refs = [
        ensure_str(ref, f"{row_id}.compat_row_refs[]")
        for ref in ensure_list(
            raw_row.get("compat_row_refs", []),
            f"{row_id}.compat_row_refs",
        )
    ]
    for cref in declared_compat_refs:
        if cref not in compat_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_compat_row",
                    message=f"{row_id} cites unknown compat_row: {cref}",
                    remediation=(
                        "Use a row_id present in "
                        "artifacts/compat/m3/compatibility_report.json."
                    ),
                    ref=row_id,
                )
            )

    # Resolve beta surface / archetype refs.
    beta_surface_ref: str | None = None
    beta_archetype_ref: str | None = None
    archetype_row_ref: str | None = None
    scorecard_ref: str | None = None
    if row_kind == "beta_surface_binding":
        beta_surface_ref = ensure_str(
            raw_row.get("beta_surface_ref"), f"{row_id}.beta_surface_ref"
        )
        if beta_surface_ref not in surface_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_beta_surface",
                    message=(
                        f"{row_id} cites unknown beta_surface_id: {beta_surface_ref}"
                    ),
                    remediation=(
                        "Use a surface_id from "
                        "artifacts/milestones/m3/claimed_surface_register.json."
                    ),
                    ref=row_id,
                )
            )
    if row_kind == "beta_archetype_binding":
        beta_archetype_ref = ensure_str(
            raw_row.get("beta_archetype_ref"), f"{row_id}.beta_archetype_ref"
        )
        archetype_row_ref = ensure_str(
            raw_row.get("archetype_row_ref"), f"{row_id}.archetype_row_ref"
        )
        scorecard_ref = ensure_str(
            raw_row.get("scorecard_ref"), f"{row_id}.scorecard_ref"
        )
        if beta_archetype_ref not in archetype_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_beta_archetype",
                    message=(
                        f"{row_id} cites unknown beta_archetype_id: {beta_archetype_ref}"
                    ),
                    remediation=(
                        "Use an archetype_surface_id from "
                        "artifacts/milestones/m3/claimed_surface_register.json."
                    ),
                    ref=row_id,
                )
            )
        else:
            arch = archetype_index[beta_archetype_ref]
            arch_row = ensure_str(
                arch.get("archetype_row_ref"), f"{beta_archetype_ref}.archetype_row_ref"
            )
            if arch_row != archetype_row_ref:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="row_archetype_row_ref_mismatch",
                        message=(
                            f"{row_id} archetype_row_ref {archetype_row_ref!r} "
                            f"does not match register {arch_row!r}"
                        ),
                        remediation=(
                            "Use the archetype_row_ref from the matching "
                            "claimed_surface_register row."
                        ),
                        ref=row_id,
                    )
                )
        if scorecard_ref not in scorecard_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_scorecard",
                    message=f"{row_id} cites unknown scorecard_id: {scorecard_ref}",
                    remediation=(
                        "Use a scorecard_id from "
                        "artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json."
                    ),
                    ref=row_id,
                )
            )

    # Compose canonical refs and posture.
    posture = derive_canonical_claim_state(canonical_rows)
    requirement_ids = collect_requirement_ids(canonical_rows)
    compat_union = collect_ref_union(canonical_rows, "compatibility_row_refs")
    for declared in declared_compat_refs:
        if declared not in compat_union:
            compat_union.append(declared)
    skew_union = collect_ref_union(canonical_rows, "version_skew_register_refs")
    for skew_ref in skew_union:
        if skew_ref not in skew_register_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_skew_register",
                    message=f"{row_id} cites unknown skew_register_id: {skew_ref}",
                    remediation=(
                        "Use a register_id from "
                        "artifacts/compat/version_skew_register.yaml#register."
                    ),
                    ref=row_id,
                )
            )

    known_limit_refs = collect_ref_union(canonical_rows, "known_limit_refs")
    exclusion_note_refs = collect_ref_union(canonical_rows, "exclusion_note_refs")
    evidence_links = collect_evidence_links(canonical_rows)
    channel_projections = collect_channel_projections(canonical_rows)

    # Support / lifecycle derivation.
    declared_support = "supported"
    effective_support = "supported"
    target_at_beta_exit: str | None = None
    target_at_stable: str | None = None
    downgrade_triggers_fired: list[str] = []
    open_waiver_count = 0
    freshness_evidence_date = matrix_as_of
    review_window_days = 21
    freshness_derivation = "current"
    evidence_owner = ensure_str(
        canonical_rows[0]
        .get("lifecycle_support", {})
        .get("support_window_ref", ""),
        "fallback",
    ) if False else "@ahmeddyounis"
    intake_owner = "@ahmeddyounis"
    triage_owner = "@ahmeddyounis"
    release_owner = "@ahmeddyounis"
    escalation_ref = "docs/governance/decision_rights_and_signoff_matrix.md"

    if row_kind == "beta_surface_binding":
        surface = surface_index.get(beta_surface_ref or "", {})
        declared_support = ensure_str(
            surface.get("support_class", "supported"),
            f"{beta_surface_ref}.support_class",
        )
        effective_support = declared_support
    elif row_kind == "beta_archetype_binding":
        archetype = archetype_index.get(beta_archetype_ref or "", {})
        declared_support = ensure_str(
            archetype.get("current_support_class", "experimental"),
            f"{beta_archetype_ref}.current_support_class",
        )
        scorecard = scorecard_index.get(scorecard_ref or "", {})
        if scorecard:
            effective_support = ensure_str(
                scorecard.get("effective_support_class", declared_support),
                f"{scorecard_ref}.effective_support_class",
            )
            target_at_beta_exit = ensure_str(
                scorecard.get(
                    "target_support_class_at_beta_exit", "supported"
                ),
                f"{scorecard_ref}.target_support_class_at_beta_exit",
            )
            target_at_stable = ensure_str(
                scorecard.get(
                    "target_support_class_at_stable", "certified"
                ),
                f"{scorecard_ref}.target_support_class_at_stable",
            )
            downgrade_triggers_fired = [
                ensure_str(t, f"{scorecard_ref}.downgrade_triggers_fired[]")
                for t in ensure_list(
                    scorecard.get("downgrade_triggers_fired", []),
                    f"{scorecard_ref}.downgrade_triggers_fired",
                )
            ]
            open_waiver_count = len(
                ensure_list(
                    scorecard.get("open_waivers", []),
                    f"{scorecard_ref}.open_waivers",
                )
            )
            freshness_evidence_date = ensure_str(
                scorecard.get("evidence_date", matrix_as_of),
                f"{scorecard_ref}.evidence_date",
            )
            review_window_days = int(
                scorecard.get("review_window_days", 21)
            )
            freshness_derivation = ensure_str(
                scorecard.get("freshness_derivation", "current"),
                f"{scorecard_ref}.freshness_derivation",
            )
            evidence_owner = ensure_str(
                scorecard.get("evidence_owner", "@ahmeddyounis"),
                f"{scorecard_ref}.evidence_owner",
            )
            handoff = ensure_dict(
                scorecard.get("owner_handoff_path", {}),
                f"{scorecard_ref}.owner_handoff_path",
            )
            intake_owner = ensure_str(
                handoff.get("intake_owner", intake_owner), "intake_owner"
            )
            triage_owner = ensure_str(
                handoff.get("triage_owner", triage_owner), "triage_owner"
            )
            release_owner = ensure_str(
                handoff.get("release_owner", release_owner), "release_owner"
            )
            escalation_ref = ensure_str(
                handoff.get("escalation_ref", escalation_ref), "escalation_ref"
            )
    else:
        # Canonical claim family: inherit support_class from the
        # canonical row's lifecycle_support.support_class label.
        lifecycle_support = ensure_dict(
            canonical_rows[0].get("lifecycle_support", {}),
            f"{canonical_rows[0].get('claim_row_id')}.lifecycle_support",
        )
        # The seed's lifecycle_support.support_class uses a different
        # vocabulary; we publish a manifest-shaped support class
        # derived from claim posture.
        if posture["effective"] == "claim_bearing":
            declared_support = "supported"
            effective_support = "supported"
        elif posture["effective"] == "limited":
            declared_support = "supported"
            effective_support = "limited"
        elif posture["effective"] == "experimental":
            declared_support = "supported"
            effective_support = "experimental"
        elif posture["effective"] == "replacement_grade":
            declared_support = "supported"
            effective_support = "limited"
        elif posture["effective"] == "policy_disabled":
            declared_support = "supported"
            effective_support = "unsupported"
        elif posture["effective"] == "seed_only":
            declared_support = "supported"
            effective_support = "experimental"
        elif posture["effective"] == "withdrawn":
            declared_support = "supported"
            effective_support = "unsupported"

    # Same-change-set freshness gate: marketed rows must read no worse
    # than warm_cached, and the scorecard evidence_date must match the
    # matrix as_of unless the matrix opts in to drift.
    freshness_badge_class = ensure_str(
        raw_row.get("freshness_badge_class"),
        f"{row_id}.freshness_badge_class",
    )
    provenance_label = ensure_str(
        raw_row.get("provenance_label"),
        f"{row_id}.provenance_label",
    )

    # Build the row payload.
    row_payload = {
        "row_id": row_id,
        "row_kind": row_kind,
        "headline": ensure_str(
            canonical_rows[0]
            .get("canonical_copy", {})
            .get("headline"),
            f"{canonical_rows[0].get('claim_row_id')}.canonical_copy.headline",
        ),
        "claim_row_refs": [
            ensure_str(r.get("claim_row_id"), "claim_row_id")
            for r in canonical_rows
        ],
        "claim_family": ensure_str(
            canonical_rows[0].get("claim_family"),
            f"{canonical_rows[0].get('claim_row_id')}.claim_family",
        ),
        "requirement_ids": requirement_ids,
        "claim_posture": posture,
        "support": {
            "declared": declared_support,
            "effective": effective_support,
            "target_at_beta_exit": target_at_beta_exit,
            "target_at_stable": target_at_stable,
            "downgrade_triggers_fired": downgrade_triggers_fired,
            "open_waiver_count": open_waiver_count,
        },
        "lifecycle": {
            "display_lifecycle_label": derive_lifecycle_label(canonical_rows),
        },
        "freshness": {
            "badge_class": freshness_badge_class,
            "evidence_date": freshness_evidence_date,
            "review_window_days": review_window_days,
            "freshness_derivation": freshness_derivation,
        },
        "provenance": {
            "label": provenance_label,
            "evidence_owner": evidence_owner,
            "owner_handoff_path": {
                "intake_owner": intake_owner,
                "triage_owner": triage_owner,
                "release_owner": release_owner,
                "escalation_ref": escalation_ref,
            },
        },
        "primary_cohort_refs": primary_cohort_refs,
        "beta_surface_ref": beta_surface_ref,
        "beta_archetype_ref": beta_archetype_ref,
        "archetype_row_ref": archetype_row_ref,
        "scorecard_ref": scorecard_ref,
        "compatibility_row_refs": compat_union,
        "version_skew_register_refs": skew_union,
        "evidence_links": evidence_links,
        "known_limit_refs": known_limit_refs,
        "exclusion_note_refs": exclusion_note_refs,
        "channel_projections": channel_projections,
        "notes": ensure_str(raw_row.get("notes"), f"{row_id}.notes"),
    }

    # Same-change-set gates --------------------------------------------------
    if freshness_evidence_date != matrix_as_of:
        findings.append(
            Finding(
                severity="error",
                check_id="upstream_as_of_drift",
                message=(
                    f"{row_id} evidence_date {freshness_evidence_date} drifts "
                    f"from matrix as_of {matrix_as_of}"
                ),
                remediation=(
                    "Refresh the scorecard register and re-run the M3 cohort/archetype "
                    "scorecard generator in the same change set as this manifest."
                ),
                ref=row_id,
            )
        )
    if (
        row_kind in {"beta_surface_binding", "beta_archetype_binding"}
        and freshness_badge_class in {"stale", "unverified"}
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="canonical_claim_row_stale_for_marketed_row",
                message=(
                    f"{row_id} is marketed but its freshness badge is "
                    f"{freshness_badge_class}, below the warm_cached floor"
                ),
                remediation=(
                    "Refresh the underlying evidence or downgrade the row's "
                    "effective claim posture in the canonical seed."
                ),
                ref=row_id,
            )
        )

    posture_strictness = {
        "claim_bearing": 1,
        "experimental": 2,
        "limited": 3,
        "replacement_grade": 4,
        "policy_disabled": 5,
        "seed_only": 6,
        "withdrawn": 7,
    }
    canonical_effective_max = max(
        posture_strictness.get(
            ensure_str(
                r.get("effective_claim_posture"),
                f"{r.get('claim_row_id')}.effective_claim_posture",
            ),
            0,
        )
        for r in canonical_rows
    )
    if (
        posture_strictness.get(posture["effective"], 0)
        < canonical_effective_max
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="effective_claim_posture_widens_beyond_canonical",
                message=(
                    f"{row_id} derived effective posture {posture['effective']!r} "
                    "widens beyond the strictest canonical row"
                ),
                remediation=(
                    "The generator computed an effective posture below the "
                    "strictest inherited claim_row. Investigate the canonical "
                    "rows referenced by this row."
                ),
                ref=row_id,
            )
        )

    support_strictness = {
        "certified": 0,
        "supported": 1,
        "limited": 2,
        "experimental": 3,
        "community": 3,
        "retest_pending": 4,
        "evidence_stale": 5,
        "unsupported": 6,
    }
    if row_kind == "beta_archetype_binding" and scorecard_ref in scorecard_index:
        scorecard = scorecard_index[scorecard_ref]
        scorecard_effective = ensure_str(
            scorecard.get("effective_support_class"),
            f"{scorecard_ref}.effective_support_class",
        )
        if (
            support_strictness.get(effective_support, 99)
            < support_strictness.get(scorecard_effective, 99)
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="support_class_widens_above_scorecard_effective",
                    message=(
                        f"{row_id} effective_support {effective_support!r} "
                        f"widens above scorecard {scorecard_effective!r}"
                    ),
                    remediation=(
                        "The manifest must not publish a stronger support "
                        "class than the scorecard register."
                    ),
                    ref=row_id,
                )
            )

    if posture["active_downgrade_reasons"] and not known_limit_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="missing_known_limit_for_active_downgrade",
                message=(
                    f"{row_id} has active_downgrade_reasons but no known_limit_refs"
                ),
                remediation=(
                    "Add a known_limit_ref to the inherited canonical claim_row "
                    "or to the matrix row before publishing."
                ),
                ref=row_id,
            )
        )

    return row_payload


def compose_manifest(
    repo_root: Path,
    matrix: dict[str, Any],
    seed: dict[str, Any],
    surface_register: dict[str, Any],
    scorecard_register: dict[str, Any],
    compat_report: dict[str, Any],
    cohort_guardrails: dict[str, Any],
    skew_register: dict[str, Any],
    generated_at: str,
    findings: list[Finding],
) -> dict[str, Any]:
    matrix_as_of = ensure_str(matrix.get("as_of"), "matrix.as_of")

    # Same-change-set guard on upstream sources.
    for label, payload in (
        ("claimed_surface_register", surface_register),
        ("scorecard_register", scorecard_register),
        ("compatibility_report", compat_report),
    ):
        as_of = payload.get("as_of")
        if as_of is not None and ensure_str(as_of, f"{label}.as_of") != matrix_as_of:
            findings.append(
                Finding(
                    severity="error",
                    check_id="upstream_as_of_drift",
                    message=(
                        f"{label} as_of {as_of} drifts from matrix as_of {matrix_as_of}"
                    ),
                    remediation=(
                        "Re-run the upstream generators in the same change set "
                        "or update the matrix as_of to match the refreshed sources."
                    ),
                    ref=label,
                )
            )

    seed_index = index_seed_claim_rows(seed)
    surface_index = index_claimed_surfaces(surface_register)
    archetype_index = index_claimed_archetypes(surface_register)
    scorecard_index = index_scorecards(scorecard_register)
    compat_index = index_compat_rows(compat_report)
    cohort_ids = index_cohorts(cohort_guardrails)
    skew_register_index = index_skew_register(skew_register)

    rows_out: list[dict[str, Any]] = []
    seen_kinds: dict[str, int] = {}
    for idx, raw_row in enumerate(ensure_list(matrix.get("rows"), "matrix.rows")):
        row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        composed = compose_row(
            repo_root=repo_root,
            raw_row=row,
            seed_index=seed_index,
            surface_index=surface_index,
            archetype_index=archetype_index,
            scorecard_index=scorecard_index,
            compat_index=compat_index,
            cohort_ids=cohort_ids,
            skew_register_index=skew_register_index,
            matrix_as_of=matrix_as_of,
            findings=findings,
        )
        if composed is None:
            continue
        seen_kinds[composed["row_kind"]] = seen_kinds.get(composed["row_kind"], 0) + 1
        rows_out.append(composed)

    # Coverage: every claimed beta surface and archetype must be present.
    covered_surfaces = {
        row["beta_surface_ref"]
        for row in rows_out
        if row["beta_surface_ref"] is not None
    }
    missing_surfaces = set(surface_index) - covered_surfaces
    for missing in sorted(missing_surfaces):
        findings.append(
            Finding(
                severity="error",
                check_id="claimed_beta_surface_missing_from_manifest",
                message=(
                    f"claimed_surface_register surface_id {missing} is not "
                    "covered by the M3 claim manifest"
                ),
                remediation=(
                    "Add a beta_surface_binding row to the matrix that "
                    "inherits at least one canonical claim_row."
                ),
                ref=missing,
            )
        )
    covered_archetypes = {
        row["beta_archetype_ref"]
        for row in rows_out
        if row["beta_archetype_ref"] is not None
    }
    missing_archetypes = set(archetype_index) - covered_archetypes
    for missing in sorted(missing_archetypes):
        findings.append(
            Finding(
                severity="error",
                check_id="claimed_beta_archetype_missing_from_manifest",
                message=(
                    f"claimed_surface_register archetype_surface_id {missing} "
                    "is not covered by the M3 claim manifest"
                ),
                remediation=(
                    "Add a beta_archetype_binding row to the matrix that "
                    "inherits at least one canonical claim_row."
                ),
                ref=missing,
            )
        )

    consuming_surfaces = [
        ensure_str(ref, f"matrix.consuming_surfaces[{i}]")
        for i, ref in enumerate(
            ensure_list(matrix.get("consuming_surfaces"), "matrix.consuming_surfaces")
        )
    ]
    for ref in consuming_surfaces:
        if not (repo_root / ref).exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="consuming_surface_missing",
                    message=f"consuming surface does not resolve: {ref}",
                    remediation=(
                        "Fix the path or seed the consuming surface before "
                        "publishing the manifest."
                    ),
                    ref=ref,
                )
            )

    backup_owner = matrix.get("backup_owner")
    backup_waiver = matrix.get("backup_waiver")
    if backup_owner is not None:
        backup_owner = ensure_str(backup_owner, "matrix.backup_owner")
    if backup_waiver is not None:
        backup_waiver = ensure_str(backup_waiver, "matrix.backup_waiver")
    if backup_owner is None and not backup_waiver:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.backup_waiver.required",
                message=(
                    "matrix backup_owner is null but backup_waiver is missing"
                ),
                remediation=(
                    "Set backup_owner to a handle or attach a backup_waiver id."
                ),
            )
        )

    vocabularies = ensure_dict(matrix.get("vocabularies"), "matrix.vocabularies")
    vocab_payload = {
        "freshness_badge_class": [
            ensure_str(v, "freshness_badge_class[]")
            for v in ensure_list(
                vocabularies.get("freshness_badge_class"),
                "matrix.vocabularies.freshness_badge_class",
            )
        ],
        "provenance_label": [
            ensure_str(v, "provenance_label[]")
            for v in ensure_list(
                vocabularies.get("provenance_label"),
                "matrix.vocabularies.provenance_label",
            )
        ],
        "claim_posture": [
            ensure_str(v, "claim_posture[]")
            for v in ensure_list(
                vocabularies.get("claim_posture"),
                "matrix.vocabularies.claim_posture",
            )
        ],
        "support_class": [
            ensure_str(v, "support_class[]")
            for v in ensure_list(
                vocabularies.get("support_class"),
                "matrix.vocabularies.support_class",
            )
        ],
        "lifecycle_label": [
            ensure_str(v, "lifecycle_label[]")
            for v in ensure_list(
                vocabularies.get("lifecycle_label"),
                "matrix.vocabularies.lifecycle_label",
            )
        ],
        "row_kind": sorted(VALID_ROW_KINDS),
    }

    return {
        "schema_version": 1,
        "record_kind": "m3_claim_manifest",
        "manifest_id": ensure_str(matrix.get("manifest_id"), "matrix.manifest_id"),
        "manifest_revision": int(matrix.get("manifest_revision", 1)),
        "milestone_id": ensure_str(matrix.get("milestone_id"), "matrix.milestone_id"),
        "release_channel_scope": ensure_str(
            matrix.get("release_channel_scope"), "matrix.release_channel_scope"
        ),
        "manifest_state": ensure_str(
            matrix.get("manifest_state"), "matrix.manifest_state"
        ),
        "as_of": matrix_as_of,
        "generated_at": generated_at,
        "owner": ensure_str(matrix.get("owner"), "matrix.owner"),
        "backup_owner": backup_owner,
        "backup_waiver": backup_waiver,
        "source_refs": {
            "claim_manifest_seed": ensure_str(
                matrix.get("claim_manifest_seed_source"),
                "matrix.claim_manifest_seed_source",
            ),
            "claimed_surface_register": ensure_str(
                matrix.get("claimed_surface_register_source"),
                "matrix.claimed_surface_register_source",
            ),
            "scorecard_register": ensure_str(
                matrix.get("scorecard_register_source"),
                "matrix.scorecard_register_source",
            ),
            "compatibility_report": ensure_str(
                matrix.get("compatibility_report_source"),
                "matrix.compatibility_report_source",
            ),
            "cohort_guardrails": ensure_str(
                matrix.get("cohort_guardrails_source"),
                "matrix.cohort_guardrails_source",
            ),
            "version_skew_register": ensure_str(
                matrix.get("version_skew_register_source"),
                "matrix.version_skew_register_source",
            ),
        },
        "vocabularies": vocab_payload,
        "consuming_surfaces": consuming_surfaces,
        "rows": rows_out,
        "notes": (
            "Generated by ci/check_m3_claim_manifest.py from the M3 claim-"
            "manifest matrix and the canonical upstream seeds. Do not edit "
            "by hand; refresh the matrix or a seed and re-run the generator."
        ),
    }


def render_markdown(manifest: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("# M3 governed claim manifest")
    lines.append("")
    lines.append(
        "This file is generated by `ci/check_m3_claim_manifest.py` from the "
        "M3 claim-manifest matrix (`artifacts/release/m3/claim_manifest_matrix.yaml`) "
        "and the canonical upstream seeds. Do not hand-edit; update the matrix "
        "or the canonical seed and re-run the generator."
    )
    lines.append("")
    lines.append("## Manifest metadata")
    lines.append("")
    lines.append(f"- **Manifest id:** `{manifest['manifest_id']}`")
    lines.append(f"- **Manifest revision:** `{manifest['manifest_revision']}`")
    lines.append(f"- **Milestone:** `{manifest['milestone_id']}`")
    lines.append(f"- **Manifest state:** `{manifest['manifest_state']}`")
    lines.append(
        f"- **Release channel scope:** `{manifest['release_channel_scope']}`"
    )
    lines.append(f"- **As of:** `{manifest['as_of']}`")
    lines.append(f"- **Generated at:** `{manifest['generated_at']}`")
    lines.append(f"- **Owner:** {manifest['owner']}")
    backup_owner = manifest["backup_owner"] or "_(none, see backup waiver)_"
    lines.append(f"- **Backup owner:** {backup_owner}")
    if manifest["backup_waiver"]:
        lines.append(f"- **Backup waiver:** `{manifest['backup_waiver']}`")
    lines.append("")

    lines.append("## Source seeds")
    lines.append("")
    for label, ref in manifest["source_refs"].items():
        lines.append(f"- `{label}`: `{ref}`")
    lines.append("")

    lines.append("## Consuming surfaces")
    lines.append("")
    for ref in manifest["consuming_surfaces"]:
        lines.append(f"- `{ref}`")
    lines.append("")

    lines.append("## Claim row summary")
    lines.append("")
    lines.append(
        "| Row | Kind | Family | Effective posture | Support (declared / effective) | "
        "Freshness | Provenance |"
    )
    lines.append("|---|---|---|---|---|---|---|")
    for row in manifest["rows"]:
        support = (
            f"{row['support']['declared']} / {row['support']['effective']}"
        )
        lines.append(
            f"| `{row['row_id']}` | {row['row_kind']} | {row['claim_family']} | "
            f"{row['claim_posture']['effective']} | {support} | "
            f"{row['freshness']['badge_class']} | {row['provenance']['label']} |"
        )
    lines.append("")

    lines.append("## Per-row detail")
    lines.append("")
    for row in manifest["rows"]:
        lines.append(f"### `{row['row_id']}`")
        lines.append("")
        lines.append(f"- **Row kind:** `{row['row_kind']}`")
        lines.append(f"- **Headline:** {row['headline']}")
        lines.append(f"- **Claim family:** `{row['claim_family']}`")
        lines.append(
            "- **Canonical claim rows:** "
            + ", ".join(f"`{r}`" for r in row["claim_row_refs"])
        )
        if row["beta_surface_ref"]:
            lines.append(f"- **Beta surface:** `{row['beta_surface_ref']}`")
        if row["beta_archetype_ref"]:
            lines.append(
                f"- **Beta archetype:** `{row['beta_archetype_ref']}` "
                f"(archetype row `{row['archetype_row_ref']}`, "
                f"scorecard `{row['scorecard_ref']}`)"
            )
        cp = row["claim_posture"]
        lines.append(
            f"- **Claim posture:** declared `{cp['declared']}`, "
            f"effective `{cp['effective']}`"
        )
        if cp["active_downgrade_reasons"]:
            lines.append(
                "- **Active downgrade reasons:** "
                + ", ".join(f"`{r}`" for r in cp["active_downgrade_reasons"])
            )
        sup = row["support"]
        lines.append(
            f"- **Support class:** declared `{sup['declared']}`, "
            f"effective `{sup['effective']}`"
        )
        if sup["target_at_beta_exit"]:
            lines.append(
                "- **Support target:** "
                f"beta_exit `{sup['target_at_beta_exit']}` / "
                f"stable `{sup['target_at_stable']}`"
            )
        if sup["downgrade_triggers_fired"]:
            lines.append(
                "- **Downgrade triggers fired:** "
                + ", ".join(f"`{t}`" for t in sup["downgrade_triggers_fired"])
            )
        if sup["open_waiver_count"]:
            lines.append(
                f"- **Open waivers:** {sup['open_waiver_count']}"
            )
        lc = row["lifecycle"]
        lines.append(
            f"- **Lifecycle label:** `{lc['display_lifecycle_label']}`"
        )
        fr = row["freshness"]
        lines.append(
            f"- **Freshness badge:** `{fr['badge_class']}` "
            f"(evidence_date `{fr['evidence_date']}`, "
            f"review_window_days `{fr['review_window_days']}`, "
            f"derivation `{fr['freshness_derivation']}`)"
        )
        prov = row["provenance"]
        lines.append(
            f"- **Provenance label:** `{prov['label']}` "
            f"(evidence_owner {prov['evidence_owner']})"
        )
        handoff = prov["owner_handoff_path"]
        lines.append(
            "  - handoff: intake "
            f"{handoff['intake_owner']}, triage {handoff['triage_owner']}, "
            f"release {handoff['release_owner']}, "
            f"escalation `{handoff['escalation_ref']}`"
        )
        if row["primary_cohort_refs"]:
            lines.append(
                "- **Primary cohorts:** "
                + ", ".join(f"`{c}`" for c in row["primary_cohort_refs"])
            )
        if row["requirement_ids"]:
            lines.append(
                "- **Requirement ids:** "
                + ", ".join(f"`{rid}`" for rid in row["requirement_ids"])
            )
        if row["compatibility_row_refs"]:
            lines.append(
                "- **Compatibility rows:** "
                + ", ".join(f"`{r}`" for r in row["compatibility_row_refs"])
            )
        if row["version_skew_register_refs"]:
            lines.append(
                "- **Version-skew registers:** "
                + ", ".join(
                    f"`{r}`" for r in row["version_skew_register_refs"]
                )
            )
        if row["evidence_links"]:
            lines.append("- **Evidence links:**")
            for link in row["evidence_links"]:
                lines.append(
                    f"  - `{link['claim_evidence_id']}` "
                    f"({link['evidence_kind']}, "
                    f"freshness {link['freshness_expectation']}, "
                    f"min status {link['minimum_result_status']}, "
                    f"{link['link_requirement']})"
                )
        if row["known_limit_refs"]:
            lines.append("- **Known limits:**")
            for ref in row["known_limit_refs"]:
                lines.append(f"  - `{ref}`")
        if row["channel_projections"]:
            lines.append("- **Channel projections:**")
            for proj in row["channel_projections"]:
                lines.append(
                    f"  - `{proj['channel_id']}`: "
                    f"{proj['binding_status']} / {proj['projection_kind']} / "
                    f"`{proj['copy_field']}` -> `{proj['surface_ref']}`"
                )
        lines.append(f"- **Notes:** {row['notes']}")
        lines.append("")

    lines.append("## How to refresh")
    lines.append("")
    lines.append(
        "Run the generator to re-derive the manifest and refresh the "
        "validation capture in the same change set:"
    )
    lines.append("")
    lines.append("```")
    lines.append("python3 ci/check_m3_claim_manifest.py --repo-root .")
    lines.append("```")
    lines.append("")
    lines.append(
        "Use `--check` in CI to fail when the on-disk manifest or capture "
        "would drift from the matrix or upstream seeds."
    )
    lines.append("")
    return "\n".join(lines)


def write_capture(
    path: Path,
    findings: list[Finding],
    matrix_rel: str,
    manifest_json_rel: str,
    manifest_md_rel: str,
    generated_at: str,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "generated_at": generated_at,
        "matrix_ref": matrix_rel,
        "manifest_json_ref": manifest_json_rel,
        "manifest_md_ref": manifest_md_rel,
        "summary": {
            "errors": sum(1 for f in findings if f.severity == "error"),
            "warnings": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


_GENERATED_AT_RE = re.compile(
    r'"generated_at":\s*"[^"]*"|\*\*Generated at:\*\*\s*`[^`]*`'
)


def _normalize(text: str) -> str:
    return _GENERATED_AT_RE.sub("__generated_at__", text)


def write_if_changed(path: Path, content: str, check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    existing: str | None = None
    if path.exists():
        existing = path.read_text(encoding="utf-8")
    changed = existing is None or _normalize(existing) != _normalize(content)
    if not check_only:
        path.write_text(content, encoding="utf-8")
    return changed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    matrix_path = repo_root / args.matrix
    manifest_json_path = repo_root / args.manifest_json
    manifest_md_path = repo_root / args.manifest_md
    capture_path = repo_root / args.capture
    schema_path = repo_root / args.schema

    if not schema_path.exists():
        raise SystemExit(f"missing schema file: {schema_path}")

    matrix = ensure_dict(render_yaml_as_json(matrix_path), "matrix")

    seed = ensure_dict(
        render_yaml_as_json(repo_root / matrix["claim_manifest_seed_source"]),
        "claim_manifest_seed",
    )
    surface_register = ensure_dict(
        load_json(repo_root / matrix["claimed_surface_register_source"]),
        "claimed_surface_register",
    )
    scorecard_register = ensure_dict(
        load_json(repo_root / matrix["scorecard_register_source"]),
        "scorecard_register",
    )
    compat_report = ensure_dict(
        load_json(repo_root / matrix["compatibility_report_source"]),
        "compatibility_report",
    )
    cohort_guardrails = ensure_dict(
        render_yaml_as_json(repo_root / matrix["cohort_guardrails_source"]),
        "cohort_guardrails",
    )
    skew_register = ensure_dict(
        render_yaml_as_json(repo_root / matrix["version_skew_register_source"]),
        "version_skew_register",
    )

    findings: list[Finding] = []
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    manifest = compose_manifest(
        repo_root=repo_root,
        matrix=matrix,
        seed=seed,
        surface_register=surface_register,
        scorecard_register=scorecard_register,
        compat_report=compat_report,
        cohort_guardrails=cohort_guardrails,
        skew_register=skew_register,
        generated_at=generated_at,
        findings=findings,
    )

    manifest_json_text = json.dumps(manifest, indent=2, sort_keys=True) + "\n"
    manifest_md_text = render_markdown(manifest)

    json_changed = write_if_changed(
        manifest_json_path, manifest_json_text, args.check
    )
    md_changed = write_if_changed(
        manifest_md_path, manifest_md_text, args.check
    )

    if args.check and (json_changed or md_changed):
        findings.append(
            Finding(
                severity="error",
                check_id="manifest.stale",
                message=(
                    "checked-in claim manifest is stale relative to the "
                    "matrix or upstream seeds"
                ),
                remediation=(
                    "Run `python3 ci/check_m3_claim_manifest.py --repo-root .` "
                    "and commit the regenerated artifacts."
                ),
                details={
                    "manifest_json_changed": json_changed,
                    "manifest_md_changed": md_changed,
                },
            )
        )

    write_capture(
        capture_path,
        findings,
        args.matrix,
        args.manifest_json,
        args.manifest_md,
        generated_at,
    )

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(
                f"ERROR [{item.check_id}]{ref}: {item.message}",
                file=sys.stderr,
            )
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    print("M3 claim manifest generated and validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
