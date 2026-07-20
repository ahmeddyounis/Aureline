#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0
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

die() { printf '[print-build-identity] error: %s\n' "$*" >&2; exit 1; }

PROFILE="dev"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      [[ $# -ge 2 ]] || die "--profile requires dev or release"
      PROFILE="$2"
      shift 2
      ;;
    --profile=*) PROFILE="${1#--profile=}"; shift ;;
    *) echo "print_build_identity: unknown argument: $1" >&2; exit 2 ;;
  esac
done

case "${PROFILE}" in
  dev|release) ;;
  *) die "unsupported profile '${PROFILE}'; expected dev or release" ;;
esac

commit="$(git rev-parse HEAD 2>/dev/null || echo "unknown")"
commit_short="$(git rev-parse --short=12 HEAD 2>/dev/null || echo "unknown")"
if [[ "${commit}" == "unknown" ]]; then
  dirty="true"
else
  worktree_status="$(
    git status --porcelain=v1 --untracked-files=all --ignore-submodules=none 2>/dev/null
  )" || die "could not inspect the complete Git working-tree state"
  if [[ -z "${worktree_status}" ]]; then
    dirty="false"
  else
    dirty="true"
  fi
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
[[ "${source_date_epoch}" =~ ^[0-9]+$ ]] || die \
  "SOURCE_DATE_EPOCH must be a non-negative decimal integer"
source_date_epoch="$(printf '%s' "${source_date_epoch}" | sed 's/^0*//; s/^$/0/')"

if build_timestamp_utc="$(
  TZ=UTC date -u -r "${source_date_epoch}" +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null
)"; then
  :
elif build_timestamp_utc="$(
  TZ=UTC date -u -d "@${source_date_epoch}" +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null
)"; then
  :
else
  die "SOURCE_DATE_EPOCH '${source_date_epoch}' is outside the supported date range"
fi

# Emit one JSON string safely without adding a trailing newline. Bash variables
# cannot contain NUL bytes; all other JSON control characters are escaped.
json_string() {
  local value="$1"
  local char code i
  local LC_ALL=C

  printf '"'
  for ((i = 0; i < ${#value}; i++)); do
    char="${value:i:1}"
    case "${char}" in
      '"') printf '\\"' ;;
      \\) printf '\\\\' ;;
      $'\b') printf '\\b' ;;
      $'\f') printf '\\f' ;;
      $'\n') printf '\\n' ;;
      $'\r') printf '\\r' ;;
      $'\t') printf '\\t' ;;
      *)
        printf -v code '%d' "'${char}"
        if ((code < 32)); then
          printf '\\u%04x' "${code}"
        else
          printf '%s' "${char}"
        fi
        ;;
    esac
  done
  printf '"'
}

printf '{\n'
printf '  "schema_version": 1,\n'
printf '  "commit": '
json_string "${commit}"
printf ',\n'
printf '  "commit_short": '
json_string "${commit_short}"
printf ',\n'
printf '  "dirty": %s,\n' "${dirty}"
for build_identity_field in \
  toolchain_channel rustc_version cargo_version host_triple target_triple profile workspace_version
do
  case "${build_identity_field}" in
    toolchain_channel) build_identity_value="${toolchain_channel}" ;;
    rustc_version) build_identity_value="${rustc_version}" ;;
    cargo_version) build_identity_value="${cargo_version}" ;;
    host_triple) build_identity_value="${host_triple}" ;;
    target_triple) build_identity_value="${target_triple}" ;;
    profile) build_identity_value="${PROFILE}" ;;
    workspace_version) build_identity_value="${workspace_version}" ;;
  esac
  printf '  "%s": ' "${build_identity_field}"
  json_string "${build_identity_value}"
  printf ',\n'
done
printf '  "source_date_epoch": %s,\n' "${source_date_epoch}"
printf '  "build_timestamp_utc": '
json_string "${build_timestamp_utc}"
printf '\n'
printf '}\n'
