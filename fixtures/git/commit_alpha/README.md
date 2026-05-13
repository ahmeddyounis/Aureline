# Git Commit Alpha Fixtures

These fixtures exercise the local commit contract for source-control rows. They
cover normal commits, author validation failure, amend guardrails, explicit
amend apply, and squash marker behavior without publishing to a remote.

Each case is replayed in a temporary Git repository by:

```sh
cargo test -p aureline-git --test commit_alpha
```

