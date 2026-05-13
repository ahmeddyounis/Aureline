# Git Publish Review Alpha Fixtures

These fixtures exercise the local Git publish contract for source-control rows.
They cover upstream publish, missing remote blocking, and failed publish recovery
that can reopen the same review state without losing local commits.

Each case is replayed in a temporary Git repository by:

```sh
cargo test -p aureline-git --test publish_review_alpha
```
