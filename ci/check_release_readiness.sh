#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ci/check_release_readiness.sh [options]

Fail closed unless every stable-release control is structurally valid, fresh at
the evaluation date, and carrying a proceed verdict.

Options:
  --repo-root PATH          Repository root (default: .)
  --out-dir PATH            Validation captures (default: target/release-readiness)
  --evaluation-date DATE    UTC evaluation date in YYYY-MM-DD form (default: today)
  -h, --help                Show this help
USAGE
}

repo_root="."
out_dir="target/release-readiness"
evaluation_date="$(date -u +%F)"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-root)
      repo_root="${2:?--repo-root requires a path}"
      shift 2
      ;;
    --out-dir)
      out_dir="${2:?--out-dir requires a path}"
      shift 2
      ;;
    --evaluation-date)
      evaluation_date="${2:?--evaluation-date requires YYYY-MM-DD}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 64
      ;;
  esac
done

repo_root="$(cd "$repo_root" && pwd)"
if [[ "$out_dir" != /* ]]; then
  out_dir="$repo_root/$out_dir"
fi
mkdir -p "$out_dir"

aggregate_status=0

run_gate() {
  local gate_name="$1"
  local checker="$2"
  local report_name="$3"
  local gate_status=0

  if python3 "$repo_root/$checker" \
    --repo-root "$repo_root" \
    --evaluation-date "$evaluation_date" \
    --require-proceed \
    --report "$out_dir/$report_name"; then
    echo "release readiness gate PASS: $gate_name"
  else
    gate_status=$?
    aggregate_status=1
    echo "release readiness gate FAIL ($gate_status): $gate_name" >&2
  fi
}

run_gate \
  "stable claim manifest" \
  "ci/check_stable_claim_manifest.py" \
  "stable_claim_manifest_validation.json"
run_gate \
  "stable qualification matrix" \
  "ci/check_stable_qualification_matrix.py" \
  "stable_qualification_matrix_validation.json"
run_gate \
  "shiproom dashboard" \
  "ci/check_shiproom_dashboard.py" \
  "shiproom_dashboard_validation.json"

if [[ "$aggregate_status" -ne 0 ]]; then
  echo "release readiness FAIL at evaluation date $evaluation_date" >&2
  exit "$aggregate_status"
fi

echo "release readiness PASS at evaluation date $evaluation_date"
