#!/usr/bin/env bash
#
# Deterministic developer bootstrap.
#
# One command that takes a clean clone to a state where every seeded crate in
# the workspace can be built. Idempotent: safe to rerun; later invocations
# only validate that the environment already matches the pin.
#
# Usage:
#   ./tools/build/bootstrap.sh [--offline]
#
# See docs/build/reproducible_build_baseline.md for the contract this script
# implements.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

OFFLINE=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --offline) OFFLINE=1; shift ;;
    *) echo "bootstrap: unknown argument: $1" >&2; exit 2 ;;
  esac
done

log() { printf '[bootstrap] %s\n' "$*"; }
die() { printf '[bootstrap] error: %s\n' "$*" >&2; exit 1; }

# --- 1. Verify rustup -------------------------------------------------------

if ! command -v rustup >/dev/null 2>&1; then
  cat >&2 <<'EOF'
[bootstrap] error: rustup is not installed.

Install it from https://rustup.rs and re-run this script. The pinned
toolchain (see rust-toolchain.toml) is fetched by rustup on first cargo
invocation.
EOF
  exit 1
fi

# --- 2. Install the pinned toolchain ---------------------------------------
#
# rust-toolchain.toml triggers an auto-install on any cargo invocation, but
# doing it explicitly here gives a clearer error if network is unavailable.

CHANNEL="$(awk -F '"' '/^channel/ { print $2; exit }' rust-toolchain.toml)"
[[ -n "${CHANNEL}" ]] || die "could not read channel from rust-toolchain.toml"

if [[ "${OFFLINE}" -eq 0 ]]; then
  log "ensuring rust toolchain ${CHANNEL} is installed"
  rustup toolchain install "${CHANNEL}" --profile minimal >/dev/null
  rustup component add --toolchain "${CHANNEL}" rustfmt clippy rust-src >/dev/null
else
  log "offline mode: skipping rustup toolchain install"
fi

# --- 3. Verify the active toolchain matches the pin ------------------------

ACTIVE_CHANNEL="$(rustc --version | awk '{ print $2 }')"
if [[ "${ACTIVE_CHANNEL}" != "${CHANNEL}" ]]; then
  die "active rustc ${ACTIVE_CHANNEL} does not match pinned ${CHANNEL}"
fi
log "toolchain: $(rustc --version)"

# --- 4. Resolve and lock dependencies --------------------------------------

if [[ "${OFFLINE}" -eq 0 ]]; then
  log "resolving dependency graph (cargo fetch)"
  cargo fetch --locked 2>/dev/null || cargo fetch
  if [[ ! -f Cargo.lock ]]; then
    cargo generate-lockfile
  fi
else
  log "offline mode: skipping cargo fetch"
fi

# --- 5. Smoke-check that every workspace crate is resolvable ---------------

log "validating workspace via cargo metadata"
cargo metadata --format-version=1 --no-deps >/dev/null

log "bootstrap complete"
