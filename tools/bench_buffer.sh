#!/usr/bin/env bash
#
# Buffer smoke / bench wrapper.
#
# Builds and runs the `bench_buffer` binary against the frozen
# scenario table in `crates/aureline-bench/src/buffer.rs` and emits:
#
#   - a structural metrics JSON record (counts only, no wall-clock)
#     under `artifacts/buffer/buffer_metrics_seed.json`;
#   - deterministic human-readable undo-example traces under
#     `artifacts/buffer/undo_examples/`.
#
# Both artifact families are byte-stable across hosts; the benchmark
# lab layers wall-clock timing on top when it needs to score against
# the protected-hot-path budgets frozen in the buffer ADR.
#
# Usage:
#   ./tools/bench_buffer.sh [--release] \
#                           [--emit PATH] \
#                           [--emit-undo-examples DIR]
#
# Defaults:
#   --release              (off; dev profile)
#   --emit                 artifacts/buffer/buffer_metrics_seed.json
#   --emit-undo-examples   artifacts/buffer/undo_examples
#
# See prototypes/buffer/README.md for the prototype's recorded holes,
# carry-forward items, and how this output feeds the later benchmark
# lab.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PROFILE_FLAG=""
EMIT="artifacts/buffer/buffer_metrics_seed.json"
EMIT_UNDO_EXAMPLES="artifacts/buffer/undo_examples"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE_FLAG="--release"; shift ;;
    --emit) EMIT="${2:-}"; shift 2 ;;
    --emit=*) EMIT="${1#--emit=}"; shift ;;
    --emit-undo-examples) EMIT_UNDO_EXAMPLES="${2:-}"; shift 2 ;;
    --emit-undo-examples=*) EMIT_UNDO_EXAMPLES="${1#--emit-undo-examples=}"; shift ;;
    -h|--help)
      sed -n '2,30p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "bench_buffer: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[bench_buffer] %s\n' "$*"; }

# Match the reproducibility posture of tools/build/build.sh: pin
# timestamp-affecting inputs so reruns produce the same metadata.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked -p aureline-bench --bin bench_buffer)
if [[ -n "${PROFILE_FLAG}" ]]; then
  BUILD_ARGS+=("${PROFILE_FLAG}")
fi

log "cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

RUN_ARGS=(--emit "${EMIT}")
if [[ -n "${EMIT_UNDO_EXAMPLES}" ]]; then
  RUN_ARGS+=(--emit-undo-examples "${EMIT_UNDO_EXAMPLES}")
fi

log "bench_buffer ${RUN_ARGS[*]}"
cargo run --quiet "${BUILD_ARGS[@]}" -- "${RUN_ARGS[@]}"

log "metrics: ${EMIT}"
if [[ -n "${EMIT_UNDO_EXAMPLES}" ]]; then
  log "undo examples: ${EMIT_UNDO_EXAMPLES}"
fi
