# Git Mutation Review Alpha Fixtures

These fixtures exercise the preview-first Git mutation contract for source
control rows. They cover stage, unstage, discard, and checkpoint restore paths
without exporting raw patch bodies.

Each case is replayed in a temporary Git repository by
`cargo test -p aureline-git --test mutation_review_alpha`.
