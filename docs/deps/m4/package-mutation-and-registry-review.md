# Package Mutation And Registry Review

The stable dependency-manager package-mutation packet lives at
`artifacts/deps/m4/package-mutation-and-registry-review.json` and is embedded in
`aureline-deps`.

Stable claims are limited to `cargo` and `node_pnpm`. The packet proves that
package operations are manifest-scoped, review-first workspace mutations:
requested package ranges are separated from resolved exact identities, registry
auth is typed and redacted, lockfile impact is quantified, script/native-build
risk is explicit, and validation plus rollback are present before stable write
claims.

The user-facing Help projection is
`docs/help/deps/package-mutation-and-registry-review.md`.
