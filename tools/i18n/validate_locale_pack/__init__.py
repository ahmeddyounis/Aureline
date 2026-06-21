"""Contribution-time validator for first-party, community, and extension locale packs.

The package validates an on-disk locale pack (an authoring manifest plus its
strings and optional glossary files) against the same stable-id, compatibility,
coverage, and host-stable-label rules every Aureline locale pack must satisfy,
so a contributor catches an incompatible pack or a forbidden label replacement
before it ever reaches runtime.

The public entry point is :func:`validate_locale_pack`, which returns a
deterministic list of :class:`Finding` records.
"""

from .validator import (
    Finding,
    GlossaryError,
    RegistryError,
    load_terminology_glossary,
    load_message_registry,
    render_human_summary,
    validate_locale_pack,
    validate_terminology_glossary,
)

__all__ = [
    "Finding",
    "GlossaryError",
    "RegistryError",
    "load_terminology_glossary",
    "load_message_registry",
    "render_human_summary",
    "validate_locale_pack",
    "validate_terminology_glossary",
]
