"""Core validation logic for locale-pack contributions.

The validator is intentionally dependency-free (standard library only) and
deterministic: contributors run it without compiling the workspace, and its
findings sort into a stable order so a captured report never drifts.

Three checked-in artifacts are the source of truth the validator reads:

* the stable message-id registry
  (``fixtures/i18n/message-id-stability/registry.json``) — the host message ids
  a first-party or community pack may translate;
* the terminology governance glossary
  (``fixtures/i18n/locale-pack-contribution/terminology_glossary.json``) — the
  host-stable-locked vocabulary a pack may render but never replace, and the
  review-governed vocabulary a pack may translate; and
* the pack's own authoring manifest, strings, and optional glossary on disk.
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

# Default repo-relative source-of-truth paths.
DEFAULT_REGISTRY_REL = "fixtures/i18n/message-id-stability/registry.json"
DEFAULT_GLOSSARY_REL = "fixtures/i18n/locale-pack-contribution/terminology_glossary.json"

# A locale tag embedded inside a message id silently breaks continuity across
# locale changes (e.g. ``msg:shell:title:es-MX``). Detected per id segment.
_LOCALE_TAG = re.compile(r"^[a-z]{2,3}-[A-Za-z]{2}$")

# Host message ids live under this prefix; contributed (extension/companion)
# packs must not write into it.
HOST_MESSAGE_ID_PREFIX = "msg:"

PACK_OWNER_CLASSES = (
    "first_party_pack",
    "community_pack",
    "extension_owned_pack",
    "companion_overlay_pack",
)

# Owner classes that translate host message ids rather than their own namespace.
HOST_TRANSLATING_OWNER_CLASSES = ("first_party_pack", "community_pack")

# Owner classes that own a private namespace and must declare a prefix.
CONTRIBUTED_OWNER_CLASSES = ("extension_owned_pack", "companion_overlay_pack")

POLICY_RECOVERY_SURFACE = "policy_legal_or_recovery_text"


class RegistryError(Exception):
    """The stable message-id registry could not be loaded."""


class GlossaryError(Exception):
    """The terminology governance glossary could not be loaded."""


@dataclass(frozen=True)
class Finding:
    """One validation result.

    ``severity`` is ``error`` for a release-blocking violation or ``warning``
    for a disclosed-but-allowed posture worth surfacing. ``code`` is a stable,
    locale-neutral finding code; ``location`` is the file or key the finding
    points at within the pack.
    """

    severity: str
    code: str
    location: str
    message: str

    def sort_key(self) -> tuple[str, str, str, str]:
        # errors before warnings, then by code/location/message for stability.
        severity_rank = "0" if self.severity == "error" else "1"
        return (severity_rank, self.code, self.location, self.message)


# --------------------------------------------------------------------------- #
# Loading the sources of truth
# --------------------------------------------------------------------------- #


def _read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def load_message_registry(path: Path) -> dict[str, Any]:
    """Loads the stable message-id registry packet."""
    if not path.exists():
        raise RegistryError(f"message-id registry not found: {path}")
    try:
        data = _read_json(path)
    except json.JSONDecodeError as exc:  # pragma: no cover - defensive
        raise RegistryError(f"message-id registry is not valid JSON: {exc}") from exc
    if not isinstance(data, dict) or not isinstance(data.get("entries"), list):
        raise RegistryError("message-id registry is missing an 'entries' array")
    return data


def load_terminology_glossary(path: Path) -> dict[str, Any]:
    """Loads the terminology governance glossary."""
    if not path.exists():
        raise GlossaryError(f"terminology glossary not found: {path}")
    try:
        data = _read_json(path)
    except json.JSONDecodeError as exc:  # pragma: no cover - defensive
        raise GlossaryError(f"terminology glossary is not valid JSON: {exc}") from exc
    if not isinstance(data, dict) or not isinstance(data.get("terms"), list):
        raise GlossaryError("terminology glossary is missing a 'terms' array")
    return data


# --------------------------------------------------------------------------- #
# Glossary self-validation (used by the gate)
# --------------------------------------------------------------------------- #


def validate_terminology_glossary(glossary: dict[str, Any]) -> list[Finding]:
    """Validates the governance glossary's own structural invariants."""
    findings: list[Finding] = []
    loc = glossary.get("glossary_id", "terminology_glossary")

    if glossary.get("record_kind") != "locale_pack_terminology_glossary":
        findings.append(Finding("error", "glossary.record_kind", loc, "record_kind is not canonical"))
    if glossary.get("schema_version") != 1:
        findings.append(Finding("error", "glossary.schema_version", loc, "schema_version is not 1"))

    prefixes = glossary.get("host_stable_namespace_prefixes") or []
    domains = set(glossary.get("domains") or [])
    seen_keys: set[str] = set()
    for term in glossary.get("terms") or []:
        key = term.get("term_key", "<missing>")
        if key in seen_keys:
            findings.append(Finding("error", "glossary.duplicate_term", key, "term_key is repeated"))
        seen_keys.add(key)
        if term.get("domain") not in domains:
            findings.append(Finding("error", "glossary.unknown_domain", key, "term domain is not declared"))
        gov = term.get("governance_class")
        if gov not in ("host_stable_locked", "translatable_with_review"):
            findings.append(Finding("error", "glossary.bad_governance_class", key, "governance_class is invalid"))
        if gov == "host_stable_locked":
            if not term.get("forbidden_to_replace"):
                findings.append(
                    Finding("error", "glossary.locked_must_forbid_replace", key,
                            "host-stable-locked term must set forbidden_to_replace=true"))
            if not term.get("host_catalog_ref"):
                findings.append(
                    Finding("error", "glossary.locked_needs_catalog", key,
                            "host-stable-locked term must name a host_catalog_ref"))
            if not _has_prefix(key, prefixes):
                findings.append(
                    Finding("error", "glossary.locked_needs_reserved_prefix", key,
                            "host-stable-locked term_key must sit under a reserved host prefix"))
        else:
            if _has_prefix(key, prefixes):
                findings.append(
                    Finding("error", "glossary.translatable_under_reserved_prefix", key,
                            "review-governed term_key must not sit under a reserved host prefix"))
    return _sorted(findings)


# --------------------------------------------------------------------------- #
# Pack validation
# --------------------------------------------------------------------------- #


def validate_locale_pack(
    pack_dir: Path,
    *,
    registry: dict[str, Any],
    glossary: dict[str, Any],
) -> list[Finding]:
    """Validates one locale pack directory and returns sorted findings."""
    findings: list[Finding] = []

    manifest_path = pack_dir / "manifest.json"
    if not manifest_path.exists():
        return [Finding("error", "manifest.missing", "manifest.json", "pack has no manifest.json")]
    try:
        manifest = _read_json(manifest_path)
    except json.JSONDecodeError as exc:
        return [Finding("error", "manifest.invalid_json", "manifest.json", f"manifest is not valid JSON: {exc}")]
    if not isinstance(manifest, dict):
        return [Finding("error", "manifest.invalid_json", "manifest.json", "manifest must be a JSON object")]

    glossary_index = _index_glossary(glossary)
    registry_index = _index_registry(registry)

    findings.extend(_validate_manifest_shape(manifest, glossary_index))
    owner_class = manifest.get("owner_class")
    owned_prefix = manifest.get("owned_namespace_prefix")

    # Collect strings across declared strings files.
    all_keys: dict[str, str] = {}
    for ref in manifest.get("strings_refs") or []:
        strings_path = pack_dir / ref
        if not strings_path.exists():
            findings.append(Finding("error", "strings.missing_file", ref, "strings file does not exist"))
            continue
        try:
            strings = _read_json(strings_path)
        except json.JSONDecodeError as exc:
            findings.append(Finding("error", "strings.invalid_json", ref, f"strings file is not valid JSON: {exc}"))
            continue
        if not isinstance(strings, dict):
            findings.append(Finding("error", "strings.invalid_json", ref, "strings file must be a JSON object"))
            continue
        findings.extend(
            _validate_strings_file(ref, strings, owner_class, owned_prefix, glossary_index, registry_index)
        )
        for key, value in strings.items():
            all_keys[key] = ref

    findings.extend(
        _validate_coverage(manifest, all_keys, owner_class, registry_index)
    )

    glossary_ref = manifest.get("glossary_ref")
    if glossary_ref:
        findings.extend(_validate_pack_glossary(pack_dir, glossary_ref, glossary_index))

    return _sorted(findings)


def _validate_manifest_shape(manifest: dict[str, Any], glossary_index: "_GlossaryIndex") -> list[Finding]:
    findings: list[Finding] = []
    loc = manifest.get("pack_id", "manifest.json")

    if manifest.get("record_kind") != "locale_pack_authoring_manifest":
        findings.append(Finding("error", "manifest.record_kind", loc, "record_kind is not canonical"))
    if manifest.get("schema_version") != 1:
        findings.append(Finding("error", "manifest.schema_version", loc, "schema_version is not 1"))

    for field in ("pack_id", "owner_id", "locale", "source_language_locale", "fallback_locale", "presentation_label"):
        if not manifest.get(field):
            findings.append(Finding("error", "manifest.field_missing", loc, f"required field '{field}' is missing"))

    owner_class = manifest.get("owner_class")
    if owner_class not in PACK_OWNER_CLASSES:
        findings.append(Finding("error", "manifest.bad_owner_class", loc, f"owner_class '{owner_class}' is not recognized"))

    # Host-stable label protection at the manifest level.
    if manifest.get("may_override_host_stable_labels") is True:
        findings.append(
            Finding("error", "manifest.override_host_stable_labels", loc,
                    "may_override_host_stable_labels must be false; host-stable labels are host-controlled"))
    owned_surfaces = manifest.get("owned_surface_families") or []
    if POLICY_RECOVERY_SURFACE in owned_surfaces:
        findings.append(
            Finding("error", "manifest.owns_policy_recovery_text", loc,
                    "a contributed pack must not own policy, legal, or recovery text"))

    # Namespace posture per owner class.
    owned_prefix = manifest.get("owned_namespace_prefix")
    if owner_class in CONTRIBUTED_OWNER_CLASSES:
        if not owned_prefix:
            findings.append(
                Finding("error", "manifest.extension_namespace_required", loc,
                        "extension-owned and companion packs must declare an owned_namespace_prefix"))
        else:
            for reserved in glossary_index.reserved_prefixes:
                if owned_prefix.startswith(reserved) or reserved.startswith(owned_prefix):
                    findings.append(
                        Finding("error", "manifest.namespace_collides_host", loc,
                                f"owned_namespace_prefix '{owned_prefix}' collides with reserved host prefix '{reserved}'"))
            if owned_prefix.startswith(HOST_MESSAGE_ID_PREFIX):
                findings.append(
                    Finding("error", "manifest.namespace_collides_host", loc,
                            "owned_namespace_prefix must not claim the host 'msg:' namespace"))

    # Fallback chain shape.
    locale = manifest.get("locale")
    source = manifest.get("source_language_locale")
    chain = manifest.get("fallback_chain") or []
    if chain:
        if chain[0] != locale:
            findings.append(Finding("error", "manifest.fallback_chain_bad", loc, "fallback_chain must start at the pack locale"))
        if chain[-1] != source:
            findings.append(Finding("error", "manifest.fallback_chain_bad", loc, "fallback_chain must end at the source language"))
    if manifest.get("discloses_source_language_fallback") is not True:
        findings.append(
            Finding("error", "manifest.fallback_not_disclosed", loc,
                    "discloses_source_language_fallback must be true; untranslated keys fall back to source language"))

    # Compatibility build range.
    findings.extend(_validate_compat_range(manifest.get("compatibility_build_range"), loc))

    return findings


def _validate_compat_range(rng: Any, loc: str) -> list[Finding]:
    findings: list[Finding] = []
    if not isinstance(rng, dict):
        return [Finding("error", "manifest.compat_range_missing", loc, "compatibility_build_range is missing")]
    lo = rng.get("min_build_identity_ref")
    hi = rng.get("max_build_identity_ref")
    if not lo or not hi:
        return [Finding("error", "manifest.compat_range_malformed", loc,
                        "compatibility_build_range must declare min and max build identities")]
    if _build_sort_key(lo) > _build_sort_key(hi):
        findings.append(
            Finding("error", "manifest.compat_range_inverted", loc,
                    f"compatibility_build_range min '{lo}' is greater than max '{hi}'"))
    return findings


def _validate_strings_file(
    ref: str,
    strings: dict[str, Any],
    owner_class: str | None,
    owned_prefix: str | None,
    glossary_index: "_GlossaryIndex",
    registry_index: "_RegistryIndex",
) -> list[Finding]:
    findings: list[Finding] = []
    for key, value in strings.items():
        where = f"{ref}#{key}"
        if not isinstance(value, str) or not value.strip():
            findings.append(Finding("error", "strings.empty_value", where, "message value must be a non-empty string"))
        if _carries_locale_tag(key):
            findings.append(Finding("error", "strings.id_carries_locale_tag", where, "message id carries a locale tag"))

        # Host-stable label / forbidden-term protection (applies to every owner class).
        if _has_prefix(key, glossary_index.reserved_prefixes):
            findings.append(
                Finding("error", "strings.host_stable_namespace_replacement", where,
                        "key replaces a host-stable label under a reserved host namespace"))
            continue
        if key in glossary_index.forbidden_term_keys:
            findings.append(
                Finding("error", "strings.forbidden_term_replacement", where,
                        "key replaces a host-stable-locked governed term"))
            continue

        # Stable-id discipline per owner class.
        if owner_class in HOST_TRANSLATING_OWNER_CLASSES:
            if not key.startswith(HOST_MESSAGE_ID_PREFIX):
                findings.append(
                    Finding("error", "strings.id_outside_host_namespace", where,
                            "first-party/community packs translate host 'msg:' message ids"))
            elif key not in registry_index.message_ids:
                findings.append(
                    Finding("error", "strings.unknown_host_id", where,
                            "message id is not a known stable host id (would fork the stable id set)"))
        elif owner_class in CONTRIBUTED_OWNER_CLASSES:
            if key.startswith(HOST_MESSAGE_ID_PREFIX) or key in registry_index.message_ids:
                findings.append(
                    Finding("error", "strings.contributed_owns_host_id", where,
                            "contributed packs must not redefine host message ids"))
            elif owned_prefix and not key.startswith(owned_prefix):
                findings.append(
                    Finding("error", "strings.id_outside_owned_namespace", where,
                            f"key must sit under the pack's owned namespace prefix '{owned_prefix}'"))
    return findings


def _validate_coverage(
    manifest: dict[str, Any],
    all_keys: dict[str, str],
    owner_class: str | None,
    registry_index: "_RegistryIndex",
) -> list[Finding]:
    findings: list[Finding] = []
    loc = manifest.get("pack_id", "manifest.json")
    if owner_class not in HOST_TRANSLATING_OWNER_CLASSES:
        return findings  # contributed packs own their keys; no host coverage baseline.

    owned_surfaces = manifest.get("owned_surface_families") or []
    expected: set[str] = set()
    for surface in owned_surfaces:
        expected |= registry_index.ids_by_surface.get(surface, set())
    present = set(all_keys)
    missing = sorted(expected - present)

    if manifest.get("claims_complete_coverage") is True and missing:
        findings.append(
            Finding("error", "coverage.incomplete_but_claimed_complete", loc,
                    f"claims_complete_coverage is true but {len(missing)} host id(s) are untranslated: "
                    + ", ".join(missing[:5]) + ("…" if len(missing) > 5 else "")))
    elif missing:
        findings.append(
            Finding("warning", "coverage.missing_keys", loc,
                    f"{len(missing)} host id(s) for owned surfaces are untranslated and will fall back to source language"))
    return findings


def _validate_pack_glossary(pack_dir: Path, glossary_ref: str, glossary_index: "_GlossaryIndex") -> list[Finding]:
    findings: list[Finding] = []
    path = pack_dir / glossary_ref
    if not path.exists():
        return [Finding("error", "glossary.missing_file", glossary_ref, "declared glossary file does not exist")]
    try:
        pack_glossary = _read_json(path)
    except json.JSONDecodeError as exc:
        return [Finding("error", "glossary.invalid_json", glossary_ref, f"glossary file is not valid JSON: {exc}")]
    if not isinstance(pack_glossary, dict):
        return [Finding("error", "glossary.invalid_json", glossary_ref, "glossary file must be a JSON object")]

    for term_key, value in pack_glossary.items():
        where = f"{glossary_ref}#{term_key}"
        if not isinstance(value, str) or not value.strip():
            findings.append(Finding("error", "glossary.empty_value", where, "glossary value must be a non-empty string"))
        if term_key not in glossary_index.all_term_keys:
            findings.append(
                Finding("error", "glossary.unknown_term", where,
                        "glossary localizes a term that is not in the governance glossary"))
            continue
        if term_key in glossary_index.forbidden_term_keys:
            findings.append(
                Finding("error", "glossary.translates_host_stable_locked", where,
                        "glossary localizes a host-stable-locked term that must be rendered from the host catalog"))
    return findings


# --------------------------------------------------------------------------- #
# Indexes and helpers
# --------------------------------------------------------------------------- #


@dataclass(frozen=True)
class _GlossaryIndex:
    reserved_prefixes: tuple[str, ...]
    forbidden_term_keys: frozenset[str]
    all_term_keys: frozenset[str]


@dataclass(frozen=True)
class _RegistryIndex:
    message_ids: frozenset[str]
    ids_by_surface: dict[str, set[str]]


def _index_glossary(glossary: dict[str, Any]) -> _GlossaryIndex:
    prefixes = tuple(glossary.get("host_stable_namespace_prefixes") or ())
    forbidden = set()
    all_keys = set()
    for term in glossary.get("terms") or []:
        key = term.get("term_key")
        if not key:
            continue
        all_keys.add(key)
        if term.get("forbidden_to_replace"):
            forbidden.add(key)
    return _GlossaryIndex(prefixes, frozenset(forbidden), frozenset(all_keys))


def _index_registry(registry: dict[str, Any]) -> _RegistryIndex:
    ids: set[str] = set()
    by_surface: dict[str, set[str]] = {}
    for entry in registry.get("entries") or []:
        mid = entry.get("message_id")
        if not mid:
            continue
        ids.add(mid)
        surface = entry.get("surface_family")
        if surface:
            by_surface.setdefault(surface, set()).add(mid)
    return _RegistryIndex(frozenset(ids), by_surface)


def _has_prefix(value: str, prefixes: Iterable[str]) -> bool:
    return any(value.startswith(p) for p in prefixes)


def _carries_locale_tag(value: str) -> bool:
    return any(_LOCALE_TAG.match(segment) for segment in re.split(r"[:./]", value))


def _build_sort_key(ref: str) -> tuple:
    # Split into alternating digit/non-digit runs so date-stamped build refs
    # compare numerically without assuming a fixed format.
    parts = re.findall(r"\d+|\D+", ref)
    return tuple((1, int(p)) if p.isdigit() else (0, p) for p in parts)


def _sorted(findings: list[Finding]) -> list[Finding]:
    return sorted(findings, key=Finding.sort_key)


def render_human_summary(findings: list[Finding], *, header: str | None = None) -> str:
    lines: list[str] = []
    if header:
        lines.append(header)
    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    for finding in findings:
        marker = "ERROR" if finding.severity == "error" else "warn "
        lines.append(f"  [{marker}] {finding.code}: {finding.location}: {finding.message}")
    lines.append(f"  -> {len(errors)} error(s), {len(warnings)} warning(s)")
    return "\n".join(lines) + "\n"
