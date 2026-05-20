#!/usr/bin/env bash
#
# CI and local entry point for the M3 provider-account / target-mapping
# continuity drill corpus.
#
# Composes:
#   - ci/check_provider_mapping_corpus.py — the deterministic corpus
#     validator. It schema-validates every drill fixture under
#     fixtures/providers/m3/account_scope_and_mapping_corpus/, enforces the
#     three lane-failing invariants (a queued draft never silently vanishes, a
#     narrowed session never appears writable, a mapping never changes without a
#     visible review) and the identity-continuity invariants (provider/account/
#     mapping identity survives support export, activity-center reopen, and
#     restart/restore with no raw credentials), and joins the enum-only corpus
#     matrix and per-lane continuity packet against the fixtures plus the report
#     and reviewer drills doc.
#   - cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta
#     -- validate — re-validates the seeded beta page the drills extend. This
#     step runs only when a Cargo toolchain is available (skipped under
#     --no-cargo or when `cargo` is absent), keeping per-PR smoke coverage fast
#     and deterministic while nightly runs exercise the full path.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

RUN_CARGO=1
REPORT_JSON=""

usage() {
  cat <<'EOF'
Usage: scripts/ci/run_provider_mapping_corpus.sh [--no-cargo] [--report-json PATH]

Runs the provider-account / target-mapping continuity drill corpus gate.

Options:
  --no-cargo          Skip the Cargo re-validation of the seeded beta page
                      (the deterministic Python corpus gate still runs).
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
        printf '[provider-mapping-corpus] error: --report-json needs a path\n' >&2
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
      printf '[provider-mapping-corpus] error: unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[provider-mapping-corpus] error: python3 is required\n' >&2
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
python3 ci/check_provider_mapping_corpus.py "${PY_ARGS[@]}" || CORPUS_EXIT=$?

CARGO_EXIT=0
if (( RUN_CARGO == 1 )) && command -v cargo >/dev/null 2>&1; then
  cargo run --quiet -p aureline-provider \
    --bin aureline_provider_target_mapping_beta -- validate >/dev/null \
    || CARGO_EXIT=$?
else
  printf '[provider-mapping-corpus] skipping Cargo re-validation (--no-cargo or cargo unavailable)\n' >&2
fi

if (( CORPUS_EXIT != 0 )) || (( CARGO_EXIT != 0 )); then
  printf '[provider-mapping-corpus] FAIL (corpus=%s, cargo=%s)\n' \
    "${CORPUS_EXIT}" "${CARGO_EXIT}" >&2
  if (( CORPUS_EXIT != 0 )); then
    exit "${CORPUS_EXIT}"
  fi
  exit "${CARGO_EXIT}"
fi

printf '[provider-mapping-corpus] PASS\n'
