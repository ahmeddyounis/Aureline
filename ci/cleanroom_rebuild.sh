#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Clean-room rebuild lane.
#
# Reuses the pinned bootstrap/build/provenance entry points from ci/build.sh,
# but wraps them in a stricter contract:
#   - the checkout must be clean;
#   - the output directory is self-contained;
#   - the lane emits a deterministic input manifest plus a provenance-capture
#     summary that makes current trust assumptions and limitations explicit.
#
# Usage:
#   ./ci/cleanroom_rebuild.sh [--out-dir PATH] [--offline]
#
# See docs/build/cleanroom_rebuild_lane.md for the governing contract.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

OUT_DIR="${REPO_ROOT}/target/cleanroom-rebuild"
OFFLINE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --out-dir=*)
      OUT_DIR="${1#--out-dir=}"
      shift
      ;;
    --offline)
      OFFLINE=1
      shift
      ;;
    *)
      echo "cleanroom_rebuild: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

log() { printf '[cleanroom-rebuild] %s\n' "$*"; }
die() { printf '[cleanroom-rebuild] error: %s\n' "$*" >&2; exit 1; }

json_escape() {
  printf '%s' "$1" | tr '\n' ' ' | sed 's/\\/\\\\/g; s/"/\\"/g'
}

sha256_file() {
  local file="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "${file}" | awk '{ print $1 }'
    return
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "${file}" | awk '{ print $1 }'
    return
  fi
  die "neither shasum nor sha256sum is available"
}

extract_field() {
  local field="$1"
  local file="$2"
  awk -v key="\"${field}\"" '
    {
      for (i = 1; i <= NF; i++) {
        if ($i == key ":" || $i == key) {
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

bool_json() {
  if [[ "$1" -eq 0 ]]; then
    printf 'false'
  else
    printf 'true'
  fi
}

producer_identifier_ref() {
  if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    printf 'producer:ci:github-actions:cleanroom-rebuild'
  else
    printf 'producer:local:cleanroom:manual'
  fi
}

artifact_family_ref_for() {
  case "$1" in
    shell_spike|shell_spike.exe)
      printf 'ide_binary.primary_shell_spike'
      ;;
    bench_*|bench_*.exe)
      printf 'workspace_binary.%s' "$1"
      ;;
    *_proto|*_proto.exe|graph_proto|graph_proto.exe|reactive_proto|reactive_proto.exe|vfs_proto|vfs_proto.exe)
      printf 'workspace_binary.%s' "$1"
      ;;
    *)
      printf 'workspace_output.%s' "$1"
      ;;
  esac
}

exact_build_family_for() {
  case "$1" in
    shell_spike|shell_spike.exe)
      printf 'ide_binary'
      ;;
    sbom_workspace.json)
      printf 'sbom_document'
      ;;
    *)
      printf ''
      ;;
  esac
}

publishability_for() {
  case "$1" in
    shell_spike|shell_spike.exe|bench_*|bench_*.exe|*_proto|*_proto.exe|graph_proto|graph_proto.exe|reactive_proto|reactive_proto.exe|vfs_proto|vfs_proto.exe)
      printf 'development_prototype_non_publishable'
      ;;
    sbom_workspace.json|provenance_summary.json)
      printf 'non_publishable_placeholder'
      ;;
    *)
      printf 'internal_control_artifact'
      ;;
  esac
}

relpath() {
  local path="$1"
  local abs
  abs="$(cd "$(dirname "${path}")" && pwd)/$(basename "${path}")"
  printf '%s' "${abs#${REPO_ROOT}/}"
}

tree_status="$(git status --porcelain --untracked-files=all)"
if [[ -n "${tree_status}" ]]; then
  die "clean-room rebuild requires a clean checkout; commit or stash local changes first"
fi

SOURCE_REF="$(git rev-parse HEAD)"
SOURCE_REF_SHORT="$(git rev-parse --short=12 HEAD)"
SOURCE_REMOTE_URL="$(git config --get remote.origin.url || true)"
SOURCE_REMOTE_URL="${SOURCE_REMOTE_URL:-not_configured}"

if [[ -z "${SOURCE_DATE_EPOCH:-}" ]]; then
  SOURCE_DATE_EPOCH="$(git log -1 --pretty=%ct)"
  export SOURCE_DATE_EPOCH
fi

mkdir -p "${OUT_DIR}"

CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${OUT_DIR}/cargo-target}"
export CARGO_TARGET_DIR

log "output directory: ${OUT_DIR}"
log "cargo target dir: ${CARGO_TARGET_DIR}"
log "source ref: ${SOURCE_REF}"
log "offline: $(bool_json "${OFFLINE}")"

if [[ "${OFFLINE}" -eq 1 ]]; then
  export CI=1
  export TZ=UTC
  export LC_ALL=C
  export CARGO_TERM_COLOR=never
  export CARGO_NET_RETRY=3
  export RUST_BACKTRACE=1

  "${REPO_ROOT}/tools/build/bootstrap.sh" --offline
  "${REPO_ROOT}/tools/build/build.sh" --release --identity-out "${OUT_DIR}/build_identity.json"
  CI_ARTIFACT_DIR="${OUT_DIR}" "${REPO_ROOT}/ci/sbom_provenance.sh"
else
  CI_ARTIFACT_DIR="${OUT_DIR}" "${SCRIPT_DIR}/build.sh" --release
fi

BUILD_IDENTITY_FILE="${OUT_DIR}/build_identity.json"
SBOM_FILE="${OUT_DIR}/sbom_workspace.json"
PROVENANCE_SUMMARY_FILE="${OUT_DIR}/provenance_summary.json"
INPUT_MANIFEST_FILE="${OUT_DIR}/cleanroom_input_manifest.json"
ARTIFACT_DIGEST_FILE="${OUT_DIR}/artifact_digests.json"
PROVENANCE_CAPTURE_FILE="${OUT_DIR}/provenance_capture.json"

[[ -s "${BUILD_IDENTITY_FILE}" ]] || die "missing ${BUILD_IDENTITY_FILE}"
[[ -s "${SBOM_FILE}" ]] || die "missing ${SBOM_FILE}"
[[ -s "${PROVENANCE_SUMMARY_FILE}" ]] || die "missing ${PROVENANCE_SUMMARY_FILE}"

COMMIT="$(extract_field commit "${BUILD_IDENTITY_FILE}")"
COMMIT_SHORT="$(extract_field commit_short "${BUILD_IDENTITY_FILE}")"
TOOLCHAIN_CHANNEL="$(extract_field toolchain_channel "${BUILD_IDENTITY_FILE}")"
RUSTC_VERSION="$(extract_field rustc_version "${BUILD_IDENTITY_FILE}")"
CARGO_VERSION="$(extract_field cargo_version "${BUILD_IDENTITY_FILE}")"
HOST_TRIPLE="$(extract_field host_triple "${BUILD_IDENTITY_FILE}")"
TARGET_TRIPLE="$(extract_field target_triple "${BUILD_IDENTITY_FILE}")"
PROFILE="$(extract_field profile "${BUILD_IDENTITY_FILE}")"
WORKSPACE_VERSION="$(extract_field workspace_version "${BUILD_IDENTITY_FILE}")"
SOURCE_DATE_EPOCH_VAL="$(extract_field source_date_epoch "${BUILD_IDENTITY_FILE}")"
BUILD_TIMESTAMP_UTC="$(extract_field build_timestamp_utc "${BUILD_IDENTITY_FILE}")"

RUST_TOOLCHAIN_DIGEST="$(sha256_file rust-toolchain.toml)"
CARGO_CONFIG_DIGEST="$(sha256_file .cargo/config.toml)"
CARGO_TOML_DIGEST="$(sha256_file Cargo.toml)"
CARGO_LOCK_DIGEST="$(sha256_file Cargo.lock)"

RUSTUP_DIST_SERVER_VAL="${RUSTUP_DIST_SERVER:-https://static.rust-lang.org}"
RUSTUP_UPDATE_ROOT_VAL="${RUSTUP_UPDATE_ROOT:-https://static.rust-lang.org/rustup}"
CARGO_REGISTRY_PROTOCOL_VAL="${CARGO_REGISTRIES_CRATES_IO_PROTOCOL:-default}"
CARGO_HOME_VAL="${CARGO_HOME:-${HOME}/.cargo}"
RUSTUP_HOME_VAL="${RUSTUP_HOME:-${HOME}/.rustup}"

log "writing artifact digest manifest to ${ARTIFACT_DIGEST_FILE}"
ARTIFACT_FILES=()
while IFS= read -r file; do
  [[ -n "${file}" ]] && ARTIFACT_FILES+=("${file}")
done < <(
  find "${CARGO_TARGET_DIR}/release" -maxdepth 1 -type f \
    ! -name '*.d' \
    ! -name 'build_identity.json' \
    ! -name '*.rlib' \
    ! -name '*.rmeta' \
    | LC_ALL=C sort
)

{
  printf '{\n'
  printf '  "schema_version": 1,\n'
  printf '  "record_kind": "cleanroom_artifact_digest_manifest",\n'
  printf '  "generated_at": "%s",\n' "$(json_escape "${BUILD_TIMESTAMP_UTC}")"
  printf '  "build_identity_ref": "%s",\n' "$(json_escape "$(basename "${BUILD_IDENTITY_FILE}")")"
  printf '  "artifacts": [\n'
  total="${#ARTIFACT_FILES[@]}"
  i=0
  for file in "${ARTIFACT_FILES[@]}"; do
    i=$((i + 1))
    basename_file="$(basename "${file}")"
    sha256="$(sha256_file "${file}")"
    size_bytes="$(wc -c < "${file}" | tr -d '[:space:]')"
    sep=","
    [[ "${i}" -eq "${total}" ]] && sep=""
    printf '    {\n'
    printf '      "artifact_id": "%s",\n' "$(json_escape "${basename_file}")"
    printf '      "artifact_family_ref": "%s",\n' "$(json_escape "$(artifact_family_ref_for "${basename_file}")")"
    printf '      "path": "%s",\n' "$(json_escape "$(relpath "${file}")")"
    printf '      "sha256": "sha256:%s",\n' "$(json_escape "${sha256}")"
    printf '      "size_bytes": %s,\n' "${size_bytes:-0}"
    printf '      "publishability_class": "%s",\n' "$(json_escape "$(publishability_for "${basename_file}")")"
    printf '      "artifact_graph_seed_ref": "%s",\n' "$(json_escape "artifact-graph-node:aureline:m0:cleanroom:${basename_file}")"
    exact_build_family="$(exact_build_family_for "${basename_file}")"
    if [[ -n "${exact_build_family}" ]]; then
      printf '      "exact_build_artifact_family_class": "%s",\n' "$(json_escape "${exact_build_family}")"
    else
      printf '      "exact_build_artifact_family_class": null,\n'
    fi
    printf '      "exact_build_linkage_state": "baseline_build_identity_only"\n'
    printf '    }%s\n' "${sep}"
  done
  printf '  ]\n'
  printf '}\n'
} > "${ARTIFACT_DIGEST_FILE}"

log "writing input manifest to ${INPUT_MANIFEST_FILE}"
{
  printf '{\n'
  printf '  "schema_version": 1,\n'
  printf '  "record_kind": "cleanroom_input_manifest",\n'
  printf '  "lane_id": "cleanroom_rebuild_seed",\n'
  printf '  "generated_at": "%s",\n' "$(json_escape "${BUILD_TIMESTAMP_UTC}")"
  printf '  "source": {\n'
  printf '    "git_ref": "commit:%s",\n' "$(json_escape "${SOURCE_REF}")"
  printf '    "git_ref_short": "%s",\n' "$(json_escape "${SOURCE_REF_SHORT}")"
  printf '    "tree_state": "clean_checkout",\n'
  printf '    "remote_origin_url": "%s"\n' "$(json_escape "${SOURCE_REMOTE_URL}")"
  printf '  },\n'
  printf '  "build_parameters": {\n'
  printf '    "profile": "release",\n'
  printf '    "offline": %s,\n' "$(bool_json "${OFFLINE}")"
  printf '    "source_date_epoch": %s,\n' "${SOURCE_DATE_EPOCH_VAL:-0}"
  printf '    "cargo_target_dir": "%s"\n' "$(json_escape "$(relpath "${CARGO_TARGET_DIR}")")"
  printf '  },\n'
  printf '  "pinned_inputs": [\n'
  printf '    { "path": "rust-toolchain.toml", "role": "rust_toolchain_pin", "sha256": "sha256:%s" },\n' "$(json_escape "${RUST_TOOLCHAIN_DIGEST}")"
  printf '    { "path": ".cargo/config.toml", "role": "workspace_cargo_config", "sha256": "sha256:%s" },\n' "$(json_escape "${CARGO_CONFIG_DIGEST}")"
  printf '    { "path": "Cargo.toml", "role": "workspace_manifest", "sha256": "sha256:%s" },\n' "$(json_escape "${CARGO_TOML_DIGEST}")"
  printf '    { "path": "Cargo.lock", "role": "lockfile", "sha256": "sha256:%s" }\n' "$(json_escape "${CARGO_LOCK_DIGEST}")"
  printf '  ],\n'
  printf '  "toolchain": {\n'
  printf '    "toolchain_channel": "%s",\n' "$(json_escape "${TOOLCHAIN_CHANNEL}")"
  printf '    "rustc_version": "%s",\n' "$(json_escape "${RUSTC_VERSION}")"
  printf '    "cargo_version": "%s",\n' "$(json_escape "${CARGO_VERSION}")"
  printf '    "host_triple": "%s",\n' "$(json_escape "${HOST_TRIPLE}")"
  printf '    "target_triple": "%s"\n' "$(json_escape "${TARGET_TRIPLE}")"
  printf '  },\n'
  printf '  "mirror_inputs": {\n'
  printf '    "mode": "%s",\n' "$([[ "${OFFLINE}" -eq 1 ]] && printf 'offline' || printf 'online')"
  printf '    "rustup_dist_server": "%s",\n' "$(json_escape "${RUSTUP_DIST_SERVER_VAL}")"
  printf '    "rustup_update_root": "%s",\n' "$(json_escape "${RUSTUP_UPDATE_ROOT_VAL}")"
  printf '    "cargo_registry_protocol": "%s",\n' "$(json_escape "${CARGO_REGISTRY_PROTOCOL_VAL}")"
  printf '    "cargo_home": "%s",\n' "$(json_escape "${CARGO_HOME_VAL}")"
  printf '    "rustup_home": "%s"\n' "$(json_escape "${RUSTUP_HOME_VAL}")"
  printf '  },\n'
  printf '  "required_commands": [\n'
  printf '    "./tools/build/bootstrap.sh%s",\n' "$([[ "${OFFLINE}" -eq 1 ]] && printf ' --offline' || printf '')"
  printf '    "./ci/build.sh --release",\n'
  printf '    "./ci/sbom_provenance.sh"\n'
  printf '  ],\n'
  printf '  "trust_assumptions": [\n'
  printf '    {\n'
  printf '      "assumption_id": "mirror.rustup_and_crates",\n'
  printf '      "assumption": "The configured rustup and Cargo endpoints serve the pinned toolchain and lockfile content without transparent mutation.",\n'
  printf '      "why_it_matters": "A clean-room rebuild is only comparable if the toolchain and dependency mirrors resolve the same bytes."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "assumption_id": "signing.release_keys_absent",\n'
  printf '      "assumption": "Release signing and attestation keys are intentionally absent from this lane.",\n'
  printf '      "why_it_matters": "The lane proves rebuild inputs and placeholder provenance capture, not release-grade signatures."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "assumption_id": "nondeterminism.binary_bytes_not_claimed",\n'
  printf '      "assumption": "Meaningful sameness is currently judged on build-identity fields plus artifact digests, not on a repository-wide byte-identical binary guarantee.",\n'
  printf '      "why_it_matters": "The reproducible-build baseline only guarantees deterministic identity records today."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "assumption_id": "developer_shortcuts.not_part_of_lane",\n'
  printf '      "assumption": "Dirty checkouts, ad hoc local cache tweaks, and unpublished signing shortcuts are outside the supported lane contract.",\n'
  printf '      "why_it_matters": "The clean-room lane must stay explainable from committed files and the documented command alone."\n'
  printf '    }\n'
  printf '  ],\n'
  printf '  "documentation_refs": [\n'
  printf '    "docs/build/reproducible_build_baseline.md",\n'
  printf '    "docs/build/cleanroom_rebuild_lane.md"\n'
  printf '  ]\n'
  printf '}\n'
} > "${INPUT_MANIFEST_FILE}"

log "writing provenance capture to ${PROVENANCE_CAPTURE_FILE}"
{
  printf '{\n'
  printf '  "schema_version": 1,\n'
  printf '  "record_kind": "cleanroom_provenance_capture",\n'
  printf '  "capture_id": "cleanroom-capture:%s:release",\n' "$(json_escape "${COMMIT_SHORT}")"
  printf '  "captured_at": "%s",\n' "$(json_escape "${BUILD_TIMESTAMP_UTC}")"
  printf '  "producer_lane": {\n'
  printf '    "lane_class": "reproduced_clean_room",\n'
  printf '    "producer_identifier_ref": "%s",\n' "$(json_escape "$(producer_identifier_ref)")"
  printf '    "documented_command": "./ci/cleanroom_rebuild.sh --out-dir %s",\n' "$(json_escape "$(relpath "${OUT_DIR}")")"
  printf '    "workflow_ref": ".github/workflows/cleanroom_rebuild.yml"\n'
  printf '  },\n'
  printf '  "build_identity_ref": "%s",\n' "$(json_escape "$(basename "${BUILD_IDENTITY_FILE}")")"
  printf '  "input_manifest_ref": "%s",\n' "$(json_escape "$(basename "${INPUT_MANIFEST_FILE}")")"
  printf '  "artifact_digest_manifest_ref": "%s",\n' "$(json_escape "$(basename "${ARTIFACT_DIGEST_FILE}")")"
  printf '  "sbom_ref": "%s",\n' "$(json_escape "$(basename "${SBOM_FILE}")")"
  printf '  "provenance_summary_ref": "%s",\n' "$(json_escape "$(basename "${PROVENANCE_SUMMARY_FILE}")")"
  printf '  "exact_build_identity_linkage": {\n'
  printf '    "model_source": "docs/build/exact_build_identity_model.md",\n'
  printf '    "linkage_state": "baseline_build_identity_only",\n'
  printf '    "shared_axes": {\n'
  printf '      "commit": "%s",\n' "$(json_escape "${COMMIT}")"
  printf '      "workspace_version": "%s",\n' "$(json_escape "${WORKSPACE_VERSION}")"
  printf '      "toolchain_channel": "%s",\n' "$(json_escape "${TOOLCHAIN_CHANNEL}")"
  printf '      "target_triple": "%s",\n' "$(json_escape "${TARGET_TRIPLE}")"
  printf '      "profile": "%s",\n' "$(json_escape "${PROFILE}")"
  printf '      "source_date_epoch": %s\n' "${SOURCE_DATE_EPOCH_VAL:-0}"
  printf '    },\n'
  printf '    "comparison_basis": "Compare the fixed build-identity axes first, then compare artifact_digests.json; byte-identical binaries are not yet a release claim.",\n'
  printf '    "artifact_graph_seed_refs": [\n'
  printf '      "artifact-graph-node:aureline:m0:cleanroom:build_identity",\n'
  printf '      "artifact-graph-node:aureline:m0:cleanroom:sbom_workspace",\n'
  printf '      "artifact-graph-node:aureline:m0:cleanroom:provenance_summary"\n'
  printf '    ]\n'
  printf '  },\n'
  printf '  "artifact_family_refs": [\n'
  total_capture_rows=$(( ${#ARTIFACT_FILES[@]} + 6 ))
  capture_row_index=0
  for file in "${ARTIFACT_FILES[@]}"; do
    capture_row_index=$((capture_row_index + 1))
    basename_file="$(basename "${file}")"
    sep=","
    [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
    printf '    {\n'
    printf '      "artifact_family_ref": "%s",\n' "$(json_escape "$(artifact_family_ref_for "${basename_file}")")"
    printf '      "output_ref": "%s",\n' "$(json_escape "$(relpath "${file}")")"
    printf '      "publishability_class": "%s",\n' "$(json_escape "$(publishability_for "${basename_file}")")"
    printf '      "artifact_graph_seed_ref": "%s",\n' "$(json_escape "artifact-graph-node:aureline:m0:cleanroom:${basename_file}")"
    exact_build_family="$(exact_build_family_for "${basename_file}")"
    if [[ -n "${exact_build_family}" ]]; then
      printf '      "exact_build_artifact_family_class": "%s"\n' "$(json_escape "${exact_build_family}")"
    else
      printf '      "exact_build_artifact_family_class": null\n'
    fi
    printf '    }%s\n' "${sep}"
  done

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "workspace_build_identity.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${BUILD_IDENTITY_FILE}")")"
  printf '      "publishability_class": "internal_control_artifact",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:build_identity",\n'
  printf '      "exact_build_artifact_family_class": null\n'
  printf '    }%s\n' "${sep}"

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "cleanroom_artifact_digest_manifest.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${ARTIFACT_DIGEST_FILE}")")"
  printf '      "publishability_class": "internal_control_artifact",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:artifact_digests",\n'
  printf '      "exact_build_artifact_family_class": null\n'
  printf '    }%s\n' "${sep}"

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "workspace_sbom_stub.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${SBOM_FILE}")")"
  printf '      "publishability_class": "non_publishable_placeholder",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:sbom_workspace",\n'
  printf '      "exact_build_artifact_family_class": "sbom_document"\n'
  printf '    }%s\n' "${sep}"

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "workspace_provenance_summary.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${PROVENANCE_SUMMARY_FILE}")")"
  printf '      "publishability_class": "non_publishable_placeholder",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:provenance_summary",\n'
  printf '      "exact_build_artifact_family_class": null\n'
  printf '    }%s\n' "${sep}"

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "cleanroom_input_manifest.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${INPUT_MANIFEST_FILE}")")"
  printf '      "publishability_class": "internal_control_artifact",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:input_manifest",\n'
  printf '      "exact_build_artifact_family_class": null\n'
  printf '    }%s\n' "${sep}"

  capture_row_index=$((capture_row_index + 1))
  sep=","
  [[ "${capture_row_index}" -eq "${total_capture_rows}" ]] && sep=""
  printf '    {\n'
  printf '      "artifact_family_ref": "cleanroom_provenance_capture.primary",\n'
  printf '      "output_ref": "%s",\n' "$(json_escape "$(basename "${PROVENANCE_CAPTURE_FILE}")")"
  printf '      "publishability_class": "internal_control_artifact",\n'
  printf '      "artifact_graph_seed_ref": "artifact-graph-node:aureline:m0:cleanroom:provenance_capture",\n'
  printf '      "exact_build_artifact_family_class": null\n'
  printf '    }%s\n' "${sep}"
  printf '  ],\n'
  printf '  "known_limitations": [\n'
  printf '    {\n'
  printf '      "row_id": "lim.signing_dependencies_absent",\n'
  printf '      "status": "open",\n'
  printf '      "summary": "Release signing, notarization, and final attestation material are intentionally absent from this lane."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "row_id": "lim.mirror_assumptions_declared_not_verified",\n'
  printf '      "status": "open",\n'
  printf '      "summary": "The lane records which rustup/Cargo mirrors were used, but it does not yet cryptographically verify mirror equivalence beyond the pinned toolchain and lockfile inputs."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "row_id": "lim.binary_byte_identity_not_claimed",\n'
  printf '      "status": "open",\n'
  printf '      "summary": "The build-identity record is deterministic today; full byte-identical binaries remain future work."\n'
  printf '    },\n'
  printf '    {\n'
  printf '      "row_id": "lim.developer_shortcuts_unsupported",\n'
  printf '      "status": "open",\n'
  printf '      "summary": "Dirty checkouts, hidden local cache edits, and unpublished signing shortcuts are intentionally rejected or treated as outside the supported lane."\n'
  printf '    }\n'
  printf '  ],\n'
  printf '  "documentation_refs": [\n'
  printf '    "docs/build/cleanroom_rebuild_lane.md",\n'
  printf '    "docs/build/reproducible_build_baseline.md",\n'
  printf '    "docs/governance/provenance_and_compliance_baseline.md"\n'
  printf '  ]\n'
  printf '}\n'
} > "${PROVENANCE_CAPTURE_FILE}"

log "clean-room rebuild artifacts:"
log "  - $(relpath "${INPUT_MANIFEST_FILE}")"
log "  - $(relpath "${BUILD_IDENTITY_FILE}")"
log "  - $(relpath "${SBOM_FILE}")"
log "  - $(relpath "${PROVENANCE_SUMMARY_FILE}")"
log "  - $(relpath "${ARTIFACT_DIGEST_FILE}")"
log "  - $(relpath "${PROVENANCE_CAPTURE_FILE}")"
