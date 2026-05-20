#!/usr/bin/env bash
#
# CI and local entry point for the maintenance & failover continuity-notice
# audit lane.
#
# Composes:
#   - ci/check_continuity_notices.py — the deterministic audit validator. It
#     schema-validates every continuity_notice_view fixture under
#     fixtures/ops/m3/maintenance_and_failover_notices/ against
#     schemas/ops/continuity_notice_view.schema.json, independently re-derives
#     the no-silent-current downgrade, the boundary-unresolved flag, the honesty
#     marker, and the summary counts, proves queued publish-later / local-draft
#     work is preserved and kept separate from successful hosted mutations, and
#     checks the corpus exercises every notice kind, category, effective
#     freshness, write posture, downgrade reason, and boundary-axis state. This
#     step always runs.
#   - cargo test -p aureline-shell --test continuity_notices_fixtures plus a
#     re-emit of the fixtures — re-verifies that the on-disk fixtures are a
#     literal projection of the shell model. This step runs only when a Cargo
#     toolchain is available (skipped under --no-cargo or when `cargo` is
#     absent).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

RUN_CARGO=1
REPORT_JSON=""
FIXTURE_DIR="fixtures/ops/m3/maintenance_and_failover_notices"

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_continuity_notices.sh [--no-cargo] [--report-json PATH]

Runs the maintenance & failover continuity-notice audit lane gate.

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
        printf '[continuity-notices] error: --report-json needs a path\n' >&2
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
      printf '[continuity-notices] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[continuity-notices] error: python3 is required\n' >&2
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
python3 ci/check_continuity_notices.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

CARGO_EXIT=0
if (( RUN_CARGO == 1 )) && command -v cargo >/dev/null 2>&1; then
  BEFORE_STATUS="$(git -C "${REPO_ROOT}" status --porcelain -- "${FIXTURE_DIR}")"
  cargo run --quiet -p aureline-shell \
    --bin aureline_shell_continuity_notices_corpus -- emit-fixtures \
    "${FIXTURE_DIR}" >/dev/null || CARGO_EXIT=$?
  AFTER_STATUS="$(git -C "${REPO_ROOT}" status --porcelain -- "${FIXTURE_DIR}")"
  if [[ "${BEFORE_STATUS}" != "${AFTER_STATUS}" ]]; then
    printf '[continuity-notices] fixtures drifted on re-emit\n' >&2
    CARGO_EXIT=1
  fi
  if (( CARGO_EXIT == 0 )); then
    cargo test --quiet -p aureline-shell \
      --test continuity_notices_fixtures >/dev/null || CARGO_EXIT=$?
  fi
else
  printf '[continuity-notices] skipping Cargo re-emit/replay (--no-cargo or cargo unavailable)\n' >&2
fi

if (( CORPUS_EXIT != 0 )) || (( CARGO_EXIT != 0 )); then
  printf '[continuity-notices] FAIL (corpus=%s, cargo=%s)\n' \
    "${CORPUS_EXIT}" "${CARGO_EXIT}" >&2
  if (( CORPUS_EXIT != 0 )); then
    exit "${CORPUS_EXIT}"
  fi
  exit "${CARGO_EXIT}"
fi

printf '[continuity-notices] PASS\n'
