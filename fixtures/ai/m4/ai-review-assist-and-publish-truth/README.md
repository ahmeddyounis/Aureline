# AI Review Assist And Publish Truth Fixtures

The canonical checked packet for this lane is:

- `artifacts/ai/m4/ai-review-assist-and-publish-truth/support_export.json`

It exercises three review scopes:

- selected diff with provider publication admitted
- uncommitted changes with missing provider write access downgraded to local/copy/export
- hosted review object marked outdated after material diff drift

The Rust tests in `crates/aureline-ai/src/ai_review_assist/tests.rs` validate the packet and mutate these cases to prove the stable invariants fail closed.
