#!/usr/bin/env bash
#
# CI and local entry point for dependency governance + health integrity checks.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${REPO_ROOT}/target/dependency-health"
CONFIG_PATH="${REPO_ROOT}/ci/check_dependency_health.yml"

usage() {
  cat <<'EOF'
Usage: ./ci/check_dependency_health.sh [--out-dir PATH] [--config PATH]
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
    --config)
      CONFIG_PATH="${2:-}"
      shift 2
      ;;
    --config=*)
      CONFIG_PATH="${1#--config=}"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf '[dependency-health] error: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[dependency-health] error: python3 is required\n' >&2
  exit 1
fi

if ! command -v ruby >/dev/null 2>&1; then
  printf '[dependency-health] error: ruby is required because the validator parses YAML via Psych\n' >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"

REPORT_PATH="${OUT_DIR}/dependency_health_report.json"
SUMMARY_PATH="${OUT_DIR}/dependency_health_summary.txt"

export TZ=UTC
export LC_ALL=C

python3 "${REPO_ROOT}/tools/governance/dependency_ingest/check_dependency_health.py" \
  --repo-root "${REPO_ROOT}" \
  --config "${CONFIG_PATH}" \
  --report "${REPORT_PATH}" | tee "${SUMMARY_PATH}"

