# Baseline Build Health

Captured on 2026-05-15 from a clean working tree before this artifact was
written.

## Scope

- Binary entry point: `crates/aureline-shell/src/bin/aureline_shell.rs`
- Bootstrap route: `aureline_shell::bootstrap::run_native_shell()`
- Headless sanity path:
  `--open <folder> --headless-test-edit-save <file> --headless-test-write-hex <hex> --headless-test-report <path>`
- Toolchain pin: `rust-toolchain.toml` channel `1.84.0`

## Build Identity

`aureline-build-info` reported:

```text
exact_build_identity_ref: build-id:aureline:dev:0.0.0:aarch64-apple-darwin:dev:05649f1324e2
```

```json
{
  "schema_version": 1,
  "commit": "05649f1324e28d92904fa6ecbb3f4348c5e8d0ca",
  "commit_short": "05649f1324e2",
  "dirty": false,
  "toolchain_channel": "1.84.0",
  "rustc_version": "rustc 1.84.0 (9fc6b4312 2025-01-07)",
  "cargo_version": "cargo 1.84.0 (66221abde 2024-11-19)",
  "host_triple": "aarch64-apple-darwin",
  "target_triple": "aarch64-apple-darwin",
  "profile": "dev",
  "workspace_version": "0.0.0",
  "source_date_epoch": 1778790361,
  "build_timestamp_utc": "2026-05-14T20:26:01Z"
}
```

Identity command:

```sh
cargo run -p aureline-shell-spike --bin shell_spike -- --about
```

Result: exit 0.

## Build Checks

```sh
cargo check -p aureline-shell --bin aureline_shell
```

Result: exit 0, finished in 33.03s.

```sh
cargo check --workspace
```

Result: exit 0, finished in 1.51s.

```sh
cargo clippy --workspace
```

Result: exit 0, finished in 42.29s.

Observed warning baseline:

| Crate / target | Location | Lint |
|---|---:|---|
| `aureline-provider` bin `aureline_provider_alpha` | `crates/aureline-provider/src/bin/aureline_provider_alpha.rs:42` | `clippy::ptr_arg` |
| `aureline-provider` bin `aureline_provider_alpha` | `crates/aureline-provider/src/bin/aureline_provider_alpha.rs:43` | `clippy::needless_borrow` |
| `aureline-provider` bin `aureline_provider_alpha` | `crates/aureline-provider/src/bin/aureline_provider_alpha.rs:72` | `clippy::ptr_arg` |
| `aureline-workspace` lib | `crates/aureline-workspace/src/admission/mod.rs:1754` | `clippy::question_mark` |
| `aureline-search` lib | `crates/aureline-search/src/query_session.rs:115` | `clippy::too_many_arguments` |
| `aureline-search` lib | `crates/aureline-search/src/query_session.rs:149` | `clippy::too_many_arguments` |
| `aureline-runtime` lib | `crates/aureline-runtime/src/discovery/pytest/mod.rs:1104` | `clippy::only_used_in_recursion` |
| `aureline-runtime` lib | `crates/aureline-runtime/src/packages/mod.rs:935` | `clippy::too_many_arguments` |
| `aureline-runtime` lib | `crates/aureline-runtime/src/packages/mod.rs:1042` | `clippy::too_many_arguments` |
| `aureline-runtime` lib | `crates/aureline-runtime/src/tests/mod.rs:1168` | `clippy::needless_lifetimes` |
| `aureline-runtime` lib | `crates/aureline-runtime/src/tests/mod.rs:1620` | `clippy::if_same_then_else` |
| `aureline-ai` lib | `crates/aureline-ai/src/routing/mod.rs:1237` | `clippy::too_many_arguments` |

No `aureline-shell` warnings appeared in the captured Clippy output.

## Binary Entrypoint Sanity

Synthetic workspace fixture command:

```sh
tmp=$(mktemp -d)
printf 'old\n' > "$tmp/notes.txt"
cargo run -p aureline-shell --bin aureline_shell -- \
  --open "$tmp" \
  --headless-test-edit-save notes.txt \
  --headless-test-write-hex 68656c6c6f2d66726f6d2d686561646c6573730a \
  --headless-test-report "$tmp/report.json"
```

Result: exit 0. The report payload included:

```json
{
  "byte_count": 20,
  "exact_build_identity_ref": "build-id:aureline:dev:0.0.0:aarch64-apple-darwin:dev:05649f1324e2",
  "mode": "headless_edit_save",
  "outcome": "committed",
  "schema_version": 1,
  "write_strategy": "atomic_replace"
}
```

The saved file SHA-256 was:

```text
536210a10cc37f83ce51871b7ea19ad2243988c2bc7ec8758a93bdd3c8c814ae
```

## Blockers

No compile blocker was found for `aureline_shell`.

The desktop windowed startup path was not exercised in this headless
environment. The available bootstrap affordance for this run was the
headless edit/save route, which starts through the binary entry point and
exits cleanly without creating a native window.
