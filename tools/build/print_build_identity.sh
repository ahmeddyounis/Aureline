#!/usr/bin/env bash
#
# Emit the workspace build identity as a JSON object on stdout.
#
# Fields match schemas/build/build_identity.schema.json. The record is
# deterministic for a given (commit, toolchain, target) triple when
# SOURCE_DATE_EPOCH is supplied; see the schema for which fields are
# provisional vs. fixed.
#
# Usage:
#   ./tools/build/print_build_identity.sh [--profile dev|release]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

PROFILE="dev"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) PROFILE="${2:-dev}"; shift 2 ;;
    --profile=*) PROFILE="${1#--profile=}"; shift ;;
    *) echo "print_build_identity: unknown argument: $1" >&2; exit 2 ;;
  esac
done

commit="$(git rev-parse HEAD 2>/dev/null || echo "unknown")"
commit_short="$(git rev-parse --short=12 HEAD 2>/dev/null || echo "unknown")"
if git diff-index --quiet HEAD -- 2>/dev/null; then
  dirty="false"
else
  dirty="true"
fi

if command -v rustc >/dev/null 2>&1; then
  rustc_version="$(rustc --version)"
  host_triple="$(rustc -vV | awk -F ': ' '/^host:/ { print $2 }')"
else
  rustc_version="unknown"
  host_triple="unknown"
fi
if command -v cargo >/dev/null 2>&1; then
  cargo_version="$(cargo --version)"
else
  cargo_version="unknown"
fi
target_triple="${CARGO_BUILD_TARGET:-${host_triple}}"

toolchain_channel="$(awk -F '"' '/^channel/ { print $2; exit }' rust-toolchain.toml 2>/dev/null || echo "unknown")"
workspace_version="$(awk -F '"' '/^version/ { print $2; exit }' Cargo.toml 2>/dev/null || echo "unknown")"

source_date_epoch="${SOURCE_DATE_EPOCH:-$(git log -1 --pretty=%ct 2>/dev/null || echo 0)}"
build_timestamp_utc="$(TZ=UTC date -u -r "${source_date_epoch}" +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null \
  || TZ=UTC date -u -d "@${source_date_epoch}" +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null \
  || echo "unknown")"

cat <<EOF
{
  "schema_version": 1,
  "commit": "${commit}",
  "commit_short": "${commit_short}",
  "dirty": ${dirty},
  "toolchain_channel": "${toolchain_channel}",
  "rustc_version": "${rustc_version}",
  "cargo_version": "${cargo_version}",
  "host_triple": "${host_triple}",
  "target_triple": "${target_triple}",
  "profile": "${PROFILE}",
  "workspace_version": "${workspace_version}",
  "source_date_epoch": ${source_date_epoch},
  "build_timestamp_utc": "${build_timestamp_utc}"
}
EOF
