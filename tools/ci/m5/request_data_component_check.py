#!/usr/bin/env python3
"""M5 request/data reusable-component certification gate.

This gate validates the checked-in certification bundle for reusable request
and data tooling components: request editor/header primitives, response tabsets,
connection browser rows, result grids, query history, and explain-plan panes.
It intentionally checks the packet as a bundle because M5 claim-bearing
surfaces consume these components by reference.

Exit codes:

- 0 -- bundle is clean.
- 1 -- one or more blocking findings.
- 2 -- usage error or missing input file.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

MATRIX_REL = Path("artifacts/design/m5-request-data-component-matrix.md")
PROOF_REL = Path("artifacts/release/m5-request-data-component-proof/proof_packet.json")
SUPPORT_REL = Path("artifacts/release/m5-request-data-component-proof/support_export.json")
PROOF_README_REL = Path("artifacts/release/m5-request-data-component-proof/README.md")
MANIFEST_REL = Path("fixtures/ui/m5-request-data-components/component_manifest.json")
FIXTURE_README_REL = Path("fixtures/ui/m5-request-data-components/README.md")
REQUEST_DOC_REL = Path("docs/request-workspace/m5-request-editor-auth-primitives.md")
BROWSER_DOC_REL = Path("docs/browser-runtime/m5-request-send-surface-projection.md")
DATA_DOC_REL = Path("docs/data/database_tooling_contract.md")

EXPECTED_PROOF_RECORD_KIND = "m5_request_data_component_proof"
EXPECTED_SUPPORT_RECORD_KIND = "m5_request_data_component_support_export"
EXPECTED_MANIFEST_RECORD_KIND = "m5_request_data_component_fixture_manifest"
EXPECTED_SCHEMA_VERSION = 1

REQUIRED_FAMILIES = (
    "request_editor_header",
    "environment_picker",
    "variable_resolution_inspector",
    "auth_sheet",
    "response_tabset",
    "request_history_row",
    "contract_source_badge",
    "connection_picker_row",
    "schema_object_row",
    "sql_run_bar",
    "result_grid",
    "query_history_row",
    "explain_plan_pane",
)

REQUIRED_PARITY_CHECKS = (
    "required_field_parity",
    "controlled_vocabulary_parity",
    "reduced_capability_disclosure",
    "secret_redaction_parity",
    "copy_export_parity",
    "estimated_actual_plan_truth",
    "accessibility_parity",
    "machine_readable_support_join_parity",
    "truth_auto_narrowing",
)

REQUIRED_NARROWING_TRIGGERS = (
    "auth_source_class",
    "origin_boundary",
    "schema_freshness",
    "plan_freshness",
    "export_redaction_posture",
)

REQUIRED_REASON_PROJECTIONS = ("gui", "cli_headless", "support_export", "release_proof")

REQUIRED_DOC_BACKLINKS = (
    MATRIX_REL.as_posix(),
    PROOF_REL.as_posix(),
    SUPPORT_REL.as_posix(),
    MANIFEST_REL.as_posix(),
    "tools/ci/m5/request_data_component_check.py",
)

REQUIRED_MATRIX_TOKENS = (
    "Request Editor Header",
    "Response Tab Set",
    "Connection Picker Row",
    "Result Grid",
    "Explain-Plan Pane",
    "truth_auto_narrowing",
    "machine_readable_support_join_parity",
)

REQUIRED_DATA_DOC_TOKENS = (
    MATRIX_REL.as_posix(),
    PROOF_REL.as_posix(),
    SUPPORT_REL.as_posix(),
    "m5-result-grid.schema.json",
    "m5-explain-plan-pane.schema.json",
)


@dataclass
class Finding:
    code: str
    message: str
    subject: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject is not None:
            out["subject"] = self.subject
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Path to repository root.")
    parser.add_argument("--format", choices=("text", "json"), default="text")
    parser.add_argument(
        "--now",
        default=None,
        help="Override current UTC time for freshness checks, e.g. 2026-07-03T12:00:00Z.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def read_text(path: Path) -> str:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
    return path.read_text(encoding="utf-8")


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def parse_utc(value: str) -> datetime:
    normalized = value.replace("Z", "+00:00")
    parsed = datetime.fromisoformat(normalized)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def maybe_validate_schema(repo_root: Path, schema_path: Path, fixture_path: Path, findings: list[Finding]) -> None:
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except ImportError:
        return

    schema = load_json(repo_root / schema_path)
    fixture = load_json(repo_root / fixture_path)
    try:
        Draft202012Validator.check_schema(schema)
    except Exception as exc:  # pragma: no cover - jsonschema exception shape varies
        findings.append(Finding("schema_invalid", str(exc), subject=schema_path.as_posix()))
        return

    validator = Draft202012Validator(schema)
    if fixture_path.name == "schema_object_rows.json":
        rows = ensure_list(ensure_dict(fixture, fixture_path.as_posix()).get("rows", []), "schema_object_rows.rows")
        for idx, row in enumerate(rows):
            candidate = dict(ensure_dict(row, f"schema_object_rows.rows[{idx}]"))
            candidate["reduced_capability_banner"] = fixture.get("reduced_capability_banner")
            candidate["provider_handoff_notes"] = fixture.get("provider_handoff_notes")
            for error in sorted(validator.iter_errors(candidate), key=lambda e: list(e.path)):
                findings.append(
                    Finding(
                        "fixture_schema_violation",
                        error.message,
                        subject=f"{fixture_path.as_posix()}#{idx}",
                        detail={"path": "/".join(str(p) for p in error.path)},
                    )
                )
        return

    for error in sorted(validator.iter_errors(fixture), key=lambda e: list(e.path)):
        findings.append(
            Finding(
                "fixture_schema_violation",
                error.message,
                subject=fixture_path.as_posix(),
                detail={"path": "/".join(str(p) for p in error.path)},
            )
        )


def check_envelope(proof: dict[str, Any], support: dict[str, Any], manifest: dict[str, Any], findings: list[Finding]) -> None:
    expected = (
        (proof, EXPECTED_PROOF_RECORD_KIND, "proof_packet"),
        (support, EXPECTED_SUPPORT_RECORD_KIND, "support_export"),
        (manifest, EXPECTED_MANIFEST_RECORD_KIND, "component_manifest"),
    )
    for obj, record_kind, label in expected:
        if obj.get("record_kind") != record_kind:
            findings.append(Finding("record_kind_mismatch", f"{label} has wrong record_kind", subject=label, detail={"record_kind": obj.get("record_kind")}))
        if obj.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            findings.append(Finding("schema_version_mismatch", f"{label} has wrong schema_version", subject=label, detail={"schema_version": obj.get("schema_version")}))

    if proof.get("matrix_ref") != MATRIX_REL.as_posix():
        findings.append(Finding("matrix_ref_mismatch", "proof packet must point at the design matrix", subject="proof_packet"))
    if support.get("matrix_ref") != MATRIX_REL.as_posix() or support.get("proof_packet_ref") != PROOF_REL.as_posix():
        findings.append(Finding("support_ref_mismatch", "support export must point at matrix and proof packet", subject="support_export"))
    if manifest.get("matrix_ref") != MATRIX_REL.as_posix():
        findings.append(Finding("manifest_ref_mismatch", "fixture manifest must point at the design matrix", subject="component_manifest"))


def check_freshness(proof: dict[str, Any], support: dict[str, Any], now: datetime, findings: list[Finding]) -> None:
    proof_freshness = ensure_dict(proof.get("proof_freshness", {}), "proof_freshness")
    support_freshness = ensure_dict(support.get("freshness_and_parity", {}), "freshness_and_parity")
    for label, freshness in (("proof_packet", proof_freshness), ("support_export", support_freshness)):
        if freshness.get("proof_fresh") is not True:
            findings.append(Finding("proof_not_fresh", "proof bundle must be marked fresh or auto-narrowed", subject=label))
        if freshness.get("auto_narrow_on_stale") is not True:
            findings.append(Finding("stale_not_auto_narrowed", "stale proof must auto-narrow claims", subject=label))
        if label == "proof_packet" and freshness.get("stale_failure_effect") != "narrow_claim":
            findings.append(Finding("stale_effect_mismatch", "stale proof must narrow claims", subject=label))
        try:
            last_refresh = parse_utc(str(freshness["last_refresh"]))
            slo_hours = int(freshness["freshness_slo_hours"])
        except (KeyError, TypeError, ValueError) as exc:
            findings.append(Finding("freshness_metadata_invalid", str(exc), subject=label))
            continue
        age_hours = (now - last_refresh).total_seconds() / 3600
        if age_hours > slo_hours and freshness.get("proof_fresh") is True:
            findings.append(
                Finding(
                    "proof_freshness_expired",
                    "proof is older than its freshness SLO but still marked fresh",
                    subject=label,
                    detail={"age_hours": round(age_hours, 2), "slo_hours": slo_hours},
                )
            )


def index_by_family(rows: list[Any], label: str, findings: list[Finding]) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for row in rows:
        row = ensure_dict(row, label)
        family = row.get("family")
        if not isinstance(family, str):
            findings.append(Finding("family_missing", "row is missing family", subject=label))
            continue
        if family in out:
            findings.append(Finding("family_duplicate", "family appears more than once", subject=family, detail={"source": label}))
        out[family] = row
    return out


def check_family_coverage(repo_root: Path, proof: dict[str, Any], support: dict[str, Any], manifest: dict[str, Any], findings: list[Finding]) -> None:
    proof_rows = index_by_family(ensure_list(proof.get("component_families", []), "proof.component_families"), "proof row", findings)
    support_rows = index_by_family(ensure_list(support.get("rows", []), "support.rows"), "support row", findings)
    manifest_rows = index_by_family(ensure_list(manifest.get("fixtures", []), "manifest.fixtures"), "manifest row", findings)

    for source, rows in (("proof_packet", proof_rows), ("support_export", support_rows), ("component_manifest", manifest_rows)):
        missing = sorted(set(REQUIRED_FAMILIES) - set(rows))
        extra = sorted(set(rows) - set(REQUIRED_FAMILIES))
        if missing:
            findings.append(Finding("family_coverage_gap", "source is missing required families", subject=source, detail={"missing": missing}))
        if extra:
            findings.append(Finding("family_unknown", "source includes unknown families", subject=source, detail={"extra": extra}))

    for family in REQUIRED_FAMILIES:
        proof_row = proof_rows.get(family)
        support_row = support_rows.get(family)
        manifest_row = manifest_rows.get(family)
        if not proof_row or not support_row or not manifest_row:
            continue
        comparable_fields = (
            ("schema_ref", "schema_ref", "machine_readable_schema_ref"),
            ("fixture_ref", "fixture_ref", "fixture_ref"),
            ("reduced_capability_banner_ref", "reduced_capability_banner_ref", "reduced_capability_banner_ref"),
            ("provider_handoff_note_refs", "provider_handoff_note_refs", "provider_handoff_note_refs"),
            ("support_export_join_id", "support_export_join.join_id", "support_export_join_id"),
        )
        for proof_key, manifest_key, support_key in comparable_fields:
            manifest_value: Any
            if "." in manifest_key:
                root, nested = manifest_key.split(".", 1)
                manifest_value = ensure_dict(manifest_row.get(root, {}), f"{family}.{root}").get(nested)
            else:
                manifest_value = manifest_row.get(manifest_key)
            if proof_row.get(proof_key) != manifest_value:
                findings.append(Finding("proof_manifest_drift", "proof row differs from component manifest", subject=family, detail={"field": proof_key}))
            if support_row.get(support_key) != proof_row.get(proof_key):
                findings.append(Finding("support_proof_drift", "support row differs from proof row", subject=family, detail={"field": support_key}))

        proof_consumers = set(proof_row.get("consumer_surfaces") or [])
        manifest_consumers = set(manifest_row.get("claimed_consumers") or [])
        if proof_consumers != manifest_consumers:
            findings.append(
                Finding(
                    "consumer_surface_drift",
                    "claimed consumers differ between proof and manifest",
                    subject=family,
                    detail={"proof": sorted(proof_consumers), "manifest": sorted(manifest_consumers)},
                )
            )

        triggers = tuple(proof_row.get("auto_narrowing_triggers") or [])
        manifest_triggers = tuple(ensure_dict(manifest_row.get("auto_narrowing_contract", {}), f"{family}.auto_narrowing_contract").get("narrow_on_missing_or_stale") or [])
        if set(triggers) != set(REQUIRED_NARROWING_TRIGGERS) or set(manifest_triggers) != set(REQUIRED_NARROWING_TRIGGERS):
            findings.append(Finding("auto_narrowing_trigger_gap", "family must carry all required auto-narrowing triggers", subject=family))
        if set(proof_row.get("narrowed_claim_reason_projection") or []) != set(REQUIRED_REASON_PROJECTIONS):
            findings.append(Finding("narrowed_reason_projection_gap", "proof must project narrowed reasons to GUI, CLI/headless, support, and release", subject=family))

        if support_row.get("gui_cli_support_label_parity") is not True:
            findings.append(Finding("support_label_parity_missing", "support row must preserve GUI/CLI/support labels", subject=family))
        if support_row.get("raw_secret_exported") is not False or support_row.get("raw_rows_exported_by_default") is not False:
            findings.append(Finding("raw_material_exported", "support export must not export raw secrets or rows by default", subject=family))

        accessibility = ensure_dict(manifest_row.get("accessibility_contract", {}), f"{family}.accessibility_contract")
        if set(accessibility.get("first_consumers_verified") or []) != manifest_consumers:
            findings.append(Finding("accessibility_consumer_gap", "accessibility contract must cover every claimed consumer", subject=family))
        support_join = ensure_dict(manifest_row.get("support_export_join", {}), f"{family}.support_export_join")
        if support_join.get("preserve_gui_labels_in_cli_and_support") is not True:
            findings.append(Finding("join_label_parity_missing", "support join must preserve GUI labels in CLI/support", subject=family))
        if support_join.get("raw_secrets_exported") is not False or support_join.get("raw_rows_exported_by_default") is not False:
            findings.append(Finding("join_raw_material_exported", "support join must exclude raw secrets and rows by default", subject=family))

        schema_path = Path(proof_row["schema_ref"])
        fixture_path = Path(proof_row["fixture_ref"])
        maybe_validate_schema(repo_root, schema_path, fixture_path, findings)


def check_parity_checks(proof: dict[str, Any], support: dict[str, Any], findings: list[Finding]) -> None:
    proof_checks = {row.get("check_id") for row in ensure_list(proof.get("parity_checks", []), "proof.parity_checks") if isinstance(row, dict)}
    support_checks = set(ensure_dict(support.get("freshness_and_parity", {}), "freshness_and_parity").get("narrowing_checks") or [])
    missing = sorted(set(REQUIRED_PARITY_CHECKS) - proof_checks)
    if missing:
        findings.append(Finding("proof_parity_check_gap", "proof packet is missing parity checks", detail={"missing": missing}))
    missing_support = sorted(set(REQUIRED_PARITY_CHECKS) - support_checks)
    if missing_support:
        findings.append(Finding("support_parity_check_gap", "support export is missing narrowing checks", detail={"missing": missing_support}))
    for row in ensure_list(proof.get("parity_checks", []), "proof.parity_checks"):
        row = ensure_dict(row, "proof.parity_checks row")
        if row.get("check_id") in REQUIRED_PARITY_CHECKS and not str(row.get("failure_effect", "")).startswith(("narrow", "block")):
            findings.append(Finding("parity_failure_effect_invalid", "parity failure must narrow or block", subject=row.get("check_id")))


def check_docs(repo_root: Path, findings: list[Finding]) -> None:
    docs = {
        "matrix": read_text(repo_root / MATRIX_REL),
        "proof_readme": read_text(repo_root / PROOF_README_REL),
        "fixture_readme": read_text(repo_root / FIXTURE_README_REL),
        "request_doc": read_text(repo_root / REQUEST_DOC_REL),
        "browser_doc": read_text(repo_root / BROWSER_DOC_REL),
        "data_doc": read_text(repo_root / DATA_DOC_REL),
    }
    for token in REQUIRED_MATRIX_TOKENS:
        if token not in docs["matrix"]:
            findings.append(Finding("matrix_token_missing", "matrix is missing required component/gate token", subject=token))
    for label in ("proof_readme", "fixture_readme"):
        for token in REQUIRED_DOC_BACKLINKS[:4]:
            if token not in docs[label]:
                findings.append(Finding("doc_backlink_missing", "doc is missing certification bundle backlink", subject=f"{label}:{token}"))
    for label in ("request_doc", "browser_doc", "data_doc"):
        for token in (MATRIX_REL.as_posix(), PROOF_REL.as_posix()):
            if token not in docs[label]:
                findings.append(Finding("doc_cert_ref_missing", "consumer doc must reference matrix and proof packet", subject=f"{label}:{token}"))
    for token in REQUIRED_DATA_DOC_TOKENS:
        if token not in docs["data_doc"]:
            findings.append(Finding("data_doc_token_missing", "data tooling doc must reference result/plan certification", subject=token))


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    now = parse_utc(args.now) if args.now else datetime.now(timezone.utc)

    proof = ensure_dict(load_json(repo_root / PROOF_REL), PROOF_REL.as_posix())
    support = ensure_dict(load_json(repo_root / SUPPORT_REL), SUPPORT_REL.as_posix())
    manifest = ensure_dict(load_json(repo_root / MANIFEST_REL), MANIFEST_REL.as_posix())

    findings: list[Finding] = []
    check_envelope(proof, support, manifest, findings)
    check_freshness(proof, support, now, findings)
    check_parity_checks(proof, support, findings)
    check_family_coverage(repo_root, proof, support, manifest, findings)
    check_docs(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"status": "fail" if findings else "pass", "findings": [f.as_dict() for f in findings]}, indent=2))
    elif findings:
        print(f"M5 request/data component check failed with {len(findings)} finding(s):", file=sys.stderr)
        for finding in findings:
            subject = f" [{finding.subject}]" if finding.subject else ""
            detail = f" {json.dumps(finding.detail, sort_keys=True)}" if finding.detail else ""
            print(f"- {finding.code}{subject}: {finding.message}{detail}", file=sys.stderr)
    else:
        print("M5 request/data component certification bundle is clean.")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
