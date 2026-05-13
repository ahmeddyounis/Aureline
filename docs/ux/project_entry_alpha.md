# Project Entry Alpha Review Path

This note describes the checked-in alpha implementation for reviewed project entry.
The runtime contract lives in `aureline-workspace::admission`, and the first shell
consumer lives in `aureline-shell::start_center::admission_review`.

## Contract

Every `Open`, `Clone`, `Import`, `Add root`, `Restore`, and drag/drop entry resolves
to one admission review packet before the shell may write bytes, grant trust, run
setup, mutate the current workspace, or hide temporary materialization.

The packet always includes:

- normalized target identity
- resulting mode
- reviewed destination
- write-scope class and proposed items
- recovery path
- explicit non-actions such as no trust grant, no setup, and no task or hook run

Clone admission also records host/certificate posture, auth mode, ref posture,
submodule/LFS posture, proxy or mirror note, explicit clone actions, and destination
collision choices. Import admission records inspect-only versus extraction/restore,
artifact class, exclusions, cleanup posture, and staging truth. Drag/drop admission
uses the same packet and adds payload kind, advertised verb, progress/cancel posture,
and checkpoint or undo group for durable mutations.

## Proof Fixtures

Protected fixtures live in `fixtures/ux/project_entry/`:

- `clone_remote_destination_review.json`
- `import_archive_inspect_review.json`
- `dragdrop_large_archive_collision_review.json`

The workspace crate test suite deserializes these packets and asserts the required
trust/setup, recovery, and write-scope invariants.

## Verification

Run:

```sh
cargo test -p aureline-workspace admission
cargo test -p aureline-shell start_center::admission_review
```
