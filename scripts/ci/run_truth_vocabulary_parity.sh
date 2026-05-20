#!/usr/bin/env bash
#
# CI and local entry point for the cross-surface truth-vocabulary parity gate.
#
# Composes:
#   - ci/check_truth_vocabulary_parity.py — the deterministic validator. It
#     loads the governed registry at
#     artifacts/governance/product_truth_vocabulary.yaml, verifies each
#     vocabulary axis mirrors its upstream schema / ledger verbatim, lints the
#     conforming surface corpus and the failure-drill conformance corpus under
#     fixtures/governance/truth_vocabulary_parity/, checks the public reference
#     doc covers every class, and drift-checks (or with --write regenerates) the
#     parity report at artifacts/release/m3/truth_vocabulary_parity_report.md.
#
# The gate fails on any blocker: registry drift, a forbidden parallel synonym
# or unknown term on a protected surface, a cross-surface conflict, an expired
# alias migration, a stale report, or a conformance drill that no longer fires.
# Warnings and in-window alias migrations are reported but non-fatal.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

WRITE=0
REPORT_JSON=""

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_truth_vocabulary_parity.sh [--write] [--report-json PATH]

Runs the cross-surface truth-vocabulary parity gate.

Options:
  --write             Regenerate the parity report instead of drift-checking it.
  --report-json PATH  Write the validator's machine-readable findings to PATH.
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --write)
      WRITE=1
      shift
      ;;
    --report-json)
      if [[ $# -lt 2 ]]; then
        printf '[truth-vocabulary] error: --report-json needs a path\n' >&2
        exit 2
      fi
      REPORT_JSON="$2"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf '[truth-vocabulary] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[truth-vocabulary] error: python3 is required\n' >&2
  exit 1
fi

if ! command -v ruby >/dev/null 2>&1; then
  printf '[truth-vocabulary] error: ruby is required (YAML decode via Psych)\n' >&2
  exit 1
fi

export TZ=UTC
export LC_ALL=C
export PYTHONDONTWRITEBYTECODE=1

PY_ARGS=(--repo-root .)
if (( WRITE == 1 )); then
  PY_ARGS+=(--write)
fi
if [[ -n "${REPORT_JSON}" ]]; then
  PY_ARGS+=(--report-json "${REPORT_JSON}")
fi

python3 ci/check_truth_vocabulary_parity.py "${PY_ARGS[@]}"
