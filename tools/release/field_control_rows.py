#!/usr/bin/env python3
"""Freeze and certify the M5 field- and control-row primitive set: the per-row
contract every mutation-capable form, wizard, and review sheet is built from —
permanent labels, required/optional clarity, source-of-value tags, exact
field-anchored validation, and restart/reconnect/trust/policy lifecycle state
surfaced on the control itself, across the provider/account-mapping,
source-registration, request-environment, package/install, and migration/import
lanes.

The canonical truth is the checked-in support export
(``artifacts/ux/m5-field-control-rows/support_export.json``). Each row binds its
label mode, requirement, source-of-value tag, validation anchor, lifecycle
implication, backing freshness, and verification proof to one inspectable record.
This tool ingests that set and, per row, **independently** re-derives an effective
claim that never reads wider than the evidence supports:

* the label is permanent, the required/optional status is marked, and the
  source-of-value tag is shown on the row;
* a blocking or warning validation is anchored directly to the field with exact
  rule text rather than deferred to a form-level summary banner;
* a restart/reconnect/trust/policy implication is surfaced on the affected control
  rather than only in a generic banner;
* a row that hides its label or source, silently overrides a policy lock, defers a
  blocking validation to a banner, buries a lifecycle implication, lets an imported
  value read as editable, or renders wider than its effective claim floors to an
  explicit blocked state that shows the reason on the row rather than a
  clean-but-false control.

The Rust truth source is ``crates/aureline-ui/src/m5_field_control_rows``; this
tool re-derives the same effective claim and narrowing reasons so the checked-in
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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-field-control-rows/support_export.json"
REPORT_REF = "artifacts/ux/m5-field-control-rows/report.md"
SCHEMA_REF = "schemas/ux/m5-field-control-rows.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-field-control-rows"

RECORD_KIND = "m5_field_control_row_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_or_restore"}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

CONSUMER_LANES = {
    "provider_account_mapping",
    "source_registration",
    "request_environment",
    "package_install",
    "migration_import",
}
SOURCE_CLASSES = {
    "default_value",
    "detected_value",
    "imported_value",
    "policy_locked",
    "user_override",
    "required_unset",
}
LIFECYCLE_IMPLICATIONS = {
    "none",
    "restart_required",
    "reconnect_required",
    "trust_required",
    "policy_blocked",
}
REQUIREMENT_CLASSES = {"required", "optional", "conditional", "system_managed"}
CONSUMER_SURFACES = {
    "form_view",
    "wizard_step",
    "review_sheet",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
}

LABS_CLAIM = "row_labs_not_claimed"
CLAIM_RANK = {
    "row_blocked": 0,
    "row_review_overlay": 1,
    "row_narrowed": 2,
    "row_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "label_not_permanent": 0,
    "source_tag_hidden": 1,
    "policy_lock_overridden": 2,
    "validation_anchor_missing": 3,
    "lifecycle_implication_hidden": 4,
    "imported_value_reads_as_editable": 5,
    "row_overclaims": 6,
    "row_backing_missing": 7,
    "requirement_unmarked": 8,
    "validation_state_unlabeled": 9,
    "async_validation_pending": 10,
    "freshness_unlabeled": 11,
    "superseded_state_not_marked": 12,
    "row_stale": 13,
    "verification_proof_stale": 14,
    "verification_proof_missing": 15,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["row_backing_missing"]}

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


def claimed_claim(row: dict) -> str:
    if row["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if row["origin"] in OVERLAY_ORIGINS:
        return "row_review_overlay"
    return "row_certified"


def intrinsic_reasons(row: dict, stale_window: bool) -> list[str]:
    overlay = row["origin"] in OVERLAY_ORIGINS
    v = row["validation"]
    reasons: list[str] = []

    # Permanent label.
    if row["label_mode"] != "permanent":
        reasons.append("label_not_permanent")

    # Source-of-value tag.
    override_not_distinct = (
        row["source_class"] == "user_override" and not row["override_distinct_from_origin"]
    )
    if (
        not row["source_tag_visible"]
        or override_not_distinct
        or any(not r["anchor_visible"] for r in row["renderings"])
    ):
        reasons.append("source_tag_hidden")

    # Policy lock.
    if row["source_class"] == "policy_locked" and not row["policy_lock_respected"]:
        reasons.append("policy_lock_overridden")

    # Validation anchoring.
    needs_anchor = v["state"] in ("invalid_blocking", "warning")
    if needs_anchor and (
        v["summary_banner_only"] or not v["anchored_to_field"] or not v["exact_rule_text_present"]
    ):
        reasons.append("validation_anchor_missing")

    # Lifecycle implication surfaced on the control.
    if row["lifecycle"]["implication"] != "none" and not row["lifecycle"]["surfaced_on_row"]:
        reasons.append("lifecycle_implication_hidden")

    # Imported/restore overlay must stay read-only.
    if overlay and (
        row["field_state"] == "editable" or any(not r["read_only"] for r in row["renderings"])
    ):
        reasons.append("imported_value_reads_as_editable")

    # Backing freshness.
    fs = row["declared_freshness_state"]
    if fs == "missing":
        reasons.append("row_backing_missing")
    elif fs == "superseded_by_newer_source" and not row["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("row_stale")

    # Required/optional clarity (recoverable).
    if not row["requirement_marked"]:
        reasons.append("requirement_unmarked")

    # Validation visibility (recoverable).
    if not v["state_labeled"]:
        reasons.append("validation_state_unlabeled")
    if v["state"] == "pending_async":
        reasons.append("async_validation_pending")

    # Freshness visibility (recoverable).
    if not row["freshness_state_visible"]:
        reasons.append("freshness_unlabeled")

    # Verification proof.
    pc = row["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "row_blocked"
    if not reasons:
        return claimed
    if claimed == "row_review_overlay":
        return "row_blocked"
    return "row_narrowed"


def row_reasons(row: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(row)
    reasons = intrinsic_reasons(row, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in row["renderings"]):
        reasons.append("row_overclaims")
    return order_reasons(reasons)


def narrow(row: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(row)
    if claimed == LABS_CLAIM:
        return {"claimed": LABS_CLAIM, "effective": LABS_CLAIM, "reasons": [], "narrowed": False}
    reasons = row_reasons(row, stale_window)
    effective = derive_effective(claimed, reasons)
    er = CLAIM_RANK.get(effective)
    cr = CLAIM_RANK.get(claimed)
    narrowed = er is not None and cr is not None and er < cr
    return {"claimed": claimed, "effective": effective, "reasons": reasons, "narrowed": narrowed}


def floored_keeps_fallback(row: dict, effective: str) -> bool:
    if effective != "row_blocked":
        return True
    return row["blocked_fallback"] in ("shows_reason_on_row", "disabled_with_hint")


def row_overclaims(row: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in row["renderings"])


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
    rows = packet.get("rows", [])
    if not rows:
        v.append("empty_rows")

    seen: set[str] = set()
    lanes: set[str] = set()
    sources: set[str] = set()
    lifecycles: set[str] = set()
    requirements: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for r in rows:
        rid = r.get("row_id", "")
        if rid in seen:
            v.append("duplicate_row_id")
        seen.add(rid)
        lanes.add(r.get("consumer_lane"))
        sources.add(r.get("source_class"))
        lifecycles.add(r.get("lifecycle", {}).get("implication"))
        requirements.add(r.get("requirement"))
        for rr in r.get("renderings", []):
            consumers.add(rr.get("surface"))

        if (
            not present(r.get("row_id"))
            or not present(r.get("label_summary"))
            or not present(r.get("consumer_surface_ref"))
        ):
            v.append("row_missing_identity")
        if r.get("origin") in OVERLAY_ORIGINS and not present(r.get("provenance_ref")):
            v.append("overlay_missing_provenance_ref")
        if not r.get("renderings"):
            v.append("row_missing_rendering")
        for rr in r.get("renderings", []):
            if not present(rr.get("source_row_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(r, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_row_missing_label_or_trigger")
        if not floored_keeps_fallback(r, decision["effective"]):
            v.append("floored_row_loses_fallback")
        if row_overclaims(r, decision["effective"]):
            v.append("rendering_row_overclaims")

    if lanes != CONSUMER_LANES:
        v.append("consumer_lane_missing")
    if sources != SOURCE_CLASSES:
        v.append("source_of_value_class_missing")
    if lifecycles != LIFECYCLE_IMPLICATIONS:
        v.append("lifecycle_implication_missing")
    if requirements != REQUIREMENT_CLASSES:
        v.append("requirement_class_missing")
    if not CONSUMER_SURFACES.issubset(consumers):
        v.append("consumer_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_row_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(rows: list[dict]) -> dict:
    dist = {"certified": 0, "narrowed": 0, "overlay": 0, "blocked": 0, "labs": 0}
    bucket = {
        "row_certified": "certified",
        "row_narrowed": "narrowed",
        "row_review_overlay": "overlay",
        "row_blocked": "blocked",
        LABS_CLAIM: "labs",
    }
    for r in rows:
        dist[bucket[narrow(r, False)["effective"]]] += 1
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


def apply_overrides(row: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(row))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_row(rows: list[dict], rid: str) -> dict:
    for r in rows:
        if r["row_id"] == rid:
            return r
    raise SystemExit(f"base row not found: {rid}")


R_PROVIDER_ENDPOINT = "row:provider-endpoint:0001"
R_SOURCE_URL = "row:source-url:0001"
R_SOURCE_KIND = "row:source-kind:0001"
R_SOURCE_TRUST = "row:source-trust-policy:0001"
R_REQUEST_HEALTH = "row:request-endpoint-health:0001"
R_PACKAGE_SCOPE = "row:package-install-scope:0001"
R_IMPORT_MAPPING = "row:import-mapping:0001"
R_LABS = "row:labs-import-preview:0001"

BLOCKED = "row_blocked"
NARROW = "row_narrowed"
CERT = "row_certified"
OVERLAY = "row_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", R_SOURCE_URL, {}, False,
     "A clean first-party source URL row with a permanent label, marked requirement, visible source tag, and an exact validation anchor certifies.",
     CERT, False, []),
    ("label-not-permanent", R_SOURCE_URL,
     {"label_mode": "placeholder_only", "renderings[*].rendered_claim": BLOCKED}, False,
     "A field whose label lives only in a placeholder floors the row to an explicit blocked state.",
     BLOCKED, True, ["label_not_permanent"]),
    ("source-tag-hidden", R_SOURCE_URL,
     {"source_tag_visible": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A row whose source-of-value tag is hidden floors.",
     BLOCKED, True, ["source_tag_hidden"]),
    ("override-not-distinct", R_SOURCE_URL,
     {"override_distinct_from_origin": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A user override that is not distinct from the value it replaced floors.",
     BLOCKED, True, ["source_tag_hidden"]),
    ("rendering-hides-anchor", R_SOURCE_URL,
     {"renderings[0].anchor_visible": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A rendering that cannot reveal the row's source/validation anchor floors.",
     BLOCKED, True, ["source_tag_hidden"]),
    ("policy-lock-overridden", R_SOURCE_TRUST,
     {"policy_lock_respected": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A silently overridden policy lock floors the row.",
     BLOCKED, True, ["policy_lock_overridden"]),
    ("validation-anchor-missing", R_SOURCE_URL,
     {"validation.state": "invalid_blocking", "validation.anchored_to_field": False,
      "validation.summary_banner_only": True, "validation.exact_rule_text_present": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A blocking validation deferred to a summary banner instead of an exact field anchor floors.",
     BLOCKED, True, ["validation_anchor_missing"]),
    ("lifecycle-implication-hidden", R_PACKAGE_SCOPE,
     {"lifecycle.surfaced_on_row": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A restart-required implication not surfaced on the control floors.",
     BLOCKED, True, ["lifecycle_implication_hidden"]),
    ("imported-reads-as-editable", R_IMPORT_MAPPING,
     {"field_state": "editable", "renderings[*].rendered_claim": BLOCKED}, False,
     "An imported value that reads as an editable local value floors below the review overlay.",
     BLOCKED, True, ["imported_value_reads_as_editable"]),
    ("row-overclaims", R_REQUEST_HEALTH,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed row whose rendering still shows certified floors as an overclaim.",
     BLOCKED, True, ["row_overclaims", "async_validation_pending"]),
    ("missing-backing", R_SOURCE_URL,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": BLOCKED}, False,
     "Missing backing data floors the row.",
     BLOCKED, True, ["row_backing_missing"]),
    ("requirement-unmarked", R_SOURCE_KIND,
     {"requirement_marked": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked required/optional status narrows the row while it stays usable.",
     NARROW, True, ["requirement_unmarked"]),
    ("validation-state-unlabeled", R_SOURCE_URL,
     {"validation.state_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding a field's validation state narrows the row.",
     NARROW, True, ["validation_state_unlabeled"]),
    ("async-validation-pending", R_SOURCE_URL,
     {"validation.state": "pending_async", "renderings[*].rendered_claim": NARROW}, False,
     "A pending async validation narrows the row.",
     NARROW, True, ["async_validation_pending"]),
    ("freshness-unlabeled", R_SOURCE_URL,
     {"freshness_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the backing freshness state narrows the row.",
     NARROW, True, ["freshness_unlabeled"]),
    ("first-party-stale", R_SOURCE_URL,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale backing source narrows rather than reading as fresh.",
     NARROW, True, ["row_stale"]),
    ("superseded-unmarked", R_SOURCE_URL,
     {"declared_freshness_state": "superseded_by_newer_source", "superseded_state_marked": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded backing source narrows the row.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-ok", R_SOURCE_URL,
     {"declared_freshness_state": "superseded_by_newer_source"}, False,
     "A marked superseded backing source stays certified because the state is visible.",
     CERT, False, []),
    ("proof-missing", R_SOURCE_URL,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the row.",
     NARROW, True, ["verification_proof_missing"]),
    ("proof-requires-review", R_SOURCE_URL,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the row.",
     NARROW, True, ["verification_proof_stale"]),
    ("stale-window", R_SOURCE_URL,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("endpoint-health-baseline", R_REQUEST_HEALTH, {}, False,
     "The canonical endpoint-health row narrows via a pending async check.",
     NARROW, True, ["async_validation_pending"]),
    ("import-overlay-baseline", R_IMPORT_MAPPING, {}, False,
     "The canonical imported mapping row stays a read-only review overlay.",
     OVERLAY, False, []),
    ("overlay-any-gap-floors", R_IMPORT_MAPPING,
     {"freshness_state_visible": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     BLOCKED, True, ["freshness_unlabeled"]),
    ("labs-not-claimed", R_LABS, {}, False,
     "A Labs import-preview row makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(rows: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        r = apply_overrides(base_row(rows, base_id), overrides)
        decision = narrow(r, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, rows: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        r = apply_overrides(base_row(rows, payload["base_row_id"]), payload["overrides"])
        decision = narrow(r, payload["stale_window"])
        exp = payload["expected"]
        if decision["effective"] != exp["effective_claim"]:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp['effective_claim']}")
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
            "base_row_id": base_id,
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
        "corpus_id": "m5-field-control-rows-corpus:0001",
        "description": (
            "Perturbation corpus for the field/control-row narrowing engine. Each case "
            "starts from a canonical row, applies dotted-path overrides, and asserts the "
            "re-derived effective claim, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("field-control-row set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["rows"])
    sys.stdout.write(
        f"field-control-row set OK: {len(packet['rows'])} rows, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    rows = packet["rows"]
    failures = run_corpus_from_cases(rows)
    failures += run_corpus_from_disk(repo_root, rows)
    if failures:
        sys.stderr.write("field-control-row corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"field-control-row corpus OK: {len(CASES)} cases\n")
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
