#!/usr/bin/env python3
"""Gate the cross-subsystem integration checkpoint.

Every Beta subsystem is already gated in isolation. This gate enforces the
*integration* requirement those per-subsystem gates cannot: that the frozen
integration-checkpoint corpus genuinely composes the six required subsystems
and proves they degrade *honestly together* rather than only in isolation.

It loads the integration-checkpoint corpus (the same frozen data the smoke
runner replays) and asserts:

- all six required subsystems are present (by id), each resolving to a real
  subsystem input file;
- there is exactly one all-green case and every required subsystem is
  exercised as the degraded subsystem in some case (a missing required
  subsystem case fails the gate);
- every per-subsystem-degraded case carries honest joint reporting: the
  degraded subsystem reports its own degradation with a typed reason, every
  healthy peer *surfaces* it (no silent success), the joint verdict derived
  from the per-subsystem contributions matches the declared expectation and
  never resolves to ``go`` while degraded, and the verdict is surfaced
  consistently to every consuming projection; and
- no degraded case is an *isolated-only proof* — a case where the degradation
  is reported by its own subsystem but surfaced by none of its peers proves
  isolation, not integration, and fails the gate.

The gate exits non-zero on any missing required subsystem or isolated-only
proof. YAML decoding goes through Ruby/Psych, matching the repository
convention shared by the adjacent checks and smoke runners.
"""

from __future__ import annotations

import argparse
import datetime as dt
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

# The six Beta subsystems this checkpoint composes. These ids are fixed here so
# the gate asserts the integration requirement independently of whatever the
# corpus happens to declare: dropping any one of these subsystems (its input,
# its declaration, or its degraded case) fails the gate.
REQUIRED_SUBSYSTEMS = {
    "extension_lifecycle",
    "workspace_trust",
    "packaging_update_rollback",
    "enterprise_proxy_policy",
    "support_bundle",
    "remote_attach",
}

LABEL = "beta-integration-checkpoint"


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
        "--corpus",
        default=DEFAULT_CORPUS_REL,
        help="Integration-checkpoint corpus YAML path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
    )
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


def derive_joint_verdict(contributions: list[str]) -> str:
    """Combine per-subsystem contributions into the joint checkpoint verdict.

    A single hard block wins (``no_go``); otherwise any degraded contribution
    keeps the checkpoint honest (``conditional_go``); only an all-clear board
    resolves to ``go``. This mirrors the smoke runner's derivation.
    """
    if "no_go" in contributions:
        return "no_go"
    if "conditional_go" in contributions:
        return "conditional_go"
    return "go"


def load_subsystems(
    repo_root: Path,
    corpus: dict[str, Any],
    findings: list[Finding],
) -> tuple[list[str], dict[str, dict[str, Any] | None], dict[str, set[str]]]:
    """Resolve the corpus subsystem inputs.

    Returns the declared subsystem ids (in order), a map of id -> case_postures
    (None when the input is missing/unreadable), and a map of id -> typed
    degradation-reason vocabulary.
    """
    subsystem_entries = ensure_list(corpus.get("subsystems"), "corpus.subsystems")
    subsystem_ids: list[str] = []
    postures: dict[str, dict[str, Any] | None] = {}
    reason_vocab: dict[str, set[str]] = {}

    for idx, raw in enumerate(subsystem_entries):
        entry = ensure_dict(raw, f"corpus.subsystems[{idx}]")
        subsystem_id = ensure_str(entry.get("id"), f"corpus.subsystems[{idx}].id")
        input_ref = ensure_str(
            entry.get("input_ref"), f"corpus.subsystems[{idx}].input_ref"
        )
        if subsystem_id in postures:
            findings.append(
                Finding(
                    severity="error",
                    check_id="subsystem.duplicate",
                    message=f"duplicate subsystem id: {subsystem_id}",
                    remediation="Subsystem ids must be unique in the corpus.",
                    ref=subsystem_id,
                )
            )
            continue
        subsystem_ids.append(subsystem_id)

        input_path = repo_root / input_ref
        if not input_path.exists():
            postures[subsystem_id] = None
            findings.append(
                Finding(
                    severity="error",
                    check_id="subsystem.input_missing",
                    message=f"subsystem '{subsystem_id}' input does not exist: {input_ref}",
                    remediation=(
                        "Restore the subsystem input; the checkpoint must integrate "
                        "all six subsystems."
                    ),
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
                    check_id="subsystem.id_mismatch",
                    message=(
                        f"subsystem input declares subsystem_id '{declared_id}' but is "
                        f"referenced as '{subsystem_id}'"
                    ),
                    remediation="Make the subsystem input's subsystem_id match the corpus.",
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

    return subsystem_ids, postures, reason_vocab


def validate_case(
    case: dict[str, Any],
    *,
    subsystem_ids: list[str],
    postures: dict[str, dict[str, Any] | None],
    projection_ids: list[str],
    verdict_vocab: set[str],
    health_vocab: set[str],
    reason_vocab: dict[str, set[str]],
    findings: list[Finding],
) -> None:
    case_id = ensure_str(case.get("case_id"), "corpus.cases[].case_id")
    degraded = case.get("degraded_subsystem")
    if degraded is not None:
        degraded = ensure_str(degraded, f"{case_id}.degraded_subsystem")
    expected = ensure_str(
        case.get("expected_joint_verdict"), f"{case_id}.expected_joint_verdict"
    )

    def fail(check_id: str, message: str, ref: str | None = None) -> None:
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"{case_id}: {message}",
                remediation=(
                    "Re-align the corpus so the six subsystems degrade honestly "
                    "together; the gate fails closed on any silent success."
                ),
                ref=ref,
            )
        )

    if degraded is not None and degraded not in subsystem_ids:
        fail(
            "case.unknown_degraded_subsystem",
            f"degraded_subsystem '{degraded}' is not a declared subsystem",
            ref=degraded,
        )
    if expected not in verdict_vocab:
        fail(
            "verdict.unknown_class",
            f"expected_joint_verdict '{expected}' is not in the verdict vocabulary",
        )

    contributions: list[str] = []
    posture_resolved = True
    peers_surfacing = 0
    peer_count = 0

    for subsystem_id in subsystem_ids:
        subsystem_postures = postures.get(subsystem_id)
        if subsystem_postures is None:
            # The input was missing; recorded once at load time. Fail closed.
            fail(
                "subsystem.posture_unavailable",
                f"subsystem '{subsystem_id}' input is unavailable",
                ref=subsystem_id,
            )
            posture_resolved = False
            continue
        posture = subsystem_postures.get(case_id)
        if not isinstance(posture, dict):
            fail(
                "subsystem.posture_missing",
                f"subsystem '{subsystem_id}' has no posture for this case",
                ref=subsystem_id,
            )
            posture_resolved = False
            continue

        self_health = ensure_str(
            posture.get("self_health"), f"{subsystem_id}.{case_id}.self_health"
        )
        contribution = ensure_str(
            posture.get("verdict_contribution"),
            f"{subsystem_id}.{case_id}.verdict_contribution",
        )
        observed = bool(posture.get("peer_degradation_observed"))
        contributions.append(contribution)

        if self_health not in health_vocab:
            fail(
                "health.unknown_class",
                f"subsystem '{subsystem_id}' self_health '{self_health}' is not in vocabulary",
                ref=subsystem_id,
            )
        if contribution not in verdict_vocab:
            fail(
                "verdict.unknown_class",
                f"subsystem '{subsystem_id}' verdict_contribution '{contribution}' is not in vocabulary",
                ref=subsystem_id,
            )

        is_source = degraded is not None and subsystem_id == degraded

        if is_source:
            if self_health != "degraded":
                fail(
                    "degraded.source_not_degraded",
                    (
                        f"subsystem '{subsystem_id}' is the degraded subsystem but reports "
                        f"self_health '{self_health}' (silent success at source)"
                    ),
                    ref=subsystem_id,
                )
            reason = posture.get("degradation_reason")
            if not is_nonempty_str(reason):
                fail(
                    "degraded.source_reason_missing",
                    f"degraded subsystem '{subsystem_id}' must declare a degradation_reason",
                    ref=subsystem_id,
                )
            elif reason.strip() not in reason_vocab.get(subsystem_id, set()):
                fail(
                    "degraded.source_reason_unknown",
                    (
                        f"degraded subsystem '{subsystem_id}' reason '{reason.strip()}' is not "
                        "in its degradation_reason_vocabulary"
                    ),
                    ref=subsystem_id,
                )
            if not is_nonempty_str(posture.get("self_degradation_detail")):
                fail(
                    "degraded.source_detail_missing",
                    f"degraded subsystem '{subsystem_id}' must explain its degradation",
                    ref=subsystem_id,
                )
            if contribution == "go":
                fail(
                    "degraded.source_contribution_go",
                    (
                        f"degraded subsystem '{subsystem_id}' must not contribute 'go' while "
                        "degraded (silent success)"
                    ),
                    ref=subsystem_id,
                )
        elif degraded is not None:
            # Healthy peer in a degraded case: MUST surface the degradation.
            peer_count += 1
            if self_health != "healthy":
                fail(
                    "degraded.unexpected_peer_degradation",
                    (
                        f"subsystem '{subsystem_id}' reports '{self_health}' but only "
                        f"'{degraded}' may degrade in this case"
                    ),
                    ref=subsystem_id,
                )
            if observed:
                if not is_nonempty_str(posture.get("peer_acknowledgement")):
                    fail(
                        "peer.acknowledgement_missing",
                        (
                            f"peer '{subsystem_id}' observed the degradation but carries no "
                            "acknowledgement text"
                        ),
                        ref=subsystem_id,
                    )
                else:
                    peers_surfacing += 1
            else:
                fail(
                    "peer.silent_success",
                    (
                        f"healthy peer '{subsystem_id}' did not surface the '{degraded}' "
                        "degradation (silent success)"
                    ),
                    ref=subsystem_id,
                )
        else:
            # Green case: every subsystem healthy, contributing go.
            if self_health != "healthy" or contribution != "go":
                fail(
                    "green.unexpected_degradation",
                    (
                        f"subsystem '{subsystem_id}' reports health '{self_health}' / "
                        f"contribution '{contribution}' in the all-healthy case"
                    ),
                    ref=subsystem_id,
                )

    if not posture_resolved:
        return

    # The integration proof: a degraded case where no peer surfaces the
    # degradation proves it in isolation only, never integration.
    if degraded is not None and peer_count > 0 and peers_surfacing == 0:
        fail(
            "joint.isolated_only_proof",
            (
                f"degraded subsystem '{degraded}' is surfaced by none of its {peer_count} "
                "peers — this proves the degradation in isolation only, not integration"
            ),
            ref=degraded,
        )

    derived = derive_joint_verdict(contributions)
    if derived != expected:
        fail(
            "joint.verdict_mismatch",
            (
                f"derived joint verdict '{derived}' does not match declared "
                f"expected_joint_verdict '{expected}'"
            ),
        )
    if degraded is not None and derived == "go":
        fail(
            "joint.degraded_case_surfaced_as_go",
            "a degraded case must never resolve to a 'go' joint verdict (silent success)",
        )

    projection_verdicts = ensure_dict(
        case.get("projection_verdicts"), f"{case_id}.projection_verdicts"
    )
    for projection_id in projection_ids:
        if projection_id not in projection_verdicts:
            fail(
                "projection.missing",
                f"projection '{projection_id}' is not surfaced by this case",
                ref=projection_id,
            )
            continue
        surfaced = ensure_str(
            projection_verdicts.get(projection_id),
            f"{case_id}.projection_verdicts.{projection_id}",
        )
        if surfaced != derived:
            fail(
                "projection.verdict_divergence",
                (
                    f"projection '{projection_id}' surfaces '{surfaced}' but the joint "
                    f"verdict is '{derived}' (inconsistent surfacing)"
                ),
                ref=projection_id,
            )
    for projection_id in projection_verdicts:
        if projection_id not in projection_ids:
            fail(
                "projection.unknown",
                f"projection '{projection_id}' is not a declared consuming projection",
                ref=projection_id,
            )


def validate_corpus(
    repo_root: Path,
    corpus: dict[str, Any],
    findings: list[Finding],
) -> None:
    schema_version = corpus.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="corpus.schema_version",
                message=f"corpus schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the gate together with the corpus if its shape changes.",
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

    subsystem_ids, postures, reason_vocab = load_subsystems(repo_root, corpus, findings)

    declared = set(subsystem_ids)
    missing_required = sorted(REQUIRED_SUBSYSTEMS - declared)
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="subsystems.missing_required",
                message=f"corpus is missing required subsystems: {', '.join(missing_required)}",
                remediation="Add the missing subsystem rows so the checkpoint composes all six Beta subsystems.",
                details={
                    "required": sorted(REQUIRED_SUBSYSTEMS),
                    "declared": sorted(declared),
                },
            )
        )
    unexpected = sorted(declared - REQUIRED_SUBSYSTEMS)
    if unexpected:
        findings.append(
            Finding(
                severity="error",
                check_id="subsystems.unexpected",
                message=f"corpus declares unexpected subsystems: {', '.join(unexpected)}",
                remediation="The integration checkpoint composes exactly the six required Beta subsystems.",
                details={"unexpected": unexpected},
            )
        )

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
                check_id="corpus.no_projections",
                message="corpus must declare at least one consuming projection",
                remediation="Name where the joint verdict is surfaced.",
            )
        )

    cases = ensure_list(corpus.get("cases"), "corpus.cases")
    seen_case_ids: set[str] = set()
    green_cases = 0
    exercised_subsystems: set[str] = set()

    for raw in cases:
        case = ensure_dict(raw, "corpus.cases[]")
        case_id = ensure_str(case.get("case_id"), "corpus.cases[].case_id")
        if case_id in seen_case_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="case.duplicate",
                    message=f"duplicate case_id: {case_id}",
                    remediation="case_ids must be unique.",
                    ref=case_id,
                )
            )
        seen_case_ids.add(case_id)

        degraded = case.get("degraded_subsystem")
        if degraded is None:
            green_cases += 1
        elif isinstance(degraded, str):
            exercised_subsystems.add(degraded.strip())

        validate_case(
            case,
            subsystem_ids=subsystem_ids,
            postures=postures,
            projection_ids=projection_ids,
            verdict_vocab=verdict_vocab,
            health_vocab=health_vocab,
            reason_vocab=reason_vocab,
            findings=findings,
        )

    if green_cases != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="coverage.green_case",
                message=f"corpus must declare exactly one all-green case, found {green_cases}",
                remediation="Seed exactly one case with degraded_subsystem: null.",
            )
        )

    # A required subsystem with no degraded case is a missing required subsystem
    # case: the checkpoint would no longer prove it integrates that subsystem.
    not_exercised = sorted(REQUIRED_SUBSYSTEMS - exercised_subsystems)
    if not_exercised:
        findings.append(
            Finding(
                severity="error",
                check_id="coverage.required_subsystem_case_missing",
                message=(
                    "every required subsystem must be the degraded subsystem in some case; "
                    f"missing cases for: {', '.join(not_exercised)}"
                ),
                remediation="Add a degraded case for each missing subsystem so the checkpoint integrates all six.",
                details={"missing": not_exercised},
            )
        )


def write_report(repo_root: Path, report_rel: str, corpus_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "beta_integration_checkpoint",
        "corpus_ref": corpus_rel,
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
    report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    corpus_rel = str(args.corpus)
    corpus_path = repo_root / corpus_rel
    corpus = ensure_dict(render_yaml_as_json(corpus_path), corpus_rel)

    findings: list[Finding] = []
    validate_corpus(repo_root, corpus, findings)

    if args.report:
        write_report(repo_root, str(args.report), corpus_rel, findings)

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
