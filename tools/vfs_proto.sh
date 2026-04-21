#!/usr/bin/env bash
#
# VFS / save prototype smoke wrapper.
#
# Builds and runs the `vfs_proto` binary against the frozen scenario
# table in `crates/aureline-vfs/src/harness.rs` and emits reviewable
# save-plan records. Counts only, no wall-clock, so the committed
# artifacts under `artifacts/fs/save_plan_examples/` stay byte-stable
# across hosts.
#
# Usage:
#   ./tools/vfs_proto.sh [--release] \
#                        [--emit PATH] \
#                        [--emit-scenarios DIR]
#
# Defaults:
#   --release           (off; dev profile)
#   --emit              <stdout> when --emit-scenarios is unset
#   --emit-scenarios    off (aggregate-only emission)
#
# See prototypes/vfs/README.md for the prototype's recorded holes,
# carry-forward items, and how this output feeds the later benchmark
# lab and production save pipeline.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PROFILE_FLAG=""
EMIT=""
EMIT_SCENARIOS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE_FLAG="--release"; shift ;;
    --emit) EMIT="${2:-}"; shift 2 ;;
    --emit=*) EMIT="${1#--emit=}"; shift ;;
    --emit-scenarios) EMIT_SCENARIOS="${2:-}"; shift 2 ;;
    --emit-scenarios=*) EMIT_SCENARIOS="${1#--emit-scenarios=}"; shift ;;
    -h|--help)
      sed -n '2,25p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "vfs_proto: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[vfs_proto] %s\n' "$*"; }

# Match the reproducibility posture of the other prototype wrappers:
# pin timestamp-affecting inputs so reruns produce the same metadata.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked -p aureline-vfs --bin vfs_proto)
if [[ -n "${PROFILE_FLAG}" ]]; then
  BUILD_ARGS+=("${PROFILE_FLAG}")
fi

log "cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

RUN_ARGS=()
if [[ -n "${EMIT}" ]]; then
  RUN_ARGS+=(--emit "${EMIT}")
fi
if [[ -n "${EMIT_SCENARIOS}" ]]; then
  RUN_ARGS+=(--emit-scenarios "${EMIT_SCENARIOS}")
fi

log "vfs_proto ${RUN_ARGS[*]}"
cargo run --quiet "${BUILD_ARGS[@]}" -- "${RUN_ARGS[@]}"

if [[ -n "${EMIT}" ]]; then
  log "aggregate: ${EMIT}"
fi
if [[ -n "${EMIT_SCENARIOS}" ]]; then
  log "scenarios: ${EMIT_SCENARIOS}"
fi
