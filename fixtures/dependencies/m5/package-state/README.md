# Package-state descriptor fixtures

These fixtures exercise the cross-ecosystem package-state descriptor
(`aureline-deps`, `package_state_descriptors`) across the source kinds the
vocabulary must work for: a **private-registry** transitive dependency with an
open advisory, a **workspace-local** member with no registry source, and a
**VCS-pinned** path/source dependency awaiting license review.

Each file is a `package_state_descriptors` packet validated against
`schemas/deps/package-state-descriptors.schema.json` and the typed model. They
use refs, redacted source labels, controlled enum values, and count buckets
only; they carry no raw registry URLs, manifests, lockfiles, package bodies,
tokens, or credential material. Every descriptor binds to the frozen
package-state matrix through `references_matrix_id`.
