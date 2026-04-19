#!/usr/bin/env bash
#
# Deterministic workspace build entry point.
#
# Builds every seeded crate under the pinned toolchain and writes a build
# identity record next to the target directory. The same command is used by
# developers and by CI; any CI-specific behavior is gated behind the CI
# environment variable.
#
# Usage:
#   ./tools/build/build.sh [--release] [--identity-out PATH]
#
# See docs/build/reproducible_build_baseline.md for the reproducibility
# contract this script implements.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

PROFILE="dev"
IDENTITY_OUT=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --release) PROFILE="release"; shift ;;
    --identity-out) IDENTITY_OUT="${2:-}"; shift 2 ;;
    --identity-out=*) IDENTITY_OUT="${1#--identity-out=}"; shift ;;
    *) echo "build: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[build] %s\n' "$*"; }

# Normalize timestamp-affecting inputs. SOURCE_DATE_EPOCH pins any embedded
# build timestamp to the commit time, so reruns of the same commit produce
# the same metadata. Callers may override by exporting it before invocation.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct 2>/dev/null || echo 0)"
fi
export SOURCE_DATE_EPOCH
export TZ=UTC
export LC_ALL=C

BUILD_ARGS=(--locked --workspace --all-targets)
if [[ "${PROFILE}" == "release" ]]; then
  BUILD_ARGS+=(--release)
fi

log "profile: ${PROFILE}"
log "SOURCE_DATE_EPOCH: ${SOURCE_DATE_EPOCH}"
log "rustc: $(rustc --version)"

log "cargo build ${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"

# Emit the build identity record. Keep this as the last step so a failed
# build does not produce a misleading artifact.
IDENTITY_DEFAULT_DIR="target"
[[ "${PROFILE}" == "release" ]] && IDENTITY_DEFAULT_DIR="target/release"
if [[ -z "${IDENTITY_OUT}" ]]; then
  IDENTITY_OUT="${IDENTITY_DEFAULT_DIR}/build_identity.json"
fi
mkdir -p "$(dirname "${IDENTITY_OUT}")"
"${SCRIPT_DIR}/print_build_identity.sh" --profile "${PROFILE}" > "${IDENTITY_OUT}"
log "build identity: ${IDENTITY_OUT}"
