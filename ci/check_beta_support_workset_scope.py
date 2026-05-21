#!/usr/bin/env python3
"""Gate the support / audit export workset / scope beta corpus.

The support / audit **export** surface promotes named-workset, sparse-slice, and
policy-limited-view scope truth to beta by replaying the frozen corpus at
``fixtures/support/workset_scope_export_beta/`` through the workspace scope-truth
projection (``crates/aureline-support/tests/workset_scope_export_beta.rs``). The
Rust test proves the *runtime* behaviour — a support/audit export declares the
active scope it was produced under, discloses out-of-scope roots and
policy-hidden members, and labels policy-limited content rather than silently
embedding it. This gate proves the *data* invariants a passing test alone cannot
defend against quiet drift:

1. **One shared scope vocabulary.** ``manifest.json``'s
   ``scope_class_vocabulary_map`` must be a 1:1 bijection between the support
   scope-class tokens and the ``aureline-workspace`` ``ScopeClass`` vocabulary.
   The gate re-derives the workspace vocabulary from crate source and fails
   closed if the map, the corpus mirror (``workspace_scope_class_vocabulary``),
   or the required scope-class coverage drift from it.

2. **One shared redaction vocabulary.** ``manifest.json``'s
   ``redaction_vocabulary`` must quote real tokens from the ``aureline-support``
   redaction enums (``RedactionState``, ``ReviewDecisionClass``,
   ``ExcludedReasonClass``). The gate re-derives those tokens from crate source
   and fails closed if the declared policy-locked / metadata labels drift.

3. **Honest scope + redaction labeling in the corpus.** Every case must keep its
   declared scope-class labels aligned with the shared map, declare a scope
   declaration, prove a real narrowed boundary (an excluded root, a
   policy-limited row, or an exclude pattern), keep the membership tallies
   aligned with the ``expect`` counts, and label *every* policy-limited row with
   the policy redaction triple (``policy_locked`` / ``omitted_policy_locked`` /
   ``policy_denied``) — a policy-limited row that is silently embedded fails the
   gate. The three promotion-target scope classes (selected_workset,
   sparse_slice, policy_limited_view) must each be covered.

The corpus is pure JSON, so this gate needs neither a Rust toolchain nor Ruby.
Exit code is non-zero on any error finding.
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

LABEL = "beta-support-workset-scope"

CORPUS_DIR_REL = "fixtures/support/workset_scope_export_beta"
MANIFEST_REL = f"{CORPUS_DIR_REL}/manifest.json"
RUST_TEST_REL = "crates/aureline-support/tests/workset_scope_export_beta.rs"
WORKSPACE_VOCAB_REL = "crates/aureline-workspace/src/worksets/mod.rs"
SUPPORT_VOCAB_REL = "crates/aureline-support/src/bundle/vocabulary.rs"

REQUIRED_SCOPE_CLASSES = {
    "selected_workset",
    "sparse_slice",
    "policy_limited_view",
}

ALLOWED_MEMBERSHIPS = {"in_scope", "out_of_scope", "policy_limited"}

ALLOWED_BROAD_ACTION_DECISIONS = {
    "allowed",
    "narrowed_to_scope",
    "blocked_by_policy",
    "blocked_by_portability",
    "blocked_by_sparse_partial",
    "blocked_by_outside_scope",
}

# The policy-locked redaction label every policy-limited content row must carry.
POLICY_REDACTION_STATE = "policy_locked"
POLICY_REVIEW_DECISION = "omitted_policy_locked"
POLICY_EXCLUDED_REASON = "policy_denied"


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


def extract_as_str_tokens(text: str, enum_name: str, source_rel: str) -> list[str]:
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


def extract_workspace_scope_classes(repo_root: Path) -> list[str]:
    text = read_text(repo_root / WORKSPACE_VOCAB_REL)
    return extract_as_str_tokens(text, "ScopeClass", WORKSPACE_VOCAB_REL)


def extract_support_redaction_vocab(repo_root: Path) -> dict[str, set[str]]:
    text = read_text(repo_root / SUPPORT_VOCAB_REL)
    return {
        "redaction_state": set(
            extract_as_str_tokens(text, "RedactionState", SUPPORT_VOCAB_REL)
        ),
        "review_decision_class": set(
            extract_as_str_tokens(text, "ReviewDecisionClass", SUPPORT_VOCAB_REL)
        ),
        "excluded_reason_class": set(
            extract_as_str_tokens(text, "ExcludedReasonClass", SUPPORT_VOCAB_REL)
        ),
    }


# --------------------------------------------------------------------------- #
# Validation
# --------------------------------------------------------------------------- #


def validate_vocabulary_map(
    manifest: dict[str, Any],
    workspace_tokens: list[str],
    findings: list[Finding],
) -> None:
    mapping = manifest.get("scope_class_vocabulary_map")
    support_only = manifest.get("support_only_scope_classes")
    mirror = manifest.get("workspace_scope_class_vocabulary")
    if not isinstance(mapping, dict) or not mapping:
        findings.append(
            Finding(
                "error",
                "manifest.map_missing",
                "manifest.scope_class_vocabulary_map must be a non-empty object",
                "Add the support->workspace scope-class mapping.",
                ref=MANIFEST_REL,
            )
        )
        return
    if not isinstance(support_only, list):
        findings.append(
            Finding(
                "error",
                "manifest.support_only_missing",
                "manifest.support_only_scope_classes must be a list",
                "List the support-only scope classes with no workspace counterpart.",
                ref=MANIFEST_REL,
            )
        )
        return
    if not isinstance(mirror, list):
        findings.append(
            Finding(
                "error",
                "manifest.mirror_missing",
                "manifest.workspace_scope_class_vocabulary must be a list",
                "Mirror the aureline-workspace ScopeClass vocabulary in the manifest.",
                ref=MANIFEST_REL,
            )
        )
        return

    workspace_set = set(workspace_tokens)

    if set(mirror) != workspace_set:
        findings.append(
            Finding(
                "error",
                "mirror.drift",
                "manifest.workspace_scope_class_vocabulary has drifted from "
                "aureline-workspace ScopeClass",
                "Refresh the manifest mirror to match the workspace crate.",
                ref=MANIFEST_REL,
                details={"mirror": sorted(mirror), "workspace": sorted(workspace_set)},
            )
        )

    for value in mapping.values():
        if value not in workspace_set:
            findings.append(
                Finding(
                    "error",
                    "map.unknown_workspace_class",
                    f"mapping value {value!r} is not a workspace ScopeClass token",
                    "Use a token from aureline-workspace ScopeClass.",
                    ref=MANIFEST_REL,
                )
            )

    overlap = set(mapping) & set(support_only)
    if overlap:
        findings.append(
            Finding(
                "error",
                "map.support_only_overlap",
                f"scope classes both mapped and declared support-only: {sorted(overlap)}",
                "A support class is either mapped to a workspace class or support-only.",
                ref=MANIFEST_REL,
            )
        )

    values = list(mapping.values())
    if len(values) != len(set(values)):
        findings.append(
            Finding(
                "error",
                "map.not_injective",
                "two support scope classes map to the same workspace scope class",
                "Make the scope-class mapping 1:1.",
                ref=MANIFEST_REL,
            )
        )

    if set(values) != workspace_set:
        findings.append(
            Finding(
                "error",
                "map.not_bijective",
                "mapped workspace classes do not cover the workspace ScopeClass "
                "vocabulary exactly",
                "Map exactly one support class to each workspace scope class.",
                ref=MANIFEST_REL,
                details={
                    "mapped": sorted(set(values)),
                    "workspace": sorted(workspace_set),
                },
            )
        )


def validate_redaction_vocabulary(
    manifest: dict[str, Any],
    support_vocab: dict[str, set[str]],
    findings: list[Finding],
) -> None:
    declared = manifest.get("redaction_vocabulary")
    if not isinstance(declared, dict) or not declared:
        findings.append(
            Finding(
                "error",
                "manifest.redaction_vocab_missing",
                "manifest.redaction_vocabulary must be a non-empty object",
                "Declare the policy-locked and metadata redaction tokens.",
                ref=MANIFEST_REL,
            )
        )
        return

    # field -> (support enum, required exact token)
    checks = (
        ("policy_limited_redaction_state", "redaction_state", POLICY_REDACTION_STATE),
        (
            "policy_limited_review_decision_class",
            "review_decision_class",
            POLICY_REVIEW_DECISION,
        ),
        (
            "policy_limited_excluded_reason_class",
            "excluded_reason_class",
            POLICY_EXCLUDED_REASON,
        ),
        ("in_scope_metadata_redaction_state", "redaction_state", None),
        ("in_scope_metadata_review_decision_class", "review_decision_class", None),
    )
    for field_name, enum_key, required in checks:
        token = declared.get(field_name)
        if token not in support_vocab[enum_key]:
            findings.append(
                Finding(
                    "error",
                    "redaction_vocab.unknown_token",
                    f"redaction_vocabulary.{field_name} {token!r} is not a real "
                    f"aureline-support {enum_key} token",
                    "Quote a token from aureline-support bundle::vocabulary.",
                    ref=MANIFEST_REL,
                    details={"valid": sorted(support_vocab[enum_key])},
                )
            )
        elif required is not None and token != required:
            findings.append(
                Finding(
                    "error",
                    "redaction_vocab.wrong_policy_token",
                    f"redaction_vocabulary.{field_name} must be {required!r}, got {token!r}",
                    "Use the policy-locked redaction tokens for policy-limited content.",
                    ref=MANIFEST_REL,
                )
            )


def validate_case(
    repo_root: Path,
    case_file: str,
    mapping: dict[str, Any],
    support_vocab: dict[str, set[str]],
    findings: list[Finding],
) -> str | None:
    """Validate one case file; return its workspace_scope_class on success."""
    path = repo_root / CORPUS_DIR_REL / case_file
    case = load_json(path)
    ref = f"{CORPUS_DIR_REL}/{case_file}"

    def fail(check_id: str, message: str) -> None:
        findings.append(
            Finding(
                "error",
                check_id,
                f"{case_file}: {message}",
                "Re-align the case so export scope truth stays declared and labeled.",
                ref=ref,
            )
        )

    if case.get("record_kind") != "support_workset_scope_export_beta_case":
        fail("case.record_kind", "record_kind must be support_workset_scope_export_beta_case")
    if case.get("schema_version") != 1:
        fail("case.schema_version", "schema_version must be the integer 1")

    support_class = case.get("support_scope_class")
    workspace_class = case.get("workspace_scope_class")
    if support_class not in mapping:
        fail(
            "case.unmapped_support_class",
            f"support_scope_class {support_class!r} is not in the vocabulary map",
        )
    elif mapping[support_class] != workspace_class:
        fail(
            "case.workspace_class_divergence",
            f"workspace_scope_class {workspace_class!r} disagrees with the map "
            f"({mapping[support_class]!r})",
        )

    artifact = case.get("artifact") or {}
    if artifact.get("scope_class") != workspace_class:
        fail(
            "case.artifact_scope_divergence",
            f"artifact.scope_class {artifact.get('scope_class')!r} disagrees with the "
            f"declared workspace_scope_class {workspace_class!r}",
        )

    expect = case.get("expect") or {}
    if not expect.get("scope_declaration_present"):
        fail(
            "case.no_scope_declaration",
            "expect.scope_declaration_present must be true; a support/audit export "
            "must declare the scope it was produced under",
        )

    rows = case.get("export_rows")
    if not isinstance(rows, list) or not rows:
        fail("case.export_rows", "export_rows must be a non-empty list")
        rows = []

    in_scope = 0
    out_of_scope = 0
    policy_limited = 0
    for row in rows:
        membership = row.get("membership")
        row_id = row.get("row_id")
        if membership not in ALLOWED_MEMBERSHIPS:
            fail(
                "case.membership_unknown",
                f"export row {row_id!r} has unknown membership {membership!r}",
            )
            continue
        if not isinstance(row_id, str) or not row_id.strip():
            fail("case.row_identity", "every export row needs a row_id")

        # Every declared redaction token must be a real support vocabulary token.
        if row.get("redaction_state") not in support_vocab["redaction_state"]:
            fail(
                "case.redaction_state_unknown",
                f"row {row_id!r} redaction_state {row.get('redaction_state')!r} is not a "
                "real RedactionState token",
            )
        if row.get("review_decision_class") not in support_vocab["review_decision_class"]:
            fail(
                "case.review_decision_unknown",
                f"row {row_id!r} review_decision_class {row.get('review_decision_class')!r} "
                "is not a real ReviewDecisionClass token",
            )
        excluded_reason = row.get("excluded_reason_class")
        if excluded_reason is not None and (
            excluded_reason not in support_vocab["excluded_reason_class"]
        ):
            fail(
                "case.excluded_reason_unknown",
                f"row {row_id!r} excluded_reason_class {excluded_reason!r} is not a real "
                "ExcludedReasonClass token",
            )

        if membership == "in_scope":
            in_scope += 1
        elif membership == "out_of_scope":
            out_of_scope += 1
        else:
            policy_limited += 1
            # Policy-limited content must carry the full policy-locked label;
            # an embedded policy-limited row without the label is a silent leak.
            if (
                row.get("redaction_state") != POLICY_REDACTION_STATE
                or row.get("review_decision_class") != POLICY_REVIEW_DECISION
                or excluded_reason != POLICY_EXCLUDED_REASON
            ):
                fail(
                    "case.policy_limited_unlabeled",
                    f"policy-limited row {row_id!r} must carry the policy redaction triple "
                    f"({POLICY_REDACTION_STATE} / {POLICY_REVIEW_DECISION} / "
                    f"{POLICY_EXCLUDED_REASON}); it must never be silently embedded",
                )

    for key, actual in (
        ("in_scope_count", in_scope),
        ("out_of_scope_count", out_of_scope),
        ("policy_limited_count", policy_limited),
    ):
        declared = expect.get(key)
        if declared != actual:
            fail(
                "case.count_mismatch",
                f"expect.{key} ({declared}) must equal the membership tally ({actual})",
            )

    # Each case must prove a real narrowed boundary, not a full-workspace export
    # masquerading as a scoped one.
    has_excluded_root = bool(expect.get("excluded_root_refs"))
    has_exclude_pattern = any(
        p.get("pattern_kind") == "exclude" for p in (artifact.get("patterns") or [])
    )
    if not (has_excluded_root or policy_limited >= 1 or has_exclude_pattern):
        fail(
            "case.no_boundary",
            "a case must prove a real narrowed boundary: an excluded root, a "
            "policy-limited row, or an exclude pattern",
        )

    for key in ("export_artifact_decision", "support_archive_decision"):
        decision = expect.get(key)
        if decision not in ALLOWED_BROAD_ACTION_DECISIONS:
            fail(
                "case.broad_action_decision",
                f"expect.{key} {decision!r} is not a known broad-action decision",
            )

    if workspace_class == "policy_limited_view":
        if policy_limited < 1:
            fail(
                "case.policy_view_no_hidden",
                "the policy_limited_view case must include at least one policy-limited row",
            )
        if not expect.get("policy_hidden_disclosed"):
            fail(
                "case.policy_view_no_disclosure",
                "the policy_limited_view case must disclose policy-hidden members "
                "(expect.policy_hidden_disclosed must be true)",
            )
        if expect.get("export_artifact_decision") == "allowed":
            fail(
                "case.policy_view_export_allowed",
                "the policy_limited_view case must block or narrow export_artifact, "
                "not allow it outright",
            )

    return workspace_class if isinstance(workspace_class, str) else None


def validate_corpus(repo_root: Path, findings: list[Finding]) -> None:
    manifest = load_json(repo_root / MANIFEST_REL)
    if manifest.get("record_kind") != "support_workset_scope_export_beta_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "manifest.record_kind must be support_workset_scope_export_beta_manifest",
                "Fix the manifest record_kind.",
                ref=MANIFEST_REL,
            )
        )
    if manifest.get("schema_version") != 1:
        findings.append(
            Finding(
                "error",
                "manifest.schema_version",
                "manifest.schema_version must be the integer 1",
                "Bump the gate together with the manifest if its shape changes.",
                ref=MANIFEST_REL,
            )
        )

    workspace_tokens = extract_workspace_scope_classes(repo_root)
    validate_vocabulary_map(manifest, workspace_tokens, findings)

    support_vocab = extract_support_redaction_vocab(repo_root)
    validate_redaction_vocabulary(manifest, support_vocab, findings)

    mapping = manifest.get("scope_class_vocabulary_map") or {}
    cases = manifest.get("cases")
    if not isinstance(cases, list) or not cases:
        findings.append(
            Finding(
                "error",
                "manifest.cases_missing",
                "manifest.cases must list at least one case file",
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
        workspace_class = validate_case(
            repo_root, case_file, mapping, support_vocab, findings
        )
        if workspace_class:
            covered.add(workspace_class)

    missing = sorted(REQUIRED_SCOPE_CLASSES - covered)
    if missing:
        findings.append(
            Finding(
                "error",
                "coverage.required_scope_class_missing",
                f"corpus is missing cases for required scope classes: {', '.join(missing)}",
                "Add a case for each of selected_workset, sparse_slice, policy_limited_view.",
                ref=MANIFEST_REL,
                details={"missing": missing},
            )
        )

    declared_required = set(manifest.get("required_scope_classes") or [])
    if declared_required != REQUIRED_SCOPE_CLASSES:
        findings.append(
            Finding(
                "error",
                "manifest.required_list_mismatch",
                "manifest.required_scope_classes must equal "
                f"{sorted(REQUIRED_SCOPE_CLASSES)}",
                "Align the manifest required list with the promotion target.",
                ref=MANIFEST_REL,
                details={"declared": sorted(declared_required)},
            )
        )

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
        "check_id": "beta_support_workset_scope",
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
