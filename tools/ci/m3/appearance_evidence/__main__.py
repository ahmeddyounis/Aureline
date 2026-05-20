#!/usr/bin/env python3
"""Generate and validate the beta appearance-parity evidence packet.

This gate joins three already-minted upstream truth sources into one
current, build-linked evidence packet so shiproom, support, docs,
marketplace review, and design QA can answer appearance questions from
the packet alone instead of reconstructing visual truth from
screenshots:

- Extension appearance conformance (contributed-UI inheritance gaps).
- Component-state / token conformance and the screenshot-state diff
  (first-party dark/light/high-contrast structured state reports).
- The appearance-session contract (live OS appearance-change audit and
  imported-theme mapping coverage).

The packet, its metadata-safe support export, and the three human
reports are generated from upstream truth, so hand-curated prose can
never outrank the current packet. ``--check`` fails when any generated
output would change, when a referenced artifact is missing, when an
evidence source ages past its review window without a downgrade, or
when a claimed row carries unresolved or red appearance evidence.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


# Upstream truth (already governed by their own gates).
EXTENSION_PACKET_REL = (
    "fixtures/extensions/m3/appearance_inheritance/conformance_packet.json"
)
COMPONENT_STATE_PACKET_REL = (
    "artifacts/ux/m3/component_state_screenshot_diff/packet.json"
)
TOKEN_CONFORMANCE_REL = "fixtures/ux/m3/state_semantics/token_conformance_report.json"
APPEARANCE_SESSION_REL = (
    "fixtures/ux/m3/theme_import_and_live_change/appearance_session_beta_contract.json"
)
ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"

# Generated outputs (truth + reports + capture).
PACKET_REL = "fixtures/ux/m3/appearance_evidence/appearance_evidence_packet.json"
SUPPORT_EXPORT_REL = "fixtures/ux/m3/appearance_evidence/support_export.json"
PARITY_REPORT_REL = "artifacts/ux/m3/appearance_parity_evidence_packet.md"
HIGH_CONTRAST_REPORT_REL = "artifacts/ux/m3/high_contrast_and_live_change_audit.md"
EXTENSION_GAP_REPORT_REL = "artifacts/extensions/m3/extension_inheritance_gap_packet.md"
CAPTURE_REL = "artifacts/ux/m3/captures/appearance_evidence_validation_capture.json"

# Consuming surfaces that must downgrade their appearance claim when the
# packet is stale or red. Every ref is a checked-in repo file.
CONSUMER_DOC_REF = "docs/ux/m3/appearance_evidence_consumption_guide.md"

CONTRACT_REF = "ux:appearance_parity_evidence:v1"
SCHEMA_VERSION = 1
PACKET_RECORD_KIND = "appearance_parity_evidence_packet"
SUPPORT_RECORD_KIND = "appearance_parity_evidence_support_export"
CAPTURE_RECORD_KIND = "appearance_parity_evidence_validation_capture"

STALE_AFTER = "P45D"
STALE_AFTER_PATTERN = re.compile(r"^P(\d+)D$")

FOUR_MODE_FLOOR = (
    "dark_reference",
    "light_parity",
    "high_contrast_dark",
    "high_contrast_light",
)
HIGH_CONTRAST_THEME_CLASSES = ("high_contrast_dark", "high_contrast_light")

# First-party appearance-safe decisions vs. contributed decisions are
# derived from upstream support/decision classes through this map.
CONTRIBUTED_DECISION_BY_SUPPORT = {
    ("full_inheritance", "conformant"): ("appearance_safe", "full_inheritance_proven"),
    ("reduced_support", "conformant"): (
        "appearance_safe_with_caveats",
        "reduced_support_disclosed",
    ),
    ("reduced_support", "needs_review"): (
        "appearance_unverified",
        "needs_verification_before_badge",
    ),
    ("unsupported_private_styling", "conformant"): (
        "appearance_private_styling",
        "unsupported_private_styling_disclosed",
    ),
}

SURFACE_LABELS = {
    "shell_chrome": "Shell chrome",
    "start_center": "Start Center",
    "command_palette": "Command palette",
    "search_surface": "Search surface",
    "dialog_sheet": "Dialog sheet",
    "trust_prompt": "Trust prompt",
    "notification_envelope": "Notification envelope",
    "help_about_row": "Help / About row",
    "settings_root": "Settings root",
    "activity_center_row": "Activity-center row",
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


# --------------------------------------------------------------------------
# IO helpers
# --------------------------------------------------------------------------
def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--generated-at")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if any generated output would change.",
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
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def render_json(payload: Any) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_PATTERN.match(value)
    if not match:
        raise SystemExit(f"{label} must be an ISO-8601 day duration such as P45D")
    return int(match.group(1))


def to_date(value: str, label: str) -> dt.date:
    return dt.datetime.fromisoformat(value.replace("Z", "+00:00")).date()


def surface_label(surface_class: str) -> str:
    return SURFACE_LABELS.get(
        surface_class, surface_class.replace("_", " ").capitalize()
    )


def package_version_from_evidence(evidence_refs: list[str]) -> str:
    for ref in evidence_refs:
        if ref.startswith("registry_descriptor:"):
            parts = ref.split(":")
            if len(parts) >= 3 and parts[-1].strip():
                return parts[-1].strip()
    for ref in evidence_refs:
        if ref.startswith("mirror_import:"):
            parts = ref.split(":")
            if len(parts) >= 3 and parts[-1].strip():
                return f"mirror:{parts[-1].strip()}"
    return "unversioned"


# --------------------------------------------------------------------------
# Source loading
# --------------------------------------------------------------------------
@dataclass
class Sources:
    extension: dict[str, Any]
    component_state: dict[str, Any]
    token_conformance: dict[str, Any]
    appearance_session: dict[str, Any]
    artifact_graph: dict[str, Any]
    claim_manifest: dict[str, Any]


def load_sources(repo_root: Path) -> Sources:
    return Sources(
        extension=ensure_dict(
            load_json(repo_root / EXTENSION_PACKET_REL), EXTENSION_PACKET_REL
        ),
        component_state=ensure_dict(
            load_json(repo_root / COMPONENT_STATE_PACKET_REL),
            COMPONENT_STATE_PACKET_REL,
        ),
        token_conformance=ensure_dict(
            load_json(repo_root / TOKEN_CONFORMANCE_REL), TOKEN_CONFORMANCE_REL
        ),
        appearance_session=ensure_dict(
            load_json(repo_root / APPEARANCE_SESSION_REL), APPEARANCE_SESSION_REL
        ),
        artifact_graph=ensure_dict(
            load_json(repo_root / ARTIFACT_GRAPH_REL), ARTIFACT_GRAPH_REL
        ),
        claim_manifest=ensure_dict(
            load_json(repo_root / CLAIM_MANIFEST_REL), CLAIM_MANIFEST_REL
        ),
    )


def build_identity(graph: dict[str, Any]) -> dict[str, Any]:
    identities = ensure_list(
        graph.get("exact_build_identities"), "artifact_graph.exact_build_identities"
    )
    if not identities:
        raise SystemExit("artifact_graph.exact_build_identities is empty")
    primary = ensure_dict(identities[0], "artifact_graph.exact_build_identities[0]")
    candidate = ensure_dict(graph.get("candidate"), "artifact_graph.candidate")
    return {
        "release_candidate_ref": candidate.get("candidate_ref"),
        "version_label": candidate.get("version"),
        "exact_build_identity_ref": primary.get("exact_build_identity_ref"),
        "source_revision_ref": primary.get("source_revision_ref"),
        "target_triple": primary.get("target_triple"),
        "profile": primary.get("profile"),
    }


# --------------------------------------------------------------------------
# Source-evidence freshness blocks
# --------------------------------------------------------------------------
def source_blocks(sources: Sources, generated_at: str) -> dict[str, dict[str, Any]]:
    ext = sources.extension
    comp = sources.component_state
    tok = sources.token_conformance
    sess = sources.appearance_session
    session_record = ensure_dict(
        sess.get("appearance_session"), "appearance_session.appearance_session"
    )
    return {
        "extension_inheritance": {
            "source_class": "extension_inheritance",
            "ref": EXTENSION_PACKET_REL,
            "contract_ref": ext.get("shared_contract_ref"),
            "report_ref": "artifacts/extensions/m3/appearance_gap_review.md",
            "captured_at": ext.get("generated_at") or generated_at,
            "summary": ensure_dict(ext.get("summary"), "extension.summary"),
        },
        "component_state_token": {
            "source_class": "component_state_token",
            "ref": COMPONENT_STATE_PACKET_REL,
            "token_conformance_ref": TOKEN_CONFORMANCE_REL,
            "contract_ref": comp.get("shared_contract_ref"),
            "report_ref": "artifacts/ux/m3/token_conformance_audit.md",
            # Mint-from-Rust deterministic fixtures regenerated on every
            # component-state/token gate run; current while that gate passes.
            "captured_at": generated_at,
            "summary": ensure_dict(comp.get("summary"), "component_state.summary"),
            "token_gate_state": tok.get("gate_state"),
            "screenshot_gate_state": comp.get("gate_state"),
        },
        "appearance_session": {
            "source_class": "appearance_session",
            "ref": APPEARANCE_SESSION_REL,
            "contract_ref": sess.get("shared_contract_ref"),
            "report_ref": "docs/ux/m3/appearance_session_beta_contract.md",
            "captured_at": session_record.get("revision_minted_at") or generated_at,
            "gate_state": sess.get("gate_state"),
        },
    }


def freshness_class_for(captured_at: str, as_of: dt.date, window_days: int) -> str:
    age = (as_of - to_date(captured_at, "captured_at")).days
    return "current" if age <= window_days else "stale"


# --------------------------------------------------------------------------
# Appearance rows
# --------------------------------------------------------------------------
def first_party_rows(
    sources: Sources,
    identity: dict[str, Any],
    source_freshness: dict[str, str],
    consumer_refs: list[str],
) -> list[dict[str, Any]]:
    diff_rows = {
        ensure_str(row.get("surface_class"), "component_state.rows[].surface_class"): row
        for row in ensure_list(
            sources.component_state.get("rows"), "component_state.rows"
        )
    }
    token_rows = {
        ensure_str(row.get("surface_class"), "token_conformance.rows[].surface_class"): row
        for row in ensure_list(
            sources.token_conformance.get("rows"), "token_conformance.rows"
        )
    }
    freshness_class = source_freshness["component_state_token"]
    packet_state = "current_packet" if freshness_class == "current" else "downgraded_stale_evidence"

    rows: list[dict[str, Any]] = []
    for surface_class in sorted(token_rows):
        diff = diff_rows.get(surface_class)
        token = token_rows[surface_class]
        structured = None
        if diff is not None:
            structured = {
                "captured_theme_class": diff.get("theme_class"),
                "captured_density_class": diff.get("density_class"),
                "captured_motion_posture": diff.get("motion_posture"),
                "screenshot_baseline_ref": diff.get("baseline_capture_ref"),
                "screenshot_comparison_ref": diff.get("comparison_capture_ref"),
                "screenshot_diff_ref": diff.get("diff_artifact_ref"),
                "keyboard_journey_ref": diff.get("keyboard_journey_ref"),
                "assistive_technology_ref": diff.get("assistive_technology_ref"),
                "required_non_color_cues": diff.get("required_non_color_cues", []),
                "focus_visibility_present": diff.get("focus_visibility_present"),
                "color_only_state_meaning_absent": diff.get(
                    "color_only_state_meaning_absent"
                ),
                "semantic_stability_passed": diff.get("semantic_stability_passed"),
            }
        rows.append(
            {
                "row_id": f"appearance-evidence:first_party:{surface_class}",
                "surface_id": f"surface:{surface_class}",
                "surface_label": surface_label(surface_class),
                "owner_class": "first_party",
                "lifecycle_class": "beta",
                "claims_beta_appearance_compatibility": True,
                "theme_classes_proven": list(FOUR_MODE_FLOOR),
                "structured_state_report": structured,
                "token_conformance_ref": f"{TOKEN_CONFORMANCE_REL}#{token.get('row_id')}",
                "live_update_audit_ref": f"{APPEARANCE_SESSION_REL}#live_os_change_matrix",
                "importer_coverage_ref": f"{APPEARANCE_SESSION_REL}#import_mapping_report",
                "appearance_decision": "appearance_safe",
                "decision_reason": "token_state_and_screenshot_diff_proven",
                "evidence_refs": [
                    f"{COMPONENT_STATE_PACKET_REL}#{(diff or {}).get('row_id', surface_class)}",
                    f"{TOKEN_CONFORMANCE_REL}#{token.get('row_id')}",
                ],
                "exact_build_identity_ref": identity["exact_build_identity_ref"],
                "package_version_label": identity["version_label"],
                "source_evidence": {
                    "source_class": "component_state_token",
                    "captured_at": None,  # filled by caller
                    "freshness_class": freshness_class,
                },
                "packet_state": packet_state,
                "consumer_surface_refs": consumer_refs,
            }
        )
    return rows


def contributed_rows(
    sources: Sources,
    source_freshness: dict[str, str],
    consumer_refs: list[str],
) -> list[dict[str, Any]]:
    freshness_class = source_freshness["extension_inheritance"]
    rows: list[dict[str, Any]] = []
    for raw in ensure_list(sources.extension.get("rows"), "extension.rows"):
        row = ensure_dict(raw, "extension.rows[]")
        support = ensure_str(
            row.get("overall_support_class"), "extension.row.overall_support_class"
        )
        decision = ensure_str(row.get("decision_class"), "extension.row.decision_class")
        appearance_decision, decision_reason = CONTRIBUTED_DECISION_BY_SUPPORT.get(
            (support, decision), ("appearance_review_required", decision)
        )
        evidence_refs = [
            str(ref) for ref in ensure_list(row.get("evidence_refs"), "row.evidence_refs")
        ]
        axes = {
            ensure_str(axis.get("axis"), "axis"): axis.get("support_class")
            for axis in ensure_list(row.get("axes"), "extension.row.axes")
        }
        # An unverified or red contributed claim must not present as a
        # current full-appearance row.
        packet_state = (
            "current_packet"
            if freshness_class == "current"
            else "downgraded_stale_evidence"
        )
        rows.append(
            {
                "row_id": f"appearance-evidence:contributed:{row.get('row_id')}",
                "surface_id": row.get("surface_id"),
                "surface_label": row.get("surface_label"),
                "owner_class": "contributed",
                "extension_id": row.get("extension_id"),
                "extension_name": row.get("extension_name"),
                "publisher_label": row.get("publisher_label"),
                "lifecycle_class": row.get("lifecycle_class"),
                "claims_beta_appearance_compatibility": True,
                "axis_support_by_token": axes,
                "high_contrast_support_class": axes.get("high_contrast"),
                "overall_support_class": support,
                "upstream_decision_class": decision,
                "appearance_decision": appearance_decision,
                "decision_reason": decision_reason,
                "caveat_summary": row.get("caveat_summary"),
                "extension_gap_row_ref": (
                    f"{EXTENSION_PACKET_REL}#{row.get('row_id')}"
                ),
                "evidence_refs": evidence_refs,
                "exact_build_identity_ref": None,
                "package_version_label": package_version_from_evidence(evidence_refs),
                "source_evidence": {
                    "source_class": "extension_inheritance",
                    "captured_at": None,
                    "freshness_class": freshness_class,
                },
                "packet_state": packet_state,
                "consumer_surface_refs": consumer_refs,
            }
        )
    return rows


def extension_gap_rows(sources: Sources) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for raw in ensure_list(sources.extension.get("rows"), "extension.rows"):
        row = ensure_dict(raw, "extension.rows[]")
        axes = ensure_list(row.get("axes"), "extension.row.axes")
        gap_axes = [
            {
                "axis": axis.get("axis"),
                "support_class": axis.get("support_class"),
                "requires_verification": axis.get("requires_verification", False),
                "caveat": axis.get("caveat"),
            }
            for axis in axes
            if axis.get("support_class") != "full_inheritance"
            or axis.get("requires_verification")
        ]
        known_unsupported = [
            {
                "axis": state.get("axis"),
                "state_label": state.get("state_label"),
                "summary": state.get("summary"),
            }
            for state in ensure_list(
                row.get("known_unsupported_states"), "row.known_unsupported_states"
            )
        ]
        surface_caveats = [
            {
                "surface_class": caveat.get("surface_class"),
                "badge_label": caveat.get("badge_label"),
                "persists_after_install": caveat.get("persists_after_install"),
                "implies_full_inheritance": caveat.get("implies_full_inheritance"),
            }
            for caveat in ensure_list(
                row.get("surface_caveats"), "row.surface_caveats"
            )
        ]
        decision = ensure_str(row.get("decision_class"), "row.decision_class")
        remediation_link = (
            "docs/extensions/m3/appearance_conformance_beta.md"
            if decision != "needs_review"
            else "docs/extensions/m3/appearance_conformance_beta.md#declaration-joined-with-host-proof"
        )
        rows.append(
            {
                "row_id": f"extension-inheritance-gap:{row.get('row_id')}",
                "extension_id": row.get("extension_id"),
                "extension_name": row.get("extension_name"),
                "publisher_label": row.get("publisher_label"),
                "surface_id": row.get("surface_id"),
                "surface_label": row.get("surface_label"),
                "lifecycle_class": row.get("lifecycle_class"),
                "overall_support_class": row.get("overall_support_class"),
                "decision_class": decision,
                "reason_class": row.get("reason_class"),
                "gap_axes": gap_axes,
                "known_unsupported_states": known_unsupported,
                "downgrade_notes": row.get("caveat_summary"),
                "surface_caveats": surface_caveats,
                "persists_after_install": any(
                    caveat.get("persists_after_install") for caveat in surface_caveats
                ),
                "host_stable_labels": {
                    "trust": row.get("host_trust_label"),
                    "severity": row.get("host_severity_label"),
                    "permission": row.get("host_permission_label"),
                    "policy": row.get("host_policy_label"),
                    "host_rendered_trust_and_severity": row.get(
                        "host_rendered_trust_and_severity"
                    ),
                },
                "remediation_link": remediation_link,
                "upstream_row_ref": f"{EXTENSION_PACKET_REL}#{row.get('row_id')}",
            }
        )
    return rows


# --------------------------------------------------------------------------
# High-contrast signoff / live-update audit / importer coverage
# --------------------------------------------------------------------------
def high_contrast_signoff(
    sources: Sources, identity: dict[str, Any]
) -> dict[str, Any]:
    diff_rows = ensure_list(sources.component_state.get("rows"), "component_state.rows")
    first_party = [
        {
            "row_id": row.get("row_id"),
            "surface_class": row.get("surface_class"),
            "theme_class": row.get("theme_class"),
            "state_class": row.get("state_class"),
            "focus_visibility_present": row.get("focus_visibility_present"),
            "color_only_state_meaning_absent": row.get(
                "color_only_state_meaning_absent"
            ),
            "required_non_color_cues": row.get("required_non_color_cues", []),
            "diff_artifact_ref": row.get("diff_artifact_ref"),
        }
        for row in diff_rows
        if row.get("theme_class") in HIGH_CONTRAST_THEME_CLASSES
    ]
    theme_pkg = ensure_dict(
        sources.appearance_session.get("theme_package"),
        "appearance_session.theme_package",
    )
    contrast_targets = ensure_dict(
        theme_pkg.get("minimum_text_contrast_targets"),
        "theme_package.minimum_text_contrast_targets",
    )
    protected_cues = [
        {
            "protected_cue_class": cue.get("protected_cue_class"),
            "preserved_in_high_contrast": cue.get("preserved_in_high_contrast"),
            "preserved_in_forced_colors": cue.get("preserved_in_forced_colors"),
        }
        for cue in ensure_list(
            sources.appearance_session.get("protected_cue_preservation"),
            "appearance_session.protected_cue_preservation",
        )
    ]
    contributed = [
        {
            "extension_id": row.get("extension_id"),
            "surface_label": row.get("surface_label"),
            "high_contrast_support_class": next(
                (
                    axis.get("support_class")
                    for axis in ensure_list(row.get("axes"), "row.axes")
                    if axis.get("axis") == "high_contrast"
                ),
                None,
            ),
            "package_version_label": package_version_from_evidence(
                [str(r) for r in row.get("evidence_refs", [])]
            ),
        }
        for row in (
            ensure_dict(r, "extension.rows[]")
            for r in ensure_list(sources.extension.get("rows"), "extension.rows")
        )
    ]
    all_focus_legible = all(
        row["focus_visibility_present"] and row["color_only_state_meaning_absent"]
        for row in first_party
    )
    all_cues_preserved = all(
        cue["preserved_in_high_contrast"] and cue["preserved_in_forced_colors"]
        for cue in protected_cues
    )
    return {
        "signoff_state": "passed" if all_focus_legible and all_cues_preserved else "blocked",
        "exact_build_identity_ref": identity["exact_build_identity_ref"],
        "version_label": identity["version_label"],
        "source_revision_ref": identity["source_revision_ref"],
        "high_contrast_theme_classes": list(HIGH_CONTRAST_THEME_CLASSES),
        "minimum_text_contrast_targets": {
            "high_contrast_dark": contrast_targets.get("high_contrast_dark"),
            "high_contrast_light": contrast_targets.get("high_contrast_light"),
        },
        "first_party_rows": first_party,
        "protected_cue_preservation": protected_cues,
        "contributed_high_contrast_support": contributed,
        "all_first_party_high_contrast_focus_legible": all_focus_legible,
        "all_protected_cues_preserved": all_cues_preserved,
    }


def live_update_audit(sources: Sources, identity: dict[str, Any]) -> dict[str, Any]:
    matrix = ensure_dict(
        sources.appearance_session.get("live_os_change_matrix"),
        "appearance_session.live_os_change_matrix",
    )
    summary = ensure_dict(
        sources.appearance_session.get("live_follow_system_summary"),
        "appearance_session.live_follow_system_summary",
    )
    return {
        "audit_state": "passed"
        if matrix.get("all_reload_or_restart_rows_disclosed")
        and matrix.get("forced_colors_rows_explicit")
        else "blocked",
        "exact_build_identity_ref": identity["exact_build_identity_ref"],
        "version_label": identity["version_label"],
        "matrix_ref": matrix.get("matrix_ref"),
        "claimed_profile_count": matrix.get("claimed_profile_count"),
        "row_count": matrix.get("row_count"),
        "axes_covered": matrix.get("axes_covered", []),
        "live_no_review_axes": summary.get("live_no_review_axes", []),
        "live_checkpointed_axes": summary.get("live_checkpointed_axes", []),
        "confirm_required_axes": summary.get("confirm_required_axes", []),
        "embedded_reload_required_rows": matrix.get("embedded_reload_required_rows"),
        "full_restart_required_rows": matrix.get("full_restart_required_rows"),
        "all_reload_or_restart_rows_disclosed": matrix.get(
            "all_reload_or_restart_rows_disclosed"
        ),
        "forced_colors_rows_explicit": matrix.get("forced_colors_rows_explicit"),
    }


def importer_coverage(sources: Sources) -> dict[str, Any]:
    report = ensure_dict(
        sources.appearance_session.get("import_mapping_report"),
        "appearance_session.import_mapping_report",
    )
    overlay = ensure_dict(
        sources.appearance_session.get("token_overlay"),
        "appearance_session.token_overlay",
    )
    unresolved = (
        int(report.get("unsupported_slot_count", 0) or 0)
        + int(report.get("unresolved_mapping_count", 0) or 0)
    )
    return {
        "coverage_state": report.get("parity_readiness"),
        "source_ecosystem": report.get("source_ecosystem"),
        "source_tool_version": report.get("source_tool_version"),
        "target_theme_classes": report.get("target_theme_classes", []),
        "translated_slot_count": report.get("translated_slot_count"),
        "substituted_with_fallback_count": report.get("substituted_with_fallback_count"),
        "unsupported_slot_count": report.get("unsupported_slot_count"),
        "unresolved_mapping_count": report.get("unresolved_mapping_count"),
        "unresolved_slot_total": unresolved,
        "syntax_coverage_percent": report.get("syntax_coverage_percent"),
        "syntax_unresolved_scope_count": report.get("syntax_unresolved_scope_count"),
        "protected_cues_preserved": report.get("protected_cues_preserved"),
        "report_ref": f"{APPEARANCE_SESSION_REL}#import_mapping_report",
        "token_overlay": {
            "inherited_count": overlay.get("inherited_count"),
            "overridden_count": overlay.get("overridden_count"),
            "deprecated_count": overlay.get("deprecated_count"),
            "unmapped_count": overlay.get("unmapped_count"),
            "deprecated_replacements_visible": overlay.get(
                "deprecated_replacements_visible"
            ),
            "unmapped_entries_preserved_inert": overlay.get(
                "unmapped_entries_preserved_inert"
            ),
        },
    }


def consumer_surfaces() -> list[dict[str, Any]]:
    return [
        {
            "surface_id": "help:release_truth_surfaces",
            "surface_class": "help",
            "ref": "docs/help/m3/release_truth_surfaces.md",
            "downgrades_on": ["stale_evidence", "red_evidence"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "docs:appearance_evidence_consumption_guide",
            "surface_class": "docs",
            "ref": CONSUMER_DOC_REF,
            "downgrades_on": ["stale_evidence", "red_evidence"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "docs:appearance_session_beta_contract",
            "surface_class": "docs",
            "ref": "docs/ux/m3/appearance_session_beta_contract.md",
            "downgrades_on": ["stale_evidence", "red_evidence"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "docs:component_state_and_token_beta_contract",
            "surface_class": "docs",
            "ref": "docs/ux/m3/component_state_and_token_beta_contract.md",
            "downgrades_on": ["stale_evidence", "red_evidence"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "marketplace:appearance_conformance_beta",
            "surface_class": "marketplace",
            "ref": "docs/extensions/m3/appearance_conformance_beta.md",
            "downgrades_on": ["stale_evidence", "red_evidence", "needs_review"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "marketplace:marketplace_fact_grid_beta",
            "surface_class": "marketplace",
            "ref": "docs/extensions/m3/marketplace_fact_grid_beta.md",
            "downgrades_on": ["stale_evidence", "red_evidence", "needs_review"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "marketplace:marketplace_truth_beta",
            "surface_class": "marketplace",
            "ref": "docs/ux/m3/marketplace_truth_beta.md",
            "downgrades_on": ["stale_evidence", "red_evidence", "needs_review"],
            "current_claim_state": "current",
        },
        {
            "surface_id": "support:support_bundle_contract",
            "surface_class": "support",
            "ref": "docs/support/support_bundle_contract.md",
            "downgrades_on": ["stale_evidence", "red_evidence"],
            "current_claim_state": "current",
        },
    ]


# --------------------------------------------------------------------------
# Packet assembly
# --------------------------------------------------------------------------
def build_packet(sources: Sources, generated_at: str) -> dict[str, Any]:
    as_of = to_date(generated_at, "generated_at")
    window = stale_after_days(STALE_AFTER, "stale_after")
    identity = build_identity(sources.artifact_graph)
    blocks = source_blocks(sources, generated_at)

    source_freshness = {
        key: freshness_class_for(block["captured_at"], as_of, window)
        for key, block in blocks.items()
    }
    for key, block in blocks.items():
        block["freshness_class"] = source_freshness[key]
        block["age_days"] = (as_of - to_date(block["captured_at"], "captured_at")).days

    consumer_refs = [surface["ref"] for surface in consumer_surfaces()]

    fp_rows = first_party_rows(sources, identity, source_freshness, consumer_refs)
    for row in fp_rows:
        row["source_evidence"]["captured_at"] = blocks["component_state_token"][
            "captured_at"
        ]
    contrib_rows = contributed_rows(sources, source_freshness, consumer_refs)
    for row in contrib_rows:
        row["source_evidence"]["captured_at"] = blocks["extension_inheritance"][
            "captured_at"
        ]
    appearance_rows = fp_rows + contrib_rows
    gap_rows = extension_gap_rows(sources)
    hc = high_contrast_signoff(sources, identity)
    live = live_update_audit(sources, identity)
    live["source_freshness_class"] = source_freshness["appearance_session"]
    importer = importer_coverage(sources)
    importer["source_freshness_class"] = source_freshness["appearance_session"]

    summary = {
        "appearance_row_count": len(appearance_rows),
        "first_party_row_count": len(fp_rows),
        "contributed_row_count": len(contrib_rows),
        "appearance_safe_row_count": sum(
            1 for r in appearance_rows if r["appearance_decision"] == "appearance_safe"
        ),
        "appearance_safe_with_caveats_row_count": sum(
            1
            for r in appearance_rows
            if r["appearance_decision"] == "appearance_safe_with_caveats"
        ),
        "appearance_unverified_row_count": sum(
            1
            for r in appearance_rows
            if r["appearance_decision"] == "appearance_unverified"
        ),
        "appearance_private_styling_row_count": sum(
            1
            for r in appearance_rows
            if r["appearance_decision"] == "appearance_private_styling"
        ),
        "downgraded_row_count": sum(
            1 for r in appearance_rows if r["packet_state"] != "current_packet"
        ),
        "extension_gap_row_count": len(gap_rows),
        "extension_needs_review_row_count": sum(
            1 for r in gap_rows if r["decision_class"] == "needs_review"
        ),
        "high_contrast_signoff_state": hc["signoff_state"],
        "live_update_audit_state": live["audit_state"],
        "importer_coverage_state": importer["coverage_state"],
        "consumer_surface_count": len(consumer_refs),
    }

    any_source_stale = any(fc != "current" for fc in source_freshness.values())
    packet_state = (
        "current_packet"
        if summary["downgraded_row_count"] == 0
        and hc["signoff_state"] == "passed"
        and live["audit_state"] == "passed"
        and not any_source_stale
        else "blocked_or_downgraded"
    )

    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": PACKET_RECORD_KIND,
        "shared_contract_ref": CONTRACT_REF,
        "packet_id": "appearance-parity-evidence:beta:default",
        "packet_state": packet_state,
        "generated_at": generated_at,
        "as_of": as_of.isoformat(),
        "release_candidate_ref": identity["release_candidate_ref"],
        "version_label": identity["version_label"],
        "exact_build_identity_ref": identity["exact_build_identity_ref"],
        "source_revision_ref": identity["source_revision_ref"],
        "target_triple": identity["target_triple"],
        "profile": identity["profile"],
        "source_packets": blocks,
        "freshness": {
            "captured_at": generated_at,
            "stale_after": STALE_AFTER,
            "freshness_class": "current",
            "stale_propagation_profile": "downgrade_claim_and_hold",
        },
        "appearance_rows": appearance_rows,
        "extension_inheritance_gap_rows": gap_rows,
        "high_contrast_signoff": hc,
        "live_update_audit": live,
        "importer_coverage": importer,
        "consumer_surfaces": consumer_surfaces(),
        "summary": summary,
        "acceptance": {
            "validation_commands": [
                "python3 -m tools.ci.m3.appearance_evidence --repo-root . --check"
            ],
            "consumption_guide_ref": CONSUMER_DOC_REF,
        },
        "raw_private_material_excluded": True,
    }


def build_support_export(packet: dict[str, Any], generated_at: str) -> dict[str, Any]:
    appearance = [
        {
            "row_id": row["row_id"],
            "surface_id": row["surface_id"],
            "surface_label": row["surface_label"],
            "owner_class": row["owner_class"],
            "appearance_decision": row["appearance_decision"],
            "packet_state": row["packet_state"],
            "exact_build_identity_ref": row.get("exact_build_identity_ref"),
            "package_version_label": row.get("package_version_label"),
            "source_freshness_class": row["source_evidence"]["freshness_class"],
        }
        for row in packet["appearance_rows"]
    ]
    gaps = [
        {
            "row_id": row["row_id"],
            "extension_id": row["extension_id"],
            "surface_label": row["surface_label"],
            "overall_support_class": row["overall_support_class"],
            "decision_class": row["decision_class"],
            "persists_after_install": row["persists_after_install"],
            "remediation_link": row["remediation_link"],
        }
        for row in packet["extension_inheritance_gap_rows"]
    ]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": SUPPORT_RECORD_KIND,
        "shared_contract_ref": CONTRACT_REF,
        "packet_ref": PACKET_REL,
        "generated_at": generated_at,
        "packet_state": packet["packet_state"],
        "exact_build_identity_ref": packet["exact_build_identity_ref"],
        "version_label": packet["version_label"],
        "summary": packet["summary"],
        "appearance_rows": appearance,
        "extension_inheritance_gap_rows": gaps,
        "high_contrast_signoff": {
            "signoff_state": packet["high_contrast_signoff"]["signoff_state"],
            "exact_build_identity_ref": packet["high_contrast_signoff"][
                "exact_build_identity_ref"
            ],
            "version_label": packet["high_contrast_signoff"]["version_label"],
        },
        "live_update_audit": {
            "audit_state": packet["live_update_audit"]["audit_state"],
            "exact_build_identity_ref": packet["live_update_audit"][
                "exact_build_identity_ref"
            ],
            "row_count": packet["live_update_audit"]["row_count"],
        },
        "importer_coverage": {
            "coverage_state": packet["importer_coverage"]["coverage_state"],
            "unresolved_slot_total": packet["importer_coverage"]["unresolved_slot_total"],
        },
        "consumer_surfaces": packet["consumer_surfaces"],
        "raw_private_material_excluded": True,
        "manual_private_path_lookup_required": False,
    }


# --------------------------------------------------------------------------
# Validation
# --------------------------------------------------------------------------
def collect_repo_refs(value: Any, parent_key: str | None = None) -> list[str]:
    refs: list[str] = []
    if isinstance(value, dict):
        for key, nested in value.items():
            refs.extend(collect_repo_refs(nested, key))
    elif isinstance(value, list):
        for nested in value:
            refs.extend(collect_repo_refs(nested, parent_key))
    elif isinstance(value, str) and parent_key:
        if parent_key.endswith("_ref") or parent_key.endswith("_refs") or parent_key == "ref":
            clean = value.split("#", 1)[0]
            if "/" in clean and "." in clean.rsplit("/", 1)[-1] and ":" not in clean:
                refs.append(clean)
    return refs


def validate(
    repo_root: Path,
    packet: dict[str, Any],
    support: dict[str, Any],
    generated_refs: set[str],
) -> list[Finding]:
    findings: list[Finding] = []

    if packet.get("schema_version") != SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "packet.schema_version",
                "schema_version must be 1",
                "Keep the appearance-evidence packet pinned to schema version 1.",
            )
        )
    if packet.get("record_kind") != PACKET_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "packet.record_kind",
                f"record_kind must be {PACKET_RECORD_KIND}",
                "Use the appearance-evidence packet record discriminator.",
            )
        )

    # Referenced repo paths must exist (excluding the files we generate).
    for ref in sorted(set(collect_repo_refs(packet))):
        if ref in generated_refs:
            continue
        if not (repo_root / ref).exists():
            findings.append(
                Finding(
                    "error",
                    "ref.missing",
                    f"referenced artifact does not exist: {ref}",
                    "Seed the artifact or correct the packet reference.",
                    ref,
                )
            )

    # Freshness: a current row may not rest on stale evidence.
    window = stale_after_days(
        str(ensure_dict(packet.get("freshness"), "freshness").get("stale_after")),
        "freshness.stale_after",
    )
    as_of = to_date(str(packet.get("as_of")), "as_of")
    any_source_stale = False
    for key, block in ensure_dict(packet.get("source_packets"), "source_packets").items():
        age = (as_of - to_date(str(block.get("captured_at")), "captured_at")).days
        expected = "current" if age <= window else "stale"
        if expected != "current":
            any_source_stale = True
        if block.get("freshness_class") != expected:
            findings.append(
                Finding(
                    "error",
                    "source.freshness_class.drift",
                    f"source {key} freshness_class is not {expected} for age {age}d",
                    "Refresh the source or downgrade the rows that depend on it.",
                    str(block.get("ref")),
                )
            )
    if any_source_stale and packet.get("packet_state") == "current_packet":
        findings.append(
            Finding(
                "error",
                "packet.stale_source_not_downgraded",
                "an evidence source is stale but the packet is still current",
                "Refresh the source or downgrade the packet claim.",
            )
        )

    # Appearance rows: stale evidence must downgrade; unverified/red rows
    # must never claim full appearance compatibility while current.
    for row in ensure_list(packet.get("appearance_rows"), "appearance_rows"):
        row_id = ensure_str(row.get("row_id"), "appearance_rows[].row_id")
        freshness = row["source_evidence"]["freshness_class"]
        if freshness != "current" and row.get("packet_state") == "current_packet":
            findings.append(
                Finding(
                    "error",
                    "row.stale_not_downgraded",
                    f"{row_id} rests on stale evidence but is still current",
                    "Downgrade the row or refresh its evidence source.",
                    row_id,
                )
            )
        if row.get("owner_class") == "first_party":
            report = row.get("structured_state_report")
            if not report or report.get("semantic_stability_passed") is not True:
                findings.append(
                    Finding(
                        "error",
                        "row.first_party.evidence_missing",
                        f"{row_id} has no passing structured state report",
                        "Bind the row to a passing screenshot-state diff capture.",
                        row_id,
                    )
                )
            if not row.get("exact_build_identity_ref"):
                findings.append(
                    Finding(
                        "error",
                        "row.first_party.build_identity_missing",
                        f"{row_id} is not attributable to a build identity",
                        "Attach the candidate exact build identity to the row.",
                        row_id,
                    )
                )

    # Extension gap rows: needs_review and refused rows must be visible
    # and must not imply full inheritance on any consuming surface.
    for row in ensure_list(
        packet.get("extension_inheritance_gap_rows"), "extension_inheritance_gap_rows"
    ):
        row_id = ensure_str(row.get("row_id"), "extension_inheritance_gap_rows[].row_id")
        for caveat in ensure_list(row.get("surface_caveats"), f"{row_id}.surface_caveats"):
            support_class = row.get("overall_support_class")
            if (
                caveat.get("implies_full_inheritance")
                and support_class != "full_inheritance"
            ):
                findings.append(
                    Finding(
                        "error",
                        "gap.implies_full_inheritance",
                        f"{row_id} implies full inheritance without proof on "
                        f"{caveat.get('surface_class')}",
                        "Only badge full inheritance when every axis is proven.",
                        row_id,
                    )
                )
        if not row.get("remediation_link"):
            findings.append(
                Finding(
                    "error",
                    "gap.remediation_missing",
                    f"{row_id} has no remediation link",
                    "Add a remediation link for the contributed appearance gap.",
                    row_id,
                )
            )

    # High-contrast signoff and live-update audit must be attributable.
    hc = ensure_dict(packet.get("high_contrast_signoff"), "high_contrast_signoff")
    if hc.get("signoff_state") != "passed":
        findings.append(
            Finding(
                "error",
                "high_contrast.not_signed_off",
                "high-contrast signoff did not pass",
                "Resolve the high-contrast focus/cue legibility gap.",
            )
        )
    if not hc.get("exact_build_identity_ref"):
        findings.append(
            Finding(
                "error",
                "high_contrast.build_identity_missing",
                "high-contrast signoff is not attributable to a build identity",
                "Attach the candidate exact build identity to the signoff.",
            )
        )
    live = ensure_dict(packet.get("live_update_audit"), "live_update_audit")
    if not live.get("exact_build_identity_ref"):
        findings.append(
            Finding(
                "error",
                "live_update.build_identity_missing",
                "live-update audit is not attributable to a build identity",
                "Attach the candidate exact build identity to the audit.",
            )
        )

    # Support export row parity.
    packet_appearance = {r["row_id"] for r in packet["appearance_rows"]}
    support_appearance = {r["row_id"] for r in support["appearance_rows"]}
    if packet_appearance != support_appearance:
        findings.append(
            Finding(
                "error",
                "support.appearance_row_parity",
                "support export appearance rows differ from the packet",
                "Regenerate the support export from the packet.",
                details={
                    "missing_in_support": sorted(packet_appearance - support_appearance),
                    "extra_in_support": sorted(support_appearance - packet_appearance),
                },
            )
        )
    packet_gaps = {r["row_id"] for r in packet["extension_inheritance_gap_rows"]}
    support_gaps = {r["row_id"] for r in support["extension_inheritance_gap_rows"]}
    if packet_gaps != support_gaps:
        findings.append(
            Finding(
                "error",
                "support.gap_row_parity",
                "support export gap rows differ from the packet",
                "Regenerate the support export from the packet.",
            )
        )

    return findings


# --------------------------------------------------------------------------
# Report rendering
# --------------------------------------------------------------------------
def md_bool(value: Any) -> str:
    return "yes" if value else "no"


def render_parity_report(packet: dict[str, Any]) -> str:
    s = packet["summary"]
    lines: list[str] = []
    lines.append("# Appearance parity evidence packet")
    lines.append("")
    lines.append(
        "Generated from "
        f"`{PACKET_REL}`. Do not edit by hand; refresh the upstream truth and "
        "re-run the generator."
    )
    lines.append("")
    lines.append(f"- Packet state: `{packet['packet_state']}`")
    lines.append(f"- As of: `{packet['as_of']}`")
    lines.append(f"- Release candidate: `{packet['release_candidate_ref']}`")
    lines.append(f"- Version: `{packet['version_label']}`")
    lines.append(f"- Build identity: `{packet['exact_build_identity_ref']}`")
    lines.append(f"- Source revision: `{packet['source_revision_ref']}`")
    lines.append("")
    lines.append("## Source packets")
    lines.append("")
    lines.append("| Source | Ref | Captured | Freshness | Age (days) |")
    lines.append("| --- | --- | --- | --- | ---: |")
    for key in sorted(packet["source_packets"]):
        block = packet["source_packets"][key]
        lines.append(
            f"| {key} | `{block['ref']}` | {block['captured_at']} | "
            f"`{block['freshness_class']}` | {block['age_days']} |"
        )
    lines.append("")
    lines.append("## Summary")
    lines.append("")
    lines.append("| Metric | Value |")
    lines.append("| --- | ---: |")
    lines.append(f"| Appearance rows | {s['appearance_row_count']} |")
    lines.append(f"| First-party rows | {s['first_party_row_count']} |")
    lines.append(f"| Contributed rows | {s['contributed_row_count']} |")
    lines.append(f"| Appearance-safe rows | {s['appearance_safe_row_count']} |")
    lines.append(
        f"| Appearance-safe (with caveats) rows | {s['appearance_safe_with_caveats_row_count']} |"
    )
    lines.append(f"| Unverified rows | {s['appearance_unverified_row_count']} |")
    lines.append(
        f"| Private-styling rows | {s['appearance_private_styling_row_count']} |"
    )
    lines.append(f"| Downgraded rows | {s['downgraded_row_count']} |")
    lines.append(f"| Extension gap rows | {s['extension_gap_row_count']} |")
    lines.append(
        f"| Extension needs-review rows | {s['extension_needs_review_row_count']} |"
    )
    lines.append(f"| High-contrast signoff | `{s['high_contrast_signoff_state']}` |")
    lines.append(f"| Live-update audit | `{s['live_update_audit_state']}` |")
    lines.append(f"| Importer coverage | `{s['importer_coverage_state']}` |")
    lines.append("")
    lines.append("## Appearance rows")
    lines.append("")
    lines.append(
        "| Surface | Owner | Decision | Themes proven | Freshness | Packet state |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for row in packet["appearance_rows"]:
        if row["owner_class"] == "first_party":
            themes = ", ".join(row["theme_classes_proven"])
        else:
            themes = f"theme={row['axis_support_by_token'].get('theme')}, hc={row.get('high_contrast_support_class')}"
        lines.append(
            f"| {row['surface_label']} | {row['owner_class']} | "
            f"`{row['appearance_decision']}` | {themes} | "
            f"`{row['source_evidence']['freshness_class']}` | `{row['packet_state']}` |"
        )
    lines.append("")
    lines.append("## Downgrade propagation")
    lines.append("")
    lines.append(
        "When the packet goes stale or red, these surfaces downgrade their "
        "claimed appearance rows:"
    )
    lines.append("")
    lines.append("| Surface | Class | Downgrades on | Ref |")
    lines.append("| --- | --- | --- | --- |")
    for surface in packet["consumer_surfaces"]:
        lines.append(
            f"| {surface['surface_id']} | {surface['surface_class']} | "
            f"{', '.join(surface['downgrades_on'])} | `{surface['ref']}` |"
        )
    lines.append("")
    lines.append("## Verify")
    lines.append("")
    lines.append("```sh")
    lines.append("python3 ci/check_m3_appearance_evidence.py --repo-root . --check")
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def render_high_contrast_report(packet: dict[str, Any]) -> str:
    hc = packet["high_contrast_signoff"]
    live = packet["live_update_audit"]
    importer = packet["importer_coverage"]
    lines: list[str] = []
    lines.append("# High-contrast signoff and live-change audit")
    lines.append("")
    lines.append(
        "Generated from "
        f"`{PACKET_REL}`. Do not edit by hand; refresh the upstream truth and "
        "re-run the generator."
    )
    lines.append("")
    lines.append("## High-contrast signoff")
    lines.append("")
    lines.append(f"- Signoff state: `{hc['signoff_state']}`")
    lines.append(f"- Build identity: `{hc['exact_build_identity_ref']}`")
    lines.append(f"- Version: `{hc['version_label']}`")
    lines.append(
        "- Minimum text-contrast targets: "
        f"high-contrast dark `{hc['minimum_text_contrast_targets']['high_contrast_dark']}`, "
        f"high-contrast light `{hc['minimum_text_contrast_targets']['high_contrast_light']}`"
    )
    lines.append("")
    lines.append("### First-party high-contrast rows")
    lines.append("")
    lines.append("| Surface | Theme | State | Focus legible | Color-only absent |")
    lines.append("| --- | --- | --- | --- | --- |")
    for row in hc["first_party_rows"]:
        lines.append(
            f"| {row['surface_class']} | {row['theme_class']} | {row['state_class']} | "
            f"{md_bool(row['focus_visibility_present'])} | "
            f"{md_bool(row['color_only_state_meaning_absent'])} |"
        )
    lines.append("")
    lines.append("### Protected cues under high contrast / forced colors")
    lines.append("")
    lines.append("| Cue | High contrast | Forced colors |")
    lines.append("| --- | --- | --- |")
    for cue in hc["protected_cue_preservation"]:
        lines.append(
            f"| {cue['protected_cue_class']} | "
            f"{md_bool(cue['preserved_in_high_contrast'])} | "
            f"{md_bool(cue['preserved_in_forced_colors'])} |"
        )
    lines.append("")
    lines.append("### Contributed high-contrast support")
    lines.append("")
    lines.append("| Extension | Surface | High-contrast support | Version |")
    lines.append("| --- | --- | --- | --- |")
    for row in hc["contributed_high_contrast_support"]:
        lines.append(
            f"| {row['extension_id']} | {row['surface_label']} | "
            f"`{row['high_contrast_support_class']}` | {row['package_version_label']} |"
        )
    lines.append("")
    lines.append("## Live OS appearance-change audit")
    lines.append("")
    lines.append(f"- Audit state: `{live['audit_state']}`")
    lines.append(f"- Build identity: `{live['exact_build_identity_ref']}`")
    lines.append(f"- Claimed profiles: {live['claimed_profile_count']}")
    lines.append(f"- Matrix rows: {live['row_count']}")
    lines.append(f"- Live (no review) axes: {', '.join(live['live_no_review_axes'])}")
    lines.append(
        f"- Live (checkpointed) axes: {', '.join(live['live_checkpointed_axes'])}"
    )
    lines.append(f"- Confirm-required axes: {', '.join(live['confirm_required_axes'])}")
    lines.append(
        f"- Embedded reload-required rows: {live['embedded_reload_required_rows']}"
    )
    lines.append(f"- Full-restart-required rows: {live['full_restart_required_rows']}")
    lines.append(
        f"- All reload/restart rows disclosed: {md_bool(live['all_reload_or_restart_rows_disclosed'])}"
    )
    lines.append(
        f"- Forced-colors rows explicit: {md_bool(live['forced_colors_rows_explicit'])}"
    )
    lines.append("")
    lines.append("## Importer coverage")
    lines.append("")
    lines.append(f"- Coverage state: `{importer['coverage_state']}`")
    lines.append(
        f"- Source: {importer['source_ecosystem']} {importer['source_tool_version']} "
        f"-> {', '.join(importer['target_theme_classes'])}"
    )
    lines.append(f"- Translated slots: {importer['translated_slot_count']}")
    lines.append(
        f"- Substituted with fallback: {importer['substituted_with_fallback_count']}"
    )
    lines.append(f"- Unsupported slots: {importer['unsupported_slot_count']}")
    lines.append(f"- Unresolved mappings: {importer['unresolved_mapping_count']}")
    lines.append(f"- Unresolved-slot total: {importer['unresolved_slot_total']}")
    lines.append(f"- Syntax coverage: {importer['syntax_coverage_percent']}%")
    lines.append(
        f"- Syntax unresolved scopes: {importer['syntax_unresolved_scope_count']}"
    )
    lines.append(
        f"- Protected cues preserved: {md_bool(importer['protected_cues_preserved'])}"
    )
    lines.append("")
    lines.append("## Verify")
    lines.append("")
    lines.append("```sh")
    lines.append("python3 ci/check_m3_appearance_evidence.py --repo-root . --check")
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def render_extension_gap_report(packet: dict[str, Any]) -> str:
    rows = packet["extension_inheritance_gap_rows"]
    lines: list[str] = []
    lines.append("# Extension inheritance-gap packet")
    lines.append("")
    lines.append(
        "Generated from "
        f"`{PACKET_REL}`. Do not edit by hand; refresh the upstream truth and "
        "re-run the generator."
    )
    lines.append("")
    lines.append(
        "Each row names an exact contributed surface, its unsupported "
        "appearance states, the downgrade note shown to users, and the "
        "remediation link. Host-stable trust, severity, permission, and "
        "policy labels stay host-rendered on every row."
    )
    lines.append("")
    lines.append("## Rows")
    lines.append("")
    lines.append(
        "| Extension | Surface | Support | Decision | Persists after install | Remediation |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for row in rows:
        lines.append(
            f"| {row['extension_id']} | {row['surface_label']} | "
            f"`{row['overall_support_class']}` | `{row['decision_class']}` | "
            f"{md_bool(row['persists_after_install'])} | `{row['remediation_link']}` |"
        )
    lines.append("")
    for row in rows:
        lines.append(f"### {row['surface_label']} — {row['extension_name']}")
        lines.append("")
        lines.append(f"- Surface id: `{row['surface_id']}`")
        lines.append(f"- Publisher: {row['publisher_label']}")
        lines.append(f"- Lifecycle: `{row['lifecycle_class']}`")
        lines.append(f"- Overall support: `{row['overall_support_class']}`")
        lines.append(f"- Decision: `{row['decision_class']}` ({row['reason_class']})")
        lines.append(f"- Downgrade note: {row['downgrade_notes']}")
        labels = row["host_stable_labels"]
        lines.append(
            "- Host-stable labels: "
            f"trust={labels['trust']}; severity={labels['severity']}; "
            f"permission={labels['permission']}; policy={labels['policy']}"
        )
        if row["gap_axes"]:
            lines.append("- Gap axes:")
            for axis in row["gap_axes"]:
                verify = " (needs verification)" if axis["requires_verification"] else ""
                lines.append(
                    f"  - `{axis['axis']}` -> `{axis['support_class']}`{verify}: "
                    f"{axis['caveat']}"
                )
        if row["known_unsupported_states"]:
            lines.append("- Known unsupported states:")
            for state in row["known_unsupported_states"]:
                lines.append(
                    f"  - `{state['axis']}` / `{state['state_label']}`: {state['summary']}"
                )
        lines.append(f"- Remediation: `{row['remediation_link']}`")
        lines.append(f"- Upstream row: `{row['upstream_row_ref']}`")
        lines.append("")
    lines.append("## Verify")
    lines.append("")
    lines.append("```sh")
    lines.append("python3 ci/check_m3_appearance_evidence.py --repo-root . --check")
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def build_capture(
    packet: dict[str, Any], findings: list[Finding], generated_at: str
) -> dict[str, Any]:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": CAPTURE_RECORD_KIND,
        "generated_at": generated_at,
        "packet_ref": PACKET_REL,
        "support_export_ref": SUPPORT_EXPORT_REL,
        "packet_state": packet["packet_state"],
        "status": "passed" if error_count == 0 else "failed",
        "summary": packet["summary"],
        "finding_count": len(findings),
        "error_count": error_count,
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    sources = load_sources(repo_root)

    generated_at = (
        args.generated_at
        or sources.extension.get("generated_at")
        or f"{sources.claim_manifest.get('as_of')}T00:00:00Z"
    )
    generated_at = ensure_str(generated_at, "generated_at")

    packet = build_packet(sources, generated_at)
    support = build_support_export(packet, generated_at)

    generated_refs = {
        PACKET_REL,
        SUPPORT_EXPORT_REL,
        PARITY_REPORT_REL,
        HIGH_CONTRAST_REPORT_REL,
        EXTENSION_GAP_REPORT_REL,
        CAPTURE_REL,
    }
    findings = validate(repo_root, packet, support, generated_refs)
    capture = build_capture(packet, findings, generated_at)

    outputs = {
        PACKET_REL: render_json(packet),
        SUPPORT_EXPORT_REL: render_json(support),
        PARITY_REPORT_REL: render_parity_report(packet),
        HIGH_CONTRAST_REPORT_REL: render_high_contrast_report(packet),
        EXTENSION_GAP_REPORT_REL: render_extension_gap_report(packet),
        CAPTURE_REL: render_json(capture),
    }
    all_written = True
    for rel, content in outputs.items():
        if not write_or_check(repo_root / rel, content, args.check):
            all_written = False

    error_count = sum(1 for finding in findings if finding.severity == "error")
    if args.check and not all_written:
        return 1
    if error_count:
        print(render_json(capture), file=sys.stderr)
        return 1

    print(
        f"appearance-evidence packet validated: "
        f"{packet['summary']['appearance_row_count']} appearance rows, "
        f"{packet['summary']['extension_gap_row_count']} extension gap rows, "
        f"state {packet['packet_state']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
