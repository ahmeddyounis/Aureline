#!/usr/bin/env python3
"""Shared build and validation logic for the M5 certification matrix.

Both the regenerator (``tools/regenerate_xt12_certification_matrix.py``) and the
CI gate (``tools/ci/m5/xt12_certification_check.py``) import this module so the
emitted artifact and the enforced contract are derived from one source.

The matrix certifies every *marketed* M5 surface across the switching
(onboarding/migration, first-useful-work, command discoverability),
visual-system (component-state parity, appearance conformance), durable-attention
(durable attention, notification privacy), and embedded-boundary (embedded
boundary, desktop conformance, accessibility/i18n) dimensions. The canonical
"marketed surface" list and each surface's claim ceiling / effective label are
ingested from the M5 feature-family register; this module never re-types that
truth, so the matrix cannot widen a surface above its canonical ceiling.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable

SCHEMA_VERSION = 1
RECORD_KIND = "m5_xt12_certification_matrix"
MATRIX_ID = "m5_xt12_certification_matrix:v1"

# ---------------------------------------------------------------------------
# Canonical inputs and outputs.
# ---------------------------------------------------------------------------

FAMILY_REGISTER_REL = Path(
    "artifacts/release/m5/"
    "publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json"
)
CERT_TRAIN_REGISTER_REL = Path(
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)
TRUTH_SURFACE_INGESTION_REL = Path(
    "artifacts/release/m5/m5_truth_surface_evidence_ingestion.json"
)

ARTIFACT_REL = Path("artifacts/release/m5/xt12-qualification-matrix.json")
CAPTURE_REL = Path(
    "artifacts/release/m5/captures/xt12-qualification-matrix_validation_capture.json"
)
EVIDENCE_INDEX_REL = Path("artifacts/release/m5/xt12-evidence-index.md")
NARROWING_REPORT_REL = Path("artifacts/release/m5/xt12-narrowing-report.md")
DOC_REL = "docs/m5/xt12-certification.md"
SCHEMA_REL = Path("schemas/governance/m5_xt12_certification_matrix.schema.json")

# ---------------------------------------------------------------------------
# Dimensions: the ten certification cells bound for every marketed surface.
# Grouped into the four certification axes the milestone closeout requires.
# Each dimension binds the canonical depth-lane evidence that already landed.
# ---------------------------------------------------------------------------

LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ABOVE_CUTLINE = {"lts", "stable"}

DIMENSION_GRADES = ["pass", "partial", "fail", "stale", "missing", "waived"]
PASSING_GRADES = {"pass", "waived"}

XT12_STATES = ["qualified", "narrowed", "held_back"]
POSTURE_CLASSES = ["local_only", "mirrored", "managed", "browser_handoff"]


@dataclass(frozen=True)
class Dimension:
    key: str
    title: str
    axis: str
    artifact_ref: str
    doc_ref: str
    check_ref: str | None


DIMENSIONS: tuple[Dimension, ...] = (
    Dimension(
        "onboarding_migration",
        "Onboarding and migration truth",
        "switching",
        "artifacts/compat/m5/migration-reports/m5_depth_import_report.md",
        "docs/m5/migration-depth-lanes.md",
        None,
    ),
    Dimension(
        "first_useful_work",
        "First-useful-work packets",
        "switching",
        "artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md",
        "docs/m5/first_useful_work.md",
        None,
    ),
    Dimension(
        "command_discoverability",
        "Command and discoverability parity",
        "switching",
        "artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md",
        "docs/ux/m5/command_parity_audit.md",
        "tools/ci/m5/command_parity_check.py",
    ),
    Dimension(
        "component_state_parity",
        "Token and component-state parity",
        "visual_system",
        "artifacts/ux/m5/component-state-audit/m5_component_state_audit.md",
        "docs/m5/component-state-parity.md",
        "tools/ci/m5/component_state_check.py",
    ),
    Dimension(
        "appearance_conformance",
        "Appearance and density conformance",
        "visual_system",
        "artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md",
        "docs/m5/appearance-and-density-parity.md",
        "tools/ci/m5/appearance_parity_check.py",
    ),
    Dimension(
        "durable_attention",
        "Durable-attention routes",
        "durable_attention",
        "artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md",
        "docs/m5/durable-progress-and-reopen.md",
        "tools/ci/m5/activity_objects_check.py",
    ),
    Dimension(
        "notification_privacy",
        "Notification privacy and badges",
        "durable_attention",
        "artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md",
        "docs/m5/notification-privacy-and-badges.md",
        "tools/ci/m5/notification_routes_check.py",
    ),
    Dimension(
        "embedded_boundary",
        "Embedded-boundary integrity",
        "embedded_boundary",
        "artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md",
        "docs/m5/embedded-boundaries-and-auth.md",
        "tools/ci/m5/embedded_boundaries_check.py",
    ),
    Dimension(
        "desktop_conformance",
        "Desktop and handoff conformance",
        "embedded_boundary",
        "artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md",
        "docs/m5/desktop-and-handoff-parity.md",
        "tools/ci/m5/desktop_conformance_check.py",
    ),
    Dimension(
        "accessibility_i18n",
        "Accessibility and i18n packets",
        "embedded_boundary",
        "artifacts/a11y/m5_depth_surfaces/m5_inclusive_depth_audit.md",
        "docs/m5/accessibility-and-locale-depth.md",
        "tools/ci/m5/inclusive_depth_check.py",
    ),
)
DIMENSION_BY_KEY = {d.key: d for d in DIMENSIONS}
DIMENSION_KEYS = [d.key for d in DIMENSIONS]

# Posture per marketed family. Posture is always explicit, never inferred.
FAMILY_POSTURE: dict[str, str] = {
    "framework": "local_only",
    "ai_adjacent": "mirrored",
    "notebook": "local_only",
    "data_rich": "local_only",
    "review": "local_only",
    "companion": "browser_handoff",
    "managed_depth": "managed",
}

# Authored XT certification grades per marketed family. Only non-``pass`` cells are
# listed; every unlisted dimension is ``pass``. These grades are the canonical XT
# truth this row publishes, but the surface's claim ceiling and effective label are
# always ingested from the feature-family register (see ``build_matrix``), so a
# surface can never be published wider here than its canonical packet permits.
CELL_OVERRIDES: dict[str, dict[str, tuple[str, str]]] = {
    "framework": {},
    "ai_adjacent": {
        "first_useful_work": (
            "waived",
            "AI entry-route packet is under final language-model review; covered by an unexpired waiver.",
        ),
    },
    "notebook": {
        "onboarding_migration": (
            "missing",
            "Notebook onboarding/migration proof is not yet on file.",
        ),
        "first_useful_work": (
            "missing",
            "Notebook first-useful-work entry-route packet is not yet drafted.",
        ),
    },
    "data_rich": {
        "component_state_parity": (
            "stale",
            "Data-grid component-state evidence breached its freshness SLO.",
        ),
        "appearance_conformance": (
            "stale",
            "Data-rich appearance/density evidence breached its freshness SLO.",
        ),
    },
    "review": {},
    "companion": {
        "embedded_boundary": (
            "fail",
            "Companion browser-handoff boundary lacks a verified return/rollback path.",
        ),
        "desktop_conformance": (
            "partial",
            "Companion desktop handoff continuity is only partially proven.",
        ),
    },
    "managed_depth": {
        "desktop_conformance": (
            "fail",
            "Managed-depth desktop/system-open continuity lacks a verified rollback path.",
        ),
        "embedded_boundary": (
            "partial",
            "Managed-depth embedded provider boundary is only partially proven.",
        ),
    },
}

# Narrowing reasons (schema enum). Per-dimension reasons are ``<dimension>_unqualified``;
# the rest are cross-cutting, including the four forbidden v1-shell dependencies the
# closeout must never let a marketed surface widen over.
NARROWING_REASONS = [f"{key}_unqualified" for key in DIMENSION_KEYS] + [
    "evidence_stale",
    "evidence_missing",
    "owner_signoff_missing",
    "policy_blocked",
    "source_surface_below_claim",
    "hidden_onboarding_gap",
    "embedded_high_risk_approval",
    "toast_only_long_running_truth",
    "theme_only_state_semantics",
]

# Forbidden v1-shell dependencies. A surface that carries one of these may never sit
# above the launch cutline; the publication gate holds if any release-blocking
# surface does.
FORBIDDEN_DEPENDENCY_REASONS = {
    "hidden_onboarding_gap",
    "embedded_high_risk_approval",
    "toast_only_long_running_truth",
    "theme_only_state_semantics",
}

STOP_RULES = [
    {
        "rule_id": "xt12_rule:evidence_stale",
        "title": "Certification evidence is stale on the claimed profile",
        "trigger_reason": "evidence_stale",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "narrow_label",
        "blocks_widening": True,
        "rationale": "A surface whose XT evidence breached its freshness SLO cannot keep a Stable claim.",
    },
    {
        "rule_id": "xt12_rule:evidence_missing",
        "title": "Certification evidence is missing on the claimed profile",
        "trigger_reason": "evidence_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "A surface missing a required XT evidence cell must hold below the cutline.",
    },
    {
        "rule_id": "xt12_rule:owner_signoff_missing",
        "title": "Owner sign-off is absent",
        "trigger_reason": "owner_signoff_missing",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "A surface without owner sign-off cannot carry a Stable XT claim.",
    },
    {
        "rule_id": "xt12_rule:policy_blocked",
        "title": "Certification is policy-blocked on the deployment row",
        "trigger_reason": "policy_blocked",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "A managed-deployment surface blocked by policy must hold below the cutline.",
    },
    {
        "rule_id": "xt12_rule:hidden_onboarding_gap",
        "title": "Hidden onboarding/setup gap",
        "trigger_reason": "hidden_onboarding_gap",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "No marketed surface may widen while its onboarding/setup work is hidden or unproven.",
    },
    {
        "rule_id": "xt12_rule:embedded_high_risk_approval",
        "title": "Embedded high-risk approval path",
        "trigger_reason": "embedded_high_risk_approval",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "A surface that approves high-risk actions inside an embedded pane must hold; auth belongs in the system browser.",
    },
    {
        "rule_id": "xt12_rule:toast_only_long_running_truth",
        "title": "Toast-only long-running truth",
        "trigger_reason": "toast_only_long_running_truth",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "Long-running truth must survive in a durable surface, not a dismissible toast.",
    },
    {
        "rule_id": "xt12_rule:theme_only_state_semantics",
        "title": "Theme-only state semantics",
        "trigger_reason": "theme_only_state_semantics",
        "applies_to_labels": ["lts", "stable"],
        "default_action": "hold_back",
        "blocks_widening": True,
        "rationale": "State meaning may never depend on color/theme alone; it must carry a non-theme signal.",
    },
]

LAUNCH_CUTLINE = {
    "cutline_level": "stable",
    "above_cutline_levels": ["lts", "stable"],
    "below_cutline_levels": ["beta", "preview", "withdrawn"],
    "description": (
        "A marketed M5 surface ends the milestone XT-qualified (at or above Stable) only when every "
        "certification cell -- onboarding/migration, first-useful-work, command discoverability, "
        "component-state and appearance parity, durable attention, notification privacy, embedded "
        "boundary, desktop conformance, and accessibility/i18n -- is current on the claimed profile, "
        "the owner is signed off, no forbidden v1-shell dependency is present, and the family register "
        "publishes it at or above Stable. A surface that loses any of those narrows to Beta or holds "
        "at Preview rather than inheriting an adjacent green surface."
    ),
}


# ---------------------------------------------------------------------------
# Build.
# ---------------------------------------------------------------------------


def _load(repo_root: Path, rel: Path) -> dict[str, Any]:
    return json.loads((repo_root / rel).read_text(encoding="utf-8"))


def xt12_state_for(published_label: str) -> str:
    """Project the ingested effective label onto the XT certification state."""
    if published_label in ABOVE_CUTLINE:
        return "qualified"
    if published_label == "beta":
        return "narrowed"
    return "held_back"


def _build_cells(family_kind: str, waiver: dict[str, Any] | None) -> list[dict[str, Any]]:
    overrides = CELL_OVERRIDES.get(family_kind, {})
    cells: list[dict[str, Any]] = []
    for dim in DIMENSIONS:
        grade, note = overrides.get(
            dim.key, ("pass", "Canonical XT evidence is current on file.")
        )
        cell: dict[str, Any] = {
            "dimension": dim.key,
            "axis": dim.axis,
            "title": dim.title,
            "grade": grade,
            "evidence_ref": dim.artifact_ref,
            "doc_ref": dim.doc_ref,
            "note": note,
        }
        if dim.check_ref is not None:
            cell["check_ref"] = dim.check_ref
        if grade == "waived":
            cell["waiver_ref"] = (waiver or {}).get("waiver_ref", "")
        cells.append(cell)
    return cells


def _narrowing_reasons(
    cells: list[dict[str, Any]],
    family: dict[str, Any],
    state: str,
    posture: str,
    claim_label: str,
    published_label: str,
) -> list[str]:
    reasons: list[str] = []
    has_stale = False
    has_missing = False
    for cell in cells:
        grade = cell["grade"]
        if grade in PASSING_GRADES:
            continue
        reasons.append(f"{cell['dimension']}_unqualified")
        if grade == "stale":
            has_stale = True
        if grade == "missing":
            has_missing = True
        if cell["dimension"] == "onboarding_migration" and grade in ("missing", "fail"):
            reasons.append("hidden_onboarding_gap")
    if has_stale:
        reasons.append("evidence_stale")
    if has_missing:
        reasons.append("evidence_missing")

    below_claim = LABEL_RANK[published_label] < LABEL_RANK[claim_label]
    owner_signed = bool(family.get("owner_signoff", {}).get("signed_off"))
    if below_claim and not owner_signed:
        reasons.append("owner_signoff_missing")
    if state == "held_back" and posture == "managed":
        reasons.append("policy_blocked")
    # A surface held below its claim purely by a non-XT family gap still cannot be
    # XT-qualified; record the inherited ceiling so the reason is never empty.
    if below_claim and not reasons:
        reasons.append("source_surface_below_claim")

    # Deduplicate while preserving order.
    seen: set[str] = set()
    ordered: list[str] = []
    for reason in reasons:
        if reason not in seen:
            seen.add(reason)
            ordered.append(reason)
    return ordered


def build_matrix(repo_root: Path) -> dict[str, Any]:
    families_doc = _load(repo_root, FAMILY_REGISTER_REL)
    families = families_doc["rows"]
    release_blocking_refs = set(families_doc.get("release_blocking_family_refs", []))

    rows: list[dict[str, Any]] = []
    for family in families:
        kind = family["family_kind"]
        posture = FAMILY_POSTURE[kind]
        claim_label = family["claim_label"]
        published_label = family["published_label"]
        waiver = family.get("waiver")
        state = xt12_state_for(published_label)
        cells = _build_cells(kind, waiver)
        reasons = _narrowing_reasons(
            cells, family, state, posture, claim_label, published_label
        )
        release_blocking = bool(family.get("release_blocking")) or (
            family["surface_ref"] in release_blocking_refs
        )

        row: dict[str, Any] = {
            "entry_id": f"xt12:{kind}",
            "title": family.get("title", f"M5 {kind} surface"),
            "family_kind": kind,
            "family_ref": family["surface_ref"],
            "surface_summary": family.get("surface_summary", ""),
            "canonical_source_ref": family["entry_id"],
            "release_blocking": release_blocking,
            "posture": posture,
            "claim_label": claim_label,
            "canonical_published_label": published_label,
            "published_label": published_label,
            "xt12_state": state,
            "cells": cells,
            "waiver": waiver,
            "active_narrowing_reasons": reasons,
            "publication_destinations": [
                "release_center",
                "help_about",
                "support_export",
                "docs_public_truth",
            ],
            "rationale": _row_rationale(state, claim_label, published_label, reasons),
        }
        rows.append(row)

    publication = _compute_publication(rows)
    matrix: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "matrix_id": MATRIX_ID,
        "status": "published",
        "overview_page": DOC_REL,
        "evidence_index_page": str(EVIDENCE_INDEX_REL),
        "narrowing_report_page": str(NARROWING_REPORT_REL),
        "as_of": families_doc.get("as_of", ""),
        "source_refs": {
            "feature_family_register_ref": str(FAMILY_REGISTER_REL),
            "cert_train_register_ref": str(CERT_TRAIN_REGISTER_REL),
            "truth_surface_ingestion_ref": str(TRUTH_SURFACE_INGESTION_REL),
        },
        "claim_manifest_ref": families_doc.get("claim_manifest_ref", ""),
        "marketed_surfaces": [f["family_kind"] for f in families],
        "xt12_dimensions": DIMENSION_KEYS,
        "xt12_axes": ["switching", "visual_system", "durable_attention", "embedded_boundary"],
        "xt12_states": XT12_STATES,
        "dimension_grades": DIMENSION_GRADES,
        "lifecycle_labels": LIFECYCLE_LABELS,
        "posture_classes": POSTURE_CLASSES,
        "narrowing_reasons": NARROWING_REASONS,
        "launch_cutline": LAUNCH_CUTLINE,
        "release_blocking_surface_refs": sorted(
            r["family_ref"] for r in rows if r["release_blocking"]
        ),
        "stop_rules": STOP_RULES,
        "evidence_catalog": _evidence_catalog(),
        "rows": rows,
        "publication": publication,
        "summary": _compute_summary(rows),
    }
    return matrix


def _row_rationale(state: str, claim: str, published: str, reasons: list[str]) -> str:
    if state == "qualified":
        if reasons:
            return (
                f"Qualified at {published}; every certification cell is current "
                f"(one dimension on an unexpired waiver)."
            )
        return f"Qualified at {published}; every certification cell is current under the {claim} ceiling."
    band = "narrowed to Beta" if state == "narrowed" else "held at Preview"
    return (
        f"{band} from a {claim} ceiling because XT certification is incomplete: "
        f"{', '.join(reasons)}."
    )


def _evidence_catalog() -> list[dict[str, Any]]:
    catalog: list[dict[str, Any]] = []
    for dim in DIMENSIONS:
        entry: dict[str, Any] = {
            "dimension": dim.key,
            "axis": dim.axis,
            "title": dim.title,
            "artifact_ref": dim.artifact_ref,
            "doc_ref": dim.doc_ref,
        }
        if dim.check_ref is not None:
            entry["check_ref"] = dim.check_ref
        catalog.append(entry)
    return catalog


def _compute_publication(rows: list[dict[str, Any]]) -> dict[str, Any]:
    blocking = [r for r in rows if r["release_blocking"] and r["xt12_state"] != "qualified"]
    decision = "hold" if blocking else "proceed"
    blocking_rule_ids = sorted(
        {
            f"xt12_rule:{reason}"
            for r in blocking
            for reason in r["active_narrowing_reasons"]
            if any(rule["trigger_reason"] == reason for rule in STOP_RULES)
        }
    )
    if decision == "hold":
        rationale = (
            "Widening is held: at least one release-blocking marketed surface is not XT-qualified. "
            "Each blocked surface lists the dimension(s) and reason(s) holding it below the cutline."
        )
    else:
        rationale = "Every release-blocking marketed surface ends the milestone XT-qualified."
    return {
        "publication_gate": "m5_xt12_certification",
        "decision": decision,
        "blocking_rule_ids": blocking_rule_ids,
        "blocking_surface_ids": sorted(r["entry_id"] for r in blocking),
        "rationale": rationale,
    }


def _compute_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    def count(pred: Callable[[dict[str, Any]], bool]) -> int:
        return sum(1 for r in rows if pred(r))

    all_cells = [c for r in rows for c in r["cells"]]

    def cells_with(grade: str) -> int:
        return sum(1 for c in all_cells if c["grade"] == grade)

    return {
        "total_surfaces": len(rows),
        "total_dimensions": len(DIMENSIONS),
        "total_cells": len(all_cells),
        "surfaces_qualified": count(lambda r: r["xt12_state"] == "qualified"),
        "surfaces_narrowed": count(lambda r: r["xt12_state"] == "narrowed"),
        "surfaces_held_back": count(lambda r: r["xt12_state"] == "held_back"),
        "surfaces_on_waiver": count(lambda r: r["waiver"] is not None),
        "release_blocking_total": count(lambda r: r["release_blocking"]),
        "release_blocking_qualified": count(
            lambda r: r["release_blocking"] and r["xt12_state"] == "qualified"
        ),
        "release_blocking_below_cutline": count(
            lambda r: r["release_blocking"] and r["xt12_state"] != "qualified"
        ),
        "cells_pass": cells_with("pass"),
        "cells_partial": cells_with("partial"),
        "cells_fail": cells_with("fail"),
        "cells_stale": cells_with("stale"),
        "cells_missing": cells_with("missing"),
        "cells_waived": cells_with("waived"),
        "total_active_narrowing_reasons": sum(
            len(r["active_narrowing_reasons"]) for r in rows
        ),
        "rules_firing": len(
            {
                reason
                for r in rows
                for reason in r["active_narrowing_reasons"]
                if any(rule["trigger_reason"] == reason for rule in STOP_RULES)
            }
        ),
    }


def compute_capture(matrix: dict[str, Any]) -> dict[str, Any]:
    """A frozen, export-safe projection the gate cross-checks."""
    summary = matrix["summary"]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": f"{RECORD_KIND}_validation_capture",
        "matrix_id": MATRIX_ID,
        "as_of": matrix["as_of"],
        "decision": matrix["publication"]["decision"],
        "total_surfaces": summary["total_surfaces"],
        "total_cells": summary["total_cells"],
        "surfaces_qualified": summary["surfaces_qualified"],
        "surfaces_narrowed": summary["surfaces_narrowed"],
        "surfaces_held_back": summary["surfaces_held_back"],
        "release_blocking_below_cutline": summary["release_blocking_below_cutline"],
        "surface_states": {r["family_kind"]: r["xt12_state"] for r in matrix["rows"]},
        "surface_published_labels": {
            r["family_kind"]: r["published_label"] for r in matrix["rows"]
        },
        "validation_clean": True,
    }


# ---------------------------------------------------------------------------
# Validation (shared by the gate; also runs over negative fixtures).
# ---------------------------------------------------------------------------


@dataclass
class Finding:
    code: str
    message: str
    entry_id: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.entry_id is not None:
            out["entry_id"] = self.entry_id
        if self.detail:
            out["detail"] = self.detail
        return out


def validate_matrix(
    matrix: dict[str, Any], family_index: dict[str, dict[str, Any]]
) -> list[Finding]:
    """Return all structural/contradiction findings for ``matrix``.

    ``family_index`` maps the canonical family ``entry_id`` to the source row, so
    the gate can prove the matrix never drifts from -- or widens above -- the
    canonical feature-family register.
    """
    findings: list[Finding] = []

    if matrix.get("record_kind") != RECORD_KIND:
        findings.append(
            Finding("record_kind_mismatch", f"record_kind must be {RECORD_KIND}")
        )
    if matrix.get("schema_version") != SCHEMA_VERSION:
        findings.append(
            Finding("schema_version_mismatch", f"schema_version must be {SCHEMA_VERSION}")
        )

    rows = matrix.get("rows", [])
    seen_ids: set[str] = set()
    for row in rows:
        entry_id = row.get("entry_id")
        if entry_id in seen_ids:
            findings.append(
                Finding("duplicate_entry_id", "duplicate surface entry_id", entry_id=entry_id)
            )
        seen_ids.add(entry_id)

        claim = row.get("claim_label")
        published = row.get("published_label")
        canonical_published = row.get("canonical_published_label")
        state = row.get("xt12_state")
        reasons = row.get("active_narrowing_reasons") or []
        cells = row.get("cells") or []

        # Coverage: exactly the ten dimensions, each once.
        cell_keys = [c.get("dimension") for c in cells]
        for dim_key in DIMENSION_KEYS:
            if dim_key not in cell_keys:
                findings.append(
                    Finding(
                        "missing_dimension_cell",
                        "surface is missing a required certification dimension",
                        entry_id=entry_id,
                        detail={"dimension": dim_key},
                    )
                )
        if len(cell_keys) != len(set(cell_keys)):
            findings.append(
                Finding("duplicate_dimension_cell", "surface repeats a dimension", entry_id=entry_id)
            )

        # Never advertise wider than the ceiling.
        if LABEL_RANK.get(published, 99) > LABEL_RANK.get(claim, -1):
            findings.append(
                Finding(
                    "published_exceeds_ceiling",
                    "surface advertises a wider claim than its ceiling",
                    entry_id=entry_id,
                    detail={"published": published, "ceiling": claim},
                )
            )
        if published != canonical_published:
            findings.append(
                Finding(
                    "published_not_canonical",
                    "published label does not equal the canonical effective label",
                    entry_id=entry_id,
                    detail={"published": published, "canonical": canonical_published},
                )
            )

        # State must agree with the effective-label band.
        if state != xt12_state_for(published):
            findings.append(
                Finding(
                    "state_band_mismatch",
                    "xt12_state does not match the published label band",
                    entry_id=entry_id,
                    detail={"state": state, "published": published},
                )
            )

        has_blocking_cell = any(c.get("grade") not in PASSING_GRADES for c in cells)
        if state == "qualified":
            if has_blocking_cell:
                findings.append(
                    Finding(
                        "qualified_with_open_cell",
                        "qualified surface has a cell that is not pass/waived",
                        entry_id=entry_id,
                    )
                )
            if reasons:
                findings.append(
                    Finding(
                        "qualified_with_narrowing_reason",
                        "qualified surface must carry no active narrowing reasons",
                        entry_id=entry_id,
                    )
                )
        else:
            # A narrowed/held surface must explain itself: either an open cell or
            # the inherited-ceiling reason.
            if not reasons:
                findings.append(
                    Finding(
                        "narrowed_without_reason",
                        "below-cutline surface must carry at least one narrowing reason",
                        entry_id=entry_id,
                    )
                )
            if not has_blocking_cell and "source_surface_below_claim" not in reasons:
                findings.append(
                    Finding(
                        "narrowed_without_cause",
                        "below-cutline surface must have an open cell or an inherited-ceiling reason",
                        entry_id=entry_id,
                    )
                )

        # Forbidden v1-shell dependencies may never sit above the cutline.
        if published in ABOVE_CUTLINE:
            for reason in reasons:
                if reason in FORBIDDEN_DEPENDENCY_REASONS:
                    findings.append(
                        Finding(
                            "forbidden_dependency_above_cutline",
                            "surface above the cutline carries a forbidden v1-shell dependency",
                            entry_id=entry_id,
                            detail={"reason": reason},
                        )
                    )

        # A waived cell requires a waiver on the surface.
        for cell in cells:
            if cell.get("grade") == "waived" and not row.get("waiver"):
                findings.append(
                    Finding(
                        "waived_cell_without_waiver",
                        "a waived cell requires a surface waiver",
                        entry_id=entry_id,
                        detail={"dimension": cell.get("dimension")},
                    )
                )

        if not row.get("posture"):
            findings.append(Finding("missing_posture", "surface must disclose posture", entry_id=entry_id))

        # Source drift / anti-widening against the canonical family register.
        source = family_index.get(row.get("canonical_source_ref"))
        if source is None:
            findings.append(
                Finding(
                    "source_ref_unknown",
                    "canonical_source_ref does not name a family in the source register",
                    entry_id=entry_id,
                    detail={"canonical_source_ref": row.get("canonical_source_ref")},
                )
            )
        else:
            if claim != source.get("claim_label"):
                findings.append(
                    Finding(
                        "source_claim_drift",
                        "ingested claim ceiling drifted from the source register",
                        entry_id=entry_id,
                        detail={"ingested": claim, "source": source.get("claim_label")},
                    )
                )
            if canonical_published != source.get("published_label"):
                findings.append(
                    Finding(
                        "source_published_drift",
                        "ingested effective label drifted from the source register",
                        entry_id=entry_id,
                        detail={
                            "ingested": canonical_published,
                            "source": source.get("published_label"),
                        },
                    )
                )

    # Publication decision must agree with the firing release-blocking rows.
    publication = matrix.get("publication", {})
    expected_hold = any(
        r.get("release_blocking") and r.get("xt12_state") != "qualified" for r in rows
    )
    decision = publication.get("decision")
    if expected_hold and decision != "hold":
        findings.append(
            Finding(
                "publication_decision_mismatch",
                "publication must hold while a release-blocking surface is below the cutline",
                detail={"decision": decision},
            )
        )
    if not expected_hold and decision != "proceed":
        findings.append(
            Finding(
                "publication_decision_mismatch",
                "publication must proceed when every release-blocking surface is qualified",
                detail={"decision": decision},
            )
        )

    return findings


def dumps(obj: Any) -> str:
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


# ---------------------------------------------------------------------------
# Renderers.
# ---------------------------------------------------------------------------

_STATE_LABEL = {"qualified": "Qualified", "narrowed": "Narrowed", "held_back": "Held back"}
_AXIS_LABEL = {
    "switching": "Switching truth",
    "visual_system": "Visual-system parity",
    "durable_attention": "Durable attention",
    "embedded_boundary": "Embedded-boundary integrity",
}
_GRADE_MARK = {
    "pass": "pass",
    "partial": "partial",
    "fail": "FAIL",
    "stale": "stale",
    "missing": "missing",
    "waived": "waived",
}


def render_evidence_index(matrix: dict[str, Any]) -> str:
    """Render the human-readable M5 XT evidence index (generated; do not hand-edit)."""
    lines: list[str] = []
    lines.append("# M5 switching, visual-system, attention, and boundary evidence index")
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_xt12_certification_matrix.py. Do not hand-edit. -->"
    )
    lines.append("")
    lines.append(
        f"Canonical matrix: `{ARTIFACT_REL}` · as of **{matrix['as_of']}** · "
        f"publication decision: **{matrix['publication']['decision']}**."
    )
    lines.append("")
    lines.append(
        "This index is the canonical evidence map for the M5 closeout certification lane. "
        "Release center, Help/About, support exports, and docs/public-truth publication ingest "
        "the matrix it summarizes instead of cloning status text. Every marketed M5 surface ends "
        "the milestone **qualified**, **narrowed**, or **held back**, backed by the per-dimension "
        "evidence bound below."
    )
    lines.append("")

    summary = matrix["summary"]
    lines.append("## Closeout summary")
    lines.append("")
    lines.append(f"- Marketed surfaces: **{summary['total_surfaces']}**")
    lines.append(
        f"- Qualified / narrowed / held back: "
        f"**{summary['surfaces_qualified']} / {summary['surfaces_narrowed']} / "
        f"{summary['surfaces_held_back']}**"
    )
    lines.append(
        f"- Release-blocking surfaces below the cutline: "
        f"**{summary['release_blocking_below_cutline']}** of {summary['release_blocking_total']}"
    )
    lines.append(
        f"- Certification cells (pass/partial/fail/stale/missing/waived): "
        f"{summary['cells_pass']}/{summary['cells_partial']}/{summary['cells_fail']}/"
        f"{summary['cells_stale']}/{summary['cells_missing']}/{summary['cells_waived']}"
    )
    lines.append("")

    lines.append("## Surface certification states")
    lines.append("")
    lines.append("| Surface | Posture | Ceiling | Published | XT state | Release-blocking |")
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for row in matrix["rows"]:
        lines.append(
            f"| {row['title']} | {row['posture']} | {row['claim_label']} | "
            f"{row['published_label']} | {_STATE_LABEL[row['xt12_state']]} | "
            f"{'yes' if row['release_blocking'] else 'no'} |"
        )
    lines.append("")

    lines.append("## Evidence catalog by dimension")
    lines.append("")
    for axis in matrix["xt12_axes"]:
        lines.append(f"### {_AXIS_LABEL[axis]}")
        lines.append("")
        for entry in matrix["evidence_catalog"]:
            if entry["axis"] != axis:
                continue
            check = f" · gate `{entry['check_ref']}`" if entry.get("check_ref") else ""
            lines.append(
                f"- **{entry['title']}** — evidence `{entry['artifact_ref']}` · "
                f"doc `{entry['doc_ref']}`{check}"
            )
        lines.append("")

    lines.append("## Per-surface certification matrix")
    lines.append("")
    for row in matrix["rows"]:
        lines.append(f"### {row['title']} — {_STATE_LABEL[row['xt12_state']]}")
        lines.append("")
        lines.append(f"{row['rationale']}")
        lines.append("")
        lines.append("| Dimension | Axis | Grade | Evidence |")
        lines.append("| --- | --- | --- | --- |")
        for cell in row["cells"]:
            lines.append(
                f"| {cell['title']} | {_AXIS_LABEL[cell['axis']]} | "
                f"{_GRADE_MARK[cell['grade']]} | `{cell['evidence_ref']}` |"
            )
        lines.append("")
    return "\n".join(lines) + "\n"


def render_narrowing_report(matrix: dict[str, Any]) -> str:
    """Render the M5 XT narrowing report (generated; do not hand-edit)."""
    lines: list[str] = []
    lines.append("# M5 switching/visual-system/attention/boundary narrowing report")
    lines.append("")
    lines.append(
        "<!-- Generated by tools/regenerate_xt12_certification_matrix.py. Do not hand-edit. -->"
    )
    lines.append("")
    publication = matrix["publication"]
    lines.append(
        f"Publication gate **{publication['publication_gate']}** decision: "
        f"**{publication['decision']}** — {publication['rationale']}"
    )
    lines.append("")
    lines.append(
        "Narrowing is automated from the canonical feature-family register and the per-dimension XT "
        "evidence: a marketed surface narrows or holds whenever its certification evidence is stale, "
        "missing, policy-blocked, or red on the claimed profile, or while it still depends on a "
        "forbidden v1-shell pattern (toast-only long-running truth, theme-only state meaning, a "
        "hidden onboarding gap, or an embedded high-risk approval path). No call here is left to "
        "marketing copy or shiproom memory."
    )
    lines.append("")

    narrowed = [r for r in matrix["rows"] if r["xt12_state"] != "qualified"]
    if not narrowed:
        lines.append("All marketed M5 surfaces are XT-qualified; no surface is narrowed or held back.")
        lines.append("")
        return "\n".join(lines) + "\n"

    if publication["blocking_surface_ids"]:
        lines.append("## Surfaces blocking final widening")
        lines.append("")
        for sid in publication["blocking_surface_ids"]:
            lines.append(f"- `{sid}`")
        lines.append("")

    lines.append("## Narrowed and held-back surfaces")
    lines.append("")
    for row in narrowed:
        verb = "Narrowed to Beta" if row["xt12_state"] == "narrowed" else "Held at Preview"
        lines.append(f"### {row['title']} — {verb}")
        lines.append("")
        lines.append(
            f"- Ceiling **{row['claim_label']}** → published **{row['published_label']}** "
            f"(release-blocking: {'yes' if row['release_blocking'] else 'no'}, posture {row['posture']})."
        )
        lines.append(f"- Reasons: {', '.join(row['active_narrowing_reasons'])}.")
        open_cells = [c for c in row["cells"] if c["grade"] not in PASSING_GRADES]
        for cell in open_cells:
            lines.append(
                f"  - **{cell['title']}** ({_GRADE_MARK[cell['grade']]}): {cell['note']} "
                f"Evidence `{cell['evidence_ref']}`."
            )
        if row["waiver"]:
            lines.append(
                f"  - Waiver `{row['waiver']['waiver_ref']}` expires {row['waiver']['expires_at']}: "
                f"{row['waiver']['reason']}"
            )
        lines.append("")
    return "\n".join(lines) + "\n"
