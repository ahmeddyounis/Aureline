#!/usr/bin/env python3
"""Gate the graph workset / scope beta coverage corpus.

The graph surface promotes named-workset, sparse-slice, and policy-limited-view
scope truth to beta by replaying the frozen corpus at
``fixtures/graph/workset_scope_beta/`` through the impact-explainer packet
builder (``crates/aureline-graph/tests/workset_scope_beta.rs``). The Rust test
proves the *runtime* behaviour — in-scope results stay visible, out-of-scope
results are labeled, policy-hidden members are disclosed. This gate proves the
two *data* invariants a passing test alone cannot defend against quiet drift:

1. **One shared scope vocabulary.** ``manifest.json``'s
   ``scope_class_vocabulary_map`` must be a 1:1 bijection between the graph
   ``WorksetScopeClass`` vocabulary and the ``aureline-workspace``
   ``ScopeClass`` vocabulary. The gate re-derives *both* vocabularies from crate
   source (``aureline-graph-proto`` and ``aureline-workspace``) and fails closed
   if the map, the graph-only extension list, or the Rust test's mirrored
   workspace vocabulary drift from them. ``aureline-graph`` deliberately does
   not depend on ``aureline-workspace``; this gate is the seam that keeps the
   two crates from forking the scope vocabulary.

2. **Honest scope labeling in the corpus.** Every case must keep its declared
   scope-class labels aligned with the shared map, count at least one
   out-of-scope result (labeled, never silently dropped), and — for the
   policy-limited-view case — disclose hidden policy members through the scope
   descriptor (``descriptor_hidden_result_count == out_of_scope_count +
   policy.hidden_member_count``). The three promotion-target scope classes
   (named_workset, sparse_slice, policy_limited_view) must each be covered.

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

LABEL = "beta-workset-scope-coverage"

CORPUS_DIR_REL = "fixtures/graph/workset_scope_beta"
MANIFEST_REL = f"{CORPUS_DIR_REL}/manifest.json"
RUST_TEST_REL = "crates/aureline-graph/tests/workset_scope_beta.rs"
GRAPH_VOCAB_REL = "crates/aureline-graph-proto/src/vocab.rs"
WORKSPACE_VOCAB_REL = "crates/aureline-workspace/src/worksets/mod.rs"

REQUIRED_GRAPH_SCOPE_CLASSES = {
    "named_workset",
    "sparse_slice",
    "policy_limited_view",
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


def extract_graph_scope_classes(repo_root: Path) -> list[str]:
    """Parse the graph WorksetScopeClass tokens from the proto crate's vocab."""
    text = read_text(repo_root / GRAPH_VOCAB_REL)
    match = re.search(r"pub enum WorksetScopeClass\s*\{(.*?)\}", text, re.DOTALL)
    if not match:
        raise SystemExit(
            f"could not locate `pub enum WorksetScopeClass` in {GRAPH_VOCAB_REL}"
        )
    tokens = re.findall(r"=>\s*\"([^\"]+)\"", match.group(1))
    if not tokens:
        raise SystemExit(f"no WorksetScopeClass tokens parsed from {GRAPH_VOCAB_REL}")
    return tokens


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


def extract_rust_test_workspace_mirror(repo_root: Path) -> list[str]:
    """Parse the WORKSPACE_SCOPE_CLASS_VOCABULARY mirror from the Rust test."""
    text = read_text(repo_root / RUST_TEST_REL)
    match = re.search(
        r"WORKSPACE_SCOPE_CLASS_VOCABULARY:\s*\[&str;\s*\d+\]\s*=\s*\[(.*?)\]",
        text,
        re.DOTALL,
    )
    if not match:
        raise SystemExit(
            f"could not locate WORKSPACE_SCOPE_CLASS_VOCABULARY in {RUST_TEST_REL}"
        )
    return re.findall(r"\"([^\"]+)\"", match.group(1))


# --------------------------------------------------------------------------- #
# Validation
# --------------------------------------------------------------------------- #


def validate_vocabulary_map(
    manifest: dict[str, Any],
    graph_tokens: list[str],
    workspace_tokens: list[str],
    rust_mirror: list[str],
    findings: list[Finding],
) -> None:
    mapping = manifest.get("scope_class_vocabulary_map")
    graph_only = manifest.get("graph_only_scope_classes")
    if not isinstance(mapping, dict) or not mapping:
        findings.append(
            Finding(
                "error",
                "manifest.map_missing",
                "manifest.scope_class_vocabulary_map must be a non-empty object",
                "Add the graph->workspace scope-class mapping.",
                ref=MANIFEST_REL,
            )
        )
        return
    if not isinstance(graph_only, list):
        findings.append(
            Finding(
                "error",
                "manifest.graph_only_missing",
                "manifest.graph_only_scope_classes must be a list",
                "List the graph-only scope classes with no workspace counterpart.",
                ref=MANIFEST_REL,
            )
        )
        return

    graph_set = set(graph_tokens)
    workspace_set = set(workspace_tokens)

    # The Rust test mirror of the workspace vocabulary must match crate source.
    if set(rust_mirror) != workspace_set:
        findings.append(
            Finding(
                "error",
                "mirror.drift",
                "the Rust test's WORKSPACE_SCOPE_CLASS_VOCABULARY mirror has drifted "
                "from aureline-workspace ScopeClass",
                "Refresh the mirror in the Rust test to match the workspace crate.",
                ref=RUST_TEST_REL,
                details={"mirror": sorted(rust_mirror), "workspace": sorted(workspace_set)},
            )
        )

    # Every map key is a real graph token; every value is a real workspace token.
    for key in mapping:
        if key not in graph_set:
            findings.append(
                Finding(
                    "error",
                    "map.unknown_graph_class",
                    f"mapping key {key!r} is not a graph WorksetScopeClass token",
                    "Use a token from aureline-graph-proto WorksetScopeClass.",
                    ref=MANIFEST_REL,
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

    # Map keys and graph-only classes must together account for the whole graph
    # vocabulary, and be disjoint.
    overlap = set(mapping) & set(graph_only)
    if overlap:
        findings.append(
            Finding(
                "error",
                "map.graph_only_overlap",
                f"scope classes both mapped and declared graph-only: {sorted(overlap)}",
                "A graph class is either mapped to a workspace class or graph-only.",
                ref=MANIFEST_REL,
            )
        )
    accounted = set(mapping) | set(graph_only)
    if accounted != graph_set:
        findings.append(
            Finding(
                "error",
                "map.incomplete_graph_coverage",
                "mapping + graph_only must account for every graph WorksetScopeClass",
                "Map or declare graph-only every graph scope class.",
                ref=MANIFEST_REL,
                details={
                    "unaccounted": sorted(graph_set - accounted),
                    "extra": sorted(accounted - graph_set),
                },
            )
        )

    # Injective: no two graph classes collapse onto one workspace class.
    values = list(mapping.values())
    if len(values) != len(set(values)):
        findings.append(
            Finding(
                "error",
                "map.not_injective",
                "two graph scope classes map to the same workspace scope class",
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
                "Map exactly one graph class to each workspace scope class.",
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
    graph_tokens: set[str],
    findings: list[Finding],
) -> str | None:
    """Validate one case file; return its graph_scope_class on success."""
    path = repo_root / CORPUS_DIR_REL / case_file
    case = load_json(path)
    ref = f"{CORPUS_DIR_REL}/{case_file}"

    def fail(check_id: str, message: str) -> None:
        findings.append(
            Finding(
                "error",
                check_id,
                f"{case_file}: {message}",
                "Re-align the case so graph scope truth stays labeled and shared.",
                ref=ref,
            )
        )

    if case.get("record_kind") != "graph_workset_scope_beta_case":
        fail("case.record_kind", "record_kind must be graph_workset_scope_beta_case")
    if case.get("schema_version") != 1:
        fail("case.schema_version", "schema_version must be the integer 1")

    graph_class = case.get("graph_scope_class")
    workspace_class = case.get("workspace_scope_class")
    if graph_class not in graph_tokens:
        fail("case.unknown_graph_class", f"graph_scope_class {graph_class!r} is unknown")
        return None
    if graph_class not in mapping:
        fail(
            "case.unmapped_graph_class",
            f"graph_scope_class {graph_class!r} is not in the vocabulary map",
        )
    elif mapping[graph_class] != workspace_class:
        fail(
            "case.workspace_class_divergence",
            f"workspace_scope_class {workspace_class!r} disagrees with the map "
            f"({mapping[graph_class]!r})",
        )

    request = case.get("request") or {}
    for key in ("query_request_id", "subject_node_id", "in_scope_id", "out_of_scope_id"):
        if not isinstance(request.get(key), str) or not request[key].strip():
            fail("case.request_field", f"request.{key} must be a non-empty string")
    if request.get("in_scope_id") == request.get("out_of_scope_id"):
        fail(
            "case.scope_collision",
            "in_scope_id and out_of_scope_id must differ so the corpus tests a "
            "real boundary",
        )

    policy = case.get("policy") or {}
    hidden_members = policy.get("hidden_member_count")
    if not isinstance(hidden_members, int) or hidden_members < 0:
        fail("case.policy_hidden", "policy.hidden_member_count must be a non-negative integer")
        hidden_members = 0

    expect = case.get("expect") or {}
    out_of_scope = expect.get("out_of_scope_count")
    if not isinstance(out_of_scope, int) or out_of_scope < 1:
        fail(
            "case.out_of_scope_not_labeled",
            "expect.out_of_scope_count must be >= 1 (an out-of-scope result must be "
            "labeled, not silently dropped)",
        )
        out_of_scope = 0
    if not expect.get("out_of_scope_edge_ids"):
        fail("case.out_of_scope_ids", "expect.out_of_scope_edge_ids must list the labeled edges")
    if not expect.get("visible_impact_edge_ids"):
        fail("case.visible_ids", "expect.visible_impact_edge_ids must list the in-scope edges")

    policy_hidden = expect.get("policy_hidden_result_count")
    descriptor_hidden = expect.get("descriptor_hidden_result_count")
    if policy_hidden != hidden_members:
        fail(
            "case.policy_hidden_mismatch",
            f"expect.policy_hidden_result_count ({policy_hidden}) must equal "
            f"policy.hidden_member_count ({hidden_members})",
        )
    if isinstance(descriptor_hidden, int) and isinstance(out_of_scope, int):
        if descriptor_hidden != out_of_scope + hidden_members:
            fail(
                "case.descriptor_disclosure",
                f"expect.descriptor_hidden_result_count ({descriptor_hidden}) must equal "
                f"out_of_scope_count + policy.hidden_member_count "
                f"({out_of_scope + hidden_members})",
            )
    else:
        fail("case.descriptor_hidden", "expect.descriptor_hidden_result_count must be an integer")

    # The policy-limited-view case must genuinely hide members so the
    # policy-limited disclosure path is exercised.
    if graph_class == "policy_limited_view" and hidden_members < 1:
        fail(
            "case.policy_view_no_hidden",
            "the policy_limited_view case must hide at least one policy member",
        )

    return graph_class if graph_class in graph_tokens else None


def validate_corpus(repo_root: Path, findings: list[Finding]) -> None:
    manifest = load_json(repo_root / MANIFEST_REL)
    if manifest.get("record_kind") != "graph_workset_scope_beta_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "manifest.record_kind must be graph_workset_scope_beta_manifest",
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

    graph_tokens = extract_graph_scope_classes(repo_root)
    workspace_tokens = extract_workspace_scope_classes(repo_root)
    rust_mirror = extract_rust_test_workspace_mirror(repo_root)

    validate_vocabulary_map(
        manifest, graph_tokens, workspace_tokens, rust_mirror, findings
    )

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
        graph_class = validate_case(
            repo_root, case_file, mapping, set(graph_tokens), findings
        )
        if graph_class:
            covered.add(graph_class)

    missing = sorted(REQUIRED_GRAPH_SCOPE_CLASSES - covered)
    if missing:
        findings.append(
            Finding(
                "error",
                "coverage.required_scope_class_missing",
                f"corpus is missing cases for required scope classes: {', '.join(missing)}",
                "Add a case for each of named_workset, sparse_slice, policy_limited_view.",
                ref=MANIFEST_REL,
                details={"missing": missing},
            )
        )

    # Cross-check the manifest's declared required list against the gate's.
    declared_required = set(manifest.get("required_graph_scope_classes") or [])
    if declared_required != REQUIRED_GRAPH_SCOPE_CLASSES:
        findings.append(
            Finding(
                "error",
                "manifest.required_list_mismatch",
                "manifest.required_graph_scope_classes must equal "
                f"{sorted(REQUIRED_GRAPH_SCOPE_CLASSES)}",
                "Align the manifest required list with the promotion target.",
                ref=MANIFEST_REL,
                details={"declared": sorted(declared_required)},
            )
        )


def write_report(repo_root: Path, report_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "beta_workset_scope_coverage",
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
