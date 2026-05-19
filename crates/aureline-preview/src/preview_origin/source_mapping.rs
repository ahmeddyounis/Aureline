//! Source-mapping quality descriptor.
//!
//! Names how trustworthy a source jump from a preview view is. The five
//! values are spec-frozen — adding a new one is additive-minor; repurposing
//! is breaking.
//!
//! `exact`     — every node maps unambiguously to a canonical-source byte
//! `heuristic` — best-effort map produced from naming / convention
//! `stale`     — map was once accurate but the source has drifted since
//! `partial`   — only a subset of nodes maps; the rest are unmappable
//! `unavailable` — no source mapping at all (e.g. a captured screenshot)

use serde::{Deserialize, Serialize};

use super::PreviewOriginFinding;

/// Closed source-mapping quality vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMappingQualityClass {
    Exact,
    Heuristic,
    Stale,
    Partial,
    Unavailable,
}

impl SourceMappingQualityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Heuristic => "heuristic",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when a source-jump action against this mapping can land on a
    /// canonical-source byte deterministically.
    pub const fn admits_deterministic_jump(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// True when a source-jump action is admissible at all (even if it
    /// only lands near the target).
    pub const fn admits_any_jump(self) -> bool {
        matches!(self, Self::Exact | Self::Heuristic | Self::Partial)
    }
}

/// Source-mapping descriptor for a single preview view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMappingDescriptor {
    pub source_mapping_quality_class: SourceMappingQualityClass,
    /// Total node count in the projected view. May be 0 when the mapping
    /// is unavailable.
    pub total_node_count: u32,
    /// Number of nodes that map unambiguously to canonical-source bytes.
    pub exact_mapped_node_count: u32,
    /// Number of nodes the mapping could not resolve.
    pub unmappable_node_count: u32,
    /// Reviewer-facing one-sentence explanation. Never contains raw
    /// stack frames or stderr.
    pub summary: String,
}

impl SourceMappingDescriptor {
    pub fn validate(&self, subject: &str) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();

        if self.exact_mapped_node_count + self.unmappable_node_count > self.total_node_count {
            findings.push(PreviewOriginFinding::new(
                "source_mapping_descriptor.node_count_overflow",
                subject,
                "exact_mapped_node_count + unmappable_node_count must not exceed total_node_count",
            ));
        }

        match self.source_mapping_quality_class {
            SourceMappingQualityClass::Exact => {
                if self.unmappable_node_count > 0 {
                    findings.push(PreviewOriginFinding::new(
                        "source_mapping_descriptor.exact_forbids_unmappable",
                        subject,
                        "exact mapping cannot report unmappable nodes",
                    ));
                }
                if self.total_node_count > 0
                    && self.exact_mapped_node_count != self.total_node_count
                {
                    findings.push(PreviewOriginFinding::new(
                        "source_mapping_descriptor.exact_requires_full_coverage",
                        subject,
                        "exact mapping requires exact_mapped_node_count = total_node_count",
                    ));
                }
            }
            SourceMappingQualityClass::Partial => {
                if self.unmappable_node_count == 0 {
                    findings.push(PreviewOriginFinding::new(
                        "source_mapping_descriptor.partial_requires_unmappable",
                        subject,
                        "partial mapping must report at least one unmappable node",
                    ));
                }
                if self.exact_mapped_node_count == 0 {
                    findings.push(PreviewOriginFinding::new(
                        "source_mapping_descriptor.partial_requires_some_mapped",
                        subject,
                        "partial mapping must report at least one exact-mapped node",
                    ));
                }
            }
            SourceMappingQualityClass::Unavailable => {
                if self.exact_mapped_node_count > 0 {
                    findings.push(PreviewOriginFinding::new(
                        "source_mapping_descriptor.unavailable_forbids_mapped",
                        subject,
                        "unavailable mapping cannot report mapped nodes",
                    ));
                }
            }
            SourceMappingQualityClass::Heuristic | SourceMappingQualityClass::Stale => {}
        }

        findings
    }
}
