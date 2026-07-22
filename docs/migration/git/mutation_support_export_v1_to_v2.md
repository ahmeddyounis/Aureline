<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

# Git mutation support export v1 to v2

## Why v2 is required

`git_mutation_support_export_record` v1 copied `workspace_ref` and several
lineage refs directly into a record labeled `metadata_safe_default`. Those refs
could contain caller-selected names and, before opaque mutation IDs, normalized
repository paths. The shape therefore crossed the support boundary with more
identity data than its redaction label admitted.

Version 2 is a privacy-narrowing replacement. It retains operation, phase,
retention, and lineage correlation through domain-separated SHA-256 digests.
It never embeds raw workspace, repository, path, actor, command, patch,
backend-output, or failure-detail values. Its strict schema is
[`mutation_support_export.schema.json`](../../../schemas/git/mutation_support_export.schema.json).

## Reader and writer rules

- Writers emit v2 only.
- V1 rows remain local inspection records; they must not be forwarded,
  attached, logged into a support packet, or converted by copying their raw
  refs.
- When the source preview or result still exists, reproject that source through
  the v2 exporter. When it does not, omit the legacy row and disclose
  `unavailable_source` in the enclosing support manifest.
- Downgrade from v2 to v1 is forbidden. Consumers that require raw refs must
  reopen the local review record under its original authority boundary rather
  than reconstructing them from support data.

The canonical fixture is
[`support_export_redacted_v2.json`](../../../fixtures/git/mutation_review_alpha/support_export_redacted_v2.json).
