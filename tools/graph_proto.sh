#!/usr/bin/env bash
#
# Semantic-workspace-graph seed prototype smoke wrapper.
#
# Builds and runs the `graph_proto` binary against the frozen
# scenario table in `crates/aureline-graph-proto/src/scenarios.rs`
# and emits reviewable counts-only structural records. No wall-clock
# times, so the committed seed artifact stays byte-stable across
# hosts.
#
# Usage:
#   ./tools/graph_proto.sh [--release] \
#                          [--emit PATH] \
#                          [--emit-scenarios DIR] \
#                          [--emit-graphs DIR]
#
# Defaults:
#   --release           (off; dev profile)
#   --emit              <stdout> when no other emit flag is set
#   --emit-scenarios    off (aggregate-only emission)
#   --emit-graphs       off (no per-scenario graph records)
#
# See prototypes/graph/README.md for the prototype's recorded holes,
# carry-forward items, and how this output feeds the later benchmark
# lab, search-planner lane, and production graph fabric.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

PROFILE_FLAG=""
EMIT=""
EMIT_SCENARIOS=""
EMIT_GRAPHS=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE_FLAG="--release"; shift ;;
    --emit) EMIT="${2:-}"; shift 2 ;;
    --emit=*) EMIT="${1#--emit=}"; shift ;;
    --emit-scenarios) EMIT_SCENARIOS="${2:-}"; shift 2 ;;
    --emit-scenarios=*) EMIT_SCENARIOS="${1#--emit-scenarios=}"; shift ;;
    --emit-graphs) EMIT_GRAPHS="${2:-}"; shift 2 ;;
    --emit-graphs=*) EMIT_GRAPHS="${1#--emit-graphs=}"; shift ;;
    -h|--help)
      sed -n '2,27p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "graph_proto: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[graph_proto] %s\n' "$*"; }

# Match the reproducibility posture of the other prototype wrappers:
# pin timestamp-affecting inputs so reruns produce the same metadata.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked -p aureline-graph-proto --bin graph_proto)
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
if [[ -n "${EMIT_GRAPHS}" ]]; then
  RUN_ARGS+=(--emit-graphs "${EMIT_GRAPHS}")
fi

log "graph_proto ${RUN_ARGS[*]}"
cargo run --quiet "${BUILD_ARGS[@]}" -- "${RUN_ARGS[@]}"

if [[ -n "${EMIT}" ]]; then
  log "aggregate: ${EMIT}"
fi
if [[ -n "${EMIT_SCENARIOS}" ]]; then
  log "scenarios: ${EMIT_SCENARIOS}"
fi
if [[ -n "${EMIT_GRAPHS}" ]]; then
  log "graphs: ${EMIT_GRAPHS}"
fi
