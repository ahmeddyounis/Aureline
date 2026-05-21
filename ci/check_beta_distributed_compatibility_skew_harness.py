#!/usr/bin/env python3
"""Validate the beta distributed-compatibility skew-test harness.

The generated distributed-compatibility manifests already decompose every
boundary row into supported / best-effort / untested / unsupported skew cases,
and the support-export projection quotes the current and unsupported cases per
row. This gate drives those manifests across the four named families
(client/helper, client/extension, schema, provider) through a single skew-test
harness that resolves every enumerated combination to one closed verdict --
``compatible`` / ``probe_required`` / ``incompatible`` -- and proves each
pairing carries a repair or safe-continuation guidance path.

The gate:

  - regenerates the frozen verdict corpus from the family manifests and the
    support-export projection (``--check`` fails if the checked-in corpus is
    stale);
  - asserts the closed status->verdict map is total, so an unhandled skew
    combination is a hard failure rather than a silent pass;
  - cross-checks the harness against the support-export projection so the
    current window resolves to ``compatible`` and every unsupported case
    resolves to ``incompatible`` with the same repair hints the support packet
    quotes; and
  - runs negative drills proving an unhandled status, a missing guidance path,
    and a dropped support-export case are all rejected.

The typed Rust consumer
(``aureline_support::distributed_compatibility::DistributedSkewHarnessCorpus``)
reads the same frozen corpus, so this gate and
``cargo test -p aureline-support --test distributed_compatibility_skew_harness``
agree on every verdict without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import re
import sys
from pathlib import Path
from typing import Any


DEFAULT_MANIFEST_DIR_REL = "artifacts/compat/m3/distributed_manifests"
DEFAULT_SUPPORT_EXPORT_REL = (
    "artifacts/release/m3/distributed_compatibility/support_export_projection.json"
)
DEFAULT_CORPUS_REL = (
    "artifacts/release/m3/distributed_compatibility/skew_harness_verdict_corpus.json"
)
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/distributed_compatibility_skew_harness_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
CORPUS_RECORD_KIND = "distributed_compatibility_skew_harness_corpus"
CORPUS_ID = "skew_harness_corpus:distributed_compatibility.beta"

REQUIRED_FAMILIES = ("client_helper", "client_extension", "schema", "provider")
STATUS_VOCABULARY = ("supported", "best_effort", "untested", "unsupported")
VERDICT_VOCABULARY = ("compatible", "probe_required", "incompatible")

# The closed, total status->verdict map. A status missing from this map is an
# unhandled skew combination and a hard failure.
STATUS_TO_VERDICT = {
    "supported": "compatible",
    "best_effort": "probe_required",
    "untested": "probe_required",
    "unsupported": "incompatible",
}

# Guidance path for a fully compatible pairing: nothing to repair, continue.
NO_REPAIR_REQUIRED_GUIDANCE = "no_repair_required"


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--manifest-dir", default=DEFAULT_MANIFEST_DIR_REL)
    parser.add_argument("--support-export", default=DEFAULT_SUPPORT_EXPORT_REL)
    parser.add_argument("--corpus", default=DEFAULT_CORPUS_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Optional path for a JSON validation capture.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the checked-in verdict corpus would change instead of rewriting it.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value


def slug_from_ref(ref: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", ref.split(":", 1)[-1].lower()).strip("_")


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


_GENERATED_AT_RE = re.compile(r'"generated_at":\s*"[^"]*"')


def normalize_generated_at(text: str) -> str:
    return _GENERATED_AT_RE.sub('"generated_at": "__generated_at__"', text)


def manifest_ref_for(manifest_dir_rel: str, family: str) -> str:
    return f"{manifest_dir_rel.rstrip('/')}/{family}.json"


def support_refs(support_export: dict[str, Any]) -> dict[str, dict[str, Any]]:
    """Maps each skew case the support export quotes to its row context."""
    refs: dict[str, dict[str, Any]] = {}
    for row in ensure_list(support_export.get("support_rows"), "support_export.support_rows"):
        item = ensure_dict(row, "support_export.support_rows[]")
        family = ensure_str(item.get("manifest_family"), "support_row.manifest_family")
        compat_ref = ensure_str(
            item.get("compatibility_row_ref"), "support_row.compatibility_row_ref"
        )
        repair_hints = list(ensure_list(item.get("repair_hints"), "support_row.repair_hints"))
        current = ensure_str(
            item.get("current_skew_case_ref"), "support_row.current_skew_case_ref"
        )
        refs[current] = {
            "role": "current",
            "manifest_family": family,
            "compatibility_row_ref": compat_ref,
            "repair_hints": repair_hints,
        }
        for ref in ensure_list(
            item.get("unsupported_case_refs"), "support_row.unsupported_case_refs"
        ):
            refs[ensure_str(ref, "support_row.unsupported_case_refs[]")] = {
                "role": "unsupported",
                "manifest_family": family,
                "compatibility_row_ref": compat_ref,
                "repair_hints": repair_hints,
            }
    return refs


def build_cases(
    *,
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    quoted_refs: dict[str, dict[str, Any]],
) -> list[dict[str, Any]]:
    cases: list[dict[str, Any]] = []
    for family in REQUIRED_FAMILIES:
        manifest = manifests[family]
        manifest_ref = manifest_refs[family]
        for row in ensure_list(manifest.get("rows"), f"{family}.rows"):
            item = ensure_dict(row, f"{family}.rows[]")
            compat_ref = ensure_str(
                item.get("compatibility_row_ref"), f"{family}.compatibility_row_ref"
            )
            manifest_row_id = ensure_str(
                item.get("manifest_row_id"), f"{family}.manifest_row_id"
            )
            repair_hints = list(
                ensure_list(item.get("repair_hints"), f"{family}.repair_hints")
            )
            skew_window = ensure_dict(item.get("skew_window"), f"{family}.skew_window")
            current_ref = ensure_str(
                skew_window.get("current_skew_case_ref"),
                f"{family}.current_skew_case_ref",
            )
            skew_cases = ensure_dict(item.get("skew_cases"), f"{family}.skew_cases")
            for status in STATUS_VOCABULARY:
                for raw_case in ensure_list(
                    skew_cases.get(status, []), f"{family}.skew_cases.{status}"
                ):
                    case = ensure_dict(raw_case, f"{family}.skew_cases.{status}[]")
                    skew_case_ref = ensure_str(
                        case.get("skew_case_ref"), f"{family}.{status}.skew_case_ref"
                    )
                    verdict = STATUS_TO_VERDICT[status]
                    is_current = skew_case_ref == current_ref
                    in_support_export = skew_case_ref in quoted_refs
                    guidance = (
                        [NO_REPAIR_REQUIRED_GUIDANCE]
                        if verdict == "compatible"
                        else repair_hints
                    )
                    cases.append(
                        {
                            "case_id": (
                                "skew_harness_case:distributed_compatibility."
                                f"{family}.{slug_from_ref(skew_case_ref)}"
                            ),
                            "manifest_family": family,
                            "manifest_ref": manifest_ref,
                            "manifest_row_id": manifest_row_id,
                            "compatibility_row_ref": compat_ref,
                            "skew_case_ref": skew_case_ref,
                            "combination_label": ensure_str(
                                case.get("combination_label"),
                                f"{family}.{status}.combination_label",
                            ),
                            "status": status,
                            "verdict": verdict,
                            "window_class": ensure_str(
                                case.get("window_class"),
                                f"{family}.{status}.window_class",
                            ),
                            "out_of_window_posture": ensure_str(
                                case.get("outside_window_posture"),
                                f"{family}.{status}.outside_window_posture",
                            ),
                            "is_current_window": is_current,
                            "mutation_blocked": verdict != "compatible",
                            "in_support_export": in_support_export,
                            "guidance_path": guidance,
                        }
                    )
    cases.sort(
        key=lambda case: (
            REQUIRED_FAMILIES.index(case["manifest_family"]),
            case["compatibility_row_ref"],
            STATUS_VOCABULARY.index(case["status"]),
            case["skew_case_ref"],
        )
    )
    return cases


def build_corpus(
    *,
    manifests: dict[str, dict[str, Any]],
    manifest_refs: dict[str, str],
    support_export: dict[str, Any],
    support_export_rel: str,
    generated_at: str,
) -> dict[str, Any]:
    quoted_refs = support_refs(support_export)
    cases = build_cases(
        manifests=manifests, manifest_refs=manifest_refs, quoted_refs=quoted_refs
    )
    families_covered = sorted({case["manifest_family"] for case in cases})
    summary = {
        "total_cases": len(cases),
        "compatible_cases": sum(1 for c in cases if c["verdict"] == "compatible"),
        "probe_required_cases": sum(
            1 for c in cases if c["verdict"] == "probe_required"
        ),
        "incompatible_cases": sum(1 for c in cases if c["verdict"] == "incompatible"),
        "families_covered": len(families_covered),
    }
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": CORPUS_RECORD_KIND,
        "corpus_id": CORPUS_ID,
        "generated_at": generated_at,
        "source_support_export_ref": support_export_rel,
        "source_manifest_refs": [manifest_refs[family] for family in REQUIRED_FAMILIES],
        "verdict_vocabulary": list(VERDICT_VOCABULARY),
        "status_vocabulary": list(STATUS_VOCABULARY),
        "no_repair_required_guidance": NO_REPAIR_REQUIRED_GUIDANCE,
        "families": list(REQUIRED_FAMILIES),
        "cases": cases,
        "summary": summary,
    }


def validate_corpus(corpus: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    corpus_id = str(corpus.get("corpus_id", "<corpus>"))

    if corpus.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("error", "corpus.schema_version", "corpus schema_version must be 1", corpus_id)
        )
    if corpus.get("record_kind") != CORPUS_RECORD_KIND:
        findings.append(
            Finding("error", "corpus.record_kind", "corpus record_kind is not supported", corpus_id)
        )
    if list(corpus.get("verdict_vocabulary", [])) != list(VERDICT_VOCABULARY):
        findings.append(
            Finding(
                "error",
                "corpus.verdict_vocabulary",
                "verdict vocabulary must be compatible/probe_required/incompatible",
                corpus_id,
            )
        )
    if list(corpus.get("status_vocabulary", [])) != list(STATUS_VOCABULARY):
        findings.append(
            Finding(
                "error",
                "corpus.status_vocabulary",
                "status vocabulary must match the version-skew register vocabulary",
                corpus_id,
            )
        )
    if corpus.get("no_repair_required_guidance") != NO_REPAIR_REQUIRED_GUIDANCE:
        findings.append(
            Finding(
                "error",
                "corpus.no_repair_required_guidance",
                "compatible-pairing guidance token must be no_repair_required",
                corpus_id,
            )
        )

    families = set(corpus.get("families", []))
    for required in REQUIRED_FAMILIES:
        if required not in families:
            findings.append(
                Finding(
                    "error",
                    "corpus.required_family_missing",
                    "corpus must declare every distributed-compatibility family",
                    required,
                )
            )

    cases = corpus.get("cases")
    if not isinstance(cases, list) or not cases:
        findings.append(
            Finding("error", "corpus.cases_empty", "corpus must enumerate skew cases", corpus_id)
        )
        return findings

    seen_case_ids: set[str] = set()
    seen_skew_refs: set[str] = set()
    coverage: dict[str, set[str]] = {family: set() for family in REQUIRED_FAMILIES}
    for raw_case in cases:
        case = ensure_dict(raw_case, "corpus.cases[]")
        findings.extend(validate_case(case))
        case_id = str(case.get("case_id", "<case>"))
        if case_id in seen_case_ids:
            findings.append(
                Finding("error", "case.duplicate_id", "case ids must be unique", case_id)
            )
        seen_case_ids.add(case_id)
        skew_ref = str(case.get("skew_case_ref", ""))
        if skew_ref in seen_skew_refs:
            findings.append(
                Finding(
                    "error",
                    "case.duplicate_skew_case_ref",
                    "each skew case may appear once in the corpus",
                    skew_ref,
                )
            )
        seen_skew_refs.add(skew_ref)
        family = str(case.get("manifest_family", ""))
        if family in coverage:
            coverage[family].add(str(case.get("verdict", "")))

    for family, verdicts in coverage.items():
        for verdict in ("compatible", "incompatible"):
            if verdict not in verdicts:
                findings.append(
                    Finding(
                        "error",
                        "coverage.missing_verdict",
                        f"{family} must enumerate at least one {verdict} pairing",
                        family,
                    )
                )

    findings.extend(validate_summary(corpus, cases))
    return findings


def validate_case(case: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    case_id = str(case.get("case_id", "<case>"))
    status = str(case.get("status", ""))
    declared_verdict = str(case.get("verdict", ""))

    resolved = STATUS_TO_VERDICT.get(status)
    if resolved is None:
        findings.append(
            Finding(
                "error",
                "case.unhandled_skew_combination",
                f"skew status {status!r} has no verdict in the closed status->verdict map",
                case_id,
            )
        )
        return findings
    if declared_verdict != resolved:
        findings.append(
            Finding(
                "error",
                "case.verdict_mismatch",
                f"verdict {declared_verdict!r} does not match the mapped verdict {resolved!r}",
                case_id,
            )
        )

    if declared_verdict not in VERDICT_VOCABULARY:
        findings.append(
            Finding("error", "case.verdict_vocabulary", "verdict is outside the closed vocabulary", case_id)
        )

    guidance = case.get("guidance_path")
    if not isinstance(guidance, list) or not guidance:
        findings.append(
            Finding(
                "error",
                "case.guidance_path_empty",
                "every pairing must carry a repair or safe-continuation guidance path",
                case_id,
            )
        )
    elif resolved == "compatible" and guidance != [NO_REPAIR_REQUIRED_GUIDANCE]:
        findings.append(
            Finding(
                "error",
                "case.compatible_guidance",
                "compatible pairings must carry the no_repair_required guidance token",
                case_id,
            )
        )

    mutation_blocked = bool(case.get("mutation_blocked"))
    if mutation_blocked != (resolved != "compatible"):
        findings.append(
            Finding(
                "error",
                "case.mutation_flag_mismatch",
                "mutation_blocked must be set for every non-compatible verdict",
                case_id,
            )
        )

    return findings


def validate_summary(corpus: dict[str, Any], cases: list[Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = corpus.get("summary")
    if not isinstance(summary, dict):
        findings.append(
            Finding("error", "summary.missing", "corpus must carry a summary block", str(corpus.get("corpus_id")))
        )
        return findings
    expected = {
        "total_cases": len(cases),
        "compatible_cases": sum(1 for c in cases if c.get("verdict") == "compatible"),
        "probe_required_cases": sum(1 for c in cases if c.get("verdict") == "probe_required"),
        "incompatible_cases": sum(1 for c in cases if c.get("verdict") == "incompatible"),
        "families_covered": len({c.get("manifest_family") for c in cases}),
    }
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(
                Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key)
            )
    return findings


def cross_check_support_export(
    corpus: dict[str, Any], support_export: dict[str, Any]
) -> list[Finding]:
    findings: list[Finding] = []
    cases_by_ref = {
        str(case.get("skew_case_ref")): case
        for case in corpus.get("cases", [])
        if isinstance(case, dict)
    }
    for row in ensure_list(support_export.get("support_rows"), "support_export.support_rows"):
        item = ensure_dict(row, "support_export.support_rows[]")
        family = str(item.get("manifest_family"))
        compat_ref = str(item.get("compatibility_row_ref"))
        repair_hints = list(item.get("repair_hints", []))
        current = str(item.get("current_skew_case_ref"))
        row_id = str(item.get("support_row_id", compat_ref))

        current_case = cases_by_ref.get(current)
        if current_case is None:
            findings.append(
                Finding(
                    "error",
                    "support_export.current_case_unmatched",
                    f"support row current skew case {current} is not in the harness",
                    row_id,
                )
            )
        else:
            findings.extend(
                _expect_case(
                    current_case,
                    row_id=row_id,
                    expected_verdict="compatible",
                    family=family,
                    compat_ref=compat_ref,
                    expect_current=True,
                )
            )

        for ref in ensure_list(
            item.get("unsupported_case_refs"), "support_row.unsupported_case_refs"
        ):
            ref = str(ref)
            unsupported_case = cases_by_ref.get(ref)
            if unsupported_case is None:
                findings.append(
                    Finding(
                        "error",
                        "support_export.unsupported_case_unmatched",
                        f"support row unsupported skew case {ref} is not in the harness",
                        row_id,
                    )
                )
                continue
            findings.extend(
                _expect_case(
                    unsupported_case,
                    row_id=row_id,
                    expected_verdict="incompatible",
                    family=family,
                    compat_ref=compat_ref,
                    expect_current=False,
                )
            )
            if list(unsupported_case.get("guidance_path", [])) != repair_hints:
                findings.append(
                    Finding(
                        "error",
                        "support_export.guidance_mismatch",
                        "incompatible harness guidance must equal the support row repair hints",
                        row_id,
                    )
                )
    return findings


def _expect_case(
    case: dict[str, Any],
    *,
    row_id: str,
    expected_verdict: str,
    family: str,
    compat_ref: str,
    expect_current: bool,
) -> list[Finding]:
    findings: list[Finding] = []
    if case.get("verdict") != expected_verdict:
        findings.append(
            Finding(
                "error",
                "support_export.verdict_disagreement",
                f"support row case must resolve to {expected_verdict}",
                row_id,
            )
        )
    if case.get("manifest_family") != family:
        findings.append(
            Finding(
                "error",
                "support_export.family_mismatch",
                "harness case family must match the support row family",
                row_id,
            )
        )
    if case.get("compatibility_row_ref") != compat_ref:
        findings.append(
            Finding(
                "error",
                "support_export.row_mismatch",
                "harness case compatibility row must match the support row",
                row_id,
            )
        )
    if not case.get("in_support_export"):
        findings.append(
            Finding(
                "error",
                "support_export.flag_unset",
                "harness case quoted by the support export must set in_support_export",
                row_id,
            )
        )
    if expect_current and not case.get("is_current_window"):
        findings.append(
            Finding(
                "error",
                "support_export.current_flag_unset",
                "support row current case must be flagged as the current window",
                row_id,
            )
        )
    return findings


@dataclasses.dataclass
class Drill:
    drill_id: str
    expected_check_id: str
    mutate: Any


def drill_unknown_status(corpus: dict[str, Any]) -> bool:
    cases = corpus.get("cases")
    if not cases:
        return False
    cases[0]["status"] = "quantum_superposed"
    return True


def drill_empty_guidance(corpus: dict[str, Any]) -> bool:
    cases = corpus.get("cases")
    if not cases:
        return False
    cases[0]["guidance_path"] = []
    return True


def drill_drop_support_case(corpus: dict[str, Any], support_export: dict[str, Any]) -> bool:
    quoted = support_refs(support_export)
    cases = corpus.get("cases", [])
    for index, case in enumerate(cases):
        if case.get("skew_case_ref") in quoted:
            del cases[index]
            return True
    return False


def run_negative_drills(
    corpus: dict[str, Any], support_export: dict[str, Any]
) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    plans = [
        Drill("unknown_status_rejected", "case.unhandled_skew_combination", drill_unknown_status),
        Drill("empty_guidance_rejected", "case.guidance_path_empty", drill_empty_guidance),
    ]
    for drill in plans:
        mutated = copy.deepcopy(corpus)
        applied = drill.mutate(mutated)
        check_ids = {f.check_id for f in validate_corpus(mutated)}
        passed = applied and drill.expected_check_id in check_ids
        results.append(
            {
                "drill_id": drill.drill_id,
                "expected_check_id": drill.expected_check_id,
                "status": "passed" if passed else "failed",
            }
        )
        if not passed:
            findings.append(
                Finding(
                    "error",
                    "negative_drill.not_rejected",
                    f"negative drill {drill.drill_id} did not raise {drill.expected_check_id}",
                    drill.drill_id,
                )
            )

    mutated = copy.deepcopy(corpus)
    applied = drill_drop_support_case(mutated, support_export)
    check_ids = {f.check_id for f in cross_check_support_export(mutated, support_export)}
    passed = applied and (
        "support_export.current_case_unmatched" in check_ids
        or "support_export.unsupported_case_unmatched" in check_ids
    )
    results.append(
        {
            "drill_id": "dropped_support_case_rejected",
            "expected_check_id": "support_export.*_case_unmatched",
            "status": "passed" if passed else "failed",
        }
    )
    if not passed:
        findings.append(
            Finding(
                "error",
                "negative_drill.not_rejected",
                "negative drill dropped_support_case_rejected did not break the cross-check",
                "dropped_support_case_rejected",
            )
        )
    return results, findings


def write_report(
    path: Path,
    corpus: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    corpus_changed: bool,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "distributed_compatibility_skew_harness_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "corpus_id": corpus.get("corpus_id"),
        "corpus_changed": corpus_changed,
        "summary": corpus.get("summary"),
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    manifests = {
        family: ensure_dict(
            load_json(repo_root / manifest_ref_for(args.manifest_dir, family)),
            f"{family}.manifest",
        )
        for family in REQUIRED_FAMILIES
    }
    manifest_refs = {
        family: manifest_ref_for(args.manifest_dir, family) for family in REQUIRED_FAMILIES
    }
    support_export = ensure_dict(
        load_json(repo_root / args.support_export), "support_export"
    )

    generated_at = generated_at_now()
    corpus = build_corpus(
        manifests=manifests,
        manifest_refs=manifest_refs,
        support_export=support_export,
        support_export_rel=args.support_export,
        generated_at=generated_at,
    )

    corpus_path = repo_root / args.corpus
    new_text = json_text(corpus)
    existing = corpus_path.read_text(encoding="utf-8") if corpus_path.exists() else None
    corpus_changed = existing is None or normalize_generated_at(existing) != normalize_generated_at(
        new_text
    )
    if not args.check:
        corpus_path.parent.mkdir(parents=True, exist_ok=True)
        corpus_path.write_text(new_text, encoding="utf-8")

    findings = validate_corpus(corpus)
    findings.extend(cross_check_support_export(corpus, support_export))
    drill_results, drill_findings = run_negative_drills(corpus, support_export)
    findings.extend(drill_findings)

    if args.check and corpus_changed:
        findings.append(
            Finding(
                "error",
                "corpus.stale",
                "checked-in skew-harness verdict corpus is stale; regenerate and commit it",
                args.corpus,
            )
        )

    report_rel = args.report
    if args.check and report_rel is None:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(
            repo_root / report_rel, corpus, findings, drill_results, corpus_changed
        )

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    print(
        "beta distributed-compatibility skew harness validated "
        f"({corpus['summary']['total_cases']} cases across "
        f"{corpus['summary']['families_covered']} families, "
        f"{len(drill_results)} negative drills)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
