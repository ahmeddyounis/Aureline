# Git Publish Review Alpha Fixtures

These fixtures exercise the local Git publish contract for source-control rows.
They cover upstream publish, missing remote blocking, stale-route blocking
before mutation, and a real attempted push that fails at the receive side.
Both failure paths reopen review without losing local commits, but the fixture
asserts the important `blocked_no_changes_made` versus `failed` distinction.

Each case is replayed in a temporary Git repository by:

```sh
cargo test -p aureline-git --test publish_review_alpha
```
