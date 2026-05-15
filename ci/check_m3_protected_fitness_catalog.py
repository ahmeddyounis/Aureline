#!/usr/bin/env python3
"""Generate and validate the M3 protected fitness-function dashboard.

The script reads the M3 protected fitness-function catalog and the M3
waiver register together with the canonical upstream seeds (canonical
fitness-function catalog, fitness-state vocabulary, claimed-surface
register, cohort guardrails) and writes:

  - artifacts/benchmarks/m3/dashboard_snapshot.json -- machine-readable
    dashboard snapshot quoted by release packets, support exports, and
    the public protected-fitness review surface; and
  - artifacts/benchmarks/m3/captures/protected_fitness_catalog_validation_capture.json
    -- checked-in validation capture for downstream reviewers.

The snapshot is generated from the catalog plus the waiver register so
docs, support exports, release packets, and partner packets all read
one machine-derived dashboard rather than restating tile prose.
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


DEFAULT_CATALOG_REL = "artifacts/benchmarks/m3/protected_fitness_catalog.yaml"
DEFAULT_SNAPSHOT_REL = "artifacts/benchmarks/m3/dashboard_snapshot.json"
DEFAULT_CAPTURE_REL = (
    "artifacts/benchmarks/m3/captures/"
    "protected_fitness_catalog_validation_capture.json"
)


REQUIRED_PROTECTED_LANES = {
    "startup",
    "typing",
    "search",
    "rollback",
    "supportability",
    "extension_isolation",
    "policy_trust",
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
    parser.add_argument("--catalog", default=DEFAULT_CATALOG_REL)
    parser.add_argument("--snapshot", default=DEFAULT_SNAPSHOT_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help=(
            "Fail if the on-disk snapshot or capture would change after "
            "regeneration. Use this in CI to keep the checked-in artifacts "
            "fresh against the catalog and waiver register."
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


def ensure_optional_str(value: Any, label: str) -> str | None:
    if value is None:
        return None
    return ensure_str(value, label)


def index_canonical_rows(
    catalog: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    rows = ensure_list(catalog.get("rows"), "canonical_catalog.rows")
    return {
        ensure_str(r.get("id"), "canonical_catalog.row.id"): r for r in rows
    }


def index_claimed_surfaces(
    register: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    rows = ensure_list(
        register.get("claimed_surfaces"),
        "claimed_surface_register.claimed_surfaces",
    )
    return {
        ensure_str(r.get("surface_id"), "claimed_surfaces.surface_id"): r
        for r in rows
    }


def index_cohort_ids(guardrails: dict[str, Any]) -> set[str]:
    cohorts = ensure_list(guardrails.get("cohorts"), "cohort_guardrails.cohorts")
    return {
        ensure_str(c.get("cohort_id"), "cohort_guardrails.cohorts[].cohort_id")
        for c in cohorts
    }


def index_waiver_rows(
    register: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    rows = ensure_list(register.get("rows"), "waiver_register.rows")
    return {
        ensure_str(
            r.get("protected_function_ref"),
            "waiver_register.row.protected_function_ref",
        ): r
        for r in rows
    }


def collect_state_vocabulary_ids(
    state_rows: dict[str, Any],
    key: str,
) -> set[str]:
    rows = ensure_list(state_rows.get(key), f"fitness_state_rows.{key}")
    return {ensure_str(r.get("id"), f"fitness_state_rows.{key}[].id") for r in rows}


def compose_tile(
    row: dict[str, Any],
    waiver_row: dict[str, Any] | None,
    canonical_index: dict[str, dict[str, Any]],
    state_vocab: dict[str, set[str]],
    surface_index: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    row_id = ensure_str(row.get("id"), "catalog.row.id")
    protected_lane_class = ensure_str(
        row.get("protected_lane_class"), f"{row_id}.protected_lane_class"
    )
    row_status = ensure_str(row.get("row_status"), f"{row_id}.row_status")
    canonical_row_refs = [
        ensure_str(r, f"{row_id}.canonical_row_refs[]")
        for r in ensure_list(
            row.get("canonical_row_refs", []),
            f"{row_id}.canonical_row_refs",
        )
    ]
    primary_canonical_row_ref = ensure_optional_str(
        row.get("primary_canonical_row_ref"),
        f"{row_id}.primary_canonical_row_ref",
    )
    beta_surface_refs = [
        ensure_str(r, f"{row_id}.beta_surface_refs[]")
        for r in ensure_list(
            row.get("beta_surface_refs", []),
            f"{row_id}.beta_surface_refs",
        )
    ]

    # Resolve canonical rows.
    for cref in canonical_row_refs:
        if cref not in canonical_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_canonical_row",
                    message=(
                        f"{row_id} cites unknown canonical fitness function: "
                        f"{cref}"
                    ),
                    remediation=(
                        "Use an id from "
                        "artifacts/bench/fitness_function_catalog.yaml#rows."
                    ),
                    ref=row_id,
                )
            )
    if (
        primary_canonical_row_ref is not None
        and primary_canonical_row_ref not in canonical_index
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="row_references_unknown_primary_canonical_row",
                message=(
                    f"{row_id} primary_canonical_row_ref unknown: "
                    f"{primary_canonical_row_ref}"
                ),
                remediation=(
                    "Use an id from "
                    "artifacts/bench/fitness_function_catalog.yaml#rows."
                ),
                ref=row_id,
            )
        )

    # Provisional rows must name a completion owner when no canonical row
    # is bound; seeded rows must bind to a canonical row.
    completion_owner = ensure_optional_str(
        row.get("completion_owner"), f"{row_id}.completion_owner"
    )
    if row_status == "seeded" and primary_canonical_row_ref is None:
        findings.append(
            Finding(
                severity="error",
                check_id="seeded_row_missing_primary_canonical_row_ref",
                message=(
                    f"{row_id} is row_status=seeded but has no "
                    "primary_canonical_row_ref"
                ),
                remediation=(
                    "Bind the row to a canonical fitness-function row or "
                    "downgrade row_status to provisional with a "
                    "completion_owner."
                ),
                ref=row_id,
            )
        )
    if row_status == "provisional" and completion_owner is None:
        findings.append(
            Finding(
                severity="error",
                check_id="provisional_row_missing_completion_owner",
                message=(
                    f"{row_id} is row_status=provisional but has no "
                    "completion_owner"
                ),
                remediation=(
                    "Set completion_owner to the handle responsible for "
                    "moving the row to seeded."
                ),
                ref=row_id,
            )
        )

    # Resolve beta surface refs.
    for sref in beta_surface_refs:
        if sref not in surface_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="row_references_unknown_beta_surface",
                    message=(
                        f"{row_id} cites unknown beta_surface_id: {sref}"
                    ),
                    remediation=(
                        "Use a surface_id from "
                        "artifacts/milestones/m3/claimed_surface_register.json."
                    ),
                    ref=row_id,
                )
            )

    threshold = ensure_dict(row.get("threshold"), f"{row_id}.threshold")
    waiver = ensure_dict(row.get("waiver"), f"{row_id}.waiver")
    tile = ensure_dict(row.get("dashboard_tile"), f"{row_id}.dashboard_tile")

    # State vocabulary checks.
    for field_name, vocab_key in (
        ("tile_state", "tile_states"),
        ("evidence_freshness_class", "evidence_freshness_classes"),
        ("mitigation_note_class", "mitigation_note_classes"),
        ("partial_profile_result_class", "partial_profile_result_classes"),
        ("corpus_profile_identity_class", "corpus_profile_identity_classes"),
    ):
        value = ensure_str(tile.get(field_name), f"{row_id}.{field_name}")
        if value not in state_vocab[vocab_key]:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"tile.{field_name}.unknown",
                    message=(
                        f"{row_id} tile.{field_name}={value!r} is not "
                        f"present in fitness_state_rows.{vocab_key}"
                    ),
                    remediation=(
                        "Use one of the closed values from "
                        "artifacts/governance/fitness_state_rows.yaml."
                    ),
                    ref=row_id,
                )
            )

    waiver_authority = ensure_str(
        waiver.get("authority"), f"{row_id}.waiver.authority"
    )
    if waiver_authority not in state_vocab["waiver_authority_classes"]:
        findings.append(
            Finding(
                severity="error",
                check_id="waiver.authority.unknown",
                message=(
                    f"{row_id} waiver.authority={waiver_authority!r} is not "
                    "in fitness_state_rows.waiver_authority_classes"
                ),
                remediation=(
                    "Use one of the closed values from "
                    "artifacts/governance/fitness_state_rows.yaml."
                ),
                ref=row_id,
            )
        )

    # Waiver register cross-check.
    waiver_block: dict[str, Any] = {
        "register_row_ref": None,
        "active_waiver_ref": None,
        "previous_waiver_ref": None,
        "register_status_class": None,
        "expiry_proximity_class": None,
        "expires_at": None,
        "current_evidence_gap_class": None,
        "renewal_requested_state_class": None,
        "repeated_path_grouping_class": None,
        "claim_scope_narrowing_class": None,
        "headline_label": None,
        "register_summary": None,
    }
    if waiver_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="waiver_register_row_missing",
                message=(
                    f"{row_id} has no row in "
                    "artifacts/milestones/m3/waiver_register.yaml"
                ),
                remediation=(
                    "Add a waiver register row whose protected_function_ref "
                    f"= {row_id} (set active_waiver_ref=null and "
                    "register_status_class=register_active_within_expiry "
                    "if no waiver is minted)."
                ),
                ref=row_id,
            )
        )
    else:
        waiver_block = {
            "register_row_ref": ensure_str(
                waiver_row.get("row_id"), "waiver_register.row.row_id"
            ),
            "active_waiver_ref": ensure_optional_str(
                waiver_row.get("active_waiver_ref"),
                "waiver_register.row.active_waiver_ref",
            ),
            "previous_waiver_ref": ensure_optional_str(
                waiver_row.get("previous_waiver_ref"),
                "waiver_register.row.previous_waiver_ref",
            ),
            "register_status_class": ensure_str(
                waiver_row.get("register_status_class"),
                "waiver_register.row.register_status_class",
            ),
            "expiry_proximity_class": ensure_str(
                waiver_row.get("expiry_proximity_class"),
                "waiver_register.row.expiry_proximity_class",
            ),
            "expires_at": ensure_optional_str(
                waiver_row.get("expires_at"), "waiver_register.row.expires_at"
            ),
            "current_evidence_gap_class": ensure_str(
                waiver_row.get("current_evidence_gap_class"),
                "waiver_register.row.current_evidence_gap_class",
            ),
            "renewal_requested_state_class": ensure_str(
                waiver_row.get("renewal_requested_state_class"),
                "waiver_register.row.renewal_requested_state_class",
            ),
            "repeated_path_grouping_class": ensure_str(
                waiver_row.get("repeated_path_grouping_class"),
                "waiver_register.row.repeated_path_grouping_class",
            ),
            "claim_scope_narrowing_class": ensure_str(
                waiver_row.get("claim_scope_narrowing_class"),
                "waiver_register.row.claim_scope_narrowing_class",
            ),
            "headline_label": ensure_str(
                waiver_row.get("headline_label"),
                "waiver_register.row.headline_label",
            ),
            "register_summary": ensure_str(
                waiver_row.get("register_summary"),
                "waiver_register.row.register_summary",
            ),
        }

    tile_state = ensure_str(tile.get("tile_state"), f"{row_id}.tile_state")
    freshness_class = ensure_str(
        tile.get("evidence_freshness_class"),
        f"{row_id}.evidence_freshness_class",
    )

    # Same-change-set gates: a passing tile cannot coexist with stale
    # evidence or with an active waiver-held release; a waived tile
    # requires a non-null active_waiver_ref.
    if tile_state == "passing" and freshness_class in {
        "stale_by_time",
        "stale_by_trigger",
        "missing",
    }:
        findings.append(
            Finding(
                severity="error",
                check_id="passing_tile_with_stale_evidence",
                message=(
                    f"{row_id} renders as passing with freshness "
                    f"{freshness_class}; stale evidence cannot project a "
                    "passing tile"
                ),
                remediation=(
                    "Refresh the evidence packet or move the tile to "
                    "evidence_stale per fitness_state_rows.yaml."
                ),
                ref=row_id,
            )
        )
    if tile_state == "waived" and waiver_block["active_waiver_ref"] is None:
        findings.append(
            Finding(
                severity="error",
                check_id="waived_tile_without_active_waiver",
                message=(
                    f"{row_id} renders as waived but has no "
                    "active_waiver_ref"
                ),
                remediation=(
                    "Attach an active_waiver_ref or move the tile to "
                    "blocked / evidence_stale."
                ),
                ref=row_id,
            )
        )
    if (
        tile_state == "waiver_expired"
        and waiver_block["previous_waiver_ref"] is None
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="waiver_expired_tile_without_previous_waiver",
                message=(
                    f"{row_id} renders as waiver_expired but has no "
                    "previous_waiver_ref"
                ),
                remediation=(
                    "Attach the previous_waiver_ref or move the tile to "
                    "blocked / evidence_stale."
                ),
                ref=row_id,
            )
        )

    return {
        "row_id": row_id,
        "protected_lane_class": protected_lane_class,
        "row_status": row_status,
        "title": ensure_str(row.get("title"), f"{row_id}.title"),
        "canonical_row_refs": canonical_row_refs,
        "primary_canonical_row_ref": primary_canonical_row_ref,
        "completion_owner": completion_owner,
        "completion_target": ensure_optional_str(
            row.get("completion_target"), f"{row_id}.completion_target"
        ),
        "beta_surface_refs": beta_surface_refs,
        "threshold": {
            "owner": ensure_str(threshold.get("owner"), f"{row_id}.threshold.owner"),
            "owning_lane": ensure_str(
                threshold.get("owning_lane"),
                f"{row_id}.threshold.owning_lane",
            ),
            "co_owning_lane": ensure_optional_str(
                threshold.get("co_owning_lane"),
                f"{row_id}.threshold.co_owning_lane",
            ),
            "backup_owner": ensure_optional_str(
                threshold.get("backup_owner"),
                f"{row_id}.threshold.backup_owner",
            ),
            "backup_waiver": ensure_optional_str(
                threshold.get("backup_waiver"),
                f"{row_id}.threshold.backup_waiver",
            ),
            "threshold_mode": ensure_str(
                threshold.get("threshold_mode"),
                f"{row_id}.threshold.threshold_mode",
            ),
            "decision_forum_ref": ensure_str(
                threshold.get("decision_forum_ref"),
                f"{row_id}.threshold.decision_forum_ref",
            ),
        },
        "waiver_policy": {
            "authority": waiver_authority,
            "default_expiry_window_days": int(
                waiver.get("default_expiry_window_days", 0)
            ),
            "renewal_requires_correction_program": bool(
                waiver.get("renewal_requires_correction_program", False)
            ),
            "escalation_path": [
                ensure_str(s, f"{row_id}.waiver.escalation_path[]")
                for s in ensure_list(
                    waiver.get("escalation_path", []),
                    f"{row_id}.waiver.escalation_path",
                )
            ],
        },
        "waiver_register_projection": waiver_block,
        "tile": {
            "tile_id": ensure_str(tile.get("tile_id"), f"{row_id}.tile_id"),
            "tile_state": tile_state,
            "evidence_freshness_class": freshness_class,
            "mitigation_note_class": ensure_str(
                tile.get("mitigation_note_class"),
                f"{row_id}.mitigation_note_class",
            ),
            "partial_profile_result_class": ensure_str(
                tile.get("partial_profile_result_class"),
                f"{row_id}.partial_profile_result_class",
            ),
            "corpus_profile_identity_class": ensure_str(
                tile.get("corpus_profile_identity_class"),
                f"{row_id}.corpus_profile_identity_class",
            ),
            "headline_label": ensure_str(
                tile.get("headline_label"),
                f"{row_id}.tile.headline_label",
            ),
            "threshold_label": ensure_str(
                tile.get("threshold_label"),
                f"{row_id}.tile.threshold_label",
            ),
            "measured_label": ensure_str(
                tile.get("measured_label"),
                f"{row_id}.tile.measured_label",
            ),
            "mitigation_summary": ensure_str(
                tile.get("mitigation_summary"),
                f"{row_id}.tile.mitigation_summary",
            ),
            "what_users_should_do": ensure_str(
                tile.get("what_users_should_do"),
                f"{row_id}.tile.what_users_should_do",
            ),
            "what_operators_should_do": ensure_str(
                tile.get("what_operators_should_do"),
                f"{row_id}.tile.what_operators_should_do",
            ),
        },
        "notes": ensure_str(row.get("notes"), f"{row_id}.notes"),
    }


def compose_snapshot(
    repo_root: Path,
    catalog: dict[str, Any],
    canonical_catalog: dict[str, Any],
    state_rows: dict[str, Any],
    surface_register: dict[str, Any],
    cohort_guardrails: dict[str, Any],
    waiver_register: dict[str, Any],
    generated_at: str,
    findings: list[Finding],
) -> dict[str, Any]:
    catalog_as_of = ensure_str(catalog.get("as_of"), "catalog.as_of")

    # Same-change-set guard on upstream sources.
    for label, payload in (
        ("waiver_register", waiver_register),
    ):
        as_of = payload.get("as_of")
        if (
            as_of is not None
            and ensure_str(as_of, f"{label}.as_of") != catalog_as_of
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="upstream_as_of_drift",
                    message=(
                        f"{label} as_of {as_of} drifts from catalog as_of "
                        f"{catalog_as_of}"
                    ),
                    remediation=(
                        "Re-run the upstream generators in the same change "
                        "set or update the catalog as_of to match."
                    ),
                    ref=label,
                )
            )

    canonical_index = index_canonical_rows(canonical_catalog)
    surface_index = index_claimed_surfaces(surface_register)
    cohort_ids = index_cohort_ids(cohort_guardrails)
    waiver_index = index_waiver_rows(waiver_register)

    state_vocab = {
        "tile_states": collect_state_vocabulary_ids(state_rows, "tile_states"),
        "evidence_freshness_classes": collect_state_vocabulary_ids(
            state_rows, "evidence_freshness_classes"
        ),
        "mitigation_note_classes": collect_state_vocabulary_ids(
            state_rows, "mitigation_note_classes"
        ),
        "partial_profile_result_classes": collect_state_vocabulary_ids(
            state_rows, "partial_profile_result_classes"
        ),
        "corpus_profile_identity_classes": collect_state_vocabulary_ids(
            state_rows, "corpus_profile_identity_classes"
        ),
        "waiver_authority_classes": collect_state_vocabulary_ids(
            state_rows, "waiver_authority_classes"
        ),
    }

    tiles_out: list[dict[str, Any]] = []
    covered_lanes: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(catalog.get("rows"), "catalog.rows")):
        row = ensure_dict(raw_row, f"catalog.rows[{idx}]")
        row_id = ensure_str(row.get("id"), f"catalog.rows[{idx}].id")
        waiver_row = waiver_index.get(row_id)
        tile = compose_tile(
            row=row,
            waiver_row=waiver_row,
            canonical_index=canonical_index,
            state_vocab=state_vocab,
            surface_index=surface_index,
            findings=findings,
        )
        covered_lanes.add(tile["protected_lane_class"])
        tiles_out.append(tile)

    # Coverage: all required protected lanes must be present.
    missing_lanes = REQUIRED_PROTECTED_LANES - covered_lanes
    for missing in sorted(missing_lanes):
        findings.append(
            Finding(
                severity="error",
                check_id="required_protected_lane_missing",
                message=(
                    f"required protected lane is not present in the M3 "
                    f"catalog: {missing}"
                ),
                remediation=(
                    "Add a row with protected_lane_class = " + missing
                ),
                ref=missing,
            )
        )

    # Coverage: waiver register must cite each catalog row exactly once.
    waiver_row_targets = {
        ensure_str(
            r.get("protected_function_ref"),
            "waiver_register.row.protected_function_ref",
        )
        for r in ensure_list(waiver_register.get("rows"), "waiver_register.rows")
    }
    catalog_row_ids = {tile["row_id"] for tile in tiles_out}
    for extra in sorted(waiver_row_targets - catalog_row_ids):
        findings.append(
            Finding(
                severity="error",
                check_id="waiver_register_row_orphan",
                message=(
                    f"waiver_register row protected_function_ref {extra} "
                    "does not resolve to a catalog row"
                ),
                remediation=(
                    "Remove the waiver register row or add a matching "
                    "catalog row."
                ),
                ref=extra,
            )
        )

    # Cohort coverage: every beta surface cited by a catalog row must
    # name at least one cohort in the cohort guardrails.
    for tile in tiles_out:
        for sref in tile["beta_surface_refs"]:
            surface = surface_index.get(sref)
            if surface is None:
                continue
            primary = ensure_list(
                surface.get("primary_cohort_refs", []),
                f"{sref}.primary_cohort_refs",
            )
            if not primary:
                continue
            for cref in primary:
                cohort_id = ensure_str(cref, f"{sref}.primary_cohort_refs[]")
                if cohort_id not in cohort_ids:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="row_references_unknown_cohort",
                            message=(
                                f"{tile['row_id']} reaches unknown cohort "
                                f"via beta surface {sref}: {cohort_id}"
                            ),
                            remediation=(
                                "Use a cohort_id from "
                                "artifacts/milestones/m3/cohort_guardrails.yaml."
                            ),
                            ref=tile["row_id"],
                        )
                    )

    consuming_surfaces = [
        ensure_str(ref, f"catalog.consuming_surfaces[{i}]")
        for i, ref in enumerate(
            ensure_list(
                catalog.get("consuming_surfaces"),
                "catalog.consuming_surfaces",
            )
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
                        "publishing the snapshot."
                    ),
                    ref=ref,
                )
            )

    backup_owner = catalog.get("backup_owner")
    backup_waiver = catalog.get("backup_waiver")
    if backup_owner is not None:
        backup_owner = ensure_str(backup_owner, "catalog.backup_owner")
    if backup_waiver is not None:
        backup_waiver = ensure_str(backup_waiver, "catalog.backup_waiver")
    if backup_owner is None and not backup_waiver:
        findings.append(
            Finding(
                severity="error",
                check_id="catalog.backup_waiver.required",
                message=(
                    "catalog backup_owner is null but backup_waiver is "
                    "missing"
                ),
                remediation=(
                    "Set backup_owner to a handle or attach a backup_waiver "
                    "id."
                ),
            )
        )

    vocabularies = ensure_dict(
        catalog.get("vocabularies"), "catalog.vocabularies"
    )

    return {
        "schema_version": 1,
        "record_kind": "m3_protected_fitness_dashboard_snapshot",
        "snapshot_id": ensure_str(
            catalog.get("catalog_id"), "catalog.catalog_id"
        ),
        "snapshot_revision": int(catalog.get("catalog_revision", 1)),
        "milestone_id": ensure_str(
            catalog.get("milestone_id"), "catalog.milestone_id"
        ),
        "release_channel_scope": ensure_str(
            catalog.get("release_channel_scope"),
            "catalog.release_channel_scope",
        ),
        "snapshot_state": ensure_str(
            catalog.get("catalog_state"), "catalog.catalog_state"
        ),
        "as_of": catalog_as_of,
        "generated_at": generated_at,
        "owner": ensure_str(catalog.get("owner"), "catalog.owner"),
        "backup_owner": backup_owner,
        "backup_waiver": backup_waiver,
        "source_refs": {
            "canonical_catalog": ensure_str(
                catalog.get("canonical_catalog_source"),
                "catalog.canonical_catalog_source",
            ),
            "fitness_state_rows": ensure_str(
                catalog.get("fitness_state_rows_source"),
                "catalog.fitness_state_rows_source",
            ),
            "claimed_surface_register": ensure_str(
                catalog.get("claimed_surface_register_source"),
                "catalog.claimed_surface_register_source",
            ),
            "cohort_guardrails": ensure_str(
                catalog.get("cohort_guardrails_source"),
                "catalog.cohort_guardrails_source",
            ),
            "waiver_register": ensure_str(
                catalog.get("waiver_register_source"),
                "catalog.waiver_register_source",
            ),
        },
        "vocabularies": {
            "protected_lane_class": [
                ensure_str(v, "protected_lane_class[]")
                for v in ensure_list(
                    vocabularies.get("protected_lane_class"),
                    "catalog.vocabularies.protected_lane_class",
                )
            ],
            "row_status": [
                ensure_str(v, "row_status[]")
                for v in ensure_list(
                    vocabularies.get("row_status"),
                    "catalog.vocabularies.row_status",
                )
            ],
            "tile_state": [
                ensure_str(v, "tile_state[]")
                for v in ensure_list(
                    vocabularies.get("tile_state"),
                    "catalog.vocabularies.tile_state",
                )
            ],
            "evidence_freshness_class": [
                ensure_str(v, "evidence_freshness_class[]")
                for v in ensure_list(
                    vocabularies.get("evidence_freshness_class"),
                    "catalog.vocabularies.evidence_freshness_class",
                )
            ],
            "mitigation_note_class": [
                ensure_str(v, "mitigation_note_class[]")
                for v in ensure_list(
                    vocabularies.get("mitigation_note_class"),
                    "catalog.vocabularies.mitigation_note_class",
                )
            ],
            "waiver_authority": [
                ensure_str(v, "waiver_authority[]")
                for v in ensure_list(
                    vocabularies.get("waiver_authority"),
                    "catalog.vocabularies.waiver_authority",
                )
            ],
        },
        "consuming_surfaces": consuming_surfaces,
        "tiles": tiles_out,
        "notes": (
            "Generated by ci/check_m3_protected_fitness_catalog.py from "
            "the M3 protected fitness-function catalog and the M3 waiver "
            "register. Do not edit by hand; refresh the catalog or the "
            "waiver register and re-run the generator."
        ),
    }


def write_capture(
    path: Path,
    findings: list[Finding],
    catalog_rel: str,
    snapshot_rel: str,
    generated_at: str,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass"
        if not any(f.severity == "error" for f in findings)
        else "fail",
        "generated_at": generated_at,
        "catalog_ref": catalog_rel,
        "snapshot_ref": snapshot_rel,
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


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


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
    catalog_path = repo_root / args.catalog
    snapshot_path = repo_root / args.snapshot
    capture_path = repo_root / args.capture

    catalog = ensure_dict(render_yaml_as_json(catalog_path), "catalog")

    canonical_catalog = ensure_dict(
        render_yaml_as_json(repo_root / catalog["canonical_catalog_source"]),
        "canonical_catalog",
    )
    state_rows = ensure_dict(
        render_yaml_as_json(repo_root / catalog["fitness_state_rows_source"]),
        "fitness_state_rows",
    )
    surface_register = ensure_dict(
        load_json(repo_root / catalog["claimed_surface_register_source"]),
        "claimed_surface_register",
    )
    cohort_guardrails = ensure_dict(
        render_yaml_as_json(repo_root / catalog["cohort_guardrails_source"]),
        "cohort_guardrails",
    )
    waiver_register = ensure_dict(
        render_yaml_as_json(repo_root / catalog["waiver_register_source"]),
        "waiver_register",
    )

    findings: list[Finding] = []
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )

    snapshot = compose_snapshot(
        repo_root=repo_root,
        catalog=catalog,
        canonical_catalog=canonical_catalog,
        state_rows=state_rows,
        surface_register=surface_register,
        cohort_guardrails=cohort_guardrails,
        waiver_register=waiver_register,
        generated_at=generated_at,
        findings=findings,
    )

    snapshot_text = json.dumps(snapshot, indent=2, sort_keys=True) + "\n"

    snapshot_changed = write_if_changed(
        snapshot_path, snapshot_text, args.check
    )

    if args.check and snapshot_changed:
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard_snapshot.stale",
                message=(
                    "checked-in dashboard snapshot is stale relative to the "
                    "catalog or waiver register"
                ),
                remediation=(
                    "Run `python3 ci/check_m3_protected_fitness_catalog.py "
                    "--repo-root .` and commit the regenerated snapshot."
                ),
                details={"snapshot_changed": snapshot_changed},
            )
        )

    write_capture(
        capture_path,
        findings,
        args.catalog,
        args.snapshot,
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

    print("M3 protected fitness-function dashboard snapshot generated and validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
