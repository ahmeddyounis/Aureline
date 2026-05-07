#!/usr/bin/env python3
"""Probe upstream metadata for dependency rows and emit an attributed report.

This helper is intentionally non-mutating: it does NOT rewrite the canonical
registers. Its purpose is to provide a mechanically refreshed, attributable
input that humans can review before updating any curated governance fields.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import ssl
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import subprocess


DEFAULT_DEP_REGISTER = "artifacts/governance/dependency_register.yaml"


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


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


def http_get_json(url: str, user_agent: str) -> tuple[int, dict[str, Any] | None, str | None]:
    req = urllib.request.Request(url, headers={"User-Agent": user_agent, "Accept": "application/json"})
    try:
        ctx = None
        for cafile in ("/etc/ssl/cert.pem", "/etc/ssl/certs/ca-certificates.crt"):
            if Path(cafile).exists():
                ctx = ssl.create_default_context(cafile=cafile)
                break
        with urllib.request.urlopen(req, timeout=20, context=ctx) as resp:
            status = getattr(resp, "status", 200)
            raw = resp.read().decode("utf-8", errors="replace")
            try:
                return status, json.loads(raw), None
            except json.JSONDecodeError as exc:
                return status, None, f"invalid JSON: {exc}"
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace") if exc.fp else ""
        return exc.code, None, f"HTTPError: {exc.reason} {body[:200]}".strip()
    except Exception as exc:  # noqa: BLE001 - intentional broad capture for tooling
        return 0, None, f"{type(exc).__name__}: {exc}"


def derive_crate_name(row_id: str) -> str | None:
    # Convention in current seeds: dep.<area>.<crate_name>
    parts = row_id.split(".")
    if len(parts) >= 3 and parts[0] == "dep":
        return parts[-1]
    return None


def parse_github_repo(url: str) -> tuple[str, str] | None:
    # Accept https://github.com/{owner}/{repo}[.git][/...]
    parsed = urllib.parse.urlparse(url)
    if parsed.netloc.lower() != "github.com":
        return None
    path = parsed.path.strip("/")
    if not path:
        return None
    segments = path.split("/")
    if len(segments) < 2:
        return None
    owner = segments[0]
    repo = segments[1]
    if repo.endswith(".git"):
        repo = repo[: -len(".git")]
    if not owner or not repo:
        return None
    return owner, repo


@dataclass
class Evidence:
    kind: str
    ref: str
    status: int | None = None
    note: str | None = None

    def as_json(self) -> dict[str, Any]:
        payload: dict[str, Any] = {"kind": self.kind, "ref": self.ref}
        if self.status is not None:
            payload["status"] = self.status
        if self.note:
            payload["note"] = self.note
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--dependency-register", default=DEFAULT_DEP_REGISTER)
    parser.add_argument("--out", default="target/dependency-ingest/upstream_observations.json")
    parser.add_argument("--user-agent", default="aureline-dependency-ingest/0.1 (repo tooling)")
    parser.add_argument("--min-delay-seconds", type=float, default=1.0)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    dep_path = repo_root / args.dependency_register
    out_path = repo_root / args.out
    out_path.parent.mkdir(parents=True, exist_ok=True)

    dep = render_yaml_as_json(dep_path)
    if not isinstance(dep, dict):
        raise SystemExit("dependency register must parse to an object")
    rows = dep.get("rows", [])
    if not isinstance(rows, list):
        raise SystemExit("dependency register rows must be a list")

    observations: list[dict[str, Any]] = []
    failures = 0
    last_request_at = 0.0

    for row in rows:
        if not isinstance(row, dict):
            continue
        row_id = row.get("id")
        kind = row.get("dependency_kind")
        if not isinstance(row_id, str) or not isinstance(kind, str):
            continue
        if kind != "cargo_crate":
            continue

        crate = derive_crate_name(row_id)
        if not crate:
            continue

        # Basic per-request pacing.
        delay = args.min_delay_seconds - (time.time() - last_request_at)
        if delay > 0:
            time.sleep(delay)

        crate_url = f"https://crates.io/api/v1/crates/{urllib.parse.quote(crate)}"
        status, payload, error = http_get_json(crate_url, args.user_agent)
        last_request_at = time.time()
        evidence: list[Evidence] = [Evidence(kind="http_json", ref=crate_url, status=status, note=error)]

        record: dict[str, Any] = {
            "source_register": "dependency_register",
            "source_id": row_id,
            "dependency_kind": kind,
            "observed_at_utc": now_utc(),
            "evidence": [e.as_json() for e in evidence],
            "observations": {},
        }

        if payload is None:
            failures += 1
            record["observations"]["crates_io_status"] = "unavailable"
            observations.append(record)
            continue

        crate_obj = payload.get("crate", {})
        if not isinstance(crate_obj, dict):
            failures += 1
            record["observations"]["crates_io_status"] = "invalid_payload"
            observations.append(record)
            continue

        max_version = crate_obj.get("max_version")
        license_expr = crate_obj.get("license")
        updated_at = crate_obj.get("updated_at")
        repo_url = crate_obj.get("repository")

        if isinstance(max_version, str) and max_version:
            record["observations"]["latest_release_or_tag"] = max_version
        if isinstance(updated_at, str) and updated_at:
            record["observations"]["latest_release_published_at"] = updated_at
        if isinstance(license_expr, str) and license_expr:
            record["observations"]["license_expression"] = license_expr

        # Best-effort maintainer count estimate (crate owners).
        delay = args.min_delay_seconds - (time.time() - last_request_at)
        if delay > 0:
            time.sleep(delay)
        owners_url = f"https://crates.io/api/v1/crates/{urllib.parse.quote(crate)}/owners"
        o_status, owners_payload, o_err = http_get_json(owners_url, args.user_agent)
        last_request_at = time.time()
        record["evidence"].append(Evidence(kind="http_json", ref=owners_url, status=o_status, note=o_err).as_json())
        if isinstance(owners_payload, dict):
            users = owners_payload.get("users")
            if isinstance(users, list):
                record["observations"]["maintainer_count_estimate"] = len(users)

        if isinstance(repo_url, str) and repo_url:
            record["observations"]["repository_url"] = repo_url

            gh = parse_github_repo(repo_url)
            if gh:
                owner, repo = gh
                # Pace between calls.
                delay = args.min_delay_seconds - (time.time() - last_request_at)
                if delay > 0:
                    time.sleep(delay)
                repo_api = f"https://api.github.com/repos/{owner}/{repo}"
                r_status, repo_payload, r_err = http_get_json(repo_api, args.user_agent)
                last_request_at = time.time()
                record["evidence"].append(Evidence(kind="http_json", ref=repo_api, status=r_status, note=r_err).as_json())
                if isinstance(repo_payload, dict):
                    pushed_at = repo_payload.get("pushed_at")
                    if isinstance(pushed_at, str) and pushed_at:
                        record["observations"]["default_branch_last_commit_at"] = pushed_at

                delay = args.min_delay_seconds - (time.time() - last_request_at)
                if delay > 0:
                    time.sleep(delay)
                latest_release_api = f"https://api.github.com/repos/{owner}/{repo}/releases/latest"
                l_status, rel_payload, l_err = http_get_json(latest_release_api, args.user_agent)
                last_request_at = time.time()
                record["evidence"].append(
                    Evidence(kind="http_json", ref=latest_release_api, status=l_status, note=l_err).as_json()
                )
                if isinstance(rel_payload, dict):
                    tag = rel_payload.get("tag_name")
                    published = rel_payload.get("published_at")
                    if isinstance(tag, str) and tag:
                        record["observations"]["latest_release_or_tag"] = tag
                    if isinstance(published, str) and published:
                        record["observations"]["latest_release_published_at"] = published

        observations.append(record)

    report = {
        "schema_version": 1,
        "generated_at_utc": now_utc(),
        "inputs": {"dependency_register": args.dependency_register},
        "row_count": len(observations),
        "failure_count": failures,
        "rows": observations,
        "notes": [
            "This report is non-mutating. Use it to review upstream drift before editing curated governance rows.",
            "Fields are best-effort: missing observations represent unavailable upstream data, not a license/health clearance.",
        ],
    }
    out_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(f"[dependency-ingest] wrote {out_path.relative_to(repo_root)} with {len(observations)} row(s)")
    if failures:
        print(f"[dependency-ingest] warning: {failures} upstream probe(s) failed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
