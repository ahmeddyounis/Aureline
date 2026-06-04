# Voice and Dictation Surface Qualification

Aureline labels voice capture by mode and processing class. Command mode runs
commands; Dictation mode inserts text. Push-to-talk or explicit activation is the
default stable posture.

Voice support is unavailable when microphone access, policy, offline state,
provider availability, or noisy conditions prevent safe capture. In those states,
the UI must name the reason and keep keyboard fallback visible.

Support exports include metadata such as mode, provider class, fallback reason,
and command lineage. Raw transcripts are excluded by default.
