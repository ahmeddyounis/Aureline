#!/usr/bin/env python3
"""Synchronize workspace package governance from Cargo manifests.

The Cargo workspace is authoritative for package names, paths, descriptions,
and production/build dependency edges. This tool combines that graph with the
reviewed package-class map below and renders the package inventory, ownership
rows, protected dependency rows, and repository topology documents.

Run without ``--write`` in CI to detect drift. Use ``--write`` only when the
workspace or a reviewed package classification changes.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
INVENTORY_PATH = REPO_ROOT / "artifacts/governance/package_inventory.yaml"
OWNERSHIP_PATH = REPO_ROOT / "artifacts/governance/ownership_matrix.yaml"
RULES_PATH = REPO_ROOT / "artifacts/architecture/protected_path_dependency_rules.yaml"
TOPOLOGY_PATH = REPO_ROOT / "docs/repo/topology.md"
DEPENDENCY_DOC_PATH = REPO_ROOT / "docs/repo/dependency_rules.md"

DEPENDENCY_CLASSES = [
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
    "off_cone",
]

OFF_CONE = {
    "aureline-bench",
    "aureline-env",
    "aureline-largefile-proto",
    "aureline-service",
    "aureline-shell-spike",
}

PROMOTED_PACKAGES = {
    "aureline-generated",
    "aureline-graph-proto",
    "aureline-reactive-state",
}

PACKAGE_CLASSES = {
    "shell_ui": {
        "aureline-activity",
        "aureline-capabilities",
        "aureline-cli",
        "aureline-commands",
        "aureline-config",
        "aureline-design-system",
        "aureline-editor",
        "aureline-input",
        "aureline-learning",
        "aureline-preview",
        "aureline-settings",
        "aureline-shell",
        "aureline-ui",
        "aureline-workspace",
    },
    "renderer": {"aureline-render"},
    "text_buffer": {
        "aureline-buffer",
        "aureline-content-safety",
        "aureline-history",
        "aureline-text",
    },
    "vfs_watchers": {"aureline-vfs"},
    "index_search": {
        "aureline-collections",
        "aureline-docs",
        "aureline-graph",
        "aureline-graph-proto",
        "aureline-graph-ui",
        "aureline-language",
        "aureline-navigation",
        "aureline-reactive-state",
        "aureline-search",
    },
    "task_execution": {
        "aureline-debug",
        "aureline-execution",
        "aureline-git",
        "aureline-notebook",
        "aureline-review",
        "aureline-runtime",
        "aureline-terminal",
    },
    "remote_helper": {
        "aureline-api",
        "aureline-collab",
        "aureline-companion",
        "aureline-remote",
        "aureline-rpc",
    },
    "ai_control_plane": {
        "aureline-ai",
        "aureline-auth",
        "aureline-data",
        "aureline-infra",
        "aureline-policy",
        "aureline-provider",
    },
    "updater_release": {
        "aureline-ecosystem",
        "aureline-extensions",
        "aureline-install",
        "aureline-release",
        "aureline-scaffold",
        "aureline-templates",
    },
    "support_diagnostics": {
        "aureline-build-farm",
        "aureline-build-info",
        "aureline-change-objects",
        "aureline-chronology",
        "aureline-continuity",
        "aureline-crash",
        "aureline-deps",
        "aureline-doctor",
        "aureline-framework",
        "aureline-generated",
        "aureline-governance",
        "aureline-i18n",
        "aureline-incident",
        "aureline-notices",
        "aureline-profiler",
        "aureline-qe",
        "aureline-records",
        "aureline-recovery",
        "aureline-runbooks",
        "aureline-service-health",
        "aureline-service-health-feed",
        "aureline-support",
        "aureline-telemetry",
    },
    "off_cone": OFF_CONE,
}

WORK_PACKAGE_TITLES = {
    "WP-01": "Shell, interaction, and desktop composition",
    "WP-02": "Editor, renderer, and text core",
    "WP-03": "Workspace, VFS, settings, and persistence",
    "WP-04": "Search, indexing, navigation, and graph",
    "WP-05": "Data tools and notebooks",
    "WP-06": "API and framework surfaces",
    "WP-07": "Debugging and quality engineering",
    "WP-08": "Task, terminal, Git, and execution runtime",
    "WP-09": "Extensions, ecosystem, templates, and scaffolding",
    "WP-10": "Identity, providers, collaboration, and companion",
    "WP-11": "Policy, governance, and managed boundaries",
    "WP-12": "AI routing and review",
    "WP-13": "Release, update, install, and build engineering",
    "WP-14": "Support, diagnostics, certification, and evidence",
    "WP-15": "Documentation, localization, and learning",
    "WP-16": "CLI and command surfaces",
    "WP-17": "Remote execution and infrastructure",
}

SPECIAL_WORK_PACKAGES = {
    "aureline-ai": ["WP-12"],
    "aureline-api": ["WP-06"],
    "aureline-cli": ["WP-16"],
    "aureline-commands": ["WP-16"],
    "aureline-data": ["WP-05"],
    "aureline-debug": ["WP-07"],
    "aureline-docs": ["WP-15"],
    "aureline-ecosystem": ["WP-09"],
    "aureline-extensions": ["WP-09"],
    "aureline-framework": ["WP-06"],
    "aureline-i18n": ["WP-15"],
    "aureline-learning": ["WP-15"],
    "aureline-notebook": ["WP-05"],
    "aureline-qe": ["WP-07"],
    "aureline-scaffold": ["WP-09"],
    "aureline-templates": ["WP-09"],
}

CLASS_WORK_PACKAGE = {
    "shell_ui": "WP-01",
    "renderer": "WP-02",
    "text_buffer": "WP-02",
    "vfs_watchers": "WP-03",
    "index_search": "WP-04",
    "task_execution": "WP-08",
    "remote_helper": "WP-17",
    "ai_control_plane": "WP-10",
    "updater_release": "WP-13",
    "support_diagnostics": "WP-14",
    "off_cone": "WP-14",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="rewrite synchronized artifacts")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def load_yaml(path: Path) -> dict[str, Any]:
    script = "require 'json'; require 'yaml'; puts JSON.generate(YAML.load_file(ARGV[0]))"
    result = subprocess.run(
        ["ruby", "-e", script, str(path)],
        check=True,
        capture_output=True,
        text=True,
    )
    payload = json.loads(result.stdout)
    if not isinstance(payload, dict):
        raise SystemExit(f"{path.relative_to(REPO_ROOT)} must contain a YAML object")
    return payload


def package_class_map() -> dict[str, str]:
    result: dict[str, str] = {}
    for class_name, packages in PACKAGE_CLASSES.items():
        for package in packages:
            if package in result:
                raise SystemExit(f"package classified more than once: {package}")
            result[package] = class_name
    return result


def workspace() -> dict[str, dict[str, Any]]:
    root = load_toml(REPO_ROOT / "Cargo.toml")
    members = root.get("workspace", {}).get("members", [])
    packages: dict[str, dict[str, Any]] = {}
    for member in members:
        manifest = load_toml(REPO_ROOT / member / "Cargo.toml")
        package = manifest["package"]
        name = package["name"]
        deps: set[str] = set()
        for section in ("dependencies", "build-dependencies"):
            for dep_name, spec in manifest.get(section, {}).items():
                if dep_name.startswith("aureline-") and isinstance(spec, dict) and "path" in spec:
                    deps.add(dep_name)
        packages[name] = {
            "name": name,
            "path": member,
            "description": package.get("description", "Internal Aureline workspace crate."),
            "deps": sorted(deps),
        }
    return packages


def compute_layers(packages: dict[str, dict[str, Any]], classes: dict[str, str]) -> dict[str, str]:
    memo: dict[str, int] = {}
    visiting: list[str] = []

    def depth(name: str) -> int:
        if name in memo:
            return memo[name]
        if name in visiting:
            cycle = " -> ".join(visiting[visiting.index(name) :] + [name])
            raise SystemExit(f"production dependency cycle: {cycle}")
        visiting.append(name)
        deps = packages[name]["deps"]
        value = 0 if not deps else 1 + max(depth(dep) for dep in deps)
        visiting.pop()
        memo[name] = value
        return value

    layers: dict[str, str] = {}
    for name in sorted(packages):
        layers[name] = "LX" if classes[name] == "off_cone" else f"L{depth(name)}"
    return layers


def yaml_list(values: list[str]) -> str:
    return "[" + ", ".join(values) + "]"


def render_inventory(
    packages: dict[str, dict[str, Any]],
    classes: dict[str, str],
    layers: dict[str, str],
    old_inventory: dict[str, Any],
) -> str:
    old_rows = {row["name"]: row for row in old_inventory.get("packages", [])}
    work_packages_by_name: dict[str, list[str]] = {}
    lines = [
        "# Package inventory.",
        "#",
        "# Generated by tools/governance/sync_workspace_governance.py from the",
        "# production/build Cargo dependency graph plus the reviewed class map.",
        "# Dev-dependencies are test topology and are intentionally excluded.",
        "",
        "schema_version: 1",
        "packages:",
    ]
    for name, package in sorted(packages.items()):
        old_row = old_rows.get(name, {})
        work_packages = old_row.get("work_packages") or SPECIAL_WORK_PACKAGES.get(name)
        if not work_packages:
            work_packages = [CLASS_WORK_PACKAGE[classes[name]]]
        work_packages_by_name[name] = list(work_packages)
        lines.extend(
            [
                f"  - name: {name}",
                f"    path: {package['path']}",
                f"    layer: {layers[name]}",
                f"    protected_path: {str(name not in OFF_CONE).lower()}",
                f"    work_packages: {yaml_list(list(work_packages))}",
            ]
        )
        deps = package["deps"]
        if deps:
            lines.append("    allowed_internal_deps:")
            lines.extend(f"      - {dep}" for dep in deps)
        else:
            lines.append("    allowed_internal_deps: []")
        lines.append(f"    depended_on_by_production: {str(name not in OFF_CONE).lower()}")
        note = (
            package["description"]
            if name in PROMOTED_PACKAGES
            else old_row.get("notes") or package["description"]
        )
        lines.append(f"    notes: {json.dumps(str(note).strip())}")
        lines.append("")

    lines.append("work_package_index:")
    for work_package, title in WORK_PACKAGE_TITLES.items():
        members = sorted(
            name for name, refs in work_packages_by_name.items() if work_package in refs
        )
        lines.extend(
            [
                f"  {work_package}:",
                f"    title: {json.dumps(title)}",
                f"    crates: {yaml_list(members)}",
            ]
        )
    return "\n".join(lines) + "\n"


def replace_section(text: str, start: str, end: str, body: str) -> str:
    prefix, remainder = text.split(start, 1)
    _, suffix = remainder.split(end, 1)
    return prefix + start + body + end + suffix


def render_ownership(
    packages: dict[str, dict[str, Any]], old_ownership: dict[str, Any], original: str
) -> str:
    old_rows = {row["name"]: row for row in old_ownership.get("packages", [])}
    lines: list[str] = []
    for name in sorted(packages):
        protected = name not in OFF_CONE
        lines.extend(
            [
                f"  - name: {name}",
                f"    protected: {str(protected).lower()}",
                '    primary_dri: "@ahmeddyounis"',
                "    backup_owner: null",
                "    backup_waiver: single-maintainer-backup" if protected else "    backup_waiver: null",
            ]
        )
        note = old_rows.get(name, {}).get("notes")
        if note:
            lines.append(f"    notes: {json.dumps(str(note).strip())}")
        lines.append("")
    body = "\n".join(lines)
    marker = "\n# -----------------------------------------------------------------------------\n# Governance lanes"
    return replace_section(original, "\npackages:\n", marker, body, )


def render_protected_rules(
    packages: dict[str, dict[str, Any]], classes: dict[str, str], original: str
) -> str:
    lines: list[str] = []
    for name, package in sorted(packages.items()):
        class_name = classes[name]
        if class_name == "off_cone":
            allowed = DEPENDENCY_CLASSES
            forbidden: list[str] = []
        else:
            allowed = sorted({classes[dep] for dep in package["deps"]})
            forbidden = [item for item in DEPENDENCY_CLASSES if item not in allowed]
        lines.extend(
            [
                f"  - package: {name}",
                f"    dependency_class: {class_name}",
                f"    protected_package: {str(name not in OFF_CONE).lower()}",
            ]
        )
        if allowed:
            lines.append("    allowed_dependency_classes:")
            lines.extend(f"      - {item}" for item in allowed)
        else:
            lines.append("    allowed_dependency_classes: []")
        if forbidden:
            lines.append("    forbidden_dependency_classes:")
            lines.extend(f"      - {item}" for item in forbidden)
        else:
            lines.append("    forbidden_dependency_classes: []")
        lines.append("")
    return replace_section(original, "\npackages:\n", "\nmodules:\n", "\n".join(lines))


def render_dependency_doc(
    packages: dict[str, dict[str, Any]], classes: dict[str, str], layers: dict[str, str]
) -> str:
    layer_rows: dict[str, list[str]] = {}
    for name, layer in layers.items():
        layer_rows.setdefault(layer, []).append(name)
    ordered_layers = sorted(
        layer_rows,
        key=lambda layer: (999 if layer == "LX" else int(layer[1:])),
    )
    lines = [
        "# Crate dependency rules",
        "",
        "This document is the human-readable projection of",
        "`artifacts/governance/package_inventory.yaml`. Production and build",
        "dependencies must point strictly downhill. Test-only `dev-dependencies`",
        "are excluded from the production graph but still compile in workspace tests.",
        "",
        "The service-plane class rules live in",
        "`artifacts/architecture/protected_path_dependency_rules.yaml`. Both files are",
        "synchronized by `tools/governance/sync_workspace_governance.py`.",
        "",
        "## Layering",
        "",
        "| Layer | Crates |",
        "|---|---|",
    ]
    for layer in ordered_layers:
        crates = ", ".join(f"`{name}`" for name in sorted(layer_rows[layer]))
        lines.append(f"| {layer} | {crates} |")
    lines.extend(["", "## Exact production/build edges", "", "| Crate | Class | May depend on |", "|---|---|---|"])
    for name, package in sorted(packages.items()):
        deps = ", ".join(f"`{dep}`" for dep in package["deps"]) or "—"
        lines.append(f"| `{name}` | `{classes[name]}` | {deps} |")
    lines.extend(
        [
            "",
            "## Rules",
            "",
            "- New crates and production/build edges must update the Cargo manifests,",
            "  package inventory, ownership matrix, protected dependency rules, and",
            "  topology docs in the same change.",
            "- Production-facing crates must not depend on `LX` crates. `LX` is reserved",
            "  for disposable spikes, benchmarks, prototypes, and isolated metadata models.",
            "- Cycles in the production/build graph are forbidden. Test-only integration",
            "  topology may use `dev-dependencies`; those edges do not authorize runtime",
            "  or library coupling.",
            "- A dependency-class change is an architecture change and requires the",
            "  applicable decision and protected-path review artifacts.",
            "- Run `python3 tools/governance/sync_workspace_governance.py` to check drift,",
            "  and use `--write` only after the new classification or edge is reviewed.",
            "",
        ]
    )
    return "\n".join(lines)


def render_topology(
    packages: dict[str, dict[str, Any]], classes: dict[str, str], layers: dict[str, str], original: str
) -> str:
    lines = [
        "## Workspace crates",
        "",
        "This table is generated from the Cargo workspace and the reviewed governance",
        "classification map. The package inventory remains the machine-readable source",
        "for protected posture, work-package ownership, and exact allowed edges.",
        "",
        "| Crate | Path | Layer | Dependency class | Protected | Role |",
        "|---|---|---|---|---|---|",
    ]
    for name, package in sorted(packages.items()):
        description = str(package["description"]).replace("|", "\\|")
        lines.append(
            f"| `{name}` | `{package['path']}/` | {layers[name]} | `{classes[name]}` | "
            f"{'yes' if name not in OFF_CONE else 'no'} | {description} |"
        )
    lines.extend(
        [
            "",
            "## Layering at a glance",
            "",
            "Active crates occupy topological layers `L0` through the current workspace",
            "maximum; every production/build edge points to a lower-numbered layer. `LX`",
            "is reserved for off-cone crates and may never become a production dependency",
            "without an explicit promotion that updates all governance artifacts together.",
            "",
            "Exact edges and layer membership are published in",
            "[`dependency_rules.md`](./dependency_rules.md). Service-plane placement and",
            "class-level direction are published in",
            "[`protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml).",
            "",
        ]
    )
    _, suffix = original.split("\n## Product boundary\n", 1)
    workspace_marker = (
        "\n## Workspace crates\n"
        if "\n## Workspace crates\n" in original
        else "\n## Seeded crates\n"
    )
    prefix, _ = original.split(workspace_marker, 1)
    return prefix + "\n" + "\n".join(lines) + "## Product boundary\n" + suffix


def main() -> int:
    args = parse_args()
    packages = workspace()
    classes = package_class_map()
    missing = sorted(set(packages) - set(classes))
    stale = sorted(set(classes) - set(packages))
    if missing or stale:
        raise SystemExit(f"package class map drift: missing={missing}, stale={stale}")
    layers = compute_layers(packages, classes)

    old_inventory = load_yaml(INVENTORY_PATH)
    old_ownership = load_yaml(OWNERSHIP_PATH)
    outputs = {
        INVENTORY_PATH: render_inventory(packages, classes, layers, old_inventory),
        OWNERSHIP_PATH: render_ownership(packages, old_ownership, OWNERSHIP_PATH.read_text()),
        RULES_PATH: render_protected_rules(packages, classes, RULES_PATH.read_text()),
        DEPENDENCY_DOC_PATH: render_dependency_doc(packages, classes, layers),
        TOPOLOGY_PATH: render_topology(packages, classes, layers, TOPOLOGY_PATH.read_text()),
    }

    drifted = []
    for path, expected in outputs.items():
        if path.read_text() == expected:
            continue
        drifted.append(path.relative_to(REPO_ROOT).as_posix())
        if args.write:
            path.write_text(expected)

    if drifted and not args.write:
        for path in drifted:
            print(f"workspace governance drift: {path}", file=sys.stderr)
        print("run with --write after reviewing the package classification", file=sys.stderr)
        return 1
    action = "updated" if drifted else "current"
    print(f"workspace governance {action}: {len(packages)} packages")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
