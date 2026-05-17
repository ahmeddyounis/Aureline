#!/usr/bin/env python3
"""Validate the beta reference-workspace corpus register."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/compat/m3/reference_workspace_register.yaml"
DEFAULT_CORPUS_REL = "fixtures/benchmarks/corpus_manifest.yaml"
DEFAULT_REFERENCE_ROWS_REL = "artifacts/compat/reference_workspace_rows.yaml"
DEFAULT_DOC_REL = "docs/compat/m3/reference_workspaces_beta.md"

REQUIRED_ROWS = {
    "m3_reference_workspace:jvm_service": {
        "reference_workspace_id": "refws.java_kotlin_service_archetype_seed",
        "archetype_row_ref": "archetype_row:java_or_kotlin_service",
        "scorecard": "artifacts/compat/m3/archetype_scorecards/java_or_kotlin_service.md",
    },
    "m3_reference_workspace:rust_workspace": {
        "reference_workspace_id": "refws.small_rust_self_host_slice",
        "archetype_row_ref": "archetype_row:rust_workspace",
        "scorecard": "artifacts/compat/m3/archetype_scorecards/rust_workspace.md",
    },
    "m3_reference_workspace:go_service": {
        "reference_workspace_id": "refws.go_service_archetype_seed",
        "archetype_row_ref": "archetype_row:go_service_or_monorepo_slice",
        "scorecard": "artifacts/compat/m3/archetype_scorecards/go_service_or_monorepo_slice.md",
    },
    "m3_reference_workspace:cpp_native": {
        "reference_workspace_id": "refws.c_cpp_native_archetype_seed",
        "archetype_row_ref": "archetype_row:c_or_cpp_native_project",
        "scorecard": "artifacts/compat/m3/archetype_scorecards/c_or_cpp_native_project.md",
    },
}

REQUIRED_OS_ARCH = {
    "macos_arm64",
    "macos_x86_64",
    "linux_x86_64",
    "windows_x86_64",
}
REQUIRED_WORKFLOW_CLASSES = {
    "benchmark",
    "run",
    "test",
    "debug",
    "migration",
    "supportability",
}
REQUIRED_CONSUMER_GROUPS = {
    "benchmark",
    "run_test_debug",
    "migration",
    "supportability",
    "release_evidence",
}
PATH_LIKE_SUFFIXES = (
    ".yaml",
    ".yml",
    ".json",
    ".md",
    ".toml",
    ".rs",
    ".py",
)
ID_PREFIXES = (
    "archetype_row:",
    "corpus.",
    "lane:",
    "launch_bundle:",
    "m3_reference_workspace:",
    "refws.",
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
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--corpus", default=DEFAULT_CORPUS_REL)
    parser.add_argument("--reference-rows", default=DEFAULT_REFERENCE_ROWS_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--report", default=None)
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
                "require 'date'; "
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Date, Time], aliases: false); "
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


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if looks_like_path(ref) and not (repo_root / strip_fragment(ref)).exists():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} references a missing artifact: {ref}",
                remediation="Seed the referenced artifact or correct the path.",
                ref=ref,
            )
        )


def parse_iso_date(value: str, label: str, findings: list[Finding], ref: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_date",
                message=f"{label} must be YYYY-MM-DD, got {value!r}",
                remediation="Use an ISO date without time.",
                ref=ref,
            )
        )


def collect_corpus_ids(corpus: dict[str, Any]) -> set[str]:
    return {
        ensure_str(ensure_dict(row, "corpus.fixtures[]").get("id"), "corpus.fixtures[].id")
        for row in ensure_list(corpus.get("fixtures"), "corpus.fixtures")
    }


def collect_reference_row_refs(reference_rows: dict[str, Any]) -> dict[str, set[str]]:
    rows: dict[str, set[str]] = {}
    for raw in ensure_list(reference_rows.get("archetype_rows"), "reference_rows.archetype_rows"):
        row = ensure_dict(raw, "reference_rows.archetype_rows[]")
        row_id = ensure_str(row.get("archetype_row_id"), "archetype_row_id")
        refs = {ensure_str(ref, f"{row_id}.reference_workspace_refs[]") for ref in ensure_list(row.get("reference_workspace_refs"), f"{row_id}.reference_workspace_refs")}
        rows[row_id] = refs
    return rows


def validate_packet(
    repo_root: Path,
    row_id: str,
    row: dict[str, Any],
    packet: dict[str, Any],
    harness: dict[str, Any],
    findings: list[Finding],
) -> None:
    if ensure_str(packet.get("register_row_ref"), "packet.register_row_ref") != row_id:
        findings.append(
            Finding(
                severity="error",
                check_id="packet.register_row_ref",
                message="workspace packet does not point back to its register row",
                remediation="Set packet.register_row_ref to the owning register row.",
                ref=row_id,
            )
        )
    reference_workspace_id = ensure_str(row.get("reference_workspace_id"), f"{row_id}.reference_workspace_id")
    if ensure_str(packet.get("reference_workspace_id"), "packet.reference_workspace_id") != reference_workspace_id:
        findings.append(
            Finding(
                severity="error",
                check_id="packet.reference_workspace_id",
                message="workspace packet reference_workspace_id does not match the register row",
                remediation="Use one reference workspace id across register, packet, and harness.",
                ref=row_id,
            )
        )
    if ensure_str(harness.get("reference_workspace_id"), "harness.reference_workspace_id") != reference_workspace_id:
        findings.append(
            Finding(
                severity="error",
                check_id="harness.reference_workspace_id",
                message="harness reference_workspace_id does not match the register row",
                remediation="Use one reference workspace id across register, packet, and harness.",
                ref=row_id,
            )
        )

    packet_workflows = ensure_list(packet.get("protected_workflows"), "packet.protected_workflows")
    workflow_classes = {
        ensure_str(ensure_dict(workflow, "packet.protected_workflows[]").get("workflow_class"), "workflow_class")
        for workflow in packet_workflows
    }
    missing_classes = REQUIRED_WORKFLOW_CLASSES - workflow_classes
    if missing_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="packet.workflow_classes.missing",
                message="workspace packet is missing required workflow classes",
                remediation="Cover benchmark, run, test, debug, migration, and supportability.",
                ref=row_id,
                details={"missing": sorted(missing_classes)},
            )
        )

    workflow_ids = {
        ensure_str(ensure_dict(workflow, "packet.protected_workflows[]").get("workflow_id"), "workflow_id")
        for workflow in packet_workflows
    }
    harness_entries = ensure_list(harness.get("harness_entries"), "harness.harness_entries")
    harness_workflow_ids: set[str] = set()
    harness_entry_ids: set[str] = set()
    result_vocabulary = set(ensure_list(harness.get("result_vocabulary"), "harness.result_vocabulary"))
    for raw_entry in harness_entries:
        entry = ensure_dict(raw_entry, "harness.harness_entries[]")
        harness_entry_ids.add(ensure_str(entry.get("harness_entry_id"), "harness_entry_id"))
        harness_workflow_ids.add(ensure_str(entry.get("workflow_id"), "workflow_id"))
        if ensure_str(entry.get("workflow_class"), "workflow_class") not in REQUIRED_WORKFLOW_CLASSES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="harness.workflow_class.invalid",
                    message="harness entry uses a workflow class outside the beta vocabulary",
                    remediation="Use the register workflow_class_vocabulary.",
                    ref=row_id,
                )
            )
        if ensure_str(entry.get("expected_outcome"), "expected_outcome") != "pass":
            findings.append(
                Finding(
                    severity="error",
                    check_id="harness.expected_outcome.not_pass",
                    message="each seeded harness entry must define its pass condition",
                    remediation="Set expected_outcome to pass and use latest_result for current run state.",
                    ref=row_id,
                )
            )
        latest_result = ensure_str(entry.get("latest_result"), "latest_result")
        if latest_result not in result_vocabulary:
            findings.append(
                Finding(
                    severity="error",
                    check_id="harness.latest_result.invalid",
                    message=f"harness latest_result {latest_result!r} is not in result_vocabulary",
                    remediation="Use a declared harness result value.",
                    ref=row_id,
                )
            )
        for fixture_ref in ensure_list(entry.get("fixture_refs"), "fixture_refs"):
            validate_path_ref(repo_root, ensure_str(fixture_ref, "fixture_refs[]"), "harness.fixture_refs", findings)

    missing_harness_workflows = workflow_ids - harness_workflow_ids
    if missing_harness_workflows:
        findings.append(
            Finding(
                severity="error",
                check_id="harness.workflow_ids.missing",
                message="harness does not cover every packet workflow",
                remediation="Add one harness entry for every protected_workflows item.",
                ref=row_id,
                details={"missing": sorted(missing_harness_workflows)},
            )
        )
    for raw_workflow in packet_workflows:
        workflow = ensure_dict(raw_workflow, "packet.protected_workflows[]")
        required_entry = ensure_str(workflow.get("required_harness_entry_ref"), "required_harness_entry_ref")
        if required_entry not in harness_entry_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="packet.required_harness_entry_ref.unknown",
                    message=f"{required_entry} is not present in the harness",
                    remediation="Use a harness_entry_id from the workspace harness.",
                    ref=row_id,
                )
            )


def validate_register(
    repo_root: Path,
    register: dict[str, Any],
    corpus_ids: set[str],
    reference_row_refs: dict[str, set[str]],
    doc_text: str,
    findings: list[Finding],
) -> None:
    if ensure_int(register.get("schema_version"), "register.schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="register.schema_version",
                message="schema_version must be 1",
                remediation="Update this validator with any schema bump.",
            )
        )
    parse_iso_date(ensure_str(register.get("as_of"), "register.as_of"), "register.as_of", findings, "register")
    ensure_str(register.get("owner"), "register.owner")

    required_classes = set(ensure_list(register.get("required_workflow_classes"), "register.required_workflow_classes"))
    missing_required_classes = REQUIRED_WORKFLOW_CLASSES - required_classes
    if missing_required_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="register.required_workflow_classes.missing",
                message="register does not declare every required beta workflow class",
                remediation="Declare benchmark, run, test, debug, migration, and supportability.",
                details={"missing": sorted(missing_required_classes)},
            )
        )

    rows = ensure_list(register.get("reference_workspaces"), "register.reference_workspaces")
    seen_rows: set[str] = set()
    for raw_row in rows:
        row = ensure_dict(raw_row, "register.reference_workspaces[]")
        row_id = ensure_str(row.get("register_row_id"), "register_row_id")
        seen_rows.add(row_id)
        expected = REQUIRED_ROWS.get(row_id)
        if expected is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.row.unknown",
                    message=f"unexpected beta reference workspace row {row_id}",
                    remediation="Use the four beta corpus rows declared by this validator.",
                    ref=row_id,
                )
            )
            continue

        for field_name in ("reference_workspace_id", "archetype_row_ref"):
            actual = ensure_str(row.get(field_name), f"{row_id}.{field_name}")
            if actual != expected[field_name]:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"register.{field_name}.mismatch",
                        message=f"{row_id} has {field_name}={actual}, expected {expected[field_name]}",
                        remediation="Bind the row to the canonical beta workspace and archetype.",
                        ref=row_id,
                    )
                )

        descriptor_ref = ensure_str(row.get("workspace_descriptor_ref"), f"{row_id}.workspace_descriptor_ref")
        validate_path_ref(repo_root, descriptor_ref, "workspace_descriptor_ref", findings)
        descriptor_path = repo_root / strip_fragment(descriptor_ref)
        if descriptor_path.exists():
            descriptor = json.loads(descriptor_path.read_text(encoding="utf-8"))
            if descriptor.get("reference_workspace_id") != expected["reference_workspace_id"]:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.reference_workspace_id",
                        message="descriptor reference_workspace_id does not match the register row",
                        remediation="Fix the descriptor or register row.",
                        ref=descriptor_ref,
                    )
                )

        packet_ref = ensure_str(row.get("workspace_packet_ref"), f"{row_id}.workspace_packet_ref")
        harness_ref = ensure_str(row.get("harness_ref"), f"{row_id}.harness_ref")
        validate_path_ref(repo_root, packet_ref, "workspace_packet_ref", findings)
        validate_path_ref(repo_root, harness_ref, "harness_ref", findings)
        packet = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(packet_ref)), "workspace_packet")
        harness = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(harness_ref)), "harness")
        validate_packet(repo_root, row_id, row, packet, harness, findings)

        owner = ensure_dict(row.get("owner"), f"{row_id}.owner")
        for field_name in (
            "owner_dri",
            "evidence_owner_ref",
            "publication_owner_ref",
            "privacy_reviewer_ref",
            "backup_owner_ref_or_waiver",
        ):
            ensure_str(owner.get(field_name), f"{row_id}.owner.{field_name}")

        toolchain = ensure_dict(row.get("toolchain_manifest"), f"{row_id}.toolchain_manifest")
        exact_versions = ensure_dict(toolchain.get("exact_versions"), f"{row_id}.toolchain_manifest.exact_versions")
        if not exact_versions:
            findings.append(
                Finding(
                    severity="error",
                    check_id="toolchain.exact_versions.empty",
                    message="toolchain_manifest.exact_versions must name at least one pinned tool",
                    remediation="Declare exact toolchain versions for the workspace.",
                    ref=row_id,
                )
            )
        coverage = set(ensure_list(toolchain.get("os_arch_coverage_targets"), f"{row_id}.toolchain_manifest.os_arch_coverage_targets"))
        missing_os_arch = REQUIRED_OS_ARCH - coverage
        if missing_os_arch:
            findings.append(
                Finding(
                    severity="error",
                    check_id="toolchain.os_arch_coverage.missing",
                    message="toolchain manifest is missing required OS/arch coverage targets",
                    remediation="Name macOS arm64/x86_64, Linux x86_64, and Windows x86_64.",
                    ref=row_id,
                    details={"missing": sorted(missing_os_arch)},
                )
            )
        supported_modes = set(ensure_list(toolchain.get("supported_modes"), f"{row_id}.toolchain_manifest.supported_modes"))
        if "local_only" not in supported_modes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="toolchain.supported_modes.local_missing",
                    message="beta reference workspace must declare local_only support",
                    remediation="Add local_only to supported_modes or remove the row.",
                    ref=row_id,
                )
            )

        privacy = ensure_dict(row.get("privacy_license"), f"{row_id}.privacy_license")
        parse_iso_date(ensure_str(privacy.get("reviewed_on"), f"{row_id}.privacy_license.reviewed_on"), "privacy_license.reviewed_on", findings, row_id)
        for field_name in ("source_class", "privacy_class", "privacy_decision", "license_status", "note_ref"):
            ensure_str(privacy.get(field_name), f"{row_id}.privacy_license.{field_name}")
        validate_path_ref(repo_root, ensure_str(privacy.get("note_ref"), f"{row_id}.privacy_license.note_ref"), "privacy_license.note_ref", findings)

        corpus_refs = set(ensure_list(row.get("corpus_refs"), f"{row_id}.corpus_refs"))
        unknown_corpus = corpus_refs - corpus_ids
        if unknown_corpus:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.corpus_refs.unknown",
                    message="register row cites corpus refs missing from the protected corpus manifest",
                    remediation="Add the corpus rows or correct the register refs.",
                    ref=row_id,
                    details={"unknown": sorted(unknown_corpus)},
                )
            )

        archetype_ref = ensure_str(row.get("archetype_row_ref"), f"{row_id}.archetype_row_ref")
        reference_refs = reference_row_refs.get(archetype_ref, set())
        if expected["reference_workspace_id"] not in reference_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="reference_rows.workspace_ref_missing",
                    message="compat reference workspace row does not cite the materialised reference workspace id",
                    remediation="Update artifacts/compat/reference_workspace_rows.yaml to use the materialised refws id.",
                    ref=archetype_ref,
                    details={"expected": expected["reference_workspace_id"], "actual": sorted(reference_refs)},
                )
            )

        consumers = ensure_dict(row.get("consumer_refs"), f"{row_id}.consumer_refs")
        missing_consumers = REQUIRED_CONSUMER_GROUPS - set(consumers)
        if missing_consumers:
            findings.append(
                Finding(
                    severity="error",
                    check_id="register.consumer_refs.missing",
                    message="register row does not bind every required consumer group",
                    remediation="Bind benchmark, run/test/debug, migration, supportability, and release evidence.",
                    ref=row_id,
                    details={"missing": sorted(missing_consumers)},
                )
            )
        for group, refs in consumers.items():
            for ref in ensure_list(refs, f"{row_id}.consumer_refs.{group}"):
                validate_path_ref(repo_root, ensure_str(ref, f"{row_id}.consumer_refs.{group}[]"), f"consumer_refs.{group}", findings)

        scorecard_path = repo_root / expected["scorecard"]
        validate_path_ref(repo_root, expected["scorecard"], "scorecard", findings)
        if scorecard_path.exists():
            scorecard_text = scorecard_path.read_text(encoding="utf-8")
            if f"{DEFAULT_REGISTER_REL}#{row_id}" not in scorecard_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scorecard.missing_register_ref",
                        message="archetype scorecard does not cite the beta reference-workspace register row",
                        remediation="Add the register row to the scorecard evidence_refs.",
                        ref=expected["scorecard"],
                    )
                )
        if expected["reference_workspace_id"] not in doc_text or row_id not in doc_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="docs.missing_workspace",
                    message="reference-workspace beta doc does not mention the register row and reference workspace id",
                    remediation="Document every beta workspace row in docs/compat/m3/reference_workspaces_beta.md.",
                    ref=row_id,
                )
            )

    missing_rows = set(REQUIRED_ROWS) - seen_rows
    if missing_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="register.rows.missing_required",
                message="register is missing required beta reference workspace rows",
                remediation="Seed JVM, Rust, Go, and C/C++ rows.",
                details={"missing": sorted(missing_rows)},
            )
        )


def validate_acceptance(register: dict[str, Any], findings: list[Finding]) -> None:
    coverage = ensure_list(register.get("acceptance_state_coverage"), "register.acceptance_state_coverage")
    states = {
        ensure_str(ensure_dict(row, "acceptance_state_coverage[]").get("exercises_state"), "exercises_state")
        for row in coverage
    }
    for required in (
        "owner_privacy_toolchain_workflow_harness_present",
        "compatibility_scorecard_release_consumers_bound",
    ):
        if required not in states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="acceptance_state_coverage.missing",
                    message=f"acceptance_state_coverage is missing {required}",
                    remediation="Add a coverage row for the acceptance state.",
                    ref=required,
                )
            )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(render_yaml_as_json(repo_root / args.register), "register")
    corpus = ensure_dict(render_yaml_as_json(repo_root / args.corpus), "corpus")
    reference_rows = ensure_dict(render_yaml_as_json(repo_root / args.reference_rows), "reference_rows")
    doc_path = repo_root / args.doc
    if not doc_path.exists():
        raise SystemExit(f"missing doc file: {doc_path}")
    doc_text = doc_path.read_text(encoding="utf-8")

    findings: list[Finding] = []
    validate_register(
        repo_root=repo_root,
        register=register,
        corpus_ids=collect_corpus_ids(corpus),
        reference_row_refs=collect_reference_row_refs(reference_rows),
        doc_text=doc_text,
        findings=findings,
    )
    validate_acceptance(register, findings)

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    warnings = [item for item in findings if item.severity == "warning"]
    for item in warnings:
        ref = f" ({item.ref})" if item.ref else ""
        print(f"WARNING [{item.check_id}]{ref}: {item.message}")

    print("beta reference-workspace register validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
