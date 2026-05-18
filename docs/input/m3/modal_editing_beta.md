# Modal Editing Beta Contract

Modal editing is product state, not a private preset detail. Every claimed
keyboard profile must expose the current mode, pending sequence, register route,
surface fidelity, and replay scope before a keypress can change meaning in a
surprising way.

## Records

- `schemas/input/modal_state.schema.json` defines the focused modal snapshot:
  mode strip, inline sequence guide, register route inspector,
  operator-pending overlay, surface-fidelity labels, and recovery actions.
- `schemas/input/macro_replay_preview.schema.json` defines the macro replay
  review sheet: source macro register, target scope, stable command ids, write
  classes, route changes, risks, and replay decision.
- `crates/aureline-input::modal` is the Rust source of truth for these records
  and validators.

## Surface Fidelity

Supported source-editor surfaces can show full modal behavior. Restricted,
browser companion, large-file, and remote clipboard-limited surfaces must label
the narrowed behavior before dispatch. Unsupported imported sequences must fail
closed with a diagnostic and a keyboard recovery action such as palette search,
keymap diagnostics, migration help, or retry on a supported editor surface.

## Register Routing

Register and clipboard rows distinguish:

- editor-local registers
- local system clipboard routes
- remote clipboard bridge routes
- named registers
- search-history registers
- macro registers
- blocked or unsupported policy routes

Rows that change result or risk must include diagnostics. Remote clipboard
suppression and admin blocks are visible before yank, paste, or macro replay.

## Macro Replay

Macro replay may proceed silently only when it remains editor-local, single
buffer, policy-allowed, and free of route changes or unsafe command steps.
Replay that crosses files, mutates settings, calls a run-capable command,
depends on unsupported imported sequences, or changes clipboard/register routing
must require review, promotion to a recipe, downgrade, or denial.

Fixtures under `fixtures/input/m3/modal_register_and_macro_safety/` cover the
claimed routes and replay decisions without raw key history, clipboard contents,
register contents, macro bodies, or edited text.
