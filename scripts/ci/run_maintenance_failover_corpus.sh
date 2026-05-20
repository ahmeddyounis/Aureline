#!/usr/bin/env bash
#
# CI and local entry point for the maintenance-window & failover communication
# corpus audit lane.
#
# Runs ci/check_maintenance_failover_corpus.py — the deterministic audit
# validator. It schema-validates every continuity_notice_view_record packet under
# fixtures/ops/m3/maintenance_failover_corpus/ against
# schemas/ops/continuity_notice_view.schema.json, rebuilds each record from its
# extracted inputs with an independent port of the model and asserts the stored
# record matches, then proves: no stale notice reads as current, every changed
# tenant / region / endpoint boundary stays visible, queued intent never silently
# replays across a changed authority boundary, every scheduled window is timezone
# unambiguous, queued publish-later / local-draft work is preserved and kept
# separate from successful hosted mutations, support-bundle and CLI / headless
# exports stay in parity with the product UI, and every claimed managed / hybrid
# beta row maps to exactly one packet. The matrix and export-parity packet are
# regenerated in-memory and drift-checked.
#
# The corpus is fully deterministic and portable: it has no Cargo dependency, so
# the same gate runs locally, in mirrored profiles, and in managed CI.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

REPORT_JSON=""

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_maintenance_failover_corpus.sh [--report-json PATH]

Runs the maintenance-window & failover communication corpus audit lane gate.

Options:
  --report-json PATH  Write the validator's machine-readable findings to PATH.
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --report-json)
      if [[ $# -lt 2 ]]; then
        printf '[maintenance-failover] error: --report-json needs a path\n' >&2
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
      printf '[maintenance-failover] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[maintenance-failover] error: python3 is required\n' >&2
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
python3 ci/check_maintenance_failover_corpus.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

if (( CORPUS_EXIT != 0 )); then
  printf '[maintenance-failover] FAIL (corpus=%s)\n' "${CORPUS_EXIT}" >&2
  exit "${CORPUS_EXIT}"
fi

printf '[maintenance-failover] PASS\n'
