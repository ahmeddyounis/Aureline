#!/usr/bin/env python3
"""Gate the docs/help browser content beta corpus.

The in-product docs/help browser **content** surface promotes its source /
version / freshness / client-scope rows to beta by replaying the frozen corpus at
``fixtures/help/docs_browser_content_beta/`` through the existing
``DocsBrowserRowCard`` projection
(``crates/aureline-shell/tests/docs_browser_content_beta.rs``). The Rust test
proves the *runtime* behaviour — each docs entry is wired to the live
``DocsBrowser`` surface binding, never over-claims freshness or the build match,
and labels degraded entries. This gate proves the *data* invariants a passing
test alone cannot defend against quiet drift:

1. **One shared label vocabulary.** ``manifest.json``'s freshness / version /
   source / identity-mode / trust-state / contract-state vocabularies must quote
   real tokens from crate source. The gate re-derives them from
   ``crates/aureline-shell/src/docs_browser/state.rs`` and ``truth_wiring.rs`` and
   fails closed if any mirror drifts.

2. **One release truth.** ``manifest.json``'s ``release_truth_binding`` must agree
   with the ``DocsBrowser`` surface binding re-derived straight from
   ``artifacts/release/m3/claim_manifest.json`` and
   ``artifacts/compat/m3/compatibility_report.json`` — the same claim manifest,
   compatibility report, ``docs_site`` channel, selected docs claim rows, docs
   freshness badge, downgrade / staleness flags, and contract state.

3. **Honest, wired docs content.** Every case must cite that release truth, keep
   its freshness at or below the release docs badge, count an exact build match
   against the running build as the only version-wired state, label every degraded
   entry, and resolve to the contract state it declares. A docs entry fresher than
   release truth, or a degraded entry without a freshness label, fails the gate —
   and the gate replays those two failures in-memory to prove they fire. The
   ``ready`` / ``stale`` / ``degraded`` contract states must each be covered.

The corpus is pure JSON, so this gate needs no Rust toolchain. Exit code is
non-zero on any error finding.
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

LABEL = "beta-docs-browser-content"

CORPUS_DIR_REL = "fixtures/help/docs_browser_content_beta"
MANIFEST_REL = f"{CORPUS_DIR_REL}/manifest.json"
RUST_TEST_REL = "crates/aureline-shell/tests/docs_browser_content_beta.rs"
STATE_SRC_REL = "crates/aureline-shell/src/docs_browser/state.rs"
TRUTH_WIRING_SRC_REL = "crates/aureline-shell/src/docs_browser/truth_wiring.rs"

REQUIRED_CONTRACT_STATES = {"ready", "stale", "degraded"}

# Confidence ordering over the docs freshness badge vocabulary. A docs entry may
# match or fall below the release-truth badge, never exceed it.
FRESHNESS_RANK = {
    "authoritative_live": 4,
    "warm_cached": 3,
    "degraded_cached": 2,
    "stale": 1,
    "unverified": 0,
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def read_text(path: Path) -> str:
    if not path.exists():
        raise SystemExit(f"missing source file: {path}")
    return path.read_text(encoding="utf-8")


# --------------------------------------------------------------------------- #
# Vocabulary extraction from crate source
# --------------------------------------------------------------------------- #


def extract_fn_tokens(text: str, fn_name: str, source_rel: str) -> list[str]:
    """Parse the snake_case tokens from a module-level `fn -> &'static str`."""
    fn_match = re.search(
        rf"fn {fn_name}\([^)]*\)\s*->\s*&'static str\s*\{{(.*?)\n\}}",
        text,
        re.DOTALL,
    )
    if not fn_match:
        raise SystemExit(f"could not locate `fn {fn_name}` in {source_rel}")
    tokens = re.findall(r"=>\s*\"([^\"]+)\"", fn_match.group(1))
    if not tokens:
        raise SystemExit(f"no tokens parsed from `{fn_name}` in {source_rel}")
    return tokens


def extract_impl_as_str_tokens(text: str, enum_name: str, source_rel: str) -> list[str]:
    """Parse the `<Enum>::as_str` snake_case tokens from crate source."""
    impl_match = re.search(rf"impl {enum_name} \{{(.*?)\n\}}", text, re.DOTALL)
    if not impl_match:
        raise SystemExit(f"could not locate `impl {enum_name}` in {source_rel}")
    as_str_match = re.search(
        r"fn as_str\(self\) -> &'static str \{(.*?)\n    \}",
        impl_match.group(1),
        re.DOTALL,
    )
    if not as_str_match:
        raise SystemExit(f"could not locate `{enum_name}::as_str` in {source_rel}")
    tokens = re.findall(r"Self::\w+\s*=>\s*\"([^\"]+)\"", as_str_match.group(1))
    if not tokens:
        raise SystemExit(f"no {enum_name} tokens parsed from {source_rel}")
    return tokens


def derive_source_vocabularies(repo_root: Path) -> dict[str, set[str]]:
    state = read_text(repo_root / STATE_SRC_REL)
    truth = read_text(repo_root / TRUTH_WIRING_SRC_REL)
    return {
        "freshness_class_vocabulary": set(
            extract_fn_tokens(state, "freshness_class_token", STATE_SRC_REL)
        ),
        "version_match_state_vocabulary": set(
            extract_fn_tokens(state, "version_match_state_token", STATE_SRC_REL)
        ),
        "source_class_vocabulary": set(
            extract_fn_tokens(state, "source_class_token", STATE_SRC_REL)
        ),
        "identity_mode_vocabulary": set(
            extract_fn_tokens(state, "identity_mode_token", STATE_SRC_REL)
        ),
        "trust_state_vocabulary": set(
            extract_fn_tokens(state, "trust_state_token", STATE_SRC_REL)
        ),
        "contract_state_vocabulary": set(
            extract_impl_as_str_tokens(
                truth, "ServiceContractState", TRUTH_WIRING_SRC_REL
            )
        ),
    }


# --------------------------------------------------------------------------- #
# Release-truth re-derivation from the artifacts
# --------------------------------------------------------------------------- #


def parse_date(value: str) -> dt.date | None:
    try:
        return dt.date.fromisoformat(value[:10])
    except (ValueError, TypeError):
        return None


def freshness_state(badge: str, evidence_date: str, window: int, as_of: str) -> str:
    """Port of `service_health::project_freshness` for the docs rows."""
    evidence = parse_date(evidence_date)
    now = parse_date(as_of)
    if evidence is None or now is None or badge not in FRESHNESS_RANK:
        return "unable_to_evaluate_freshness"
    days = (now - evidence).days
    if days <= window:
        remaining = window - days
        if badge == "unverified":
            return "unverified_but_current"
        if badge == "degraded_cached":
            return "degraded_but_current"
        if badge == "stale":
            return "review_due_soon"
        if window > 0 and remaining <= window // 2:
            return "review_due_soon"
        return "current"
    overdue = days - window
    return "evidence_expired" if overdue > window else "review_overdue"


@dataclass
class DocsReleaseTruth:
    claim_manifest_ref: str
    compatibility_report_ref: str
    required_channel_id: str
    release_freshness_badge: str | None
    service_contract_state: str
    freshness_state_tokens: list[str]
    evidence_stale: bool
    claim_downgraded: bool
    docs_claim_row_refs: list[str]


def derive_docs_release_truth(
    repo_root: Path,
    binding: dict[str, Any],
    findings: list[Finding],
) -> DocsReleaseTruth | None:
    """Re-derive the DocsBrowser surface binding from the release artifacts."""
    manifest_ref = binding.get("claim_manifest_ref")
    compat_ref = binding.get("compatibility_report_ref")
    if not isinstance(manifest_ref, str) or not isinstance(compat_ref, str):
        findings.append(
            Finding(
                "error",
                "binding.refs_missing",
                "release_truth_binding must cite claim_manifest_ref and "
                "compatibility_report_ref",
                "Point the binding at the release artifacts.",
                ref=MANIFEST_REL,
            )
        )
        return None

    manifest = load_json(repo_root / manifest_ref)
    compat = load_json(repo_root / compat_ref)
    if manifest.get("record_kind") != "m3_claim_manifest":
        findings.append(
            Finding(
                "error",
                "binding.manifest_record_kind",
                f"{manifest_ref} is not an m3_claim_manifest",
                "Cite the generated claim manifest.",
                ref=manifest_ref,
            )
        )
        return None
    if compat.get("record_kind") != "compatibility_report":
        findings.append(
            Finding(
                "error",
                "binding.compat_record_kind",
                f"{compat_ref} is not a compatibility_report",
                "Cite the generated compatibility report.",
                ref=compat_ref,
            )
        )
        return None

    compat_ids = {row.get("row_id") for row in compat.get("rows", [])}
    as_of = manifest.get("as_of", "")

    docs_rows = []
    for row in manifest.get("rows", []):
        if row.get("claim_family") != "docs_freshness":
            continue
        if not any(
            cp.get("channel_id") == "docs_site" and cp.get("binding_status") == "required"
            for cp in row.get("channel_projections", [])
        ):
            continue
        docs_rows.append(row)

    if not docs_rows:
        findings.append(
            Finding(
                "error",
                "binding.no_docs_rows",
                "no docs_freshness rows bind the docs_site channel as required",
                "Restore the docs claim rows in the manifest.",
                ref=manifest_ref,
            )
        )
        return None

    badges = {row["freshness"]["badge_class"] for row in docs_rows}
    release_badge = next(iter(badges)) if len(badges) == 1 else None
    if release_badge is None:
        findings.append(
            Finding(
                "error",
                "binding.badge_disagreement",
                "docs freshness rows do not share a single freshness badge",
                "Re-evaluate the docs freshness truth.",
                ref=manifest_ref,
                details={"badges": sorted(badges)},
            )
        )

    claim_downgraded = any(
        row["claim_posture"]["declared"] != row["claim_posture"]["effective"]
        for row in docs_rows
    )
    support_downgraded = any(
        row["support"]["declared"] != row["support"]["effective"] for row in docs_rows
    )
    policy_blocked = any(
        row["claim_posture"]["effective"] == "policy_disabled" for row in docs_rows
    )
    withdrawn = any(
        row["claim_posture"]["effective"] == "withdrawn" for row in docs_rows
    )

    states = sorted(
        {
            freshness_state(
                row["freshness"]["badge_class"],
                row["freshness"]["evidence_date"],
                int(row["freshness"]["review_window_days"]),
                as_of,
            )
            for row in docs_rows
        }
    )
    evidence_stale = any(
        s in {"review_due_soon", "review_overdue", "evidence_expired"} for s in states
    )

    missing_compat: list[str] = []
    for row in docs_rows:
        for compat_row in row.get("compatibility_row_refs", []):
            if compat_row not in compat_ids:
                missing_compat.append(compat_row)
    if missing_compat:
        findings.append(
            Finding(
                "error",
                "binding.missing_compatibility_rows",
                "docs claim rows cite compatibility rows missing from the report",
                "Restore the compatibility rows or refresh the claim manifest.",
                ref=compat_ref,
                details={"missing": sorted(set(missing_compat))},
            )
        )

    if missing_compat:
        contract_state = "contract_mismatch"
    elif policy_blocked:
        contract_state = "policy_blocked"
    elif withdrawn:
        contract_state = "unavailable"
    elif evidence_stale:
        contract_state = "stale"
    elif claim_downgraded or support_downgraded:
        contract_state = "degraded"
    else:
        contract_state = "ready"

    return DocsReleaseTruth(
        claim_manifest_ref=manifest_ref,
        compatibility_report_ref=compat_ref,
        required_channel_id="docs_site",
        release_freshness_badge=release_badge,
        service_contract_state=contract_state,
        freshness_state_tokens=states,
        evidence_stale=evidence_stale,
        claim_downgraded=claim_downgraded,
        docs_claim_row_refs=sorted(row["row_id"] for row in docs_rows),
    )


def validate_binding(
    binding: dict[str, Any],
    derived: DocsReleaseTruth,
    findings: list[Finding],
) -> None:
    def check(field_name: str, declared: Any, expected: Any, normalize=lambda x: x) -> None:
        if normalize(declared) != normalize(expected):
            findings.append(
                Finding(
                    "error",
                    f"binding.{field_name}",
                    f"release_truth_binding.{field_name} drifted from the "
                    "re-derived docs binding",
                    "Refresh the corpus binding from the release artifacts.",
                    ref=MANIFEST_REL,
                    details={"declared": declared, "derived": expected},
                )
            )

    if binding.get("surface_class") != "docs_browser":
        findings.append(
            Finding(
                "error",
                "binding.surface_class",
                "release_truth_binding.surface_class must be docs_browser",
                "Describe the docs browser surface.",
                ref=MANIFEST_REL,
            )
        )
    check("required_channel_id", binding.get("required_channel_id"), derived.required_channel_id)
    check(
        "service_contract_state",
        binding.get("service_contract_state"),
        derived.service_contract_state,
    )
    check("evidence_stale", binding.get("evidence_stale"), derived.evidence_stale)
    check("claim_downgraded", binding.get("claim_downgraded"), derived.claim_downgraded)
    check(
        "freshness_state_tokens",
        binding.get("freshness_state_tokens"),
        derived.freshness_state_tokens,
        normalize=lambda v: sorted(v or []),
    )
    check(
        "docs_claim_row_refs",
        binding.get("docs_claim_row_refs"),
        derived.docs_claim_row_refs,
        normalize=lambda v: sorted(v or []),
    )
    if derived.release_freshness_badge is not None:
        check(
            "release_freshness_badge",
            binding.get("release_freshness_badge"),
            derived.release_freshness_badge,
        )


# --------------------------------------------------------------------------- #
# Per-entry validation (mirror of the Rust `validate_entry`)
# --------------------------------------------------------------------------- #


def derive_entry_contract_state(card: dict[str, Any]) -> str:
    badge = card["freshness_row"]["class_token"]
    version = card["version_row"]["state_token"]
    if badge == "stale":
        return "stale"
    if badge in {"degraded_cached", "unverified"}:
        return "degraded"
    if badge in {"authoritative_live", "warm_cached"}:
        return "ready" if version == "exact_build_match" else "degraded"
    return "contract_mismatch"


def snapshot_label_present(card: dict[str, Any]) -> bool:
    label = card["source_row"].get("snapshot_age_label")
    return isinstance(label, str) and label.strip() != ""


def entry_violations(
    case: dict[str, Any],
    binding: dict[str, Any],
    manifest: dict[str, Any],
) -> list[str]:
    """Return the violation codes for one docs content entry."""
    card = case["row_card"]
    wiring = case["wiring"]
    expect = case["expect"]
    out: list[str] = []

    if wiring.get("claim_manifest_ref") != binding.get("claim_manifest_ref"):
        out.append("wrong_claim_manifest_ref")
    if wiring.get("compatibility_report_ref") != binding.get("compatibility_report_ref"):
        out.append("wrong_compatibility_report_ref")
    if wiring.get("required_channel_id") != binding.get("required_channel_id"):
        out.append("wrong_channel")
    if wiring.get("docs_claim_row_ref") not in binding.get("docs_claim_row_refs", []):
        out.append("unknown_docs_claim_row")

    release_badge = binding.get("release_freshness_badge")
    release_rank = FRESHNESS_RANK.get(release_badge)
    badge = card["freshness_row"]["class_token"]
    entry_rank = FRESHNESS_RANK.get(badge)
    if entry_rank is None:
        out.append("unknown_freshness_class")
    elif release_rank is not None and entry_rank > release_rank:
        out.append("freshness_overclaims_release_truth")
    freshness_wired = entry_rank is not None and entry_rank == release_rank
    if freshness_wired != bool(expect.get("freshness_wired")):
        out.append("freshness_wired_mismatch")
    if bool(card["freshness_row"].get("degraded")) != bool(expect.get("freshness_degraded")):
        out.append("freshness_degraded_mismatch")
    if card["freshness_row"].get("degraded") and not snapshot_label_present(card):
        out.append("degraded_entry_missing_freshness_label")

    version = card["version_row"]["state_token"]
    version_wired = version == "exact_build_match"
    if version_wired != bool(expect.get("version_wired")):
        out.append("version_wired_mismatch")
    if version_wired and card["version_row"].get("running_build_identity_ref") != binding.get(
        "running_build_identity_ref"
    ):
        out.append("exact_version_wrong_running_build")

    if version not in manifest.get("version_match_state_vocabulary", []):
        out.append("unknown_version_state")
    if card["source_row"]["class_token"] not in manifest.get("source_class_vocabulary", []):
        out.append("unknown_source_class")
    if card["client_scope_row"]["identity_mode_token"] not in manifest.get(
        "identity_mode_vocabulary", []
    ):
        out.append("unknown_identity_mode")
    if card["client_scope_row"]["trust_state_token"] not in manifest.get(
        "trust_state_vocabulary", []
    ):
        out.append("unknown_trust_state")

    if expect.get("contract_state") not in manifest.get("contract_state_vocabulary", []):
        out.append("unknown_contract_state")
    if derive_entry_contract_state(card) != expect.get("contract_state"):
        out.append("contract_state_mismatch")
    if snapshot_label_present(card) != bool(expect.get("freshness_label_present")):
        out.append("freshness_label_present_mismatch")

    return out


# --------------------------------------------------------------------------- #
# Validation driver
# --------------------------------------------------------------------------- #


def validate_vocabularies(
    manifest: dict[str, Any],
    source_vocab: dict[str, set[str]],
    findings: list[Finding],
) -> None:
    for key, source_set in source_vocab.items():
        mirror = manifest.get(key)
        if not isinstance(mirror, list) or not mirror:
            findings.append(
                Finding(
                    "error",
                    f"vocab.{key}_missing",
                    f"manifest.{key} must be a non-empty list",
                    f"Mirror the crate source vocabulary for {key}.",
                    ref=MANIFEST_REL,
                )
            )
            continue
        if set(mirror) != source_set:
            findings.append(
                Finding(
                    "error",
                    f"vocab.{key}_drift",
                    f"manifest.{key} drifted from crate source",
                    "Refresh the manifest vocabulary mirror.",
                    ref=MANIFEST_REL,
                    details={"mirror": sorted(mirror), "source": sorted(source_set)},
                )
            )


def validate_case_file(
    repo_root: Path,
    case_file: str,
    binding: dict[str, Any],
    manifest: dict[str, Any],
    findings: list[Finding],
) -> str | None:
    case_ref = f"{CORPUS_DIR_REL}/{case_file}"
    case = load_json(repo_root / case_ref)
    if case.get("record_kind") != "docs_browser_content_beta_case":
        findings.append(
            Finding(
                "error",
                "case.record_kind",
                f"{case_file} has an unexpected record_kind",
                "Use record_kind docs_browser_content_beta_case.",
                ref=case_ref,
            )
        )
        return None

    for violation in entry_violations(case, binding, manifest):
        findings.append(
            Finding(
                "error",
                f"case.{violation}",
                f"{case.get('case_id', case_file)}: {violation}",
                "Align the docs entry with the release-truth binding.",
                ref=case_ref,
            )
        )
    return case.get("expect", {}).get("contract_state")


def run_negative_drills(
    repo_root: Path,
    binding: dict[str, Any],
    manifest: dict[str, Any],
    findings: list[Finding],
) -> None:
    """Replay the two acceptance failures in-memory so the gate proves they fire."""
    current = load_json(
        repo_root / f"{CORPUS_DIR_REL}/project_docs_release_current.json"
    )
    current["row_card"]["freshness_row"]["class_token"] = "authoritative_live"
    current["row_card"]["freshness_row"]["label"] = "Authoritative (live)"
    current["row_card"]["freshness_row"]["degraded"] = False
    if "freshness_overclaims_release_truth" not in entry_violations(
        current, binding, manifest
    ):
        findings.append(
            Finding(
                "error",
                "drill.overclaim_not_detected",
                "an entry fresher than release truth was not rejected",
                "Restore the freshness-ceiling check.",
                ref=MANIFEST_REL,
            )
        )

    stale = load_json(repo_root / f"{CORPUS_DIR_REL}/mirrored_docs_stale_labeled.json")
    stale["row_card"]["source_row"].pop("snapshot_age_label", None)
    if "degraded_entry_missing_freshness_label" not in entry_violations(
        stale, binding, manifest
    ):
        findings.append(
            Finding(
                "error",
                "drill.unlabeled_stale_not_detected",
                "a degraded entry without a freshness label was not rejected",
                "Restore the degraded-label check.",
                ref=MANIFEST_REL,
            )
        )


def validate_corpus(repo_root: Path, findings: list[Finding]) -> None:
    manifest = load_json(repo_root / MANIFEST_REL)
    if manifest.get("record_kind") != "docs_browser_content_beta_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "manifest.json has an unexpected record_kind",
                "Use record_kind docs_browser_content_beta_manifest.",
                ref=MANIFEST_REL,
            )
        )

    source_vocab = derive_source_vocabularies(repo_root)
    validate_vocabularies(manifest, source_vocab, findings)

    binding = manifest.get("release_truth_binding")
    if not isinstance(binding, dict):
        findings.append(
            Finding(
                "error",
                "manifest.binding_missing",
                "manifest.release_truth_binding must be an object",
                "Add the docs browser release-truth binding snapshot.",
                ref=MANIFEST_REL,
            )
        )
        return

    derived = derive_docs_release_truth(repo_root, binding, findings)
    if derived is not None:
        validate_binding(binding, derived, findings)

    cases = manifest.get("cases")
    if not isinstance(cases, list) or not cases:
        findings.append(
            Finding(
                "error",
                "manifest.cases_missing",
                "manifest.cases must be a non-empty list",
                "List the corpus case files.",
                ref=MANIFEST_REL,
            )
        )
        cases = []

    covered: set[str] = set()
    for case_file in cases:
        if not isinstance(case_file, str):
            findings.append(
                Finding(
                    "error",
                    "manifest.case_entry",
                    f"case entry {case_file!r} must be a string filename",
                    "Use the case file name.",
                    ref=MANIFEST_REL,
                )
            )
            continue
        state = validate_case_file(repo_root, case_file, binding, manifest, findings)
        if state:
            covered.add(state)

    missing = sorted(REQUIRED_CONTRACT_STATES - covered)
    if missing:
        findings.append(
            Finding(
                "error",
                "coverage.required_contract_state_missing",
                f"corpus is missing cases for contract states: {', '.join(missing)}",
                "Add a case for each of ready, stale, degraded.",
                ref=MANIFEST_REL,
                details={"missing": missing},
            )
        )

    declared_required = set(manifest.get("required_contract_states") or [])
    if declared_required != REQUIRED_CONTRACT_STATES:
        findings.append(
            Finding(
                "error",
                "manifest.required_list_mismatch",
                "manifest.required_contract_states must equal "
                f"{sorted(REQUIRED_CONTRACT_STATES)}",
                "Align the manifest required list with the promotion target.",
                ref=MANIFEST_REL,
                details={"declared": sorted(declared_required)},
            )
        )

    if cases:
        run_negative_drills(repo_root, binding, manifest, findings)

    rust_test = read_text(repo_root / RUST_TEST_REL)
    if CORPUS_DIR_REL not in rust_test:
        findings.append(
            Finding(
                "error",
                "rust_test.corpus_unreferenced",
                f"the Rust drill {RUST_TEST_REL} does not reference {CORPUS_DIR_REL}",
                "Point the Rust drill at the frozen corpus directory.",
                ref=RUST_TEST_REL,
            )
        )


def write_report(repo_root: Path, report_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "beta_docs_browser_content",
        "corpus_ref": CORPUS_DIR_REL,
        "generated_at": dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    report_path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    validate_corpus(repo_root, findings)

    if args.report:
        write_report(repo_root, str(args.report), findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"

    print(f"[{LABEL}] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[{LABEL}] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[{LABEL}]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(f"[{LABEL}] interrupted", file=sys.stderr)
        sys.exit(130)
