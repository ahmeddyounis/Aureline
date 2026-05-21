#!/usr/bin/env python3
"""Cross-subsystem integration-checkpoint smoke runner.

Every Beta subsystem is already gated in isolation. This lane proves the
integration requirement those isolated gates cannot: that six subsystems
degrade *honestly together*.

It joins a frozen, deterministic corpus
(``tests/desktop/integration_checkpoint_smoke/corpus/``) that composes six
subsystem inputs:

- extension install / disable / update
- trust / restricted mode
- packaging / update / rollback
- enterprise proxy / policy path
- support bundle
- remote attach

For every case in the corpus the runner asserts:

- exactly the declared ``degraded_subsystem`` reports ``degraded`` (and it
  declares a typed degradation reason from its own vocabulary), every other
  subsystem reports ``healthy``;
- the degraded subsystem never contributes a silent ``go`` while degraded;
- every healthy peer *surfaces* the degradation (``peer_degradation_observed``
  with a non-empty acknowledgement) — a healthy peer that stays silent is
  silent success and fails;
- the joint verdict derived from the per-subsystem contributions matches the
  case's declared ``expected_joint_verdict`` (and a degraded case never
  resolves to ``go``);
- the joint verdict is surfaced consistently to every consuming projection.

It also asserts coverage: there is one all-green case and every one of the six
subsystems is exercised as the degraded subsystem in some case.

The lane is pure data: it has no running app. Deleting an ``input_ref`` (or
stubbing a case posture inside it) fails the lane, which is what proves the
checkpoint actually integrates all six subsystems. ``--omit-subsystem`` makes
that fail-closed behavior demonstrable without mutating files.

YAML decoding goes through Ruby/Psych, matching the repository convention used
by the adjacent smoke runners. Output is deterministic: the JSON capture is
byte-identical for a given corpus.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_CORPUS_REL = (
    "tests/desktop/integration_checkpoint_smoke/corpus/"
    "integration_checkpoint_corpus.yaml"
)
DEFAULT_REPORT_REL = (
    "artifacts/integration/integration_checkpoint_smoke_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

EXPECTED_SUBSYSTEM_COUNT = 6
LABEL = "integration-checkpoint-smoke"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    case_id: str | None = None
    subsystem_id: str | None = None
    ref: str | None = None

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        for key in ("case_id", "subsystem_id", "ref"):
            if payload[key] is None:
                payload.pop(key)
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--corpus",
        default=DEFAULT_CORPUS_REL,
        help="Integration-checkpoint corpus YAML path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the deterministic JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build identity record referenced in the capture.",
    )
    parser.add_argument(
        "--omit-subsystem",
        action="append",
        default=[],
        metavar="SUBSYSTEM_ID",
        help=(
            "Simulate a removed subsystem input without deleting files. May be "
            "repeated. The lane MUST fail when any subsystem is omitted."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def is_nonempty_str(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


# ---- joint verdict ---------------------------------------------------------


def derive_joint_verdict(contributions: list[str]) -> str:
    """Combine per-subsystem contributions into the joint checkpoint verdict.

    A single hard block wins (``no_go``); otherwise any degraded contribution
    keeps the checkpoint honest (``conditional_go``); only an all-clear board
    resolves to ``go``.
    """
    if "no_go" in contributions:
        return "no_go"
    if "conditional_go" in contributions:
        return "conditional_go"
    return "go"


# ---- per-case replay -------------------------------------------------------


@dataclass
class CaseResult:
    case_id: str
    degraded_subsystem: str | None
    expected_joint_verdict: str
    derived_joint_verdict: str | None = None
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    per_subsystem: dict[str, dict[str, Any]] = field(default_factory=dict)


def evaluate_case(
    case: dict[str, Any],
    *,
    subsystem_ids: list[str],
    postures: dict[str, dict[str, Any] | None],
    projection_ids: list[str],
    verdict_vocab: set[str],
    health_vocab: set[str],
    reason_vocab: dict[str, set[str]],
    findings: list[Finding],
) -> CaseResult:
    case_id = ensure_str(case.get("case_id"), "case.case_id")
    degraded = case.get("degraded_subsystem")
    if degraded is not None:
        degraded = ensure_str(degraded, f"{case_id}.degraded_subsystem")
    expected = ensure_str(
        case.get("expected_joint_verdict"),
        f"{case_id}.expected_joint_verdict",
    )
    result = CaseResult(
        case_id=case_id,
        degraded_subsystem=degraded,
        expected_joint_verdict=expected,
    )

    def fail(check_id: str, message: str, subsystem_id: str | None = None) -> None:
        result.failed_checks.append({"check_id": check_id, "message": message})
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"{case_id}: {message}",
                remediation=(
                    "Re-align the corpus so the six subsystems degrade honestly "
                    "together; failures carry the precise actionable check_id."
                ),
                case_id=case_id,
                subsystem_id=subsystem_id,
            )
        )

    if degraded is not None and degraded not in subsystem_ids:
        fail(
            "integration_checkpoint.case.unknown_degraded_subsystem",
            f"degraded_subsystem '{degraded}' is not a declared subsystem",
        )
    if expected not in verdict_vocab:
        fail(
            "integration_checkpoint.verdict.unknown_class",
            f"expected_joint_verdict '{expected}' is not in the verdict vocabulary",
        )

    contributions: list[str] = []
    posture_resolved = True

    for subsystem_id in subsystem_ids:
        subsystem = postures.get(subsystem_id)
        if subsystem is None:
            # Input file was missing/omitted; recorded once at load time. The
            # case cannot be evaluated for this subsystem -> fail closed.
            fail(
                "integration_checkpoint.subsystem.posture_unavailable",
                f"subsystem '{subsystem_id}' input is unavailable",
                subsystem_id=subsystem_id,
            )
            posture_resolved = False
            continue
        posture = subsystem.get(case_id)
        if not isinstance(posture, dict):
            fail(
                "integration_checkpoint.subsystem.posture_missing",
                f"subsystem '{subsystem_id}' has no posture for this case",
                subsystem_id=subsystem_id,
            )
            posture_resolved = False
            continue

        self_health = ensure_str(
            posture.get("self_health"),
            f"{subsystem_id}.{case_id}.self_health",
        )
        contribution = ensure_str(
            posture.get("verdict_contribution"),
            f"{subsystem_id}.{case_id}.verdict_contribution",
        )
        observed = bool(posture.get("peer_degradation_observed"))
        result.per_subsystem[subsystem_id] = {
            "self_health": self_health,
            "verdict_contribution": contribution,
            "peer_degradation_observed": observed,
        }
        contributions.append(contribution)

        if self_health not in health_vocab:
            fail(
                "integration_checkpoint.health.unknown_class",
                f"subsystem '{subsystem_id}' self_health '{self_health}' is not in vocabulary",
                subsystem_id=subsystem_id,
            )
        if contribution not in verdict_vocab:
            fail(
                "integration_checkpoint.verdict.unknown_class",
                f"subsystem '{subsystem_id}' verdict_contribution '{contribution}' is not in vocabulary",
                subsystem_id=subsystem_id,
            )

        is_source = degraded is not None and subsystem_id == degraded

        if is_source:
            # The degrading subsystem MUST report its own degradation honestly.
            if self_health != "degraded":
                fail(
                    "integration_checkpoint.degraded.source_not_degraded",
                    (
                        f"subsystem '{subsystem_id}' is the degraded subsystem but "
                        f"reports self_health '{self_health}' (silent success at source)"
                    ),
                    subsystem_id=subsystem_id,
                )
            reason = posture.get("degradation_reason")
            if not is_nonempty_str(reason):
                fail(
                    "integration_checkpoint.degraded.source_reason_missing",
                    f"degraded subsystem '{subsystem_id}' must declare a degradation_reason",
                    subsystem_id=subsystem_id,
                )
            elif reason.strip() not in reason_vocab.get(subsystem_id, set()):
                fail(
                    "integration_checkpoint.degraded.source_reason_unknown",
                    (
                        f"degraded subsystem '{subsystem_id}' reason '{reason.strip()}' is not "
                        "in its degradation_reason_vocabulary"
                    ),
                    subsystem_id=subsystem_id,
                )
            if not is_nonempty_str(posture.get("self_degradation_detail")):
                fail(
                    "integration_checkpoint.degraded.source_detail_missing",
                    f"degraded subsystem '{subsystem_id}' must explain its degradation",
                    subsystem_id=subsystem_id,
                )
            if contribution == "go":
                fail(
                    "integration_checkpoint.degraded.source_contribution_go",
                    (
                        f"degraded subsystem '{subsystem_id}' must not contribute 'go' while "
                        "degraded (silent success)"
                    ),
                    subsystem_id=subsystem_id,
                )
        elif degraded is not None:
            # Healthy peer in a degraded case: MUST surface the degradation.
            if self_health != "healthy":
                fail(
                    "integration_checkpoint.degraded.unexpected_peer_degradation",
                    (
                        f"subsystem '{subsystem_id}' reports '{self_health}' but only "
                        f"'{degraded}' may degrade in this case"
                    ),
                    subsystem_id=subsystem_id,
                )
            if not observed:
                fail(
                    "integration_checkpoint.peer.silent_success",
                    (
                        f"healthy peer '{subsystem_id}' did not surface the '{degraded}' "
                        "degradation (silent success)"
                    ),
                    subsystem_id=subsystem_id,
                )
            elif not is_nonempty_str(posture.get("peer_acknowledgement")):
                fail(
                    "integration_checkpoint.peer.acknowledgement_missing",
                    (
                        f"peer '{subsystem_id}' observed the degradation but carries no "
                        "acknowledgement text"
                    ),
                    subsystem_id=subsystem_id,
                )
        else:
            # Green case: every subsystem healthy, contributing go, no peer noise.
            if self_health != "healthy":
                fail(
                    "integration_checkpoint.green.unexpected_degradation",
                    f"subsystem '{subsystem_id}' reports '{self_health}' in the all-healthy case",
                    subsystem_id=subsystem_id,
                )
            if contribution != "go":
                fail(
                    "integration_checkpoint.green.unexpected_degradation",
                    f"subsystem '{subsystem_id}' contributes '{contribution}' in the all-healthy case",
                    subsystem_id=subsystem_id,
                )

    if not posture_resolved:
        return result

    derived = derive_joint_verdict(contributions)
    result.derived_joint_verdict = derived

    if derived != expected:
        fail(
            "integration_checkpoint.joint.verdict_mismatch",
            (
                f"derived joint verdict '{derived}' does not match declared "
                f"expected_joint_verdict '{expected}'"
            ),
        )
    else:
        result.passed_checks.append(f"joint verdict '{derived}' matches expectation")

    if degraded is not None and derived == "go":
        fail(
            "integration_checkpoint.joint.degraded_case_surfaced_as_go",
            "a degraded case must never resolve to a 'go' joint verdict (silent success)",
        )

    # Joint verdict must be surfaced consistently to every consuming projection.
    projection_verdicts = ensure_dict(
        case.get("projection_verdicts"), f"{case_id}.projection_verdicts"
    )
    for projection_id in projection_ids:
        if projection_id not in projection_verdicts:
            fail(
                "integration_checkpoint.projection.missing",
                f"projection '{projection_id}' is not surfaced by this case",
            )
            continue
        surfaced = ensure_str(
            projection_verdicts.get(projection_id),
            f"{case_id}.projection_verdicts.{projection_id}",
        )
        if surfaced != derived:
            fail(
                "integration_checkpoint.projection.verdict_divergence",
                (
                    f"projection '{projection_id}' surfaces '{surfaced}' but the joint "
                    f"verdict is '{derived}' (inconsistent surfacing)"
                ),
            )
        else:
            result.passed_checks.append(
                f"projection '{projection_id}' surfaces '{derived}' consistently"
            )
    for projection_id in projection_verdicts:
        if projection_id not in projection_ids:
            fail(
                "integration_checkpoint.projection.unknown",
                f"projection '{projection_id}' is not a declared consuming projection",
            )

    return result


# ---- main ------------------------------------------------------------------


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    corpus_rel = args.corpus
    corpus_path = repo_root / corpus_rel
    if not corpus_path.exists():
        raise SystemExit(f"missing corpus file: {corpus_path}")
    corpus = ensure_dict(render_yaml_as_json(corpus_path), corpus_rel)

    findings: list[Finding] = []

    schema_version = corpus.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="integration_checkpoint.corpus.schema_version",
                message=f"corpus schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the runner together with the corpus if its shape changes.",
            )
        )

    verdict_vocab = {
        ensure_str(v, "corpus.verdict_class_vocabulary[]")
        for v in ensure_list(
            corpus.get("verdict_class_vocabulary"),
            "corpus.verdict_class_vocabulary",
        )
    }
    health_vocab = {
        ensure_str(v, "corpus.health_class_vocabulary[]")
        for v in ensure_list(
            corpus.get("health_class_vocabulary"),
            "corpus.health_class_vocabulary",
        )
    }

    # ---- subsystems ----
    subsystem_entries = ensure_list(corpus.get("subsystems"), "corpus.subsystems")
    omit = {s.strip() for s in args.omit_subsystem if s.strip()}
    subsystem_ids: list[str] = []
    subsystem_refs: dict[str, str] = {}
    postures: dict[str, dict[str, Any] | None] = {}
    reason_vocab: dict[str, set[str]] = {}

    for idx, raw in enumerate(subsystem_entries):
        entry = ensure_dict(raw, f"corpus.subsystems[{idx}]")
        subsystem_id = ensure_str(entry.get("id"), f"corpus.subsystems[{idx}].id")
        input_ref = ensure_str(
            entry.get("input_ref"), f"corpus.subsystems[{idx}].input_ref"
        )
        if subsystem_id in subsystem_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="integration_checkpoint.subsystem.duplicate",
                    message=f"duplicate subsystem id: {subsystem_id}",
                    remediation="Subsystem ids must be unique.",
                    subsystem_id=subsystem_id,
                )
            )
            continue
        subsystem_ids.append(subsystem_id)
        subsystem_refs[subsystem_id] = input_ref

        input_path = repo_root / input_ref
        if subsystem_id in omit:
            postures[subsystem_id] = None
            findings.append(
                Finding(
                    severity="error",
                    check_id="integration_checkpoint.subsystem.input_omitted",
                    message=f"subsystem '{subsystem_id}' input omitted via --omit-subsystem",
                    remediation="Remove --omit-subsystem to run the full integrated lane.",
                    subsystem_id=subsystem_id,
                    ref=input_ref,
                )
            )
            continue
        if not input_path.exists():
            postures[subsystem_id] = None
            findings.append(
                Finding(
                    severity="error",
                    check_id="integration_checkpoint.subsystem.input_missing",
                    message=f"subsystem '{subsystem_id}' input does not exist: {input_ref}",
                    remediation=(
                        "Restore the subsystem input; the checkpoint must integrate "
                        "all six subsystems."
                    ),
                    subsystem_id=subsystem_id,
                    ref=input_ref,
                )
            )
            continue

        payload = ensure_dict(render_yaml_as_json(input_path), input_ref)
        declared_id = ensure_str(payload.get("subsystem_id"), f"{input_ref}.subsystem_id")
        if declared_id != subsystem_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="integration_checkpoint.subsystem.id_mismatch",
                    message=(
                        f"subsystem input declares subsystem_id '{declared_id}' but is "
                        f"referenced as '{subsystem_id}'"
                    ),
                    remediation="Make the subsystem input's subsystem_id match the corpus.",
                    subsystem_id=subsystem_id,
                    ref=input_ref,
                )
            )
        reason_vocab[subsystem_id] = {
            ensure_str(r, f"{input_ref}.degradation_reason_vocabulary[]")
            for r in ensure_list(
                payload.get("degradation_reason_vocabulary"),
                f"{input_ref}.degradation_reason_vocabulary",
            )
        }
        postures[subsystem_id] = ensure_dict(
            payload.get("case_postures"), f"{input_ref}.case_postures"
        )

    if len(subsystem_ids) != EXPECTED_SUBSYSTEM_COUNT:
        findings.append(
            Finding(
                severity="error",
                check_id="integration_checkpoint.corpus.subsystem_count",
                message=(
                    f"corpus must declare exactly {EXPECTED_SUBSYSTEM_COUNT} subsystems, "
                    f"found {len(subsystem_ids)}"
                ),
                remediation="The integration checkpoint composes exactly the six Beta subsystems.",
            )
        )

    # ---- projections ----
    projection_entries = ensure_list(
        corpus.get("consuming_projections"), "corpus.consuming_projections"
    )
    projection_ids: list[str] = []
    for idx, raw in enumerate(projection_entries):
        entry = ensure_dict(raw, f"corpus.consuming_projections[{idx}]")
        projection_ids.append(
            ensure_str(entry.get("id"), f"corpus.consuming_projections[{idx}].id")
        )
    if not projection_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="integration_checkpoint.corpus.no_projections",
                message="corpus must declare at least one consuming projection",
                remediation="Name where the joint verdict is surfaced.",
            )
        )

    # ---- cases ----
    cases = ensure_list(corpus.get("cases"), "corpus.cases")
    case_ids: list[str] = []
    case_results: list[CaseResult] = []
    seen_case_ids: set[str] = set()
    green_cases = 0
    exercised_subsystems: set[str] = set()

    for raw in cases:
        case = ensure_dict(raw, "corpus.cases[]")
        case_id = ensure_str(case.get("case_id"), "corpus.cases[].case_id")
        case_ids.append(case_id)
        if case_id in seen_case_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="integration_checkpoint.case.duplicate",
                    message=f"duplicate case_id: {case_id}",
                    remediation="case_ids must be unique.",
                    case_id=case_id,
                )
            )
        seen_case_ids.add(case_id)

        degraded = case.get("degraded_subsystem")
        if degraded is None:
            green_cases += 1
        elif isinstance(degraded, str):
            exercised_subsystems.add(degraded.strip())

        result = evaluate_case(
            case,
            subsystem_ids=subsystem_ids,
            postures=postures,
            projection_ids=projection_ids,
            verdict_vocab=verdict_vocab,
            health_vocab=health_vocab,
            reason_vocab=reason_vocab,
            findings=findings,
        )
        case_results.append(result)

    # Every subsystem case posture must map to a real case (no orphan stubs).
    for subsystem_id in subsystem_ids:
        subsystem_postures = postures.get(subsystem_id)
        if not isinstance(subsystem_postures, dict):
            continue
        for posture_case_id in sorted(subsystem_postures):
            if posture_case_id not in seen_case_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="integration_checkpoint.subsystem.posture_orphan",
                        message=(
                            f"subsystem '{subsystem_id}' declares a posture for unknown "
                            f"case '{posture_case_id}'"
                        ),
                        remediation="Remove the orphan posture or add the matching case.",
                        subsystem_id=subsystem_id,
                        case_id=posture_case_id,
                    )
                )

    # ---- coverage ----
    if green_cases != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="integration_checkpoint.coverage.green_case",
                message=f"corpus must declare exactly one all-green case, found {green_cases}",
                remediation="Seed exactly one case with degraded_subsystem: null.",
            )
        )
    not_exercised = [s for s in subsystem_ids if s not in exercised_subsystems]
    if not_exercised:
        findings.append(
            Finding(
                severity="error",
                check_id="integration_checkpoint.coverage.subsystem_not_exercised",
                message=(
                    "every subsystem must be the degraded subsystem in some case; "
                    f"missing: {sorted(not_exercised)}"
                ),
                remediation="Add a degraded case for each missing subsystem.",
            )
        )

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "integration_checkpoint_smoke_validation_capture",
        "corpus_id": corpus.get("corpus_id"),
        "corpus_revision": corpus.get("corpus_revision"),
        "corpus_ref": corpus_rel,
        "build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/desktop/integration_checkpoint_smoke/"
            "run_integration_checkpoint_smoke.py --repo-root ."
        ),
        "status": status,
        "subsystems": subsystem_ids,
        "consuming_projections": projection_ids,
        "omitted_subsystems": sorted(omit),
        "cases": [
            {
                "case_id": r.case_id,
                "degraded_subsystem": r.degraded_subsystem,
                "expected_joint_verdict": r.expected_joint_verdict,
                "derived_joint_verdict": r.derived_joint_verdict,
                "per_subsystem": r.per_subsystem,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
            }
            for r in case_results
        ],
        "finding_counts": {"error": len(errors), "warning": len(warnings)},
        "findings": [f.as_report() for f in findings],
    }

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    print(
        f"[{LABEL}] {status} ({len(errors)} errors, {len(warnings)} warnings, "
        f"{len(case_results)} cases) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        loc = finding.case_id or ""
        if finding.subsystem_id:
            loc = f"{loc}/{finding.subsystem_id}" if loc else finding.subsystem_id
        loc_suffix = f" ({loc})" if loc else ""
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[{LABEL}] {prefix} {finding.check_id}: {finding.message}{loc_suffix}{ref_suffix}")
        print(f"[{LABEL}]   remediation: {finding.remediation}")

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(f"[{LABEL}] interrupted", file=sys.stderr)
        sys.exit(130)
