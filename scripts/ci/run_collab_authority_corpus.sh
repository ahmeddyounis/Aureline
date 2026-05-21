#!/usr/bin/env bash
#
# CI and local entry point for the collaboration moderation & authority-escrow
# corpus audit lane.
#
# Runs ci/check_moderation_authority_corpus.py — the deterministic audit
# validator. It schema-validates the invite_session_manifest and
# authority_escrow_ticket records in every drill under
# fixtures/collab/m3/moderation_and_authority_escrow_corpus/ against
# schemas/collab/invite_session_manifest.schema.json and
# schemas/collab/authority_escrow_ticket.schema.json, re-runs an independent port
# of the InviteAuthorityReviewSheet model, and proves: lobby moderation
# (admit/deny/defer/decline) agrees with the grant it leaves behind, observer-first
# joins only confer the right to request control, a revoked/expired/denied grant
# never displays as active or silently resumes after a reconnect or handoff, an
# expired or declined invite never still appears active, and the durable audit
# export preserves stable IDs, actor/owner, scope, reason code, and final
# resolution without leaking a session secret. Reject drills prove each documented
# drift is caught with its expected typed reason. The matrix, export-parity packet,
# and the claimed-beta/preview-row scorecard (with per-lane preview/beta status)
# are regenerated in-memory and drift-checked.
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
Usage: scripts/ci/run_collab_authority_corpus.sh [--report-json PATH]

Runs the collaboration moderation & authority-escrow corpus audit lane gate.

Options:
  --report-json PATH  Write the validator's machine-readable findings to PATH.
  -h, --help          Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --report-json)
      if [[ $# -lt 2 ]]; then
        printf '[moderation-authority] error: --report-json needs a path\n' >&2
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
      printf '[moderation-authority] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[moderation-authority] error: python3 is required\n' >&2
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
python3 ci/check_moderation_authority_corpus.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

if (( CORPUS_EXIT != 0 )); then
  printf '[moderation-authority] FAIL (corpus=%s)\n' "${CORPUS_EXIT}" >&2
  exit "${CORPUS_EXIT}"
fi

printf '[moderation-authority] PASS\n'
