#!/usr/bin/env bash
#
# CI and local entry point for contract-artifact validation.
#
# Runs the shared validator, writes a machine-readable JSON report, and
# captures the same human-readable stdout summary that CI surfaces in logs.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${REPO_ROOT}/target/contract-validation"
SCENARIO=""

usage() {
  cat <<'EOF'
Usage: ./ci/contract_validation.sh [--out-dir PATH] [--scenario PATH]
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
      printf '[contract-validation] error: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[contract-validation] error: python3 is required\n' >&2
  exit 1
fi

if ! command -v ruby >/dev/null 2>&1; then
  printf '[contract-validation] error: ruby is required because the validator parses YAML via Psych\n' >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"

REPORT_PATH="${OUT_DIR}/contract_validation_report.json"
SUMMARY_PATH="${OUT_DIR}/contract_validation_summary.txt"

ARGS=(
  --repo-root "${REPO_ROOT}"
  --report "${REPORT_PATH}"
)
if [[ -n "${SCENARIO}" ]]; then
  ARGS+=(--scenario "${SCENARIO}")
fi

export TZ=UTC
export LC_ALL=C

python3 "${REPO_ROOT}/tools/ci/validate_contract_artifacts.py" "${ARGS[@]}" | tee "${SUMMARY_PATH}"

printf '\n[contract-validation] validating geometry token ledger and cases\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/validate_geometry_cases.py" | tee -a "${SUMMARY_PATH}"

printf '\n[contract-validation] validating motion token ledger and cases\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/validate_motion_cases.py" | tee -a "${SUMMARY_PATH}"

printf '\n[contract-validation] validating component metrics ledger and cases\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/validate_component_metric_cases.py" | tee -a "${SUMMARY_PATH}"

printf '\n[contract-validation] validating palette mapping examples\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/validate_palette_examples.py" | tee -a "${SUMMARY_PATH}"

printf '\n[contract-validation] validating UI copy lint rules and cases\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/validate_ui_copy_cases.py" | tee -a "${SUMMARY_PATH}"

printf '\n[contract-validation] validating component conformance packets\n' | tee -a "${SUMMARY_PATH}"
python3 "${REPO_ROOT}/tools/ci/check_component_conformance.py" | tee -a "${SUMMARY_PATH}"
