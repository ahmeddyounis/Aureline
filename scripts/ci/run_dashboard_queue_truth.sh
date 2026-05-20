#!/usr/bin/env bash
#
# CI and local entry point for the operator dashboard & queue truth audit lane.
#
# Composes:
#   - ci/check_dashboard_queue_truth.py — the deterministic audit validator. It
#     schema-validates every dashboard-truth view fixture (the Rust-pinned
#     runtime fixtures under fixtures/ops/m3/dashboard_and_queue_truth/ plus the
#     audit drills under fixtures/ops/m3/dashboard_queue_truth_corpus/),
#     independently re-derives the no-silent-green model, enforces order /
#     narrowing explainability and canonical routing, proves the risk-vs-time-
#     vs-owner ordering and restart/reconnect broken-evidence drills, checks
#     support-export and CLI/headless export parity, and drift-checks the corpus
#     matrix and parity packet. This step always runs.
#   - cargo test -p aureline-shell --test dashboard_truth_fixtures plus a
#     re-emit of the runtime fixtures — re-verifies that the product corpus
#     still matches its source and that the on-disk runtime fixtures are a
#     literal projection of the shell model. This step runs only when a Cargo
#     toolchain is available (skipped under --no-cargo or when `cargo` is
#     absent), keeping per-PR smoke coverage fast while nightly runs exercise
#     the full path.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

RUN_CARGO=1
REPORT_JSON=""
RUNTIME_FIXTURE_DIR="fixtures/ops/m3/dashboard_and_queue_truth"

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_dashboard_queue_truth.sh [--no-cargo] [--report-json PATH]

Runs the operator dashboard & queue truth audit lane gate.

Options:
  --no-cargo          Skip the Cargo re-emit + replay test (the deterministic
                      Python audit gate still runs).
  --report-json PATH  Write the validator's machine-readable findings to PATH.
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-cargo)
      RUN_CARGO=0
      shift
      ;;
    --report-json)
      if [[ $# -lt 2 ]]; then
        printf '[dashboard-queue-truth] error: --report-json needs a path\n' >&2
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
      printf '[dashboard-queue-truth] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[dashboard-queue-truth] error: python3 is required\n' >&2
  exit 1
fi

export TZ=UTC
export LC_ALL=C
export PYTHONDONTWRITEBYTECODE=1

PY_ARGS=(--repo-root .)
if [[ -n "${REPORT_JSON}" ]]; then
  PY_ARGS+=(--report-json "${REPORT_JSON}")
fi

CORPUS_EXIT=0
python3 ci/check_dashboard_queue_truth.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

CARGO_EXIT=0
if (( RUN_CARGO == 1 )) && command -v cargo >/dev/null 2>&1; then
  # Re-emit the runtime fixtures and fail if they drift, then run the replay
  # test that pins them bit-for-bit to the shell model.
  BEFORE_STATUS="$(git -C "${REPO_ROOT}" status --porcelain -- "${RUNTIME_FIXTURE_DIR}")"
  cargo run --quiet -p aureline-shell \
    --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures \
    "${RUNTIME_FIXTURE_DIR}" >/dev/null || CARGO_EXIT=$?
  AFTER_STATUS="$(git -C "${REPO_ROOT}" status --porcelain -- "${RUNTIME_FIXTURE_DIR}")"
  if [[ "${BEFORE_STATUS}" != "${AFTER_STATUS}" ]]; then
    printf '[dashboard-queue-truth] runtime fixtures drifted on re-emit\n' >&2
    CARGO_EXIT=1
  fi
  if (( CARGO_EXIT == 0 )); then
    cargo test --quiet -p aureline-shell \
      --test dashboard_truth_fixtures >/dev/null || CARGO_EXIT=$?
  fi
else
  printf '[dashboard-queue-truth] skipping Cargo re-emit/replay (--no-cargo or cargo unavailable)\n' >&2
fi

if (( CORPUS_EXIT != 0 )) || (( CARGO_EXIT != 0 )); then
  printf '[dashboard-queue-truth] FAIL (corpus=%s, cargo=%s)\n' \
    "${CORPUS_EXIT}" "${CARGO_EXIT}" >&2
  if (( CORPUS_EXIT != 0 )); then
    exit "${CORPUS_EXIT}"
  fi
  exit "${CARGO_EXIT}"
fi

printf '[dashboard-queue-truth] PASS\n'
