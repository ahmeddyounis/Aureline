#!/usr/bin/env python3
"""Unattended skew / drift smoke runner.

Replays every row in
``fixtures/skew/m1_channel_and_schema_cases.yaml`` against the canonical
sources the matrix joins (the qualification matrix seed, the version
skew register, the skew-windows declaration, the install-topology
matrix, the state-root map, and the dogfood matrix) and asserts:

- every row's ``compatibility_row_ref`` resolves to a row in
  ``artifacts/compat/qualification_matrix_seed.yaml``;
- every row's ``version_skew_register_ref`` resolves to a skew_case in
  ``artifacts/compat/version_skew_register.yaml``;
- ``side_by_side_install`` rows' ``install_topology_refs`` resolve to
  ``install_profile_card`` ids in the install-topology matrix, and their
  ``expected_durable_state_root_refs`` resolve to state-root rows whose
  ``owning_channels`` admit the channel; cross-channel state sharing is
  refused;
- ``state_schema_migration`` rows pin an explicit
  ``expected_migration_direction`` and a non-null ``recovery_rung_ref``
  when degraded fidelity is the expected outcome; "exact" fidelity is
  refused under ``repairable`` skew;
- ``helper_agent_attach`` rows that claim ``compatible`` skew with a
  review-only outcome are refused;
- the row's ``surface_class``, ``skew_state_class``,
  ``outcome_label_class``, ``promotion_decision_class``, and
  ``boundary_family`` are in the closed matrix vocabularies;
- the row's ``inherited_dogfood_row_id`` resolves to the dogfood matrix;
- every row declares exactly one named failure drill drawn from the
  matrix ``failure_drill_id_vocabulary``; and
- the row's named failure drill, when forced, reproduces the exact
  ``expected_check_id`` the row claims (no silent regression cover).

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill`` replays the named
drill on the named row and exits 0 only when the runner reproduces the
declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix files
are parsed via Ruby/Psych.
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


DEFAULT_MATRIX_REL = "fixtures/skew/m1_channel_and_schema_cases.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/skew_drift_smoke_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"


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
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Smoke matrix YAML path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build identity record to embed in the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<smoke_row_id>:<drill_id>'. The runner exits 0 only when "
            "the row's failure drill reproduces the exact expected_check_id."
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
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


# ---- canonical-source loaders ----------------------------------------------


def load_compat_row_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(
        payload.get("qualification_rows"), f"{ref}.qualification_rows"
    )
    ids: set[str] = set()
    for idx, row in enumerate(rows):
        row = ensure_dict(row, f"{ref}.qualification_rows[{idx}]")
        ids.add(
            ensure_str(
                row.get("row_id"), f"{ref}.qualification_rows[{idx}].row_id"
            )
        )
    return ids


def load_skew_register_case_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    register = ensure_list(payload.get("register"), f"{ref}.register")
    ids: set[str] = set()
    for idx, entry in enumerate(register):
        entry = ensure_dict(entry, f"{ref}.register[{idx}]")
        for bucket in ("supported", "best_effort", "untested", "unsupported"):
            cases = entry.get(bucket) or []
            if not isinstance(cases, list):
                continue
            for jdx, case in enumerate(cases):
                case = ensure_dict(
                    case, f"{ref}.register[{idx}].{bucket}[{jdx}]"
                )
                ids.add(
                    ensure_str(
                        case.get("skew_case_id"),
                        f"{ref}.register[{idx}].{bucket}[{jdx}].skew_case_id",
                    )
                )
    return ids


def load_skew_window_boundary_families(
    repo_root: Path, ref: str
) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    vocab = ensure_list(
        payload.get("boundary_family_vocabulary"),
        f"{ref}.boundary_family_vocabulary",
    )
    return {ensure_str(item, f"{ref}.boundary_family_vocabulary[]") for item in vocab}


def load_install_card_ids(repo_root: Path, ref: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    cards = ensure_list(
        payload.get("install_profile_cards"), f"{ref}.install_profile_cards"
    )
    by_id: dict[str, dict[str, Any]] = {}
    for idx, raw_card in enumerate(cards):
        card = ensure_dict(raw_card, f"{ref}.install_profile_cards[{idx}]")
        card_id = ensure_str(
            card.get("id"), f"{ref}.install_profile_cards[{idx}].id"
        )
        by_id[card_id] = card
    return by_id


def load_state_roots(repo_root: Path, ref: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(payload.get("state_roots"), f"{ref}.state_roots")
    by_id: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"{ref}.state_roots[{idx}]")
        row_id = ensure_str(row.get("id"), f"{ref}.state_roots[{idx}].id")
        by_id[row_id] = row
    return by_id


def load_dogfood_row_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(payload.get("rows"), f"{ref}.rows")
    ids: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"{ref}.rows[{idx}]")
        ids.add(
            ensure_str(row.get("row_id"), f"{ref}.rows[{idx}].row_id")
        )
    return ids


# ---- matrix replay --------------------------------------------------------


@dataclass
class RowResult:
    smoke_row_id: str
    inherited_dogfood_row_id: str
    surface_class: str
    boundary_family: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def replay_row(
    row: dict[str, Any],
    *,
    compat_row_ids: set[str],
    skew_case_ids: set[str],
    skew_boundary_families: set[str],
    install_cards: dict[str, dict[str, Any]],
    state_roots: dict[str, dict[str, Any]],
    dogfood_row_ids: set[str],
    surface_class_vocab: set[str],
    skew_state_class_vocab: set[str],
    outcome_label_vocab: set[str],
    promotion_decision_vocab: set[str],
    boundary_family_vocab: set[str],
    failure_drill_id_vocab: set[str],
    forced_overrides: dict[str, Any] | None = None,
) -> RowResult:
    forced_overrides = forced_overrides or {}
    smoke_row_id = ensure_str(row.get("smoke_row_id"), "row.smoke_row_id")
    inherited_dogfood = ensure_str(
        row.get("inherited_dogfood_row_id"),
        f"{smoke_row_id}.inherited_dogfood_row_id",
    )
    surface_class = ensure_str(
        row.get("surface_class"), f"{smoke_row_id}.surface_class"
    )
    boundary_family = ensure_str(
        row.get("boundary_family"), f"{smoke_row_id}.boundary_family"
    )
    result = RowResult(
        smoke_row_id=smoke_row_id,
        inherited_dogfood_row_id=inherited_dogfood,
        surface_class=surface_class,
        boundary_family=boundary_family,
    )

    # Closed-vocabulary checks.
    if surface_class not in surface_class_vocab:
        fail(
            result,
            "skew_drift.surface_class_unknown",
            f"surface_class '{surface_class}' is not in the matrix vocabulary",
        )
    else:
        result.passed_checks.append(
            f"surface_class '{surface_class}' is in the matrix vocabulary"
        )

    if boundary_family not in boundary_family_vocab:
        fail(
            result,
            "skew_drift.boundary_family_unknown",
            f"boundary_family '{boundary_family}' is not in the matrix vocabulary",
        )
    if boundary_family not in skew_boundary_families:
        fail(
            result,
            "skew_drift.boundary_family_missing_from_skew_windows",
            (
                f"boundary_family '{boundary_family}' is not in "
                "artifacts/compat/skew_windows.yaml boundary_family_vocabulary"
            ),
        )

    skew_state = ensure_str(
        row.get("skew_state_class"), f"{smoke_row_id}.skew_state_class"
    )
    if "rewrite_skew_state_class" in forced_overrides:
        skew_state = ensure_str(
            forced_overrides["rewrite_skew_state_class"],
            f"{smoke_row_id}.failure_drill.forced_input.rewrite_skew_state_class",
        )
    if skew_state not in skew_state_class_vocab:
        fail(
            result,
            "skew_drift.skew_state_class_unknown",
            f"skew_state_class '{skew_state}' is not in the matrix vocabulary",
        )

    outcome_label = ensure_str(
        row.get("outcome_label_class"), f"{smoke_row_id}.outcome_label_class"
    )
    if "rewrite_outcome_label_class" in forced_overrides:
        outcome_label = ensure_str(
            forced_overrides["rewrite_outcome_label_class"],
            f"{smoke_row_id}.failure_drill.forced_input.rewrite_outcome_label_class",
        )
    if outcome_label not in outcome_label_vocab:
        fail(
            result,
            "skew_drift.outcome_label_unknown_class",
            f"outcome_label_class '{outcome_label}' is not in the matrix vocabulary",
        )

    promotion_decision = ensure_str(
        row.get("promotion_decision_class"),
        f"{smoke_row_id}.promotion_decision_class",
    )
    if "rewrite_promotion_decision_class" in forced_overrides:
        promotion_decision = ensure_str(
            forced_overrides["rewrite_promotion_decision_class"],
            f"{smoke_row_id}.failure_drill.forced_input.rewrite_promotion_decision_class",
        )
    if promotion_decision not in promotion_decision_vocab:
        fail(
            result,
            "skew_drift.promotion_decision_unknown_class",
            (
                f"promotion_decision_class '{promotion_decision}' is not in "
                "the matrix vocabulary"
            ),
        )

    # Dogfood row resolution.
    if inherited_dogfood not in dogfood_row_ids:
        fail(
            result,
            "dogfood_matrix.inherited_row_missing",
            (
                f"inherited_dogfood_row_id '{inherited_dogfood}' is not in "
                "the dogfood matrix"
            ),
        )
    else:
        result.passed_checks.append(
            f"inherited_dogfood_row_id '{inherited_dogfood}' resolves"
        )

    # Compatibility row + skew register resolution.
    compatibility_row_ref = ensure_str(
        row.get("compatibility_row_ref"),
        f"{smoke_row_id}.compatibility_row_ref",
    )
    if compatibility_row_ref not in compat_row_ids:
        fail(
            result,
            "skew_drift.compatibility_row_missing",
            (
                f"compatibility_row_ref '{compatibility_row_ref}' is not in "
                "artifacts/compat/qualification_matrix_seed.yaml"
            ),
        )
    else:
        result.passed_checks.append(
            f"compatibility_row_ref '{compatibility_row_ref}' resolves"
        )

    skew_register_ref = ensure_str(
        row.get("version_skew_register_ref"),
        f"{smoke_row_id}.version_skew_register_ref",
    )
    if skew_register_ref not in skew_case_ids:
        fail(
            result,
            "skew_drift.skew_register_case_missing",
            (
                f"version_skew_register_ref '{skew_register_ref}' is not in "
                "artifacts/compat/version_skew_register.yaml"
            ),
        )
    else:
        result.passed_checks.append(
            f"version_skew_register_ref '{skew_register_ref}' resolves"
        )

    # Per-surface checks.
    install_topology_refs_raw = ensure_list(
        row.get("install_topology_refs") or [],
        f"{smoke_row_id}.install_topology_refs",
    )
    install_topology_refs = [
        ensure_str(item, f"{smoke_row_id}.install_topology_refs[]")
        for item in install_topology_refs_raw
    ]

    expected_state_root_refs = row.get("expected_durable_state_root_refs") or {}
    if not isinstance(expected_state_root_refs, dict):
        fail(
            result,
            "skew_drift.expected_state_root_refs_shape",
            "expected_durable_state_root_refs must be a YAML mapping",
        )
        expected_state_root_refs = {}

    # Apply forced override for cross-channel state root rewrite.
    rewrite_preview_state = forced_overrides.get("rewrite_preview_state_root")
    if isinstance(rewrite_preview_state, dict):
        from_ref = rewrite_preview_state.get("from")
        to_ref = rewrite_preview_state.get("to")
        preview_list = expected_state_root_refs.get("preview")
        if (
            isinstance(from_ref, str)
            and isinstance(to_ref, str)
            and isinstance(preview_list, list)
            and from_ref in preview_list
        ):
            expected_state_root_refs = {
                **expected_state_root_refs,
                "preview": [
                    to_ref if ref == from_ref else ref for ref in preview_list
                ],
            }

    if surface_class == "side_by_side_install":
        # Resolve cards.
        for card_id in install_topology_refs:
            if card_id not in install_cards:
                fail(
                    result,
                    "skew_drift.install_card_missing",
                    (
                        f"install_topology_refs entry '{card_id}' is not in "
                        "the install-topology matrix"
                    ),
                )
            else:
                result.passed_checks.append(
                    f"install_topology_refs entry '{card_id}' resolves"
                )

        # Channel ownership of state roots and cross-channel collision.
        seen_state_root_ids: dict[str, str] = {}
        for channel_label, root_refs in expected_state_root_refs.items():
            if not isinstance(channel_label, str) or not isinstance(root_refs, list):
                fail(
                    result,
                    "skew_drift.state_root_channel_shape",
                    (
                        "expected_durable_state_root_refs entries must be "
                        "channel_label -> list-of-state-root-ids"
                    ),
                )
                continue
            for ref_id in root_refs:
                if not isinstance(ref_id, str):
                    continue
                sr_row = state_roots.get(ref_id)
                if sr_row is None:
                    fail(
                        result,
                        "skew_drift.state_root_missing",
                        (
                            f"expected_durable_state_root_refs entry '{ref_id}' "
                            "is not in the state-root map"
                        ),
                    )
                    continue
                owning_channels = sr_row.get("owning_channels") or []
                shared = bool(sr_row.get("shared_across_channels"))
                if not shared and channel_label not in owning_channels:
                    fail(
                        result,
                        "side_by_side.state_root_owning_channel_collision",
                        (
                            f"state_root '{ref_id}' owning_channels "
                            f"{owning_channels} does not include "
                            f"channel '{channel_label}'"
                        ),
                    )
                # Cross-channel collision detection: same state-root id
                # cannot appear under two distinct channels.
                prior_channel = seen_state_root_ids.get(ref_id)
                if prior_channel is not None and prior_channel != channel_label:
                    fail(
                        result,
                        "side_by_side.state_root_owning_channel_collision",
                        (
                            f"state_root '{ref_id}' is referenced by both "
                            f"channels '{prior_channel}' and '{channel_label}' "
                            "on a side-by-side row"
                        ),
                    )
                seen_state_root_ids[ref_id] = channel_label

    elif surface_class == "downgrade_upgrade_rollback":
        for card_id in install_topology_refs:
            if card_id not in install_cards:
                fail(
                    result,
                    "skew_drift.install_card_missing",
                    (
                        f"install_topology_refs entry '{card_id}' is not in "
                        "the install-topology matrix"
                    ),
                )
        for channel_label, root_refs in expected_state_root_refs.items():
            if not isinstance(root_refs, list):
                continue
            for ref_id in root_refs:
                if isinstance(ref_id, str) and ref_id not in state_roots:
                    fail(
                        result,
                        "skew_drift.state_root_missing",
                        (
                            f"expected_durable_state_root_refs entry "
                            f"'{ref_id}' is not in the state-root map"
                        ),
                    )

    elif surface_class == "state_schema_migration":
        direction = ensure_str(
            row.get("expected_migration_direction"),
            f"{smoke_row_id}.expected_migration_direction",
        )
        fidelity = ensure_str(
            row.get("expected_fidelity_label"),
            f"{smoke_row_id}.expected_fidelity_label",
        )
        if "rewrite_expected_fidelity_label" in forced_overrides:
            fidelity = ensure_str(
                forced_overrides["rewrite_expected_fidelity_label"],
                f"{smoke_row_id}.failure_drill.forced_input.rewrite_expected_fidelity_label",
            )

        # Silent fidelity downgrade: repairable skew must not declare
        # exact fidelity.
        if skew_state == "repairable" and fidelity == "exact":
            fail(
                result,
                "state_migration.silent_fidelity_downgrade",
                (
                    "state_schema_migration row claims repairable skew with "
                    "expected_fidelity_label='exact'; additive migration "
                    "must report 'compatible' fidelity, never 'exact'"
                ),
            )
        # Direction sanity: blocked skew implies new_to_old direction;
        # repairable in the additive direction is old_to_new.
        if skew_state == "blocked" and direction != "blocked_new_to_old":
            fail(
                result,
                "state_migration.direction_misaligned_with_skew_state",
                (
                    "state_schema_migration row claims blocked skew but "
                    f"expected_migration_direction='{direction}'; blocked "
                    "downgrade must declare 'blocked_new_to_old'"
                ),
            )

        recovery_rung = row.get("recovery_rung_ref")
        # Apply drop_recovery_rung override.
        if forced_overrides.get("drop_recovery_rung"):
            recovery_rung = None
        # Blocked or degraded migrations must point at a concrete
        # recovery rung; pure compatible migration may omit it.
        if skew_state in {"blocked", "degraded", "repairable"} and not recovery_rung:
            fail(
                result,
                "state_migration.recovery_rung_missing",
                (
                    f"state_schema_migration row with skew_state_class "
                    f"'{skew_state}' must point at a recovery_rung_ref"
                ),
            )

    elif surface_class == "helper_agent_attach":
        # A review-only attach is degraded skew, not compatible.
        if (
            skew_state == "compatible"
            and outcome_label == "attach_degraded_review_only"
        ):
            fail(
                result,
                "helper_agent_attach.skew_state_not_aligned_with_outcome",
                (
                    "helper_agent_attach row claims compatible skew with "
                    "outcome_label_class='attach_degraded_review_only'; "
                    "review-only attach is degraded skew, not compatible"
                ),
            )

    # Failure drill structure.
    failure_drill = ensure_dict(
        row.get("failure_drill"), f"{smoke_row_id}.failure_drill"
    )
    drill_id = ensure_str(
        failure_drill.get("drill_id"), f"{smoke_row_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "failure_drills.unknown_vocabulary",
            f"failure_drill.drill_id '{drill_id}' is not in the matrix vocabulary",
        )
    expected_check = ensure_str(
        failure_drill.get("expected_check_id"),
        f"{smoke_row_id}.failure_drill.expected_check_id",
    )
    actionable_owner = ensure_str(
        failure_drill.get("actionable_owner_ref"),
        f"{smoke_row_id}.failure_drill.actionable_owner_ref",
    )
    next_action = ensure_str(
        failure_drill.get("next_action"),
        f"{smoke_row_id}.failure_drill.next_action",
    )

    result.diagnostics.update(
        {
            "surface_class": surface_class,
            "boundary_family": boundary_family,
            "skew_state_class": skew_state,
            "outcome_label_class": outcome_label,
            "promotion_decision_class": promotion_decision,
            "compatibility_row_ref": compatibility_row_ref,
            "version_skew_register_ref": skew_register_ref,
            "install_topology_refs": install_topology_refs,
            "expected_durable_state_root_refs": expected_state_root_refs,
            "recovery_rung_ref": row.get("recovery_rung_ref"),
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": expected_check,
                "actionable_owner_ref": actionable_owner,
                "next_action": next_action,
            },
            "forced_overrides_applied": forced_overrides,
        }
    )

    return result


# ---- main -----------------------------------------------------------------


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=f"matrix schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the runner together with the schema if the matrix shape changes.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    overview_page = ensure_str(matrix.get("overview_page"), "matrix.overview_page")
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer-facing landing page or fix the path.",
                ref=overview_page,
            )
        )

    vocab_sources = ensure_dict(
        matrix.get("vocabulary_sources"), "matrix.vocabulary_sources"
    )
    for key, value in vocab_sources.items():
        ref = ensure_str(value, f"matrix.vocabulary_sources.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.vocabulary_sources.missing",
                    message=f"vocabulary_sources.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    hard_deps = ensure_list(
        matrix.get("hard_dependency_refs"), "matrix.hard_dependency_refs"
    )
    for idx, ref in enumerate(hard_deps):
        ref = ensure_str(ref, f"matrix.hard_dependency_refs[{idx}]")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.hard_dependency_refs.missing",
                    message=f"hard_dependency_refs[{idx}] does not exist: {ref}",
                    remediation="Fix the dependency path; the lane must consume live upstream surfaces.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
                message=f"validation_lane_ref base does not exist: {validation_lane_ref}",
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    compat_row_ids = load_compat_row_ids(
        repo_root, vocab_sources["qualification_matrix_ref"]
    )
    skew_case_ids = load_skew_register_case_ids(
        repo_root, vocab_sources["version_skew_register_ref"]
    )
    skew_boundary_families = load_skew_window_boundary_families(
        repo_root, vocab_sources["skew_windows_ref"]
    )
    install_cards = load_install_card_ids(
        repo_root, vocab_sources["install_topology_matrix_ref"]
    )
    state_roots = load_state_roots(
        repo_root, vocab_sources["state_root_map_ref"]
    )
    dogfood_row_ids = load_dogfood_row_ids(
        repo_root, vocab_sources["dogfood_matrix_ref"]
    )

    surface_class_vocab = {
        ensure_str(item, "matrix.surface_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("surface_class_vocabulary"),
            "matrix.surface_class_vocabulary",
        )
    }
    skew_state_class_vocab = {
        ensure_str(item, "matrix.skew_state_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("skew_state_class_vocabulary"),
            "matrix.skew_state_class_vocabulary",
        )
    }
    outcome_label_vocab = {
        ensure_str(item, "matrix.outcome_label_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("outcome_label_class_vocabulary"),
            "matrix.outcome_label_class_vocabulary",
        )
    }
    promotion_decision_vocab = {
        ensure_str(item, "matrix.promotion_decision_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("promotion_decision_class_vocabulary"),
            "matrix.promotion_decision_class_vocabulary",
        )
    }
    boundary_family_vocab = {
        ensure_str(item, "matrix.boundary_family_vocabulary[]")
        for item in ensure_list(
            matrix.get("boundary_family_vocabulary"),
            "matrix.boundary_family_vocabulary",
        )
    }
    failure_drill_id_vocab = {
        ensure_str(item, "matrix.failure_drill_id_vocabulary[]")
        for item in ensure_list(
            matrix.get("failure_drill_id_vocabulary"),
            "matrix.failure_drill_id_vocabulary",
        )
    }
    required_surface_class_coverage = {
        ensure_str(item, "matrix.required_surface_class_coverage[]")
        for item in ensure_list(
            matrix.get("required_surface_class_coverage"),
            "matrix.required_surface_class_coverage",
        )
    }

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least one smoke row",
                remediation="Seed the required smoke rows.",
            )
        )

    # Resolve --force-drill if requested.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<smoke_row_id>:<drill_id>'"
            )
        forced_row_id, forced_drill_id = args.force_drill.split(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_surface_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")

        forced_overrides: dict[str, Any] = {}
        if forced_row_id is not None and ensure_str(
            row.get("smoke_row_id"), "row.smoke_row_id"
        ) == forced_row_id:
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            drill_id = ensure_str(
                failure_drill.get("drill_id"),
                f"{forced_row_id}.failure_drill.drill_id",
            )
            if drill_id != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id '{forced_drill_id}' does not match the "
                    f"row's failure_drill.drill_id '{drill_id}'"
                )
            forced_overrides = ensure_dict(
                failure_drill.get("forced_input"),
                f"{forced_row_id}.failure_drill.forced_input",
            )

        result = replay_row(
            row,
            compat_row_ids=compat_row_ids,
            skew_case_ids=skew_case_ids,
            skew_boundary_families=skew_boundary_families,
            install_cards=install_cards,
            state_roots=state_roots,
            dogfood_row_ids=dogfood_row_ids,
            surface_class_vocab=surface_class_vocab,
            skew_state_class_vocab=skew_state_class_vocab,
            outcome_label_vocab=outcome_label_vocab,
            promotion_decision_vocab=promotion_decision_vocab,
            boundary_family_vocab=boundary_family_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            forced_overrides=forced_overrides,
        )
        row_results.append(result)

        if result.smoke_row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate smoke_row_id: {result.smoke_row_id}",
                    remediation="smoke_row_ids must be unique.",
                    ref=result.smoke_row_id,
                )
            )
        seen_ids.add(result.smoke_row_id)
        seen_surface_classes.add(result.surface_class)

        if (
            forced_row_id is not None
            and result.smoke_row_id == forced_row_id
            and forced_overrides
        ):
            failure_drill = ensure_dict(
                row.get("failure_drill"), f"{forced_row_id}.failure_drill"
            )
            expected_check = ensure_str(
                failure_drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "smoke_row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_surfaces = required_surface_class_coverage - seen_surface_classes
    if missing_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_surfaces",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(required_surface_class_coverage)}; missing: "
                    f"{sorted(missing_surfaces)}"
                ),
                remediation=(
                    "Add the missing rows so every required surface class is "
                    "exercised."
                ),
            )
        )

    # Promote per-row failures into findings, but skip them under force-drill
    # mode for the targeted row so the runner's exit can reflect the
    # reproduce verdict rather than failing on the reproduced check.
    for result in row_results:
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get("check_id", "matrix.row.failed_check"),
                    message=f"{result.smoke_row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the canonical sources or fix the "
                        "drift in the matrix; failures are reported with the precise "
                        "actionable check_id."
                    ),
                    ref=result.smoke_row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "skew_drift_smoke_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/smoke/skew_drift/run_skew_drift_smoke.py --repo-root ."
        ),
        "status": status,
        "required_surface_class_coverage": sorted(required_surface_class_coverage),
        "observed_surface_classes": sorted(seen_surface_classes),
        "rows": [
            {
                "smoke_row_id": r.smoke_row_id,
                "inherited_dogfood_row_id": r.inherited_dogfood_row_id,
                "surface_class": r.surface_class,
                "boundary_family": r.boundary_family,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "skew-drift-smoke"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[{label}] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']} on "
                f"{forced_replay_record['smoke_row_id']} reproduced "
                f"{forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on "
            f"{forced_replay_record['smoke_row_id']} did NOT reproduce "
            f"{forced_replay_record['expected_check_id']}; "
            f"observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[skew-drift-smoke] interrupted", file=sys.stderr)
        sys.exit(130)
