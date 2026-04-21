#!/usr/bin/env bash
#
# Large-file path prototype smoke wrapper.
#
# Builds and runs the `largefile_proto` binary against the frozen
# scenario table in `crates/aureline-largefile-proto/src/harness.rs`
# and emits a structural metrics JSON record (counts only, no
# wall-clock) under `artifacts/bench/large_file_proto_metrics.json`.
#
# The artifact is byte-stable across hosts; the benchmark lab layers
# wall-clock timing on top when it scores against the protected-hot-
# path budgets frozen in the buffer / large-file ADR.
#
# Usage:
#   ./tools/largefile_proto.sh [--release] \
#                              [--emit PATH] \
#                              [--scratch-dir DIR]
#
# Defaults:
#   --release       (off; dev profile)
#   --emit          artifacts/bench/large_file_proto_metrics.json
#   --scratch-dir   <a fresh temp dir>
#
# See prototypes/large_file/README.md for the prototype's recorded
# holes, carry-forward items, and how this output feeds the later
# benchmark lab.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PROFILE_FLAG=""
EMIT="artifacts/bench/large_file_proto_metrics.json"
SCRATCH_DIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE_FLAG="--release"; shift ;;
    --emit) EMIT="${2:-}"; shift 2 ;;
    --emit=*) EMIT="${1#--emit=}"; shift ;;
    --scratch-dir) SCRATCH_DIR="${2:-}"; shift 2 ;;
    --scratch-dir=*) SCRATCH_DIR="${1#--scratch-dir=}"; shift ;;
    -h|--help)
      sed -n '2,30p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "largefile_proto: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[largefile_proto] %s\n' "$*"; }

# Match the reproducibility posture of tools/build/build.sh and
# tools/bench_buffer.sh: pin timestamp-affecting inputs so reruns
# produce the same metadata.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked -p aureline-largefile-proto --bin largefile_proto)
if [[ -n "${PROFILE_FLAG}" ]]; then
  BUILD_ARGS+=("${PROFILE_FLAG}")
fi

log "cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

RUN_ARGS=(--emit "${EMIT}")
if [[ -n "${SCRATCH_DIR}" ]]; then
  RUN_ARGS+=(--scratch-dir "${SCRATCH_DIR}")
fi

log "largefile_proto ${RUN_ARGS[*]}"
cargo run --quiet "${BUILD_ARGS[@]}" -- "${RUN_ARGS[@]}"

log "metrics: ${EMIT}"
