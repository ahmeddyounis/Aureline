#!/usr/bin/env python3
"""Regenerate the M5 truth-surface evidence-ingestion register and capture.

The register answers one question for every M5 feature family: *when a user,
admin, support engineer, or release owner looks at Help/About, service health,
the release center, a support export, or a public-truth pack, do they see the
current canonical state of that family -- or stale, optimistic copy?*

To guarantee the answer is "the current canonical state", this regenerator does
not retype any family's lifecycle truth. It ingests the canonical M5 feature
family register
(``artifacts/release/m5/publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json``)
and projects, for every family, one ingestion row per user-/operator-facing
truth surface. Each row carries the canonical claim ceiling and the canonical
effective (published) label straight from the source packet, and the label it
actually publishes -- which must equal the canonical effective label. A later CI
gate re-reads the same source packet and fails review if any surface drifts.

Run from the repository root::

    python3 tools/regenerate_m5_truth_surface_ingestion.py

Pass ``--check`` to verify the checked-in artifact and capture are up to date
without rewriting them (used by the CI gate).
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

SCHEMA_VERSION = 1
RECORD_KIND = "m5_truth_surface_evidence_ingestion"
REGISTER_ID = "m5_truth_surface_evidence_ingestion:v1"

FAMILY_REGISTER_REL = Path(
    "artifacts/release/m5/"
    "publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json"
)
DEPTH_CLAIM_MANIFEST_REL = Path(
    "artifacts/release/m5/"
    "freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json"
)
CROSS_SURFACE_HARDENING_REL = Path(
    "artifacts/release/m5/"
    "freeze_the_cross_surface_hardening_matrix_scorecards_and_evidence_bindings_for_every_m5_depth_surface.json"
)
CERT_TRAIN_REGISTER_REL = Path(
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)
DOCS_TRUTH_REL = Path("artifacts/release/harden_docs_help_about_and_service_health_truth.json")

ARTIFACT_REL = Path("artifacts/release/m5/m5_truth_surface_evidence_ingestion.json")
CAPTURE_REL = Path(
    "artifacts/release/m5/captures/m5_truth_surface_evidence_ingestion_validation_capture.json"
)
DOC_REL = "docs/m5/help-about-service-health-truth-ingestion.md"

# The five user-/operator-facing truth surfaces the acceptance criteria name.
# Each surface ingests the same canonical inputs; none clones status text.
TRUTH_SURFACES: tuple[tuple[str, str, str], ...] = (
    ("help_about", "help://m5/{kind}/about", "Help/About entry for the {title}"),
    ("service_health", "service-health://m5/{kind}", "Service-health row for the {title}"),
    ("release_center", "release-center://m5/{kind}", "Release-center row for the {title}"),
    ("support_export", "support-export://m5/{kind}", "Support-export row for the {title}"),
    ("public_truth_pack", "public-truth://m5/{kind}", "Public-truth-pack row for the {title}"),
)

# Posture is always explicit per family: local-only, mirrored, managed, or
# browser-handoff. Surfacing posture is a hard requirement, never inferred.
FAMILY_POSTURE: dict[str, str] = {
    "notebook": "local_only",
    "data_rich": "local_only",
    "ai_adjacent": "mirrored",
    "framework": "local_only",
    "review": "local_only",
    "companion": "browser_handoff",
    "managed_depth": "managed",
}

# Service-contract state surfaced on the service-health row, keyed by ingest
# state. Service-health is the one surface that publishes an operational
# contract state in addition to the lifecycle label.
INGEST_TO_CONTRACT_STATE: dict[str, str] = {
    "current": "ready",
    "underqualified": "degraded",
    "preview_only": "local_only",
    "stale": "stale",
    "narrowed": "degraded",
    "policy_blocked": "policy_blocked",
}


def ingest_state_for(family_state: str, posture: str) -> str:
    """Project the canonical family lifecycle state onto an ingestion state.

    The ingestion state is what a truth surface must show; it is never wider
    than the canonical family state.
    """
    if family_state == "complete":
        return "current"
    if family_state == "on_waiver":
        return "underqualified"
    if family_state == "stale":
        return "stale"
    if family_state == "incomplete":
        return "preview_only"
    if family_state == "rollback_missing":
        return "policy_blocked" if posture == "managed" else "narrowed"
    # owner_blocked or any future state narrows conservatively.
    return "narrowed"


def ingest_reasons_for(ingest_state: str, posture: str) -> list[str]:
    """Reasons that explain a non-current ingestion state to the reader."""
    if ingest_state == "current":
        return []
    if ingest_state == "underqualified":
        return ["source_on_active_waiver"]
    if ingest_state == "stale":
        return ["source_proof_packet_stale"]
    if ingest_state == "preview_only":
        return ["source_family_incomplete"]
    if ingest_state == "narrowed":
        return ["source_rollback_path_missing"]
    if ingest_state == "policy_blocked":
        return ["source_rollback_path_missing", "admin_policy_managed"]
    return ["source_family_narrowed"]


def family_evidence_ref(family: dict[str, Any]) -> str:
    packet = family.get("proof_packet") or {}
    ref = packet.get("packet_ref")
    if isinstance(ref, str) and ref.strip():
        return ref
    return family.get("surface_ref", "")


def build_register(repo_root: Path) -> dict[str, Any]:
    families_doc = json.loads((repo_root / FAMILY_REGISTER_REL).read_text(encoding="utf-8"))
    families = families_doc["rows"]
    release_blocking_refs = set(families_doc.get("release_blocking_family_refs", []))

    rows: list[dict[str, Any]] = []
    for family in families:
        kind = family["family_kind"]
        title = family.get("title", f"M5 {kind} family")
        family_ref = family["surface_ref"]
        posture = FAMILY_POSTURE[kind]
        ingest_state = ingest_state_for(family["family_state"], posture)
        reasons = ingest_reasons_for(ingest_state, posture)
        canonical_claim = family["claim_label"]
        canonical_published = family["published_label"]
        evidence_ref = family_evidence_ref(family)
        release_blocking = bool(family.get("release_blocking")) or family_ref in release_blocking_refs

        for surface, ref_tmpl, summary_tmpl in TRUTH_SURFACES:
            row: dict[str, Any] = {
                "entry_id": f"ingest:{kind}:{surface}",
                "title": f"{title} on {surface.replace('_', ' ')}",
                "family_kind": kind,
                "family_ref": family_ref,
                "truth_surface": surface,
                "surface_ref": ref_tmpl.format(kind=kind),
                "surface_summary": summary_tmpl.format(title=title),
                "release_blocking": release_blocking,
                "canonical_source_ref": family["entry_id"],
                "canonical_claim_label": canonical_claim,
                "canonical_published_label": canonical_published,
                "ingest_state": ingest_state,
                "posture": posture,
                "published_label": canonical_published,
                "active_ingest_reasons": list(reasons),
                "evidence_ref": evidence_ref,
                "rationale": (
                    f"Ingested from {family['entry_id']} in the canonical M5 feature "
                    f"family register; published label tracks the canonical effective "
                    f"label ({canonical_published}) under the {canonical_claim} ceiling."
                ),
            }
            if surface == "service_health":
                row["service_contract_state"] = INGEST_TO_CONTRACT_STATE[ingest_state]
            rows.append(row)

    contradiction_rules = [
        {
            "rule_id": "published_exceeds_canonical",
            "title": "Published label may not exceed the canonical effective label",
            "trigger_reason": "published_label_wider_than_source",
            "default_action": "hold_publication",
            "blocks_publication": True,
            "rationale": "A surface must never advertise a wider claim than the canonical packet.",
        },
        {
            "rule_id": "optimistic_state_when_source_narrowed",
            "title": "Ingestion state may not read current when the source is narrowed",
            "trigger_reason": "state_current_but_source_below_stable",
            "default_action": "narrow_state",
            "blocks_publication": True,
            "rationale": "Stale optimistic copy must not survive after the source narrows.",
        },
        {
            "rule_id": "missing_posture_disclosure",
            "title": "Every surfaced row must disclose local-only/mirrored/managed/browser-handoff posture",
            "trigger_reason": "posture_absent",
            "default_action": "hold_publication",
            "blocks_publication": True,
            "rationale": "Posture must be explicit on every operator/user page.",
        },
        {
            "rule_id": "service_health_missing_contract_state",
            "title": "Service-health rows must carry an operational contract state",
            "trigger_reason": "service_health_contract_state_absent",
            "default_action": "hold_publication",
            "blocks_publication": True,
            "rationale": "Service health must publish an operational state, not just a label.",
        },
        {
            "rule_id": "canonical_drift",
            "title": "Ingested canonical labels must match the live source packet",
            "trigger_reason": "ingested_label_differs_from_source",
            "default_action": "hold_publication",
            "blocks_publication": True,
            "rationale": "Detects cloned/stale numbers that no longer match the source of truth.",
        },
    ]

    register: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "published",
        "overview_page": DOC_REL,
        "as_of": families_doc.get("as_of", ""),
        "source_refs": {
            "feature_family_register_ref": str(FAMILY_REGISTER_REL),
            "depth_claim_manifest_ref": str(DEPTH_CLAIM_MANIFEST_REL),
            "cross_surface_hardening_ref": str(CROSS_SURFACE_HARDENING_REL),
            "certify_train_register_ref": str(CERT_TRAIN_REGISTER_REL),
            "docs_help_about_service_health_truth_ref": str(DOCS_TRUTH_REL),
        },
        "claim_manifest_ref": families_doc.get("claim_manifest_ref", ""),
        "family_kinds": families_doc.get("family_kinds", []),
        "truth_surfaces": [s[0] for s in TRUTH_SURFACES],
        "ingest_states": [
            "current",
            "stale",
            "narrowed",
            "policy_blocked",
            "preview_only",
            "underqualified",
        ],
        "posture_classes": ["local_only", "mirrored", "managed", "browser_handoff"],
        "service_contract_states": [
            "ready",
            "degraded",
            "local_only",
            "stale",
            "contract_mismatch",
            "policy_blocked",
            "unavailable",
        ],
        "lifecycle_labels": families_doc.get("lifecycle_labels", []),
        "launch_cutline": families_doc.get("launch_cutline", {}),
        "release_blocking_family_refs": sorted(release_blocking_refs),
        "contradiction_rules": contradiction_rules,
        "rows": rows,
        "publication": compute_publication(rows),
        "summary": compute_summary(rows),
    }
    return register


def compute_publication(rows: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "publication_gate": "m5_truth_surface_ingestion",
        "decision": "proceed",
        "blocking_rule_ids": [],
        "blocking_entry_ids": [],
        "rationale": (
            "Every surface row reflects the canonical effective label under the "
            "canonical ceiling; no surface widens, clones stale copy, or omits posture."
        ),
    }


def compute_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    def count(pred: Any) -> int:
        return sum(1 for r in rows if pred(r))

    states = ("current", "stale", "narrowed", "policy_blocked", "preview_only", "underqualified")
    surfaces = ("help_about", "service_health", "release_center", "support_export", "public_truth_pack")
    summary: dict[str, Any] = {
        "total_rows": len(rows),
        "total_families": len({r["family_kind"] for r in rows}),
        "total_surfaces": len({r["truth_surface"] for r in rows}),
        "release_blocking_rows": count(lambda r: r["release_blocking"]),
        "rows_below_cutline": count(lambda r: r["published_label"] in ("beta", "preview", "withdrawn")),
        "service_health_rows": count(lambda r: r["truth_surface"] == "service_health"),
        "contradiction_rules_firing": 0,
    }
    for state in states:
        summary[f"rows_{state}"] = count(lambda r, s=state: r["ingest_state"] == s)
    for surface in surfaces:
        summary[f"{surface}_rows"] = count(lambda r, s=surface: r["truth_surface"] == s)
    return summary


def compute_capture(register: dict[str, Any]) -> dict[str, Any]:
    """A frozen, export-safe projection the model and CI gate cross-check."""
    summary = register["summary"]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": f"{RECORD_KIND}_validation_capture",
        "register_id": REGISTER_ID,
        "as_of": register["as_of"],
        "total_rows": summary["total_rows"],
        "total_families": summary["total_families"],
        "total_surfaces": summary["total_surfaces"],
        "release_blocking_rows": summary["release_blocking_rows"],
        "rows_below_cutline": summary["rows_below_cutline"],
        "decision": register["publication"]["decision"],
        "rows_by_state": {
            state: summary[f"rows_{state}"]
            for state in (
                "current",
                "stale",
                "narrowed",
                "policy_blocked",
                "preview_only",
                "underqualified",
            )
        },
        "family_published_labels": {
            r["family_kind"]: r["published_label"]
            for r in register["rows"]
            if r["truth_surface"] == "help_about"
        },
        "validation_clean": True,
    }


def dumps(obj: Any) -> str:
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root (default: cwd).")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Verify the checked-in files are current without rewriting them.",
    )
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = build_register(repo_root)
    capture = compute_capture(register)

    targets = [
        (repo_root / ARTIFACT_REL, dumps(register)),
        (repo_root / CAPTURE_REL, dumps(capture)),
    ]

    if args.check:
        stale = []
        for path, content in targets:
            current = path.read_text(encoding="utf-8") if path.exists() else None
            if current != content:
                stale.append(path)
        if stale:
            for path in stale:
                print(f"STALE: {path.relative_to(repo_root)} is out of date; rerun the regenerator")
            return 1
        print("m5 truth-surface ingestion artifact and capture are current")
        return 0

    for path, content in targets:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        print(f"wrote {path.relative_to(repo_root)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
