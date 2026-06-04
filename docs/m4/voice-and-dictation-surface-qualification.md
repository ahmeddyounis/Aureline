# Voice and Dictation Surface Qualification

The release packet at
`artifacts/release/m4/voice-and-dictation-surface-qualification.json` is the
source of truth for voice and dictation labels.

Stable speech support is limited to explicit command/dictation mode surfaces,
push-to-talk or explicit activation defaults, visible provider/privacy state,
bounded transcript handling, first-class unavailable fallbacks, and command
graph parity for spoken actions.

Third-party provider routing, wake-word behavior, and background listening remain
Preview until their own current packet proves opt-in, persistent indicators,
privacy review, and command parity on every claimed platform.

Verification:

```sh
cargo test -p aureline-release --test voice_and_dictation_surface_qualification
```
