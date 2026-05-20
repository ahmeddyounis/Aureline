#!/usr/bin/env bash
#
# CI and local entry point for the handoff & reproduction-packet corpus audit
# lane.
#
# Runs ci/check_handoff_repro_corpus.py — the deterministic audit validator. It
# schema-validates the handoff_target_review and repro_packet_preview records in
# every drill under fixtures/public/m3/handoff_repro_corpus/ against
# schemas/public/handoff_target_review.schema.json and
# schemas/public/repro_packet_preview.schema.json, re-runs an independent port of
# the HandoffReviewSheet model, and proves: every handoff names an exact target
# identity and visibility class, a route never coerces a disclosure onto a public
# target, the system browser only opens after the reproduction-packet preview is
# confirmed, the redaction posture is safe for the chosen visibility, the exact
# anchor / object identity and versioned build-context export survive every
# prepare/preview/block/retry/export/reopen stage, a field omitted from the
# review sheet is never exported, browser-blocked / offline / policy-denied
# handoffs preserve the prepared draft instead of losing it, and the
# support-boundary copy matches the actual target boundary. Reject drills prove
# each documented drift is caught with its expected typed reason. The matrix,
# export-parity packet, and the claimed-beta-row scorecard are regenerated
# in-memory and drift-checked.
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
Usage: scripts/ci/run_handoff_repro_corpus.sh [--report-json PATH]

Runs the handoff & reproduction-packet corpus audit lane gate.

Options:
  --report-json PATH  Write the validator's machine-readable findings to PATH.
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --report-json)
      if [[ $# -lt 2 ]]; then
        printf '[handoff-repro] error: --report-json needs a path\n' >&2
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
      printf '[handoff-repro] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[handoff-repro] error: python3 is required\n' >&2
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
python3 ci/check_handoff_repro_corpus.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

if (( CORPUS_EXIT != 0 )); then
  printf '[handoff-repro] FAIL (corpus=%s)\n' "${CORPUS_EXIT}" >&2
  exit "${CORPUS_EXIT}"
fi

printf '[handoff-repro] PASS\n'
