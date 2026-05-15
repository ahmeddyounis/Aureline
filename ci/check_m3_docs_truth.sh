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
#
# Both validators refresh their durable JSON captures on every run;
# under --check mode the freshness gate also fails if the generated
# truth report would drift from upstream truth.

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
manifest vocabularies.

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

FRESHNESS_EXIT=0
STALE_EXIT=0

python3 tools/ci/m3/docs_freshness_gate.py --repo-root . ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} \
  || FRESHNESS_EXIT=$?

python3 tools/ci/m3/stale_example_checker.py --repo-root . \
  || STALE_EXIT=$?

if (( FRESHNESS_EXIT != 0 )) || (( STALE_EXIT != 0 )); then
  printf '[m3-docs-truth] FAIL (freshness=%s, stale_examples=%s)\n' \
    "${FRESHNESS_EXIT}" "${STALE_EXIT}" >&2
  if (( FRESHNESS_EXIT != 0 )); then
    exit "${FRESHNESS_EXIT}"
  fi
  exit "${STALE_EXIT}"
fi

printf '[m3-docs-truth] PASS\n'
