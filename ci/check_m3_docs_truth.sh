#!/usr/bin/env bash
#
# CI and local entry point for the M3 docs / public-truth gate.
#
# Composes:
#   - tools/ci/m3/docs_freshness_gate.py — freshness, vocabulary,
#     consuming-surface, and release-notes back-link checks plus the
#     generated truth report regeneration.
#   - tools/ci/m3/stale_example_checker.py — protected fixture-backed
#     example validation against schemas and manifest vocabularies.
#   - tools/ci/m3/docs_public_proof_gate/ — parity blocker that joins
#     docs captures, stale examples, public-proof packets, and the
#     Help/About freshness badge vocabulary on marketed beta rows.
#   - ci/check_m3_public_benchmark_beta.py — public benchmark copy
#     gate that blocks unsupported benchmark wording while the packet
#     remains methodology-only.
#
# Validators refresh their durable JSON captures on every run; under
# --check mode generated reports also fail when they drift from
# upstream truth.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

CHECK_MODE=0
EXTRA_ARGS=()

usage() {
  cat <<'EOF'
Usage: ci/check_m3_docs_truth.sh [--check]

Runs the M3 docs / public-truth gate. The freshness gate regenerates
the docs truth report and validation captures; the stale-example
checker re-validates protected fixtures against the live claim
manifest vocabularies; the parity blocker verifies marketed rows are
not fresher than their public proof; the public benchmark gate blocks
unsupported benchmark wording.

Options:
  --check       Fail when the on-disk truth report would drift from
                upstream truth (use in CI alongside the regenerator).
  -h, --help    Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check)
      CHECK_MODE=1
      EXTRA_ARGS+=("--check")
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf '[m3-docs-truth] error: unknown argument: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

if ! command -v python3 >/dev/null 2>&1; then
  printf '[m3-docs-truth] error: python3 is required\n' >&2
  exit 1
fi

if ! command -v ruby >/dev/null 2>&1; then
  printf '[m3-docs-truth] error: ruby is required (YAML decode via Psych)\n' >&2
  exit 1
fi

export TZ=UTC
export LC_ALL=C
export PYTHONDONTWRITEBYTECODE=1

FRESHNESS_EXIT=0
STALE_EXIT=0
PARITY_EXIT=0
PUBLIC_BENCH_EXIT=0

python3 tools/ci/m3/docs_freshness_gate.py --repo-root . ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} \
  || FRESHNESS_EXIT=$?

python3 tools/ci/m3/stale_example_checker.py --repo-root . \
  || STALE_EXIT=$?

python3 -m tools.ci.m3.docs_public_proof_gate --repo-root . ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} \
  || PARITY_EXIT=$?

python3 ci/check_m3_public_benchmark_beta.py --repo-root . ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} \
  || PUBLIC_BENCH_EXIT=$?

if (( FRESHNESS_EXIT != 0 )) || (( STALE_EXIT != 0 )) || (( PARITY_EXIT != 0 )) || (( PUBLIC_BENCH_EXIT != 0 )); then
  printf '[m3-docs-truth] FAIL (freshness=%s, stale_examples=%s, parity=%s, public_benchmark=%s)\n' \
    "${FRESHNESS_EXIT}" "${STALE_EXIT}" "${PARITY_EXIT}" "${PUBLIC_BENCH_EXIT}" >&2
  if (( FRESHNESS_EXIT != 0 )); then
    exit "${FRESHNESS_EXIT}"
  fi
  if (( STALE_EXIT != 0 )); then
    exit "${STALE_EXIT}"
  fi
  if (( PARITY_EXIT != 0 )); then
    exit "${PARITY_EXIT}"
  fi
  exit "${PUBLIC_BENCH_EXIT}"
fi

printf '[m3-docs-truth] PASS\n'
