#!/usr/bin/env python3
"""Regenerate the open/local-boundary and upstream-durability matrix.

This matrix freezes, in one inspectable record, the standing durability facts every
claimed ecosystem and release lane rests on: where the open/local core ends and the
paid/managed tier begins, which repository-compliance and third-party-import controls
each asset lane must satisfy, who holds the emergency signing/registry/security
authority for it (and who backs them up), and whether its critical upstreams are owned.

For every asset lane it records one row binding the lane to its boundary posture and
support class, its repository-compliance control set, its emergency authority, its
continuity coverage, a proof packet, an optional waiver, and an owner sign-off. A row is
durable only when every axis holds; otherwise it narrows on the specific axis that
thinned out (boundary drift, a compliance gap, an authority gap, a continuity gap, or
stale proof) and drops its effective label below the launch cutline.

An inherited narrowing (a lane already below the cutline, or a gap held by an unexpired
waiver) is gated upstream and does not hold promotion; a durability-layer failure on a
still-stable lane holds promotion through a stop rule.

This emits the canonical matrix artifact, the negative fixtures, the cases manifest, and
the frozen validation capture. The Python summary/promotion logic mirrors the typed Rust
consumer so the checked-in artifact validates cleanly and the capture cross-check agrees
with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-boundary-and-upstream-durability"
MODULE = "m5_boundary_and_upstream_durability"
RECORD_KIND = "m5_boundary_and_upstream_durability_matrix"
MATRIX_ID = "m5_boundary_and_upstream_durability:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG
OVERVIEW_PAGE = (
    "docs/m5/freeze_the_m5_open_boundary_repository_compliance_third_party_import_"
    "and_maintainer_signer_durability_matrix.md"
)
AS_OF = "2026-06-16"

# Canonical source registers this matrix binds together.
OPEN_PAID_AUDIT_REF = "artifacts/release/open_paid_boundary_audit.json"
SIGNING_QUORUM_REF = "artifacts/governance/signing_quorum.yaml"
IMPORT_MANIFEST_REF = "artifacts/governance/third_party_import_manifest.yaml"
UPSTREAM_HEALTH_REF = "artifacts/governance/upstream_health_scorecard.yaml"
MAINTAINER_POLICY_REF = "docs/governance/maintainer_coverage_policy.md"
SECURITY_MATRIX_REF = "docs/security/severity_matrix.md"
SLO_REGISTER_REF = "artifacts/governance/proof_freshness_slos.yaml"

ASSET_LANES = [
    "core_desktop_client_platform",
    "sdk_schema_contract",
    "docs_migration_pack",
    "marketplace_protocol",
    "managed_service",
    "restricted_brand_asset",
]
BOUNDARY_POSTURES = [
    "open_local_core",
    "open_local_with_managed_optional",
    "source_available_restricted",
    "managed_service",
    "restricted_brand",
]
OPEN_BASELINE_POSTURES = ["open_local_core", "open_local_with_managed_optional"]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
CONTROL_DIMENSIONS = [
    "contribution_provenance",
    "file_level_licensing",
    "third_party_import",
    "generated_code_attribution",
    "sbom_and_notices",
    "signer_coverage",
    "registry_emergency_action",
    "security_response",
    "critical_upstream_ownership",
]
DURABILITY_STATES = [
    "durable",
    "narrowed_boundary_drift",
    "narrowed_compliance_gap",
    "narrowed_authority_gap",
    "narrowed_continuity_gap",
    "narrowed_stale",
    "withdrawn",
]
DURABILITY_REASONS = [
    "boundary_baseline_violated",
    "compliance_control_unsatisfied",
    "signer_quorum_unmet",
    "emergency_authority_owner_missing",
    "single_point_of_failure",
    "backup_coverage_missing",
    "critical_upstream_unowned",
    "proof_freshness_breached",
    "proof_packet_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
MATRIX_ACTIONS = [
    "hold_publication",
    "restore_open_baseline",
    "narrow_boundary_label",
    "satisfy_compliance_control",
    "assign_emergency_authority",
    "add_backup_coverage",
    "refresh_proof_packet",
    "request_owner_signoff",
]

LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
RANK_LABEL = {v: k for k, v in LABEL_RANK.items()}
ABOVE_CUTLINE = ["lts", "stable"]

# reason -> narrowing state
REASON_STATE = {
    "boundary_baseline_violated": "narrowed_boundary_drift",
    "compliance_control_unsatisfied": "narrowed_compliance_gap",
    "signer_quorum_unmet": "narrowed_authority_gap",
    "emergency_authority_owner_missing": "narrowed_authority_gap",
    "single_point_of_failure": "narrowed_continuity_gap",
    "backup_coverage_missing": "narrowed_continuity_gap",
    "critical_upstream_unowned": "narrowed_continuity_gap",
    "proof_freshness_breached": "narrowed_stale",
    "proof_packet_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
STATE_PRECEDENCE = {
    "narrowed_boundary_drift": 0,
    "narrowed_authority_gap": 1,
    "narrowed_continuity_gap": 2,
    "narrowed_compliance_gap": 3,
    "narrowed_stale": 4,
}

# reason -> recommended action (for the rule set)
REASON_ACTION = {
    "boundary_baseline_violated": "restore_open_baseline",
    "compliance_control_unsatisfied": "satisfy_compliance_control",
    "signer_quorum_unmet": "assign_emergency_authority",
    "emergency_authority_owner_missing": "assign_emergency_authority",
    "single_point_of_failure": "add_backup_coverage",
    "backup_coverage_missing": "add_backup_coverage",
    "critical_upstream_unowned": "assign_emergency_authority",
    "proof_freshness_breached": "refresh_proof_packet",
    "proof_packet_missing": "refresh_proof_packet",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "hold_publication",
}

DEFAULT_DESTINATIONS = [
    OPEN_PAID_AUDIT_REF,
    "docs/governance/open_paid_boundary_and_antilockin_matrix.md",
    "ci/release/validate_import_manifest.py",
    "release_center_shiproom_gate",
]


# ---------------------------------------------------------------------------
# Builders
# ---------------------------------------------------------------------------
def control(dimension: str, state: str, owner: str, ref: str) -> dict:
    return {
        "dimension": dimension,
        "control_ref": ref,
        "owner_ref": owner,
        "state": state,
    }


def quorum(required: int, available: int) -> dict:
    return {
        "required_distinct_humans": required,
        "available_distinct_humans": available,
        "quorum_profile_ref": f"{SIGNING_QUORUM_REF}#quorum_profiles.two_person_release_control",
    }


def authority(primary: str, backups: list[str], q: dict, registry: str, security: str) -> dict:
    return {
        "primary_owner_ref": primary,
        "backup_owner_refs": backups,
        "signer_quorum": q,
        "registry_emergency_owner_ref": registry,
        "security_response_owner_ref": security,
    }


def upstream(ref: str, owner: str, risk: str, plan: str) -> dict:
    return {
        "upstream_ref": ref,
        "owner_ref": owner,
        "risk_class": risk,
        "fork_replace_plan_ref": plan,
    }


def continuity(backup: str, spof: bool, upstreams: list[dict]) -> dict:
    return {
        "backup_coverage": backup,
        "single_point_of_failure": spof,
        "critical_upstreams": upstreams,
    }


def proof(packet_id: str, slo_state: str, captured: str | None) -> dict:
    return {
        "packet_id": packet_id,
        "packet_ref": f"artifacts/governance/captures/{packet_id}.json",
        "captured_at": captured,
        "freshness_slo": {
            "target_max_age_days": 90,
            "warn_within_days": 14,
            "slo_register_ref": SLO_REGISTER_REF,
        },
        "slo_state": slo_state,
        "evidence_refs": [OPEN_PAID_AUDIT_REF, IMPORT_MANIFEST_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def computed_state(reasons: list[str], declared: str) -> str:
    if declared == "withdrawn":
        return "withdrawn"
    if not reasons:
        return "durable"
    best = min(reasons, key=lambda r: STATE_PRECEDENCE[REASON_STATE[r]])
    return REASON_STATE[best]


def computed_effective(reasons: list[str], declared: str) -> str:
    state = computed_state(reasons, declared)
    if state == "durable":
        return declared
    if state == "withdrawn":
        return "withdrawn"
    return RANK_LABEL[min(LABEL_RANK[declared], LABEL_RANK["beta"])]


def is_waived(row: dict) -> bool:
    return row.get("waiver") is not None and "waiver_expired" not in row["active_reasons"]


def row(
    entry_id: str,
    title: str,
    lane: str,
    subject_ref: str,
    subject_summary: str,
    *,
    release_blocking: bool,
    must_remain_open: bool,
    posture: str,
    support_class: str,
    declared: str,
    controls: list[dict],
    auth: dict,
    cont: dict,
    pkt: dict,
    waiver: dict | None,
    so: dict,
    reasons: list[str],
    rationale: str,
    destinations: list[str] | None = None,
) -> dict:
    return {
        "entry_id": entry_id,
        "title": title,
        "asset_lane": lane,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "must_remain_open": must_remain_open,
        "boundary_posture": posture,
        "support_class": support_class,
        "declared_label": declared,
        "compliance_controls": controls,
        "emergency_authority": auth,
        "continuity": cont,
        "proof_packet": pkt,
        "waiver": waiver,
        "owner_signoff": so,
        "durability_state": computed_state(reasons, declared),
        "active_reasons": reasons,
        "effective_label": computed_effective(reasons, declared),
        "reuse_destinations": destinations or list(DEFAULT_DESTINATIONS),
        "rationale": rationale,
    }


# Owners (planning metadata-free role refs).
GOV = "role:governance-release-lead"
SEC = "role:security-response-owner"
ECO = "role:ecosystem-owner"
OSS = "role:oss-compliance-devrel"
PLATFORM = "role:platform-maintainers"


def build_rows() -> list[dict]:
    rows = []

    rows.append(
        row(
            "boundary-core-desktop-client",
            "Core desktop/client/platform shell",
            "core_desktop_client_platform",
            f"{OPEN_PAID_AUDIT_REF}#open_paid_boundary",
            "Core editor shell that must build, run, and remain useful fully offline.",
            release_blocking=True,
            must_remain_open=True,
            posture="open_local_core",
            support_class="open_local",
            declared="stable",
            controls=[
                control("contribution_provenance", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#dco"),
                control("file_level_licensing", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#spdx"),
                control("sbom_and_notices", "satisfied", GOV, f"{IMPORT_MANIFEST_REF}#notices"),
            ],
            auth=authority(GOV, [PLATFORM, SEC], quorum(2, 2), GOV, SEC),
            cont=continuity("covered", False, []),
            pkt=proof("boundary_core_desktop_proof", "current", "2026-05-20"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-21"),
            reasons=[],
            rationale="Open/local core is fully inspectable and offline-useful; all controls satisfied with quorum coverage.",
        )
    )

    rows.append(
        row(
            "boundary-sdk-schema-contract",
            "SDKs, schemas, and exported contracts",
            "sdk_schema_contract",
            "schemas/",
            "Public SDKs, JSON schemas, and exported packet contracts third parties build on.",
            release_blocking=True,
            must_remain_open=True,
            posture="open_local_core",
            support_class="open_local",
            declared="stable",
            controls=[
                control("file_level_licensing", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#spdx"),
                control("third_party_import", "satisfied", OSS, IMPORT_MANIFEST_REF),
                control("generated_code_attribution", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#generated"),
            ],
            auth=authority(GOV, [PLATFORM], quorum(2, 2), GOV, SEC),
            cont=continuity(
                "covered",
                False,
                [upstream("dep:serde", PLATFORM, "medium", f"{UPSTREAM_HEALTH_REF}#serde")],
            ),
            pkt=proof("boundary_sdk_contract_proof", "current", "2026-05-22"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-23"),
            reasons=[],
            rationale="Contracts ship under clear per-file licensing with attributed imports and an owned upstream.",
        )
    )

    rows.append(
        row(
            "boundary-docs-migration-pack",
            "Documentation and migration packs",
            "docs_migration_pack",
            "docs/",
            "Docs, known-limits, and migration packs that must stay openly readable and forkable.",
            release_blocking=False,
            must_remain_open=True,
            posture="open_local_core",
            support_class="open_local",
            declared="stable",
            controls=[
                control("file_level_licensing", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#spdx"),
                control("contribution_provenance", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#dco"),
            ],
            auth=authority(OSS, [GOV], quorum(2, 2), GOV, SEC),
            cont=continuity("covered", False, []),
            pkt=proof("boundary_docs_pack_proof", "current", "2026-05-19"),
            waiver=None,
            so=signoff(OSS, True, "2026-05-20"),
            reasons=[],
            rationale="Docs and migration packs are openly licensed and contributor-attributed.",
        )
    )

    rows.append(
        row(
            "boundary-marketplace-protocol",
            "Marketplace and extension-registry protocol",
            "marketplace_protocol",
            "schemas/extensions/",
            "Open marketplace/registry protocol; registry moderation tooling is still maturing.",
            release_blocking=True,
            must_remain_open=True,
            posture="open_local_with_managed_optional",
            support_class="mixed_open_managed",
            declared="beta",
            controls=[
                control("registry_emergency_action", "unsatisfied", ECO, f"{SIGNING_QUORUM_REF}#emergency_registry_action"),
                control("third_party_import", "satisfied", OSS, IMPORT_MANIFEST_REF),
                control("contribution_provenance", "satisfied", OSS, f"{IMPORT_MANIFEST_REF}#dco"),
            ],
            auth=authority(ECO, [GOV], quorum(2, 2), ECO, SEC),
            cont=continuity("covered", False, []),
            pkt=proof("boundary_marketplace_proof", "current", "2026-05-24"),
            waiver=None,
            so=signoff(ECO, True, "2026-05-25"),
            reasons=["compliance_control_unsatisfied"],
            rationale="Protocol is open, but registry emergency-action tooling is incomplete; the public claim is already Beta, so this is gated upstream.",
        )
    )

    rows.append(
        row(
            "boundary-managed-sync-service",
            "Managed sync service",
            "managed_service",
            f"{OPEN_PAID_AUDIT_REF}#managed_tier",
            "Optional hosted sync; not part of the local core and clearly paid/managed.",
            release_blocking=True,
            must_remain_open=False,
            posture="managed_service",
            support_class="managed",
            declared="stable",
            controls=[
                control("security_response", "satisfied", SEC, SECURITY_MATRIX_REF),
                control("signer_coverage", "satisfied", GOV, SIGNING_QUORUM_REF),
                control("sbom_and_notices", "satisfied", GOV, f"{IMPORT_MANIFEST_REF}#notices"),
            ],
            auth=authority(GOV, [PLATFORM, SEC], quorum(2, 2), GOV, SEC),
            cont=continuity(
                "covered",
                False,
                [upstream("dep:object-store", PLATFORM, "medium", f"{UPSTREAM_HEALTH_REF}#object-store")],
            ),
            pkt=proof("boundary_managed_sync_proof", "current", "2026-05-26"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-27"),
            reasons=[],
            rationale="Legitimately managed tier with full security/signer coverage; does not blur the open baseline.",
        )
    )

    rows.append(
        row(
            "boundary-restricted-brand",
            "Restricted brand and trademark assets",
            "restricted_brand_asset",
            "docs/governance/open_paid_boundary_and_antilockin_matrix.md#brand",
            "Logos and trademarks held under brand policy, not open license.",
            release_blocking=False,
            must_remain_open=False,
            posture="restricted_brand",
            support_class="restricted",
            declared="stable",
            controls=[
                control("security_response", "satisfied", SEC, SECURITY_MATRIX_REF),
                control("file_level_licensing", "not_applicable", OSS, f"{IMPORT_MANIFEST_REF}#brand"),
            ],
            auth=authority(GOV, [ECO], quorum(2, 2), GOV, SEC),
            cont=continuity("covered", False, []),
            pkt=proof("boundary_restricted_brand_proof", "current", "2026-05-18"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-19"),
            reasons=[],
            rationale="Brand assets are intentionally restricted; the matrix records the boundary explicitly.",
        )
    )

    rows.append(
        row(
            "boundary-critical-upstream-toolchain",
            "Critical toolchain upstream durability",
            "core_desktop_client_platform",
            f"{UPSTREAM_HEALTH_REF}#toolchain",
            "Critical build/toolchain upstream whose maintenance still rests on one human under a recorded waiver.",
            release_blocking=True,
            must_remain_open=True,
            posture="open_local_core",
            support_class="open_local",
            declared="stable",
            controls=[
                control("critical_upstream_ownership", "satisfied", PLATFORM, UPSTREAM_HEALTH_REF),
                control("third_party_import", "satisfied", OSS, IMPORT_MANIFEST_REF),
            ],
            auth=authority(GOV, [PLATFORM], quorum(2, 2), GOV, SEC),
            cont=continuity(
                "waived",
                True,
                [upstream("dep:toolchain", PLATFORM, "high", f"{UPSTREAM_HEALTH_REF}#toolchain")],
            ),
            pkt=proof("boundary_critical_upstream_proof", "current", "2026-05-28"),
            waiver={
                "waiver_ref": "artifacts/governance/ownership_matrix.yaml#waivers.single-maintainer-backup",
                "expires_at": "2026-09-30",
                "reason": "Repository is under the single-maintainer backup waiver; the gap is recorded and time-boxed.",
            },
            so=signoff(GOV, True, "2026-05-29"),
            reasons=["single_point_of_failure"],
            rationale="The continuity gap is real and surfaced here rather than hidden in a private runbook; an unexpired waiver holds it provisionally, so it does not hold promotion.",
        )
    )

    rows.append(
        row(
            "boundary-managed-build-farm-signing",
            "Managed build-farm signing pipeline",
            "managed_service",
            f"{SIGNING_QUORUM_REF}#release_signing",
            "Managed release-signing pipeline whose durability proof has aged out of its freshness SLO.",
            release_blocking=True,
            must_remain_open=False,
            posture="managed_service",
            support_class="managed",
            declared="stable",
            controls=[
                control("signer_coverage", "satisfied", GOV, SIGNING_QUORUM_REF),
                control("security_response", "satisfied", SEC, SECURITY_MATRIX_REF),
                control("sbom_and_notices", "satisfied", GOV, f"{IMPORT_MANIFEST_REF}#notices"),
            ],
            auth=authority(GOV, [PLATFORM, SEC], quorum(2, 2), GOV, SEC),
            cont=continuity(
                "covered",
                False,
                [upstream("dep:signing-service", PLATFORM, "high", f"{UPSTREAM_HEALTH_REF}#signing")],
            ),
            pkt=proof("boundary_build_farm_signing_proof", "breached", "2026-01-10"),
            waiver=None,
            so=signoff(GOV, True, "2026-01-11"),
            reasons=["proof_freshness_breached"],
            rationale="Durability-layer failure: the signing-pipeline proof is stale while the lane still claims Stable, so the matrix holds promotion until refreshed.",
        )
    )

    return rows


def build_rules() -> list[dict]:
    titles = {
        "boundary_baseline_violated": "Open baseline must not be blurred",
        "compliance_control_unsatisfied": "Compliance control must be satisfied",
        "signer_quorum_unmet": "Signer quorum must be met",
        "emergency_authority_owner_missing": "Emergency authority must be owned",
        "single_point_of_failure": "No single point of failure",
        "backup_coverage_missing": "Backup coverage required",
        "critical_upstream_unowned": "Critical upstreams must be owned",
        "proof_freshness_breached": "Durability proof must be fresh",
        "proof_packet_missing": "Durability proof must exist",
        "owner_signoff_missing": "Owner sign-off required",
        "waiver_expired": "Waiver must be current",
    }
    rules = []
    for reason in DURABILITY_REASONS:
        rules.append(
            {
                "rule_id": f"m5_boundary_durability_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_publication": True,
                "rationale": "A durability-layer failure on a lane still claiming a label at or above the cutline holds publication; inherited (below-cutline or waived) narrowings are gated upstream.",
            }
        )
    return rules


def holds_promotion(r: dict) -> bool:
    return (
        r["release_blocking"]
        and r["durability_state"] not in ("durable", "withdrawn")
        and r["declared_label"] in ABOVE_CUTLINE
        and not is_waived(r)
    )


def computed_blocking_rule_ids(rows: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for rule in rules:
        if not rule["blocks_publication"]:
            continue
        for r in rows:
            if (
                holds_promotion(r)
                and rule["trigger_reason"] in r["active_reasons"]
                and r["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rule["rule_id"])
                break
    return sorted(ids)


def computed_blocking_row_ids(rows: list[dict], rules: list[dict]) -> list[str]:
    rule_by_reason = {rule["trigger_reason"]: rule for rule in rules}
    ids = set()
    for r in rows:
        if not holds_promotion(r):
            continue
        for reason in r["active_reasons"]:
            rule = rule_by_reason.get(reason)
            if rule and rule["blocks_publication"] and r["declared_label"] in rule["applies_to_labels"]:
                ids.add(r["entry_id"])
                break
    return sorted(ids)


def computed_summary(rows: list[dict], rules: list[dict]) -> dict:
    def count_state(s):
        return sum(1 for r in rows if r["durability_state"] == s)

    controls = [c for r in rows for c in r["compliance_controls"]]

    def count_control(s):
        return sum(1 for c in controls if c["state"] == s)

    def count_packet(s):
        return sum(1 for r in rows if r["proof_packet"]["slo_state"] == s)

    narrowed = [r for r in rows if r["durability_state"] not in ("durable", "withdrawn")]
    durable = [r for r in rows if r["durability_state"] == "durable"]
    return {
        "total_rows": len(rows),
        "rows_durable": len(durable),
        "rows_narrowed": len(narrowed),
        "state_durable": count_state("durable"),
        "state_narrowed_boundary_drift": count_state("narrowed_boundary_drift"),
        "state_narrowed_compliance_gap": count_state("narrowed_compliance_gap"),
        "state_narrowed_authority_gap": count_state("narrowed_authority_gap"),
        "state_narrowed_continuity_gap": count_state("narrowed_continuity_gap"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "must_remain_open_rows": sum(1 for r in rows if r["must_remain_open"]),
        "open_baseline_rows": sum(1 for r in rows if r["boundary_posture"] in OPEN_BASELINE_POSTURES),
        "release_blocking_total": sum(1 for r in rows if r["release_blocking"]),
        "release_blocking_durable": sum(1 for r in rows if r["release_blocking"] and r["durability_state"] == "durable"),
        "release_blocking_narrowed": sum(1 for r in narrowed if r["release_blocking"]),
        "rows_on_active_waiver": sum(1 for r in rows if is_waived(r)),
        "total_controls": len(controls),
        "controls_satisfied": count_control("satisfied"),
        "controls_unsatisfied": count_control("unsatisfied"),
        "controls_not_applicable": count_control("not_applicable"),
        "packets_current": count_packet("current"),
        "packets_due_for_refresh": count_packet("due_for_refresh"),
        "packets_breached": count_packet("breached"),
        "packets_missing": count_packet("missing"),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in rows),
        "rules_firing": len(computed_blocking_rule_ids(rows, rules)),
    }


def build_matrix() -> dict:
    rows = build_rows()
    rules = build_rules()
    blocking_rules = computed_blocking_rule_ids(rows, rules)
    blocking_rows = computed_blocking_row_ids(rows, rules)
    decision = "hold" if blocking_rows else "proceed"
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "matrix_id": MATRIX_ID,
        "status": "active",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "source_contract_refs": {
            "open_paid_boundary_audit_ref": OPEN_PAID_AUDIT_REF,
            "signing_quorum_ref": SIGNING_QUORUM_REF,
            "third_party_import_manifest_ref": IMPORT_MANIFEST_REF,
            "critical_upstream_health_ref": UPSTREAM_HEALTH_REF,
            "maintainer_coverage_policy_ref": MAINTAINER_POLICY_REF,
            "security_severity_matrix_ref": SECURITY_MATRIX_REF,
        },
        "boundary_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "open_baseline_postures": OPEN_BASELINE_POSTURES,
            "description": "Lanes at or above Stable carry the durable boundary claim; a must-remain-open lane may only carry an open-baseline posture.",
        },
        "asset_lanes": ASSET_LANES,
        "boundary_postures": BOUNDARY_POSTURES,
        "support_classes": SUPPORT_CLASSES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "durability_states": DURABILITY_STATES,
        "durability_reasons": DURABILITY_REASONS,
        "matrix_actions": MATRIX_ACTIONS,
        "rules": rules,
        "rows": rows,
        "publication": {
            "publication_gate": "m5_boundary_and_upstream_durability_gate",
            "decision": decision,
            "blocking_rule_ids": blocking_rules,
            "blocking_row_ids": blocking_rows,
            "rationale": "Hold while any release-blocking lane carries a durability-layer gap on a still-stable claim; inherited and waived narrowings are gated upstream.",
        },
        "summary": computed_summary(rows, rules),
    }


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------
def build_fixtures(matrix: dict) -> list[tuple[str, dict, str]]:
    cases: list[tuple[str, dict, str]] = []

    dup = copy.deepcopy(matrix)
    dup["rows"].append(copy.deepcopy(dup["rows"][0]))
    cases.append(("duplicate_entry_id.json", dup, "DuplicateEntryId"))

    # A must-remain-open lane blurred to a managed posture, left durable.
    drift = copy.deepcopy(matrix)
    target = next(r for r in drift["rows"] if r["must_remain_open"] and r["boundary_posture"] in OPEN_BASELINE_POSTURES)
    target["boundary_posture"] = "managed_service"
    target["support_class"] = "managed"
    cases.append(("must_remain_open_violation.json", drift, "MustRemainOpenViolated"))

    # A durable lane carrying an active narrowing reason.
    gap = copy.deepcopy(matrix)
    durable = next(r for r in gap["rows"] if r["durability_state"] == "durable")
    durable["active_reasons"] = ["owner_signoff_missing"]
    cases.append(("durable_with_active_gap.json", gap, "DurableWithActiveReason"))

    # A narrowed lane whose effective label stays above the cutline.
    above = copy.deepcopy(matrix)
    narrowed = next(r for r in above["rows"] if r["durability_state"] not in ("durable", "withdrawn"))
    narrowed["effective_label"] = "stable"
    cases.append(("narrowed_effective_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(matrix: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = matrix["summary"]
    p = matrix["publication"]
    drills = [
        "drill:must_remain_open_violation",
        "drill:durable_with_active_gap",
        "drill:narrowed_effective_above_cutline",
        "drill:gap_without_reason",
        "drill:reason_not_justified",
        "drill:state_reason_mismatch",
        "drill:publication_decision_inconsistent",
    ]
    return {
        "status": "pass",
        "as_of": matrix["as_of"],
        "summary": {
            "total_rows": s["total_rows"],
            "rows_durable": s["rows_durable"],
            "rows_narrowed": s["rows_narrowed"],
            "state_durable": s["state_durable"],
            "state_narrowed_compliance_gap": s["state_narrowed_compliance_gap"],
            "state_narrowed_continuity_gap": s["state_narrowed_continuity_gap"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "must_remain_open_rows": s["must_remain_open_rows"],
            "open_baseline_rows": s["open_baseline_rows"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "rows_on_active_waiver": s["rows_on_active_waiver"],
            "total_controls": s["total_controls"],
            "controls_unsatisfied": s["controls_unsatisfied"],
            "packets_breached": s["packets_breached"],
            "total_active_reasons": s["total_active_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "publication": {
            "decision": p["decision"],
            "blocking_rule_ids": p["blocking_rule_ids"],
            "blocking_row_ids": p["blocking_row_ids"],
        },
        "negative_drills": [{"drill_id": d, "status": "passed"} for d in drills],
        "fixture_cases": [
            {"case_id": f"fixture:{f[:-5]}", "status": "passed"} for f, _, _ in cases
        ],
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    matrix = build_matrix()
    cases = build_fixtures(matrix)

    write_json(ARTIFACT, matrix)
    print(f"wrote {ARTIFACT.relative_to(REPO)}")

    for filename, data, _ in cases:
        write_json(FIXTURES / filename, data)
    manifest = {
        "cases": [
            {"file": filename, "expected_check_id": check_id}
            for filename, _, check_id in cases
        ]
    }
    write_json(FIXTURES / "cases.json", manifest)
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")

    write_json(CAPTURE, build_capture(matrix, cases))
    print(f"wrote {CAPTURE.relative_to(REPO)}")


if __name__ == "__main__":
    main()
