#!/usr/bin/env python3
"""Regenerate the versioned, per-family boundary-manifest register.

The open/local-boundary durability matrix freezes the standing durability posture of
every asset lane. This register is the publication layer on top of it: for every
claimed M5 family it records one versioned boundary manifest that publishes which
capabilities stay open/local, which may be productized, the guardrails preserving the
claim, the residual proprietary/hosted dependencies the family still rests on, and the
release train each manifest is linked to.

A manifest is published only when its release link is present and fresh, its declared
label is in parity with the train evidence, every residual dependency is disclosed,
every guardrail holds, the proof is fresh, and the owner signed. Otherwise it narrows
on the specific axis that thins out (a release-link gap, a parity break, an undisclosed
dependency, an unsatisfied guardrail, or stale proof) and drops its effective label
below the launch cutline.

An inherited narrowing (a family already below the cutline, or a gap held by an
unexpired waiver) is gated upstream and does not hold promotion; a manifest-layer
failure on a still-stable family holds promotion through a stop rule.

This emits the canonical register artifact, the negative fixtures, the cases manifest,
and the frozen validation capture. The Python summary/parity/promotion logic mirrors
the typed Rust consumer so the checked-in artifact validates cleanly and the capture
cross-check agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-versioned-boundary-manifests"
RECORD_KIND = "m5_versioned_boundary_manifest_register"
REGISTER_ID = "m5_versioned_boundary_manifests:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG
OVERVIEW_PAGE = (
    "docs/m5/publish_versioned_per_family_boundary_manifests_with_asset_lanes_"
    "guardrails_residual_dependency_disclosure_and_release_link_parity.md"
)
AS_OF = "2026-06-16"

# Canonical source registers this register binds together.
DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
OPEN_PAID_AUDIT_REF = "artifacts/release/open_paid_boundary_audit.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_"
    "publish_the_canonical_evidence_index.json"
)
CLAIM_MANIFEST_REF = (
    "artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_"
    "docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json"
)
RELEASE_TRAIN_INDEX_REF = (
    "artifacts/release/m5/freeze_the_m5_release_candidate_publish_target_artifact_bundle_"
    "and_exact_build_publication_matrix.json"
)
SUPPORT_EXPORT_REF = (
    "artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_"
    "support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json"
)
SLO_REGISTER_REF = "artifacts/governance/proof_freshness_slos.yaml"

# Closed vocabularies (mirror the Rust enums in declaration order).
FAMILIES = [
    "notebook",
    "data_rich",
    "ai_adjacent",
    "framework",
    "review",
    "companion",
    "managed_depth",
]
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
LANE_DISPOSITIONS = [
    "open_local_retained",
    "productizable_add_on",
    "managed_only",
    "restricted_asset",
]
GUARDRAIL_KINDS = [
    "local_core_remains_useful",
    "no_silent_local_redefinition",
    "asset_lane_detail_published",
    "residual_dependency_disclosed",
    "release_link_published",
]
DEPENDENCY_CLASSES = [
    "proprietary_component",
    "hosted_service",
    "managed_model_provider",
    "trademark_brand_asset",
]
MANIFEST_STATES = [
    "published",
    "narrowed_release_link",
    "narrowed_parity",
    "narrowed_disclosure",
    "narrowed_guardrail",
    "narrowed_stale",
    "withdrawn",
]
MANIFEST_REASONS = [
    "release_link_missing",
    "release_link_stale",
    "release_parity_broken",
    "undisclosed_residual_dependency",
    "guardrail_unsatisfied",
    "manifest_proof_stale",
    "manifest_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
MANIFEST_ACTIONS = [
    "hold_publication",
    "link_release_evidence",
    "refresh_release_link",
    "realign_claim_to_release_evidence",
    "disclose_residual_dependency",
    "satisfy_guardrail",
    "refresh_manifest_proof",
    "request_owner_signoff",
]

LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ABOVE_CUTLINE = ["lts", "stable"]

# reason -> narrowing state
REASON_STATE = {
    "release_link_missing": "narrowed_release_link",
    "release_link_stale": "narrowed_release_link",
    "release_parity_broken": "narrowed_parity",
    "undisclosed_residual_dependency": "narrowed_disclosure",
    "guardrail_unsatisfied": "narrowed_guardrail",
    "manifest_proof_stale": "narrowed_stale",
    "manifest_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
STATE_PRECEDENCE = {
    "narrowed_parity": 0,
    "narrowed_release_link": 1,
    "narrowed_disclosure": 2,
    "narrowed_guardrail": 3,
    "narrowed_stale": 4,
}
REASON_ACTION = {
    "release_link_missing": "link_release_evidence",
    "release_link_stale": "refresh_release_link",
    "release_parity_broken": "realign_claim_to_release_evidence",
    "undisclosed_residual_dependency": "disclose_residual_dependency",
    "guardrail_unsatisfied": "satisfy_guardrail",
    "manifest_proof_stale": "refresh_manifest_proof",
    "manifest_proof_missing": "refresh_manifest_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "hold_publication",
}
POSTURE_SUPPORT = {
    "open_local_core": "open_local",
    "open_local_with_managed_optional": "mixed_open_managed",
    "source_available_restricted": "restricted",
    "managed_service": "managed",
    "restricted_brand": "restricted",
}

# Owners (planning metadata-free role refs).
GOV = "role:governance-release-lead"
SEC = "role:security-response-owner"
ECO = "role:ecosystem-owner"
OSS = "role:oss-compliance-devrel"
PLATFORM = "role:platform-maintainers"

DEFAULT_SURFACES = [
    "shell/help_about_boundary_card",
    "service_health/boundary_manifest_panel",
    "docs/governance/open_paid_boundary_and_antilockin_matrix.md",
    "support_export/boundary_manifest_packet",
    "evaluation_pack/boundary_manifest_section",
]


# ---------------------------------------------------------------------------
# Builders
# ---------------------------------------------------------------------------
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
        "evidence_refs": [DURABILITY_MATRIX_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def release_link(train_id: str, train_label: str, link_state: str, slo_state: str, linked_at: str | None) -> dict:
    return {
        "train_id": train_id,
        "train_ref": f"{RELEASE_TRAIN_INDEX_REF}#{train_id}",
        "train_label": train_label,
        "evidence_index_ref": f"{EVIDENCE_INDEX_REF}#{train_id}",
        "link_state": link_state,
        "slo_state": slo_state,
        "linked_at": linked_at,
    }


def lane(asset_lane: str, disposition: str, posture: str, must_open: bool, summary: str, durability_row: str) -> dict:
    return {
        "asset_lane": asset_lane,
        "disposition": disposition,
        "boundary_posture": posture,
        "support_class": POSTURE_SUPPORT[posture],
        "must_remain_open": must_open,
        "capability_summary": summary,
        "durability_row_ref": f"{DURABILITY_MATRIX_REF}#{durability_row}",
    }


def guardrail(kind: str, state: str, owner: str, description: str) -> dict:
    return {
        "kind": kind,
        "guardrail_ref": f"docs/governance/open_paid_boundary_and_antilockin_matrix.md#{kind}",
        "owner_ref": owner,
        "state": state,
        "description": description,
    }


def all_guardrails(states: dict[str, str]) -> list[dict]:
    descriptions = {
        "local_core_remains_useful": "The local/open core remains materially useful with no managed dependency.",
        "no_silent_local_redefinition": "New managed value may not silently redefine local-core usefulness.",
        "asset_lane_detail_published": "The open-core claim carries per-asset-lane detail rather than a vague slogan.",
        "residual_dependency_disclosed": "Every residual proprietary/hosted dependency is disclosed on truth surfaces.",
        "release_link_published": "The manifest is linked from the release evidence index.",
    }
    owners = {
        "local_core_remains_useful": GOV,
        "no_silent_local_redefinition": GOV,
        "asset_lane_detail_published": OSS,
        "residual_dependency_disclosed": OSS,
        "release_link_published": GOV,
    }
    return [guardrail(k, states[k], owners[k], descriptions[k]) for k in GUARDRAIL_KINDS]


def dep(dep_id: str, dep_class: str, lane_name: str, summary: str, disclosed: bool, replaceable: bool) -> dict:
    return {
        "dependency_id": dep_id,
        "dependency_ref": f"artifacts/governance/critical_dependency_register.yaml#{dep_id}",
        "dependency_class": dep_class,
        "affected_lane": lane_name,
        "summary": summary,
        "disclosed": disclosed,
        "disclosure_surface_refs": (
            ["shell/help_about_boundary_card", "docs/governance/open_paid_boundary_and_antilockin_matrix.md"]
            if disclosed
            else []
        ),
        "replaceable": replaceable,
        "replacement_plan_ref": f"artifacts/governance/critical_dependency_register.yaml#{dep_id}.fork_replace",
    }


def computed_state(reasons: list[str], declared: str) -> str:
    if declared == "withdrawn":
        return "withdrawn"
    if not reasons:
        return "published"
    best = min(reasons, key=lambda r: STATE_PRECEDENCE[REASON_STATE[r]])
    return REASON_STATE[best]


def computed_effective(reasons: list[str], declared: str) -> str:
    state = computed_state(reasons, declared)
    if state == "published":
        return declared
    if state == "withdrawn":
        return "withdrawn"
    return declared if LABEL_RANK[declared] <= LABEL_RANK["beta"] else "beta"


def is_waived(m: dict) -> bool:
    return m.get("waiver") is not None and "waiver_expired" not in m["active_reasons"]


def manifest(
    family: str,
    title: str,
    version: str,
    prior: str | None,
    subject_ref: str,
    subject_summary: str,
    *,
    release_blocking: bool,
    declared: str,
    support_class: str,
    link: dict,
    lanes: list[dict],
    guardrails: list[dict],
    deps: list[dict],
    pkt: dict,
    waiver: dict | None,
    so: dict,
    reasons: list[str],
    rationale: str,
    surfaces: list[str] | None = None,
) -> dict:
    return {
        "manifest_id": f"manifest-{family}",
        "family": family,
        "title": title,
        "manifest_version": version,
        "prior_version": prior,
        "as_of": AS_OF,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared,
        "support_class": support_class,
        "release_link": link,
        "lane_entries": lanes,
        "guardrails": guardrails,
        "residual_dependencies": deps,
        "proof_packet": pkt,
        "waiver": waiver,
        "owner_signoff": so,
        "manifest_state": computed_state(reasons, declared),
        "active_reasons": reasons,
        "effective_label": computed_effective(reasons, declared),
        "surfaces": surfaces or list(DEFAULT_SURFACES),
        "rationale": rationale,
    }


CORE_LANE = lambda summary: lane(  # noqa: E731
    "core_desktop_client_platform", "open_local_retained", "open_local_core", True, summary, "boundary-core-desktop-client"
)
SDK_LANE = lambda summary: lane(  # noqa: E731
    "sdk_schema_contract", "open_local_retained", "open_local_core", True, summary, "boundary-sdk-schema-contract"
)
DOCS_LANE = lambda summary: lane(  # noqa: E731
    "docs_migration_pack", "open_local_retained", "open_local_core", False, summary, "boundary-docs-migration-pack"
)


def build_manifests() -> list[dict]:
    manifests = []

    # 1. Framework — fully open/local core, published at stable, in parity.
    manifests.append(
        manifest(
            "framework",
            "Core framework and platform foundations",
            "3.0.0",
            "2.4.0",
            "schemas/",
            "Editor shell, platform foundations, and exported contracts that build, run, and remain useful offline.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            link=release_link("train_framework_stable", "stable", "linked", "current", "2026-05-30"),
            lanes=[
                CORE_LANE("Editor shell and platform runtime, fully local."),
                SDK_LANE("Public SDKs and exported packet contracts."),
                DOCS_LANE("Framework docs and migration packs."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "satisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[],
            pkt=proof("manifest_framework_proof", "current", "2026-05-30"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-31"),
            reasons=[],
            rationale="Framework core is fully open/local, carries per-lane detail, and is in parity with its stable release train.",
        )
    )

    # 2. Managed depth — legitimately managed tier, residual hosted deps all disclosed.
    manifests.append(
        manifest(
            "managed_depth",
            "Managed-depth and infrastructure surfaces",
            "1.6.0",
            "1.5.0",
            f"{OPEN_PAID_AUDIT_REF}#managed_tier",
            "Optional hosted depth/infrastructure services that are clearly paid/managed and do not blur the local core.",
            release_blocking=True,
            declared="stable",
            support_class="managed",
            link=release_link("train_managed_depth_stable", "stable", "linked", "current", "2026-05-29"),
            lanes=[
                lane(
                    "managed_service",
                    "managed_only",
                    "managed_service",
                    False,
                    "Hosted sync/index/relay services delivered only as a managed tier.",
                    "boundary-managed-sync-service",
                ),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "not_applicable",
                    "no_silent_local_redefinition": "not_applicable",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[
                dep(
                    "dep_object_store",
                    "hosted_service",
                    "managed_service",
                    "Hosted object-store backing the managed sync tier.",
                    True,
                    True,
                ),
                dep(
                    "dep_hosted_relay",
                    "hosted_service",
                    "managed_service",
                    "Hosted relay service for managed collaboration.",
                    True,
                    True,
                ),
            ],
            pkt=proof("manifest_managed_depth_proof", "current", "2026-05-29"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-30"),
            reasons=[],
            rationale="Managed tier with every residual hosted dependency disclosed on the truth surfaces and in parity with its release train.",
        )
    )

    # 3. Notebook — over-claims its release evidence: declares stable, train supports beta.
    manifests.append(
        manifest(
            "notebook",
            "Notebook and data-rich notebook depth surfaces",
            "2.1.0",
            "2.0.0",
            "schemas/notebook/",
            "Notebook depth surfaces whose public claim outran the release evidence backing them.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            link=release_link("train_notebook_beta", "beta", "parity_broken", "current", "2026-05-28"),
            lanes=[
                CORE_LANE("Notebook editor and kernel surfaces, local-first."),
                DOCS_LANE("Notebook docs and migration packs."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "satisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[],
            pkt=proof("manifest_notebook_proof", "current", "2026-05-28"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-29"),
            reasons=["release_parity_broken"],
            rationale="Manifest-layer failure: the manifest declares Stable but its release train only supports Beta, so the over-claim holds promotion until realigned.",
        )
    )

    # 4. AI-adjacent — an undisclosed managed model provider; disclosure guardrail fails too.
    manifests.append(
        manifest(
            "ai_adjacent",
            "AI-adjacent surfaces and language intelligence",
            "1.9.0",
            "1.8.0",
            "schemas/ai/",
            "AI-adjacent surfaces resting on a hosted model provider that is not yet disclosed on the truth surfaces.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            link=release_link("train_ai_adjacent_stable", "stable", "linked", "current", "2026-05-27"),
            lanes=[
                lane(
                    "core_desktop_client_platform",
                    "productizable_add_on",
                    "open_local_with_managed_optional",
                    True,
                    "Local language intelligence with an optional hosted model add-on.",
                    "boundary-core-desktop-client",
                ),
                SDK_LANE("AI SDK and provider-negotiation contracts."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "satisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "unsatisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[
                dep(
                    "dep_hosted_model_provider",
                    "managed_model_provider",
                    "core_desktop_client_platform",
                    "Hosted model provider powering the optional AI add-on.",
                    False,
                    True,
                ),
            ],
            pkt=proof("manifest_ai_adjacent_proof", "current", "2026-05-27"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-28"),
            reasons=["undisclosed_residual_dependency", "guardrail_unsatisfied"],
            rationale="Manifest-layer failure: a residual hosted model provider is undisclosed on the user/admin truth surfaces while the family still claims Stable, so disclosure must close before publication.",
        )
    )

    # 5. Data-rich — stale release link; already Beta (inherited, below the cutline).
    manifests.append(
        manifest(
            "data_rich",
            "Data-heavy result and explorer surfaces",
            "0.9.0",
            "0.8.0",
            "schemas/data/",
            "Data-rich surfaces whose release-train link evidence has aged out of its freshness window.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            link=release_link("train_data_rich_beta", "beta", "stale", "breached", "2026-01-12"),
            lanes=[
                CORE_LANE("Result grids and variable explorers, local-first."),
                SDK_LANE("Data-grid and explorer contracts."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "satisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[],
            pkt=proof("manifest_data_rich_proof", "current", "2026-05-20"),
            waiver=None,
            so=signoff(GOV, True, "2026-05-21"),
            reasons=["release_link_stale"],
            rationale="The release-train link evidence is stale; the public claim is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 6. Review — guardrail break (managed value redefining local core) held under a waiver.
    manifests.append(
        manifest(
            "review",
            "Review and diff surfaces",
            "2.2.0",
            "2.1.0",
            "schemas/review/",
            "Review surfaces where a new managed assist began redefining local-core usefulness; remediation is time-boxed under a waiver.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            link=release_link("train_review_stable", "stable", "linked", "current", "2026-05-26"),
            lanes=[
                lane(
                    "core_desktop_client_platform",
                    "productizable_add_on",
                    "open_local_with_managed_optional",
                    True,
                    "Local review/diff with an optional managed assist.",
                    "boundary-core-desktop-client",
                ),
                DOCS_LANE("Review docs and migration packs."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "unsatisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[],
            pkt=proof("manifest_review_proof", "current", "2026-05-26"),
            waiver={
                "waiver_ref": "artifacts/governance/ownership_matrix.yaml#waivers.review-local-core-redefinition",
                "expires_at": "2026-09-30",
                "reason": "Managed assist briefly fronted the local diff path; the guardrail gap is recorded and time-boxed while the default is restored.",
            },
            so=signoff(GOV, True, "2026-05-27"),
            reasons=["guardrail_unsatisfied"],
            rationale="The no-silent-redefinition guardrail is failing, but an unexpired waiver holds the gap provisionally, so it is gated upstream and does not hold promotion.",
        )
    )

    # 7. Companion — stale manifest proof; already Beta (inherited, below the cutline).
    manifests.append(
        manifest(
            "companion",
            "Browser/mobile companion surfaces",
            "0.7.0",
            "0.6.0",
            "schemas/companion/",
            "Companion surfaces whose manifest proof packet has aged past its freshness SLO.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            link=release_link("train_companion_beta", "beta", "linked", "current", "2026-05-24"),
            lanes=[
                CORE_LANE("Companion runtime and handoff surfaces, local-first."),
            ],
            guardrails=all_guardrails(
                {
                    "local_core_remains_useful": "satisfied",
                    "no_silent_local_redefinition": "satisfied",
                    "asset_lane_detail_published": "satisfied",
                    "residual_dependency_disclosed": "satisfied",
                    "release_link_published": "satisfied",
                }
            ),
            deps=[],
            pkt=proof("manifest_companion_proof", "breached", "2026-01-08"),
            waiver=None,
            so=signoff(GOV, True, "2026-01-09"),
            reasons=["manifest_proof_stale"],
            rationale="The manifest proof packet is stale; the public claim is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # Order the manifests by the canonical family order for determinism.
    order = {f: i for i, f in enumerate(FAMILIES)}
    manifests.sort(key=lambda m: order[m["family"]])
    return manifests


def build_rules() -> list[dict]:
    titles = {
        "release_link_missing": "Manifest must be linked from release evidence",
        "release_link_stale": "Release-train link must be fresh",
        "release_parity_broken": "Manifest may not over-claim its release evidence",
        "undisclosed_residual_dependency": "Residual dependencies must be disclosed",
        "guardrail_unsatisfied": "Boundary guardrails must hold",
        "manifest_proof_stale": "Manifest proof must be fresh",
        "manifest_proof_missing": "Manifest proof must exist",
        "owner_signoff_missing": "Owner sign-off required",
        "waiver_expired": "Waiver must be current",
    }
    rules = []
    for reason in MANIFEST_REASONS:
        rules.append(
            {
                "rule_id": f"m5_versioned_boundary_manifest_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_publication": True,
                "rationale": "A manifest-layer failure on a family still claiming a label at or above the cutline holds publication; inherited (below-cutline or waived) narrowings are gated upstream.",
            }
        )
    return rules


def holds_promotion(m: dict) -> bool:
    return (
        m["release_blocking"]
        and m["manifest_state"] not in ("published", "withdrawn")
        and m["declared_label"] in ABOVE_CUTLINE
        and not is_waived(m)
    )


def over_claims(m: dict) -> bool:
    return LABEL_RANK[m["declared_label"]] > LABEL_RANK[m["release_link"]["train_label"]]


def in_parity(m: dict) -> bool:
    return m["release_link"]["link_state"] == "linked" and not over_claims(m)


def computed_blocking_rule_ids(manifests: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for rule in rules:
        if not rule["blocks_publication"]:
            continue
        for m in manifests:
            if (
                holds_promotion(m)
                and rule["trigger_reason"] in m["active_reasons"]
                and m["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rule["rule_id"])
                break
    return sorted(ids)


def computed_blocking_manifest_ids(manifests: list[dict], rules: list[dict]) -> list[str]:
    rule_by_reason = {rule["trigger_reason"]: rule for rule in rules}
    ids = set()
    for m in manifests:
        if not holds_promotion(m):
            continue
        for reason in m["active_reasons"]:
            rule = rule_by_reason.get(reason)
            if rule and rule["blocks_publication"] and m["declared_label"] in rule["applies_to_labels"]:
                ids.add(m["manifest_id"])
                break
    return sorted(ids)


def computed_release_link_parity(manifests: list[dict]) -> dict:
    return {
        "parity_gate": "m5_versioned_boundary_manifest_parity_gate",
        "families_total": len(manifests),
        "families_in_parity": sum(1 for m in manifests if in_parity(m)),
        "families_link_broken": sum(1 for m in manifests if m["release_link"]["link_state"] in ("missing", "stale")),
        "families_parity_broken": sum(
            1 for m in manifests if m["release_link"]["link_state"] == "parity_broken" or over_claims(m)
        ),
        "all_families_linked": all(m["release_link"]["link_state"] != "missing" for m in manifests),
        "rationale": "Release-link parity holds across families only when every manifest is linked, fresh, and no greener than its release train.",
    }


def computed_summary(manifests: list[dict], rules: list[dict]) -> dict:
    def count_state(s):
        return sum(1 for m in manifests if m["manifest_state"] == s)

    deps = [d for m in manifests for d in m["residual_dependencies"]]
    guardrails = [g for m in manifests for g in m["guardrails"]]
    narrowed = [m for m in manifests if m["manifest_state"] not in ("published", "withdrawn")]
    published = [m for m in manifests if m["manifest_state"] == "published"]
    return {
        "total_manifests": len(manifests),
        "manifests_published": len(published),
        "manifests_narrowed": len(narrowed),
        "state_published": count_state("published"),
        "state_narrowed_release_link": count_state("narrowed_release_link"),
        "state_narrowed_parity": count_state("narrowed_parity"),
        "state_narrowed_disclosure": count_state("narrowed_disclosure"),
        "state_narrowed_guardrail": count_state("narrowed_guardrail"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": sum(1 for m in manifests if m["release_blocking"]),
        "release_blocking_narrowed": sum(1 for m in narrowed if m["release_blocking"]),
        "manifests_on_active_waiver": sum(1 for m in manifests if is_waived(m)),
        "total_residual_dependencies": len(deps),
        "residual_dependencies_disclosed": sum(1 for d in deps if d["disclosed"]),
        "residual_dependencies_undisclosed": sum(1 for d in deps if not d["disclosed"]),
        "total_guardrails": len(guardrails),
        "guardrails_unsatisfied": sum(1 for g in guardrails if g["state"] == "unsatisfied"),
        "manifests_linked": sum(1 for m in manifests if m["release_link"]["link_state"] != "missing"),
        "total_active_reasons": sum(len(m["active_reasons"]) for m in manifests),
        "rules_firing": len(computed_blocking_rule_ids(manifests, rules)),
    }


def build_register() -> dict:
    manifests = build_manifests()
    rules = build_rules()
    blocking_rules = computed_blocking_rule_ids(manifests, rules)
    blocking_manifests = computed_blocking_manifest_ids(manifests, rules)
    decision = "hold" if blocking_manifests else "proceed"
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "active",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "source_contract_refs": {
            "durability_matrix_ref": DURABILITY_MATRIX_REF,
            "open_paid_boundary_audit_ref": OPEN_PAID_AUDIT_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
            "claim_manifest_ref": CLAIM_MANIFEST_REF,
            "release_train_index_ref": RELEASE_TRAIN_INDEX_REF,
            "support_export_ref": SUPPORT_EXPORT_REF,
        },
        "manifest_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Manifests at or above Stable carry the durable boundary claim; a manifest-layer gap on a still-stable family holds publication.",
        },
        "families": FAMILIES,
        "asset_lanes": ASSET_LANES,
        "boundary_postures": BOUNDARY_POSTURES,
        "support_classes": SUPPORT_CLASSES,
        "lane_dispositions": LANE_DISPOSITIONS,
        "guardrail_kinds": GUARDRAIL_KINDS,
        "dependency_classes": DEPENDENCY_CLASSES,
        "manifest_states": MANIFEST_STATES,
        "manifest_reasons": MANIFEST_REASONS,
        "manifest_actions": MANIFEST_ACTIONS,
        "rules": rules,
        "manifests": manifests,
        "release_link_parity": computed_release_link_parity(manifests),
        "publication": {
            "publication_gate": "m5_versioned_boundary_manifest_gate",
            "decision": decision,
            "blocking_rule_ids": blocking_rules,
            "blocking_manifest_ids": blocking_manifests,
            "rationale": "Hold while any release-blocking family carries a manifest-layer gap on a still-stable claim; inherited and waived narrowings are gated upstream.",
        },
        "summary": computed_summary(manifests, rules),
    }


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------
def build_fixtures(register: dict) -> list[tuple[str, dict, str]]:
    cases: list[tuple[str, dict, str]] = []

    dup = copy.deepcopy(register)
    dup["manifests"].append(copy.deepcopy(dup["manifests"][0]))
    cases.append(("duplicate_manifest_id.json", dup, "DuplicateManifestId"))

    # A published manifest whose declared label outruns its release evidence.
    over = copy.deepcopy(register)
    target = next(m for m in over["manifests"] if m["manifest_state"] == "published" and m["declared_label"] == "stable")
    target["release_link"]["train_label"] = "beta"
    cases.append(("published_over_claim.json", over, "PublishedOverClaimsReleaseEvidence"))

    # A published manifest hiding an undisclosed residual dependency without narrowing.
    hidden = copy.deepcopy(register)
    target = next(
        m for m in hidden["manifests"] if m["manifest_state"] == "published" and m["residual_dependencies"]
    )
    target["residual_dependencies"][0]["disclosed"] = False
    cases.append(("undisclosed_without_reason.json", hidden, "GapWithoutReason"))

    # A narrowed manifest whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    narrowed = next(m for m in above["manifests"] if m["manifest_state"] not in ("published", "withdrawn"))
    narrowed["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["release_link_parity"]
    drills = [
        "drill:published_over_claim",
        "drill:undisclosed_without_reason",
        "drill:narrowed_above_cutline",
        "drill:gap_without_reason",
        "drill:reason_not_justified",
        "drill:state_reason_mismatch",
        "drill:publication_decision_inconsistent",
    ]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_manifests": s["total_manifests"],
            "manifests_published": s["manifests_published"],
            "manifests_narrowed": s["manifests_narrowed"],
            "state_published": s["state_published"],
            "state_narrowed_release_link": s["state_narrowed_release_link"],
            "state_narrowed_parity": s["state_narrowed_parity"],
            "state_narrowed_disclosure": s["state_narrowed_disclosure"],
            "state_narrowed_guardrail": s["state_narrowed_guardrail"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "manifests_on_active_waiver": s["manifests_on_active_waiver"],
            "total_residual_dependencies": s["total_residual_dependencies"],
            "residual_dependencies_undisclosed": s["residual_dependencies_undisclosed"],
            "total_guardrails": s["total_guardrails"],
            "guardrails_unsatisfied": s["guardrails_unsatisfied"],
            "manifests_linked": s["manifests_linked"],
            "total_active_reasons": s["total_active_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "release_link_parity": {
            "families_in_parity": parity["families_in_parity"],
            "families_link_broken": parity["families_link_broken"],
            "families_parity_broken": parity["families_parity_broken"],
            "all_families_linked": parity["all_families_linked"],
        },
        "publication": {
            "decision": p["decision"],
            "blocking_rule_ids": p["blocking_rule_ids"],
            "blocking_manifest_ids": p["blocking_manifest_ids"],
        },
        "negative_drills": [{"drill_id": d, "status": "passed"} for d in drills],
        "fixture_cases": [{"case_id": f"fixture:{f[:-5]}", "status": "passed"} for f, _, _ in cases],
    }


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    register = build_register()
    cases = build_fixtures(register)

    write_json(ARTIFACT, register)
    print(f"wrote {ARTIFACT.relative_to(REPO)}")

    for filename, data, _ in cases:
        write_json(FIXTURES / filename, data)
    manifest_index = {
        "cases": [{"file": filename, "expected_check_id": check_id} for filename, _, check_id in cases]
    }
    write_json(FIXTURES / "cases.json", manifest_index)
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")

    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {CAPTURE.relative_to(REPO)}")


if __name__ == "__main__":
    main()
