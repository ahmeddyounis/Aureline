# M5 Badge Vocabulary And Explanation Drawers

- Packet: `m5-badge-vocabulary:stable:0001`
- Label: `M5 badge vocabulary and explanation drawers`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Badges: 44 across 4 families and 8 dimensions
- Tone: 8 authoritative, 2 informational, 26 caution, 8 blocking
- Claim effect: 26 narrow, 8 block
- Required terms covered: 13/13
- Rendered by: release center, Help/About, marketplace, docs/help, support, companion

## Provenance / source-origin badges

| Badge id | Label | Tone | Claim effect | Explanation drawer |
|----------|-------|------|--------------|--------------------|
| `source_origin.first_party_signed` | Official | `authoritative` | `none` | This artifact is published by the project's own release identity and carries a verified first-party signature. It is the only origin that can carry an unqualified Stable claim. |
| `source_origin.vendor` | Vendor | `caution` | `narrows` | A known vendor or partner authored this under a governed agreement. The origin is accountable but is not first-party, so a claim built on it narrows below Stable until first-party evidence is present. |
| `source_origin.community` | Community | `caution` | `narrows` | The community contributed and reviewed this artifact. It is legitimate but not first-party, so a claim built on it narrows below Stable. |
| `source_origin.mirror` | Mirrored | `caution` | `narrows` | This came from a mirror copy rather than the first-party channel. The mirror's freshness stays inspectable without reaching vendor services, but a mirrored origin narrows the claim until first-party provenance is confirmed. |
| `source_origin.offline_bundle` | Offline bundle | `caution` | `narrows` | This was installed from an offline bundle. Its origin is recorded but not live-verified against the channel, so the claim narrows until the origin can be reverified online. |
| `source_origin.side_loaded` | Side-loaded | `caution` | `narrows` | This was side-loaded outside the governed channel. The origin is shown rather than hidden, but a side-loaded artifact cannot carry a Stable claim until it is reconciled with the governed channel. |
| `source_origin.not_provided` | Not provided | `blocking` | `blocks` | No origin evidence was provided for this artifact. A missing origin is recorded explicitly — never left blank — and blocks any Stable claim until provenance is supplied. |
| `signature_state.signed_attested` | Signature verified | `authoritative` | `none` | The artifact's signature was checked against the release identity and a build attestation is present and valid. This is the strongest signature state. |
| `signature_state.signed_unverified` | Signature unverified | `caution` | `narrows` | A signature is present but could not be verified against a trusted key in the current context. The claim narrows until the signature verifies. |
| `signature_state.attestation_only` | Attestation available | `informational` | `none` | A build attestation is available describing how this artifact was produced. Attestation is positive evidence but does not by itself substitute for a verified signature. |
| `signature_state.unsigned` | Unsigned | `caution` | `narrows` | This artifact carries no signature. An unsigned artifact narrows the claim until signing evidence is added. |
| `signature_state.signature_invalid` | Signature invalid | `blocking` | `blocks` | A signature is present but failed verification — the bytes do not match the claimed identity. An invalid signature blocks any Stable claim and is surfaced rather than ignored. |
| `signature_state.not_provided` | Signature not provided | `caution` | `narrows` | No signature evidence was provided for this artifact. The absence is recorded explicitly and narrows the claim until signature state is supplied. |

## Evidence-freshness badges

| Badge id | Label | Tone | Claim effect | Explanation drawer |
|----------|-------|------|--------------|--------------------|
| `freshness_state.current` | Evidence current | `authoritative` | `none` | The evidence behind this claim is within its freshness window. The claim stands at its full class. |
| `freshness_state.stale` | Evidence aging | `caution` | `narrows` | The evidence has fallen outside its freshness window. Stale evidence automatically narrows the claim below Stable until it is refreshed. |
| `freshness_state.expired` | Evidence expired | `blocking` | `blocks` | The evidence has passed its hard expiry. Expired evidence blocks a Stable claim until it is renewed. |
| `freshness_state.missing` | Evidence missing | `blocking` | `blocks` | No usable evidence exists for this claim. A missing-evidence state blocks a Stable claim and is recorded explicitly rather than omitted. |
| `evidence_state.complete` | Complete | `authoritative` | `none` | The evidence covers the full claimed scope. Nothing in the claimed matrix is left unverified. |
| `evidence_state.limited` | Limited evidence | `caution` | `narrows` | The evidence covers a narrower scope than the claim. The claim narrows to what the evidence actually supports. |
| `evidence_state.partial` | Partial | `caution` | `narrows` | Only part of the claimed scope is backed by evidence. The unverified remainder is named rather than implied, and the claim narrows accordingly. |
| `evidence_state.retest_pending` | Retest pending | `caution` | `narrows` | The evidence is awaiting re-verification — a retest is queued or in progress. The claim narrows until the retest completes. |
| `evidence_state.evidence_stale` | Evidence stale | `caution` | `narrows` | The evidence body itself has aged past its freshness window. A stale evidence body narrows the claim below Stable until it is refreshed. |
| `evidence_state.not_provided` | Evidence not provided | `blocking` | `blocks` | No evidence body was provided for this claim. The absence is recorded explicitly and blocks a Stable claim until evidence is supplied. |

## Qualification / support-class badges

| Badge id | Label | Tone | Claim effect | Explanation drawer |
|----------|-------|------|--------------|--------------------|
| `support_class.certified` | Certified | `authoritative` | `none` | This surface is certified against current reference evidence — the strongest support claim. Certification narrows automatically if its evidence goes stale or missing. |
| `support_class.supported` | Supported | `informational` | `none` | This surface is supported by current evidence under a defined support window. It is a full claim short of formal certification. |
| `support_class.limited` | Limited | `caution` | `narrows` | This surface is supported only with narrower guarantees than a full claim. The reduced scope is stated rather than implied, and the claim narrows accordingly. |
| `support_class.community` | Community | `caution` | `narrows` | Support for this surface is community-maintained rather than first-party. It is a legitimate but narrower support class, so the claim narrows below a first-party class. |
| `support_class.experimental` | Experimental | `caution` | `narrows` | This surface is experimental and sits behind an explicit gate. Experimental support narrows the claim well below Stable. |
| `support_class.unsupported` | Unsupported | `blocking` | `blocks` | Current evidence does not support this surface. An unsupported state blocks a Stable claim and is surfaced rather than hidden. |

## Client-scope badges

| Badge id | Label | Tone | Claim effect | Explanation drawer |
|----------|-------|------|--------------|--------------------|
| `client_kind.desktop_full` | Desktop (full) | `authoritative` | `none` | The full desktop product surface. Only this client scope carries full authority and capability parity. |
| `client_kind.companion_scoped` | Companion (scoped) | `caution` | `narrows` | A companion surface with bounded scope relayed through the desktop host. It narrows a claim so it can never imply the desktop's authority or capability parity. |
| `client_kind.mobile_companion` | Mobile companion | `caution` | `narrows` | A mobile companion surface with bounded scope. It narrows a claim and cannot imply desktop parity. |
| `client_kind.embedded_panel` | Embedded panel | `caution` | `narrows` | A panel hosted inside another surface, under the host's constraints. It narrows a claim and cannot imply full-surface authority. |
| `client_kind.browser_reference` | Browser reference | `caution` | `narrows` | A browser reference surface — read-only and informational. It narrows a claim to discovery or reference and cannot carry in-product authority. |
| `client_kind.handoff_only` | Handoff only | `caution` | `narrows` | This surface can only create or open a desktop handoff. It narrows a claim to handoff actions and carries no standalone authority. |
| `authority_class.full_authority` | Full authority | `authoritative` | `none` | This surface carries full authority — the actions it offers are authoritative and at parity with the desktop. |
| `authority_class.scoped_authority` | Scoped authority | `caution` | `narrows` | This surface's authority is bounded to a relayed scope. It narrows a claim and cannot widen to desktop authority. |
| `authority_class.reference_only` | Reference only | `caution` | `narrows` | This surface is reference-only — it shows information but carries no authority to act. The claim narrows to reference. |
| `authority_class.handoff_only` | Handoff authority | `caution` | `narrows` | This surface's authority is limited to creating a desktop handoff. It narrows a claim and never acts authoritatively on its own. |
| `authority_class.not_provided` | Authority not provided | `blocking` | `blocks` | No authority class was provided for this surface. The absence is recorded explicitly and blocks a Stable claim until it is supplied. |
| `handoff_requirement.not_required` | No handoff required | `authoritative` | `none` | This surface acts in place and requires no handoff to another client. This is the only handoff state that does not narrow on handoff grounds. |
| `handoff_requirement.desktop_handoff_required` | Desktop handoff required | `caution` | `narrows` | Privileged actions on this surface require handing off to the desktop. The requirement is named rather than failing silently, and it narrows the surface's standalone claim. |
| `handoff_requirement.console_handoff_required` | Console handoff required | `caution` | `narrows` | Privileged actions here require handing off to a vendor console. The requirement is named explicitly and narrows the surface's standalone claim. |
| `handoff_requirement.not_provided` | Handoff state not provided | `blocking` | `blocks` | No handoff requirement was provided for this surface. The absence is recorded explicitly and blocks a Stable claim until it is supplied. |

## Required user-facing terms

| Term | Badge id | Dimension |
|------|----------|-----------|
| Signature verified | `signature_state.signed_attested` | `signature_state` |
| Attestation available | `signature_state.attestation_only` | `signature_state` |
| Mirrored | `source_origin.mirror` | `source_origin` |
| Side-loaded | `source_origin.side_loaded` | `source_origin` |
| Official | `source_origin.first_party_signed` | `source_origin` |
| Not provided | `source_origin.not_provided` | `source_origin` |
| Partial | `evidence_state.partial` | `evidence_state` |
| Certified | `support_class.certified` | `support_class` |
| Supported | `support_class.supported` | `support_class` |
| Limited | `support_class.limited` | `support_class` |
| Experimental | `support_class.experimental` | `support_class` |
| Retest pending | `evidence_state.retest_pending` | `evidence_state` |
| Evidence stale | `evidence_state.evidence_stale` | `evidence_state` |
