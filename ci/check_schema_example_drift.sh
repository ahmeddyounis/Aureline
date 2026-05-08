#!/usr/bin/env bash
#
# CI and local entry point for schema/example drift validation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${REPO_ROOT}/target/schema-example-drift"
SCENARIO=""

usage() {
  cat <<'EOF'
Usage: ./ci/check_schema_example_drift.sh [--out-dir PATH] [--scenario PATH]
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
    --scenario)
      SCENARIO="${2:-}"
      shift 2
      ;;
    --scenario=*)
      SCENARIO="${1#--scenario=}"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf '[schema-example-drift] error: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[schema-example-drift] error: python3 is required\n' >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"

REPORT_PATH="${OUT_DIR}/schema_example_drift_report.json"
SUMMARY_PATH="${OUT_DIR}/schema_example_drift_summary.txt"

export TZ=UTC
export LC_ALL=C

ARGS=(--repo-root "${REPO_ROOT}" --report "${REPORT_PATH}")
if [[ -n "${SCENARIO}" ]]; then
  ARGS+=(--scenario "${SCENARIO}")
fi

python3 "${REPO_ROOT}/tools/check_schema_example_drift.py" "${ARGS[@]}" | tee "${SUMMARY_PATH}"

