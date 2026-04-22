#!/usr/bin/env bash
#
# Nightly benchmark-lab wrapper.
#
# Builds the seeded benchmark binaries (shell spike, bench_text_stack,
# bench_buffer), drives them against the protected corpus or a smoke
# subset, and writes one run-result record plus one human-readable
# summary per run. Output conforms to
# `schemas/benchmarks/run_result.schema.json`; the committed seeds under
# `artifacts/benchmarks/dashboard_seed/` are the reference shape.
#
# The script is the single entry point the nightly CI lane
# (`.github/workflows/nightly_benchmark.yml`) and a developer running
# `./tools/benchmark_lab.sh` both invoke, so CI and developer runs share
# the same build, corpus selection, and report shape.
#
# Usage:
#   ./tools/benchmark_lab.sh [--corpus-subset full|smoke]
#                            [--run-context reference_capture|provisional_capture|self_capture|smoke_subset]
#                            [--lane ci_nightly|ci_merge_queue|ci_preview|developer_local]
#                            [--trigger scheduled_nightly|manual_dispatch|commit_gated|developer_invocation]
#                            [--environment-preset self_capture_current_machine|ref_macos_arm64_nominal|ref_windows_x86_64_nominal|ref_linux_x86_64_nominal]
#                            [--out-dir DIR]
#                            [--regression-demo]
#                            [--skip-build]
#                            [--verify-seed]
#
# Defaults:
#   --corpus-subset       smoke
#   --run-context         self_capture
#   --lane                developer_local
#   --trigger             developer_invocation
#   --environment-preset  self_capture_current_machine
#   --out-dir             artifacts/benchmarks/dashboard_seed
#
# Modes:
#   (default)            Build the seeded bench binaries, run the smoke
#                        subset, and write a fresh run record + summary
#                        under --out-dir (subdirectories raw/ and report/).
#   --regression-demo    Emit the seeded regression run that intentionally
#                        trips `ff.benchmark_lab_health` with
#                        `regression_trigger_ref.kind = corpus_row_missing`.
#                        The script exits non-zero after writing the record,
#                        so the lane can demonstrate the failure path.
#   --verify-seed        Do not run any benchmark; diff the committed seed
#                        under `artifacts/benchmarks/dashboard_seed/` against
#                        the files this script would emit. Used by CI to
#                        keep the committed seed honest.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

CORPUS_SUBSET="smoke"
RUN_CONTEXT="self_capture"
LANE="developer_local"
TRIGGER="developer_invocation"
ENVIRONMENT_PRESET="self_capture_current_machine"
OUT_DIR="artifacts/benchmarks/dashboard_seed"
REGRESSION_DEMO="0"
SKIP_BUILD="0"
VERIFY_SEED="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --corpus-subset) CORPUS_SUBSET="${2:-}"; shift 2 ;;
    --corpus-subset=*) CORPUS_SUBSET="${1#--corpus-subset=}"; shift ;;
    --run-context) RUN_CONTEXT="${2:-}"; shift 2 ;;
    --run-context=*) RUN_CONTEXT="${1#--run-context=}"; shift ;;
    --lane) LANE="${2:-}"; shift 2 ;;
    --lane=*) LANE="${1#--lane=}"; shift ;;
    --trigger) TRIGGER="${2:-}"; shift 2 ;;
    --trigger=*) TRIGGER="${1#--trigger=}"; shift ;;
    --environment-preset) ENVIRONMENT_PRESET="${2:-}"; shift 2 ;;
    --environment-preset=*) ENVIRONMENT_PRESET="${1#--environment-preset=}"; shift ;;
    --out-dir) OUT_DIR="${2:-}"; shift 2 ;;
    --out-dir=*) OUT_DIR="${1#--out-dir=}"; shift ;;
    --regression-demo) REGRESSION_DEMO="1"; shift ;;
    --skip-build) SKIP_BUILD="1"; shift ;;
    --verify-seed) VERIFY_SEED="1"; shift ;;
    -h|--help)
      sed -n '2,60p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "benchmark_lab: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[benchmark_lab] %s\n' "$*"; }

mkdir -p "${OUT_DIR}/raw" "${OUT_DIR}/report"

if [[ "${VERIFY_SEED}" == "1" ]]; then
  export SOURCE_DATE_EPOCH=0
  export TZ=UTC
  export LC_ALL=C
  log "verifying committed seed under artifacts/benchmarks/dashboard_seed/"
  exec python3 "${SCRIPT_DIR}/benchmark_lab_emit.py" \
       --repo-root "${REPO_ROOT}" \
       --verify-seed
fi

# Pin timestamp-affecting inputs so reruns on the same commit produce
# byte-stable run records. Mirrors tools/build/build.sh.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

if [[ "${SKIP_BUILD}" != "1" ]]; then
  log "cargo build --locked --workspace --all-targets"
  cargo build --locked --workspace --all-targets >/dev/null
fi

PY_ARGS=(
  "${SCRIPT_DIR}/benchmark_lab_emit.py"
  --repo-root "${REPO_ROOT}"
  --out-dir "${OUT_DIR}"
  --corpus-subset "${CORPUS_SUBSET}"
  --run-context "${RUN_CONTEXT}"
  --lane "${LANE}"
  --trigger "${TRIGGER}"
  --environment-preset "${ENVIRONMENT_PRESET}"
)

if [[ "${REGRESSION_DEMO}" == "1" ]]; then
  PY_ARGS+=(--regression-demo)
fi

log "emitting run record"
python3 "${PY_ARGS[@]}"
RC=$?

if [[ "${REGRESSION_DEMO}" == "1" ]]; then
  # The regression-demo path is expected to exit non-zero so the lane
  # can demonstrate the failure path end to end.
  exit "${RC}"
fi

exit "${RC}"
