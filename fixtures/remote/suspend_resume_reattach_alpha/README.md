# Managed Workspace Suspend/Resume/Reattach Alpha Fixtures

These fixtures protect the bounded managed-workspace preview/runtime inspection
lane implemented in `crates/aureline-runtime/src/managed_alpha/`.

The cases prove:

- local and helper-backed runtime labels stay distinct;
- suspended workspaces expose stale, inspect-only preview truth;
- resumed workspaces can remain inspect-only until session refresh;
- reattach only permits mutation when the target witness still matches;
- reconnect-required rows block rerun and mutation until review completes.

Run the fixture replay with:

```sh
cargo test -p aureline-runtime --test managed_alpha
```
