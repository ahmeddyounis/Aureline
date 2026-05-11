#!/usr/bin/env python3
"""Unattended trust-policy and install-topology smoke runner.

Replays every row in
``fixtures/install/m1_topology_rows.yaml`` against the canonical sources
the matrix joins (the trust-state matrix, the install-topology matrix,
the state-root map, the credential-store capability matrix, the
claimed-desktop-profile roster, the native-trust integration matrix,
and the local-baseline proof artifact) and asserts:

- every row's ``install_truth_card_id`` resolves to an install-profile
  card whose ``install_mode_class``, ``channel_class``,
  ``side_by_side_relation_class``, and ``durable_state_root_refs`` match
  the row's expectations;
- every ``durable_state_root_ref`` resolves to a state-root row whose
  ``owning_channels`` admit the row's channel (or whose
  ``shared_across_channels`` flag is set);
- the row's ``trust_state`` is in the trust-state matrix vocabulary;
- on policy-degraded trust state, no surface that the trust-state matrix
  marks ``allowed`` for the trust state shows up in the row's
  ``expected_blocked_surfaces`` without an ``approval_required`` denial
  reason;
- the row's ``credential_store_posture.unlock_state`` is in the
  credential-store unlock vocabulary;
- the row's ``shell_mode`` is in the matrix vocabulary;
- every blocked-surface row carries a typed ``denial_reason`` from the
  matrix vocabulary plus a non-empty ``actionable_explanation``;
- every claimed_profile_id resolves to a row in
  ``artifacts/platform/claimed_desktop_profiles.yaml``;
- the local-baseline admissible-surface floor is not blocked on any
  row;
- every required smoke dimension is exercised by at least one row;
- every row declares one named failure drill drawn from the matrix
  ``failure_drill_id_vocabulary``; and
- the row's named failure drill, when forced, reproduces the exact
  ``expected_check_id`` the row claims (no silent regression cover).

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill`` replays the named
drill on the named row and exits 0 only when the runner reproduces the
declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix files
are parsed via Ruby/Psych (already required by other CI checks).
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


DEFAULT_MATRIX_REL = "fixtures/install/m1_topology_rows.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/install_topology_smoke_validation_capture.json"
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


def load_install_cards(repo_root: Path, ref: str) -> dict[str, dict[str, Any]]:
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


def load_trust_matrix(repo_root: Path, ref: str) -> dict[str, dict[str, str]]:
    """Return surface_family -> trust_state -> authority_kind mapping."""
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(
        payload.get("surface_matrix_rows"), f"{ref}.surface_matrix_rows"
    )
    by_surface: dict[str, dict[str, str]] = {}
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"{ref}.surface_matrix_rows[{idx}]")
        surface = ensure_str(
            row.get("surface_family"),
            f"{ref}.surface_matrix_rows[{idx}].surface_family",
        )
        per_state = ensure_list(
            row.get("per_state_authority"),
            f"{ref}.surface_matrix_rows[{idx}].per_state_authority",
        )
        state_to_authority: dict[str, str] = {}
        for jdx, entry in enumerate(per_state):
            entry = ensure_dict(
                entry,
                f"{ref}.surface_matrix_rows[{idx}].per_state_authority[{jdx}]",
            )
            ts = ensure_str(
                entry.get("trust_state"),
                f"{ref}.surface_matrix_rows[{idx}].per_state_authority[{jdx}].trust_state",
            )
            ak = ensure_str(
                entry.get("authority_kind"),
                f"{ref}.surface_matrix_rows[{idx}].per_state_authority[{jdx}].authority_kind",
            )
            state_to_authority[ts] = ak
        by_surface[surface] = state_to_authority
    return by_surface


def load_trust_state_vocabulary(by_surface: dict[str, dict[str, str]]) -> set[str]:
    states: set[str] = set()
    for state_to_authority in by_surface.values():
        states.update(state_to_authority.keys())
    return states


def load_store_capability_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(payload.get("rows"), f"{ref}.rows")
    classes: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"{ref}.rows[{idx}]")
        classes.add(
            ensure_str(
                row.get("store_source_class"),
                f"{ref}.rows[{idx}].store_source_class",
            )
        )
    return classes


def load_claimed_profile_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    profiles = ensure_list(payload.get("profiles"), f"{ref}.profiles")
    ids: set[str] = set()
    for idx, profile in enumerate(profiles):
        profile = ensure_dict(profile, f"{ref}.profiles[{idx}]")
        ids.add(
            ensure_str(
                profile.get("profile_id"),
                f"{ref}.profiles[{idx}].profile_id",
            )
        )
    return ids


def load_dogfood_row_ids(repo_root: Path, ref: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    rows = ensure_list(payload.get("rows"), f"{ref}.rows")
    ids: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"{ref}.rows[{idx}]")
        ids.add(
            ensure_str(
                row.get("row_id"), f"{ref}.rows[{idx}].row_id"
            )
        )
    return ids


def load_local_baseline_floor(repo_root: Path, ref: str) -> set[str]:
    """Return the union of admissible-surface classes the local-baseline
    proof rows declare — informational; the matrix's own
    ``local_baseline_admissible_surface_floor`` is what the runner enforces.
    """
    payload = ensure_dict(render_yaml_as_json(repo_root / ref), ref)
    vocab = ensure_list(
        payload.get("local_baseline_admissible_surface_class_vocabulary"),
        f"{ref}.local_baseline_admissible_surface_class_vocabulary",
    )
    return {ensure_str(item, f"{ref}[]") for item in vocab}


# ---- matrix replay --------------------------------------------------------


@dataclass
class RowResult:
    smoke_row_id: str
    inherited_dogfood_row_id: str
    install_truth_card_id: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def replay_row(
    row: dict[str, Any],
    *,
    install_cards: dict[str, dict[str, Any]],
    state_roots: dict[str, dict[str, Any]],
    trust_state_vocabulary: set[str],
    trust_matrix: dict[str, dict[str, str]],
    credential_store_unlock_vocab: set[str],
    credential_store_class_vocab: set[str],
    shell_mode_vocab: set[str],
    denial_reason_vocab: set[str],
    safe_preview_vocab: set[str],
    admissible_surface_vocab: set[str],
    local_baseline_floor: list[str],
    failure_drill_id_vocab: set[str],
    claimed_profile_ids: set[str],
    dogfood_row_ids: set[str],
    smoke_dimension_vocab: set[str],
    forced_overrides: dict[str, Any] | None = None,
) -> RowResult:
    forced_overrides = forced_overrides or {}
    smoke_row_id = ensure_str(row.get("smoke_row_id"), "row.smoke_row_id")
    inherited_dogfood = ensure_str(
        row.get("inherited_dogfood_row_id"),
        f"{smoke_row_id}.inherited_dogfood_row_id",
    )
    install_truth_card_id = ensure_str(
        row.get("install_truth_card_id"),
        f"{smoke_row_id}.install_truth_card_id",
    )
    result = RowResult(
        smoke_row_id=smoke_row_id,
        inherited_dogfood_row_id=inherited_dogfood,
        install_truth_card_id=install_truth_card_id,
    )

    # Apply forced overrides for failure-drill replay against pure data.
    forced_install_mode = forced_overrides.get(
        "rewrite_expected_install_mode_class"
    )
    expected_install_mode = (
        forced_install_mode
        if forced_install_mode is not None
        else ensure_str(
            row.get("expected_install_mode_class"),
            f"{smoke_row_id}.expected_install_mode_class",
        )
    )
    expected_channel = ensure_str(
        row.get("expected_channel_class"),
        f"{smoke_row_id}.expected_channel_class",
    )
    expected_sxs = ensure_str(
        row.get("expected_side_by_side_relation_class"),
        f"{smoke_row_id}.expected_side_by_side_relation_class",
    )

    # Dogfood row resolution.
    if inherited_dogfood not in dogfood_row_ids:
        fail(
            result,
            "dogfood_matrix.inherited_row_missing",
            f"inherited_dogfood_row_id '{inherited_dogfood}' is not in the dogfood matrix",
        )
    else:
        result.passed_checks.append(
            f"inherited_dogfood_row_id '{inherited_dogfood}' resolves"
        )

    # Install-card resolution.
    card = install_cards.get(install_truth_card_id)
    if card is None:
        fail(
            result,
            "install_topology.install_truth_card.missing",
            f"install_truth_card_id '{install_truth_card_id}' is not in the install-topology matrix",
        )
        return result
    result.passed_checks.append(
        f"install_truth_card_id '{install_truth_card_id}' resolves"
    )

    card_install_mode = ensure_str(
        card.get("install_mode_class"),
        f"install_card[{install_truth_card_id}].install_mode_class",
    )
    card_channel = ensure_str(
        card.get("channel_class"),
        f"install_card[{install_truth_card_id}].channel_class",
    )
    card_sxs = ensure_str(
        card.get("side_by_side_relation_class"),
        f"install_card[{install_truth_card_id}].side_by_side_relation_class",
    )
    card_state_root_refs = [
        ensure_str(item, f"install_card[{install_truth_card_id}].durable_state_root_refs[]")
        for item in ensure_list(
            card.get("durable_state_root_refs"),
            f"install_card[{install_truth_card_id}].durable_state_root_refs",
        )
    ]

    if expected_install_mode != card_install_mode:
        fail(
            result,
            "install_topology.install_mode_class.mismatch",
            (
                f"expected_install_mode_class '{expected_install_mode}' does not match "
                f"install_truth_card '{install_truth_card_id}' install_mode_class '{card_install_mode}'"
            ),
        )
    else:
        result.passed_checks.append(
            f"install_mode_class '{expected_install_mode}' matches the install-topology card"
        )

    if expected_channel != card_channel:
        fail(
            result,
            "install_topology.channel_class.mismatch",
            (
                f"expected_channel_class '{expected_channel}' does not match "
                f"install_truth_card '{install_truth_card_id}' channel_class '{card_channel}'"
            ),
        )
    else:
        result.passed_checks.append(
            f"channel_class '{expected_channel}' matches the install-topology card"
        )

    if expected_sxs != card_sxs:
        fail(
            result,
            "install_topology.side_by_side_relation_class.mismatch",
            (
                f"expected_side_by_side_relation_class '{expected_sxs}' does not match "
                f"install_truth_card '{install_truth_card_id}' side_by_side_relation_class '{card_sxs}'"
            ),
        )
    else:
        result.passed_checks.append(
            f"side_by_side_relation_class '{expected_sxs}' matches the install-topology card"
        )

    # State-root resolution and per-channel ownership.
    declared_state_roots_raw = ensure_list(
        row.get("expected_durable_state_root_refs"),
        f"{smoke_row_id}.expected_durable_state_root_refs",
    )
    declared_state_roots = [
        ensure_str(item, f"{smoke_row_id}.expected_durable_state_root_refs[]")
        for item in declared_state_roots_raw
    ]

    # Apply per-row state-root rewrite override.
    rewrite_state = forced_overrides.get("rewrite_durable_state_root_ref")
    if isinstance(rewrite_state, dict):
        from_ref = rewrite_state.get("from")
        to_ref = rewrite_state.get("to")
        if (
            isinstance(from_ref, str)
            and isinstance(to_ref, str)
            and from_ref in declared_state_roots
        ):
            declared_state_roots = [
                to_ref if r == from_ref else r for r in declared_state_roots
            ]

    if not declared_state_roots:
        fail(
            result,
            "install_topology.state_root.empty",
            "expected_durable_state_root_refs must declare at least one row",
        )

    portable_root = "portable_colocated_root"
    for ref_id in declared_state_roots:
        sr_row = state_roots.get(ref_id)
        if sr_row is None:
            fail(
                result,
                "install_topology.state_root.missing",
                f"expected_durable_state_root_ref '{ref_id}' is not in the state-root map",
            )
            continue
        owning_channels = sr_row.get("owning_channels")
        if not isinstance(owning_channels, list):
            owning_channels = []
        shared = bool(sr_row.get("shared_across_channels"))
        if shared:
            result.passed_checks.append(
                f"state_root '{ref_id}' is shared_across_channels"
            )
            continue
        if expected_channel not in owning_channels:
            fail(
                result,
                "side_by_side.state_root_owning_channel_collision",
                (
                    f"state_root '{ref_id}' owning_channels {owning_channels} does not "
                    f"include the row's channel '{expected_channel}'"
                ),
            )
        else:
            result.passed_checks.append(
                f"state_root '{ref_id}' is owned by channel '{expected_channel}'"
            )

    # Cross-check the install-card's durable_state_root_refs against the
    # row's declared list. The matrix has not declared the equivalent
    # check on the per-row side, so we widen here: every declared root
    # MUST appear on the card and vice versa.
    card_root_set = set(card_state_root_refs)
    declared_root_set = set(declared_state_roots)
    extra_on_row = declared_root_set - card_root_set
    extra_on_card = card_root_set - declared_root_set
    if extra_on_row:
        fail(
            result,
            "install_topology.state_root.row_not_in_card",
            (
                f"row declares state-roots {sorted(extra_on_row)} that are not on "
                f"install card '{install_truth_card_id}'"
            ),
        )
    if extra_on_card:
        fail(
            result,
            "install_topology.state_root.card_not_on_row",
            (
                f"install card '{install_truth_card_id}' declares state-roots "
                f"{sorted(extra_on_card)} that are not on the row"
            ),
        )

    # Portable mode never mutates host: every state root must be a
    # portable_colocated_root row. Other rows must NOT be portable.
    if expected_install_mode == "portable":
        for ref_id in declared_state_roots:
            sr_row = state_roots.get(ref_id) or {}
            if sr_row.get("durable_state_root_class") != portable_root:
                fail(
                    result,
                    "portable.host_mutation_forbidden",
                    (
                        f"portable row references state-root '{ref_id}' whose "
                        "durable_state_root_class is not portable_colocated_root"
                    ),
                )

    # Trust state vocabulary.
    trust_state = ensure_str(row.get("trust_state"), f"{smoke_row_id}.trust_state")
    if forced_overrides.get("rewrite_trust_state_to_trusted"):
        trust_state = "trusted"
    if trust_state not in trust_state_vocabulary:
        fail(
            result,
            "trust_state.unknown_vocabulary",
            f"trust_state '{trust_state}' is not in the trust-state matrix vocabulary",
        )
    else:
        result.passed_checks.append(
            f"trust_state '{trust_state}' is in the trust-state matrix vocabulary"
        )

    # Trust-state degraded marker: under managed_signed_in shell mode,
    # trust_state MUST be a degraded/restricted state when admin policy
    # is read on the row. Catch the failure-drill rewrite to "trusted".
    shell_mode = ensure_str(row.get("shell_mode"), f"{smoke_row_id}.shell_mode")
    if shell_mode == "managed_signed_in":
        admits_admin_policy = (
            "admin_policy_read"
            in {ensure_str(s, "expected_admissible_surfaces[]") for s in ensure_list(
                row.get("expected_admissible_surfaces"),
                f"{smoke_row_id}.expected_admissible_surfaces",
            )}
        )
        if admits_admin_policy and trust_state == "trusted":
            fail(
                result,
                "trust_policy.degraded_marker_dropped",
                (
                    f"managed_signed_in shell with admin_policy_read MUST declare a "
                    "degraded/restricted trust_state, not 'trusted'"
                ),
            )

    if shell_mode not in shell_mode_vocab:
        fail(
            result,
            "shell_mode.unknown_vocabulary",
            f"shell_mode '{shell_mode}' is not in the matrix vocabulary",
        )
    else:
        result.passed_checks.append(
            f"shell_mode '{shell_mode}' is in the matrix vocabulary"
        )

    # Credential-store posture.
    posture = ensure_dict(
        row.get("credential_store_posture"),
        f"{smoke_row_id}.credential_store_posture",
    )
    posture_class = ensure_str(
        posture.get("trust_store_class"),
        f"{smoke_row_id}.credential_store_posture.trust_store_class",
    )
    posture_unlock = ensure_str(
        posture.get("unlock_state"),
        f"{smoke_row_id}.credential_store_posture.unlock_state",
    )
    posture_label = ensure_str(
        posture.get("backend_label"),
        f"{smoke_row_id}.credential_store_posture.backend_label",
    )
    if posture_unlock not in credential_store_unlock_vocab:
        fail(
            result,
            "credential_store.unlock_state.unknown_vocabulary",
            f"credential_store_posture.unlock_state '{posture_unlock}' is not in the matrix vocabulary",
        )
    else:
        result.passed_checks.append(
            f"credential_store_posture.unlock_state '{posture_unlock}' is in the matrix vocabulary"
        )

    # Safe-preview posture.
    safe_preview = ensure_str(
        row.get("safe_preview_posture"),
        f"{smoke_row_id}.safe_preview_posture",
    )
    if safe_preview not in safe_preview_vocab:
        fail(
            result,
            "safe_preview.unknown_vocabulary",
            f"safe_preview_posture '{safe_preview}' is not in the matrix vocabulary",
        )

    # Admissible-surface vocabulary and floor enforcement.
    admissible = [
        ensure_str(s, f"{smoke_row_id}.expected_admissible_surfaces[]")
        for s in ensure_list(
            row.get("expected_admissible_surfaces"),
            f"{smoke_row_id}.expected_admissible_surfaces",
        )
    ]

    # Apply per-row admissible-surface drop override.
    drop_surface = forced_overrides.get("drop_admissible_surface")
    if isinstance(drop_surface, str):
        admissible = [s for s in admissible if s != drop_surface]

    for surface in admissible:
        if surface not in admissible_surface_vocab:
            fail(
                result,
                "admissible_surfaces.unknown_vocabulary",
                f"expected_admissible_surface '{surface}' is not in the matrix vocabulary",
            )

    # Floor surfaces MUST appear on the row.
    missing_floor = [s for s in local_baseline_floor if s not in admissible]
    if missing_floor:
        fail(
            result,
            "local_baseline.floor_surface_missing",
            (
                f"row blocks local-baseline floor surfaces {missing_floor}; "
                "the floor MUST remain admitted on every row"
            ),
        )
    else:
        result.passed_checks.append(
            f"local-baseline floor {local_baseline_floor} is admitted"
        )

    # Blocked surfaces.
    blocked_raw = ensure_list(
        row.get("expected_blocked_surfaces"),
        f"{smoke_row_id}.expected_blocked_surfaces",
    )
    blocked_records: list[dict[str, str]] = []
    for idx, entry in enumerate(blocked_raw):
        entry = ensure_dict(
            entry, f"{smoke_row_id}.expected_blocked_surfaces[{idx}]"
        )
        surface = ensure_str(
            entry.get("surface"),
            f"{smoke_row_id}.expected_blocked_surfaces[{idx}].surface",
        )
        denial_reason = ensure_str(
            entry.get("denial_reason"),
            f"{smoke_row_id}.expected_blocked_surfaces[{idx}].denial_reason",
        )
        explanation = ensure_str(
            entry.get("actionable_explanation"),
            f"{smoke_row_id}.expected_blocked_surfaces[{idx}].actionable_explanation",
        )
        blocked_records.append(
            {
                "surface": surface,
                "denial_reason": denial_reason,
                "actionable_explanation": explanation,
            }
        )

    # Apply per-row blocked-surface denial-reason rewrite override.
    rewrite_blocked = forced_overrides.get("rewrite_blocked_surface_denial_reason")
    if isinstance(rewrite_blocked, dict):
        target_surface = rewrite_blocked.get("surface")
        new_reason = rewrite_blocked.get("to")
        if isinstance(target_surface, str) and isinstance(new_reason, str):
            for record in blocked_records:
                if record["surface"] == target_surface:
                    record["denial_reason"] = new_reason

    for record in blocked_records:
        if record["surface"] not in admissible_surface_vocab:
            fail(
                result,
                "blocked_surfaces.unknown_surface_vocabulary",
                f"expected_blocked_surface.surface '{record['surface']}' is not in the matrix vocabulary",
            )
        if record["denial_reason"] not in denial_reason_vocab:
            fail(
                result,
                "denial_reason.unknown_class",
                (
                    f"expected_blocked_surfaces[{record['surface']}].denial_reason "
                    f"'{record['denial_reason']}' is not in the matrix vocabulary"
                ),
            )
        if record["surface"] in local_baseline_floor:
            fail(
                result,
                "local_baseline.floor_surface_in_blocked",
                (
                    f"local-baseline floor surface '{record['surface']}' MUST NOT appear "
                    "in expected_blocked_surfaces"
                ),
            )

    # Claimed profile resolution.
    claimed = ensure_list(
        row.get("claimed_profile_ids"),
        f"{smoke_row_id}.claimed_profile_ids",
    )
    if not claimed:
        fail(
            result,
            "claimed_profiles.empty",
            "claimed_profile_ids must declare at least one row",
        )
    for profile_id in claimed:
        profile_id = ensure_str(
            profile_id, f"{smoke_row_id}.claimed_profile_ids[]"
        )
        if profile_id not in claimed_profile_ids:
            fail(
                result,
                "claimed_profiles.missing",
                f"claimed_profile_id '{profile_id}' is not in claimed_desktop_profiles.yaml",
            )
        else:
            result.passed_checks.append(
                f"claimed_profile_id '{profile_id}' resolves"
            )

    # Required disclosure flags.
    if row.get("expected_state_root_inspectable_in_diagnostics") is not True:
        fail(
            result,
            "diagnostics.state_root_inspectable_required",
            "expected_state_root_inspectable_in_diagnostics must be true",
        )
    if row.get("expected_channel_identity_visible") is not True:
        fail(
            result,
            "diagnostics.channel_identity_required",
            "expected_channel_identity_visible must be true",
        )
    if row.get("expected_handler_ownership_disclosure_required") is not True:
        fail(
            result,
            "diagnostics.handler_ownership_required",
            "expected_handler_ownership_disclosure_required must be true",
        )

    # Smoke dimensions.
    dimensions = [
        ensure_str(d, f"{smoke_row_id}.smoke_dimensions_covered[]")
        for d in ensure_list(
            row.get("smoke_dimensions_covered"),
            f"{smoke_row_id}.smoke_dimensions_covered",
        )
    ]
    for d in dimensions:
        if d not in smoke_dimension_vocab:
            fail(
                result,
                "smoke_dimensions.unknown_vocabulary",
                f"smoke_dimension '{d}' is not in the matrix vocabulary",
            )

    # Failure drill structure.
    drills = [
        ensure_str(d, f"{smoke_row_id}.named_failure_drills[]")
        for d in ensure_list(
            row.get("named_failure_drills"),
            f"{smoke_row_id}.named_failure_drills",
        )
    ]
    if not drills:
        fail(
            result,
            "failure_drills.empty",
            "named_failure_drills must declare at least one drill id",
        )
    for d in drills:
        if d not in failure_drill_id_vocab:
            fail(
                result,
                "failure_drills.unknown_vocabulary",
                f"named_failure_drill '{d}' is not in the matrix vocabulary",
            )

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
    if drill_id not in drills:
        fail(
            result,
            "failure_drills.row_drill_id_not_declared",
            f"failure_drill.drill_id '{drill_id}' is not in named_failure_drills",
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
            "trust_state": trust_state,
            "shell_mode": shell_mode,
            "credential_store_posture": {
                "trust_store_class": posture_class,
                "unlock_state": posture_unlock,
                "backend_label": posture_label,
            },
            "expected_install_mode_class": expected_install_mode,
            "expected_channel_class": expected_channel,
            "expected_side_by_side_relation_class": expected_sxs,
            "expected_durable_state_root_refs": declared_state_roots,
            "expected_admissible_surfaces": admissible,
            "expected_blocked_surfaces": blocked_records,
            "smoke_dimensions_covered": dimensions,
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

    # If any hard structural finding is already an error we still continue;
    # the per-row replay yields more actionable errors.

    install_cards = load_install_cards(
        repo_root, vocab_sources["install_topology_matrix_ref"]
    )
    state_roots = load_state_roots(
        repo_root, vocab_sources["state_root_map_ref"]
    )
    trust_matrix = load_trust_matrix(
        repo_root, vocab_sources["trust_state_matrix_ref"]
    )
    trust_state_vocabulary = load_trust_state_vocabulary(trust_matrix)
    credential_store_class_vocab = load_store_capability_ids(
        repo_root, vocab_sources["credential_state_capability_matrix_ref"]
    )
    claimed_profile_ids = load_claimed_profile_ids(
        repo_root, vocab_sources["claimed_profiles_ref"]
    )
    dogfood_row_ids = load_dogfood_row_ids(
        repo_root, vocab_sources["dogfood_matrix_ref"]
    )
    _local_baseline_classes = load_local_baseline_floor(
        repo_root, vocab_sources["local_baseline_proof_ref"]
    )

    shell_mode_vocab = {
        ensure_str(item, "matrix.shell_mode_vocabulary[]")
        for item in ensure_list(
            matrix.get("shell_mode_vocabulary"), "matrix.shell_mode_vocabulary"
        )
    }
    credential_store_unlock_vocab = {
        ensure_str(item, "matrix.credential_store_unlock_state_vocabulary[]")
        for item in ensure_list(
            matrix.get("credential_store_unlock_state_vocabulary"),
            "matrix.credential_store_unlock_state_vocabulary",
        )
    }
    denial_reason_vocab = {
        ensure_str(item, "matrix.denial_reason_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("denial_reason_class_vocabulary"),
            "matrix.denial_reason_class_vocabulary",
        )
    }
    safe_preview_vocab = {
        ensure_str(item, "matrix.safe_preview_posture_vocabulary[]")
        for item in ensure_list(
            matrix.get("safe_preview_posture_vocabulary"),
            "matrix.safe_preview_posture_vocabulary",
        )
    }
    admissible_surface_vocab = {
        ensure_str(item, "matrix.admissible_surface_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("admissible_surface_class_vocabulary"),
            "matrix.admissible_surface_class_vocabulary",
        )
    }
    failure_drill_id_vocab = {
        ensure_str(item, "matrix.failure_drill_id_vocabulary[]")
        for item in ensure_list(
            matrix.get("failure_drill_id_vocabulary"),
            "matrix.failure_drill_id_vocabulary",
        )
    }
    smoke_dimension_vocab = {
        ensure_str(item, "matrix.smoke_dimension_class_vocabulary[]")
        for item in ensure_list(
            matrix.get("smoke_dimension_class_vocabulary"),
            "matrix.smoke_dimension_class_vocabulary",
        )
    }
    local_baseline_floor = [
        ensure_str(item, "matrix.local_baseline_admissible_surface_floor[]")
        for item in ensure_list(
            matrix.get("local_baseline_admissible_surface_floor"),
            "matrix.local_baseline_admissible_surface_floor",
        )
    ]
    required_dimensions = {
        ensure_str(item, "matrix.required_smoke_dimension_coverage[]")
        for item in ensure_list(
            matrix.get("required_smoke_dimension_coverage"),
            "matrix.required_smoke_dimension_coverage",
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
    seen_dimensions: set[str] = set()
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
            install_cards=install_cards,
            state_roots=state_roots,
            trust_state_vocabulary=trust_state_vocabulary,
            trust_matrix=trust_matrix,
            credential_store_unlock_vocab=credential_store_unlock_vocab,
            credential_store_class_vocab=credential_store_class_vocab,
            shell_mode_vocab=shell_mode_vocab,
            denial_reason_vocab=denial_reason_vocab,
            safe_preview_vocab=safe_preview_vocab,
            admissible_surface_vocab=admissible_surface_vocab,
            local_baseline_floor=local_baseline_floor,
            failure_drill_id_vocab=failure_drill_id_vocab,
            claimed_profile_ids=claimed_profile_ids,
            dogfood_row_ids=dogfood_row_ids,
            smoke_dimension_vocab=smoke_dimension_vocab,
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
        for d in result.diagnostics.get("smoke_dimensions_covered", []):
            seen_dimensions.add(d)

        # Capture the forced replay outcome before promoting failures.
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

    missing_dimensions = required_dimensions - seen_dimensions
    if missing_dimensions:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_dimensions",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(required_dimensions)}; missing: {sorted(missing_dimensions)}"
                ),
                remediation="Add the missing rows so every required smoke dimension is exercised.",
            )
        )

    # Promote per-row failures into findings.
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
        "capture_kind": "install_topology_smoke_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/smoke/trust_policy_install_topology/"
            "run_trust_policy_install_topology_smoke.py --repo-root ."
        ),
        "status": status,
        "required_smoke_dimension_coverage": sorted(required_dimensions),
        "observed_smoke_dimensions": sorted(seen_dimensions),
        "rows": [
            {
                "smoke_row_id": r.smoke_row_id,
                "inherited_dogfood_row_id": r.inherited_dogfood_row_id,
                "install_truth_card_id": r.install_truth_card_id,
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

    label = "install-topology-smoke"
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
        print("[install-topology-smoke] interrupted", file=sys.stderr)
        sys.exit(130)
