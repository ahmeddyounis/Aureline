#!/usr/bin/env python3
"""Validate the M3 cohort and certified-archetype scorecards.

For every named scorecard the validator:

  - parses the YAML front matter block;
  - enforces required fields (owner, freshness date, downgrade policy,
    owner handoff path, consuming surfaces);
  - applies the canonical downgrade automation rules from the scorecard
    indices to compute a derived effective support class; and
  - writes the derived register so docs, support, and release packets can
    consume one canonical truth rather than restating freshness logic.

Acceptance shape exercised by this validator:

  - Each cohort and archetype row owns a current scorecard with owner,
    freshness date, open waivers, and downgrade policy.
  - Expired or degraded scorecards automatically downgrade to
    ``retest_pending``, ``limited``, ``preview``, or ``evidence_stale``
    rather than staying green.
  - The derived register is the single artifact docs/support/release
    packets cite.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_COHORT_INDEX_REL = "artifacts/milestones/m3/cohorts/scorecard_index.yaml"
DEFAULT_ARCHETYPE_INDEX_REL = (
    "artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml"
)
DEFAULT_REGISTER_REL = (
    "artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json"
)
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m3/captures/"
    "cohort_archetype_scorecard_validation_capture.json"
)
DEFAULT_CLAIMED_SURFACE_REGISTER_REL = (
    "artifacts/milestones/m3/claimed_surface_register.json"
)
DEFAULT_COHORT_GUARDRAILS_REL = "artifacts/milestones/m3/cohort_guardrails.yaml"

REQUIRED_COHORT_SCORECARD_IDS = {
    "cohort_scorecard:design_partner",
    "cohort_scorecard:extension_author",
    "cohort_scorecard:managed_pilot",
}

REQUIRED_ARCHETYPE_SCORECARD_IDS = {
    "archetype_scorecard:ts_web_app_or_service",
    "archetype_scorecard:python_service_or_data_app",
    "archetype_scorecard:java_or_kotlin_service",
    "archetype_scorecard:rust_workspace",
    "archetype_scorecard:go_service_or_monorepo_slice",
    "archetype_scorecard:c_or_cpp_native_project",
}

DERIVED_STATE_VOCABULARY = {
    "current",
    "retest_pending",
    "limited",
    "preview",
    "evidence_stale",
}

FRONT_MATTER_RE = re.compile(r"\A---\n(.*?)\n---\n", re.DOTALL)

ID_PREFIXES = (
    "archetype_row:",
    "archetype_certification_seed:",
    "beta_archetype:",
    "beta_surface:",
    "cohort:",
    "compat_row:",
    "cohort_scorecard:",
    "archetype_scorecard:",
    "waiver:",
)
PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py", ".mmd")


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
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--cohort-index", default=DEFAULT_COHORT_INDEX_REL)
    parser.add_argument("--archetype-index", default=DEFAULT_ARCHETYPE_INDEX_REL)
    parser.add_argument(
        "--claimed-surface-register",
        default=DEFAULT_CLAIMED_SURFACE_REGISTER_REL,
    )
    parser.add_argument(
        "--cohort-guardrails",
        default=DEFAULT_COHORT_GUARDRAILS_REL,
    )
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
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
                "require 'date';"
                " payload = YAML.safe_load(File.read(ARGV[0]),"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def render_yaml_string_as_json(text: str, label: str) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "require 'date';"
                " payload = YAML.safe_load(STDIN.read,"
                " permitted_classes: [Date, Time], aliases: false);"
                " STDOUT.write(JSON.generate(payload))"
            ),
        ],
        input=text,
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML front matter in {label}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {label}: {exc}"
        ) from exc


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object/mapping")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(
            f"{label} must be a YYYY-MM-DD date, got {value!r}"
        ) from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_refs(
    repo_root: Path,
    refs: list[Any],
    label: str,
    findings: list[Finding],
) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a "
                    "repo-relative artifact path or stable row id.",
                )
            )
            continue
        ref = raw_ref.strip()
        if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_ref",
                    message=f"{label}[{idx}] does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced artifact "
                    "so the scorecard lane stays inspectable.",
                    ref=ref,
                )
            )


def validate_header(
    payload: dict[str, Any],
    label: str,
    findings: list[Finding],
) -> dt.date:
    schema_version = ensure_int(
        payload.get("schema_version"), f"{label}.schema_version"
    )
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=(
                    f"{label}.schema_version must be 1, got {schema_version}"
                ),
                remediation="Update the validator in the same change that "
                "bumps the artifact schema.",
            )
        )
    as_of = parse_iso_date(
        ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of"
    )
    ensure_str(payload.get("owner"), f"{label}.owner")
    return as_of


def extract_front_matter(text: str, label: str) -> dict[str, Any]:
    match = FRONT_MATTER_RE.match(text)
    if not match:
        raise SystemExit(
            f"{label}: scorecard markdown must start with a YAML front matter "
            "block delimited by '---' lines"
        )
    block = match.group(1)
    payload = render_yaml_string_as_json(block, label)
    return ensure_dict(payload, f"{label}.front_matter")


def derive_effective_state(
    scorecard: dict[str, Any],
    automation: dict[str, Any],
    index_as_of: dt.date,
    findings: list[Finding],
) -> dict[str, Any]:
    label = scorecard.get("scorecard_id", "<unknown>")
    declared = ensure_str(
        scorecard.get("declared_support_class"),
        f"{label}.declared_support_class",
    )
    evidence_date = parse_iso_date(
        ensure_str(scorecard.get("evidence_date"), f"{label}.evidence_date"),
        f"{label}.evidence_date",
    )
    review_window_days = ensure_int(
        scorecard.get("review_window_days"), f"{label}.review_window_days"
    )
    if review_window_days <= 0:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.review_window_days.invalid",
                message=f"{label}.review_window_days must be positive",
                remediation="Set a positive review window so the validator can "
                "compute freshness.",
                ref=label,
            )
        )
        review_window_days = 1

    retest_multiplier = float(
        automation.get("retest_pending_multiplier", 1.0)
    )
    stale_multiplier = float(automation.get("evidence_stale_multiplier", 3.0))
    waiver_active_label = automation.get(
        "waiver_state_active_label", "active"
    )

    evidence_age_days = (index_as_of - evidence_date).days
    triggers: list[str] = []

    # Freshness derivation is independent of waivers.
    freshness_derivation = "current"
    if evidence_age_days > review_window_days * stale_multiplier:
        freshness_derivation = "evidence_stale"
        triggers.append(
            f"evidence_stale:age_days={evidence_age_days}"
            f":window={review_window_days}"
        )
    elif evidence_age_days > review_window_days * retest_multiplier:
        freshness_derivation = "retest_pending"
        triggers.append(
            f"retest_pending:age_days={evidence_age_days}"
            f":window={review_window_days}"
        )

    # Waiver derivation is independent of freshness.
    waivers = ensure_list(
        scorecard.get("open_waivers", []), f"{label}.open_waivers"
    )
    waiver_derivation = "none"
    for waiver_idx, raw_waiver in enumerate(waivers):
        waiver = ensure_dict(raw_waiver, f"{label}.open_waivers[{waiver_idx}]")
        state = ensure_str(
            waiver.get("state"), f"{label}.open_waivers[{waiver_idx}].state"
        )
        ensure_str(
            waiver.get("waiver_id"),
            f"{label}.open_waivers[{waiver_idx}].waiver_id",
        )
        ensure_str(
            waiver.get("expires_on"),
            f"{label}.open_waivers[{waiver_idx}].expires_on",
        )
        if state == waiver_active_label:
            triggers.append(f"open_waiver:{waiver.get('waiver_id')}")
            if scorecard.get("display_lifecycle_label") == "preview":
                waiver_derivation = "preview"
            else:
                waiver_derivation = "limited"

    # Effective support class precedence:
    #   evidence_stale > retest_pending > waiver downgrade (limited/preview)
    #   > declared support class.
    if freshness_derivation == "evidence_stale":
        effective_support_class = "evidence_stale"
    elif freshness_derivation == "retest_pending":
        effective_support_class = "retest_pending"
    elif waiver_derivation in {"limited", "preview"}:
        effective_support_class = waiver_derivation
    else:
        effective_support_class = declared

    if not triggers:
        triggers.append("none")

    declared_freshness = ensure_str(
        scorecard.get("freshness_state"), f"{label}.freshness_state"
    )
    if declared_freshness not in {
        "current",
        "retest_pending",
        "evidence_stale",
    }:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.freshness_state.invalid",
                message=(
                    f"{label}.freshness_state must be one of current, "
                    "retest_pending, evidence_stale"
                ),
                remediation="Use a freshness state from the cohort guardrails "
                "evidence_freshness_vocabulary.",
                ref=label,
            )
        )
    if declared_freshness != freshness_derivation:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.freshness_state.contradicts_derived",
                message=(
                    f"{label}.freshness_state declared {declared_freshness} but "
                    f"automation derives {freshness_derivation}"
                ),
                remediation="Refresh evidence or update freshness_state to "
                "match the computed derivation.",
                ref=label,
                details={
                    "freshness_derivation": freshness_derivation,
                    "triggers": triggers,
                },
            )
        )

    if effective_support_class not in (
        DERIVED_STATE_VOCABULARY | {declared}
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.effective_support_class.invalid",
                message=(
                    f"{label} effective_support_class {effective_support_class}"
                    " is not the declared class or a known downgrade state"
                ),
                remediation="Fix the declared_support_class or extend the "
                "automation vocabulary.",
                ref=label,
            )
        )

    return {
        "freshness_derivation": freshness_derivation,
        "waiver_derivation": waiver_derivation,
        "effective_support_class": effective_support_class,
        "evidence_age_days": evidence_age_days,
        "triggers": triggers,
    }


def validate_common_scorecard(
    repo_root: Path,
    scorecard: dict[str, Any],
    label: str,
    expected_kind: str,
    findings: list[Finding],
) -> None:
    schema_version = ensure_int(
        scorecard.get("schema_version"), f"{label}.schema_version"
    )
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.schema_version.unsupported",
                message=(
                    f"{label}.schema_version must be 1, got {schema_version}"
                ),
                remediation="Update the validator and downstream consumers in "
                "the same change that bumps the schema.",
                ref=label,
            )
        )
    kind = ensure_str(scorecard.get("scorecard_kind"), f"{label}.scorecard_kind")
    if kind != expected_kind:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.scorecard_kind.mismatch",
                message=(
                    f"{label}.scorecard_kind must be {expected_kind}, got {kind}"
                ),
                remediation="Fix the scorecard_kind in the front matter.",
                ref=label,
            )
        )
    ensure_str(scorecard.get("scorecard_id"), f"{label}.scorecard_id")
    ensure_str(scorecard.get("title"), f"{label}.title")
    ensure_str(scorecard.get("owner"), f"{label}.owner")
    ensure_str(scorecard.get("evidence_owner"), f"{label}.evidence_owner")
    parse_iso_date(
        ensure_str(scorecard.get("as_of"), f"{label}.as_of"),
        f"{label}.as_of",
    )
    ensure_str(
        scorecard.get("display_lifecycle_label"),
        f"{label}.display_lifecycle_label",
    )

    evidence_refs = ensure_list(
        scorecard.get("evidence_refs"), f"{label}.evidence_refs"
    )
    if not evidence_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.evidence_refs.empty",
                message=f"{label}.evidence_refs must not be empty",
                remediation="Cite at least one inspectable evidence artifact.",
                ref=label,
            )
        )
    validate_path_refs(repo_root, evidence_refs, f"{label}.evidence_refs", findings)

    consuming_surfaces = ensure_list(
        scorecard.get("consuming_surfaces"),
        f"{label}.consuming_surfaces",
    )
    if not consuming_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.consuming_surfaces.empty",
                message=f"{label}.consuming_surfaces must not be empty",
                remediation="Name at least one docs/support/release surface "
                "that consumes the scorecard.",
                ref=label,
            )
        )
    validate_path_refs(
        repo_root,
        consuming_surfaces,
        f"{label}.consuming_surfaces",
        findings,
    )

    downgrade_policy = ensure_list(
        scorecard.get("downgrade_policy"),
        f"{label}.downgrade_policy",
    )
    if not downgrade_policy:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.downgrade_policy.empty",
                message=f"{label}.downgrade_policy must not be empty",
                remediation="Declare at least one downgrade trigger so the "
                "claim cannot drift silently.",
                ref=label,
            )
        )
    for rule_idx, raw_rule in enumerate(downgrade_policy):
        rule = ensure_dict(
            raw_rule, f"{label}.downgrade_policy[{rule_idx}]"
        )
        ensure_str(
            rule.get("trigger"),
            f"{label}.downgrade_policy[{rule_idx}].trigger",
        )
        ensure_str(
            rule.get("downgrade_to"),
            f"{label}.downgrade_policy[{rule_idx}].downgrade_to",
        )
        validate_path_refs(
            repo_root,
            ensure_list(
                rule.get("propagation_refs"),
                f"{label}.downgrade_policy[{rule_idx}].propagation_refs",
            ),
            f"{label}.downgrade_policy.propagation_refs",
            findings,
        )

    owner_handoff = ensure_dict(
        scorecard.get("owner_handoff_path"),
        f"{label}.owner_handoff_path",
    )
    for required in ("intake_owner", "triage_owner", "release_owner"):
        ensure_str(
            owner_handoff.get(required),
            f"{label}.owner_handoff_path.{required}",
        )
    escalation = ensure_str(
        owner_handoff.get("escalation_ref"),
        f"{label}.owner_handoff_path.escalation_ref",
    )
    if not artifact_ref_exists(repo_root, escalation):
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.owner_handoff_path.escalation_ref.missing",
                message=(
                    f"{label}.owner_handoff_path.escalation_ref does not "
                    f"resolve: {escalation}"
                ),
                remediation="Point escalation at a real governance doc.",
                ref=escalation,
            )
        )

    # open_waivers may be empty; structure is validated in derive_effective_state.
    ensure_list(
        scorecard.get("open_waivers", []),
        f"{label}.open_waivers",
    )


def load_scorecard_index(
    repo_root: Path,
    rel: str,
    label: str,
    findings: list[Finding],
) -> tuple[dict[str, Any], dt.date]:
    index = ensure_dict(
        render_yaml_as_json(repo_root / rel),
        label,
    )
    as_of = validate_header(index, label, findings)
    ensure_str(index.get("index_id"), f"{label}.index_id")
    validator = ensure_dict(index.get("validator"), f"{label}.validator")
    ensure_str(validator.get("script_ref"), f"{label}.validator.script_ref")
    ensure_str(validator.get("command"), f"{label}.validator.command")
    validate_path_refs(
        repo_root,
        [validator.get("script_ref")],
        f"{label}.validator.script_ref",
        findings,
    )
    # derived_register_ref and validation_capture_ref are validator outputs.
    # Only check that they are declared and have plausible relative paths;
    # existence is enforced at the end after write.
    ensure_str(
        index.get("derived_register_ref"),
        f"{label}.derived_register_ref",
    )
    ensure_str(
        index.get("validation_capture_ref"),
        f"{label}.validation_capture_ref",
    )
    validate_path_refs(
        repo_root,
        [index.get("human_entrypoint_ref")],
        f"{label}.human_entrypoint_ref",
        findings,
    )
    validate_path_refs(
        repo_root,
        ensure_list(
            index.get("consuming_surfaces"),
            f"{label}.consuming_surfaces",
        ),
        f"{label}.consuming_surfaces",
        findings,
    )
    return index, as_of


def validate_cohort_scorecards(
    repo_root: Path,
    index: dict[str, Any],
    index_as_of: dt.date,
    known_cohort_ids: set[str],
    known_surface_ids: set[str],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    automation = ensure_dict(
        index.get("downgrade_automation"),
        "cohort_index.downgrade_automation",
    )
    rows: list[dict[str, Any]] = []
    scorecards = ensure_list(
        index.get("scorecards"), "cohort_index.scorecards"
    )
    seen_ids: set[str] = set()
    for idx, raw_entry in enumerate(scorecards):
        entry = ensure_dict(raw_entry, f"cohort_index.scorecards[{idx}]")
        scorecard_id = ensure_str(
            entry.get("scorecard_id"),
            f"cohort_index.scorecards[{idx}].scorecard_id",
        )
        cohort_id = ensure_str(
            entry.get("cohort_id"),
            f"cohort_index.scorecards[{idx}].cohort_id",
        )
        sub_focus = ensure_str(
            entry.get("cohort_sub_focus"),
            f"cohort_index.scorecards[{idx}].cohort_sub_focus",
        )
        scorecard_ref = ensure_str(
            entry.get("scorecard_ref"),
            f"cohort_index.scorecards[{idx}].scorecard_ref",
        )
        seen_ids.add(scorecard_id)
        if cohort_id not in known_cohort_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohort_scorecard.unknown_cohort_id",
                    message=f"{scorecard_id} cites unknown cohort_id: {cohort_id}",
                    remediation="Use a cohort_id from cohort_guardrails.yaml.",
                    ref=scorecard_id,
                )
            )

        scorecard_path = repo_root / scorecard_ref
        if not scorecard_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohort_scorecard.missing",
                    message=f"{scorecard_id} scorecard file missing: {scorecard_ref}",
                    remediation="Author the scorecard markdown.",
                    ref=scorecard_ref,
                )
            )
            continue
        text = scorecard_path.read_text(encoding="utf-8")
        scorecard = extract_front_matter(text, scorecard_ref)
        validate_common_scorecard(
            repo_root,
            scorecard,
            scorecard_ref,
            expected_kind="cohort_scorecard",
            findings=findings,
        )
        primary_surfaces = ensure_list(
            scorecard.get("primary_surface_refs"),
            f"{scorecard_ref}.primary_surface_refs",
        )
        for surface_ref in primary_surfaces:
            if surface_ref not in known_surface_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="cohort_scorecard.unknown_surface_ref",
                        message=(
                            f"{scorecard_id} cites unknown surface_ref: "
                            f"{surface_ref}"
                        ),
                        remediation="Use a beta_surface id from "
                        "claimed_surface_register.json.",
                        ref=scorecard_id,
                    )
                )
        body_id = scorecard.get("scorecard_id")
        if body_id != scorecard_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohort_scorecard.id_mismatch",
                    message=(
                        f"index scorecard_id {scorecard_id} does not match "
                        f"front matter {body_id}"
                    ),
                    remediation="Keep the index and scorecard front matter ids "
                    "in lockstep.",
                    ref=scorecard_id,
                )
            )
        body_cohort = scorecard.get("cohort_id")
        if body_cohort != cohort_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohort_scorecard.cohort_id_mismatch",
                    message=(
                        f"index cohort_id {cohort_id} does not match scorecard "
                        f"front matter cohort_id {body_cohort}"
                    ),
                    remediation="Keep the index and scorecard cohort bindings "
                    "in lockstep.",
                    ref=scorecard_id,
                )
            )
        body_sub_focus = scorecard.get("cohort_sub_focus")
        if body_sub_focus != sub_focus:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cohort_scorecard.sub_focus_mismatch",
                    message=(
                        f"index cohort_sub_focus {sub_focus} does not match "
                        f"front matter {body_sub_focus}"
                    ),
                    remediation="Keep the index and scorecard cohort_sub_focus "
                    "in lockstep.",
                    ref=scorecard_id,
                )
            )

        derivation = derive_effective_state(
            scorecard, automation, index_as_of, findings
        )
        rows.append(
            {
                "scorecard_id": scorecard_id,
                "scorecard_kind": "cohort_scorecard",
                "scorecard_ref": scorecard_ref,
                "cohort_id": cohort_id,
                "cohort_sub_focus": sub_focus,
                "owner": scorecard.get("owner"),
                "evidence_owner": scorecard.get("evidence_owner"),
                "evidence_date": scorecard.get("evidence_date"),
                "review_window_days": scorecard.get("review_window_days"),
                "evidence_age_days": derivation["evidence_age_days"],
                "declared_support_class": scorecard.get(
                    "declared_support_class"
                ),
                "freshness_derivation": derivation["freshness_derivation"],
                "waiver_derivation": derivation["waiver_derivation"],
                "effective_support_class": derivation[
                    "effective_support_class"
                ],
                "display_lifecycle_label": scorecard.get(
                    "display_lifecycle_label"
                ),
                "open_waivers": scorecard.get("open_waivers", []),
                "downgrade_triggers_fired": derivation["triggers"],
                "consuming_surfaces": scorecard.get("consuming_surfaces", []),
                "owner_handoff_path": scorecard.get("owner_handoff_path", {}),
            }
        )

    missing = REQUIRED_COHORT_SCORECARD_IDS - seen_ids
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="cohort_index.missing_required_scorecards",
                message="cohort index is missing required scorecards",
                remediation="Cover partner, extension-author, and managed-pilot.",
                details={"missing": sorted(missing)},
            )
        )
    return rows


def validate_archetype_scorecards(
    repo_root: Path,
    index: dict[str, Any],
    index_as_of: dt.date,
    known_archetype_refs: set[str],
    automation: dict[str, Any],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    scorecards = ensure_list(
        index.get("scorecards"), "archetype_index.scorecards"
    )
    seen_ids: set[str] = set()
    for idx, raw_entry in enumerate(scorecards):
        entry = ensure_dict(raw_entry, f"archetype_index.scorecards[{idx}]")
        scorecard_id = ensure_str(
            entry.get("scorecard_id"),
            f"archetype_index.scorecards[{idx}].scorecard_id",
        )
        archetype_ref = ensure_str(
            entry.get("archetype_row_ref"),
            f"archetype_index.scorecards[{idx}].archetype_row_ref",
        )
        scorecard_ref = ensure_str(
            entry.get("scorecard_ref"),
            f"archetype_index.scorecards[{idx}].scorecard_ref",
        )
        seen_ids.add(scorecard_id)
        if archetype_ref not in known_archetype_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="archetype_scorecard.unknown_row_ref",
                    message=(
                        f"{scorecard_id} cites unknown archetype_row_ref: "
                        f"{archetype_ref}"
                    ),
                    remediation="Use an archetype_row_ref from "
                    "claimed_surface_register.json.",
                    ref=scorecard_id,
                )
            )

        scorecard_path = repo_root / scorecard_ref
        if not scorecard_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="archetype_scorecard.missing",
                    message=(
                        f"{scorecard_id} scorecard file missing: {scorecard_ref}"
                    ),
                    remediation="Author the scorecard markdown.",
                    ref=scorecard_ref,
                )
            )
            continue
        text = scorecard_path.read_text(encoding="utf-8")
        scorecard = extract_front_matter(text, scorecard_ref)
        validate_common_scorecard(
            repo_root,
            scorecard,
            scorecard_ref,
            expected_kind="archetype_scorecard",
            findings=findings,
        )
        body_id = scorecard.get("scorecard_id")
        if body_id != scorecard_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="archetype_scorecard.id_mismatch",
                    message=(
                        f"index scorecard_id {scorecard_id} does not match "
                        f"front matter {body_id}"
                    ),
                    remediation="Keep the index and scorecard front matter ids "
                    "in lockstep.",
                    ref=scorecard_id,
                )
            )
        body_archetype = scorecard.get("archetype_row_ref")
        if body_archetype != archetype_ref:
            findings.append(
                Finding(
                    severity="error",
                    check_id="archetype_scorecard.row_ref_mismatch",
                    message=(
                        f"index archetype_row_ref {archetype_ref} does not "
                        f"match front matter {body_archetype}"
                    ),
                    remediation="Keep the index and scorecard archetype "
                    "bindings in lockstep.",
                    ref=scorecard_id,
                )
            )
        for matrix_field in ("minimum_platform_matrix", "minimum_mode_matrix"):
            entries = ensure_list(
                scorecard.get(matrix_field),
                f"{scorecard_ref}.{matrix_field}",
            )
            if not entries:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"archetype_scorecard.{matrix_field}.empty",
                        message=(
                            f"{scorecard_id}.{matrix_field} must not be empty"
                        ),
                        remediation="Name at least one platform/mode the "
                        "archetype claim is bound to.",
                        ref=scorecard_id,
                    )
                )

        derivation = derive_effective_state(
            scorecard, automation, index_as_of, findings
        )
        rows.append(
            {
                "scorecard_id": scorecard_id,
                "scorecard_kind": "archetype_scorecard",
                "scorecard_ref": scorecard_ref,
                "archetype_row_ref": archetype_ref,
                "owner": scorecard.get("owner"),
                "evidence_owner": scorecard.get("evidence_owner"),
                "evidence_date": scorecard.get("evidence_date"),
                "review_window_days": scorecard.get("review_window_days"),
                "evidence_age_days": derivation["evidence_age_days"],
                "declared_support_class": scorecard.get(
                    "declared_support_class"
                ),
                "freshness_derivation": derivation["freshness_derivation"],
                "waiver_derivation": derivation["waiver_derivation"],
                "effective_support_class": derivation[
                    "effective_support_class"
                ],
                "target_support_class_at_beta_exit": scorecard.get(
                    "target_support_class_at_beta_exit"
                ),
                "target_support_class_at_stable": scorecard.get(
                    "target_support_class_at_stable"
                ),
                "display_lifecycle_label": scorecard.get(
                    "display_lifecycle_label"
                ),
                "open_waivers": scorecard.get("open_waivers", []),
                "downgrade_triggers_fired": derivation["triggers"],
                "consuming_surfaces": scorecard.get("consuming_surfaces", []),
                "owner_handoff_path": scorecard.get("owner_handoff_path", {}),
            }
        )

    missing = REQUIRED_ARCHETYPE_SCORECARD_IDS - seen_ids
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="archetype_index.missing_required_scorecards",
                message="archetype index is missing required scorecards",
                remediation=(
                    "Cover TS/JS, Python, Java/Kotlin, Rust, Go, and C/C++."
                ),
                details={"missing": sorted(missing)},
            )
        )
    return rows


def write_register(
    path: Path,
    cohort_rows: list[dict[str, Any]],
    archetype_rows: list[dict[str, Any]],
    cohort_index: dict[str, Any],
    archetype_index: dict[str, Any],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )
    payload = {
        "schema_version": 1,
        "register_id": "m3_cohort_archetype_scorecard_register",
        "generated_at": generated_at,
        "as_of": cohort_index.get("as_of"),
        "cohort_index_ref": (
            "artifacts/milestones/m3/cohorts/scorecard_index.yaml"
        ),
        "archetype_index_ref": (
            "artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml"
        ),
        "downgrade_automation": cohort_index.get("downgrade_automation"),
        "cohort_rows": cohort_rows,
        "archetype_rows": archetype_rows,
    }
    path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    generated_at = (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )
    payload = {
        "schema_version": 1,
        "status": "pass"
        if not any(item.severity == "error" for item in findings)
        else "fail",
        "generated_at": generated_at,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def collect_cohort_ids(cohort_guardrails: dict[str, Any]) -> set[str]:
    return {
        ensure_str(
            ensure_dict(row, "cohort_guardrails.cohorts[]").get("cohort_id"),
            "cohort_guardrails.cohorts[].cohort_id",
        )
        for row in ensure_list(
            cohort_guardrails.get("cohorts"), "cohort_guardrails.cohorts"
        )
    }


def collect_register_surfaces(register: dict[str, Any]) -> set[str]:
    return {
        ensure_str(
            ensure_dict(row, "claimed_surfaces[]").get("surface_id"),
            "claimed_surfaces[].surface_id",
        )
        for row in ensure_list(
            register.get("claimed_surfaces"), "register.claimed_surfaces"
        )
    }


def collect_register_archetype_refs(register: dict[str, Any]) -> set[str]:
    return {
        ensure_str(
            ensure_dict(row, "claimed_archetype_rows[]").get("archetype_row_ref"),
            "claimed_archetype_rows[].archetype_row_ref",
        )
        for row in ensure_list(
            register.get("claimed_archetype_rows"),
            "register.claimed_archetype_rows",
        )
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    findings: list[Finding] = []
    cohort_index, cohort_as_of = load_scorecard_index(
        repo_root, args.cohort_index, "cohort_index", findings
    )
    archetype_index, archetype_as_of = load_scorecard_index(
        repo_root, args.archetype_index, "archetype_index", findings
    )

    claimed_register = ensure_dict(
        load_json(repo_root / args.claimed_surface_register),
        "claimed_surface_register",
    )
    cohort_guardrails = ensure_dict(
        render_yaml_as_json(repo_root / args.cohort_guardrails),
        "cohort_guardrails",
    )
    known_cohort_ids = collect_cohort_ids(cohort_guardrails)
    known_surface_ids = collect_register_surfaces(claimed_register)
    known_archetype_refs = collect_register_archetype_refs(claimed_register)

    automation = ensure_dict(
        cohort_index.get("downgrade_automation"),
        "cohort_index.downgrade_automation",
    )

    cohort_rows = validate_cohort_scorecards(
        repo_root,
        cohort_index,
        cohort_as_of,
        known_cohort_ids,
        known_surface_ids,
        findings,
    )
    archetype_rows = validate_archetype_scorecards(
        repo_root,
        archetype_index,
        archetype_as_of,
        known_archetype_refs,
        automation,
        findings,
    )

    # The validator always writes the derived register so docs / support /
    # release packets consume one canonical truth.
    write_register(
        repo_root / args.register,
        cohort_rows,
        archetype_rows,
        cohort_index,
        archetype_index,
    )
    write_report(repo_root / args.report, findings)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(
                f"ERROR [{item.check_id}]{ref}: {item.message}",
                file=sys.stderr,
            )
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    warnings = [item for item in findings if item.severity == "warning"]
    for item in warnings:
        ref = f" ({item.ref})" if item.ref else ""
        print(
            f"WARNING [{item.check_id}]{ref}: {item.message}",
            file=sys.stderr,
        )

    print("cohort and archetype scorecards validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
