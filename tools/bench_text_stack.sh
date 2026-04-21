#!/usr/bin/env bash
#
# Text-stack smoke / bench wrapper.
#
# Builds and runs the `bench_text_stack` binary against the committed
# shaping-smoke corpus and emits a structural metrics JSON. The record
# is counts-only (no wall-clock) so the committed seed under
# `artifacts/bench/text_stack_metrics_seed.json` stays byte-stable
# across hosts; the benchmark lab layers timing on top when it needs
# it.
#
# Usage:
#   ./tools/bench_text_stack.sh [--release] [--iterations N] \
#                               [--corpus PATH] [--emit PATH]
#
# Defaults:
#   --release      (off; dev profile)
#   --iterations   2
#   --corpus       fixtures/text/shaping_smoke_cases.txt
#   --emit         artifacts/bench/text_stack_metrics_seed.json
#
# See prototypes/text_stack/README.md for the prototype's recorded
# holes, carry-forward items, and how this output feeds the later
# benchmark lab.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PROFILE_FLAG=""
ITERATIONS="2"
CORPUS="fixtures/text/shaping_smoke_cases.txt"
EMIT="artifacts/bench/text_stack_metrics_seed.json"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE_FLAG="--release"; shift ;;
    --iterations) ITERATIONS="${2:-}"; shift 2 ;;
    --iterations=*) ITERATIONS="${1#--iterations=}"; shift ;;
    --corpus) CORPUS="${2:-}"; shift 2 ;;
    --corpus=*) CORPUS="${1#--corpus=}"; shift ;;
    --emit) EMIT="${2:-}"; shift 2 ;;
    --emit=*) EMIT="${1#--emit=}"; shift ;;
    -h|--help)
      sed -n '2,22p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "bench_text_stack: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[bench_text_stack] %s\n' "$*"; }

# Match the reproducibility posture of tools/build/build.sh: pin
# timestamp-affecting inputs so reruns produce the same metadata.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked -p aureline-bench --bin bench_text_stack)
if [[ -n "${PROFILE_FLAG}" ]]; then
  BUILD_ARGS+=("${PROFILE_FLAG}")
fi

log "cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

RUN_ARGS=(--corpus "${CORPUS}" --iterations "${ITERATIONS}" --emit "${EMIT}")
log "bench_text_stack ${RUN_ARGS[*]}"
cargo run --quiet "${BUILD_ARGS[@]}" -- "${RUN_ARGS[@]}"

log "metrics: ${EMIT}"
