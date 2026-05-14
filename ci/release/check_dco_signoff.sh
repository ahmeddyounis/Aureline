#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

python3 - "$@" <<'PY'
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path


SIGNOFF_RE = re.compile(
    r"^Signed-off-by:\s+[^<>\n]+ <[^<>\s]+@[^<>\s]+>$",
    re.MULTILINE,
)
BASELINE_RE = re.compile(r"dco-baseline-cutoff:\s*([0-9a-fA-F]{7,40})")
EXCEPTION_RE = re.compile(r"dco-exception:\s*([0-9a-fA-F]{7,40})")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Check Developer Certificate of Origin sign-off trailers."
    )
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--range", dest="commit_range", default=None)
    parser.add_argument("--audit", default="artifacts/governance/dco_merge_audit_alpha.md")
    parser.add_argument("--allow-empty", action="store_true")
    return parser.parse_args()


def git(repo_root: Path, *args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", "-C", str(repo_root), *args],
        text=True,
        capture_output=True,
        check=check,
    )


def detect_range(repo_root: Path, explicit: str | None) -> str:
    if explicit:
        return explicit
    env_range = subprocess.run(
        ["sh", "-c", "printf '%s' \"${AURELINE_DCO_COMMIT_RANGE:-}\""],
        text=True,
        capture_output=True,
        check=True,
    ).stdout.strip()
    if env_range:
        return env_range
    upstream = git(
        repo_root,
        "rev-parse",
        "--abbrev-ref",
        "--symbolic-full-name",
        "@{upstream}",
        check=False,
    )
    if upstream.returncode == 0 and upstream.stdout.strip():
        return f"{upstream.stdout.strip()}..HEAD"
    origin_main = git(repo_root, "rev-parse", "--verify", "origin/main", check=False)
    if origin_main.returncode == 0:
        return "origin/main..HEAD"
    return "HEAD"


def read_audit_exceptions(repo_root: Path, audit_ref: str) -> tuple[str | None, set[str]]:
    audit_path = repo_root / audit_ref
    if not audit_path.exists():
        return None, set()
    text = audit_path.read_text(encoding="utf-8")
    baseline_match = BASELINE_RE.search(text)
    baseline = baseline_match.group(1) if baseline_match else None
    explicit = set(EXCEPTION_RE.findall(text))
    return baseline, explicit


def rev_list(repo_root: Path, commit_range: str) -> list[str]:
    result = git(repo_root, "rev-list", "--reverse", commit_range, check=False)
    if result.returncode != 0:
        raise SystemExit(result.stderr.strip() or f"invalid commit range: {commit_range}")
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def commit_message(repo_root: Path, commit: str) -> str:
    return git(repo_root, "log", "-1", "--format=%B", commit).stdout


def is_ancestor(repo_root: Path, maybe_ancestor: str, descendant: str) -> bool:
    result = git(repo_root, "merge-base", "--is-ancestor", maybe_ancestor, descendant, check=False)
    return result.returncode == 0


def normalize_commit(repo_root: Path, ref: str) -> str | None:
    result = git(repo_root, "rev-parse", "--verify", ref, check=False)
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a git repository: {repo_root}")

    commit_range = detect_range(repo_root, args.commit_range)
    commits = rev_list(repo_root, commit_range)
    if not commits:
        message = f"DCO sign-off check: no commits in range {commit_range}"
        if args.allow_empty:
            print(message)
            return 0
        print(message)
        return 0

    baseline_ref, explicit_exception_refs = read_audit_exceptions(repo_root, args.audit)
    baseline = normalize_commit(repo_root, baseline_ref) if baseline_ref else None
    explicit_exceptions = {
        normalized
        for ref in explicit_exception_refs
        if (normalized := normalize_commit(repo_root, ref)) is not None
    }

    failures: list[str] = []
    exceptions: list[str] = []
    for commit in commits:
        message = commit_message(repo_root, commit)
        if SIGNOFF_RE.search(message):
            continue
        if commit in explicit_exceptions:
            exceptions.append(commit)
            continue
        if baseline and is_ancestor(repo_root, commit, baseline):
            exceptions.append(commit)
            continue
        subject = git(repo_root, "log", "-1", "--format=%h %s", commit).stdout.strip()
        failures.append(subject)

    if failures:
        print("DCO sign-off check FAILED", file=sys.stderr)
        print(f"range: {commit_range}", file=sys.stderr)
        for failure in failures:
            print(f"missing Signed-off-by: {failure}", file=sys.stderr)
        print("add a Developer Certificate of Origin 1.1 trailer with git commit -s", file=sys.stderr)
        return 1

    print(
        "DCO sign-off check PASS "
        f"({len(commits)} commits checked, {len(exceptions)} documented exceptions)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY
