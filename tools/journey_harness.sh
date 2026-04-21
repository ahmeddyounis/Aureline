#!/usr/bin/env bash
#
# Protected user-journey trace harness wrapper.
#
# Drives tools/journey_harness/journey_harness.py for startup, shell open,
# placeholder open / edit / save, and restore-adjacent journeys. Emits one
# journey-trace record per invocation conforming to
# `schemas/traces/journey_trace.schema.json`; the committed seeds under
# `fixtures/journeys/` are the reference shape.
#
# The script is the single entry point developers and (later) CI share,
# so both paths honour the same reproducibility posture
# (SOURCE_DATE_EPOCH, TZ, LC_ALL).
#
# Usage:
#   ./tools/journey_harness.sh [--journey JOURNEY_ID]
#                              [--emit-all]
#                              [--out-dir DIR]
#                              [--host-os macos|linux|windows|unknown]
#                              [--rustc-target-triple TRIPLE]
#                              [--verify-seed]
#
# Defaults:
#   --emit-all                       (when --journey is not set)
#   --out-dir   fixtures/journeys
#   --host-os                linux
#   --rustc-target-triple    unknown-linux-gnu
#
# Modes:
#   (default / --emit-all)  Emit every seeded journey under --out-dir.
#   --journey JOURNEY_ID    Emit exactly one journey by stable id.
#   --verify-seed           Do not emit anywhere on disk; re-emit under a
#                           tempdir and unified-diff against the committed
#                           seed. CI uses this to keep the committed seed
#                           honest.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

JOURNEY=""
EMIT_ALL="0"
OUT_DIR="fixtures/journeys"
HOST_OS="linux"
RUSTC_TARGET_TRIPLE="unknown-linux-gnu"
VERIFY_SEED="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --journey) JOURNEY="${2:-}"; shift 2 ;;
    --journey=*) JOURNEY="${1#--journey=}"; shift ;;
    --emit-all) EMIT_ALL="1"; shift ;;
    --out-dir) OUT_DIR="${2:-}"; shift 2 ;;
    --out-dir=*) OUT_DIR="${1#--out-dir=}"; shift ;;
    --host-os) HOST_OS="${2:-}"; shift 2 ;;
    --host-os=*) HOST_OS="${1#--host-os=}"; shift ;;
    --rustc-target-triple) RUSTC_TARGET_TRIPLE="${2:-}"; shift 2 ;;
    --rustc-target-triple=*) RUSTC_TARGET_TRIPLE="${1#--rustc-target-triple=}"; shift ;;
    --verify-seed) VERIFY_SEED="1"; shift ;;
    -h|--help)
      sed -n '2,40p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "journey_harness: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[journey_harness] %s\n' "$*"; }

# Pin timestamp-affecting inputs so reruns on the same commit produce
# byte-stable journey records. Mirrors tools/benchmark_lab.sh.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

if [[ "${VERIFY_SEED}" == "1" ]]; then
  log "verifying committed seed under ${OUT_DIR}/"
  exec python3 "${SCRIPT_DIR}/journey_harness/journey_harness.py" \
       --repo-root "${REPO_ROOT}" \
       --verify-seed
fi

PY_ARGS=(
  "${SCRIPT_DIR}/journey_harness/journey_harness.py"
  --repo-root "${REPO_ROOT}"
  --out-dir "${OUT_DIR}"
  --host-os "${HOST_OS}"
  --rustc-target-triple "${RUSTC_TARGET_TRIPLE}"
)

if [[ -n "${JOURNEY}" ]]; then
  PY_ARGS+=(--journey "${JOURNEY}")
else
  # Default: emit every seeded journey.
  PY_ARGS+=(--emit-all)
  EMIT_ALL="1"
fi

log "emitting journey trace(s) under ${OUT_DIR}/"
python3 "${PY_ARGS[@]}"
