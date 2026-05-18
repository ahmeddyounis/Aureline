#!/usr/bin/env python3
"""Validate governed corpus lineage and derive claim freshness state."""

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


DEFAULT_REGISTRY_REL = "fixtures/registry/corpus_registry.yaml"
DEFAULT_POLICY_REL = "fixtures/registry/evidence_freshness_policy.yaml"
DEFAULT_REPORT_REL = "artifacts/registry/corpus_freshness_report.json"

STALE_AFTER_RE = re.compile(r"^P(\d+)D$")
VOLATILE_REPORT_RE = re.compile(
    r'"generated_at":\s*"[^"]*"|'
    r'"evaluated_on":\s*"[^"]*"|'
    r'"age_days":\s*-?\d+'
)

EXTERNAL_SOURCE_CLASSES = {
    "partner_customer_derived",
    "design_partner_derived",
    "field_derived",
}

PATH_LIKE_SUFFIXES = (
    ".json",
    ".md",
    ".py",
    ".rs",
    ".sh",
    ".toml",
    ".yaml",
    ".yml",
)

NON_PATH_PREFIXES = (
    "archetype_row:",
    "beta_archetype:",
    "claim_evidence:",
    "cohort:",
    "compat_row:",
    "corpus.",
    "corpus.asset.",
    "fuw_packet:",
    "hardware_definition.",
    "known_limit:",
    "lane:",
    "m3_claim_row:",
    "migration-corpus-flow:",
    "migration_source:",
    "privacy_intake:",
    "reference_workspace_report_row:",
    "refws.",
    "scorecard_row:",
    "support.m3.scenario.",
    "waiver:",
    "workflow.",
)


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
    parser.add_argument("--registry", default=DEFAULT_REGISTRY_REL)
    parser.add_argument("--policy", default=DEFAULT_POLICY_REL)
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
    parser.add_argument(
        "--today",
        default=None,
        help="Override today's UTC date for deterministic freshness checks.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the checked-in freshness report would change.",
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
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
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
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    severity: str = "error",
    details: dict[str, Any] | None = None,
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def parse_today(value: str | None) -> dt.date:
    if value:
        try:
            return dt.date.fromisoformat(value)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {value!r}") from exc
    return dt.datetime.now(dt.timezone.utc).date()


def parse_instant(value: str, label: str) -> dt.datetime | None:
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        try:
            as_date = dt.date.fromisoformat(value)
        except ValueError:
            return None
        return dt.datetime.combine(as_date, dt.time(), tzinfo=dt.timezone.utc)
    if parsed.tzinfo is None:
        return parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def parse_stale_after_days(value: str) -> int | None:
    match = STALE_AFTER_RE.match(value)
    if not match:
        return None
    return int(match.group(1))


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean or clean in {"none", "not_required", "not_required_synthetic"}:
        return False
    if clean.startswith(NON_PATH_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_ref(
    repo_root: Path,
    ref: str,
    label: str,
    findings: list[Finding],
) -> None:
    if looks_like_path(ref) and not ref_exists(repo_root, ref):
        add_finding(
            findings,
            f"{label}.missing_ref",
            f"{label} references a missing artifact: {ref}",
            "Seed the referenced artifact or correct the registry path.",
            ref=ref,
        )


def collect_policy_sets(policy: dict[str, Any]) -> tuple[set[str], set[str]]:
    proof_classes = {
        ensure_str(row.get("proof_class_id"), "policy.proof_classes[].proof_class_id")
        for row in ensure_list(policy.get("proof_classes"), "policy.proof_classes")
        if isinstance(row, dict)
    }
    propagation_profiles = {
        ensure_str(row.get("profile_id"), "policy.stale_propagation_profiles[].profile_id")
        for row in ensure_list(
            policy.get("stale_propagation_profiles"),
            "policy.stale_propagation_profiles",
        )
        if isinstance(row, dict)
    }
    return proof_classes, propagation_profiles


def validate_registry_header(
    repo_root: Path,
    registry: dict[str, Any],
    policy: dict[str, Any],
    findings: list[Finding],
) -> None:
    if registry.get("schema_version") != 1:
        add_finding(
            findings,
            "registry.schema_version",
            f"registry schema_version must be 1, got {registry.get('schema_version')!r}",
            "Update the validator together with any registry schema bump.",
        )
    if policy.get("schema_version") != 1:
        add_finding(
            findings,
            "policy.schema_version",
            f"policy schema_version must be 1, got {policy.get('schema_version')!r}",
            "Update the validator together with any policy schema bump.",
        )

    for ref in (
        ensure_str(registry.get("freshness_policy_ref"), "registry.freshness_policy_ref"),
        ensure_str(registry.get("intake_checklist_ref"), "registry.intake_checklist_ref"),
        ensure_str(registry.get("lineage_doc_ref"), "registry.lineage_doc_ref"),
    ):
        validate_path_ref(repo_root, ref, "registry", findings)

    validator = ensure_dict(registry.get("validator"), "registry.validator")
    for key in ("script_ref", "wrapper_ref", "latest_report_ref"):
        validate_path_ref(
            repo_root,
            ensure_str(validator.get(key), f"registry.validator.{key}"),
            "registry.validator",
            findings,
        )

    source_refs = ensure_dict(registry.get("source_contract_refs"), "source_contract_refs")
    for key, ref in source_refs.items():
        validate_path_ref(repo_root, ensure_str(ref, f"source_contract_refs.{key}"), key, findings)


def validate_assets(
    repo_root: Path,
    registry: dict[str, Any],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    assets: dict[str, dict[str, Any]] = {}
    for idx, raw in enumerate(ensure_list(registry.get("corpus_assets"), "corpus_assets")):
        row = ensure_dict(raw, f"corpus_assets[{idx}]")
        asset_id = ensure_str(row.get("corpus_asset_id"), f"corpus_assets[{idx}].corpus_asset_id")
        if asset_id in assets:
            add_finding(
                findings,
                "asset.duplicate_id",
                f"duplicate corpus asset id: {asset_id}",
                "Corpus asset ids must be unique and stable.",
                ref=asset_id,
            )
        assets[asset_id] = row

        ensure_str(row.get("title"), f"{asset_id}.title")
        source_class = ensure_str(row.get("source_class"), f"{asset_id}.source_class")
        owner = ensure_dict(row.get("owner"), f"{asset_id}.owner")
        ensure_str(owner.get("owner_dri"), f"{asset_id}.owner.owner_dri")
        ensure_str(owner.get("evidence_owner_ref"), f"{asset_id}.owner.evidence_owner_ref")

        privacy = ensure_dict(row.get("privacy"), f"{asset_id}.privacy")
        clearance_state = ensure_str(privacy.get("clearance_state"), f"{asset_id}.privacy.clearance_state")
        approved_for_ci = ensure_bool(
            privacy.get("approved_for_ci"),
            f"{asset_id}.privacy.approved_for_ci",
        )
        approved_for_public_proof = ensure_bool(
            privacy.get("approved_for_public_proof"),
            f"{asset_id}.privacy.approved_for_public_proof",
        )
        ensure_str(privacy.get("license_status"), f"{asset_id}.privacy.license_status")
        ensure_str(privacy.get("retention_class"), f"{asset_id}.privacy.retention_class")

        if source_class in EXTERNAL_SOURCE_CLASSES:
            intake_ref = ensure_str(privacy.get("intake_record_ref"), f"{asset_id}.privacy.intake_record_ref")
            redaction_state = ensure_str(privacy.get("redaction_state"), f"{asset_id}.privacy.redaction_state")
            license_clearance = ensure_str(
                privacy.get("license_clearance"),
                f"{asset_id}.privacy.license_clearance",
            )
            retention_approval = ensure_str(
                privacy.get("retention_approval"),
                f"{asset_id}.privacy.retention_approval",
            )
            if approved_for_ci or approved_for_public_proof:
                required_state = "cleared_for_ci_and_public_proof"
                if clearance_state != required_state:
                    add_finding(
                        findings,
                        "asset.external.clearance_state",
                        (
                            f"external asset {asset_id} is approved for CI/public proof "
                            f"without {required_state}"
                        ),
                        "Complete intake review before admitting partner, customer, or field data.",
                        ref=asset_id,
                    )
                for field_name, value in (
                    ("redaction_state", redaction_state),
                    ("license_clearance", license_clearance),
                    ("retention_approval", retention_approval),
                ):
                    if not value.startswith("approved"):
                        add_finding(
                            findings,
                            f"asset.external.{field_name}",
                            f"external asset {asset_id} has non-approved {field_name}: {value}",
                            "Mark the asset manual-review only until privacy, licensing, and retention are approved.",
                            ref=intake_ref,
                        )
            if not intake_ref.startswith("privacy_intake:"):
                validate_path_ref(repo_root, intake_ref, "asset.external.intake_record", findings)

        refresh = ensure_dict(row.get("refresh"), f"{asset_id}.refresh")
        ensure_str(refresh.get("cadence"), f"{asset_id}.refresh.cadence")
        last_reviewed = ensure_str(refresh.get("last_reviewed_on"), f"{asset_id}.refresh.last_reviewed_on")
        try:
            dt.date.fromisoformat(last_reviewed)
        except ValueError:
            add_finding(
                findings,
                "asset.refresh.invalid_last_reviewed",
                f"{asset_id} last_reviewed_on is not an ISO date: {last_reviewed}",
                "Use YYYY-MM-DD for refresh review dates.",
                ref=asset_id,
            )
        if not isinstance(refresh.get("stale_after_days"), int):
            add_finding(
                findings,
                "asset.refresh.stale_after_days",
                f"{asset_id} refresh.stale_after_days must be an integer",
                "Use an integer day count for asset-level freshness.",
                ref=asset_id,
            )

        for list_name in (
            "fixture_refs",
            "evidence_refs",
            "consuming_surfaces",
            "intended_claim_refs",
        ):
            for ref in ensure_list(row.get(list_name), f"{asset_id}.{list_name}"):
                validate_path_ref(repo_root, ensure_str(ref, f"{asset_id}.{list_name}[]"), list_name, findings)

        if not ensure_list(row.get("corpus_ids"), f"{asset_id}.corpus_ids"):
            add_finding(
                findings,
                "asset.corpus_ids.empty",
                f"{asset_id} must name at least one corpus or fixture id",
                "Bind every governed asset to stable corpus, scenario, or fixture ids.",
                ref=asset_id,
            )

        change_log = ensure_list(row.get("change_log"), f"{asset_id}.change_log")
        if not change_log:
            add_finding(
                findings,
                "asset.change_log.empty",
                f"{asset_id} must have at least one change-log entry",
                "Add a registry change-log entry so corpus updates remain auditable.",
                ref=asset_id,
            )

    return assets


def validate_claim_bindings(
    repo_root: Path,
    registry: dict[str, Any],
    assets: dict[str, dict[str, Any]],
    proof_classes: set[str],
    propagation_profiles: set[str],
    today: dt.date,
    findings: list[Finding],
) -> list[dict[str, Any]]:
    results: list[dict[str, Any]] = []
    seen_ids: set[str] = set()
    for idx, raw in enumerate(ensure_list(registry.get("claim_bindings"), "claim_bindings")):
        row = ensure_dict(raw, f"claim_bindings[{idx}]")
        binding_id = ensure_str(row.get("binding_id"), f"claim_bindings[{idx}].binding_id")
        if binding_id in seen_ids:
            add_finding(
                findings,
                "binding.duplicate_id",
                f"duplicate claim binding id: {binding_id}",
                "Claim binding ids must be unique and stable.",
                ref=binding_id,
            )
        seen_ids.add(binding_id)

        claim_refs = [
            ensure_str(ref, f"{binding_id}.claim_row_refs[]")
            for ref in ensure_list(row.get("claim_row_refs"), f"{binding_id}.claim_row_refs")
        ]
        if not claim_refs:
            add_finding(
                findings,
                "binding.claim_row_refs.empty",
                f"{binding_id} does not name a claim row",
                "Bind each proof corpus to at least one claim row or support row.",
                ref=binding_id,
            )

        asset_refs = [
            ensure_str(ref, f"{binding_id}.corpus_asset_refs[]")
            for ref in ensure_list(row.get("corpus_asset_refs"), f"{binding_id}.corpus_asset_refs")
        ]
        if not asset_refs:
            add_finding(
                findings,
                "binding.corpus_asset_refs.empty",
                f"{binding_id} does not name a corpus asset",
                "Bind every claim proof row to at least one governed corpus asset.",
                ref=binding_id,
            )

        blocked_assets: list[str] = []
        for asset_ref in asset_refs:
            asset = assets.get(asset_ref)
            if asset is None:
                add_finding(
                    findings,
                    "binding.unknown_asset",
                    f"{binding_id} cites unknown corpus asset: {asset_ref}",
                    "Use corpus_asset_id values from corpus_assets.",
                    ref=asset_ref,
                )
                continue
            privacy = ensure_dict(asset.get("privacy"), f"{asset_ref}.privacy")
            if not bool(privacy.get("approved_for_ci")) or not bool(privacy.get("approved_for_public_proof")):
                blocked_assets.append(asset_ref)

        if blocked_assets:
            add_finding(
                findings,
                "binding.asset_not_cleared",
                f"{binding_id} cites assets that are not cleared for CI/public proof",
                "Complete privacy/licensing intake or remove the asset from the claim binding.",
                ref=binding_id,
                details={"blocked_assets": blocked_assets},
            )

        proof_class = ensure_str(row.get("proof_class_id"), f"{binding_id}.proof_class_id")
        if proof_class not in proof_classes:
            add_finding(
                findings,
                "binding.unknown_proof_class",
                f"{binding_id} uses unknown proof_class_id {proof_class}",
                "Use a proof class declared in the freshness policy.",
                ref=proof_class,
            )

        propagation = ensure_str(
            row.get("stale_propagation_profile"),
            f"{binding_id}.stale_propagation_profile",
        )
        if propagation not in propagation_profiles:
            add_finding(
                findings,
                "binding.unknown_propagation_profile",
                f"{binding_id} uses unknown stale propagation profile {propagation}",
                "Use a stale propagation profile declared in the freshness policy.",
                ref=propagation,
            )

        evidence_ref = ensure_str(row.get("evidence_packet_ref"), f"{binding_id}.evidence_packet_ref")
        validate_path_ref(repo_root, evidence_ref, "binding.evidence_packet_ref", findings)
        for ref in ensure_list(row.get("supporting_artifact_refs"), f"{binding_id}.supporting_artifact_refs"):
            validate_path_ref(
                repo_root,
                ensure_str(ref, f"{binding_id}.supporting_artifact_refs[]"),
                "binding.supporting_artifact_refs",
                findings,
            )

        captured_at = ensure_str(row.get("evidence_captured_at"), f"{binding_id}.evidence_captured_at")
        captured = parse_instant(captured_at, f"{binding_id}.evidence_captured_at")
        if captured is None:
            add_finding(
                findings,
                "binding.evidence_captured_at.invalid",
                f"{binding_id} evidence_captured_at is not ISO-8601: {captured_at}",
                "Use an ISO-8601 UTC timestamp.",
                ref=binding_id,
            )
            age_days = None
        else:
            age_days = (today - captured.date()).days

        stale_after = ensure_str(row.get("stale_after"), f"{binding_id}.stale_after")
        stale_after_days = parse_stale_after_days(stale_after)
        if stale_after_days is None:
            add_finding(
                findings,
                "binding.stale_after.invalid",
                f"{binding_id} stale_after must use P<N>D syntax, got {stale_after!r}",
                "Use an ISO-8601 day duration such as P14D.",
                ref=binding_id,
            )

        declared_state = ensure_str(row.get("declared_claim_state"), f"{binding_id}.declared_claim_state")
        stale_state = ensure_str(row.get("stale_effective_state"), f"{binding_id}.stale_effective_state")
        if stale_state not in {"retest_pending", "blocked", "claim_narrowed"}:
            add_finding(
                findings,
                "binding.stale_effective_state.invalid",
                f"{binding_id} stale_effective_state must downgrade, got {stale_state}",
                "Use retest_pending, claim_narrowed, or blocked for stale proof.",
                ref=binding_id,
            )

        if age_days is None or stale_after_days is None:
            freshness_state = "missing"
            effective_state = stale_state
        elif age_days > stale_after_days:
            freshness_state = "stale"
            effective_state = stale_state
        elif age_days >= int(stale_after_days * 0.8):
            freshness_state = "near_expiry"
            effective_state = declared_state
        else:
            freshness_state = "current"
            effective_state = declared_state

        results.append(
            {
                "binding_id": binding_id,
                "claim_row_refs": claim_refs,
                "corpus_asset_refs": asset_refs,
                "evidence_packet_ref": evidence_ref,
                "proof_class_id": proof_class,
                "stale_propagation_profile": propagation,
                "evidence_captured_at": captured_at,
                "age_days": age_days,
                "stale_after_days": stale_after_days,
                "freshness_state": freshness_state,
                "declared_claim_state": declared_state,
                "effective_claim_state": effective_state,
                "stale_effective_state": stale_state,
            }
        )

    return results


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def normalized(text: str) -> str:
    def replace(match: re.Match[str]) -> str:
        value = match.group(0)
        if value.startswith('"generated_at"'):
            return '"generated_at": "__generated_at__"'
        if value.startswith('"evaluated_on"'):
            return '"evaluated_on": "__evaluated_on__"'
        return '"age_days": 0'

    return VOLATILE_REPORT_RE.sub(replace, text)


def write_report(path: Path, payload: dict[str, Any], check_only: bool) -> bool:
    path.parent.mkdir(parents=True, exist_ok=True)
    new_text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    old_text = path.read_text(encoding="utf-8") if path.exists() else None
    changed = old_text is None or normalized(old_text) != normalized(new_text)
    if not check_only:
        path.write_text(new_text, encoding="utf-8")
    return changed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    today = parse_today(args.today)
    registry = ensure_dict(render_yaml_as_json(repo_root / args.registry), "registry")
    policy = ensure_dict(render_yaml_as_json(repo_root / args.policy), "policy")

    findings: list[Finding] = []
    validate_registry_header(repo_root, registry, policy, findings)
    proof_classes, propagation_profiles = collect_policy_sets(policy)
    assets = validate_assets(repo_root, registry, findings)
    binding_results = validate_claim_bindings(
        repo_root,
        registry,
        assets,
        proof_classes,
        propagation_profiles,
        today,
        findings,
    )

    payload = {
        "schema_version": 1,
        "check_id": "corpus_freshness",
        "generated_at": now_iso_z(),
        "registry_ref": args.registry,
        "policy_ref": args.policy,
        "evaluated_on": today.isoformat(),
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "asset_count": len(assets),
        "binding_count": len(binding_results),
        "freshness_counts": {
            state: sum(1 for row in binding_results if row["freshness_state"] == state)
            for state in ("current", "near_expiry", "stale", "missing")
        },
        "binding_results": binding_results,
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [finding.as_report() for finding in findings],
    }

    changed = write_report(repo_root / args.report, payload, args.check)
    if args.check and changed:
        add_finding(
            findings,
            "report.stale",
            "checked-in corpus freshness report is stale",
            "Run `python3 ci/check_corpus_freshness.py --repo-root .` and commit the regenerated report.",
            ref=args.report,
        )
        payload["status"] = "fail"
        payload["finding_counts"] = {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        }
        payload["findings"] = [finding.as_report() for finding in findings]
        write_report(repo_root / args.report, payload, False)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[corpus-freshness] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- report: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[corpus-freshness] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[corpus-freshness]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[corpus-freshness] interrupted", file=sys.stderr)
        sys.exit(130)
