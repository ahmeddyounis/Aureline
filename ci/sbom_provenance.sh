#!/usr/bin/env bash
#
# Placeholder SBOM and provenance lane.
#
# This script makes the provenance / SBOM lane observable in CI from day
# one without claiming release-grade SBOM or attestation conformance.
# Real generators replace it incrementally:
#   - tools/governance/spdx_sbom.sh    — SPDX SBOM emission for the
#                                        workspace + transitive Cargo graph.
#   - tools/governance/cyclonedx.sh    — CycloneDX export when a security-
#                                        tooling consumer requires it.
#   - tools/governance/attest.sh       — in-toto / SLSA-style attestation
#                                        once release signing exists.
#
# Until those land, this script:
#   1. resolves the workspace build identity (the anchor that real SBOM
#      and provenance documents will reference);
#   2. emits a minimal structural SBOM stub describing the workspace
#      crates declared in Cargo.toml;
#   3. records a placeholder provenance summary that names the build
#      identity, the toolchain pin, and the compliance checklist
#      version it consumed;
#   4. exits zero on success.
#
# See docs/governance/provenance_and_compliance_baseline.md for the
# governance baseline this script implements.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${CI_ARTIFACT_DIR:-${REPO_ROOT}/target/ci-artifacts}"
mkdir -p "${OUT_DIR}"

log() { printf '[sbom-provenance] %s\n' "$*"; }

# ---------------------------------------------------------------------------
# 1. Build identity.
# ---------------------------------------------------------------------------
BUILD_IDENTITY_FILE="${OUT_DIR}/build_identity.json"
if [[ ! -s "${BUILD_IDENTITY_FILE}" ]]; then
  log "writing build identity to ${BUILD_IDENTITY_FILE}"
  "${REPO_ROOT}/tools/build/print_build_identity.sh" > "${BUILD_IDENTITY_FILE}"
else
  log "reusing build identity at ${BUILD_IDENTITY_FILE}"
fi

# Pull a couple of fields out of the build identity for downstream stubs.
# Avoid jq: this script must run on a stock CI host without extra deps.
extract_field() {
  local field="$1"
  local file="$2"
  awk -v key="\"${field}\"" '
    {
      for (i = 1; i <= NF; i++) {
        if ($i == key ":" || $i == key) {
          # Find the value: everything after the colon on this line.
          line = $0
          sub(/^[^:]*:[[:space:]]*/, "", line)
          sub(/,[[:space:]]*$/, "", line)
          gsub(/^"|"$/, "", line)
          print line
          exit
        }
      }
    }
  ' "${file}"
}

COMMIT="$(extract_field commit "${BUILD_IDENTITY_FILE}")"
COMMIT_SHORT="$(extract_field commit_short "${BUILD_IDENTITY_FILE}")"
TOOLCHAIN_CHANNEL="$(extract_field toolchain_channel "${BUILD_IDENTITY_FILE}")"
WORKSPACE_VERSION="$(extract_field workspace_version "${BUILD_IDENTITY_FILE}")"
SOURCE_DATE_EPOCH_VAL="$(extract_field source_date_epoch "${BUILD_IDENTITY_FILE}")"
BUILD_TIMESTAMP_UTC="$(extract_field build_timestamp_utc "${BUILD_IDENTITY_FILE}")"

# ---------------------------------------------------------------------------
# 2. Workspace SBOM stub.
#
# Enumerates the workspace crates declared in Cargo.toml. Deliberately
# does NOT claim SPDX or CycloneDX conformance — this is a structural
# placeholder. The real SBOM emitter will replace this file with an
# SPDX document of the same name.
# ---------------------------------------------------------------------------
SBOM_FILE="${OUT_DIR}/sbom_workspace.json"
log "writing workspace SBOM stub to ${SBOM_FILE}"

# Read workspace member paths from Cargo.toml without invoking cargo, so
# this step is independent of toolchain availability on minimal CI hosts.
WORKSPACE_MEMBERS=()
while IFS= read -r member; do
  [[ -n "${member}" ]] && WORKSPACE_MEMBERS+=("${member}")
done < <(awk '
  /^\[workspace\]/ { in_ws = 1; next }
  /^\[/ && !/^\[workspace\]/ { in_ws = 0 }
  in_ws && /members[[:space:]]*=/ { in_members = 1 }
  in_members {
    while (match($0, /"[^"]+"/)) {
      print substr($0, RSTART + 1, RLENGTH - 2)
      $0 = substr($0, RSTART + RLENGTH)
    }
    if ($0 ~ /\]/) { in_members = 0 }
  }
' "${REPO_ROOT}/Cargo.toml")

{
  printf '{\n'
  printf '  "schema_version": 1,\n'
  printf '  "format": "aureline-workspace-sbom-stub",\n'
  printf '  "format_note": "Structural placeholder. Not an SPDX or CycloneDX document. Replaced by tools/governance/spdx_sbom.sh when the real SBOM lane lands.",\n'
  printf '  "build_identity": {\n'
  printf '    "commit": "%s",\n' "${COMMIT}"
  printf '    "commit_short": "%s",\n' "${COMMIT_SHORT}"
  printf '    "workspace_version": "%s",\n' "${WORKSPACE_VERSION}"
  printf '    "toolchain_channel": "%s",\n' "${TOOLCHAIN_CHANNEL}"
  printf '    "source_date_epoch": %s,\n' "${SOURCE_DATE_EPOCH_VAL:-0}"
  printf '    "build_timestamp_utc": "%s"\n' "${BUILD_TIMESTAMP_UTC}"
  printf '  },\n'
  printf '  "workspace_license": "Apache-2.0",\n'
  printf '  "external_dependencies": [],\n'
  printf '  "external_dependencies_note": "Workspace currently has zero external Cargo dependencies; see artifacts/governance/compliance_checklist.yaml.",\n'
  printf '  "workspace_members": [\n'
  total="${#WORKSPACE_MEMBERS[@]}"
  i=0
  for member in "${WORKSPACE_MEMBERS[@]}"; do
    i=$((i + 1))
    sep=","
    [[ ${i} -eq ${total} ]] && sep=""
    printf '    { "path": "%s" }%s\n' "${member}" "${sep}"
  done
  printf '  ]\n'
  printf '}\n'
} > "${SBOM_FILE}"

# ---------------------------------------------------------------------------
# 3. Provenance summary stub.
# ---------------------------------------------------------------------------
PROVENANCE_FILE="${OUT_DIR}/provenance_summary.json"
log "writing provenance summary stub to ${PROVENANCE_FILE}"

CHECKLIST_PATH="artifacts/governance/compliance_checklist.yaml"
CHECKLIST_SCHEMA_VERSION="$(awk -F ':' '
  /^schema_version[[:space:]]*:/ {
    val = $2
    gsub(/[[:space:]]/, "", val)
    print val
    exit
  }
' "${REPO_ROOT}/${CHECKLIST_PATH}" 2>/dev/null || echo "unknown")"

{
  printf '{\n'
  printf '  "schema_version": 1,\n'
  printf '  "format": "aureline-provenance-summary-stub",\n'
  printf '  "format_note": "Placeholder. Replaced by tools/governance/attest.sh when the release signing and attestation lane lands.",\n'
  printf '  "build_identity_ref": "%s",\n' "$(basename "${BUILD_IDENTITY_FILE}")"
  printf '  "sbom_ref": "%s",\n' "$(basename "${SBOM_FILE}")"
  printf '  "build_identity": {\n'
  printf '    "commit": "%s",\n' "${COMMIT}"
  printf '    "commit_short": "%s",\n' "${COMMIT_SHORT}"
  printf '    "toolchain_channel": "%s",\n' "${TOOLCHAIN_CHANNEL}"
  printf '    "workspace_version": "%s",\n' "${WORKSPACE_VERSION}"
  printf '    "source_date_epoch": %s,\n' "${SOURCE_DATE_EPOCH_VAL:-0}"
  printf '    "build_timestamp_utc": "%s"\n' "${BUILD_TIMESTAMP_UTC}"
  printf '  },\n'
  printf '  "compliance_checklist": {\n'
  printf '    "path": "%s",\n' "${CHECKLIST_PATH}"
  printf '    "schema_version": %s\n' "${CHECKLIST_SCHEMA_VERSION:-0}"
  printf '  },\n'
  printf '  "attestations": [],\n'
  printf '  "signatures": []\n'
  printf '}\n'
} > "${PROVENANCE_FILE}"

log "done"
