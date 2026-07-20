#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Aureline contributors
# SPDX-License-Identifier: Apache-2.0
#
# Focused regression tests for the reproducible-build bootstrap and identity
# scripts. The tests run in a disposable Git fixture with fake Rust commands,
# so they do not modify the caller's checkout or contact the network.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/aureline-build-script-tests.XXXXXX")"
FIXTURE_ROOT="${TEST_ROOT}/repo"
FAKE_BIN="${TEST_ROOT}/bin"
COMMAND_LOG="${TEST_ROOT}/cargo.log"

cleanup() {
  rm -rf "${TEST_ROOT}"
}
trap cleanup EXIT

fail() {
  printf '[build-script-tests] FAIL: %s\n' "$*" >&2
  exit 1
}

assert_log_line() {
  local expected="$1"
  grep -Fqx -- "${expected}" "${COMMAND_LOG}" || \
    fail "cargo log did not contain: ${expected}"
}

mkdir -p "${FIXTURE_ROOT}/tools/build" "${FAKE_BIN}"
cp "${SCRIPT_DIR}/bootstrap.sh" "${FIXTURE_ROOT}/tools/build/bootstrap.sh"
cp "${SCRIPT_DIR}/print_build_identity.sh" \
  "${FIXTURE_ROOT}/tools/build/print_build_identity.sh"

cat > "${FIXTURE_ROOT}/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "1.75.0"
EOF
cat > "${FIXTURE_ROOT}/Cargo.toml" <<'EOF'
[workspace]
members = []

[workspace.package]
version = "0.1.0"
EOF
printf '# locked fixture\n' > "${FIXTURE_ROOT}/Cargo.lock"

cat > "${FAKE_BIN}/rustup" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
cat > "${FAKE_BIN}/rustc" <<'EOF'
#!/usr/bin/env bash
if [[ "${1:-}" == "-vV" ]]; then
  printf 'rustc 1.75.0 (fixture)\nhost: x86_64-fixture-linux-gnu\n'
else
  printf 'rustc 1.75.0 (fixture)\n'
fi
EOF
cat > "${FAKE_BIN}/cargo" <<'EOF'
#!/usr/bin/env bash
if [[ "${1:-}" == "--version" ]]; then
  printf 'cargo 1.75.0 (fixture)\n'
  exit 0
fi
printf '%s\n' "$*" >> "${TEST_CARGO_LOG:?}"
if [[ "${1:-}" == "fetch" && "${TEST_FETCH_FAIL:-0}" == "1" ]]; then
  exit 41
fi
exit 0
EOF
chmod +x "${FAKE_BIN}/rustup" "${FAKE_BIN}/rustc" "${FAKE_BIN}/cargo"

git -C "${FIXTURE_ROOT}" init -q
git -C "${FIXTURE_ROOT}" config user.name "Aureline Test"
git -C "${FIXTURE_ROOT}" config user.email "test@aureline.invalid"
git -C "${FIXTURE_ROOT}" add .
git -c core.hooksPath=/dev/null -c commit.gpgsign=false \
  -C "${FIXTURE_ROOT}" commit -q -m "test: seed fixture"

export PATH="${FAKE_BIN}:${PATH}"
export TEST_CARGO_LOG="${COMMAND_LOG}"

IDENTITY_FILE="${TEST_ROOT}/identity.json"
SOURCE_DATE_EPOCH=0 \
  "${FIXTURE_ROOT}/tools/build/print_build_identity.sh" > "${IDENTITY_FILE}"
python3 - "${IDENTITY_FILE}" <<'PY'
import json
import pathlib
import sys

payload = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["dirty"] is False, payload
assert payload["source_date_epoch"] == 0, payload
assert payload["build_timestamp_utc"] == "1970-01-01T00:00:00Z", payload
PY

printf 'untracked\n' > "${FIXTURE_ROOT}/untracked.txt"
TARGET_WITH_CONTROLS=$'fixture"target\\path\nline\tend\n'
SOURCE_DATE_EPOCH=0000 CARGO_BUILD_TARGET="${TARGET_WITH_CONTROLS}" \
  "${FIXTURE_ROOT}/tools/build/print_build_identity.sh" > "${IDENTITY_FILE}"
EXPECTED_TARGET="${TARGET_WITH_CONTROLS}" python3 - "${IDENTITY_FILE}" <<'PY'
import json
import os
import pathlib
import sys

payload = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
assert payload["dirty"] is True, payload
assert payload["target_triple"] == os.environ["EXPECTED_TARGET"], payload
assert payload["source_date_epoch"] == 0, payload
PY

if SOURCE_DATE_EPOCH='0,"injected":true' \
  "${FIXTURE_ROOT}/tools/build/print_build_identity.sh" \
  > "${IDENTITY_FILE}" 2> "${TEST_ROOT}/identity-error.log"; then
  fail "build identity accepted a non-numeric SOURCE_DATE_EPOCH"
fi
grep -Fq 'SOURCE_DATE_EPOCH must be a non-negative decimal integer' \
  "${TEST_ROOT}/identity-error.log" || fail "missing SOURCE_DATE_EPOCH error"

printf 'sentinel\n' > "${COMMAND_LOG}"
if TEST_FETCH_FAIL=1 "${FIXTURE_ROOT}/tools/build/bootstrap.sh" \
  > "${TEST_ROOT}/bootstrap-failure.log" 2>&1; then
  fail "bootstrap hid a cargo fetch --locked failure"
fi
assert_log_line 'fetch --locked'
[[ "$(wc -l < "${COMMAND_LOG}" | tr -d ' ')" == "2" ]] || \
  fail "bootstrap attempted an unlocked fallback after locked fetch failed"

: > "${COMMAND_LOG}"
TEST_FETCH_FAIL=0 "${FIXTURE_ROOT}/tools/build/bootstrap.sh" \
  > "${TEST_ROOT}/bootstrap-success.log" 2>&1
assert_log_line 'fetch --locked'
assert_log_line 'metadata --locked --format-version=1 --no-deps'

: > "${COMMAND_LOG}"
"${FIXTURE_ROOT}/tools/build/bootstrap.sh" --offline \
  > "${TEST_ROOT}/bootstrap-offline.log" 2>&1
assert_log_line 'metadata --locked --format-version=1 --no-deps --offline'
if grep -Fq 'fetch' "${COMMAND_LOG}"; then
  fail "offline bootstrap attempted cargo fetch"
fi

mv "${FIXTURE_ROOT}/Cargo.lock" "${TEST_ROOT}/Cargo.lock"
: > "${COMMAND_LOG}"
if "${FIXTURE_ROOT}/tools/build/bootstrap.sh" --offline \
  > "${TEST_ROOT}/bootstrap-missing-lock.log" 2>&1; then
  fail "bootstrap accepted a missing Cargo.lock"
fi
[[ ! -s "${COMMAND_LOG}" ]] || fail "bootstrap invoked cargo without Cargo.lock"
grep -Fq 'Cargo.lock is missing' "${TEST_ROOT}/bootstrap-missing-lock.log" || \
  fail "missing Cargo.lock error was not actionable"

printf '[build-script-tests] PASS\n'
