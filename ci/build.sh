#!/usr/bin/env bash
#
# CI entry point for the reproducible build lane.
#
# This script deliberately delegates to tools/build/*.sh so that CI and a
# developer running `./tools/build/build.sh` execute the exact same steps.
# The only CI-specific behavior is exporting a deterministic environment and
# copying the build identity into the CI artifact directory.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

export CI=1
export TZ=UTC
export LC_ALL=C
export CARGO_TERM_COLOR=never
export CARGO_NET_RETRY=3
export RUST_BACKTRACE=1

# SOURCE_DATE_EPOCH defaults to the commit time so rebuilds of the same
# commit produce identical build-identity records.
if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git -C "${REPO_ROOT}" log -1 --pretty=%ct)"
  export SOURCE_DATE_EPOCH
fi

ARTIFACT_DIR="${CI_ARTIFACT_DIR:-${REPO_ROOT}/target/ci-artifacts}"
mkdir -p "${ARTIFACT_DIR}"

"${REPO_ROOT}/tools/build/bootstrap.sh"
"${REPO_ROOT}/tools/build/build.sh" "$@" --identity-out "${ARTIFACT_DIR}/build_identity.json"
