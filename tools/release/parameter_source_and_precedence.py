#!/usr/bin/env python3
"""Freeze and certify the M5 parameter-source and precedence inspector set: the
inspector an M5 mutation-capable form opens to answer *why a current value is present
and which source actually wins* before a change is committed.

One inspector model is reused across the provider account-mapping, source-
registration, request-environment, package-install, settings-config, import-
migration, and project-bootstrap forms. The canonical truth is the checked-in support
export (``artifacts/ux/m5-parameter-source-and-precedence/support_export.json``). Each
field binds its per-layer source candidates (default, detected, imported,
environment_resolved, user_override, policy_provided), each with a personal/local vs
workspace/shared vs policy-owned value scope, to an effective resolution, a policy
lock, and a fallback disclosure. This tool ingests that set and, per field,
**independently** re-derives an effective claim that never reads wider than the
evidence supports:

* the effective source layer is surfaced and its winning candidate is present and
  labelled;
* the distinct source layers stay visually distinct rather than collapsing into one
  current field state;
* the declared effective layer is the highest-precedence present candidate, and its
  declared rank matches the canonical rank;
* a policy lock is surfaced and pins the effective value to the policy-provided value,
  forbidding a silent override;
* a fallback to a built-in/auto source discloses why, the value scope is surfaced, and
  a mutation-capable field never allows a submit from an ambiguous source-hidden state;
* an imported/migration review never reads as a user-set value, the inspect-to-source
  path is kept, and no rendering surface overclaims;
* a field that breaks any of these floors to parameter_unsafe and falls back to an
  explicit blocked-submit state with an inspect/keyboard recovery path, while a
  labelled recoverable gap holds a first-party field at parameter_narrowed.

The Rust truth source is ``crates/aureline-ui/src/m5_parameter_source_and_precedence``;
this tool re-derives the same effective claim and narrowing reasons so the checked-in
artifacts can never imply a wider claim than the current evidence backs.

Subcommands::

    validate     Re-derive from the support export and fail on any overclaim
    corpus       Run the narrowing engine over the checked-in fixture corpus
    emit-corpus  Regenerate the fixture corpus from the embedded case list
    self-test    End-to-end: validate plus the corpus pass
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SUPPORT_EXPORT_REF = "artifacts/ux/m5-parameter-source-and-precedence/support_export.json"
REPORT_REF = "artifacts/ux/m5-parameter-source-and-precedence/report.md"
SCHEMA_REF = "schemas/ux/m5-parameter-source-and-precedence.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-parameter-source-and-precedence"

RECORD_KIND = "m5_parameter_source_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_review"}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

FIELD_FORMS = {
    "provider_account_mapping",
    "source_registration",
    "request_environment",
    "package_install_config",
    "settings_config_editor",
    "import_migration_mapping",
    "project_bootstrap",
}
FIELD_LANES = {"provider", "admin", "request", "package", "settings", "import", "projects"}
SOURCE_LAYERS = {
    "default",
    "detected",
    "imported",
    "environment_resolved",
    "user_override",
    "policy_provided",
}
PRECEDENCE_RANK = {
    "default": 0,
    "detected": 1,
    "imported": 2,
    "environment_resolved": 3,
    "user_override": 4,
    "policy_provided": 5,
}
VALUE_SCOPES = {"personal_local", "workspace_shared", "policy_owned"}
CONSUMER_SURFACES = {
    "inspector_panel",
    "field_popover",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
    "cli_inspect",
}

LABS_CLAIM = "parameter_labs_not_claimed"
CLAIM_RANK = {
    "parameter_unsafe": 0,
    "parameter_review_overlay": 1,
    "parameter_narrowed": 2,
    "parameter_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "effective_source_hidden": 0,
    "sources_collapsed": 1,
    "precedence_inconsistent": 2,
    "policy_lock_hidden": 3,
    "policy_lock_not_enforced": 4,
    "imported_value_reads_as_user_set": 5,
    "fallback_reason_hidden": 6,
    "value_scope_hidden": 7,
    "ambiguous_submit_allowed": 8,
    "inspect_path_lost": 9,
    "inspector_overclaims": 10,
    "provenance_backing_missing": 11,
    "source_labels_unlabeled": 12,
    "scope_labels_unlabeled": 13,
    "fallback_reason_unlabeled": 14,
    "precedence_explanation_unlabeled": 15,
    "detection_state_unlabeled": 16,
    "detection_superseded_unmarked": 17,
    "detection_stale": 18,
    "verification_proof_stale": 19,
    "verification_proof_missing": 20,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["provenance_backing_missing"]
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


def present(value) -> bool:
    return isinstance(value, str) and value.strip() != ""


def order_reasons(reasons: list[str]) -> list[str]:
    seen: list[str] = []
    for reason in sorted(reasons, key=lambda r: REASON_ORDER.get(r, 99)):
        if reason not in seen:
            seen.append(reason)
    return seen


def overclaims(effective: str, rendered: str) -> bool:
    er = CLAIM_RANK.get(effective)
    rr = CLAIM_RANK.get(rendered)
    if er is not None and rr is not None:
        return rr > er
    return effective != rendered


def highest_present_layer(rec: dict):
    present_layers = [
        c["source_layer"] for c in rec["inspector"]["candidates"] if c["present"]
    ]
    if not present_layers:
        return None
    return max(present_layers, key=lambda l: PRECEDENCE_RANK.get(l, -1))


def effective_candidate(rec: dict):
    eff = rec["inspector"]["effective"]["effective_source_layer"]
    for c in rec["inspector"]["candidates"]:
        if c["source_layer"] == eff:
            return c
    return None


def claimed_claim(rec: dict) -> str:
    if rec["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if rec["origin"] in OVERLAY_ORIGINS:
        return "parameter_review_overlay"
    return "parameter_certified"


def intrinsic_reasons(rec: dict, stale_window: bool) -> list[str]:
    insp = rec["inspector"]
    eff = insp["effective"]
    integ = rec["integrity"]
    overlay = rec["origin"] in OVERLAY_ORIGINS
    eff_layer = eff["effective_source_layer"]
    eff_cand = effective_candidate(rec)
    policy = insp["policy_lock"]
    fallback = insp["fallback"]
    reasons: list[str] = []

    # Effective source visible + winning candidate present and labelled.
    if (
        not eff["effective_source_visible"]
        or not integ["effective_source_visible"]
        or eff_cand is None
        or not eff_cand["present"]
        or not eff_cand["source_labeled"]
    ):
        reasons.append("effective_source_hidden")

    # Distinct source layers.
    if not insp["sources_distinct"] or not integ["sources_visually_distinct"]:
        reasons.append("sources_collapsed")

    # Precedence.
    if highest_present_layer(rec) != eff_layer or eff["precedence_rank_declared"] != PRECEDENCE_RANK.get(
        eff_layer
    ):
        reasons.append("precedence_inconsistent")

    # Policy lock.
    if policy["policy_locked"] and (not policy["lock_surfaced"] or not integ["policy_lock_visible"]):
        reasons.append("policy_lock_hidden")
    if policy["policy_locked"] and (
        policy["override_allowed_despite_lock"] or eff_layer != "policy_provided"
    ):
        reasons.append("policy_lock_not_enforced")

    # Imported overlay read-only.
    if overlay and not integ["imported_review_read_only"]:
        reasons.append("imported_value_reads_as_user_set")

    # Fallback reason.
    if fallback["is_fallback"] and (
        not fallback["fallback_reason_disclosed"] or not integ["fallback_reason_visible"]
    ):
        reasons.append("fallback_reason_hidden")

    # Value scope.
    if (
        not eff["effective_scope_visible"]
        or not integ["value_scope_visible"]
        or eff_cand is None
        or not eff_cand["scope_labeled"]
    ):
        reasons.append("value_scope_hidden")

    # Ambiguous submit (the guardrail). An overlay is read-only, so the gate does not
    # apply.
    if not overlay and not integ["submit_gated_on_source_clarity"]:
        reasons.append("ambiguous_submit_allowed")

    # Inspect-to-source path.
    if any(not r["source_visible"] for r in rec["renderings"]) or (
        rec["declared_reopen_target"] == "none_keyboard_fallback"
    ):
        reasons.append("inspect_path_lost")

    # Provenance backing freshness.
    ds = rec["declared_detection_state"]
    if ds == "missing":
        reasons.append("provenance_backing_missing")
    elif ds == "superseded_by_newer_source" and not integ["superseded_state_marked"]:
        reasons.append("detection_superseded_unmarked")
    elif ds == "stale_expired" and not overlay:
        reasons.append("detection_stale")
    if not integ["detection_state_visible"]:
        reasons.append("detection_state_unlabeled")

    # Non-winning candidate labelling (non-floor).
    if any(
        c["present"] and c["source_layer"] != eff_layer and not c["source_labeled"]
        for c in insp["candidates"]
    ):
        reasons.append("source_labels_unlabeled")
    if any(
        c["present"] and c["source_layer"] != eff_layer and not c["scope_labeled"]
        for c in insp["candidates"]
    ):
        reasons.append("scope_labels_unlabeled")

    # Fallback reason labelling (non-floor).
    if fallback["is_fallback"] and not fallback["fallback_reason_labeled"]:
        reasons.append("fallback_reason_unlabeled")

    # Precedence explanation (non-floor).
    if not insp["precedence_explained"] or not integ["precedence_visible"]:
        reasons.append("precedence_explanation_unlabeled")

    # Verification proof.
    pc = rec["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "parameter_unsafe"
    if not reasons:
        return claimed
    if claimed == "parameter_review_overlay":
        return "parameter_unsafe"
    return "parameter_narrowed"


def record_reasons(rec: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(rec)
    reasons = intrinsic_reasons(rec, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in rec["renderings"]):
        reasons.append("inspector_overclaims")
    return order_reasons(reasons)


def narrow(rec: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(rec)
    if claimed == LABS_CLAIM:
        return {"claimed": LABS_CLAIM, "effective": LABS_CLAIM, "reasons": [], "narrowed": False}
    reasons = record_reasons(rec, stale_window)
    effective = derive_effective(claimed, reasons)
    er = CLAIM_RANK.get(effective)
    cr = CLAIM_RANK.get(claimed)
    narrowed = er is not None and cr is not None and er < cr
    return {"claimed": claimed, "effective": effective, "reasons": reasons, "narrowed": narrowed}


def floored_keeps_fallback(rec: dict, effective: str) -> bool:
    if effective != "parameter_unsafe":
        return True
    return rec["declared_reopen_target"] in ("inspector_only", "none_keyboard_fallback") or present(
        rec["lineage"]["reopen_backlink_ref"]
    )


def record_overclaims(rec: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in rec["renderings"])


def contains_forbidden(value) -> bool:
    if isinstance(value, str):
        low = value.lower()
        return any(sub in low for sub in FORBIDDEN_SUBSTRINGS)
    if isinstance(value, list):
        return any(contains_forbidden(v) for v in value)
    if isinstance(value, dict):
        return any(contains_forbidden(v) for v in value.values())
    return False


def load_support_export(repo_root: Path) -> dict:
    return json.loads((repo_root / SUPPORT_EXPORT_REF).read_text(encoding="utf-8"))


def validate_packet(packet: dict) -> list[str]:
    v: list[str] = []
    if packet.get("record_kind") != RECORD_KIND:
        v.append("wrong_record_kind")
    if packet.get("schema_version") != SCHEMA_VERSION:
        v.append("wrong_schema_version")
    if packet.get("taxonomy_version") != TAXONOMY_VERSION:
        v.append("wrong_taxonomy_version")
    for key in ("packet_id", "label", "as_of", "redaction_class_token"):
        if not present(packet.get(key)):
            v.append("missing_identity")
            break
    if packet.get("redaction_class_token") not in REDACTION_TOKENS:
        v.append("invalid_redaction_class")
    vf = packet.get("verification_freshness", {})
    if vf.get("verification_freshness_slo_hours", 0) < 1 or not present(
        vf.get("last_verification_refresh")
    ):
        v.append("evidence_freshness_incomplete")
    fields = packet.get("fields", [])
    if not fields:
        v.append("empty_fields")

    seen: set[str] = set()
    forms: set[str] = set()
    lanes: set[str] = set()
    source_layers: set[str] = set()
    value_scopes: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for f in fields:
        fid = f.get("field_id", "")
        if fid in seen:
            v.append("duplicate_field_id")
        seen.add(fid)
        forms.add(f.get("form"))
        lanes.add(f.get("lane"))
        for c in f.get("inspector", {}).get("candidates", []):
            source_layers.add(c.get("source_layer"))
            value_scopes.add(c.get("value_scope"))
        for r in f.get("renderings", []):
            consumers.add(r.get("surface"))

        if (
            not present(f.get("field_id"))
            or not present(f.get("label_summary"))
            or not present(f.get("lineage", {}).get("session_ref"))
        ):
            v.append("field_missing_identity")
        if f.get("origin") in OVERLAY_ORIGINS and not (
            present(f.get("lineage", {}).get("provider_ref"))
            or present(f.get("lineage", {}).get("source_artifact_ref"))
        ):
            v.append("overlay_missing_provenance_ref")
        if not f.get("inspector", {}).get("candidates"):
            v.append("field_missing_candidates")
        if not f.get("renderings"):
            v.append("field_missing_rendering")
        for r in f.get("renderings", []):
            if not present(r.get("source_field_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(f, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_field_missing_label_or_trigger")
        if not floored_keeps_fallback(f, decision["effective"]):
            v.append("floored_field_loses_fallback")
        if record_overclaims(f, decision["effective"]):
            v.append("rendering_field_overclaims")

    if not FIELD_FORMS.issubset(forms):
        v.append("form_missing")
    if not FIELD_LANES.issubset(lanes):
        v.append("field_lane_missing")
    if not SOURCE_LAYERS.issubset(source_layers):
        v.append("source_layer_missing")
    if not VALUE_SCOPES.issubset(value_scopes):
        v.append("value_scope_missing")
    if not CONSUMER_SURFACES.issubset(consumers):
        v.append("consumer_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_field_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(fields: list[dict]) -> dict:
    dist = {"certified": 0, "narrowed": 0, "overlay": 0, "unsafe": 0, "labs": 0}
    bucket = {
        "parameter_certified": "certified",
        "parameter_narrowed": "narrowed",
        "parameter_review_overlay": "overlay",
        "parameter_unsafe": "unsafe",
        LABS_CLAIM: "labs",
    }
    for f in fields:
        dist[bucket[narrow(f, False)["effective"]]] += 1
    return dist


# --------------------------------------------------------------------------- #
# Override engine + perturbation corpus.
# --------------------------------------------------------------------------- #

_TOKEN = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)(?:\[(\*|\d+)\])?$")


def _set_path(node, parts: list[str], value) -> None:
    head, *rest = parts
    m = _TOKEN.match(head)
    if not m:
        raise SystemExit(f"bad override token: {head}")
    key, idx = m.group(1), m.group(2)
    if idx is None:
        if rest:
            _set_path(node[key], rest, value)
        else:
            node[key] = value
    elif idx == "*":
        if not rest:
            raise SystemExit(f"cannot assign scalar to a list via [*]: {head}")
        for elem in node[key]:
            _set_path(elem, rest, value)
    else:
        i = int(idx)
        if rest:
            _set_path(node[key][i], rest, value)
        else:
            node[key][i] = value


def apply_overrides(rec: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(rec))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_record(fields: list[dict], fid: str) -> dict:
    for f in fields:
        if f["field_id"] == fid:
            return f
    raise SystemExit(f"base field not found: {fid}")


F_PROVIDER = "field:provider-account-mapping:0001"
F_ADMIN = "field:source-registration:0001"
F_REQUEST = "field:request-environment:0001"
F_PACKAGE = "field:package-install-config:0001"
F_SETTINGS = "field:settings-config-editor:0001"
F_IMPORT = "field:import-migration-mapping:0001"
F_LABS = "field:project-bootstrap:0001"

UNSAFE = "parameter_unsafe"
NARROW = "parameter_narrowed"
CERT = "parameter_certified"
OVERLAY = "parameter_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", F_SETTINGS, {}, False,
     "A user override that wins over imported, detected, and default values, all distinct, certifies.",
     CERT, False, []),
    ("provider-detected-certified", F_PROVIDER, {}, False,
     "A detected provider account that wins over the built-in default certifies.",
     CERT, False, []),
    ("admin-policy-locked-certified", F_ADMIN, {}, False,
     "A policy-provided, policy-locked value with the losing user override kept visible certifies.",
     CERT, False, []),
    ("package-fallback-certified", F_PACKAGE, {}, False,
     "A fall back to the built-in default with the reason disclosed certifies.",
     CERT, False, []),
    ("request-narrowed-baseline", F_REQUEST, {}, False,
     "An environment-resolved value whose verification proof requires review narrows.",
     NARROW, True, ["verification_proof_stale"]),
    ("import-overlay-baseline", F_IMPORT, {}, False,
     "An imported migration value stays a read-only review overlay, never a user-set value.",
     OVERLAY, False, []),
    ("labs-not-claimed", F_LABS, {}, False,
     "A Labs project-template field makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),

    # Floors.
    ("effective-source-hidden", F_SETTINGS,
     {"inspector.effective.effective_source_visible": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A field that hides the effective value's source layer floors.",
     UNSAFE, True, ["effective_source_hidden"]),
    ("sources-collapsed", F_SETTINGS,
     {"inspector.sources_distinct": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "Collapsing the distinct source layers into one current field state floors.",
     UNSAFE, True, ["sources_collapsed"]),
    ("precedence-inconsistent", F_SETTINGS,
     {"inspector.effective.effective_source_layer": "detected",
      "inspector.effective.precedence_rank_declared": 1,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "Declaring a lower layer effective while a higher-precedence candidate is present floors.",
     UNSAFE, True, ["precedence_inconsistent"]),
    ("declared-rank-mismatch", F_SETTINGS,
     {"inspector.effective.precedence_rank_declared": 0, "renderings[*].rendered_claim": UNSAFE}, False,
     "A declared precedence rank that disagrees with the effective layer floors.",
     UNSAFE, True, ["precedence_inconsistent"]),
    ("policy-lock-hidden", F_ADMIN,
     {"inspector.policy_lock.lock_surfaced": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A policy-locked field that does not surface the lock floors.",
     UNSAFE, True, ["policy_lock_hidden"]),
    ("policy-lock-not-enforced", F_ADMIN,
     {"inspector.policy_lock.override_allowed_despite_lock": True, "renderings[*].rendered_claim": UNSAFE}, False,
     "A policy-locked field that still allows a silent user override floors.",
     UNSAFE, True, ["policy_lock_not_enforced"]),
    ("imported-reads-as-user-set", F_IMPORT,
     {"integrity.imported_review_read_only": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "An imported/migration review that reads as a user-set value floors below the review overlay.",
     UNSAFE, True, ["imported_value_reads_as_user_set"]),
    ("fallback-reason-hidden", F_PACKAGE,
     {"inspector.fallback.fallback_reason_disclosed": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A fall back to a default whose reason is not disclosed floors.",
     UNSAFE, True, ["fallback_reason_hidden"]),
    ("value-scope-hidden", F_SETTINGS,
     {"inspector.effective.effective_scope_visible": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A field that hides the effective value's scope floors.",
     UNSAFE, True, ["value_scope_hidden"]),
    ("ambiguous-submit-allowed", F_SETTINGS,
     {"integrity.submit_gated_on_source_clarity": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A mutation-capable field that allows submit from an ambiguous source-hidden state floors.",
     UNSAFE, True, ["ambiguous_submit_allowed"]),
    ("overlay-submit-gate-na-ok", F_IMPORT,
     {"integrity.submit_gated_on_source_clarity": False}, False,
     "A read-only overlay never applies the submit gate, so an unset gate stays a review overlay.",
     OVERLAY, False, []),
    ("inspect-path-lost-keeps-fallback", F_SETTINGS,
     {"declared_reopen_target": "none_keyboard_fallback", "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing the inspect-to-source path floors but keeps a keyboard fallback.",
     UNSAFE, True, ["inspect_path_lost"]),
    ("provenance-backing-missing", F_SETTINGS,
     {"declared_detection_state": "missing", "renderings[*].rendered_claim": UNSAFE}, False,
     "A missing source-provenance snapshot floors the field.",
     UNSAFE, True, ["provenance_backing_missing"]),
    ("overlay-any-gap-floors", F_IMPORT,
     {"inspector.precedence_explained": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     UNSAFE, True, ["precedence_explanation_unlabeled"]),
    ("inspector-overclaims", F_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed field whose rendering still shows certified floors as an overclaim.",
     UNSAFE, True, ["inspector_overclaims", "verification_proof_stale"]),

    # Narrows.
    ("source-labels-unlabeled", F_SETTINGS,
     {"inspector.candidates[1].source_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding a non-winning candidate's source label narrows the field.",
     NARROW, True, ["source_labels_unlabeled"]),
    ("absent-candidate-unlabeled-ok", F_PACKAGE,
     {"inspector.candidates[1].source_labeled": False}, False,
     "An absent candidate's missing source label is exempt, so the field stays certified.",
     CERT, False, []),
    ("scope-labels-unlabeled", F_SETTINGS,
     {"inspector.candidates[2].scope_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding a non-winning candidate's scope label narrows the field.",
     NARROW, True, ["scope_labels_unlabeled"]),
    ("fallback-reason-unlabeled", F_PACKAGE,
     {"inspector.fallback.fallback_reason_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "A generic/unlabelled fallback reason narrows the field.",
     NARROW, True, ["fallback_reason_unlabeled"]),
    ("precedence-explanation-unlabeled", F_SETTINGS,
     {"inspector.precedence_explained": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the precedence explanation narrows the field.",
     NARROW, True, ["precedence_explanation_unlabeled"]),
    ("detection-state-unlabeled", F_SETTINGS,
     {"integrity.detection_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the detection freshness state narrows the field.",
     NARROW, True, ["detection_state_unlabeled"]),
    ("detection-superseded-unmarked", F_SETTINGS,
     {"declared_detection_state": "superseded_by_newer_source",
      "integrity.superseded_state_marked": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded detection snapshot narrows the field.",
     NARROW, True, ["detection_superseded_unmarked"]),
    ("detection-superseded-marked-ok", F_SETTINGS,
     {"declared_detection_state": "superseded_by_newer_source"}, False,
     "A marked superseded detection snapshot stays certified because the state is visible.",
     CERT, False, []),
    ("detection-stale", F_SETTINGS,
     {"declared_detection_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale detection snapshot narrows rather than reading as fresh.",
     NARROW, True, ["detection_stale"]),
    ("proof-missing", F_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the field.",
     NARROW, True, ["verification_proof_missing"]),
    ("proof-requires-review", F_PROVIDER,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the field.",
     NARROW, True, ["verification_proof_stale"]),
    ("stale-window", F_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
]


def run_corpus_from_cases(fields: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        r = apply_overrides(base_record(fields, base_id), overrides)
        decision = narrow(r, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, fields: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        r = apply_overrides(base_record(fields, payload["base_field_id"]), payload["overrides"])
        decision = narrow(r, payload["stale_window"])
        exp = payload["expected"]
        if decision["effective"] != exp["effective_claim"]:
            failures.append(
                f"{case_id}: effective {decision['effective']} != {exp['effective_claim']}"
            )
        if decision["narrowed"] != exp["narrowed"]:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp['narrowed']}")
        if decision["reasons"] != exp["active_narrowing_reasons"]:
            failures.append(
                f"{case_id}: reasons {decision['reasons']} != {exp['active_narrowing_reasons']}"
            )
    return failures


def write_corpus(repo_root: Path) -> None:
    out_dir = repo_root / FIXTURE_DIR
    out_dir.mkdir(parents=True, exist_ok=True)
    case_files = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, desc, exp_eff, exp_narrowed, exp_reasons = case
        payload = {
            "case_id": case_id,
            "kind": "narrowing",
            "description": desc,
            "base_field_id": base_id,
            "stale_window": stale_window,
            "overrides": overrides,
            "expected": {
                "effective_claim": exp_eff,
                "narrowed": exp_narrowed,
                "active_narrowing_reasons": exp_reasons,
            },
        }
        filename = f"{case_id}.json"
        (out_dir / filename).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        case_files.append(filename)
    index = {
        "corpus_id": "m5-parameter-source-and-precedence-corpus:0001",
        "description": (
            "Perturbation corpus for the parameter-source and precedence inspector narrowing "
            "engine. Each case starts from a canonical field, applies dotted-path overrides, and "
            "asserts the re-derived effective claim, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("parameter-source set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["fields"])
    sys.stdout.write(
        f"parameter-source set OK: {len(packet['fields'])} fields, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    fields = packet["fields"]
    failures = run_corpus_from_cases(fields)
    failures += run_corpus_from_disk(repo_root, fields)
    if failures:
        sys.stderr.write("parameter-source corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"parameter-source corpus OK: {len(CASES)} cases\n")
    return 0


def cmd_emit_corpus(repo_root: Path) -> int:
    write_corpus(repo_root)
    sys.stdout.write(f"wrote {len(CASES)} cases + index to {FIXTURE_DIR}\n")
    return 0


def cmd_self_test(repo_root: Path) -> int:
    rc = cmd_validate(repo_root)
    rc |= cmd_corpus(repo_root)
    return rc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["validate", "corpus", "emit-corpus", "self-test"])
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()
    return {
        "validate": cmd_validate,
        "corpus": cmd_corpus,
        "emit-corpus": cmd_emit_corpus,
        "self-test": cmd_self_test,
    }[args.command](repo_root)


if __name__ == "__main__":
    raise SystemExit(main())
