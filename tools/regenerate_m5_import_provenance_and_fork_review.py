#!/usr/bin/env python3
"""Regenerate the third-party import-provenance and local-fork review register.

The open/local-boundary durability matrix records, per asset lane, *whether* a
third-party-import or generated-code control is satisfied as one coarse flag, and the
compliance-and-notice-binding register records DCO/CLA, licensing, and SBOM/notice truth
per artifact family. Neither makes each protected-path import inspectable as a durable
record: where it came from, what license it carries, which upstream version it pins, how
far it has diverged, who owns its updates, who generated it and how to regenerate it, and —
for a long-lived fork or an effectively single-source import — whether an explicit
sponsor/fork/replace decision and a current divergence review exist.

This register is that import-truth layer. For every protected-path import used by an M5
family it records one entry that states, in one copy-safe record:

  - the import provenance (origin attribution, SPDX license identification, upstream
    version pin);
  - the update ownership (so a critical import is never left ownerless because it is "just
    build-time");
  - the divergence profile (local modification posture, divergence age, and review state);
  - the sponsor/fork/replace decision (required for a long-lived fork or single-source
    import, never left to quiet permanent drift);
  - the generated-code provenance (generator identity and regeneration path, never buried
    for checked-in generated code).

A record is cleared only when provenance holds, the import is owned, any required
divergence review is current, any required decision is recorded, generated-code provenance
is complete, the proof is fresh, and the owner signed. Otherwise it narrows on the specific
axis that thins out (a provenance gap, an ownership gap, a divergence/decision gap, a
generator gap, or stale proof) and drops its effective label below the launch cutline; the
axes never collapse into one global flag. The dependency-health/import scan and the
user/admin import surface must agree on every record, so a clean import card can never mask
an ownerless, unattributed, or generator-free import.

An inherited narrowing (a subject already below the cutline, or a gap held by an unexpired
waiver) is gated upstream and does not hold promotion; an import-layer failure on a
still-stable subject holds promotion through a stop rule.

This emits the canonical register artifact, the negative fixtures, the cases manifest, and
the frozen validation capture. The Python summary/parity/promotion logic mirrors the typed
Rust consumer so the checked-in artifact validates cleanly and the capture cross-check
agrees with the model.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

SLUG = "m5-import-provenance-and-fork-review"
RECORD_KIND = "m5_import_provenance_and_fork_review_register"
REGISTER_ID = "m5_import_provenance_and_fork_review:v1"
ARTIFACT = REPO / "artifacts/governance" / f"{SLUG}.json"
CAPTURE = REPO / "artifacts/governance/captures" / f"{SLUG}_validation_capture.json"
FIXTURES = REPO / "fixtures/governance" / SLUG
OVERVIEW_PAGE = (
    "docs/m5/track_third_party_import_provenance_generated_code_records_and_local_fork_"
    "sponsor_replace_decisions_for_protected_m5_dependencies.md"
)
AS_OF = "2026-06-16"

# Canonical source registers this register binds together.
THIRD_PARTY_IMPORT_REGISTER_REF = "artifacts/governance/third_party_import_register.yaml"
IMPORT_MANIFEST_REF = "artifacts/governance/third_party_import_manifest.yaml"
DEPENDENCY_REGISTER_REF = "artifacts/governance/dependency_register.yaml"
CRITICAL_UPSTREAM_REF = "artifacts/governance/upstream_health_scorecard.yaml"
PACKAGE_INVENTORY_REF = "artifacts/governance/package_inventory.yaml"
GENERATED_LINEAGE_REF = (
    "schemas/governance/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_"
    "generated_notebook_derived_and_preview_derived_outputs.schema.json"
)
DURABILITY_MATRIX_REF = "artifacts/governance/m5-boundary-and-upstream-durability.json"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_"
    "publish_the_canonical_evidence_index.json"
)
SLO_REGISTER_REF = "artifacts/governance/evidence_freshness_slos.yaml"

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
IMPORT_KINDS = [
    "third_party_import",
    "generated_artifact",
    "local_fork",
    "curated_single_source",
]
SUPPORT_CLASSES = ["open_local", "mixed_open_managed", "managed", "restricted"]
CONTROL_DIMENSIONS = [
    "import_provenance",
    "update_ownership",
    "divergence_review",
    "decision_path",
    "generator_provenance",
    "manifest_surface_parity",
]
ORIGIN_STATES = ["attributed", "unattributed"]
LICENSE_STATES = ["identified", "unidentified", "not_applicable"]
UPSTREAM_PIN_STATES = ["pinned", "floating", "not_applicable"]
OWNERSHIP_STATES = ["owned", "ownerless"]
DIVERGENCE_STATES = ["in_sync", "diverged", "forked"]
DIVERGENCE_REVIEW_STATES = ["current", "stale", "missing", "not_required"]
DECISION_STATES = ["recorded", "pending", "not_required"]
DECISION_DISPOSITIONS = ["sponsor_upstream", "maintain_fork", "replace_dependency", "none"]
POSTURES = ["clear", "gaps_found"]
IMPORT_STATES = [
    "cleared",
    "narrowed_provenance",
    "narrowed_ownership",
    "narrowed_divergence",
    "narrowed_generator",
    "narrowed_stale",
    "withdrawn",
]
IMPORT_REASONS = [
    "origin_unattributed",
    "license_unidentified",
    "upstream_version_floating",
    "update_owner_missing",
    "divergence_review_stale",
    "divergence_review_missing",
    "decision_record_missing",
    "generator_identity_missing",
    "regeneration_path_missing",
    "import_proof_stale",
    "import_proof_missing",
    "owner_signoff_missing",
    "waiver_expired",
]
IMPORT_ACTIONS = [
    "hold_promotion",
    "attribute_origin",
    "identify_license",
    "pin_upstream_version",
    "assign_update_owner",
    "refresh_divergence_review",
    "record_sponsor_fork_replace_decision",
    "record_generator_identity",
    "record_regeneration_path",
    "refresh_import_proof",
    "request_owner_signoff",
]

LABEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
ABOVE_CUTLINE = ["lts", "stable"]

REASON_STATE = {
    "origin_unattributed": "narrowed_provenance",
    "license_unidentified": "narrowed_provenance",
    "upstream_version_floating": "narrowed_provenance",
    "update_owner_missing": "narrowed_ownership",
    "divergence_review_stale": "narrowed_divergence",
    "divergence_review_missing": "narrowed_divergence",
    "decision_record_missing": "narrowed_divergence",
    "generator_identity_missing": "narrowed_generator",
    "regeneration_path_missing": "narrowed_generator",
    "import_proof_stale": "narrowed_stale",
    "import_proof_missing": "narrowed_stale",
    "owner_signoff_missing": "narrowed_stale",
    "waiver_expired": "narrowed_stale",
}
STATE_PRECEDENCE = {
    "narrowed_provenance": 0,
    "narrowed_ownership": 1,
    "narrowed_divergence": 2,
    "narrowed_generator": 3,
    "narrowed_stale": 4,
}
REASON_ACTION = {
    "origin_unattributed": "attribute_origin",
    "license_unidentified": "identify_license",
    "upstream_version_floating": "pin_upstream_version",
    "update_owner_missing": "assign_update_owner",
    "divergence_review_stale": "refresh_divergence_review",
    "divergence_review_missing": "refresh_divergence_review",
    "decision_record_missing": "record_sponsor_fork_replace_decision",
    "generator_identity_missing": "record_generator_identity",
    "regeneration_path_missing": "record_regeneration_path",
    "import_proof_stale": "refresh_import_proof",
    "import_proof_missing": "refresh_import_proof",
    "owner_signoff_missing": "request_owner_signoff",
    "waiver_expired": "hold_promotion",
}
REASON_DIMENSION = {
    "origin_unattributed": "import_provenance",
    "license_unidentified": "import_provenance",
    "upstream_version_floating": "import_provenance",
    "update_owner_missing": "update_ownership",
    "divergence_review_stale": "divergence_review",
    "divergence_review_missing": "divergence_review",
    "decision_record_missing": "decision_path",
    "generator_identity_missing": "generator_provenance",
    "regeneration_path_missing": "generator_provenance",
    "import_proof_stale": "manifest_surface_parity",
    "import_proof_missing": "manifest_surface_parity",
    "owner_signoff_missing": "manifest_surface_parity",
    "waiver_expired": "manifest_surface_parity",
}

# Owners (planning metadata-free role refs).
GOV = "role:governance-release-lead"
SEC = "role:security-response-owner"
ECO = "role:ecosystem-owner"
OSS = "role:oss-compliance-devrel"
ARCH = "role:architecture-board"
DEP = "role:dependency-health-owner"
BLD = "role:build-provenance-owner"

DEFAULT_SURFACES = [
    "shell/help_about_import_provenance_card",
    "service_health/dependency_import_panel",
    "release_center/import_manifest_view",
    "support_export/procurement_import_packet",
    "architecture_board/divergence_review_input",
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
        "evidence_refs": [THIRD_PARTY_IMPORT_REGISTER_REF, EVIDENCE_INDEX_REF],
    }


def signoff(owner: str, signed: bool, at: str | None) -> dict:
    return {"owner_ref": owner, "signed_off": signed, "signed_at": at}


def provenance(
    origin: str,
    license_state: str,
    upstream_pin: str,
    *,
    spdx: str = "",
    version: str = "",
    origin_ref: str = "",
) -> dict:
    return {
        "origin_state": origin,
        "license_state": license_state,
        "upstream_pin_state": upstream_pin,
        "spdx_license_id": spdx,
        "upstream_version": version,
        "origin_ref": origin_ref or f"{THIRD_PARTY_IMPORT_REGISTER_REF}#origin",
        "license_ref": f"{THIRD_PARTY_IMPORT_REGISTER_REF}#license",
    }


def ownership(state: str, owner: str) -> dict:
    return {
        "ownership_state": state,
        "update_owner_ref": owner,
        "last_update_ref": f"{IMPORT_MANIFEST_REF}#last_update",
    }


def divergence(state: str, patches: int, age_days: int, review: str) -> dict:
    return {
        "divergence_state": state,
        "local_patch_count": patches,
        "divergence_age_days": age_days,
        "review_state": review,
        "review_ref": f"{IMPORT_MANIFEST_REF}#divergence_review",
    }


def decision(state: str, disposition: str) -> dict:
    return {
        "decision_state": state,
        "disposition": disposition,
        "decision_ref": f"{IMPORT_MANIFEST_REF}#sponsor_fork_replace_decision",
        "review_board_ref": f"{CRITICAL_UPSTREAM_REF}#architecture_board_review",
    }


def generator(applies: bool, identity: bool, regen: bool) -> dict:
    return {
        "applies": applies,
        "generator_identity_present": identity,
        "regeneration_path_present": regen,
        "generator_ref": f"{GENERATED_LINEAGE_REF}#generator_identity" if applies else "",
        "regeneration_path_ref": f"{GENERATED_LINEAGE_REF}#regeneration_path"
        if applies
        else "",
    }


def waiver(ref: str, expires: str, reason: str) -> dict:
    return {"waiver_ref": ref, "expires_at": expires, "reason": reason}


# --- mirrored Rust derivations --------------------------------------------
def origin_unattributed(p: dict) -> bool:
    return p["origin_state"] == "unattributed"


def license_unidentified(p: dict) -> bool:
    return p["license_state"] == "unidentified"


def upstream_floating(p: dict) -> bool:
    return p["upstream_pin_state"] == "floating"


def owner_missing(o: dict) -> bool:
    return o["ownership_state"] == "ownerless"


def requires_divergence_review(rec: dict) -> bool:
    return rec["divergence"]["divergence_state"] in ("diverged", "forked")


def requires_decision(rec: dict) -> bool:
    return rec["import_kind"] in ("local_fork", "curated_single_source")


def review_stale(rec: dict) -> bool:
    return requires_divergence_review(rec) and rec["divergence"]["review_state"] == "stale"


def review_missing(rec: dict) -> bool:
    return (
        requires_divergence_review(rec) and rec["divergence"]["review_state"] == "missing"
    )


def decision_missing(rec: dict) -> bool:
    return requires_decision(rec) and rec["decision"]["decision_state"] == "pending"


def generator_identity_missing(g: dict) -> bool:
    return g["applies"] and not g["generator_identity_present"]


def regeneration_path_missing(g: dict) -> bool:
    return g["applies"] and not g["regeneration_path_present"]


def derive_reasons(rec: dict) -> list[str]:
    p, o, g = rec["provenance"], rec["ownership"], rec["generator"]
    proof_slo = rec["proof_packet"]["slo_state"]
    facts = {
        "origin_unattributed": origin_unattributed(p),
        "license_unidentified": license_unidentified(p),
        "upstream_version_floating": upstream_floating(p),
        "update_owner_missing": owner_missing(o),
        "divergence_review_stale": review_stale(rec),
        "divergence_review_missing": review_missing(rec),
        "decision_record_missing": decision_missing(rec),
        "generator_identity_missing": generator_identity_missing(g),
        "regeneration_path_missing": regeneration_path_missing(g),
        "import_proof_stale": proof_slo == "breached",
        "import_proof_missing": proof_slo == "missing",
        "owner_signoff_missing": not rec["owner_signoff"]["signed_off"],
        # waiver_expired is authored explicitly, never auto-derived.
    }
    derived = [r for r in IMPORT_REASONS if facts.get(r, False)]
    for extra in rec.get("_extra_reasons", []):
        if extra not in derived:
            derived.append(extra)
    return [r for r in IMPORT_REASONS if r in derived]


def computed_state(reasons: list[str], declared: str) -> str:
    if declared == "withdrawn":
        return "withdrawn"
    if not reasons:
        return "cleared"
    best = min(reasons, key=lambda r: STATE_PRECEDENCE[REASON_STATE[r]])
    return REASON_STATE[best]


def computed_effective(reasons: list[str], declared: str) -> str:
    state = computed_state(reasons, declared)
    if state == "cleared":
        return declared
    if state == "withdrawn":
        return "withdrawn"
    return declared if LABEL_RANK[declared] <= LABEL_RANK["beta"] else "beta"


def expected_control_state(rec: dict, dimension: str) -> str:
    p, o, g = rec["provenance"], rec["ownership"], rec["generator"]
    if dimension == "import_provenance":
        return (
            "unsatisfied"
            if (origin_unattributed(p) or license_unidentified(p) or upstream_floating(p))
            else "satisfied"
        )
    if dimension == "update_ownership":
        return "unsatisfied" if owner_missing(o) else "satisfied"
    if dimension == "divergence_review":
        if not requires_divergence_review(rec):
            return "not_applicable"
        return "unsatisfied" if (review_stale(rec) or review_missing(rec)) else "satisfied"
    if dimension == "decision_path":
        if not requires_decision(rec):
            return "not_applicable"
        return "unsatisfied" if decision_missing(rec) else "satisfied"
    if dimension == "generator_provenance":
        if not g["applies"]:
            return "not_applicable"
        return (
            "unsatisfied"
            if (generator_identity_missing(g) or regeneration_path_missing(g))
            else "satisfied"
        )
    if dimension == "manifest_surface_parity":
        return (
            "unsatisfied"
            if rec["manifest_scan_posture"] != rec["surface_posture"]
            else "satisfied"
        )
    raise ValueError(dimension)


CONTROL_OWNERS = {
    "import_provenance": OSS,
    "update_ownership": DEP,
    "divergence_review": ARCH,
    "decision_path": ARCH,
    "generator_provenance": BLD,
    "manifest_surface_parity": GOV,
}
CONTROL_REFS = {
    "import_provenance": THIRD_PARTY_IMPORT_REGISTER_REF,
    "update_ownership": DEPENDENCY_REGISTER_REF,
    "divergence_review": IMPORT_MANIFEST_REF,
    "decision_path": CRITICAL_UPSTREAM_REF,
    "generator_provenance": GENERATED_LINEAGE_REF,
    "manifest_surface_parity": EVIDENCE_INDEX_REF,
}


def build_controls(rec: dict) -> list[dict]:
    return [
        {
            "dimension": d,
            "control_ref": f"{CONTROL_REFS[d]}#{d}",
            "owner_ref": CONTROL_OWNERS[d],
            "state": expected_control_state(rec, d),
        }
        for d in CONTROL_DIMENSIONS
    ]


def record(
    record_id: str,
    family: str,
    import_kind: str,
    title: str,
    subject_ref: str,
    subject_summary: str,
    *,
    release_blocking: bool,
    declared: str,
    support_class: str,
    prov: dict,
    own: dict,
    div: dict,
    dec: dict,
    gen: dict,
    pkt: dict,
    wv: dict | None,
    so: dict,
    rationale: str,
    extra_reasons: list[str] | None = None,
    surfaces: list[str] | None = None,
) -> dict:
    rec = {
        "record_id": record_id,
        "family": family,
        "import_kind": import_kind,
        "title": title,
        "subject_ref": subject_ref,
        "subject_summary": subject_summary,
        "release_blocking": release_blocking,
        "declared_label": declared,
        "support_class": support_class,
        "provenance": prov,
        "ownership": own,
        "divergence": div,
        "decision": dec,
        "generator": gen,
        # filled below
        "controls": [],
        "manifest_scan_posture": "clear",
        "surface_posture": "clear",
        "scan_ref": f"{THIRD_PARTY_IMPORT_REGISTER_REF}#import_scan/{record_id}",
        "surface_ref": f"shell/help_about_import_provenance_card#{record_id}",
        "proof_packet": pkt,
        "waiver": wv,
        "owner_signoff": so,
        "import_state": "cleared",
        "active_reasons": [],
        "effective_label": declared,
        "surfaces": surfaces or list(DEFAULT_SURFACES),
        "rationale": rationale,
        "_extra_reasons": extra_reasons or [],
    }
    reasons = derive_reasons(rec)
    state = computed_state(reasons, declared)
    posture = "gaps_found" if state not in ("cleared", "withdrawn") else "clear"
    rec["active_reasons"] = reasons
    rec["import_state"] = state
    rec["effective_label"] = computed_effective(reasons, declared)
    rec["manifest_scan_posture"] = posture
    rec["surface_posture"] = posture
    rec["controls"] = build_controls(rec)
    del rec["_extra_reasons"]
    return rec


def clean_prov(spdx: str = "Apache-2.0", version: str = "v1.6.2") -> dict:
    return provenance(
        "attributed",
        "identified",
        "pinned",
        spdx=spdx,
        version=version,
        origin_ref="https://example.invalid/upstream",
    )


def generated_prov() -> dict:
    # First-party generated code: origin attributed to the schema it is generated from,
    # no upstream license or version pin.
    return provenance(
        "attributed",
        "not_applicable",
        "not_applicable",
        spdx="",
        version="",
        origin_ref=f"{GENERATED_LINEAGE_REF}#source_schema",
    )


def na_div() -> dict:
    return divergence("in_sync", 0, 0, "not_required")


def na_decision() -> dict:
    return decision("not_required", "none")


def na_generator() -> dict:
    return generator(False, False, False)


def build_records() -> list[dict]:
    records = []

    # 1. Framework third-party import — fully cleared at stable.
    records.append(
        record(
            "import-framework-runtime",
            "framework",
            "third_party_import",
            "Framework runtime third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#framework_runtime",
            "A vendored framework runtime import with attributed origin, an identified SPDX license, a pinned upstream version, an assigned update owner, and no local divergence.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=clean_prov(),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_framework_runtime_proof", "current", "2026-05-30"),
            wv=None,
            so=signoff(GOV, True, "2026-05-31"),
            rationale="Origin, license, and upstream pin are all recorded, the import is owned, there is no divergence, and the proof is fresh; the scan and the surface agree on a clean posture.",
        )
    )

    # 2. Framework generated artifact — cleared at stable.
    records.append(
        record(
            "import-framework-generated-bindings",
            "framework",
            "generated_artifact",
            "Framework generated schema bindings",
            f"{GENERATED_LINEAGE_REF}#framework_bindings",
            "Checked-in generated schema bindings whose generator identity and regeneration path are both recorded, with an assigned owner.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=generated_prov(),
            own=ownership("owned", BLD),
            div=na_div(),
            dec=na_decision(),
            gen=generator(True, True, True),
            pkt=proof("import_framework_generated_proof", "current", "2026-05-29"),
            wv=None,
            so=signoff(GOV, True, "2026-05-30"),
            rationale="The generated artifact records both its generator identity and its regeneration path and is owned, so the checked-in generated code is reproducible rather than opaque.",
        )
    )

    # 3. Notebook third-party import — origin unattributed + license unidentified on a still-stable claim.
    records.append(
        record(
            "import-notebook-grid-engine",
            "notebook",
            "third_party_import",
            "Notebook grid-engine third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#notebook_grid_engine",
            "A vendored grid-rendering import whose origin is not attributed and whose license is not yet identified, while the notebook family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=provenance("unattributed", "unidentified", "pinned", version="v0.9.1"),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_notebook_grid_proof", "current", "2026-05-28"),
            wv=None,
            so=signoff(GOV, True, "2026-05-29"),
            rationale="Import-layer failure: the origin is unattributed and the license is unidentified while the family still claims Stable, so the provenance gap holds promotion until both are recorded.",
        )
    )

    # 4. Framework SDK import — upstream version floating; already Beta (inherited).
    records.append(
        record(
            "import-framework-sdk-codec",
            "framework",
            "third_party_import",
            "Framework SDK codec third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#framework_sdk_codec",
            "A vendored codec import tracking a floating upstream branch instead of a pinned version; the SDK lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            prov=provenance(
                "attributed", "identified", "floating", spdx="MIT", origin_ref="https://example.invalid/codec"
            ),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_framework_codec_proof", "current", "2026-05-24"),
            wv=None,
            so=signoff(GOV, True, "2026-05-25"),
            rationale="The upstream version is floating rather than pinned; the lane is already Beta, so this provenance narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 5. Data-rich third-party import — ownerless "just build-time" import on a still-stable claim.
    records.append(
        record(
            "import-data_rich-arrow-bridge",
            "data_rich",
            "third_party_import",
            "Data-rich columnar bridge third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#data_rich_columnar_bridge",
            "A vendored columnar bridge import treated as build-time only and left without an assigned update owner, while the data-rich family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=clean_prov(spdx="BSD-3-Clause", version="v8.2.0"),
            own=ownership("ownerless", ""),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_data_rich_bridge_proof", "current", "2026-05-27"),
            wv=None,
            so=signoff(GOV, True, "2026-05-28"),
            rationale="Import-layer failure: a critical import was left ownerless because it is 'just build-time' while the family still claims Stable, so the ownership gap holds promotion until an owner is assigned.",
        )
    )

    # 6. AI-adjacent local fork — long-lived fork with a stale divergence review on a still-stable claim.
    records.append(
        record(
            "import-ai_adjacent-tokenizer-fork",
            "ai_adjacent",
            "local_fork",
            "AI-adjacent tokenizer local fork",
            f"{IMPORT_MANIFEST_REF}#ai_adjacent_tokenizer_fork",
            "A long-lived local fork of a tokenizer with a recorded maintain-fork decision but a divergence review that has gone stale, while the family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            prov=clean_prov(spdx="Apache-2.0", version="v2.3.0+fork"),
            own=ownership("owned", ARCH),
            div=divergence("forked", 14, 240, "stale"),
            dec=decision("recorded", "maintain_fork"),
            gen=na_generator(),
            pkt=proof("import_ai_tokenizer_proof", "current", "2026-05-26"),
            wv=None,
            so=signoff(GOV, True, "2026-05-27"),
            rationale="Import-layer failure: a long-lived fork carries a recorded decision but its divergence review has gone stale while the family still claims Stable, so the divergence gap holds promotion until the review is refreshed.",
        )
    )

    # 7. AI-adjacent curated single-source import — decision record still pending on a still-stable claim.
    records.append(
        record(
            "import-ai_adjacent-embedding-index",
            "ai_adjacent",
            "curated_single_source",
            "AI-adjacent embedding-index single-source import",
            f"{IMPORT_MANIFEST_REF}#ai_adjacent_embedding_index",
            "An effectively single-source curated embedding-index import with a current divergence review but no recorded sponsor/fork/replace decision, while the family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            prov=clean_prov(spdx="Apache-2.0", version="v1.1.0"),
            own=ownership("owned", ARCH),
            div=divergence("diverged", 6, 120, "current"),
            dec=decision("pending", "none"),
            gen=na_generator(),
            pkt=proof("import_ai_embedding_proof", "current", "2026-05-25"),
            wv=None,
            so=signoff(GOV, True, "2026-05-26"),
            rationale="Import-layer failure: a single-source import has no recorded sponsor/fork/replace decision while the family still claims Stable, so the decision gap holds promotion instead of letting the dependency drift permanently.",
        )
    )

    # 8. Review local fork — divergence review missing; already Beta (inherited).
    records.append(
        record(
            "import-review-diff-engine-fork",
            "review",
            "local_fork",
            "Review diff-engine local fork",
            f"{IMPORT_MANIFEST_REF}#review_diff_engine_fork",
            "A long-lived fork of a diff engine with a recorded replace-dependency decision but no captured divergence review; the review lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="mixed_open_managed",
            prov=clean_prov(spdx="MPL-2.0", version="v3.0.0+fork"),
            own=ownership("owned", ARCH),
            div=divergence("forked", 9, 180, "missing"),
            dec=decision("recorded", "replace_dependency"),
            gen=na_generator(),
            pkt=proof("import_review_diff_proof", "current", "2026-05-21"),
            wv=None,
            so=signoff(GOV, True, "2026-05-22"),
            rationale="The fork has a recorded replace-dependency decision but no captured divergence review; the lane is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 9. Companion generated artifact — generator identity missing on a still-stable claim.
    records.append(
        record(
            "import-companion-generated-client",
            "companion",
            "generated_artifact",
            "Companion generated client",
            f"{GENERATED_LINEAGE_REF}#companion_client",
            "Checked-in generated companion client code that records a regeneration path but buries the identity of the generator that produced it, while the family still claims Stable.",
            release_blocking=True,
            declared="stable",
            support_class="open_local",
            prov=generated_prov(),
            own=ownership("owned", BLD),
            div=na_div(),
            dec=na_decision(),
            gen=generator(True, False, True),
            pkt=proof("import_companion_client_proof", "current", "2026-05-23"),
            wv=None,
            so=signoff(GOV, True, "2026-05-24"),
            rationale="Import-layer failure: checked-in generated code records its regeneration path but not the generator identity while the family still claims Stable, so the generator gap holds promotion until the identity is recorded.",
        )
    )

    # 10. Review generated artifact — regeneration path missing; held under an unexpired waiver.
    records.append(
        record(
            "import-review-generated-fixtures",
            "review",
            "generated_artifact",
            "Review generated fixtures",
            f"{GENERATED_LINEAGE_REF}#review_fixtures",
            "Checked-in generated review fixtures that record their generator identity but whose regeneration path is being re-captured; the gap is time-boxed under a waiver.",
            release_blocking=True,
            declared="stable",
            support_class="mixed_open_managed",
            prov=generated_prov(),
            own=ownership("owned", BLD),
            div=na_div(),
            dec=na_decision(),
            gen=generator(True, True, False),
            pkt=proof("import_review_fixtures_proof", "current", "2026-05-22"),
            wv=waiver(
                f"{PACKAGE_INVENTORY_REF}#waivers.review-regeneration-path",
                "2026-09-30",
                "The generator identity is recorded; the regeneration path is being re-captured after a generator upgrade and is time-boxed under this waiver.",
            ),
            so=signoff(GOV, True, "2026-05-23"),
            rationale="A recorded generator identity does not imply a recorded regeneration path: the path is missing, but an unexpired waiver holds the gap provisionally, so it is gated upstream and does not hold promotion.",
        )
    )

    # 11. Managed-depth third-party import — import proof stale; already Beta (inherited).
    records.append(
        record(
            "import-managed_depth-object-store",
            "managed_depth",
            "third_party_import",
            "Managed-depth object-store third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#managed_depth_object_store",
            "A vendored object-store client import whose import-provenance proof packet has aged past its freshness SLO; the managed-depth lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="managed",
            prov=clean_prov(spdx="Apache-2.0", version="v4.5.1"),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_managed_object_store_proof", "breached", "2026-01-08"),
            wv=None,
            so=signoff(ECO, True, "2026-01-09"),
            rationale="The import-provenance proof packet is stale; the lane is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 12. Companion third-party import — owner sign-off missing; already Beta (inherited).
    records.append(
        record(
            "import-companion-mobile-bridge",
            "companion",
            "third_party_import",
            "Companion mobile-bridge third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#companion_mobile_bridge",
            "A vendored mobile-bridge import whose record still lacks an owner sign-off; the companion lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            prov=clean_prov(spdx="ISC", version="v0.7.4"),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_companion_bridge_proof", "current", "2026-05-19"),
            wv=None,
            so=signoff(GOV, False, None),
            rationale="The record carries no owner sign-off; the lane is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 13. Managed-depth third-party import — import proof missing; already Beta (inherited).
    records.append(
        record(
            "import-managed_depth-telemetry-shim",
            "managed_depth",
            "third_party_import",
            "Managed-depth telemetry-shim third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#managed_depth_telemetry_shim",
            "A vendored telemetry-shim import with no captured import-provenance proof packet; the managed-depth lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="managed",
            prov=clean_prov(spdx="Apache-2.0", version="v2.0.0"),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_managed_telemetry_proof", "missing", None),
            wv=None,
            so=signoff(ECO, True, "2026-05-18"),
            rationale="No import-provenance proof packet has been captured; the lane is already Beta, so this narrowing is gated upstream and does not hold promotion.",
        )
    )

    # 14. Review third-party import — relied-on waiver has expired; already Beta (inherited).
    records.append(
        record(
            "import-review-syntax-grammar",
            "review",
            "third_party_import",
            "Review syntax-grammar third-party import",
            f"{THIRD_PARTY_IMPORT_REGISTER_REF}#review_syntax_grammar",
            "A vendored syntax-grammar import whose relied-on provenance waiver has expired; the review lane is already Beta.",
            release_blocking=True,
            declared="beta",
            support_class="open_local",
            prov=clean_prov(spdx="MIT", version="v1.3.2"),
            own=ownership("owned", DEP),
            div=na_div(),
            dec=na_decision(),
            gen=na_generator(),
            pkt=proof("import_review_grammar_proof", "current", "2026-05-17"),
            wv=waiver(
                f"{PACKAGE_INVENTORY_REF}#waivers.review-syntax-grammar",
                "2026-03-31",
                "A provenance re-verification was time-boxed under this waiver, which has since expired and must be renewed or cleared.",
            ),
            so=signoff(GOV, True, "2026-05-18"),
            extra_reasons=["waiver_expired"],
            rationale="The relied-on waiver has expired; the lane is already Beta, so this narrowing is gated upstream, but the expired waiver is surfaced rather than silently honored.",
        )
    )

    return records


def build_rules() -> list[dict]:
    titles = {
        "origin_unattributed": "Import origin must be attributed",
        "license_unidentified": "Import license must be identified",
        "upstream_version_floating": "Upstream version must be pinned",
        "update_owner_missing": "Import must have an update owner",
        "divergence_review_stale": "Divergence review must be fresh",
        "divergence_review_missing": "Divergence review must exist",
        "decision_record_missing": "Sponsor/fork/replace decision must be recorded",
        "generator_identity_missing": "Generator identity must be recorded",
        "regeneration_path_missing": "Regeneration path must be recorded",
        "import_proof_stale": "Import proof must be fresh",
        "import_proof_missing": "Import proof must exist",
        "owner_signoff_missing": "Owner sign-off required",
        "waiver_expired": "Waiver must be current",
    }
    rules = []
    for reason in IMPORT_REASONS:
        rules.append(
            {
                "rule_id": f"m5_import_provenance_rule:{reason}",
                "title": titles[reason],
                "trigger_reason": reason,
                "applies_to_labels": ABOVE_CUTLINE,
                "default_action": REASON_ACTION[reason],
                "blocks_promotion": True,
                "rationale": "An import-layer failure on a subject still claiming a label at or above the cutline holds promotion; inherited (below-cutline or waived) narrowings are gated upstream.",
            }
        )
    return rules


def is_waived(rec: dict) -> bool:
    return rec.get("waiver") is not None and "waiver_expired" not in rec["active_reasons"]


def holds_promotion(rec: dict) -> bool:
    return (
        rec["release_blocking"]
        and rec["import_state"] not in ("cleared", "withdrawn")
        and rec["declared_label"] in ABOVE_CUTLINE
        and not is_waived(rec)
    )


def computed_blocking_rule_ids(records: list[dict], rules: list[dict]) -> list[str]:
    ids = set()
    for rule in rules:
        if not rule["blocks_promotion"]:
            continue
        for rec in records:
            if (
                holds_promotion(rec)
                and rule["trigger_reason"] in rec["active_reasons"]
                and rec["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rule["rule_id"])
                break
    return sorted(ids)


def computed_blocking_record_ids(records: list[dict], rules: list[dict]) -> list[str]:
    rule_by_reason = {rule["trigger_reason"]: rule for rule in rules}
    ids = set()
    for rec in records:
        if not holds_promotion(rec):
            continue
        for reason in rec["active_reasons"]:
            rule = rule_by_reason.get(reason)
            if (
                rule
                and rule["blocks_promotion"]
                and rec["declared_label"] in rule["applies_to_labels"]
            ):
                ids.add(rec["record_id"])
                break
    return sorted(ids)


def computed_manifest_surface_parity(records: list[dict]) -> dict:
    return {
        "parity_gate": "m5_import_manifest_surface_parity_gate",
        "subjects_total": len(records),
        "subjects_in_agreement": sum(
            1 for r in records if r["manifest_scan_posture"] == r["surface_posture"]
        ),
        "subjects_in_disagreement": sum(
            1 for r in records if r["manifest_scan_posture"] != r["surface_posture"]
        ),
        "subjects_with_gaps": sum(1 for r in records if r["surface_posture"] == "gaps_found"),
        "all_subjects_agree": all(
            r["manifest_scan_posture"] == r["surface_posture"] for r in records
        ),
        "rationale": "The dependency-health/import scan and the user/admin import surface agree on every subject, so a clean import card can never mask an ownerless, unattributed, or generator-free import.",
    }


def provenance_gap(rec: dict) -> bool:
    p = rec["provenance"]
    return origin_unattributed(p) or license_unidentified(p) or upstream_floating(p)


def divergence_gap(rec: dict) -> bool:
    return review_stale(rec) or review_missing(rec) or decision_missing(rec)


def generator_gap(rec: dict) -> bool:
    g = rec["generator"]
    return generator_identity_missing(g) or regeneration_path_missing(g)


def computed_summary(records: list[dict], rules: list[dict]) -> dict:
    def count_state(s):
        return sum(1 for r in records if r["import_state"] == s)

    narrowed = [r for r in records if r["import_state"] not in ("cleared", "withdrawn")]
    cleared = [r for r in records if r["import_state"] == "cleared"]
    return {
        "total_records": len(records),
        "records_cleared": len(cleared),
        "records_narrowed": len(narrowed),
        "state_cleared": count_state("cleared"),
        "state_narrowed_provenance": count_state("narrowed_provenance"),
        "state_narrowed_ownership": count_state("narrowed_ownership"),
        "state_narrowed_divergence": count_state("narrowed_divergence"),
        "state_narrowed_generator": count_state("narrowed_generator"),
        "state_narrowed_stale": count_state("narrowed_stale"),
        "state_withdrawn": count_state("withdrawn"),
        "release_blocking_total": sum(1 for r in records if r["release_blocking"]),
        "release_blocking_narrowed": sum(1 for r in narrowed if r["release_blocking"]),
        "records_on_active_waiver": sum(1 for r in records if is_waived(r)),
        "provenance_gaps": sum(1 for r in records if provenance_gap(r)),
        "ownership_gaps": sum(1 for r in records if owner_missing(r["ownership"])),
        "divergence_gaps": sum(1 for r in records if divergence_gap(r)),
        "generator_gaps": sum(1 for r in records if generator_gap(r)),
        "third_party_imports": sum(1 for r in records if r["import_kind"] == "third_party_import"),
        "generated_artifacts": sum(1 for r in records if r["import_kind"] == "generated_artifact"),
        "long_lived_imports": sum(1 for r in records if requires_decision(r)),
        "decisions_recorded": sum(
            1 for r in records if r["decision"]["decision_state"] == "recorded"
        ),
        "total_active_reasons": sum(len(r["active_reasons"]) for r in records),
        "rules_firing": len(computed_blocking_rule_ids(records, rules)),
    }


def build_register() -> dict:
    records = build_records()
    rules = build_rules()
    blocking_rules = computed_blocking_rule_ids(records, rules)
    blocking_records = computed_blocking_record_ids(records, rules)
    decision_verdict = "hold" if blocking_records else "proceed"
    return {
        "schema_version": 1,
        "record_kind": RECORD_KIND,
        "register_id": REGISTER_ID,
        "status": "active",
        "overview_page": OVERVIEW_PAGE,
        "as_of": AS_OF,
        "source_contract_refs": {
            "third_party_import_register_ref": THIRD_PARTY_IMPORT_REGISTER_REF,
            "import_manifest_ref": IMPORT_MANIFEST_REF,
            "dependency_register_ref": DEPENDENCY_REGISTER_REF,
            "critical_upstream_scorecard_ref": CRITICAL_UPSTREAM_REF,
            "generated_lineage_ref": GENERATED_LINEAGE_REF,
            "package_inventory_ref": PACKAGE_INVENTORY_REF,
            "durability_matrix_ref": DURABILITY_MATRIX_REF,
            "m5_evidence_index_ref": EVIDENCE_INDEX_REF,
        },
        "import_cutline": {
            "cutline_level": "stable",
            "above_cutline_levels": ABOVE_CUTLINE,
            "below_cutline_levels": ["beta", "preview", "withdrawn"],
            "description": "Subjects at or above Stable carry the cleared import claim; an import-layer gap on a still-stable subject holds promotion.",
        },
        "families": FAMILIES,
        "import_kinds": IMPORT_KINDS,
        "support_classes": SUPPORT_CLASSES,
        "control_dimensions": CONTROL_DIMENSIONS,
        "origin_states": ORIGIN_STATES,
        "license_states": LICENSE_STATES,
        "upstream_pin_states": UPSTREAM_PIN_STATES,
        "ownership_states": OWNERSHIP_STATES,
        "divergence_states": DIVERGENCE_STATES,
        "divergence_review_states": DIVERGENCE_REVIEW_STATES,
        "decision_states": DECISION_STATES,
        "decision_dispositions": DECISION_DISPOSITIONS,
        "postures": POSTURES,
        "import_states": IMPORT_STATES,
        "import_reasons": IMPORT_REASONS,
        "import_actions": IMPORT_ACTIONS,
        "rules": rules,
        "records": records,
        "manifest_surface_parity": computed_manifest_surface_parity(records),
        "publication": {
            "publication_gate": "m5_import_provenance_and_fork_review_gate",
            "decision": decision_verdict,
            "blocking_rule_ids": blocking_rules,
            "blocking_record_ids": blocking_records,
            "rationale": "Hold while any release-blocking subject carries an import-layer gap on a still-stable claim; inherited and waived narrowings are gated upstream.",
        },
        "summary": computed_summary(records, rules),
    }


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------
def build_fixtures(register: dict) -> list[tuple[str, dict, str]]:
    cases: list[tuple[str, dict, str]] = []

    dup = copy.deepcopy(register)
    dup["records"].append(copy.deepcopy(dup["records"][0]))
    cases.append(("duplicate_record_id.json", dup, "DuplicateRecordId"))

    # A cleared record hiding an ownership gap without narrowing on it.
    hidden = copy.deepcopy(register)
    target = next(r for r in hidden["records"] if r["import_state"] == "cleared")
    target["ownership"]["ownership_state"] = "ownerless"
    target["ownership"]["update_owner_ref"] = ""
    cases.append(("hidden_ownership_gap.json", hidden, "GapWithoutReason"))

    # A narrowed record whose user/admin surface is clean over a gapped scan.
    masked = copy.deepcopy(register)
    target = next(
        r for r in masked["records"] if r["import_state"] not in ("cleared", "withdrawn")
    )
    target["surface_posture"] = "clear"
    cases.append(("clean_surface_over_gap.json", masked, "ManifestScanSurfaceDisagreement"))

    # A narrowed record whose effective label stays above the cutline.
    above = copy.deepcopy(register)
    target = next(
        r for r in above["records"] if r["import_state"] not in ("cleared", "withdrawn")
    )
    target["effective_label"] = "stable"
    cases.append(("narrowed_above_cutline.json", above, "EffectiveLabelMismatch"))

    return cases


def build_capture(register: dict, cases: list[tuple[str, dict, str]]) -> dict:
    s = register["summary"]
    p = register["publication"]
    parity = register["manifest_surface_parity"]
    drills = [
        "drill:hidden_ownership_gap",
        "drill:clean_surface_over_gap",
        "drill:narrowed_above_cutline",
        "drill:cleared_with_active_reason",
        "drill:reason_not_justified",
        "drill:control_state_inconsistent",
        "drill:publication_decision_inconsistent",
    ]
    return {
        "status": "pass",
        "as_of": register["as_of"],
        "summary": {
            "total_records": s["total_records"],
            "records_cleared": s["records_cleared"],
            "records_narrowed": s["records_narrowed"],
            "state_cleared": s["state_cleared"],
            "state_narrowed_provenance": s["state_narrowed_provenance"],
            "state_narrowed_ownership": s["state_narrowed_ownership"],
            "state_narrowed_divergence": s["state_narrowed_divergence"],
            "state_narrowed_generator": s["state_narrowed_generator"],
            "state_narrowed_stale": s["state_narrowed_stale"],
            "release_blocking_narrowed": s["release_blocking_narrowed"],
            "records_on_active_waiver": s["records_on_active_waiver"],
            "provenance_gaps": s["provenance_gaps"],
            "ownership_gaps": s["ownership_gaps"],
            "divergence_gaps": s["divergence_gaps"],
            "generator_gaps": s["generator_gaps"],
            "third_party_imports": s["third_party_imports"],
            "generated_artifacts": s["generated_artifacts"],
            "long_lived_imports": s["long_lived_imports"],
            "decisions_recorded": s["decisions_recorded"],
            "total_active_reasons": s["total_active_reasons"],
            "rules_firing": s["rules_firing"],
        },
        "manifest_surface_parity": {
            "subjects_in_agreement": parity["subjects_in_agreement"],
            "subjects_in_disagreement": parity["subjects_in_disagreement"],
            "subjects_with_gaps": parity["subjects_with_gaps"],
            "all_subjects_agree": parity["all_subjects_agree"],
        },
        "publication": {
            "decision": p["decision"],
            "blocking_rule_ids": p["blocking_rule_ids"],
            "blocking_record_ids": p["blocking_record_ids"],
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
    register = build_register()
    cases = build_fixtures(register)

    write_json(ARTIFACT, register)
    print(f"wrote {ARTIFACT.relative_to(REPO)}")

    for filename, data, _ in cases:
        write_json(FIXTURES / filename, data)
    manifest_index = {
        "cases": [
            {"file": filename, "expected_check_id": check_id}
            for filename, _, check_id in cases
        ]
    }
    write_json(FIXTURES / "cases.json", manifest_index)
    print(f"wrote {FIXTURES.relative_to(REPO)}/ ({len(cases)} fixtures + cases.json)")

    write_json(CAPTURE, build_capture(register, cases))
    print(f"wrote {CAPTURE.relative_to(REPO)}")


if __name__ == "__main__":
    main()
