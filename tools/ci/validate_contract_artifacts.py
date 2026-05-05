#!/usr/bin/env python3
"""Validate Aureline's contract-bearing governance artifacts.

This validator turns the repository's architecture-freeze, package-group,
stable-surface, public-truth, and boundary artifacts into one repeatable
gate. It emits:

- human-readable findings on stdout;
- a machine-readable JSON report via `--report`;
- a non-zero exit code when error-severity findings are present.

The repository is YAML-heavy, but the toolchain is intentionally light.
This script therefore uses Python stdlib for control flow plus Ruby's
built-in Psych parser for YAML decoding. macOS and GitHub-hosted Ubuntu
both ship Ruby by default; the wrapper and workflow make that explicit.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

CURRENT_DIR = Path(__file__).resolve().parent
if str(CURRENT_DIR) not in sys.path:
    sys.path.insert(0, str(CURRENT_DIR))

from frozen_surface_validation import (
    FROZEN_SURFACE_MANIFEST_REL,
    FROZEN_SURFACE_SCENARIO_REL,
    FROZEN_SURFACE_TOOL_REL,
    validate_frozen_surface_manifest,
)
from protected_dependency_validation import (
    OVERVIEW_DOC_REL as PROTECTED_DEPENDENCY_OVERVIEW_REL,
    PROCESS_MAP_REL as PROTECTED_DEPENDENCY_PROCESS_MAP_REL,
    RULES_REL as PROTECTED_DEPENDENCY_RULES_REL,
    validate_protected_dependency_rules,
)

CONTROL_ARTIFACT_INDEX_REL = "artifacts/governance/control_artifact_index.yaml"
PACKAGE_INVENTORY_REL = "artifacts/governance/package_inventory.yaml"
OWNERSHIP_MATRIX_REL = "artifacts/governance/ownership_matrix.yaml"
STABLE_SURFACE_INVENTORY_REL = "artifacts/governance/stable_surface_inventory.yaml"
INTERFACE_FREEZE_MATRIX_REL = "artifacts/governance/interface_freeze_matrix.yaml"
DECISION_INDEX_REL = "artifacts/governance/decision_index.yaml"
QUALIFICATION_MATRIX_REL = "artifacts/compat/qualification_matrix_seed.yaml"
VERSION_SKEW_REGISTER_REL = "artifacts/compat/version_skew_register.yaml"
CLAIM_MANIFEST_REL = "artifacts/governance/claim_manifest_seed.yaml"
SOURCE_OF_TRUTH_MAP_REL = "artifacts/governance/source_of_truth_map.yaml"
BOUNDARY_MANIFEST_REL = "docs/product/boundary_manifest_strawman.md"
REPO_TOPOLOGY_REL = "docs/repo/topology.md"
DEPENDENCY_RULES_REL = "docs/repo/dependency_rules.md"
ROOT_CARGO_TOML_REL = "Cargo.toml"
COMMAND_PARITY_TOOL_REL = "tools/commands/parity_diff_seed.py"
COMMAND_PARITY_SEED_REL = "artifacts/commands/command_parity_seed.yaml"
CONTRACT_VALIDATION_WRAPPER_REL = "ci/contract_validation.sh"
CONTRACT_VALIDATION_WORKFLOW_REL = ".github/workflows/contract_validation.yml"
CONTRACT_VALIDATION_DOC_REL = "docs/ci/control_artifact_validation.md"
CONTRACT_VALIDATION_SCENARIO_REL = "fixtures/ci/contract_validation/missing_deployment_profile.json"
PROTECTED_DEPENDENCY_TOOL_REL = "tools/check_protected_dependencies.py"
PROTECTED_DEPENDENCY_SCENARIO_REL = "fixtures/ci/contract_validation/protected_dependency_violation.json"
PRINCIPLE_ENFORCEMENT_MATRIX_DOC_REL = "docs/architecture/principle_enforcement_matrix.md"
PRINCIPLE_CHECKS_REL = "artifacts/architecture/principle_checks.yaml"
PRINCIPLE_VIOLATION_EXAMPLES_REL = "artifacts/architecture/principle_violation_examples.yaml"
FITNESS_CATALOG_REL = "artifacts/bench/fitness_function_catalog.yaml"
REJECTED_PATTERNS_REL = "artifacts/architecture/driver_to_rejected_pattern_refs.yaml"
MANDATORY_REVIEW_ARTIFACTS_REL = "artifacts/governance/mandatory_review_artifacts.yaml"

SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}
LAYER_ORDER = {"L0": 0, "L1": 1, "L2": 2, "L3": 3, "LX": 99}

REQUIRED_CONTROL_ROWS = {
    "public_surface_truth_map": SOURCE_OF_TRUTH_MAP_REL,
    "boundary_manifest_strawman": BOUNDARY_MANIFEST_REL,
    "repository_package_inventory": PACKAGE_INVENTORY_REL,
    "service_topology_process_placement_policy": PROTECTED_DEPENDENCY_OVERVIEW_REL,
    "protected_path_dependency_rules": PROTECTED_DEPENDENCY_RULES_REL,
    "process_placement_map": PROTECTED_DEPENDENCY_PROCESS_MAP_REL,
    "architecture_principle_enforcement_matrix": PRINCIPLE_ENFORCEMENT_MATRIX_DOC_REL,
    "architecture_principle_checks": PRINCIPLE_CHECKS_REL,
    "architecture_principle_violation_examples": PRINCIPLE_VIOLATION_EXAMPLES_REL,
    "contract_artifact_validation_lane": CONTRACT_VALIDATION_WORKFLOW_REL,
    "frozen_surface_manifests": FROZEN_SURFACE_MANIFEST_REL,
}

BOUNDARY_ROW_RE = re.compile(
    r"^#### `(?P<capability_id>[^`]+)` — (?P<title>.+?)\n(?P<body>.*?)(?=^#### `|\Z)",
    re.MULTILINE | re.DOTALL,
)


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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--scenario",
        default=None,
        help=(
            "Optional JSON mutation scenario used to demonstrate a failing case "
            "without editing the checked-in artifacts."
        ),
    )
    return parser.parse_args()


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def load_json(path: Path) -> Any:
    with path.open("rb") as fh:
        return json.load(fh)


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as fh:
        return tomllib.load(fh)


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


def is_path_like(ref: str | None) -> bool:
    if not ref:
        return False
    if ref in SENTINEL_REFS:
        return False
    if ref.startswith(
        (
            "compat_row:",
            "skew_register:",
            "skew_case:",
            "claim_row:",
            "lane:",
            "package:",
            "build:",
            "evidence.",
            "docs-pack:",
            "dest:",
            "producer:",
            "launch_bundle:",
            "policy-",
            "cmd:",
            "label:",
            "menu:",
            "field:",
        )
    ):
        return False
    return (
        "/" in ref
        or ref.endswith((".md", ".yaml", ".yml", ".json", ".py", ".sh", ".toml"))
        or ref in {"README.md", "CONTRIBUTING.md", "CODEOWNERS"}
    )


def strip_path_annotations(ref: str) -> str:
    path = ref
    for separator in ("#", " §", " line ", " @"):
        if separator in path:
            path = path.split(separator, 1)[0]
    return path.strip()


def inline_code_values(text: str) -> list[str]:
    return re.findall(r"`([^`]+)`", text)


def normalize_text_block(text: str) -> str:
    return re.sub(r"\s+", " ", text.strip())


def extract_bullet_field(section_body: str, label: str) -> str | None:
    match = re.search(
        rf"- \*\*{re.escape(label)}:\*\* (?P<value>.*?)(?=\n- \*\*|\Z)",
        section_body,
        re.DOTALL,
    )
    if not match:
        return None
    return normalize_text_block(match.group("value"))


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


class RepoView:
    def __init__(self, root: Path, scenario_path: Path | None) -> None:
        self.root = root
        self.scenario = self._load_scenario(scenario_path)
        self.yaml_cache: dict[str, Any] = {}
        self.json_cache: dict[str, Any] = {}
        self.text_cache: dict[str, str] = {}

    def _load_scenario(self, scenario_path: Path | None) -> dict[str, Any] | None:
        if scenario_path is None:
            return None
        path = scenario_path if scenario_path.is_absolute() else self.root / scenario_path
        if not path.exists():
            raise SystemExit(f"scenario file does not exist: {path}")
        payload = load_json(path)
        if not isinstance(payload, dict) or not isinstance(payload.get("mutations"), list):
            raise SystemExit(f"scenario file must contain a JSON object with a 'mutations' list: {path}")
        payload["_path"] = str(path.relative_to(self.root))
        return payload

    def rel(self, relative: str) -> Path:
        return self.root / relative

    def exists(self, relative: str) -> bool:
        return self.rel(relative).exists()

    def text(self, relative: str) -> str:
        cached = self.text_cache.get(relative)
        if cached is not None:
            return cached
        text = read_text(self.rel(relative))
        self.text_cache[relative] = text
        return text

    def yaml(self, relative: str) -> Any:
        if relative not in self.yaml_cache:
            payload = render_yaml_as_json(self.rel(relative))
            self._apply_scenario_mutations(relative, payload)
            self.yaml_cache[relative] = payload
        return self.yaml_cache[relative]

    def json(self, relative: str) -> Any:
        if relative not in self.json_cache:
            payload = load_json(self.rel(relative))
            self._apply_scenario_mutations(relative, payload)
            self.json_cache[relative] = payload
        return self.json_cache[relative]

    def _apply_scenario_mutations(self, relative: str, payload: Any) -> None:
        if self.scenario is None:
            return
        for mutation in self.scenario["mutations"]:
            if mutation.get("target") != relative:
                continue
            path = mutation.get("path")
            if not isinstance(path, list) or not path:
                raise SystemExit(f"scenario mutation for {relative} must contain a non-empty path list")
            self._apply_single_mutation(relative, payload, path, mutation.get("value"))

    def _apply_single_mutation(self, relative: str, payload: Any, path: list[Any], value: Any) -> None:
        cursor = payload
        for segment in path[:-1]:
            try:
                cursor = cursor[segment]
            except (KeyError, IndexError, TypeError) as exc:
                raise SystemExit(
                    f"scenario mutation path {path!r} is invalid for {relative}"
                ) from exc
        leaf = path[-1]
        try:
            cursor[leaf] = value
        except (KeyError, IndexError, TypeError) as exc:
            raise SystemExit(
                f"scenario mutation path {path!r} is invalid for {relative}"
            ) from exc


def register_duplicates(values: list[str]) -> list[str]:
    return sorted([value for value, count in Counter(values).items() if count > 1])


def parse_boundary_manifest(text: str) -> tuple[list[str], list[dict[str, Any]]]:
    deployment_section_match = re.search(
        r"## Deployment profiles\n(?P<body>.*?)(?=\n## )",
        text,
        re.DOTALL,
    )
    if not deployment_section_match:
        raise SystemExit(f"could not locate deployment-profile table in {BOUNDARY_MANIFEST_REL}")

    deployment_profiles: list[str] = []
    for line in deployment_section_match.group("body").splitlines():
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if not cells or cells[0] in {"Profile id", "-----------------------"}:
            continue
        deployment_profiles.append(cells[0].strip("`"))

    deployment_profile_set = set(deployment_profiles)

    rows: list[dict[str, Any]] = []
    for match in BOUNDARY_ROW_RE.finditer(text):
        body = match.group("body")
        classification = extract_bullet_field(body, "Classification") or ""
        status = extract_bullet_field(body, "Status") or ""
        rows.append(
            {
                "capability_id": match.group("capability_id"),
                "title": normalize_text_block(match.group("title")),
                "classification": inline_code_values(classification)[0] if inline_code_values(classification) else classification,
                "deployment_profiles": [
                    code
                    for code in inline_code_values(extract_bullet_field(body, "Deployment profiles") or "")
                    if code in deployment_profile_set
                ],
                "linked_decisions": sorted(
                    set(re.findall(r"D-\d{4}", extract_bullet_field(body, "Linked decisions") or ""))
                ),
                "linked_lanes": inline_code_values(extract_bullet_field(body, "Linked lanes") or ""),
                "status": inline_code_values(status)[0] if inline_code_values(status) else status,
            }
        )
    return deployment_profiles, rows


def parse_workspace(repo: RepoView) -> tuple[list[str], dict[str, dict[str, Any]]]:
    root_manifest = load_toml(repo.rel(ROOT_CARGO_TOML_REL))
    members = root_manifest.get("workspace", {}).get("members", [])
    if not isinstance(members, list):
        raise SystemExit("workspace.members is not a TOML list")

    package_map: dict[str, dict[str, Any]] = {}
    for member in members:
        if not isinstance(member, str):
            raise SystemExit(f"workspace member is not a string: {member!r}")
        cargo_toml = load_toml(repo.rel(f"{member}/Cargo.toml"))
        name = cargo_toml["package"]["name"]
        package_map[name] = {
            "path": member,
            "manifest": cargo_toml,
            "internal_deps": extract_internal_deps(cargo_toml),
        }
    return members, package_map


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


def resolve_owner_ref(owner_ref: str, packages: set[str], lanes: set[str]) -> bool:
    if owner_ref.startswith("package:"):
        return owner_ref.split(":", 1)[1] in packages
    if owner_ref.startswith("lane:"):
        return owner_ref.split(":", 1)[1] in lanes
    return False


def existing_path_ref(repo: RepoView, ref: str) -> bool:
    if not is_path_like(ref):
        return True
    return repo.exists(strip_path_annotations(ref))


def validate_package_inventory(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    inventory = repo.yaml(PACKAGE_INVENTORY_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    _, workspace_packages = parse_workspace(repo)

    inventory_rows = inventory.get("packages", [])
    if not isinstance(inventory_rows, list):
        raise SystemExit(f"{PACKAGE_INVENTORY_REL} must define a top-level packages list")

    inventory_names = [row["name"] for row in inventory_rows]
    inventory_paths = [row["path"] for row in inventory_rows]
    for duplicate in register_duplicates(inventory_names):
        findings.append(
            make_finding(
                "error",
                "package_inventory.unique_package_names",
                PACKAGE_INVENTORY_REL,
                PACKAGE_INVENTORY_REL,
                f"package inventory duplicates the package name '{duplicate}'",
                "Keep exactly one package_inventory row per Cargo package name.",
                row_ref=duplicate,
            )
        )
    for duplicate in register_duplicates(inventory_paths):
        findings.append(
            make_finding(
                "error",
                "package_inventory.unique_package_paths",
                PACKAGE_INVENTORY_REL,
                PACKAGE_INVENTORY_REL,
                f"package inventory duplicates the path '{duplicate}'",
                "Keep exactly one package_inventory row per crate path.",
                row_ref=duplicate,
            )
        )

    work_package_index = inventory.get("work_package_index", {})
    by_name = {row["name"]: row for row in inventory_rows}
    ownership_rows = {row["name"]: row for row in ownership.get("packages", [])}

    for row in inventory_rows:
        row_ref = item_ref(row["name"], PACKAGE_INVENTORY_REL)
        path = row["path"]
        if not repo.exists(path):
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.paths_exist",
                    PACKAGE_INVENTORY_REL,
                    row_ref,
                    f"package '{row['name']}' points at missing path '{path}'",
                    "Fix the package_inventory path or land the crate directory in the same change.",
                    row_ref=row["name"],
                )
            )
            continue

        cargo_path = f"{path}/Cargo.toml"
        if not repo.exists(cargo_path):
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.cargo_manifest_exists",
                    PACKAGE_INVENTORY_REL,
                    row_ref,
                    f"package '{row['name']}' is missing '{cargo_path}'",
                    "Every package_inventory row must point at a real crate with a Cargo.toml manifest.",
                    row_ref=row["name"],
                )
            )
            continue

        cargo_name = load_toml(repo.rel(cargo_path))["package"]["name"]
        if cargo_name != row["name"]:
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.name_matches_manifest",
                    PACKAGE_INVENTORY_REL,
                    row_ref,
                    f"package_inventory row '{row['name']}' points at a crate manifest named '{cargo_name}'",
                    "Keep package_inventory name/path pairs aligned with the crate's package.name.",
                    row_ref=row["name"],
                    cargo_manifest=cargo_path,
                )
            )

        for work_package in row.get("work_packages", []):
            if work_package not in work_package_index:
                findings.append(
                    make_finding(
                        "error",
                        "package_inventory.work_package_index",
                        PACKAGE_INVENTORY_REL,
                        row_ref,
                        f"package '{row['name']}' references unknown work package '{work_package}'",
                        "Add the work package to work_package_index or remove the stale reference.",
                        row_ref=row["name"],
                    )
                )

        ownership_row = ownership_rows.get(row["name"])
        severity = "error" if row.get("protected_path") else "warning"
        if ownership_row is None:
            findings.append(
                make_finding(
                    severity,
                    "package_inventory.ownership_coverage",
                    OWNERSHIP_MATRIX_REL,
                    row_ref,
                    f"package '{row['name']}' is missing a matching ownership_matrix package row",
                    "Add the crate to artifacts/governance/ownership_matrix.yaml in the same change.",
                    row_ref=row["name"],
                )
            )
        elif bool(ownership_row.get("protected")) != bool(row.get("protected_path")):
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.protected_flag_parity",
                    OWNERSHIP_MATRIX_REL,
                    row_ref,
                    f"package '{row['name']}' disagrees on protected status between package_inventory and ownership_matrix",
                    "Keep protected-path posture aligned across package_inventory.yaml and ownership_matrix.yaml.",
                    row_ref=row["name"],
                )
            )

        for dep_name in row.get("allowed_internal_deps", []):
            if dep_name not in by_name and dep_name not in workspace_packages:
                findings.append(
                    make_finding(
                        "error",
                        "package_inventory.allowed_deps_resolve",
                        PACKAGE_INVENTORY_REL,
                        row_ref,
                        f"package '{row['name']}' allows unknown internal dependency '{dep_name}'",
                        "Only allow internal dependencies that exist as workspace crates.",
                        row_ref=row["name"],
                    )
                )

    for package_name, package_data in workspace_packages.items():
        row = by_name.get(package_name)
        if row is None:
            severity = "warning" if package_name.endswith("-proto") else "error"
            findings.append(
                make_finding(
                    severity,
                    "package_inventory.workspace_member_coverage",
                    PACKAGE_INVENTORY_REL,
                    ROOT_CARGO_TOML_REL,
                    f"workspace crate '{package_name}' is not listed in package_inventory.yaml",
                    "Add every workspace crate to artifacts/governance/package_inventory.yaml so dependency gates stay authoritative.",
                    row_ref=package_name,
                    crate_path=package_data["path"],
                )
            )
            continue

        row_ref = item_ref(package_name, PACKAGE_INVENTORY_REL)
        actual_deps = set(package_data["internal_deps"])
        allowed_deps = set(row.get("allowed_internal_deps", []))
        disallowed = sorted(actual_deps - allowed_deps)
        if disallowed:
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.forbidden_dependency_edges",
                    PACKAGE_INVENTORY_REL,
                    row_ref,
                    f"package '{package_name}' declares internal dependencies not allowed by package_inventory: {', '.join(disallowed)}",
                    "Update the package_inventory allowed_internal_deps and dependency_rules.md only if the edge is intentional and reviewed; otherwise remove the edge.",
                    row_ref=package_name,
                    actual_deps=sorted(actual_deps),
                    allowed_deps=sorted(allowed_deps),
                )
            )

        source_layer = row.get("layer")
        for dep_name in actual_deps:
            dep_row = by_name.get(dep_name)
            if dep_row is None:
                findings.append(
                    make_finding(
                        "error",
                        "package_inventory.dependency_row_coverage",
                        PACKAGE_INVENTORY_REL,
                        row_ref,
                        f"package '{package_name}' depends on '{dep_name}', but '{dep_name}' has no package_inventory row",
                        "Add the depended-on crate to package_inventory.yaml before protected packages depend on it.",
                        row_ref=package_name,
                    )
                )
                continue

            dep_layer = dep_row.get("layer")
            if source_layer in LAYER_ORDER and dep_layer in LAYER_ORDER and source_layer != "LX":
                if dep_layer == "LX" and row.get("depended_on_by_production"):
                    findings.append(
                        make_finding(
                            "error",
                            "package_inventory.off_cone_dependency",
                            PACKAGE_INVENTORY_REL,
                            row_ref,
                            f"production-facing package '{package_name}' depends on off-cone crate '{dep_name}'",
                            "Protected or production-facing crates must not depend on spike, benchmark, or prototype crates.",
                            row_ref=package_name,
                        )
                    )
                elif dep_layer != "LX" and LAYER_ORDER[dep_layer] >= LAYER_ORDER[source_layer]:
                    findings.append(
                        make_finding(
                            "error",
                            "package_inventory.layering",
                            PACKAGE_INVENTORY_REL,
                            row_ref,
                            f"package '{package_name}' (layer {source_layer}) depends on '{dep_name}' (layer {dep_layer}), which is not a lower layer",
                            "Keep protected layering strictly downhill unless the source package is explicitly off-cone (layer LX).",
                            row_ref=package_name,
                            dependency=dep_name,
                        )
                    )

    graph = {name: set(data["internal_deps"]) for name, data in workspace_packages.items()}
    temp: set[str] = set()
    seen: set[str] = set()
    cycle_paths: list[list[str]] = []

    def visit(node: str, stack: list[str]) -> None:
        if node in temp:
            try:
                cycle_start = stack.index(node)
            except ValueError:
                cycle_start = 0
            cycle_paths.append(stack[cycle_start:] + [node])
            return
        if node in seen:
            return
        seen.add(node)
        temp.add(node)
        for neighbor in sorted(graph.get(node, ())):
            visit(neighbor, stack + [neighbor])
        temp.remove(node)

    for node in sorted(graph):
        visit(node, [node])

    for cycle in cycle_paths:
        findings.append(
            make_finding(
                "error",
                "package_inventory.cycles",
                PACKAGE_INVENTORY_REL,
                PACKAGE_INVENTORY_REL,
                f"internal dependency cycle detected: {' -> '.join(cycle)}",
                "Break the cycle and keep the protected dependency graph acyclic.",
                row_ref=cycle[0],
            )
        )

    for required_rel in (REPO_TOPOLOGY_REL, DEPENDENCY_RULES_REL):
        if not repo.exists(required_rel):
            findings.append(
                make_finding(
                    "error",
                    "package_inventory.companion_docs_exist",
                    required_rel,
                    PACKAGE_INVENTORY_REL,
                    f"required repository-governance companion doc is missing: {required_rel}",
                    "Keep docs/repo/topology.md and docs/repo/dependency_rules.md present alongside package_inventory.yaml.",
                )
            )

    return findings


def validate_control_artifact_index(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    index = repo.yaml(CONTROL_ARTIFACT_INDEX_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    rows = index.get("artifacts", [])
    lanes = {row["id"] for row in ownership.get("governance_lanes", [])}

    row_ids = [row["id"] for row in rows]
    for duplicate in register_duplicates(row_ids):
        findings.append(
            make_finding(
                "error",
                "control_artifact_index.unique_ids",
                CONTROL_ARTIFACT_INDEX_REL,
                CONTROL_ARTIFACT_INDEX_REL,
                f"control-artifact index duplicates the row id '{duplicate}'",
                "Keep exactly one control_artifact_index row per governed asset.",
                row_ref=duplicate,
            )
        )

    by_id = {row["id"]: row for row in rows}
    for row in rows:
        row_ref = item_ref(row["id"], CONTROL_ARTIFACT_INDEX_REL)
        for field_name in ("owner_dri", "owning_lane", "review_cadence", "visibility_class", "next_milestone", "status"):
            if not row.get(field_name):
                findings.append(
                    make_finding(
                        "error",
                        f"control_artifact_index.required_field.{field_name}",
                        CONTROL_ARTIFACT_INDEX_REL,
                        row_ref,
                        f"control-artifact row '{row['id']}' is missing required field '{field_name}'",
                        "Populate the missing control_artifact_index field so review ownership and cadence stay explicit.",
                        row_ref=row["id"],
                    )
                )

        owning_lane = row.get("owning_lane")
        if owning_lane and owning_lane not in lanes:
            findings.append(
                make_finding(
                    "error",
                    "control_artifact_index.owning_lane_resolves",
                    CONTROL_ARTIFACT_INDEX_REL,
                    row_ref,
                    f"control-artifact row '{row['id']}' points at unknown owning_lane '{owning_lane}'",
                    "Use an ownership_matrix governance_lanes id as the control-artifact owning_lane.",
                    row_ref=row["id"],
                )
            )

        canonical_location = row.get("canonical_location")
        if canonical_location and canonical_location not in SENTINEL_REFS and not repo.exists(canonical_location):
            findings.append(
                make_finding(
                    "error",
                    "control_artifact_index.canonical_location_exists",
                    CONTROL_ARTIFACT_INDEX_REL,
                    row_ref,
                    f"control-artifact row '{row['id']}' points at missing canonical_location '{canonical_location}'",
                    "Keep the canonical_location path real or change the row status to an explicit not_yet_seeded/outline_only sentinel.",
                    row_ref=row["id"],
                )
            )

        overview_page = row.get("overview_page")
        if overview_page and overview_page not in SENTINEL_REFS and not repo.exists(overview_page):
            findings.append(
                make_finding(
                    "error",
                    "control_artifact_index.overview_page_exists",
                    CONTROL_ARTIFACT_INDEX_REL,
                    row_ref,
                    f"control-artifact row '{row['id']}' points at missing overview_page '{overview_page}'",
                    "Keep overview_page refs updated so reviewers can trace the owning artifact and narrative together.",
                    row_ref=row["id"],
                )
            )

    for required_id, expected_path in REQUIRED_CONTROL_ROWS.items():
        row = by_id.get(required_id)
        if row is None:
            findings.append(
                make_finding(
                    "error",
                    "control_artifact_index.required_rows",
                    CONTROL_ARTIFACT_INDEX_REL,
                    CONTROL_ARTIFACT_INDEX_REL,
                    f"control-artifact index is missing required row '{required_id}'",
                    "Add the governed artifact to control_artifact_index.yaml in the same change that introduces or broadens it.",
                    row_ref=required_id,
                    expected_canonical_location=expected_path,
                )
            )
        elif row.get("canonical_location") != expected_path:
            findings.append(
                make_finding(
                    "error",
                    "control_artifact_index.required_row_location",
                    CONTROL_ARTIFACT_INDEX_REL,
                    item_ref(required_id, CONTROL_ARTIFACT_INDEX_REL),
                    f"control-artifact row '{required_id}' points at '{row.get('canonical_location')}', expected '{expected_path}'",
                    "Keep the required control-artifact row pinned to the current canonical location.",
                    row_ref=required_id,
                )
            )

    return findings


def validate_source_of_truth_map(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    source_map = repo.yaml(SOURCE_OF_TRUTH_MAP_REL)
    control_index = repo.yaml(CONTROL_ARTIFACT_INDEX_REL)
    control_ids = {row["id"] for row in control_index.get("artifacts", [])}

    truth_domains = source_map.get("truth_domains", [])
    for duplicate in register_duplicates([row["truth_domain_id"] for row in truth_domains]):
        findings.append(
            make_finding(
                "error",
                "source_of_truth_map.unique_truth_domains",
                SOURCE_OF_TRUTH_MAP_REL,
                SOURCE_OF_TRUTH_MAP_REL,
                f"source_of_truth_map duplicates truth_domain_id '{duplicate}'",
                "Keep each truth domain represented exactly once in source_of_truth_map.yaml.",
                row_ref=duplicate,
            )
        )

    for duplicate in register_duplicates([row["severity_id"] for row in source_map.get("severity_classes", [])]):
        findings.append(
            make_finding(
                "error",
                "source_of_truth_map.unique_severity_ids",
                SOURCE_OF_TRUTH_MAP_REL,
                SOURCE_OF_TRUTH_MAP_REL,
                f"source_of_truth_map duplicates severity_id '{duplicate}'",
                "Keep each public-truth severity class unique.",
                row_ref=duplicate,
            )
        )

    def require_path(ref: str, owner_ref: str, check_id: str, message: str, remediation: str) -> None:
        if ref not in SENTINEL_REFS and not existing_path_ref(repo, ref):
            findings.append(
                make_finding(
                    "error",
                    check_id,
                    SOURCE_OF_TRUTH_MAP_REL,
                    owner_ref,
                    message,
                    remediation,
                )
            )

    require_path(
        source_map.get("overview_doc", ""),
        SOURCE_OF_TRUTH_MAP_REL,
        "source_of_truth_map.overview_doc_exists",
        f"source_of_truth_map overview_doc is missing: {source_map.get('overview_doc')}",
        "Keep docs/governance/public_surface_truth_map.md aligned with source_of_truth_map.yaml.",
    )
    require_path(
        source_map.get("drift_rules_doc", ""),
        SOURCE_OF_TRUTH_MAP_REL,
        "source_of_truth_map.drift_rules_doc_exists",
        f"source_of_truth_map drift_rules_doc is missing: {source_map.get('drift_rules_doc')}",
        "Keep docs/governance/drift_blocking_rules.md aligned with source_of_truth_map.yaml.",
    )
    control_ref = source_map.get("control_artifact_index_ref", "")
    if strip_path_annotations(control_ref) != CONTROL_ARTIFACT_INDEX_REL:
        findings.append(
            make_finding(
                "error",
                "source_of_truth_map.control_index_ref",
                SOURCE_OF_TRUTH_MAP_REL,
                SOURCE_OF_TRUTH_MAP_REL,
                f"source_of_truth_map points at unexpected control_artifact_index_ref '{control_ref}'",
                "Point the source-of-truth map back at artifacts/governance/control_artifact_index.yaml.",
            )
        )
    elif "public_surface_truth_map" not in control_ids:
        findings.append(
            make_finding(
                "error",
                "source_of_truth_map.control_index_row_exists",
                CONTROL_ARTIFACT_INDEX_REL,
                CONTROL_ARTIFACT_INDEX_REL,
                "control_artifact_index.yaml no longer exposes the public_surface_truth_map row",
                "Keep the public-surface source-of-truth map indexed as a governed control artifact.",
                row_ref="public_surface_truth_map",
            )
        )

    workflow_integrations = source_map.get("workflow_integrations", {})
    for integration_id, row in workflow_integrations.items():
        owner_ref = item_ref(integration_id, SOURCE_OF_TRUTH_MAP_REL)
        for key in ("owner_artifact", "contract_ref", "narrative_ref"):
            ref = row.get(key)
            if isinstance(ref, str) and ref not in SENTINEL_REFS and not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        f"source_of_truth_map.workflow_integrations.{key}",
                        SOURCE_OF_TRUTH_MAP_REL,
                        owner_ref,
                        f"workflow integration '{integration_id}' references missing {key} '{ref}'",
                        "Update the owner/contract/narrative ref so the public-truth workflow can be traced to a real artifact.",
                        row_ref=integration_id,
                    )
                )
        for key in ("packet_refs", "required_truth_artifacts", "companion_refs"):
            for ref in row.get(key, []):
                if ref not in SENTINEL_REFS and not existing_path_ref(repo, ref):
                    findings.append(
                        make_finding(
                            "error",
                            f"source_of_truth_map.workflow_integrations.{key}",
                            SOURCE_OF_TRUTH_MAP_REL,
                            owner_ref,
                            f"workflow integration '{integration_id}' references missing artifact '{ref}' in {key}",
                            "Keep every workflow-integrated public-truth artifact real and reviewable.",
                            row_ref=integration_id,
                        )
                    )

    for domain in truth_domains:
        owner_ref = item_ref(domain["truth_domain_id"], SOURCE_OF_TRUTH_MAP_REL)
        for key in ("canonical_owner_artifact", "narrative_contract_ref"):
            ref = domain.get(key)
            if ref not in SENTINEL_REFS and not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        f"source_of_truth_map.truth_domains.{key}",
                        SOURCE_OF_TRUTH_MAP_REL,
                        owner_ref,
                        f"truth domain '{domain['truth_domain_id']}' references missing {key} '{ref}'",
                        "Point the truth domain at real owner and narrative artifacts.",
                        row_ref=domain["truth_domain_id"],
                    )
                )
        for ref in domain.get("downstream_projection_artifacts", []):
            if ref not in SENTINEL_REFS and not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        "source_of_truth_map.truth_domains.downstream_artifacts",
                        SOURCE_OF_TRUTH_MAP_REL,
                        owner_ref,
                        f"truth domain '{domain['truth_domain_id']}' references missing downstream_projection_artifact '{ref}'",
                        "Keep every downstream projection artifact ref current so truth-drift audits can follow the projection chain.",
                        row_ref=domain["truth_domain_id"],
                    )
                )

    return findings


def validate_decision_index(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    decision_index = repo.yaml(DECISION_INDEX_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    waivers = ownership.get("waivers", {})

    decision_ids = [row["decision_id"] for row in decision_index.get("decisions", [])]
    for duplicate in register_duplicates(decision_ids):
        findings.append(
            make_finding(
                "error",
                "decision_index.unique_decision_ids",
                DECISION_INDEX_REL,
                DECISION_INDEX_REL,
                f"decision_index.yaml duplicates decision_id '{duplicate}'",
                "Keep every decision row keyed by one unique decision_id.",
                row_ref=duplicate,
            )
        )

    for section_name in ("assumptions", "dependencies"):
        entries = decision_index.get(section_name, [])
        key = "id"
        duplicates = register_duplicates([row[key] for row in entries])
        for duplicate in duplicates:
            findings.append(
                make_finding(
                    "error",
                    f"decision_index.unique_{section_name}",
                    DECISION_INDEX_REL,
                    DECISION_INDEX_REL,
                    f"decision_index.yaml duplicates {section_name[:-1]} id '{duplicate}'",
                    "Keep assumption and dependency ids unique inside decision_index.yaml.",
                    row_ref=duplicate,
                )
            )

    for row in decision_index.get("decisions", []):
        decision_ref = item_ref(row["decision_id"], DECISION_INDEX_REL)
        if row.get("backup_owner") is None and row.get("backup_waiver") not in waivers:
            findings.append(
                make_finding(
                    "error",
                    "decision_index.backup_waiver_resolves",
                    DECISION_INDEX_REL,
                    decision_ref,
                    f"decision '{row['decision_id']}' is missing a valid backup_waiver for its null backup_owner",
                    "Use an active ownership_matrix waiver when a protected decision row has no named backup owner.",
                    row_ref=row["decision_id"],
                )
            )

        source_anchors = row.get("source_anchors", [])
        if not source_anchors:
            findings.append(
                make_finding(
                    "error",
                    "decision_index.source_anchors_present",
                    DECISION_INDEX_REL,
                    decision_ref,
                    f"decision '{row['decision_id']}' does not name any source_anchors",
                    "Keep source_anchors on every decision row so reviewers can trace the owning artifact and remediation note.",
                    row_ref=row["decision_id"],
                )
            )
        else:
            for anchor in source_anchors:
                doc_ref = anchor.get("doc")
                if isinstance(doc_ref, str) and not existing_path_ref(repo, doc_ref):
                    findings.append(
                        make_finding(
                            "error",
                            "decision_index.source_anchor_docs_exist",
                            DECISION_INDEX_REL,
                            decision_ref,
                            f"decision '{row['decision_id']}' references missing source-anchor doc '{doc_ref}'",
                            "Fix source_anchors.doc so the decision row points at a real source artifact.",
                            row_ref=row["decision_id"],
                        )
                    )

        linked_adr = row.get("linked_adr")
        if row.get("status") == "decided" and not linked_adr:
            findings.append(
                make_finding(
                    "error",
                    "decision_index.decided_rows_link_adr",
                    DECISION_INDEX_REL,
                    decision_ref,
                    f"decided decision '{row['decision_id']}' is missing linked_adr",
                    "Every decided decision row must point at the ADR that closed it.",
                    row_ref=row["decision_id"],
                )
            )
        elif linked_adr and not existing_path_ref(repo, linked_adr):
            findings.append(
                make_finding(
                    "error",
                    "decision_index.linked_adr_exists",
                    DECISION_INDEX_REL,
                    decision_ref,
                    f"decision '{row['decision_id']}' references missing linked_adr '{linked_adr}'",
                    "Keep linked_adr pointed at a real ADR path.",
                    row_ref=row["decision_id"],
                )
            )

    return findings


def validate_interface_freeze_matrix(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    freeze_matrix = repo.yaml(INTERFACE_FREEZE_MATRIX_REL)
    rows = freeze_matrix.get("rows", [])
    row_ids = [row["row_id"] for row in rows]
    known_row_ids = set(row_ids)

    for duplicate in register_duplicates(row_ids):
        findings.append(
            make_finding(
                "error",
                "interface_freeze_matrix.unique_row_ids",
                INTERFACE_FREEZE_MATRIX_REL,
                INTERFACE_FREEZE_MATRIX_REL,
                f"interface_freeze_matrix duplicates row_id '{duplicate}'",
                "Keep every freeze-matrix row id unique.",
                row_ref=duplicate,
            )
        )

    gate_ids = [row["gate_id"] for row in freeze_matrix.get("gate_row_map", [])]
    for duplicate in register_duplicates(gate_ids):
        findings.append(
            make_finding(
                "error",
                "interface_freeze_matrix.unique_gate_ids",
                INTERFACE_FREEZE_MATRIX_REL,
                INTERFACE_FREEZE_MATRIX_REL,
                f"interface_freeze_matrix duplicates gate_id '{duplicate}'",
                "Keep every gate_row_map entry keyed by a unique gate id.",
                row_ref=duplicate,
            )
        )

    for gate in freeze_matrix.get("gate_row_map", []):
        gate_ref = item_ref(gate["gate_id"], INTERFACE_FREEZE_MATRIX_REL)
        for required_row in gate.get("required_rows", []):
            if required_row not in known_row_ids:
                findings.append(
                    make_finding(
                        "error",
                        "interface_freeze_matrix.required_rows_resolve",
                        INTERFACE_FREEZE_MATRIX_REL,
                        gate_ref,
                        f"gate '{gate['gate_id']}' references unknown required_row '{required_row}'",
                        "Keep gate_row_map.required_rows aligned with the freeze-matrix row ids.",
                        row_ref=gate["gate_id"],
                    )
                )

    for row in rows:
        row_ref = item_ref(row["row_id"], INTERFACE_FREEZE_MATRIX_REL)
        if row.get("freeze_status") in {"frozen", "provisional"} and not row.get("canonical_refs"):
            findings.append(
                make_finding(
                    "error",
                    "interface_freeze_matrix.traceability_refs",
                    INTERFACE_FREEZE_MATRIX_REL,
                    row_ref,
                    f"freeze-matrix row '{row['row_id']}' has no canonical_refs despite being {row.get('freeze_status')}",
                    "Keep at least one canonical ref on every frozen or provisional row so downstream work can trace the owning artifact.",
                    row_ref=row["row_id"],
                )
            )
        for ref in row.get("canonical_refs", []):
            if not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        "interface_freeze_matrix.canonical_refs_exist",
                        INTERFACE_FREEZE_MATRIX_REL,
                        row_ref,
                        f"freeze-matrix row '{row['row_id']}' references missing canonical_ref '{ref}'",
                        "Keep canonical_refs updated so the freeze row remains traceable back to its owning artifact.",
                        row_ref=row["row_id"],
                    )
                )

    return findings


def validate_stable_surface_inventory(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    inventory = repo.yaml(STABLE_SURFACE_INVENTORY_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    compat = repo.yaml(QUALIFICATION_MATRIX_REL)
    surface_rows = inventory.get("rows", [])
    waivers = ownership.get("waivers", {})
    lanes = {row["id"] for row in ownership.get("governance_lanes", [])}
    compat_rows = {row["row_id"] for row in compat.get("qualification_rows", [])}
    surface_ids = [row["surface_id"] for row in surface_rows]
    surface_id_set = set(surface_ids)

    for duplicate in register_duplicates(surface_ids):
        findings.append(
            make_finding(
                "error",
                "stable_surface_inventory.unique_surface_ids",
                STABLE_SURFACE_INVENTORY_REL,
                STABLE_SURFACE_INVENTORY_REL,
                f"stable_surface_inventory duplicates surface_id '{duplicate}'",
                "Keep every stable-surface row keyed by one unique surface_id.",
                row_ref=duplicate,
            )
        )

    for row in surface_rows:
        surface_ref = item_ref(row["surface_id"], STABLE_SURFACE_INVENTORY_REL)
        maturity = row.get("maturity_lane")
        severity = "error" if maturity in {"stable", "beta"} else "warning"
        ownership_row = row.get("ownership", {})
        if not ownership_row.get("owner_dri"):
            findings.append(
                make_finding(
                    severity,
                    "stable_surface_inventory.owner_present",
                    STABLE_SURFACE_INVENTORY_REL,
                    surface_ref,
                    f"surface '{row['surface_id']}' is missing ownership.owner_dri",
                    "Populate ownership.owner_dri so the surface has a named accountable owner.",
                    row_ref=row["surface_id"],
                )
            )
        owning_lane = ownership_row.get("owning_lane")
        if not owning_lane or owning_lane not in lanes:
            findings.append(
                make_finding(
                    severity,
                    "stable_surface_inventory.owning_lane_resolves",
                    STABLE_SURFACE_INVENTORY_REL,
                    surface_ref,
                    f"surface '{row['surface_id']}' points at unknown owning_lane '{owning_lane}'",
                    "Use an ownership_matrix governance_lanes id for every stable-surface row.",
                    row_ref=row["surface_id"],
                )
            )
        if ownership_row.get("backup_owner") is None and ownership_row.get("backup_waiver") not in waivers:
            findings.append(
                make_finding(
                    severity,
                    "stable_surface_inventory.backup_waiver_resolves",
                    STABLE_SURFACE_INVENTORY_REL,
                    surface_ref,
                    f"surface '{row['surface_id']}' is missing a valid backup_waiver for its null backup_owner",
                    "Use an active ownership_matrix waiver when a stable-surface row has no named backup owner.",
                    row_ref=row["surface_id"],
                )
            )

        versioning = row.get("versioning", {})
        lifecycle = row.get("lifecycle", {})
        review = row.get("review", {})
        required_blocks = {
            "versioning.rule_summary": versioning.get("rule_summary"),
            "versioning.compatibility_window_source_ref": versioning.get("compatibility_window_source_ref"),
            "lifecycle.deprecation_posture": lifecycle.get("deprecation_posture"),
            "review.review_cadence": review.get("review_cadence"),
        }
        for label, value in required_blocks.items():
            if not value:
                findings.append(
                    make_finding(
                        severity,
                        f"stable_surface_inventory.{label}",
                        STABLE_SURFACE_INVENTORY_REL,
                        surface_ref,
                        f"surface '{row['surface_id']}' is missing {label}",
                        "Populate the missing lifecycle/versioning/review field so the surface remains governable.",
                        row_ref=row["surface_id"],
                    )
                )

        publication = row.get("publication", {})
        publication_refs = publication.get("publication_artifact_refs", [])
        schema_refs = publication.get("schema_or_interface_directory_refs", [])
        if maturity in {"stable", "beta"}:
            if not publication_refs:
                findings.append(
                    make_finding(
                        "error",
                        "stable_surface_inventory.publication_refs_present",
                        STABLE_SURFACE_INVENTORY_REL,
                        surface_ref,
                        f"stable-facing surface '{row['surface_id']}' has no publication_artifact_refs",
                        "Stable and beta surfaces must cite their schema/interface packet refs directly.",
                        row_ref=row["surface_id"],
                    )
                )
            if not schema_refs:
                findings.append(
                    make_finding(
                        "error",
                        "stable_surface_inventory.schema_refs_present",
                        STABLE_SURFACE_INVENTORY_REL,
                        surface_ref,
                        f"stable-facing surface '{row['surface_id']}' has no schema_or_interface_directory_refs",
                        "Stable and beta surfaces must cite their schema/interface directories directly.",
                        row_ref=row["surface_id"],
                    )
                )

        for ref in publication_refs + schema_refs + review.get("docs_touchpoint_refs", []):
            if ref in SENTINEL_REFS:
                if maturity in {"stable", "beta"}:
                    findings.append(
                        make_finding(
                            "error",
                            "stable_surface_inventory.placeholder_refs",
                            STABLE_SURFACE_INVENTORY_REL,
                            surface_ref,
                            f"stable-facing surface '{row['surface_id']}' still uses placeholder ref '{ref}'",
                            "Replace placeholders with the actual schema/interface packet refs before a beta or stable surface broadens.",
                            row_ref=row["surface_id"],
                        )
                    )
                continue
            if not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        severity,
                        "stable_surface_inventory.path_refs_exist",
                        STABLE_SURFACE_INVENTORY_REL,
                        surface_ref,
                        f"surface '{row['surface_id']}' references missing artifact '{ref}'",
                        "Keep publication, schema/interface, and docs touchpoint refs current for every surface row.",
                        row_ref=row["surface_id"],
                    )
                )

        compat_ref = normalize_text_block(versioning.get("compatibility_window_source_ref", ""))
        if compat_ref:
            path = strip_path_annotations(compat_ref)
            if compat_ref.startswith(f"{QUALIFICATION_MATRIX_REL}#"):
                fragment = compat_ref.split("#", 1)[1]
                if fragment not in compat_rows:
                    findings.append(
                        make_finding(
                            severity,
                            "stable_surface_inventory.compatibility_ref_resolves",
                            STABLE_SURFACE_INVENTORY_REL,
                            surface_ref,
                            f"surface '{row['surface_id']}' points at unknown compatibility row '{fragment}'",
                            "Point compatibility_window_source_ref at a real qualification_matrix row id.",
                            row_ref=row["surface_id"],
                        )
                    )
            elif compat_ref.startswith(f"{STABLE_SURFACE_INVENTORY_REL}#"):
                fragment = compat_ref.split("#", 1)[1]
                if fragment not in surface_id_set:
                    findings.append(
                        make_finding(
                            severity,
                            "stable_surface_inventory.self_compatibility_ref_resolves",
                            STABLE_SURFACE_INVENTORY_REL,
                            surface_ref,
                            f"surface '{row['surface_id']}' points at unknown stable-surface row '{fragment}'",
                            "Point compatibility_window_source_ref at a real stable_surface_inventory row id when self-referencing.",
                            row_ref=row["surface_id"],
                        )
                    )
            elif not repo.exists(path):
                findings.append(
                    make_finding(
                        severity,
                        "stable_surface_inventory.compatibility_ref_path_exists",
                        STABLE_SURFACE_INVENTORY_REL,
                        surface_ref,
                        f"surface '{row['surface_id']}' points at missing compatibility_window_source_ref '{compat_ref}'",
                        "Use a real compatibility row or governed source artifact as the compatibility_window_source_ref.",
                        row_ref=row["surface_id"],
                    )
                )

    return findings


def validate_compatibility_matrix(repo: RepoView, boundary_profiles: list[str]) -> list[Finding]:
    findings: list[Finding] = []
    matrix = repo.yaml(QUALIFICATION_MATRIX_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    skew_register = repo.yaml(VERSION_SKEW_REGISTER_REL)
    row_ids = [row["row_id"] for row in matrix.get("qualification_rows", [])]
    compat_rows = {row["row_id"] for row in matrix.get("qualification_rows", [])}
    deployment_profiles = matrix.get("deployment_profile_vocabulary", [])
    owner_packages = {row["name"] for row in ownership.get("packages", [])}
    owner_lanes = {row["id"] for row in ownership.get("governance_lanes", [])}
    register_ids = {row["register_id"] for row in skew_register.get("register", [])}

    if sorted(boundary_profiles) != sorted(deployment_profiles):
        findings.append(
            make_finding(
                "error",
                "compatibility_matrix.deployment_profile_vocabulary",
                QUALIFICATION_MATRIX_REL,
                QUALIFICATION_MATRIX_REL,
                "compatibility_matrix deployment_profile_vocabulary drifted from the boundary-manifest deployment-profile table",
                "Keep compatibility and boundary-manifest deployment-profile ids aligned in the same change.",
                boundary_manifest_profiles=boundary_profiles,
                compatibility_matrix_profiles=deployment_profiles,
            )
        )

    for duplicate in register_duplicates(row_ids):
        findings.append(
            make_finding(
                "error",
                "compatibility_matrix.unique_row_ids",
                QUALIFICATION_MATRIX_REL,
                QUALIFICATION_MATRIX_REL,
                f"compatibility_matrix duplicates row_id '{duplicate}'",
                "Keep every qualification row keyed by a unique row_id.",
                row_ref=duplicate,
            )
        )

    for row in matrix.get("qualification_rows", []):
        row_ref = item_ref(row["row_id"], QUALIFICATION_MATRIX_REL)
        unknown_profiles = sorted(set(row.get("claimed_deployment_profiles", [])) - set(deployment_profiles))
        if unknown_profiles:
            findings.append(
                make_finding(
                    "error",
                    "compatibility_matrix.deployment_profiles_resolve",
                    QUALIFICATION_MATRIX_REL,
                    row_ref,
                    f"compatibility row '{row['row_id']}' uses unknown deployment profile ids: {', '.join(unknown_profiles)}",
                    "Use only deployment-profile ids from the boundary-manifest vocabulary.",
                    row_ref=row["row_id"],
                )
            )

        owner_ref = row.get("owner_ref", "")
        if not resolve_owner_ref(owner_ref, owner_packages, owner_lanes):
            findings.append(
                make_finding(
                    "error",
                    "compatibility_matrix.owner_ref_resolves",
                    QUALIFICATION_MATRIX_REL,
                    row_ref,
                    f"compatibility row '{row['row_id']}' uses unknown owner_ref '{owner_ref}'",
                    "Use lane:<id> or package:<name> values that resolve in artifacts/governance/ownership_matrix.yaml.",
                    row_ref=row["row_id"],
                )
            )

        version_skew_ref = row.get("version_skew_register_ref")
        if version_skew_ref not in register_ids:
            findings.append(
                make_finding(
                    "error",
                    "compatibility_matrix.version_skew_ref_resolves",
                    QUALIFICATION_MATRIX_REL,
                    row_ref,
                    f"compatibility row '{row['row_id']}' points at unknown version_skew_register_ref '{version_skew_ref}'",
                    "Point version_skew_register_ref at a real register_id in artifacts/compat/version_skew_register.yaml.",
                    row_ref=row["row_id"],
                )
            )

        for ref in row.get("supporting_artifact_refs", []):
            if not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        "compatibility_matrix.supporting_artifacts_exist",
                        QUALIFICATION_MATRIX_REL,
                        row_ref,
                        f"compatibility row '{row['row_id']}' references missing supporting_artifact_ref '{ref}'",
                        "Keep supporting_artifact_refs current so reviewers can trace the boundary row back to its owning artifact.",
                        row_ref=row["row_id"],
                    )
                )

    # Keep the compatibility registry available to downstream claim/boundary checks.
    if not compat_rows:
        findings.append(
            make_finding(
                "error",
                "compatibility_matrix.rows_present",
                QUALIFICATION_MATRIX_REL,
                QUALIFICATION_MATRIX_REL,
                "qualification_matrix_seed.yaml no longer defines any qualification_rows",
                "Keep the compatibility matrix seeded so claim and release artifacts have canonical boundary row ids to cite.",
            )
        )

    return findings


def validate_boundary_manifest(repo: RepoView, compatibility_profiles: list[str]) -> tuple[list[str], list[Finding]]:
    findings: list[Finding] = []
    text = repo.text(BOUNDARY_MANIFEST_REL)
    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    decision_index = repo.yaml(DECISION_INDEX_REL)
    lane_ids = {row["id"] for row in ownership.get("governance_lanes", [])}
    package_ids = {row["name"] for row in ownership.get("packages", [])}
    decision_ids = {row["decision_id"] for row in decision_index.get("decisions", [])}

    deployment_profiles, rows = parse_boundary_manifest(text)

    for duplicate in register_duplicates(deployment_profiles):
        findings.append(
            make_finding(
                "error",
                "boundary_manifest.unique_profile_ids",
                BOUNDARY_MANIFEST_REL,
                BOUNDARY_MANIFEST_REL,
                f"boundary manifest duplicates deployment-profile id '{duplicate}'",
                "Keep deployment-profile ids unique in the boundary manifest table.",
                row_ref=duplicate,
            )
        )

    for duplicate in register_duplicates([row["capability_id"] for row in rows]):
        findings.append(
            make_finding(
                "error",
                "boundary_manifest.unique_capability_ids",
                BOUNDARY_MANIFEST_REL,
                BOUNDARY_MANIFEST_REL,
                f"boundary manifest duplicates capability id '{duplicate}'",
                "Keep exactly one boundary row per capability id.",
                row_ref=duplicate,
            )
        )

    if sorted(deployment_profiles) != sorted(compatibility_profiles):
        findings.append(
            make_finding(
                "error",
                "boundary_manifest.deployment_profile_vocabulary",
                BOUNDARY_MANIFEST_REL,
                BOUNDARY_MANIFEST_REL,
                "boundary-manifest deployment-profile ids drifted from the compatibility matrix vocabulary",
                "Update docs/product/boundary_manifest_strawman.md and artifacts/compat/qualification_matrix_seed.yaml in the same change.",
                boundary_manifest_profiles=deployment_profiles,
                compatibility_matrix_profiles=compatibility_profiles,
            )
        )

    for row in rows:
        row_ref = item_ref(row["capability_id"], BOUNDARY_MANIFEST_REL)
        unknown_profiles = sorted(set(row["deployment_profiles"]) - set(deployment_profiles))
        if unknown_profiles:
            findings.append(
                make_finding(
                    "error",
                    "boundary_manifest.row_profiles_resolve",
                    BOUNDARY_MANIFEST_REL,
                    row_ref,
                    f"boundary row '{row['capability_id']}' uses unknown deployment-profile ids: {', '.join(unknown_profiles)}",
                    "Use only deployment-profile ids from the boundary-manifest vocabulary table.",
                    row_ref=row["capability_id"],
                )
            )

        linked_decisions = row["linked_decisions"]
        if row["status"] == "accepted" and not linked_decisions:
            findings.append(
                make_finding(
                    "error",
                    "boundary_manifest.accepted_rows_link_decisions",
                    BOUNDARY_MANIFEST_REL,
                    row_ref,
                    f"accepted boundary row '{row['capability_id']}' does not cite any decision-register rows",
                    "Accepted boundary rows must point at the decision register rows that ratified them.",
                    row_ref=row["capability_id"],
                )
            )
        missing_decisions = sorted(set(linked_decisions) - decision_ids)
        if missing_decisions:
            severity = "error" if row["status"] == "accepted" else "warning"
            findings.append(
                make_finding(
                    severity,
                    "boundary_manifest.linked_decisions_resolve",
                    BOUNDARY_MANIFEST_REL,
                    row_ref,
                    f"boundary row '{row['capability_id']}' cites unknown decision ids: {', '.join(missing_decisions)}",
                    "Point Linked decisions at real decision_index rows or remove the stale references.",
                    row_ref=row["capability_id"],
                )
            )

        unknown_lanes = sorted(
            lane for lane in row["linked_lanes"] if lane not in lane_ids and lane not in package_ids
        )
        if unknown_lanes:
            findings.append(
                make_finding(
                    "error",
                    "boundary_manifest.linked_lanes_resolve",
                    BOUNDARY_MANIFEST_REL,
                    row_ref,
                    f"boundary row '{row['capability_id']}' cites unknown linked lanes/packages: {', '.join(unknown_lanes)}",
                    "Use ownership_matrix governance_lanes ids or package names in Linked lanes.",
                    row_ref=row["capability_id"],
                )
            )

    return deployment_profiles, findings


def validate_claim_manifest(repo: RepoView, deployment_profiles: list[str]) -> list[Finding]:
    findings: list[Finding] = []
    manifest = repo.yaml(CLAIM_MANIFEST_REL)
    compatibility = repo.yaml(QUALIFICATION_MATRIX_REL)
    skew_register = repo.yaml(VERSION_SKEW_REGISTER_REL)
    compat_rows = {row["row_id"] for row in compatibility.get("qualification_rows", [])}
    skew_rows = {row["register_id"] for row in skew_register.get("register", [])}
    claim_rows = manifest.get("claim_rows", [])
    claim_row_ids = [row["claim_row_id"] for row in claim_rows]

    for duplicate in register_duplicates(claim_row_ids):
        findings.append(
            make_finding(
                "error",
                "claim_manifest.unique_claim_row_ids",
                CLAIM_MANIFEST_REL,
                CLAIM_MANIFEST_REL,
                f"claim_manifest duplicates claim_row_id '{duplicate}'",
                "Keep every claim row keyed by one unique claim_row_id.",
                row_ref=duplicate,
            )
        )

    claim_row_set = set(claim_row_ids)
    for ref in manifest.get("shared_header", {}).get("coverage", {}).get("claim_row_refs", []):
        if ref not in claim_row_set:
            findings.append(
                make_finding(
                    "error",
                    "claim_manifest.shared_header_claim_refs_resolve",
                    CLAIM_MANIFEST_REL,
                    CLAIM_MANIFEST_REL,
                    f"shared_header references missing claim_row_ref '{ref}'",
                    "Keep shared_header.coverage.claim_row_refs aligned with the concrete claim_rows list.",
                    row_ref=ref,
                )
            )

    for row in claim_rows:
        row_ref = item_ref(row["claim_row_id"], CLAIM_MANIFEST_REL)
        unknown_profiles = sorted(
            set(row.get("claim_scope", {}).get("deployment_profiles", [])) - set(deployment_profiles)
        )
        if unknown_profiles:
            findings.append(
                make_finding(
                    "error",
                    "claim_manifest.deployment_profiles_resolve",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row['claim_row_id']}' uses unknown deployment-profile ids: {', '.join(unknown_profiles)}",
                    "Use only deployment-profile ids from the shared boundary/compatibility vocabulary.",
                    row_ref=row["claim_row_id"],
                )
            )

        for compat_ref in row.get("compatibility_row_refs", []):
            if compat_ref not in compat_rows:
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.compatibility_refs_resolve",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' references unknown compatibility_row_ref '{compat_ref}'",
                        "Point compatibility_row_refs at real qualification_matrix row ids.",
                        row_ref=row["claim_row_id"],
                    )
                )

        for skew_ref in row.get("version_skew_register_refs", []):
            if skew_ref not in skew_rows:
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.version_skew_refs_resolve",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' references unknown version_skew_register_ref '{skew_ref}'",
                        "Point version_skew_register_refs at real version-skew register ids.",
                        row_ref=row["claim_row_id"],
                    )
                )

        for ref in row.get("source_anchor_refs", []):
            if not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.source_anchor_refs_exist",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' references missing source_anchor_ref '{ref}'",
                        "Keep source_anchor_refs pointed at real upstream artifacts.",
                        row_ref=row["claim_row_id"],
                    )
                )

        for ref in row.get("known_limit_refs", []) + row.get("exclusion_note_refs", []):
            if not existing_path_ref(repo, ref):
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.caveat_refs_exist",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' references missing caveat ref '{ref}'",
                        "Keep known_limit_refs and exclusion_note_refs pointed at real caveat artifacts.",
                        row_ref=row["claim_row_id"],
                    )
                )

        support_window_ref = row.get("lifecycle_support", {}).get("support_window_ref")
        if isinstance(support_window_ref, str) and not existing_path_ref(repo, support_window_ref):
            findings.append(
                make_finding(
                    "error",
                    "claim_manifest.support_window_ref_exists",
                    CLAIM_MANIFEST_REL,
                    row_ref,
                    f"claim row '{row['claim_row_id']}' points at missing support_window_ref '{support_window_ref}'",
                    "Keep support_window_ref tied to a real support-window artifact.",
                    row_ref=row["claim_row_id"],
                )
            )

        known_limit_refs = set(row.get("known_limit_refs", []))
        exclusion_note_refs = set(row.get("exclusion_note_refs", []))
        for channel_id, binding in row.get("channel_bindings", {}).items():
            surface_ref = binding.get("surface_ref")
            if isinstance(surface_ref, str) and surface_ref not in SENTINEL_REFS and not existing_path_ref(repo, surface_ref):
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.channel_surface_refs_exist",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' channel '{channel_id}' points at missing surface_ref '{surface_ref}'",
                        "Keep every required claim-manifest channel binding tied to a real surface artifact.",
                        row_ref=row["claim_row_id"],
                    )
                )

            missing_known_limits = sorted(set(binding.get("required_known_limit_refs", [])) - known_limit_refs)
            if missing_known_limits:
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.required_known_limit_refs",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' channel '{channel_id}' requires known-limit refs that the row does not carry: {', '.join(missing_known_limits)}",
                        "Add the required caveat refs to the claim row or remove the stale channel-binding requirement.",
                        row_ref=row["claim_row_id"],
                    )
                )

            missing_exclusion_refs = sorted(set(binding.get("required_exclusion_note_refs", [])) - exclusion_note_refs)
            if missing_exclusion_refs:
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.required_exclusion_note_refs",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' channel '{channel_id}' requires exclusion-note refs that the row does not carry: {', '.join(missing_exclusion_refs)}",
                        "Add the required exclusion notes to the claim row or remove the stale channel-binding requirement.",
                        row_ref=row["claim_row_id"],
                    )
                )

        for evidence_link in row.get("evidence_links", []):
            packet_ref = evidence_link.get("packet_ref")
            if isinstance(packet_ref, str) and not existing_path_ref(repo, packet_ref):
                findings.append(
                    make_finding(
                        "error",
                        "claim_manifest.evidence_packet_refs_exist",
                        CLAIM_MANIFEST_REL,
                        row_ref,
                        f"claim row '{row['claim_row_id']}' references missing evidence packet '{packet_ref}'",
                        "Keep evidence_links.packet_ref tied to a real packet or source artifact.",
                        row_ref=row["claim_row_id"],
                    )
                )

    return findings


def validate_command_parity(repo: RepoView) -> tuple[list[Finding], dict[str, Any] | None]:
    findings: list[Finding] = []
    tool_path = repo.rel(COMMAND_PARITY_TOOL_REL)
    if not tool_path.exists():
        findings.append(
            make_finding(
                "error",
                "command_parity.tool_exists",
                COMMAND_PARITY_TOOL_REL,
                COMMAND_PARITY_TOOL_REL,
                f"command parity tool is missing: {COMMAND_PARITY_TOOL_REL}",
                "Keep tools/commands/parity_diff_seed.py present so command and CLI surface validation remains enforceable.",
            )
        )
        return findings, None

    proc = subprocess.run(
        [
            sys.executable,
            str(tool_path),
            "--repo-root",
            str(repo.root),
            "--format",
            "json",
        ],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        findings.append(
            make_finding(
                "error",
                "command_parity.tool_execution",
                COMMAND_PARITY_SEED_REL,
                item_ref("command_plane.command_descriptor_and_invocation_session", STABLE_SURFACE_INVENTORY_REL),
                f"command parity tool failed: {(proc.stderr or proc.stdout).strip()}",
                "Fix tools/commands/parity_diff_seed.py or the command-parity seed corpus before widening command or CLI surfaces.",
            )
        )
        return findings, None

    try:
        analysis = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        findings.append(
            make_finding(
                "error",
                "command_parity.json_output",
                COMMAND_PARITY_SEED_REL,
                COMMAND_PARITY_TOOL_REL,
                f"command parity tool emitted invalid JSON: {exc}",
                "Keep the command parity tool's machine-readable report valid JSON.",
            )
        )
        return findings, None

    return findings, analysis


def validate_principle_checks(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    if not repo.exists(PRINCIPLE_CHECKS_REL):
        findings.append(
            make_finding(
                "error",
                "principle_checks.file_exists",
                PRINCIPLE_CHECKS_REL,
                PRINCIPLE_CHECKS_REL,
                f"missing architecture principle checks: {PRINCIPLE_CHECKS_REL}",
                "Add artifacts/architecture/principle_checks.yaml and register it in control_artifact_index.yaml.",
            )
        )
        return findings

    payload = repo.yaml(PRINCIPLE_CHECKS_REL)
    if payload.get("artifact_id") != "aureline.architecture_principle_checks":
        findings.append(
            make_finding(
                "error",
                "principle_checks.artifact_id",
                PRINCIPLE_CHECKS_REL,
                PRINCIPLE_CHECKS_REL,
                "principle_checks artifact_id must be 'aureline.architecture_principle_checks'",
                "Fix artifacts/architecture/principle_checks.yaml artifact_id to match the canonical id.",
            )
        )

    if payload.get("overview_document") != PRINCIPLE_ENFORCEMENT_MATRIX_DOC_REL:
        findings.append(
            make_finding(
                "error",
                "principle_checks.overview_document",
                PRINCIPLE_CHECKS_REL,
                PRINCIPLE_CHECKS_REL,
                "principle_checks overview_document must point at the enforcement matrix doc",
                f"Set overview_document to {PRINCIPLE_ENFORCEMENT_MATRIX_DOC_REL}.",
            )
        )

    principles = payload.get("principles")
    if not isinstance(principles, list) or not principles:
        findings.append(
            make_finding(
                "error",
                "principle_checks.principles_list",
                PRINCIPLE_CHECKS_REL,
                PRINCIPLE_CHECKS_REL,
                "principle_checks must define a non-empty top-level principles list",
                "Populate artifacts/architecture/principle_checks.yaml with one row per architecture principle.",
            )
        )
        return findings

    catalog = repo.yaml(FITNESS_CATALOG_REL)
    allowed_principles = set(catalog.get("architecture_principles", []))
    if not allowed_principles:
        raise SystemExit(f"{FITNESS_CATALOG_REL} must define architecture_principles vocabulary")

    rejected = repo.yaml(REJECTED_PATTERNS_REL)
    rejected_ids = {row.get("pattern_id") for row in rejected.get("rejected_patterns", []) if isinstance(row, dict)}

    mandatory_review = repo.yaml(MANDATORY_REVIEW_ARTIFACTS_REL)
    artifact_classes = {
        row.get("artifact_class_id")
        for row in mandatory_review.get("artifact_classes", [])
        if isinstance(row, dict) and row.get("artifact_class_id")
    }

    ownership = repo.yaml(OWNERSHIP_MATRIX_REL)
    decision_forums = {row.get("id") for row in ownership.get("decision_forums", []) if isinstance(row, dict)}

    principle_ids = [row.get("principle_id") for row in principles if isinstance(row, dict)]
    for duplicate in register_duplicates([pid for pid in principle_ids if isinstance(pid, str)]):
        findings.append(
            make_finding(
                "error",
                "principle_checks.unique_principle_ids",
                PRINCIPLE_CHECKS_REL,
                PRINCIPLE_CHECKS_REL,
                f"principle_checks duplicates principle_id '{duplicate}'",
                "Keep exactly one principle_checks row per architecture principle id.",
                row_ref=duplicate,
            )
        )

    for row in principles:
        if not isinstance(row, dict):
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.row_shape",
                    PRINCIPLE_CHECKS_REL,
                    PRINCIPLE_CHECKS_REL,
                    "principle_checks contains a non-object row",
                    "Keep every principles[] entry a YAML mapping/object.",
                )
            )
            continue

        principle_id = row.get("principle_id")
        row_ref = None if not isinstance(principle_id, str) else f"principle_id:{principle_id}"

        if not isinstance(principle_id, str) or principle_id not in allowed_principles:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.principle_id_vocabulary",
                    PRINCIPLE_CHECKS_REL,
                    FITNESS_CATALOG_REL,
                    f"principle_checks row uses unknown principle_id '{principle_id}'",
                    "Use a principle_id from artifacts/bench/fitness_function_catalog.yaml#architecture_principles.",
                    row_ref=row_ref,
                    allowed_count=len(allowed_principles),
                )
            )

        for required_field in ("title", "statement", "protected_paths", "invariants", "enforcement"):
            if required_field not in row:
                findings.append(
                    make_finding(
                        "error",
                        f"principle_checks.required_field.{required_field}",
                        PRINCIPLE_CHECKS_REL,
                        PRINCIPLE_CHECKS_REL,
                        f"principle_checks row '{principle_id}' missing required field '{required_field}'",
                        "Populate the missing required field so reviewers and CI have a complete checklist.",
                        row_ref=row_ref,
                    )
                )

        invariants = row.get("invariants")
        if not isinstance(invariants, list) or not invariants:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.invariants_non_empty",
                    PRINCIPLE_CHECKS_REL,
                    PRINCIPLE_CHECKS_REL,
                    f"principle_checks row '{principle_id}' must define a non-empty invariants list",
                    "Add at least one invariant that names what must hold and what failure looks like.",
                    row_ref=row_ref,
                )
            )

        protected_paths = row.get("protected_paths", {})
        if isinstance(protected_paths, dict):
            for ref in protected_paths.get("control_artifact_refs", []) or []:
                if isinstance(ref, str) and not existing_path_ref(repo, ref):
                    findings.append(
                        make_finding(
                            "error",
                            "principle_checks.control_artifact_ref_exists",
                            PRINCIPLE_CHECKS_REL,
                            ref,
                            f"principle_checks row '{principle_id}' references missing control artifact '{ref}'",
                            "Keep control_artifact_refs limited to real repo paths (or remove the ref).",
                            row_ref=row_ref,
                        )
                    )

        for anti_ref in row.get("anti_pattern_refs", []) or []:
            if not isinstance(anti_ref, str) or anti_ref not in rejected_ids:
                findings.append(
                    make_finding(
                        "warning",
                        "principle_checks.anti_pattern_ref_resolves",
                        PRINCIPLE_CHECKS_REL,
                        REJECTED_PATTERNS_REL,
                        f"principle_checks row '{principle_id}' cites unknown anti-pattern ref '{anti_ref}'",
                        "Prefer refs from artifacts/architecture/driver_to_rejected_pattern_refs.yaml#rejected_patterns[].pattern_id.",
                        row_ref=row_ref,
                    )
                )

        enforcement = row.get("enforcement")
        if not isinstance(enforcement, dict):
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.enforcement_object",
                    PRINCIPLE_CHECKS_REL,
                    PRINCIPLE_CHECKS_REL,
                    f"principle_checks row '{principle_id}' enforcement must be an object",
                    "Define enforcement.controlling_forums, waiver_authority_forum, evidence_packet_families, and machine_gates.",
                    row_ref=row_ref,
                )
            )
            continue

        controlling_forums = enforcement.get("controlling_forums")
        if not isinstance(controlling_forums, list) or not controlling_forums:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.controlling_forums_non_empty",
                    PRINCIPLE_CHECKS_REL,
                    OWNERSHIP_MATRIX_REL,
                    f"principle_checks row '{principle_id}' must name at least one controlling forum",
                    "Add at least one forum id from artifacts/governance/ownership_matrix.yaml#decision_forums.",
                    row_ref=row_ref,
                )
            )
        else:
            unknown = sorted({forum for forum in controlling_forums if isinstance(forum, str)} - decision_forums)
            if unknown:
                findings.append(
                    make_finding(
                        "error",
                        "principle_checks.controlling_forums_resolve",
                        PRINCIPLE_CHECKS_REL,
                        OWNERSHIP_MATRIX_REL,
                        f"principle_checks row '{principle_id}' references unknown controlling forum(s): {', '.join(unknown)}",
                        "Use decision forum ids from artifacts/governance/ownership_matrix.yaml#decision_forums.",
                        row_ref=row_ref,
                    )
                )

        waiver_forum = enforcement.get("waiver_authority_forum")
        if not isinstance(waiver_forum, str) or waiver_forum not in decision_forums:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.waiver_authority_forum_resolves",
                    PRINCIPLE_CHECKS_REL,
                    OWNERSHIP_MATRIX_REL,
                    f"principle_checks row '{principle_id}' has unknown waiver_authority_forum '{waiver_forum}'",
                    "Set waiver_authority_forum to a decision forum id from artifacts/governance/ownership_matrix.yaml#decision_forums.",
                    row_ref=row_ref,
                )
            )

        evidence_families = enforcement.get("evidence_packet_families")
        if not isinstance(evidence_families, list) or not evidence_families:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.evidence_packet_families_non_empty",
                    PRINCIPLE_CHECKS_REL,
                    MANDATORY_REVIEW_ARTIFACTS_REL,
                    f"principle_checks row '{principle_id}' must name at least one evidence packet family",
                    "List one or more artifact_class_id values from artifacts/governance/mandatory_review_artifacts.yaml#artifact_classes.",
                    row_ref=row_ref,
                )
            )
        else:
            unknown = sorted({family for family in evidence_families if isinstance(family, str)} - artifact_classes)
            if unknown:
                findings.append(
                    make_finding(
                        "error",
                        "principle_checks.evidence_packet_families_resolve",
                        PRINCIPLE_CHECKS_REL,
                        MANDATORY_REVIEW_ARTIFACTS_REL,
                        f"principle_checks row '{principle_id}' references unknown evidence packet family(s): {', '.join(unknown)}",
                        "Use artifact_class_id values from artifacts/governance/mandatory_review_artifacts.yaml.",
                        row_ref=row_ref,
                    )
                )

        machine_gates = enforcement.get("machine_gates")
        if not isinstance(machine_gates, list) or not machine_gates:
            findings.append(
                make_finding(
                    "error",
                    "principle_checks.machine_gates_non_empty",
                    PRINCIPLE_CHECKS_REL,
                    PRINCIPLE_CHECKS_REL,
                    f"principle_checks row '{principle_id}' must define at least one machine gate or explicit review gate",
                    "Add at least one enforcement.machine_gates[] entry (or explicitly scope the principle to review-only with a documented gate).",
                    row_ref=row_ref,
                )
            )
        else:
            for gate in machine_gates:
                if not isinstance(gate, dict):
                    findings.append(
                        make_finding(
                            "error",
                            "principle_checks.machine_gate_shape",
                            PRINCIPLE_CHECKS_REL,
                            PRINCIPLE_CHECKS_REL,
                            f"principle_checks row '{principle_id}' contains a non-object machine_gates entry",
                            "Keep every enforcement.machine_gates[] entry a YAML mapping/object.",
                            row_ref=row_ref,
                        )
                    )
                    continue
                tool_ref = gate.get("tool_ref")
                if isinstance(tool_ref, str) and not existing_path_ref(repo, tool_ref):
                    findings.append(
                        make_finding(
                            "error",
                            "principle_checks.machine_gate_tool_ref_exists",
                            PRINCIPLE_CHECKS_REL,
                            tool_ref,
                            f"principle_checks row '{principle_id}' references missing gate tool '{tool_ref}'",
                            "Keep tool_ref limited to real repo paths (or remove the ref).",
                            row_ref=row_ref,
                        )
                    )
                for ref in gate.get("primary_config_refs", []) or []:
                    if isinstance(ref, str) and not existing_path_ref(repo, ref):
                        findings.append(
                            make_finding(
                                "error",
                                "principle_checks.machine_gate_config_ref_exists",
                                PRINCIPLE_CHECKS_REL,
                                ref,
                                f"principle_checks row '{principle_id}' references missing gate config '{ref}'",
                                "Keep primary_config_refs limited to real repo paths (or remove the ref).",
                                row_ref=row_ref,
                            )
                        )

    return findings


def validate_principle_violation_examples(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    if not repo.exists(PRINCIPLE_VIOLATION_EXAMPLES_REL):
        findings.append(
            make_finding(
                "error",
                "principle_violation_examples.file_exists",
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                f"missing architecture principle violation examples: {PRINCIPLE_VIOLATION_EXAMPLES_REL}",
                "Add artifacts/architecture/principle_violation_examples.yaml and register it in control_artifact_index.yaml.",
            )
        )
        return findings

    payload = repo.yaml(PRINCIPLE_VIOLATION_EXAMPLES_REL)
    if payload.get("artifact_id") != "aureline.architecture_principle_violation_examples":
        findings.append(
            make_finding(
                "error",
                "principle_violation_examples.artifact_id",
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                "principle_violation_examples artifact_id must be 'aureline.architecture_principle_violation_examples'",
                "Fix artifacts/architecture/principle_violation_examples.yaml artifact_id to match the canonical id.",
            )
        )

    examples = payload.get("examples")
    if not isinstance(examples, list) or not examples:
        findings.append(
            make_finding(
                "error",
                "principle_violation_examples.examples_list",
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                "principle_violation_examples must define a non-empty examples list",
                "Populate artifacts/architecture/principle_violation_examples.yaml with worked cases.",
            )
        )
        return findings

    catalog = repo.yaml(FITNESS_CATALOG_REL)
    allowed_principles = set(catalog.get("architecture_principles", []))
    classes = payload.get("example_class_vocabulary", [])
    allowed_classes = set(classes) if isinstance(classes, list) else set()
    if not allowed_classes:
        allowed_classes = {"temporary_narrowing", "release_blocking_contradiction"}

    example_ids = [row.get("example_id") for row in examples if isinstance(row, dict)]
    for duplicate in register_duplicates([eid for eid in example_ids if isinstance(eid, str)]):
        findings.append(
            make_finding(
                "error",
                "principle_violation_examples.unique_example_ids",
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                PRINCIPLE_VIOLATION_EXAMPLES_REL,
                f"principle_violation_examples duplicates example_id '{duplicate}'",
                "Keep exactly one example per example_id.",
                row_ref=duplicate,
            )
        )

    by_principle: dict[str, set[str]] = defaultdict(set)
    for row in examples:
        if not isinstance(row, dict):
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.row_shape",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    "principle_violation_examples contains a non-object row",
                    "Keep every examples[] entry a YAML mapping/object.",
                )
            )
            continue

        example_id = row.get("example_id")
        principle_id = row.get("principle_id")
        classification = row.get("classification")
        row_ref = None if not isinstance(example_id, str) else f"example_id:{example_id}"

        for field_name in ("example_id", "principle_id", "classification", "scenario", "required_controls"):
            if field_name not in row:
                findings.append(
                    make_finding(
                        "error",
                        f"principle_violation_examples.required_field.{field_name}",
                        PRINCIPLE_VIOLATION_EXAMPLES_REL,
                        PRINCIPLE_VIOLATION_EXAMPLES_REL,
                        f"principle_violation_examples row '{example_id}' missing required field '{field_name}'",
                        "Populate the missing required field so the example is reviewable and machine-checkable.",
                        row_ref=row_ref,
                    )
                )

        if classification == "temporary_narrowing" and not isinstance(row.get("why_it_is_acceptable"), str):
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.why_required_for_narrowing",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    f"principle_violation_examples row '{example_id}' must include why_it_is_acceptable for temporary_narrowing",
                    "Add why_it_is_acceptable so reviewers can see why the narrowing remains truthful and bounded.",
                    row_ref=row_ref,
                )
            )
        if classification == "release_blocking_contradiction" and not isinstance(row.get("why_it_is_blocking"), str):
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.why_required_for_blocking",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    f"principle_violation_examples row '{example_id}' must include why_it_is_blocking for release_blocking_contradiction",
                    "Add why_it_is_blocking so reviewers can see why it contradicts a protected principle or claim-bearing posture.",
                    row_ref=row_ref,
                )
            )

        if not isinstance(principle_id, str) or principle_id not in allowed_principles:
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.principle_id_vocabulary",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    FITNESS_CATALOG_REL,
                    f"principle_violation_examples row '{example_id}' uses unknown principle_id '{principle_id}'",
                    "Use a principle_id from artifacts/bench/fitness_function_catalog.yaml#architecture_principles.",
                    row_ref=row_ref,
                )
            )
        else:
            if isinstance(classification, str):
                by_principle[principle_id].add(classification)

        if not isinstance(classification, str) or classification not in allowed_classes:
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.classification_vocabulary",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    f"principle_violation_examples row '{example_id}' uses unknown classification '{classification}'",
                    "Use a classification from example_class_vocabulary.",
                    row_ref=row_ref,
                )
            )

        required_controls = row.get("required_controls")
        if not isinstance(required_controls, list) or not required_controls:
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.required_controls_non_empty",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    f"principle_violation_examples row '{example_id}' must carry a non-empty required_controls list",
                    "List at least one concrete machine gate or review forum used to adjudicate the scenario.",
                    row_ref=row_ref,
                )
            )

    # Ensure the most commonly debated principles have both narrowing and blocking examples.
    required_pairs = {
        "local_first_shell_remote_capable_services",
        "one_command_graph",
        "one_execution_context_model",
        "caches_disposable_user_state_durable",
        "optional_services_additive",
        "accessibility_and_trust_are_system_qualities",
    }
    for principle in sorted(required_pairs):
        seen = by_principle.get(principle, set())
        missing = sorted({"temporary_narrowing", "release_blocking_contradiction"} - seen)
        if missing:
            findings.append(
                make_finding(
                    "error",
                    "principle_violation_examples.required_principle_pairing",
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    PRINCIPLE_VIOLATION_EXAMPLES_REL,
                    f"principle_violation_examples is missing {', '.join(missing)} example(s) for principle '{principle}'",
                    "Add at least one worked example for each classification so reviewers can distinguish narrowing from contradiction.",
                    row_ref=f"principle_id:{principle}",
                )
            )

    return findings


def validate_contract_validation_lane(repo: RepoView) -> list[Finding]:
    findings: list[Finding] = []
    lane_ref = item_ref("contract_artifact_validation_lane", CONTROL_ARTIFACT_INDEX_REL)
    required_paths = [
        (
            CONTRACT_VALIDATION_WRAPPER_REL,
            "contract_validation_lane.wrapper_exists",
            f"contract-validation wrapper is missing: {CONTRACT_VALIDATION_WRAPPER_REL}",
            "Keep ci/contract_validation.sh present so local runs and CI invoke the same validator entry point.",
        ),
        (
            CONTRACT_VALIDATION_WORKFLOW_REL,
            "contract_validation_lane.workflow_exists",
            f"contract-validation workflow is missing: {CONTRACT_VALIDATION_WORKFLOW_REL}",
            "Keep the CI workflow present so the contract-artifact validation lane is enforced on pull requests.",
        ),
        (
            CONTRACT_VALIDATION_DOC_REL,
            "contract_validation_lane.doc_exists",
            f"contract-validation doc is missing: {CONTRACT_VALIDATION_DOC_REL}",
            "Keep the local-run documentation present so contributors can execute the same gate before opening a pull request.",
        ),
        (
            CONTRACT_VALIDATION_SCENARIO_REL,
            "contract_validation_lane.scenario_fixture_exists",
            f"contract-validation scenario fixture is missing: {CONTRACT_VALIDATION_SCENARIO_REL}",
            "Keep a checked-in failing scenario fixture so the deployment-profile and decision-reference checks stay demonstrable.",
        ),
        (
            FROZEN_SURFACE_TOOL_REL,
            "contract_validation_lane.frozen_surface_tool_exists",
            f"frozen-surface validator is missing: {FROZEN_SURFACE_TOOL_REL}",
            "Keep tools/check_frozen_surfaces.py present so contributors can run the frozen-surface gate directly.",
        ),
        (
            FROZEN_SURFACE_SCENARIO_REL,
            "contract_validation_lane.frozen_surface_scenario_exists",
            f"frozen-surface scenario fixture is missing: {FROZEN_SURFACE_SCENARIO_REL}",
            "Keep a checked-in failing frozen-surface scenario so missing diff metadata stays demonstrable.",
        ),
        (
            PROTECTED_DEPENDENCY_TOOL_REL,
            "contract_validation_lane.protected_dependency_tool_exists",
            f"protected-dependency validator is missing: {PROTECTED_DEPENDENCY_TOOL_REL}",
            "Keep tools/check_protected_dependencies.py present so contributors can run the protected-path boundary gate directly.",
        ),
        (
            PROTECTED_DEPENDENCY_SCENARIO_REL,
            "contract_validation_lane.protected_dependency_scenario_exists",
            f"protected-dependency scenario fixture is missing: {PROTECTED_DEPENDENCY_SCENARIO_REL}",
            "Keep a checked-in failing protected-dependency scenario so package-direction and hot-path sentinel checks stay demonstrable.",
        ),
    ]
    for path, check_id, message, remediation in required_paths:
        if not repo.exists(path):
            findings.append(
                make_finding(
                    "error",
                    check_id,
                    path,
                    lane_ref,
                    message,
                    remediation,
                )
            )

    if repo.exists(CONTRACT_VALIDATION_WRAPPER_REL):
        wrapper_text = repo.text(CONTRACT_VALIDATION_WRAPPER_REL)
        if "tools/ci/validate_contract_artifacts.py" not in wrapper_text:
            findings.append(
                make_finding(
                    "error",
                    "contract_validation_lane.wrapper_invokes_validator",
                    CONTRACT_VALIDATION_WRAPPER_REL,
                    lane_ref,
                    "contract-validation wrapper no longer invokes tools/ci/validate_contract_artifacts.py",
                    "Keep ci/contract_validation.sh wired to the shared validator script instead of a divergent local command.",
                )
            )

    if repo.exists(CONTRACT_VALIDATION_WORKFLOW_REL):
        workflow_text = repo.text(CONTRACT_VALIDATION_WORKFLOW_REL)
        if CONTRACT_VALIDATION_WRAPPER_REL not in workflow_text:
            findings.append(
                make_finding(
                    "error",
                    "contract_validation_lane.workflow_invokes_wrapper",
                    CONTRACT_VALIDATION_WORKFLOW_REL,
                    lane_ref,
                    "contract-validation workflow does not invoke ci/contract_validation.sh",
                    "Run the shared wrapper from CI so local and CI contract-artifact validation stay aligned.",
                )
            )

    if repo.exists(CONTRACT_VALIDATION_DOC_REL):
        doc_text = repo.text(CONTRACT_VALIDATION_DOC_REL)
        for needle, check_id, message, remediation in (
            (
                CONTRACT_VALIDATION_WRAPPER_REL,
                "contract_validation_lane.doc_mentions_wrapper",
                "contract-validation doc no longer mentions ci/contract_validation.sh",
                "Document the shared wrapper command so local developers run the same gate as CI.",
            ),
            (
                "tools/ci/validate_contract_artifacts.py",
                "contract_validation_lane.doc_mentions_validator",
                "contract-validation doc no longer mentions tools/ci/validate_contract_artifacts.py",
                "Keep the direct validator invocation documented for debugging and report customization.",
            ),
            (
                CONTRACT_VALIDATION_SCENARIO_REL,
                "contract_validation_lane.doc_mentions_scenario",
                "contract-validation doc no longer mentions the failing scenario fixture",
                "Document the checked-in failing example so reviewers can prove the deployment-profile gate still trips.",
            ),
            (
                FROZEN_SURFACE_TOOL_REL,
                "contract_validation_lane.doc_mentions_frozen_surface_tool",
                "contract-validation doc no longer mentions tools/check_frozen_surfaces.py",
                "Document the direct frozen-surface validator so reviewers can debug missing diff metadata locally.",
            ),
            (
                FROZEN_SURFACE_SCENARIO_REL,
                "contract_validation_lane.doc_mentions_frozen_surface_scenario",
                "contract-validation doc no longer mentions the frozen-surface failing scenario fixture",
                "Document the checked-in frozen-surface failing example so reviewers can prove the same-train gate still trips.",
            ),
            (
                PROTECTED_DEPENDENCY_TOOL_REL,
                "contract_validation_lane.doc_mentions_protected_dependency_tool",
                "contract-validation doc no longer mentions tools/check_protected_dependencies.py",
                "Document the direct protected-dependency validator so reviewers can debug package-direction and hot-path sentinel failures locally.",
            ),
            (
                PROTECTED_DEPENDENCY_SCENARIO_REL,
                "contract_validation_lane.doc_mentions_protected_dependency_scenario",
                "contract-validation doc no longer mentions the protected-dependency failing scenario fixture",
                "Document the checked-in protected-dependency failing example so reviewers can prove the service-boundary gate still trips.",
            ),
        ):
            if needle not in doc_text:
                findings.append(
                    make_finding(
                        "error",
                        check_id,
                        CONTRACT_VALIDATION_DOC_REL,
                        lane_ref,
                        message,
                        remediation,
                    )
                )

    if repo.exists(CONTRACT_VALIDATION_SCENARIO_REL):
        try:
            scenario_repo = RepoView(repo.root, repo.rel(CONTRACT_VALIDATION_SCENARIO_REL))
            boundary_profiles, _ = validate_boundary_manifest(
                scenario_repo,
                scenario_repo.yaml(QUALIFICATION_MATRIX_REL).get("deployment_profile_vocabulary", []),
            )
            scenario_findings = validate_compatibility_matrix(scenario_repo, boundary_profiles)
        except SystemExit as exc:
            findings.append(
                make_finding(
                    "error",
                    "contract_validation_lane.scenario_fixture_loads",
                    CONTRACT_VALIDATION_SCENARIO_REL,
                    lane_ref,
                    f"contract-validation scenario fixture could not be applied: {exc}",
                    "Keep the checked-in scenario JSON structurally valid and pointed at a real mutable field.",
                )
            )
        else:
            if not any(finding.check_id == "compatibility_matrix.deployment_profiles_resolve" for finding in scenario_findings):
                findings.append(
                    make_finding(
                        "error",
                        "contract_validation_lane.scenario_fixture_fails",
                        CONTRACT_VALIDATION_SCENARIO_REL,
                        lane_ref,
                        "contract-validation scenario fixture no longer triggers the missing deployment-profile check",
                        "Keep one checked-in scenario that deterministically fails on a missing deployment-profile reference.",
                    )
                )

    if repo.exists(FROZEN_SURFACE_SCENARIO_REL):
        try:
            frozen_findings, _ = validate_frozen_surface_manifest(repo.root, repo.rel(FROZEN_SURFACE_SCENARIO_REL))
        except SystemExit as exc:
            findings.append(
                make_finding(
                    "error",
                    "contract_validation_lane.frozen_surface_scenario_loads",
                    FROZEN_SURFACE_SCENARIO_REL,
                    lane_ref,
                    f"frozen-surface scenario fixture could not be applied: {exc}",
                    "Keep the checked-in frozen-surface scenario JSON structurally valid and pointed at a real monitored path.",
                )
            )
        else:
            expected_ids = {
                "frozen_surface_manifest.diff_metadata_required",
                "frozen_surface_manifest.same_train_follow_up_required",
            }
            if not any(finding.check_id in expected_ids for finding in frozen_findings):
                findings.append(
                    make_finding(
                        "error",
                        "contract_validation_lane.frozen_surface_scenario_fails",
                        FROZEN_SURFACE_SCENARIO_REL,
                        lane_ref,
                        "frozen-surface scenario fixture no longer triggers the missing diff-metadata or same-train follow-up checks",
                        "Keep one checked-in scenario that deterministically fails when a frozen surface changes without its required metadata and companion updates.",
                    )
                )

    if repo.exists(PROTECTED_DEPENDENCY_SCENARIO_REL):
        try:
            protected_findings, _ = validate_protected_dependency_rules(
                repo.root,
                repo.rel(PROTECTED_DEPENDENCY_SCENARIO_REL),
            )
        except SystemExit as exc:
            findings.append(
                make_finding(
                    "error",
                    "contract_validation_lane.protected_dependency_scenario_loads",
                    PROTECTED_DEPENDENCY_SCENARIO_REL,
                    lane_ref,
                    f"protected-dependency scenario fixture could not be applied: {exc}",
                    "Keep the checked-in protected-dependency scenario JSON structurally valid and pointed at real monitored packages and module paths.",
                )
            )
        else:
            expected_ids = {
                "protected_dependency_rules.package_forbidden_dependency_class",
                "protected_dependency_rules.module_forbidden_sentinel",
            }
            if not expected_ids.issubset({finding.check_id for finding in protected_findings}):
                findings.append(
                    make_finding(
                        "error",
                        "contract_validation_lane.protected_dependency_scenario_fails",
                        PROTECTED_DEPENDENCY_SCENARIO_REL,
                        lane_ref,
                        "protected-dependency scenario fixture no longer triggers both the package-direction and hot-path sentinel checks",
                        "Keep one checked-in scenario that deterministically fails on both a forbidden dependency direction and a blocking-I/O sentinel on the protected path.",
                    )
                )

    return findings


def group_findings(findings: list[Finding]) -> dict[str, dict[str, int]]:
    counts: dict[str, dict[str, int]] = defaultdict(lambda: {"error": 0, "warning": 0})
    for finding in findings:
        counts[finding.check_id][finding.severity] += 1
    return counts


def render_human_summary(findings: list[Finding], check_count: int, scenario: dict[str, Any] | None) -> str:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    warning_count = sum(1 for finding in findings if finding.severity == "warning")
    status = "FAIL" if error_count else "PASS"
    lines = [
        f"[contract-validation] {status} ({check_count} checks, {error_count} errors, {warning_count} warnings)",
    ]
    if scenario is not None:
        scenario_path = scenario.get("_path", "<unknown>")
        lines.append(f"[contract-validation] scenario: {scenario_path}")

    if not findings:
        lines.append("[contract-validation] no findings")
        return "\n".join(lines) + "\n"

    for finding in sorted(findings, key=lambda item: (item.severity != "error", item.check_id, item.owner_artifact_ref)):
        lines.append(
            f"[{finding.severity.upper()}] {finding.check_id} :: {finding.message}"
        )
        lines.append(f"  artifact: {finding.artifact_ref}")
        lines.append(f"  owner:    {finding.owner_artifact_ref}")
        if finding.row_ref:
            lines.append(f"  row:      {finding.row_ref}")
        lines.append(f"  fix:      {finding.remediation}")
    return "\n".join(lines) + "\n"


def build_report(
    repo: RepoView,
    findings: list[Finding],
    check_ids: list[str],
    command_parity_analysis: dict[str, Any] | None,
    frozen_surface_analysis: dict[str, Any] | None,
    protected_dependency_analysis: dict[str, Any] | None,
) -> dict[str, Any]:
    grouped = group_findings(findings)
    return {
        "report_kind": "contract_artifact_validation_report",
        "schema_version": 1,
        "generated_at": now_utc(),
        "repo_root": str(repo.root),
        "scenario": None
        if repo.scenario is None
        else {
            "path": repo.scenario.get("_path"),
            "scenario_id": repo.scenario.get("scenario_id"),
            "description": repo.scenario.get("description"),
        },
        "summary": {
            "check_count": len(check_ids),
            "error_count": sum(1 for finding in findings if finding.severity == "error"),
            "warning_count": sum(1 for finding in findings if finding.severity == "warning"),
        },
        "checks": [
            {
                "check_id": check_id,
                "error_count": grouped[check_id]["error"],
                "warning_count": grouped[check_id]["warning"],
                "status": "fail" if grouped[check_id]["error"] else ("warn" if grouped[check_id]["warning"] else "pass"),
            }
            for check_id in check_ids
        ],
        "findings": [finding.as_report() for finding in findings],
        "command_parity_summary": None if command_parity_analysis is None else command_parity_analysis.get("summary"),
        "frozen_surface_summary": None
        if frozen_surface_analysis is None
        else {
            "changed_file_count": frozen_surface_analysis["summary"]["changed_file_count"],
            "changed_surface_count": frozen_surface_analysis["summary"]["changed_surface_count"],
            "error_count": frozen_surface_analysis["summary"]["error_count"],
            "warning_count": frozen_surface_analysis["summary"]["warning_count"],
        },
        "protected_dependency_summary": None
        if protected_dependency_analysis is None
        else protected_dependency_analysis.get("summary"),
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    scenario_path = None if args.scenario is None else Path(args.scenario)
    repo = RepoView(repo_root, scenario_path)

    boundary_profiles, boundary_findings = validate_boundary_manifest(
        repo,
        repo.yaml(QUALIFICATION_MATRIX_REL).get("deployment_profile_vocabulary", []),
    )
    checks: list[tuple[str, list[Finding]]] = [
        ("package_inventory", validate_package_inventory(repo)),
        ("control_artifact_index", validate_control_artifact_index(repo)),
        ("source_of_truth_map", validate_source_of_truth_map(repo)),
        ("decision_index", validate_decision_index(repo)),
        ("interface_freeze_matrix", validate_interface_freeze_matrix(repo)),
        ("stable_surface_inventory", validate_stable_surface_inventory(repo)),
        ("compatibility_matrix", validate_compatibility_matrix(repo, boundary_profiles)),
        ("boundary_manifest", boundary_findings),
        ("claim_manifest", validate_claim_manifest(repo, boundary_profiles)),
    ]
    frozen_surface_findings, frozen_surface_analysis = validate_frozen_surface_manifest(repo.root, repo.scenario)
    checks.append(
        (
            "frozen_surface_manifest",
            [Finding(**finding.as_report()) for finding in frozen_surface_findings],
        )
    )
    protected_dependency_findings, protected_dependency_analysis = validate_protected_dependency_rules(
        repo.root,
        repo.scenario,
    )
    checks.append(
        (
            "protected_dependencies",
            [Finding(**finding.as_report()) for finding in protected_dependency_findings],
        )
    )
    command_findings, command_parity_analysis = validate_command_parity(repo)
    checks.append(("command_parity", command_findings))
    checks.append(("principle_checks", validate_principle_checks(repo)))
    checks.append(("principle_violation_examples", validate_principle_violation_examples(repo)))
    checks.append(("contract_validation_lane", validate_contract_validation_lane(repo)))

    all_findings = [finding for _, findings in checks for finding in findings]
    human = render_human_summary(all_findings, len(checks), repo.scenario)
    sys.stdout.write(human)

    if args.report:
        check_ids = [check_id for check_id, _ in checks]
        report_path = repo.rel(args.report)
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(
                build_report(
                    repo,
                    all_findings,
                    check_ids,
                    command_parity_analysis,
                    frozen_surface_analysis,
                    protected_dependency_analysis,
                ),
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )

    return 1 if any(finding.severity == "error" for finding in all_findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
