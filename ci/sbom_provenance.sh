#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0
#
# Emit the structural Cargo.lock SBOM projection and unsigned provenance
# summary. The lane deliberately remains narrower than release-grade SPDX,
# CycloneDX, or signed-attestation generation, but every field it does emit is
# derived from validated, content-digested repository inputs.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${CI_ARTIFACT_DIR:-${REPO_ROOT}/target/ci-artifacts}"
BUILD_IDENTITY_FILE="${OUT_DIR}/build_identity.json"
SBOM_FILE="${OUT_DIR}/sbom_workspace.json"
PROVENANCE_FILE="${OUT_DIR}/provenance_summary.json"

log() { printf '[sbom-provenance] %s\n' "$*"; }
die() { printf '[sbom-provenance] error: %s\n' "$*" >&2; exit 1; }

mkdir -p "${OUT_DIR}"

STAGING_DIR="$(mktemp -d "${OUT_DIR}/.sbom-provenance.XXXXXX")"
cleanup() {
  rm -rf -- "${STAGING_DIR}"
}
trap cleanup EXIT

# Remove exact generated outputs before validating any input. A failed rerun
# must not leave a stale artifact that a later step could mistake for current
# evidence.
rm -f -- "${SBOM_FILE}" "${PROVENANCE_FILE}"

if [[ ! -s "${BUILD_IDENTITY_FILE}" ]]; then
  log "writing build identity to ${BUILD_IDENTITY_FILE}"
  "${REPO_ROOT}/tools/build/print_build_identity.sh" > "${BUILD_IDENTITY_FILE}"
else
  log "validating existing build identity at ${BUILD_IDENTITY_FILE}"
fi

if command -v sha256sum >/dev/null 2>&1; then
  SHA256_PROGRAM="$(command -v sha256sum)"
  SHA256_ARGS=()
elif command -v shasum >/dev/null 2>&1; then
  SHA256_PROGRAM="$(command -v shasum)"
  SHA256_ARGS=(--sha256-arg=-a --sha256-arg=256)
else
  die "sha256sum or shasum is required; refusing to emit undigested provenance"
fi
[[ "${SHA256_PROGRAM}" == /* ]] || die \
  "SHA-256 program did not resolve to an absolute path: ${SHA256_PROGRAM}"

if grep -Eq '"profile"[[:space:]]*:[[:space:]]*"release"' \
  "${BUILD_IDENTITY_FILE}"; then
  BUILD_PROFILE="release"
  CARGO_PROFILE_ARGS=(--release)
  PROFILE_DIR="release"
elif grep -Eq '"profile"[[:space:]]*:[[:space:]]*"dev"' \
  "${BUILD_IDENTITY_FILE}"; then
  BUILD_PROFILE="dev"
  CARGO_PROFILE_ARGS=()
  PROFILE_DIR="debug"
else
  die "build identity does not declare profile dev or release"
fi

# A structurally valid identity can still have come from a previous checkout.
# Reproduce it with the same profile and require byte-for-byte equality before
# allowing it to anchor current provenance.
CURRENT_BUILD_IDENTITY="${STAGING_DIR}/current_build_identity.json"
"${REPO_ROOT}/tools/build/print_build_identity.sh" \
  --profile "${BUILD_PROFILE}" > "${CURRENT_BUILD_IDENTITY}"
cmp -s "${CURRENT_BUILD_IDENTITY}" "${BUILD_IDENTITY_FILE}" || die \
  "existing build identity does not match the current checkout and build environment"

CARGO_TARGET_ROOT="${CARGO_TARGET_DIR:-${REPO_ROOT}/target}"
TARGET_SUBDIR=""
if [[ -n "${CARGO_BUILD_TARGET:-}" ]]; then
  TARGET_SUBDIR="${CARGO_BUILD_TARGET}/"
fi
BIN_SUFFIX=""
if [[ "${OS:-}" == "Windows_NT" ]]; then
  BIN_SUFFIX=".exe"
fi
GENERATOR_BIN="${CARGO_TARGET_ROOT}/${TARGET_SUBDIR}${PROFILE_DIR}/aureline-sbom-provenance${BIN_SUFFIX}"

if [[ ! -x "${GENERATOR_BIN}" ]]; then
  log "building the locked provenance generator"
  CARGO_ARGS=(build --locked -p aureline-notices --bin aureline-sbom-provenance)
  if [[ ${#CARGO_PROFILE_ARGS[@]} -gt 0 ]]; then
    CARGO_ARGS+=("${CARGO_PROFILE_ARGS[@]}")
  fi
  if [[ "${AURELINE_SBOM_PROVENANCE_OFFLINE:-0}" == "1" ]]; then
    CARGO_ARGS+=(--offline)
  fi
  cargo "${CARGO_ARGS[@]}"
fi
[[ -x "${GENERATOR_BIN}" ]] || die \
  "provenance generator was not produced at ${GENERATOR_BIN}"

GENERATOR_ARGS=(
  --repo-root "${REPO_ROOT}"
  --out-dir "${STAGING_DIR}"
  --build-identity "${BUILD_IDENTITY_FILE}"
  --sha256-program "${SHA256_PROGRAM}"
)
if [[ ${#SHA256_ARGS[@]} -gt 0 ]]; then
  GENERATOR_ARGS+=("${SHA256_ARGS[@]}")
fi

log "validating registers, lockfile checksums, and build identity"
"${GENERATOR_BIN}" "${GENERATOR_ARGS[@]}"

[[ -s "${STAGING_DIR}/sbom_workspace.json" ]] || die \
  "generator did not emit sbom_workspace.json"
[[ -s "${STAGING_DIR}/provenance_summary.json" ]] || die \
  "generator did not emit provenance_summary.json"

mv -- "${STAGING_DIR}/sbom_workspace.json" "${SBOM_FILE}"
mv -- "${STAGING_DIR}/provenance_summary.json" "${PROVENANCE_FILE}"
log "wrote checksum-complete Cargo.lock projection to ${SBOM_FILE}"
log "wrote input-digested provenance summary to ${PROVENANCE_FILE}"
