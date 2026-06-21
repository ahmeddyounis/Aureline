#!/usr/bin/env python3
"""Command-line entry point for the locale-pack contribution validator.

Examples::

    # Validate a pack you are authoring.
    python3 -m tools.i18n.validate_locale_pack path/to/my-locale-pack

    # Validate the governance glossary itself.
    python3 -m tools.i18n.validate_locale_pack --check-glossary

The exit code is ``0`` when the pack has no errors (warnings are allowed) and
``1`` when any error finding is raised, so the tool fits a pre-submission hook
or a CI gate.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

if __package__ in (None, ""):
    # Allow direct invocation by file path, not just `python3 -m ...`.
    sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
    from validate_locale_pack.validator import (
        DEFAULT_GLOSSARY_REL,
        DEFAULT_REGISTRY_REL,
        GlossaryError,
        RegistryError,
        load_message_registry,
        load_terminology_glossary,
        render_human_summary,
        validate_locale_pack,
        validate_terminology_glossary,
    )
else:
    from .validator import (
        DEFAULT_GLOSSARY_REL,
        DEFAULT_REGISTRY_REL,
        GlossaryError,
        RegistryError,
        load_message_registry,
        load_terminology_glossary,
        render_human_summary,
        validate_locale_pack,
        validate_terminology_glossary,
    )


def _find_repo_root(start: Path) -> Path:
    """Walks up from ``start`` to the directory holding the message registry."""
    for candidate in [start, *start.parents]:
        if (candidate / DEFAULT_REGISTRY_REL).exists():
            return candidate
    return start


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(prog="validate_locale_pack", description=__doc__)
    parser.add_argument("pack_dir", nargs="?", help="Path to the locale pack directory to validate.")
    parser.add_argument("--repo-root", default=None, help="Repo root (auto-detected by default).")
    parser.add_argument("--registry", default=None, help="Override the message-id registry path (repo-relative).")
    parser.add_argument("--glossary", default=None, help="Override the terminology glossary path (repo-relative).")
    parser.add_argument("--check-glossary", action="store_true", help="Validate the governance glossary itself and exit.")
    parser.add_argument("--report", default=None, help="Write a machine-readable JSON report to this path.")
    return parser.parse_args(argv)


def _resolve(repo_root: Path, override: str | None, default_rel: str) -> Path:
    if override:
        p = Path(override)
        return p if p.is_absolute() else repo_root / p
    return repo_root / default_rel


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)

    if args.repo_root:
        repo_root = Path(args.repo_root).resolve()
    else:
        anchor = Path(args.pack_dir).resolve() if args.pack_dir else Path.cwd()
        repo_root = _find_repo_root(anchor)

    glossary_path = _resolve(repo_root, args.glossary, DEFAULT_GLOSSARY_REL)
    registry_path = _resolve(repo_root, args.registry, DEFAULT_REGISTRY_REL)

    try:
        glossary = load_terminology_glossary(glossary_path)
    except GlossaryError as exc:
        sys.stderr.write(f"{exc}\n")
        return 2

    if args.check_glossary:
        findings = validate_terminology_glossary(glossary)
        sys.stdout.write(render_human_summary(findings, header=f"terminology glossary: {glossary_path}"))
        _maybe_write_report(args.report, repo_root, findings)
        return 1 if any(f.severity == "error" for f in findings) else 0

    if not args.pack_dir:
        sys.stderr.write("a pack directory is required unless --check-glossary is given\n")
        return 2

    try:
        registry = load_message_registry(registry_path)
    except RegistryError as exc:
        sys.stderr.write(f"{exc}\n")
        return 2

    pack_dir = Path(args.pack_dir).resolve()
    findings = validate_locale_pack(pack_dir, registry=registry, glossary=glossary)
    sys.stdout.write(render_human_summary(findings, header=f"locale pack: {pack_dir}"))
    _maybe_write_report(args.report, repo_root, findings)
    return 1 if any(f.severity == "error" for f in findings) else 0


def _maybe_write_report(report: str | None, repo_root: Path, findings) -> None:
    if not report:
        return
    payload = {
        "errors": sum(1 for f in findings if f.severity == "error"),
        "warnings": sum(1 for f in findings if f.severity == "warning"),
        "findings": [
            {"severity": f.severity, "code": f.code, "location": f.location, "message": f.message}
            for f in findings
        ],
    }
    path = Path(report)
    path = path if path.is_absolute() else repo_root / path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    raise SystemExit(main())
