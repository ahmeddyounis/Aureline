#!/usr/bin/env python3
"""Freeze and certify the M5 mutation-capable form family certification set: the promotion
lane that certifies each claimed M5 form family (provider, admin, request, package,
settings, import, and project lanes) against the shared structured-input component lanes and
auto-narrows the family's qualification claim when its proof is stale, partial, missing, or
failing.

The canonical truth is the checked-in support export
(``artifacts/ux/m5-form-family-certification/support_export.json``). Each family binds one
evidence cell per required ``(dimension, lane)`` proof pair — field/form validation,
parameter provenance, draft-versus-applied truth, interruption recovery, and
staged-review-before-commit — each pointing at the upstream lane's support export. This tool
ingests that set and, per family, **independently** re-derives an effective qualification
tier that never reads wider than the evidence supports:

* a stale or partial proof caps the family at beta;
* a missing required proof caps it at preview;
* a failing proof, or a consumer surface that renders a wider tier than the evidence
  supports, withdraws it;
* an elapsed certification-freshness window ages every certified family to beta; and
* a narrowed family must keep an actionable rerun path and a non-generic narrow label.

The Rust truth source is ``crates/aureline-ui/src/m5_form_family_certification``; this tool
re-derives the same effective tier and narrowing reasons so the checked-in artifacts can
never imply a wider claim than the current evidence backs.

Subcommands::

    validate     Re-derive from the support export and fail on any overclaim
    corpus       Run the narrowing engine over the checked-in fixture corpus
    emit-corpus  Regenerate the fixture corpus from the embedded case list
    self-test    End-to-end: validate plus the corpus pass
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SUPPORT_EXPORT_REF = "artifacts/ux/m5-form-family-certification/support_export.json"
REPORT_REF = "artifacts/ux/m5-form-family-certification/report.md"
SCHEMA_REF = "schemas/ux/m5-form-family-certification.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-form-family-certification"

RECORD_KIND = "m5_form_family_certification_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

FORM_FAMILIES = {
    "provider_connect",
    "admin_source_management",
    "request_workspace",
    "package_install_review",
    "settings_config_editor",
    "import_migration_center",
    "generated_project_bootstrap",
}
PROOF_DIMENSIONS = [
    "field_form_validation",
    "parameter_provenance",
    "draft_versus_applied",
    "interruption_recovery",
    "staged_review_before_commit",
]
PROOF_LANES = {
    "field_control_rows",
    "form_validation_and_blocked_submit",
    "parameter_source_and_precedence",
    "draft_state_and_autosave",
    "staged_review_sheets",
    "structured_input_and_staged_review",
    "accessibility_and_continuity",
}
CONSUMER_SURFACES = {
    "about",
    "help_inline",
    "service_health",
    "compatibility",
    "release_packet",
    "support_export",
    "docs_public_truth",
}

# The required (dimension, lane) proof pairs, in evidence-array order.
REQUIRED_PROOF_PAIRS = [
    ("field_form_validation", "field_control_rows"),
    ("field_form_validation", "form_validation_and_blocked_submit"),
    ("parameter_provenance", "parameter_source_and_precedence"),
    ("draft_versus_applied", "draft_state_and_autosave"),
    ("interruption_recovery", "accessibility_and_continuity"),
    ("staged_review_before_commit", "staged_review_sheets"),
    ("staged_review_before_commit", "structured_input_and_staged_review"),
]

TIER_RANK = {"stable": 0, "beta": 1, "preview": 2, "withdrawn": 3}
RANK_TIER = {rank: tier for tier, rank in TIER_RANK.items()}

STATE_FLOOR = {
    "current": None,
    "not_applicable": None,
    "stale": "beta",
    "partial": "beta",
    "missing": "preview",
    "failing": "withdrawn",
}
STALE_OR_MISSING_STATES = {"stale", "partial", "missing", "failing"}
HAS_CAPTURE_STATES = {"current", "stale", "partial", "failing"}

DIMENSION_NARROW_REASON = {
    "field_form_validation": "field_form_validation_uncertified",
    "parameter_provenance": "parameter_provenance_uncertified",
    "draft_versus_applied": "draft_recovery_uncertified",
    "interruption_recovery": "interruption_recovery_uncertified",
    "staged_review_before_commit": "staged_review_uncertified",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "field_form_validation_uncertified": 0,
    "parameter_provenance_uncertified": 1,
    "draft_recovery_uncertified": 2,
    "interruption_recovery_uncertified": 3,
    "staged_review_uncertified": 4,
    "required_proof_missing": 5,
    "verdict_overclaim": 6,
    "surface_reuse_incomplete": 7,
    "certification_proof_stale": 8,
}

GENERIC_LABELS = {
    "",
    "unavailable",
    "not available",
    "n/a",
    "error",
    "failed",
    "downgraded",
    "unverified",
    "narrowed",
    "stale",
    "blocked",
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


def present(value) -> bool:
    return isinstance(value, str) and value.strip() != ""


def label_is_generic(label) -> bool:
    if not isinstance(label, str):
        return True
    return label.strip().lower() in GENERIC_LABELS


def order_reasons(reasons: list[str]) -> list[str]:
    seen: list[str] = []
    for reason in sorted(reasons, key=lambda r: REASON_ORDER.get(r, 99)):
        if reason not in seen:
            seen.append(reason)
    return seen


def cell_for(family: dict, dimension: str, lane: str) -> dict | None:
    for cell in family.get("evidence", []):
        if cell.get("dimension") == dimension and cell.get("source_lane") == lane:
            return cell
    return None


def surface_reuse_complete(family: dict) -> bool:
    rendered = {r.get("surface") for r in family.get("renderings", [])}
    return CONSUMER_SURFACES.issubset(rendered)


def overclaims(effective: str, rendered: str) -> bool:
    return TIER_RANK.get(rendered, 0) < TIER_RANK.get(effective, 0)


def parse_date(value) -> dt.datetime | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def freshness_overclaims(cell: dict, as_of: str, slo_hours: int) -> bool:
    if cell.get("state") != "current":
        return False
    captured = parse_date(cell.get("captured_at"))
    now = parse_date(as_of)
    if captured is None or now is None:
        return False
    return (now - captured) > dt.timedelta(hours=slo_hours)


def narrow(family: dict, stale_window: bool) -> dict:
    claimed = family.get("claimed_tier", "stable")
    rank = TIER_RANK.get(claimed, 0)
    reasons: list[str] = []
    stale_or_missing: list[str] = []

    for dimension, lane in REQUIRED_PROOF_PAIRS:
        cell = cell_for(family, dimension, lane)
        if cell is None:
            rank = max(rank, TIER_RANK["preview"])
            reasons.append("required_proof_missing")
            if dimension not in stale_or_missing:
                stale_or_missing.append(dimension)
            continue
        floor = STATE_FLOOR.get(cell.get("state"))
        if floor is not None:
            rank = max(rank, TIER_RANK[floor])
            reasons.append(DIMENSION_NARROW_REASON[dimension])
        if cell.get("state") in STALE_OR_MISSING_STATES and dimension not in stale_or_missing:
            stale_or_missing.append(dimension)

    if stale_window:
        rank = max(rank, TIER_RANK["beta"])
        reasons.append("certification_proof_stale")
    if not surface_reuse_complete(family):
        rank = max(rank, TIER_RANK["beta"])
        reasons.append("surface_reuse_incomplete")

    intrinsic = RANK_TIER[rank]
    if any(overclaims(intrinsic, r.get("rendered_tier")) for r in family.get("renderings", [])):
        reasons.append("verdict_overclaim")
        rank = max(rank, TIER_RANK["withdrawn"])

    effective = RANK_TIER[rank]
    narrowed = rank > TIER_RANK.get(claimed, 0)
    if effective == "withdrawn":
        verdict = "withdrawn"
    elif narrowed:
        verdict = "narrowed"
    else:
        verdict = "certified"

    # Stale/missing dimensions, sorted by declared dimension order.
    som = sorted(stale_or_missing, key=PROOF_DIMENSIONS.index)
    return {
        "claimed": claimed,
        "effective": effective,
        "verdict": verdict,
        "certified": verdict == "certified",
        "narrowed": narrowed,
        "reasons": order_reasons(reasons),
        "stale_or_missing_dimensions": som,
    }


def narrowed_label(family: dict, decision: dict) -> str | None:
    if not decision["narrowed"]:
        return None
    if not decision["reasons"]:
        return None
    return f"{family['family']} narrowed to {decision['effective']}: {decision['reasons'][0]}"


def floored_keeps_fallback(family: dict, effective: str) -> bool:
    if effective == "stable":
        return True
    return present(family.get("lineage", {}).get("rerun_ref"))


def family_overclaims(family: dict, effective: str) -> bool:
    return any(overclaims(effective, r.get("rendered_tier")) for r in family.get("renderings", []))


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
    cf = packet.get("certification_freshness", {})
    slo = cf.get("certification_freshness_slo_hours", 0)
    if slo < 1 or not present(cf.get("last_certification_refresh")):
        v.append("evidence_freshness_incomplete")
    families = packet.get("families", [])
    if not families:
        v.append("empty_families")

    as_of = packet.get("as_of", "")
    stale_window = False  # canonical packet is evaluated at its own as_of

    seen: set[str] = set()
    fams: set[str] = set()
    dims: set[str] = set()
    lanes: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False

    for f in families:
        fid = f.get("family_id", "")
        if fid in seen:
            v.append("duplicate_family_id")
        seen.add(fid)
        fams.add(f.get("family"))
        for cell in f.get("evidence", []):
            dims.add(cell.get("dimension"))
            lanes.add(cell.get("source_lane"))
        for r in f.get("renderings", []):
            consumers.add(r.get("surface"))

        if (
            not present(f.get("family_id"))
            or not present(f.get("label_summary"))
            or not present(f.get("lineage", {}).get("evidence_run_ref"))
        ):
            v.append("family_missing_identity")
        if not f.get("renderings"):
            v.append("family_missing_rendering")
        for r in f.get("renderings", []):
            if not present(r.get("source_family_ref")):
                v.append("rendering_missing_source_ref")

        for dimension, lane in REQUIRED_PROOF_PAIRS:
            cell = cell_for(f, dimension, lane)
            if cell is None:
                v.append("required_proof_pair_missing")
                continue
            requires_ref = cell.get("state") in HAS_CAPTURE_STATES
            if requires_ref != present(cell.get("proof_ref")) or requires_ref != present(
                cell.get("captured_at")
            ):
                v.append("evidence_ref_incoherent")
            if label_is_generic(cell.get("proof_label")):
                v.append("evidence_label_generic")
            if freshness_overclaims(cell, as_of, slo):
                v.append("evidence_freshness_overclaim")

        decision = narrow(f, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            label = narrowed_label(f, decision)
            if not decision["reasons"] or label is None or label_is_generic(label):
                v.append("narrowed_family_missing_label_or_trigger")
        if not floored_keeps_fallback(f, decision["effective"]):
            v.append("floored_family_loses_fallback")
        if family_overclaims(f, decision["effective"]):
            v.append("rendering_surface_overclaims")

    if not FORM_FAMILIES.issubset(fams):
        v.append("form_family_missing")
    if not set(PROOF_DIMENSIONS).issubset(dims):
        v.append("proof_dimension_missing")
    if not PROOF_LANES.issubset(lanes):
        v.append("proof_lane_missing")
    if not CONSUMER_SURFACES.issubset(consumers):
        v.append("consumer_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_family_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def verdict_distribution(families: list[dict]) -> dict:
    dist = {"certified": 0, "narrowed": 0, "withdrawn": 0}
    for f in families:
        dist[narrow(f, False)["verdict"]] += 1
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


def base_family(families: list[dict], fid: str) -> dict:
    for f in families:
        if f["family_id"] == fid:
            return f
    raise SystemExit(f"base family not found: {fid}")


F_PROVIDER = "family:provider-connect:0001"
F_ADMIN = "family:admin-source-management:0001"
F_REQUEST = "family:request-workspace:0001"
F_PACKAGE = "family:package-install-review:0001"
F_SETTINGS = "family:settings-config-editor:0001"
F_IMPORT = "family:import-migration-center:0001"
F_PROJECTS = "family:generated-project-bootstrap:0001"

STABLE, BETA, PREVIEW, WITHDRAWN = "stable", "beta", "preview", "withdrawn"

# evidence[] index of each (dimension, lane) pair, mirroring REQUIRED_PROOF_PAIRS order.
IDX_FIELD_CONTROL = 0
IDX_FORM_VALIDATION = 1
IDX_PROVENANCE = 2
IDX_DRAFT = 3
IDX_INTERRUPTION = 4
IDX_STAGED_SHEET = 5
IDX_STRUCTURED_INPUT = 6

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("provider-certified", F_PROVIDER, {}, False,
     "A provider connect form with all proof current certifies at stable.",
     STABLE, False, []),
    ("admin-certified", F_ADMIN, {}, False,
     "An admin source-management family with all proof current certifies at stable.",
     STABLE, False, []),
    ("package-certified", F_PACKAGE, {}, False,
     "A package install-review family with all proof current certifies at stable.",
     STABLE, False, []),
    ("settings-certified", F_SETTINGS, {}, False,
     "A settings config editor family with all proof current certifies at stable.",
     STABLE, False, []),
    ("import-certified", F_IMPORT, {}, False,
     "An import/migration-center family with all proof current certifies at stable.",
     STABLE, False, []),
    ("projects-certified", F_PROJECTS, {}, False,
     "A generated-project/bootstrap family with all proof current certifies at stable.",
     STABLE, False, []),
    ("request-narrowed-baseline", F_REQUEST, {}, False,
     "The request-workspace family whose parameter provenance proof is stale narrows to beta.",
     BETA, True, ["parameter_provenance_uncertified"]),

    # Per-dimension narrowing.
    ("field-validation-partial-beta", F_SETTINGS,
     {f"evidence[{IDX_FORM_VALIDATION}].state": "partial", "renderings[*].rendered_tier": BETA}, False,
     "A partial field/form validation proof caps the family at beta.",
     BETA, True, ["field_form_validation_uncertified"]),
    ("provenance-stale-beta", F_SETTINGS,
     {f"evidence[{IDX_PROVENANCE}].state": "stale", "renderings[*].rendered_tier": BETA}, False,
     "A stale parameter-provenance proof caps the family at beta.",
     BETA, True, ["parameter_provenance_uncertified"]),
    ("draft-missing-preview", F_SETTINGS,
     {f"evidence[{IDX_DRAFT}].state": "missing", f"evidence[{IDX_DRAFT}].proof_ref": None,
      f"evidence[{IDX_DRAFT}].captured_at": None, "renderings[*].rendered_tier": PREVIEW}, False,
     "A missing draft-recovery proof caps the family at preview.",
     PREVIEW, True, ["draft_recovery_uncertified"]),
    ("interruption-missing-preview", F_PROVIDER,
     {f"evidence[{IDX_INTERRUPTION}].state": "missing", f"evidence[{IDX_INTERRUPTION}].proof_ref": None,
      f"evidence[{IDX_INTERRUPTION}].captured_at": None, "renderings[*].rendered_tier": PREVIEW}, False,
     "A missing interruption-recovery proof caps the family at preview.",
     PREVIEW, True, ["interruption_recovery_uncertified"]),
    ("staged-review-failing-withdrawn", F_PACKAGE,
     {f"evidence[{IDX_STAGED_SHEET}].state": "failing", "renderings[*].rendered_tier": WITHDRAWN}, False,
     "A failing staged-review proof withdraws the family.",
     WITHDRAWN, True, ["staged_review_uncertified"]),
    ("structured-input-failing-withdrawn", F_IMPORT,
     {f"evidence[{IDX_STRUCTURED_INPUT}].state": "failing", "renderings[*].rendered_tier": WITHDRAWN}, False,
     "A failing structured-input staged-review proof withdraws the family.",
     WITHDRAWN, True, ["staged_review_uncertified"]),

    # Combined narrowing keeps the weakest floor and orders reasons deterministically.
    ("provenance-and-draft-missing-preview", F_SETTINGS,
     {f"evidence[{IDX_PROVENANCE}].state": "stale", f"evidence[{IDX_DRAFT}].state": "missing",
      f"evidence[{IDX_DRAFT}].proof_ref": None, f"evidence[{IDX_DRAFT}].captured_at": None,
      "renderings[*].rendered_tier": PREVIEW}, False,
     "A stale provenance proof plus a missing draft proof takes the lower preview floor.",
     PREVIEW, True, ["parameter_provenance_uncertified", "draft_recovery_uncertified"]),

    # Freshness window.
    ("stale-window-beta", F_SETTINGS,
     {"renderings[*].rendered_tier": BETA}, True,
     "An elapsed certification window ages a fully-current family to beta.",
     BETA, True, ["certification_proof_stale"]),
    ("stale-window-with-failing-withdrawn", F_PACKAGE,
     {f"evidence[{IDX_STAGED_SHEET}].state": "failing", "renderings[*].rendered_tier": WITHDRAWN}, True,
     "A failing proof withdraws even with the freshness window also elapsed.",
     WITHDRAWN, True, ["staged_review_uncertified", "certification_proof_stale"]),

    # Overclaim guard.
    ("overclaim-withdraws", F_REQUEST,
     {"renderings[*].rendered_tier": STABLE}, False,
     "A consumer surface rendering stable over a beta family overclaims and withdraws it.",
     WITHDRAWN, True, ["parameter_provenance_uncertified", "verdict_overclaim"]),
]


def run_corpus_from_cases(families: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        f = apply_overrides(base_family(families, base_id), overrides)
        decision = narrow(f, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, families: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        f = apply_overrides(base_family(families, payload["base_family_id"]), payload["overrides"])
        decision = narrow(f, payload["stale_window"])
        exp = payload["expected"]
        if decision["effective"] != exp["effective_tier"]:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp['effective_tier']}")
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
            "base_family_id": base_id,
            "stale_window": stale_window,
            "overrides": overrides,
            "expected": {
                "effective_tier": exp_eff,
                "narrowed": exp_narrowed,
                "active_narrowing_reasons": exp_reasons,
            },
        }
        filename = f"{case_id}.json"
        (out_dir / filename).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        case_files.append(filename)
    index = {
        "corpus_id": "m5-form-family-certification-corpus:0001",
        "description": (
            "Perturbation corpus for the form-family certification narrowing engine. Each case "
            "starts from a canonical family, applies dotted-path overrides, and asserts the "
            "re-derived effective tier, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("form-family certification set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = verdict_distribution(packet["families"])
    sys.stdout.write(
        f"form-family certification set OK: {len(packet['families'])} families, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    families = packet["families"]
    failures = run_corpus_from_cases(families)
    failures += run_corpus_from_disk(repo_root, families)
    if failures:
        sys.stderr.write("form-family certification corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"form-family certification corpus OK: {len(CASES)} cases\n")
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
