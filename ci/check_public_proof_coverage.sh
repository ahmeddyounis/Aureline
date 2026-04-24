#!/usr/bin/env bash
#
# CI and local entry point for the public-proof coverage audit.
#
# Runs the shared coverage validator, writes a machine-readable JSON
# report, and captures the same human-readable stdout summary that CI
# surfaces in logs.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${REPO_ROOT}/target/public-proof-coverage"

usage() {
  cat <<'EOF'
Usage: ./ci/check_public_proof_coverage.sh [--out-dir PATH]

Validates the join between requirement register, claim manifest,
assurance claim matrix, public-proof packet fixtures, workflow-bundle
register, known-limit classes, docs destinations, and exact-build
identity fixtures. Fails on orphan public truth.

Review guidance lives in
artifacts/governance/public_proof_coverage_report.md.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --out-dir=*)
      OUT_DIR="${1#--out-dir=}"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf '[public-proof-coverage] error: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[public-proof-coverage] error: python3 is required\n' >&2
  exit 1
fi

if ! command -v ruby >/dev/null 2>&1; then
  printf '[public-proof-coverage] error: ruby is required because the validator parses YAML via Psych\n' >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"

REPORT_PATH="${OUT_DIR}/public_proof_coverage_report.json"
SUMMARY_PATH="${OUT_DIR}/public_proof_coverage_summary.txt"

export TZ=UTC
export LC_ALL=C

python3 "${REPO_ROOT}/tools/ci/validate_public_proof_coverage.py" \
  --repo-root "${REPO_ROOT}" \
  --report "${REPORT_PATH}" | tee "${SUMMARY_PATH}"
