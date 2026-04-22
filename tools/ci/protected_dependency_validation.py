#!/usr/bin/env python3
"""Validate protected dependency classes and hot-path blocking-I/O sentinels."""

from __future__ import annotations

import datetime as dt
import json
import re
import subprocess
import tomllib
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

RULES_REL = "artifacts/architecture/protected_path_dependency_rules.yaml"
PROCESS_MAP_REL = "artifacts/architecture/process_placement_map.yaml"
OVERVIEW_DOC_REL = "docs/architecture/service_topology_and_process_placement.md"
PACKAGE_INVENTORY_REL = "artifacts/governance/package_inventory.yaml"

REQUIRED_PLANE_IDS = {
    "shell_ui",
    "renderer",
    "text_buffer",
    "vfs_watchers",
    "index_search",
    "task_execution",
    "remote_helper",
    "ai_control_plane",
    "updater_release",
    "support_diagnostics",
}

REQUIRED_PROCESS_ROLE_IDS = {
    "desktop_shell_process",
    "local_supervisor",
    "knowledge_worker_group",
    "execution_helper_group",
    "remote_connector",
    "ai_runtime",
    "updater_installer",
    "support_collector",
}

LOCAL_IMPORT_RE = re.compile(r"(?m)^\s*use\s+(?:crate|super)::([A-Za-z_][A-Za-z0-9_]*)")


@dataclass
class Finding:
    severity: str
    check_id: str
    artifact_ref: str
    owner_artifact_ref: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if not payload["details"]:
            payload.pop("details")
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        return payload


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def item_ref(row_id: str, rel: str) -> str:
    return f"{rel}#{row_id}"


def make_finding(
    severity: str,
    check_id: str,
    artifact_ref: str,
    owner_artifact_ref: str,
    message: str,
    remediation: str,
    row_ref: str | None = None,
    **details: Any,
) -> Finding:
    return Finding(
        severity=severity,
        check_id=check_id,
        artifact_ref=artifact_ref,
        owner_artifact_ref=owner_artifact_ref,
        message=message,
        remediation=remediation,
        row_ref=row_ref,
        details=details,
    )


def render_yaml_as_json(path: Path) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
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


def load_json(path: Path) -> Any:
    with path.open("rb") as fh:
        return json.load(fh)


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as fh:
        return tomllib.load(fh)


def extract_internal_deps(cargo_toml: dict[str, Any]) -> list[str]:
    deps: set[str] = set()
    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        entries = cargo_toml.get(section, {})
        if not isinstance(entries, dict):
            continue
        for dep_name, dep_spec in entries.items():
            if not dep_name.startswith("aureline-"):
                continue
            if isinstance(dep_spec, dict) and "path" in dep_spec:
                deps.add(dep_name)
    return sorted(deps)


def parse_workspace(repo_root: Path) -> dict[str, dict[str, Any]]:
    root_manifest = load_toml(repo_root / "Cargo.toml")
    members = root_manifest.get("workspace", {}).get("members", [])
    if not isinstance(members, list):
        raise SystemExit("workspace.members is not a TOML list")

    package_map: dict[str, dict[str, Any]] = {}
    for member in members:
        if not isinstance(member, str):
            raise SystemExit(f"workspace member is not a string: {member!r}")
        cargo_toml = load_toml(repo_root / member / "Cargo.toml")
        name = cargo_toml["package"]["name"]
        package_map[name] = {
            "path": member,
            "manifest": cargo_toml,
            "internal_deps": extract_internal_deps(cargo_toml),
        }
    return package_map


def scenario_rel(repo_root: Path, scenario_path: Path | None) -> str | None:
    if scenario_path is None:
        return None
    resolved = scenario_path if scenario_path.is_absolute() else repo_root / scenario_path
    return resolved.relative_to(repo_root).as_posix()


def resolve_scenario_path(repo_root: Path, scenario_path: Path) -> Path:
    return scenario_path if scenario_path.is_absolute() else repo_root / scenario_path


def load_scenario(repo_root: Path, scenario_path: Path | None) -> dict[str, Any]:
    if scenario_path is None:
        return {
            "package_dependency_injections": [],
            "module_text_append": [],
        }

    resolved = resolve_scenario_path(repo_root, scenario_path)
    payload = load_json(resolved)
    if not isinstance(payload, dict):
        raise SystemExit("protected-dependency scenario must be a JSON object")

    package_injections = payload.get("package_dependency_injections", [])
    if not isinstance(package_injections, list):
        raise SystemExit("scenario field package_dependency_injections must be a list")
    for row in package_injections:
        if not isinstance(row, dict):
            raise SystemExit("each package_dependency_injections row must be an object")
        if not isinstance(row.get("package"), str) or not isinstance(row.get("dependency"), str):
            raise SystemExit("package_dependency_injections rows require string package and dependency")

    module_appends = payload.get("module_text_append", [])
    if not isinstance(module_appends, list):
        raise SystemExit("scenario field module_text_append must be a list")
    for row in module_appends:
        if not isinstance(row, dict):
            raise SystemExit("each module_text_append row must be an object")
        if not isinstance(row.get("path"), str) or not isinstance(row.get("text"), str):
            raise SystemExit("module_text_append rows require string path and text")

    return {
        "package_dependency_injections": package_injections,
        "module_text_append": module_appends,
    }


def append_text_for_path(scenario: dict[str, Any], rel_path: str) -> str:
    chunks = [row["text"] for row in scenario.get("module_text_append", []) if row["path"] == rel_path]
    return "".join(chunks)


def validate_required_inputs(
    repo_root: Path,
    findings: list[Finding],
) -> tuple[dict[str, Any] | None, dict[str, Any] | None, dict[str, Any] | None]:
    rules_path = repo_root / RULES_REL
    process_map_path = repo_root / PROCESS_MAP_REL
    overview_path = repo_root / OVERVIEW_DOC_REL
    inventory_path = repo_root / PACKAGE_INVENTORY_REL

    rules = process_map = inventory = None

    if not rules_path.exists():
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.rules_file_exists",
                RULES_REL,
                RULES_REL,
                f"protected dependency rule file is missing: {RULES_REL}",
                "Keep the protected dependency rule set checked in so CI can evaluate protected package and module boundaries.",
            )
        )
    else:
        rules = render_yaml_as_json(rules_path)

    if not process_map_path.exists():
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.process_map_exists",
                PROCESS_MAP_REL,
                RULES_REL,
                f"process placement map is missing: {PROCESS_MAP_REL}",
                "Keep the machine-readable process placement map checked in alongside the dependency rules.",
            )
        )
    else:
        process_map = render_yaml_as_json(process_map_path)

    if not overview_path.exists():
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.overview_doc_exists",
                OVERVIEW_DOC_REL,
                RULES_REL,
                f"service-topology overview doc is missing: {OVERVIEW_DOC_REL}",
                "Keep the narrative service-topology and process-placement policy checked in alongside the machine-readable maps.",
            )
        )

    if not inventory_path.exists():
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.package_inventory_exists",
                PACKAGE_INVENTORY_REL,
                RULES_REL,
                f"package inventory is missing: {PACKAGE_INVENTORY_REL}",
                "Keep the package inventory present because the protected dependency rules validate current workspace crates against it.",
            )
        )
    else:
        inventory = render_yaml_as_json(inventory_path)

    return rules, process_map, inventory


def validate_process_map(process_map: dict[str, Any], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    plane_rows = process_map.get("service_planes", [])
    if not isinstance(plane_rows, list):
        raise SystemExit(f"{PROCESS_MAP_REL} must define a top-level service_planes list")
    process_roles = process_map.get("process_roles", [])
    if not isinstance(process_roles, list):
        raise SystemExit(f"{PROCESS_MAP_REL} must define a top-level process_roles list")

    plane_ids = [row.get("plane_id") for row in plane_rows if isinstance(row, dict)]
    missing_planes = sorted(REQUIRED_PLANE_IDS - set(plane_ids))
    for plane_id in missing_planes:
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.required_plane_rows",
                PROCESS_MAP_REL,
                PROCESS_MAP_REL,
                f"process placement map is missing required plane '{plane_id}'",
                "Keep the machine-readable plane map complete for the protected runtime planes named by the architecture policy.",
                row_ref=plane_id,
            )
        )

    process_role_ids = [row.get("process_role_id") for row in process_roles if isinstance(row, dict)]
    missing_roles = sorted(REQUIRED_PROCESS_ROLE_IDS - set(process_role_ids))
    for role_id in missing_roles:
        findings.append(
            make_finding(
                "error",
                "protected_dependency_rules.required_process_roles",
                PROCESS_MAP_REL,
                PROCESS_MAP_REL,
                f"process placement map is missing required process role '{role_id}'",
                "Keep the machine-readable process placement map aligned with the architecture runtime model.",
                row_ref=role_id,
            )
        )

    plane_by_id = {row["plane_id"]: row for row in plane_rows if isinstance(row, dict) and row.get("plane_id")}
    role_ids = set(process_role_ids)
    for plane_id, row in plane_by_id.items():
        if row.get("primary_process_role") not in role_ids:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.plane_process_role_resolves",
                    PROCESS_MAP_REL,
                    item_ref(plane_id, PROCESS_MAP_REL),
                    f"plane '{plane_id}' points at unknown primary_process_role '{row.get('primary_process_role')}'",
                    "Use a declared process_roles entry for every plane's primary placement.",
                    row_ref=plane_id,
                )
            )

    return plane_by_id


def validate_rule_cross_links(
    rules: dict[str, Any],
    process_map: dict[str, Any],
    findings: list[Finding],
) -> None:
    refs = [
        ("overview_document", OVERVIEW_DOC_REL, "protected_dependency_rules.overview_ref"),
        ("process_placement_map_ref", PROCESS_MAP_REL, "protected_dependency_rules.process_map_ref"),
        ("package_inventory_ref", PACKAGE_INVENTORY_REL, "protected_dependency_rules.package_inventory_ref"),
    ]
    for field_name, expected, check_id in refs:
        actual = rules.get(field_name)
        if actual != expected:
            findings.append(
                make_finding(
                    "error",
                    check_id,
                    RULES_REL,
                    RULES_REL,
                    f"protected dependency rules field '{field_name}' points at '{actual}', expected '{expected}'",
                    "Keep the rule file's companion references aligned with the canonical repo paths.",
                )
            )

    process_refs = [
        ("overview_document", OVERVIEW_DOC_REL, "protected_dependency_rules.process_map_overview_ref"),
        ("dependency_rules_ref", RULES_REL, "protected_dependency_rules.process_map_rules_ref"),
    ]
    for field_name, expected, check_id in process_refs:
        actual = process_map.get(field_name)
        if actual != expected:
            findings.append(
                make_finding(
                    "error",
                    check_id,
                    PROCESS_MAP_REL,
                    PROCESS_MAP_REL,
                    f"process placement map field '{field_name}' points at '{actual}', expected '{expected}'",
                    "Keep the process placement map cross-linked to the canonical overview document and rule file.",
                )
            )


def validate_package_rules(
    repo_root: Path,
    rules: dict[str, Any],
    inventory: dict[str, Any],
    plane_by_id: dict[str, dict[str, Any]],
    scenario: dict[str, Any],
    findings: list[Finding],
) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    dependency_class_vocabulary = rules.get("dependency_class_vocabulary", {})
    if not isinstance(dependency_class_vocabulary, dict):
        raise SystemExit(f"{RULES_REL} must define a top-level dependency_class_vocabulary map")

    package_rows = rules.get("packages", [])
    if not isinstance(package_rows, list):
        raise SystemExit(f"{RULES_REL} must define a top-level packages list")

    workspace_packages = parse_workspace(repo_root)
    inventory_rows = {row["name"]: row for row in inventory.get("packages", [])}
    package_rules = {row["package"]: row for row in package_rows if isinstance(row, dict) and row.get("package")}

    for required_class in REQUIRED_PLANE_IDS | {"off_cone"}:
        if required_class not in dependency_class_vocabulary:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.required_dependency_classes",
                    RULES_REL,
                    RULES_REL,
                    f"protected dependency rules are missing dependency class '{required_class}'",
                    "Keep the dependency-class vocabulary aligned with the protected service-plane map and the off-cone sentinel class.",
                    row_ref=required_class,
                )
            )

    for class_name, class_row in dependency_class_vocabulary.items():
        if not isinstance(class_row, dict):
            raise SystemExit(f"dependency_class_vocabulary entry '{class_name}' must be an object")
        plane_id = class_row.get("plane")
        if plane_id not in plane_by_id and class_name != "off_cone":
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.class_plane_resolves",
                    RULES_REL,
                    RULES_REL,
                    f"dependency class '{class_name}' points at unknown plane '{plane_id}'",
                    "Map each dependency class to a declared service plane.",
                    row_ref=class_name,
                )
            )

    for package_name in sorted(workspace_packages):
        if package_name not in package_rules:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_rule_coverage",
                    RULES_REL,
                    RULES_REL,
                    f"workspace package '{package_name}' is missing a protected dependency rule row",
                    "Keep every workspace package mapped to an explicit dependency class, even when the package is off-cone.",
                    row_ref=package_name,
                )
            )

    injections_by_package: dict[str, list[str]] = {}
    for row in scenario.get("package_dependency_injections", []):
        package_name = row["package"]
        dependency = row["dependency"]
        if package_name not in workspace_packages:
            raise SystemExit(f"scenario references unknown package '{package_name}'")
        injections_by_package.setdefault(package_name, []).append(dependency)

    for package_name, row in package_rules.items():
        row_ref = item_ref(package_name, RULES_REL)
        if package_name not in workspace_packages:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_rule_resolves",
                    RULES_REL,
                    row_ref,
                    f"package rule references unknown workspace package '{package_name}'",
                    "Keep the protected dependency rule rows aligned with the Cargo workspace.",
                    row_ref=package_name,
                )
            )
            continue

        dependency_class = row.get("dependency_class")
        if dependency_class not in dependency_class_vocabulary:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_class_resolves",
                    RULES_REL,
                    row_ref,
                    f"package '{package_name}' points at unknown dependency class '{dependency_class}'",
                    "Use a declared dependency class for every package rule row.",
                    row_ref=package_name,
                )
            )

        allowed_classes = set(row.get("allowed_dependency_classes", []))
        forbidden_classes = set(row.get("forbidden_dependency_classes", []))
        overlap = sorted(allowed_classes & forbidden_classes)
        if overlap:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_class_overlap",
                    RULES_REL,
                    row_ref,
                    f"package '{package_name}' lists the same dependency classes as allowed and forbidden: {', '.join(overlap)}",
                    "Keep allowed and forbidden dependency-class sets disjoint.",
                    row_ref=package_name,
                )
            )

        inventory_row = inventory_rows.get(package_name)
        if inventory_row is None:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_inventory_coverage",
                    RULES_REL,
                    row_ref,
                    f"package '{package_name}' is missing from package_inventory.yaml",
                    "Keep the package inventory and the protected dependency rules in sync.",
                    row_ref=package_name,
                )
            )
        elif bool(inventory_row.get("protected_path")) != bool(row.get("protected_package")):
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.package_protected_flag_parity",
                    RULES_REL,
                    row_ref,
                    f"package '{package_name}' disagrees on protected status between protected_path_dependency_rules.yaml and package_inventory.yaml",
                    "Keep protected-package posture aligned across the package inventory and protected dependency rule file.",
                    row_ref=package_name,
                )
            )

        actual_deps = set(workspace_packages[package_name]["internal_deps"])
        actual_deps.update(injections_by_package.get(package_name, []))
        for dep_name in sorted(actual_deps):
            if dep_name not in package_rules:
                findings.append(
                    make_finding(
                        "error",
                        "protected_dependency_rules.depends_on_unclassified_package",
                        RULES_REL,
                        row_ref,
                        f"package '{package_name}' depends on '{dep_name}', which has no dependency-class rule row",
                        "Add a package rule row for every depended-on workspace crate.",
                        row_ref=package_name,
                        dependency=dep_name,
                    )
                )
                continue

            dep_class = package_rules[dep_name].get("dependency_class")
            if dep_class in forbidden_classes:
                findings.append(
                    make_finding(
                        "error",
                        "protected_dependency_rules.package_forbidden_dependency_class",
                        RULES_REL,
                        row_ref,
                        f"package '{package_name}' depends on '{dep_name}' in forbidden dependency class '{dep_class}'",
                        "Remove the dependency or update the service-topology rule set only if the boundary change is intentional and reviewed.",
                        row_ref=package_name,
                        dependency=dep_name,
                        dependency_class=dep_class,
                    )
                )
            elif dep_class not in allowed_classes:
                findings.append(
                    make_finding(
                        "error",
                        "protected_dependency_rules.package_dependency_direction",
                        RULES_REL,
                        row_ref,
                        f"package '{package_name}' depends on '{dep_name}' in dependency class '{dep_class}', which is not an allowed direction",
                        "Keep compile-time dependency directions aligned with the service-topology map; remove the dependency or promote the boundary explicitly in the rule file and companion docs.",
                        row_ref=package_name,
                        dependency=dep_name,
                        dependency_class=dep_class,
                    )
                )

    return package_rules, workspace_packages


def validate_module_rules(
    repo_root: Path,
    rules: dict[str, Any],
    package_rules: dict[str, dict[str, Any]],
    scenario: dict[str, Any],
    findings: list[Finding],
) -> list[dict[str, Any]]:
    module_rows = rules.get("modules", [])
    if not isinstance(module_rows, list):
        raise SystemExit(f"{RULES_REL} must define a top-level modules list")

    sentinel_vocabulary = rules.get("sentinel_class_vocabulary", {})
    if not isinstance(sentinel_vocabulary, dict):
        raise SystemExit(f"{RULES_REL} must define a top-level sentinel_class_vocabulary map")

    monitored_paths = {row.get("path") for row in module_rows if isinstance(row, dict)}
    for row in scenario.get("module_text_append", []):
        if row["path"] not in monitored_paths:
            raise SystemExit(f"scenario references unknown monitored module path '{row['path']}'")

    for row in module_rows:
        if not isinstance(row, dict):
            raise SystemExit("each protected module rule row must be an object")
        path = row.get("path")
        if not isinstance(path, str):
            raise SystemExit("each protected module rule row requires a string path")
        row_ref = item_ref(path, RULES_REL)
        module_path = repo_root / path
        if not module_path.exists():
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.module_path_exists",
                    RULES_REL,
                    row_ref,
                    f"protected module rule points at missing path '{path}'",
                    "Keep monitored hot-path module paths current when files move or split.",
                    row_ref=path,
                )
            )
            continue

        owner_class = row.get("owner_dependency_class")
        if owner_class not in rules.get("dependency_class_vocabulary", {}):
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.module_owner_class_resolves",
                    RULES_REL,
                    row_ref,
                    f"module '{path}' points at unknown owner dependency class '{owner_class}'",
                    "Use a declared dependency class for each protected module row.",
                    row_ref=path,
                )
            )

        allowed_local_modules = set(row.get("allowed_local_modules", []))
        content = module_path.read_text(encoding="utf-8") + append_text_for_path(scenario, path)
        local_imports = {match.group(1) for match in LOCAL_IMPORT_RE.finditer(content)}
        unexpected_local_imports = sorted(local_imports - allowed_local_modules)
        if unexpected_local_imports:
            findings.append(
                make_finding(
                    "error",
                    "protected_dependency_rules.module_local_dependency_direction",
                    RULES_REL,
                    row_ref,
                    f"module '{path}' imports unexpected crate-local modules: {', '.join(unexpected_local_imports)}",
                    "Keep protected modules importing only the narrow crate-local seams named in the rule file.",
                    row_ref=path,
                    imported_modules=sorted(local_imports),
                    allowed_local_modules=sorted(allowed_local_modules),
                )
            )

        for sentinel_name in row.get("forbidden_sentinel_classes", []):
            sentinel_row = sentinel_vocabulary.get(sentinel_name)
            if sentinel_row is None:
                findings.append(
                    make_finding(
                        "error",
                        "protected_dependency_rules.module_sentinel_resolves",
                        RULES_REL,
                        row_ref,
                        f"module '{path}' references unknown sentinel class '{sentinel_name}'",
                        "Use a declared sentinel class for every protected module rule.",
                        row_ref=path,
                    )
                )
                continue

            patterns = sentinel_row.get("patterns", [])
            if not isinstance(patterns, list):
                raise SystemExit(f"sentinel class '{sentinel_name}' must define a patterns list")

            for pattern in patterns:
                if re.search(pattern, content, re.MULTILINE):
                    findings.append(
                        make_finding(
                            "error",
                            "protected_dependency_rules.module_forbidden_sentinel",
                            RULES_REL,
                            row_ref,
                            f"module '{path}' matched forbidden sentinel class '{sentinel_name}'",
                            "Move blocking I/O or process-launch work off the protected module, or narrow the sentinel rule only with an intentional architecture change.",
                            row_ref=path,
                            sentinel_class=sentinel_name,
                            pattern=pattern,
                        )
                    )
                    break

    return module_rows


def build_report(
    findings: list[Finding],
    plane_count: int,
    package_rule_count: int,
    module_rule_count: int,
    scenario_ref: str | None,
) -> dict[str, Any]:
    errors = sum(1 for finding in findings if finding.severity == "error")
    warnings = sum(1 for finding in findings if finding.severity == "warning")
    return {
        "report_kind": "protected_dependency_validation_report",
        "generated_at": now_utc(),
        "inputs": {
            "rules_ref": RULES_REL,
            "process_map_ref": PROCESS_MAP_REL,
            "overview_document_ref": OVERVIEW_DOC_REL,
            "package_inventory_ref": PACKAGE_INVENTORY_REL,
            "scenario": scenario_ref,
        },
        "summary": {
            "plane_count": plane_count,
            "package_rule_count": package_rule_count,
            "module_rule_count": module_rule_count,
            "error_count": errors,
            "warning_count": warnings,
        },
        "findings": [finding.as_report() for finding in findings],
    }


def render_human_summary(findings: list[Finding], analysis: dict[str, Any]) -> str:
    summary = analysis["summary"]
    lines = [
        "Protected dependency validation",
        f"planes: {summary['plane_count']}; package rules: {summary['package_rule_count']}; module rules: {summary['module_rule_count']}",
        f"errors: {summary['error_count']}; warnings: {summary['warning_count']}",
    ]

    if not findings:
        lines.append("PASS: no protected dependency or hot-path sentinel findings.")
        return "\n".join(lines) + "\n"

    lines.append("Findings:")
    for finding in findings:
        location = f" ({finding.row_ref})" if finding.row_ref else ""
        lines.append(f"- [{finding.severity}] {finding.check_id}{location}: {finding.message}")
    return "\n".join(lines) + "\n"


def validate_protected_dependency_rules(
    repo_root: Path,
    scenario_path: Path | None,
) -> tuple[list[Finding], dict[str, Any]]:
    findings: list[Finding] = []
    scenario = load_scenario(repo_root, scenario_path)
    rules, process_map, inventory = validate_required_inputs(repo_root, findings)

    plane_by_id: dict[str, dict[str, Any]] = {}
    package_rules: dict[str, dict[str, Any]] = {}
    module_rows: list[dict[str, Any]] = []

    if rules is not None and process_map is not None:
        plane_by_id = validate_process_map(process_map, findings)
        validate_rule_cross_links(rules, process_map, findings)

    if rules is not None and inventory is not None and process_map is not None:
        package_rules, _ = validate_package_rules(
            repo_root,
            rules,
            inventory,
            plane_by_id,
            scenario,
            findings,
        )
        module_rows = validate_module_rules(repo_root, rules, package_rules, scenario, findings)

    analysis = build_report(
        findings,
        plane_count=len(plane_by_id),
        package_rule_count=len(package_rules),
        module_rule_count=len(module_rows),
        scenario_ref=scenario_rel(repo_root, scenario_path),
    )
    return findings, analysis
