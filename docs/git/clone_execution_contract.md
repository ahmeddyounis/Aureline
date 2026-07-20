<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

# Reviewed Git clone execution contract

This contract owns the subprocess and filesystem boundary that materializes a
Git source after the clone-review surface has resolved every disclosure axis.
It composes with
[`clone_review_contract.md`](../ux/clone_review_contract.md),
[`source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md),
[`repository_acquisition_beta.md`](../workspace/m3/repository_acquisition_beta.md),
[`transport_permission_matrix.md`](../network/transport_permission_matrix.md),
and [`git_service_alpha.md`](./git_service_alpha.md). The review contract owns
what the user sees; this document owns how approved values reach Git.

## 1. Authority and API boundary

`CloneRequest` is preview-only input. It may be validated, normalized, and
serialized, but it MUST NOT start a process or write a destination. Execution
consumes one non-cloneable `ApprovedCloneExecution` minted only after all of
these bindings validate:

- clone-review, source-locator, checkout-plan, and policy-decision refs;
- a `vcs_network` transport-decision ref for every network-bearing source;
- the reviewed transport and credential-free normalized locator, compared
  exactly with the current request, including the canonical local source
  identity for local acquisition;
- the destination presentation path, canonical parent, parent filesystem
  identity, and separately reviewed final path component;
- exact ref plus reviewed full commit OID;
- full or shallow history depth and explicit false values for partial clone,
  sparse checkout, submodule recursion, and LFS hydration;
- anonymous auth or a current, target-bound SSH-agent authority projection;
- reviewed proxy and CA inputs, execution deadlines, and post-clone action.

`CloneApproval` is a serializable evidence record, not process authority. Its
reviewed transport, normalized source, canonical destination parent, final
path component, and local-source binding are all revalidated when `approve`
mints the execution value. The execution value is passed by value and cannot
be serialized or cloned. This is an in-process misuse barrier, not a
substitute for source-acquisition, transport-policy, or secret-broker
validation. A caller migrating from the legacy API must call `review_facts`,
emit and approve the owning records, copy those exact facts into
`CloneApproval`, call `approve`, retain the cancellation token if needed, and
move the returned execution into `clone_repository`. Passing `CloneRequest`
directly to `clone_repository` is intentionally no longer source-compatible.

## 2. Locator grammar

The execution allowlist is closed:

| Form | Transport | Requirements |
| --- | --- | --- |
| `https://host/path` | `https` | no userinfo, query, fragment, redirect, or ambient credential helper |
| `ssh://`, `git+ssh://`, `ssh+git://` | `ssh` | normalized ASCII host, optional user, strict reviewed SSH projection |
| `user@host:path` and `host:path` | `ssh` | balanced brackets, non-empty host/path, strict reviewed SSH projection |
| `git://host/path` | `git_protocol` | explicit reviewed transport decision; anonymous only |
| absolute local path | `local_filesystem` | canonical source path and filesystem identity bound at approval |
| `file:///absolute/path` or `file://localhost/absolute/path` | `local_filesystem` | no percent escaping; canonical local binding |

`http://`, hosted `file://host/...`, relative local paths, external remote
helpers, embedded credentials, malformed IPv6/port/user authorities, non-ASCII
hosts that have not already been normalized to IDNA, and query or fragment data
are rejected. Non-UTF-8 local sources require the future typed-filesystem
locator; they cannot be smuggled through a lossy string.
On Windows, local paths and `file:` URLs must resolve from an ordinary drive
prefix. UNC, device-namespace, and verbatim paths are network/filesystem
authority surfaces and remain blocked until they have their own reviewed
acquisition class; they cannot bypass transport review as `local_filesystem`.

## 3. Ref and topology binding

Approval requires a valid Git ref label and a full 40- or 64-hex-character
commit OID. Clone fetches the reviewed ref with `--no-checkout`, verifies
`HEAD^{commit}` against the OID, performs the inert checkout, and verifies the
OID again. Any mismatch is `ref_mismatch`; bytes remain an interrupted partial
and are never handed to trust admission as a successful clone.
The acquisition checkout is detached at the reviewed OID; attaching or moving
a local branch is a later admitted Git action.

Full history and positive shallow depths are implemented. Partial clone,
sparse checkout, submodule recursion, and LFS hydration fail approval and must
be separate reviewed bootstrap actions.

Local sources use Git's non-local transport path plus `--no-hardlinks`; a
successful destination may not contain `objects/info/alternates` or
`http-alternates`. The acquired repository therefore cannot silently borrow
mutable objects from the source or another ambient object store.

## 4. Environment, credentials, and transport

Every Git subprocess starts from `env_clear`. The executor projects only:

- the pinned Git directory and, for SSH, pinned SSH directory into `PATH`;
- `LC_ALL=C` and `LANG=C` for stable classification;
- a private empty `HOME`, XDG config root, system/global Git config, hook
  directory, template directory, and attributes file;
- prompt-, askpass-, pager-, editor-, LFS-smudge-, optional-lock-, and
  submodule-disabling values;
- explicit reviewed proxy/CA values and, only for SSH, the reviewed agent
  socket, known-hosts file, SSH executable, and current authority ticket.

Ambient Git config, URL rewrites, proxy variables, TLS bypass variables,
credential helpers, askpass programs, SSH config, SSH agents, trace outputs,
filters, hooks, templates, and alternate object stores are not inherited.
`http.sslVerify=true`, redirects disabled, credential helpers empty, protocol
defaults denied, and only the reviewed source protocol enabled are pinned on
the command line.

SSH uses batch mode, no TTY, no password, keyboard-interactive, GSSAPI, or
host-based auth, no forwarding or local command, strict host-key checking, the
reviewed known-hosts file, and zero password prompts. Private HTTPS is blocked
until the secret broker supplies a narrow ticket-bound credential projection;
the executor does not fall back to the user's ambient credential helper.
On Unix the admitted agent endpoint must be an actual socket. Windows named
pipe agent projection remains blocked until the platform authority boundary can
bind pipe identity without ambient lookup.

## 5. Process supervision and output

Git 2.30.0 is the minimum supported acquisition version. The executable is
canonicalized and its filesystem identity is checked before every phase.
Stdin is null. Stdout/stderr are drained asynchronously through a bounded
channel. Captured stdout is limited to 4 KiB, progress lines to 4 KiB, progress
events to 128, and public messages to 256 bytes. Stderr is reduced to fixed
classification flags and fixed progress labels; raw diagnostics, URLs, paths,
and secrets never cross the result boundary.

Every phase is subject to the reviewed idle deadline and to the remaining
reviewed operation-wide deadline. Cancellation, idle timeout, overall timeout,
output overflow, read failure, and a descendant retaining an output pipe all
kill and reap the child. Unix commands run in a fresh process group and the
supervisor attempts group termination before direct-child termination. The
child lifetime is unwind-guarded, and a progress-observer failure is converted
to a fixed typed error so useful partial bytes follow the normal recovery path
instead of being orphaned or silently discarded. The
standard library has no portable Windows Job Object API: Windows guarantees
direct-child kill/reap, while descendant-tree termination remains a declared
residual until the platform process-host boundary supplies a Job Object.

## 6. Destination ownership and recovery

Approval binds the canonical parent and its filesystem identity. Execution
rechecks that identity, creates the final directory atomically, captures the
new directory identity, restricts permissions while acquiring, and rechecks
identity between phases. The newly created `.git` directory receives its own
identity binding before verification or checkout. Existing files, directories,
symlinks, and junctions are collisions; no in-place clone or overwrite path
exists.

Unix identity uses device/inode pairs. Stable Rust 1.75 does not expose Windows
volume/file-index identity or handle-relative recursive removal, so the Windows
fallback binds canonical paths and rejects visible reparse-point destinations.
Junction/file-ID hardening remains a declared platform residual; Windows
qualification must fail closed for paths whose canonical/reparse posture cannot
be proven stable.

Before recursive deletion, a known owned directory is renamed to a fresh
randomized sibling quarantine path and its identity is verified again. Unknown
or swapped objects are never recursively deleted. The same rule applies to the
randomized private guard directory and its cleanup, which must complete before
success is emitted.

Portable `std` does not provide handle-relative recursive deletion. The final
identity check after quarantine is therefore fail-closed, but replacement by a
same-user actor after that check remains a declared residual on every platform;
qualification must keep the quarantine parent private from less-trusted actors.

Failures before useful `.git` state exists automatically discard the freshly
owned destination. Once `.git` contains acquisition state, failure returns a
unique `ClonePartialAcquisition` and one of:

- `interrupted_resumable` during fetch or verification;
- `interrupted_open_read_only_available` after worktree materialization;
- `interrupted_discard_required` when automatic cleanup cannot be proven safe;
- `no_partial_bytes` only after verified cleanup or before destination creation.

Partial state is preserved by default. `discard` consumes its unique handle
and performs quarantine-first identity-safe cleanup. Resume and read-only open
remain separately reviewed source-acquisition actions; clone execution never
implicitly retries, trusts, opens, installs, restores, or runs repository code.

## 7. Required verification

Focused tests must cover the closed locator grammar, review-binding mismatch,
exact-OID drift, hostile ambient environment/config, inert filters/LFS/hooks,
strict SSH construction, overall/idle timeout, cancellation, output bounds,
descendant pipe retention, pre-useful cleanup, meaningful-partial preservation,
quarantine identity mismatch, checkout/read-only recovery, symlink and parent
identity drift, Git version floor, redacted `Debug`, and a real local clone.
Windows qualification additionally covers junction/reparse-point collision and
documents the process-tree and file-ID residuals above.
