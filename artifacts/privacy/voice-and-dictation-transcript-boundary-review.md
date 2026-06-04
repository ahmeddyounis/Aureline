# Voice and Dictation Transcript Boundary Review

Voice and dictation evidence is metadata-first. Stable rows may record mode,
processing class, retention posture, fallback state, canonical command ID,
approval posture, undo group, packet ID, and evidence refs.

Raw transcripts and correction buffers are sensitive input. They are ephemeral or
bounded by default, require explicit delete/export controls, and are excluded
from support and diagnostics exports unless a reviewed flow opts in.
