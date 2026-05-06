#!/usr/bin/env python3
"""Check semantic-token conformance without external deps.

This gate enforces the repository's semantic-token discipline in first-party
code by:

- scanning first-party source trees for raw-color literals (hex / rgb[a] /
  hsl[a] / 0x... forms); and
- validating the time-bounded exception registry and synthetic violation
  fixtures that prove detection works.

The policy is defined in docs/design/token_conformance_gate.md.
"""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

REPO_ROOT = Path(__file__).resolve().parents[2]

EXCEPTION_REGISTRY_PATH = REPO_ROOT / "artifacts/design/raw_color_exception_registry.yaml"
FIXTURE_DIR = REPO_ROOT / "fixtures/design/raw_color_violation_cases"

DEFAULT_FIRST_PARTY_ROOTS = (
    "crates",
    "extensions",
)

SKIP_TOP_LEVEL_DIRS = {
    ".git",
    ".idea",
    ".logs",
    ".t2",
    ".plans",
    "artifacts",
    "docs",
    "fixtures",
    "schemas",
    "target",
}

SCAN_SUFFIXES = {
    ".rs",
    ".css",
    ".scss",
    ".sass",
    ".less",
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".json",
    ".toml",
    ".yaml",
    ".yml",
    ".html",
    ".svg",
    ".md",
}

RAW_COLOR_RE = re.compile(
    r"(#(?:[0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})\b)"
    r"|(\b(?:rgb|rgba|hsl|hsla)\([^)\n]*\))"
    r"|(\b0x[0-9a-fA-F]{6,8}\b)"
)

TOKEN_REF_RE = re.compile(r"\b(?:(?:al\.color|status|trust)\.[A-Za-z0-9_.-]+)\b")

DOMAIN_FORBIDDEN_TOKEN_PREFIXES: dict[str, tuple[str, ...]] = {
    "syntax": (
        "al.color.diff.",
        "al.color.chart.",
        "al.color.state.",
        "al.color.accent.",
        "status.",
        "trust.",
    ),
    "diff": (
        "al.color.syntax.",
        "al.color.chart.",
        "al.color.state.",
        "al.color.accent.interactive",
        "status.",
        "trust.",
    ),
    "chart": (
        "al.color.syntax.",
        "al.color.diff.",
        "al.color.state.",
        "al.color.accent.interactive",
        "status.",
        "trust.",
    ),
    "diagnostic": (
        "al.color.syntax.",
        "al.color.diff.",
        "al.color.chart.",
    ),
    "state": (
        "al.color.syntax.",
        "al.color.diff.",
        "al.color.chart.",
    ),
}

DOMAIN_TAG_RE = re.compile(r"\baureline-token-domain\s*:\s*(?P<domain>[A-Za-z_]+)\b")


class GateError(RuntimeError):
    """Raised when conformance fails."""


@dataclass(frozen=True)
class ExceptionScope:
    path_globs: tuple[str, ...]
    locate_hint: str


@dataclass(frozen=True)
class ExceptionEntry:
    exception_id: str
    exception_class: str
    status: str
    owner_team: str
    owner_contact: str
    rationale: str
    scope: ExceptionScope
    raw_color_literals: tuple[str, ...]
    non_semantic_token_refs: tuple[str, ...]
    expires_at: dt.datetime
    follow_up_ref: str
    evidence_refs: tuple[str, ...]


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
        raise GateError(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise GateError(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def parse_timestamp(value: str, *, where: str) -> dt.datetime:
    if not isinstance(value, str) or not value:
        raise GateError(f"{where}: expected non-empty ISO 8601 timestamp string")
    raw = value
    if raw.endswith("Z"):
        raw = raw[:-1] + "+00:00"
    try:
        parsed = dt.datetime.fromisoformat(raw)
    except ValueError as exc:
        raise GateError(f"{where}: invalid ISO 8601 timestamp {value!r}: {exc}") from exc
    if parsed.tzinfo is None:
        raise GateError(f"{where}: timestamp must be timezone-aware (use 'Z' or an offset): {value!r}")
    return parsed


def normalize_rel_path(path: Path) -> str:
    return path.as_posix()


def matches_any_glob(rel_path: str, patterns: Iterable[str]) -> bool:
    for pattern in patterns:
        if pattern.startswith("/") or pattern.startswith(".."):
            raise GateError(f"exception scope glob must be repo-relative (no leading '/' or '..'): {pattern!r}")
        if fnmatch.fnmatchcase(rel_path, pattern):
            return True
    return False


def load_exception_registry(path: Path) -> list[ExceptionEntry]:
    payload = render_yaml_as_json(path)
    if not isinstance(payload, dict):
        raise GateError(f"{path}: expected mapping at root")
    if payload.get("record_kind") != "raw_color_exception_registry_record":
        raise GateError(f"{path}: record_kind must be raw_color_exception_registry_record")
    if payload.get("raw_color_exception_schema_version") != 1:
        raise GateError(f"{path}: raw_color_exception_schema_version must be 1")

    minted_at = payload.get("minted_at")
    if minted_at:
        parse_timestamp(minted_at, where=f"{path}:minted_at")

    entries: list[ExceptionEntry] = []
    seen_ids: set[str] = set()
    now = dt.datetime.now(dt.timezone.utc)

    exceptions = payload.get("exceptions", [])
    if exceptions is None:
        exceptions = []
    if not isinstance(exceptions, list):
        raise GateError(f"{path}: exceptions must be a list")

    for index, raw in enumerate(exceptions):
        where = f"{path}:exceptions[{index}]"
        if not isinstance(raw, dict):
            raise GateError(f"{where}: expected mapping")

        exception_id = raw.get("exception_id")
        if not isinstance(exception_id, str) or not exception_id:
            raise GateError(f"{where}: exception_id is required")
        if exception_id in seen_ids:
            raise GateError(f"{where}: duplicate exception_id {exception_id!r}")
        seen_ids.add(exception_id)

        exception_class = raw.get("exception_class")
        if exception_class not in ("raw_color_literal", "non_semantic_token_reference"):
            raise GateError(f"{where}: exception_class must be raw_color_literal or non_semantic_token_reference")

        status = raw.get("status")
        if status not in ("active", "expired"):
            raise GateError(f"{where}: status must be active or expired")

        owner = raw.get("owner", {})
        if not isinstance(owner, dict):
            raise GateError(f"{where}: owner must be a mapping")
        owner_team = owner.get("team")
        owner_contact = owner.get("contact")
        if not isinstance(owner_team, str) or not owner_team:
            raise GateError(f"{where}: owner.team is required")
        if not isinstance(owner_contact, str) or not owner_contact:
            raise GateError(f"{where}: owner.contact is required")

        rationale = raw.get("rationale")
        if not isinstance(rationale, str) or not rationale:
            raise GateError(f"{where}: rationale is required")

        scope = raw.get("scope", {})
        if not isinstance(scope, dict):
            raise GateError(f"{where}: scope must be a mapping")
        path_globs = scope.get("path_globs", [])
        locate_hint = scope.get("locate_hint")
        if not isinstance(path_globs, list) or not path_globs or not all(
            isinstance(item, str) and item for item in path_globs
        ):
            raise GateError(f"{where}: scope.path_globs must be a non-empty list of strings")
        if not isinstance(locate_hint, str) or not locate_hint:
            raise GateError(f"{where}: scope.locate_hint is required")

        raw_color_literals = tuple(raw.get("raw_color_literals") or [])
        non_semantic_token_refs = tuple(raw.get("non_semantic_token_refs") or [])
        if exception_class == "raw_color_literal" and not raw_color_literals:
            raise GateError(f"{where}: raw_color_literals must be non-empty for raw_color_literal exceptions")
        if exception_class == "non_semantic_token_reference" and not non_semantic_token_refs:
            raise GateError(
                f"{where}: non_semantic_token_refs must be non-empty for non_semantic_token_reference exceptions"
            )

        expires_at_value = raw.get("expires_at")
        expires_at = parse_timestamp(expires_at_value, where=f"{where}:expires_at")
        if expires_at <= now:
            raise GateError(f"{where}: exception has expired (expires_at={expires_at_value!r})")

        follow_up_ref = raw.get("follow_up_ref")
        if not isinstance(follow_up_ref, str) or not follow_up_ref:
            raise GateError(f"{where}: follow_up_ref is required")

        evidence_refs = raw.get("evidence_refs", [])
        if not isinstance(evidence_refs, list) or not evidence_refs or not all(
            isinstance(item, str) and item for item in evidence_refs
        ):
            raise GateError(f"{where}: evidence_refs must be a non-empty list of strings")

        entries.append(
            ExceptionEntry(
                exception_id=exception_id,
                exception_class=exception_class,
                status=status,
                owner_team=owner_team,
                owner_contact=owner_contact,
                rationale=rationale,
                scope=ExceptionScope(path_globs=tuple(path_globs), locate_hint=locate_hint),
                raw_color_literals=tuple(raw_color_literals),
                non_semantic_token_refs=tuple(non_semantic_token_refs),
                expires_at=expires_at,
                follow_up_ref=follow_up_ref,
                evidence_refs=tuple(evidence_refs),
            )
        )

    return entries


def detect_raw_color_literals(text: str) -> list[str]:
    return [match.group(0) for match in RAW_COLOR_RE.finditer(text)]


def detect_token_refs(text: str) -> list[str]:
    return [match.group(0) for match in TOKEN_REF_RE.finditer(text)]


def detect_domain_violations(domain: str | None, token_refs: Iterable[str]) -> list[dict[str, str]]:
    if not domain:
        return []
    domain = domain.lower()
    forbidden = DOMAIN_FORBIDDEN_TOKEN_PREFIXES.get(domain)
    if not forbidden:
        return []
    violations: list[dict[str, str]] = []
    for token_ref in token_refs:
        for prefix in forbidden:
            if token_ref == prefix or token_ref.startswith(prefix):
                violations.append({"token_ref": token_ref, "reason": "forbidden_token_for_domain"})
                break
    return violations


def find_declared_domain(text: str) -> str | None:
    match = DOMAIN_TAG_RE.search(text)
    if not match:
        return None
    return match.group("domain")


def is_skipped_by_top_level_dir(rel_path: Path) -> bool:
    if not rel_path.parts:
        return False
    return rel_path.parts[0] in SKIP_TOP_LEVEL_DIRS


def iter_first_party_files(repo_root: Path, roots: Iterable[str]) -> Iterable[Path]:
    for root in roots:
        base = repo_root / root
        if not base.exists():
            continue
        for path in base.rglob("*"):
            if not path.is_file():
                continue
            rel = path.relative_to(repo_root)
            if is_skipped_by_top_level_dir(rel):
                continue
            if path.suffix.lower() not in SCAN_SUFFIXES:
                continue
            yield path


def exception_allows_literal(
    *,
    rel_path: str,
    literal: str,
    exception_entries: Iterable[ExceptionEntry],
) -> ExceptionEntry | None:
    for entry in exception_entries:
        if entry.exception_class != "raw_color_literal":
            continue
        if not matches_any_glob(rel_path, entry.scope.path_globs):
            continue
        if literal not in entry.raw_color_literals:
            continue
        return entry
    return None


def exception_allows_token_ref(
    *,
    rel_path: str,
    token_ref: str,
    exception_entries: Iterable[ExceptionEntry],
) -> ExceptionEntry | None:
    for entry in exception_entries:
        if entry.exception_class != "non_semantic_token_reference":
            continue
        if not matches_any_glob(rel_path, entry.scope.path_globs):
            continue
        if token_ref not in entry.non_semantic_token_refs:
            continue
        return entry
    return None


def check_first_party_sources(
    *,
    repo_root: Path,
    roots: Iterable[str],
    exception_entries: list[ExceptionEntry],
) -> list[str]:
    failures: list[str] = []
    for path in iter_first_party_files(repo_root, roots):
        rel_path = normalize_rel_path(path.relative_to(repo_root))
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue

        declared_domain = find_declared_domain(text)
        if declared_domain:
            token_refs = detect_token_refs(text)
            domain_violations = detect_domain_violations(declared_domain, token_refs)
            for violation in domain_violations:
                token_ref = violation["token_ref"]
                allowed_by = exception_allows_token_ref(
                    rel_path=rel_path,
                    token_ref=token_ref,
                    exception_entries=exception_entries,
                )
                if allowed_by is None:
                    failures.append(
                        f"{rel_path}: token {token_ref!r} violates declared domain {declared_domain!r} (no active exception)"
                    )

        raw_literals = detect_raw_color_literals(text)
        if not raw_literals:
            continue

        for literal in raw_literals:
            allowed_by = exception_allows_literal(
                rel_path=rel_path,
                literal=literal,
                exception_entries=exception_entries,
            )
            if allowed_by is None:
                failures.append(f"{rel_path}: raw-color literal {literal!r} is forbidden (no active exception)")
    return failures


def validate_fixture_cases(fixtures_dir: Path) -> list[str]:
    if not fixtures_dir.exists():
        return [f"[token-conformance] missing fixture directory at {fixtures_dir}"]

    failures: list[str] = []
    paths = sorted(path for path in fixtures_dir.glob("*.yaml") if path.name != "README.md")
    for fixture_path in paths:
        payload = render_yaml_as_json(fixture_path)
        if not isinstance(payload, dict):
            failures.append(f"{fixture_path}: expected mapping at root")
            continue

        source_text = payload.get("source_text", "")
        surface_domain = payload.get("surface_domain")
        expected = payload.get("expected", {})
        if not isinstance(source_text, str) or not source_text.strip():
            failures.append(f"{fixture_path}: source_text is required")
            continue
        if surface_domain is not None and not isinstance(surface_domain, str):
            failures.append(f"{fixture_path}: surface_domain must be null or string")
            continue
        if not isinstance(expected, dict):
            failures.append(f"{fixture_path}: expected must be a mapping")
            continue

        expected_raw_literals = expected.get("raw_color_literals", [])
        expected_domain_violations = expected.get("domain_violations", [])
        if not isinstance(expected_raw_literals, list) or not all(isinstance(item, str) for item in expected_raw_literals):
            failures.append(f"{fixture_path}: expected.raw_color_literals must be a list of strings")
            continue
        if not isinstance(expected_domain_violations, list):
            failures.append(f"{fixture_path}: expected.domain_violations must be a list")
            continue

        raw_literals = detect_raw_color_literals(source_text)
        if raw_literals != expected_raw_literals:
            failures.append(
                f"{fixture_path}: raw_color_literals mismatch: expected={expected_raw_literals!r} got={raw_literals!r}"
            )

        token_refs = detect_token_refs(source_text)
        domain_violations = detect_domain_violations(surface_domain, token_refs)
        if domain_violations != expected_domain_violations:
            failures.append(
                f"{fixture_path}: domain_violations mismatch: expected={expected_domain_violations!r} got={domain_violations!r}"
            )

    if failures:
        return failures
    return [f"[token-conformance] OK ({len(paths)} fixture cases validated)"]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    parser.add_argument(
        "--roots",
        default=",".join(DEFAULT_FIRST_PARTY_ROOTS),
        help="Comma-separated repo-relative roots to scan for first-party code.",
    )
    parser.add_argument("--exceptions", default=str(EXCEPTION_REGISTRY_PATH))
    parser.add_argument("--fixtures", default=str(FIXTURE_DIR))
    parser.add_argument("--skip-scan", action="store_true", help="Skip scanning the repository and only validate artifacts.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    try:
        exception_entries = load_exception_registry(Path(args.exceptions))
    except GateError as exc:
        print(f"[token-conformance] exception registry invalid: {exc}", file=sys.stderr)
        return 2

    fixture_results = validate_fixture_cases(Path(args.fixtures))
    if fixture_results and not fixture_results[0].startswith("[token-conformance] OK"):
        print("[token-conformance] fixture validation failed:", file=sys.stderr)
        for line in fixture_results:
            print(f"  - {line}", file=sys.stderr)
        return 2
    print(fixture_results[0] if fixture_results else "[token-conformance] OK (no fixtures)")

    if args.skip_scan:
        return 0

    roots = [part.strip() for part in args.roots.split(",") if part.strip()]
    scan_failures = check_first_party_sources(repo_root=repo_root, roots=roots, exception_entries=exception_entries)
    if scan_failures:
        print("[token-conformance] raw-color conformance failed:", file=sys.stderr)
        for failure in scan_failures:
            print(f"  - {failure}", file=sys.stderr)
        print(
            "\n[token-conformance] remediation: replace raw colors with semantic token refs, or add a time-bounded\n"
            "exception to artifacts/design/raw_color_exception_registry.yaml with owner/rationale/scope/expiry/evidence.\n",
            file=sys.stderr,
        )
        return 1

    print("[token-conformance] OK (no raw-color literals found in scanned first-party roots)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
