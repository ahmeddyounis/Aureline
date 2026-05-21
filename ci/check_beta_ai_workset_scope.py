#!/usr/bin/env python3
"""Gate the AI composer / context-inspector workset / scope beta corpus.

The AI surface promotes named-workset, sparse-slice, and policy-limited-view
scope truth to beta by replaying the frozen corpus at
``fixtures/ai/workset_scope_beta/`` through the AI composer / context-inspector
projection (``crates/aureline-ai/tests/workset_scope_beta.rs``). The Rust test
proves the *runtime* behaviour — in-scope context stays drawn, out-of-scope and
policy-limited context is labeled, and the same scope truth flows into the
evidence handoff and spend receipt. This gate proves the two *data* invariants a
passing test alone cannot defend against quiet drift:

1. **One shared scope vocabulary.** ``manifest.json``'s
   ``scope_class_vocabulary_map`` must be a 1:1 bijection between the AI
   scope-class tokens and the ``aureline-workspace`` ``ScopeClass`` vocabulary.
   The gate re-derives the workspace vocabulary from crate source
   (``aureline-workspace``) and fails closed if the map, the corpus mirror
   (``workspace_scope_class_vocabulary``), or the required scope-class coverage
   drift from it. ``aureline-ai`` does not depend on ``aureline-workspace`` in
   production; this gate is the seam that keeps the AI and workspace surfaces
   from forking the scope vocabulary.

2. **Honest scope labeling in the corpus.** Every case must keep its declared
   scope-class labels aligned with the shared map, count at least one escaping
   (out-of-scope or policy-limited) context row that is *labeled*, keep the
   declared membership tallies aligned with the ``expect`` counts, and name the
   labeled escaping rows as evidence survivors. The three promotion-target scope
   classes (selected_workset, sparse_slice, policy_limited_view) must each be
   covered.

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

LABEL = "beta-ai-workset-scope"

CORPUS_DIR_REL = "fixtures/ai/workset_scope_beta"
MANIFEST_REL = f"{CORPUS_DIR_REL}/manifest.json"
RUST_TEST_REL = "crates/aureline-ai/tests/workset_scope_beta.rs"
WORKSPACE_VOCAB_REL = "crates/aureline-workspace/src/worksets/mod.rs"

REQUIRED_SCOPE_CLASSES = {
    "selected_workset",
    "sparse_slice",
    "policy_limited_view",
}

ALLOWED_MEMBERSHIPS = {"in_scope", "out_of_scope", "policy_limited"}

ALLOWED_AI_APPLY_DECISIONS = {
    "allowed",
    "narrowed_to_scope",
    "blocked_by_policy",
    "blocked_by_portability",
    "blocked_by_sparse_partial",
    "blocked_by_outside_scope",
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


def extract_workspace_scope_classes(repo_root: Path) -> list[str]:
    """Parse the workspace ScopeClass::as_str tokens from the workspace crate."""
    text = read_text(repo_root / WORKSPACE_VOCAB_REL)
    impl_match = re.search(r"impl ScopeClass \{(.*?)\n\}", text, re.DOTALL)
    if not impl_match:
        raise SystemExit(f"could not locate `impl ScopeClass` in {WORKSPACE_VOCAB_REL}")
    as_str_match = re.search(
        r"fn as_str\(self\) -> &'static str \{(.*?)\n    \}",
        impl_match.group(1),
        re.DOTALL,
    )
    if not as_str_match:
        raise SystemExit(
            f"could not locate `ScopeClass::as_str` in {WORKSPACE_VOCAB_REL}"
        )
    tokens = re.findall(r"Self::\w+\s*=>\s*\"([^\"]+)\"", as_str_match.group(1))
    if not tokens:
        raise SystemExit(f"no ScopeClass tokens parsed from {WORKSPACE_VOCAB_REL}")
    return tokens


# --------------------------------------------------------------------------- #
# Validation
# --------------------------------------------------------------------------- #


def validate_vocabulary_map(
    manifest: dict[str, Any],
    workspace_tokens: list[str],
    findings: list[Finding],
) -> None:
    mapping = manifest.get("scope_class_vocabulary_map")
    ai_only = manifest.get("ai_only_scope_classes")
    mirror = manifest.get("workspace_scope_class_vocabulary")
    if not isinstance(mapping, dict) or not mapping:
        findings.append(
            Finding(
                "error",
                "manifest.map_missing",
                "manifest.scope_class_vocabulary_map must be a non-empty object",
                "Add the ai->workspace scope-class mapping.",
                ref=MANIFEST_REL,
            )
        )
        return
    if not isinstance(ai_only, list):
        findings.append(
            Finding(
                "error",
                "manifest.ai_only_missing",
                "manifest.ai_only_scope_classes must be a list",
                "List the AI-only scope classes with no workspace counterpart.",
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

    # The manifest mirror of the workspace vocabulary must match crate source.
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

    # Every map value is a real workspace token.
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

    # Map keys and AI-only classes must be disjoint.
    overlap = set(mapping) & set(ai_only)
    if overlap:
        findings.append(
            Finding(
                "error",
                "map.ai_only_overlap",
                f"scope classes both mapped and declared AI-only: {sorted(overlap)}",
                "An AI class is either mapped to a workspace class or AI-only.",
                ref=MANIFEST_REL,
            )
        )

    # Injective: no two AI classes collapse onto one workspace class.
    values = list(mapping.values())
    if len(values) != len(set(values)):
        findings.append(
            Finding(
                "error",
                "map.not_injective",
                "two AI scope classes map to the same workspace scope class",
                "Make the scope-class mapping 1:1.",
                ref=MANIFEST_REL,
            )
        )

    # Surjective onto the workspace vocabulary: every workspace class is covered.
    if set(values) != workspace_set:
        findings.append(
            Finding(
                "error",
                "map.not_bijective",
                "mapped workspace classes do not cover the workspace ScopeClass "
                "vocabulary exactly",
                "Map exactly one AI class to each workspace scope class.",
                ref=MANIFEST_REL,
                details={
                    "mapped": sorted(set(values)),
                    "workspace": sorted(workspace_set),
                },
            )
        )


def validate_case(
    repo_root: Path,
    case_file: str,
    mapping: dict[str, Any],
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
                "Re-align the case so AI scope truth stays labeled and shared.",
                ref=ref,
            )
        )

    if case.get("record_kind") != "ai_workset_scope_beta_case":
        fail("case.record_kind", "record_kind must be ai_workset_scope_beta_case")
    if case.get("schema_version") != 1:
        fail("case.schema_version", "schema_version must be the integer 1")

    ai_class = case.get("ai_scope_class")
    workspace_class = case.get("workspace_scope_class")
    if ai_class not in mapping:
        fail(
            "case.unmapped_ai_class",
            f"ai_scope_class {ai_class!r} is not in the vocabulary map",
        )
    elif mapping[ai_class] != workspace_class:
        fail(
            "case.workspace_class_divergence",
            f"workspace_scope_class {workspace_class!r} disagrees with the map "
            f"({mapping[ai_class]!r})",
        )

    artifact = case.get("artifact") or {}
    if artifact.get("scope_class") != workspace_class:
        fail(
            "case.artifact_scope_divergence",
            f"artifact.scope_class {artifact.get('scope_class')!r} disagrees with the "
            f"declared workspace_scope_class {workspace_class!r}",
        )

    items = case.get("context_items")
    if not isinstance(items, list) or not items:
        fail("case.context_items", "context_items must be a non-empty list")
        items = []

    in_scope = 0
    out_of_scope = 0
    policy_limited = 0
    escaping_ids: list[str] = []
    for item in items:
        membership = item.get("membership")
        item_id = item.get("context_item_id")
        if membership not in ALLOWED_MEMBERSHIPS:
            fail(
                "case.membership_unknown",
                f"context item {item_id!r} has unknown membership {membership!r}",
            )
            continue
        if not isinstance(item_id, str) or not item_id.strip():
            fail("case.item_identity", "every context item needs a context_item_id")
        if membership == "in_scope":
            in_scope += 1
        elif membership == "out_of_scope":
            out_of_scope += 1
            escaping_ids.append(item_id)
        else:
            policy_limited += 1
            escaping_ids.append(item_id)

    if out_of_scope + policy_limited < 1:
        fail(
            "case.no_escaping_row",
            "a case must include at least one out-of-scope or policy-limited row "
            "(an escaping result must be labeled, not silently absent)",
        )

    expect = case.get("expect") or {}
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

    survivors = expect.get("evidence_survivor_ids")
    if not isinstance(survivors, list) or not survivors:
        fail(
            "case.no_evidence_survivors",
            "expect.evidence_survivor_ids must name the labeled rows that survive "
            "into the evidence handoff",
        )
    else:
        for survivor in survivors:
            if survivor not in escaping_ids:
                fail(
                    "case.survivor_not_escaping",
                    f"evidence survivor {survivor!r} is not a labeled escaping row",
                )

    decision = expect.get("ai_apply_decision")
    if decision not in ALLOWED_AI_APPLY_DECISIONS:
        fail(
            "case.ai_apply_decision",
            f"expect.ai_apply_decision {decision!r} is not a known broad-action decision",
        )

    # The policy-limited-view case must genuinely carry a policy-limited row and
    # block (not narrow) ai_apply so the policy disclosure path is exercised.
    if workspace_class == "policy_limited_view":
        if policy_limited < 1:
            fail(
                "case.policy_view_no_hidden",
                "the policy_limited_view case must include at least one policy-limited row",
            )
        if not expect.get("policy_hidden_excluded"):
            fail(
                "case.policy_view_no_disclosure",
                "the policy_limited_view case must disclose policy-hidden members "
                "(expect.policy_hidden_excluded must be true)",
            )

    return workspace_class if isinstance(workspace_class, str) else None


def validate_corpus(repo_root: Path, findings: list[Finding]) -> None:
    manifest = load_json(repo_root / MANIFEST_REL)
    if manifest.get("record_kind") != "ai_workset_scope_beta_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "manifest.record_kind must be ai_workset_scope_beta_manifest",
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
        workspace_class = validate_case(repo_root, case_file, mapping, findings)
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

    # Cross-check the manifest's declared required list against the gate's.
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

    # The Rust drill must consume this corpus.
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
        "check_id": "beta_ai_workset_scope",
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
