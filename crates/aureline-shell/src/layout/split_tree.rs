// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

use crate::layout::zone_registry::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaneId(u64);

impl PaneId {
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Stable identifier for one live split node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SplitId(u64);

impl SplitId {
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    Vertical,
}

/// Read-only structural snapshot of a live split tree.
///
/// The snapshot carries no pane contents or authority. It exists so session
/// restore can preserve split identity, nesting, child order, orientation,
/// and weights without reaching into the mutable layout tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitTopologyNode {
    Leaf {
        pane_id: PaneId,
    },
    Split {
        split_id: SplitId,
        axis: SplitAxis,
        first_weight: u16,
        second_weight: u16,
        first: Box<SplitTopologyNode>,
        second: Box<SplitTopologyNode>,
    },
}

impl SplitAxis {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Leaf {
        pane_id: PaneId,
    },
    Split {
        split_id: SplitId,
        axis: SplitAxis,
        first_weight: u16,
        second_weight: u16,
        first: Box<Node>,
        second: Box<Node>,
    },
}

impl Node {
    fn leaf_count(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Split { first, second, .. } => first.leaf_count() + second.leaf_count(),
        }
    }

    fn leaf_ids_in_order(&self, out: &mut Vec<PaneId>) {
        match self {
            Self::Leaf { pane_id } => out.push(*pane_id),
            Self::Split { first, second, .. } => {
                first.leaf_ids_in_order(out);
                second.leaf_ids_in_order(out);
            }
        }
    }

    fn contains_leaf(&self, leaf: PaneId) -> bool {
        match self {
            Self::Leaf { pane_id } => *pane_id == leaf,
            Self::Split { first, second, .. } => {
                first.contains_leaf(leaf) || second.contains_leaf(leaf)
            }
        }
    }

    fn split_leaf(
        &mut self,
        leaf: PaneId,
        axis: SplitAxis,
        new_leaf: PaneId,
        new_split: SplitId,
    ) -> bool {
        match self {
            Self::Leaf { pane_id } => {
                if *pane_id != leaf {
                    return false;
                }
                let old_leaf = *pane_id;
                *self = Self::Split {
                    split_id: new_split,
                    axis,
                    first_weight: 1,
                    second_weight: 1,
                    first: Box::new(Self::Leaf { pane_id: old_leaf }),
                    second: Box::new(Self::Leaf { pane_id: new_leaf }),
                };
                true
            }
            Self::Split { first, second, .. } => {
                if first.split_leaf(leaf, axis, new_leaf, new_split) {
                    true
                } else {
                    second.split_leaf(leaf, axis, new_leaf, new_split)
                }
            }
        }
    }

    fn structural_snapshot(&self) -> SplitTopologyNode {
        match self {
            Self::Leaf { pane_id } => SplitTopologyNode::Leaf { pane_id: *pane_id },
            Self::Split {
                split_id,
                axis,
                first_weight,
                second_weight,
                first,
                second,
            } => SplitTopologyNode::Split {
                split_id: *split_id,
                axis: *axis,
                first_weight: *first_weight,
                second_weight: *second_weight,
                first: Box::new(first.structural_snapshot()),
                second: Box::new(second.structural_snapshot()),
            },
        }
    }

    fn remove_leaf(&mut self, leaf: PaneId) -> bool {
        match self {
            Self::Leaf { .. } => false,
            Self::Split { first, second, .. } => match (&mut **first, &mut **second) {
                (Node::Leaf { pane_id }, _) if *pane_id == leaf => {
                    let replacement =
                        core::mem::replace(&mut **second, Node::Leaf { pane_id: PaneId(0) });
                    *self = replacement;
                    true
                }
                (_, Node::Leaf { pane_id }) if *pane_id == leaf => {
                    let replacement =
                        core::mem::replace(&mut **first, Node::Leaf { pane_id: PaneId(0) });
                    *self = replacement;
                    true
                }
                _ => first.remove_leaf(leaf) || second.remove_leaf(leaf),
            },
        }
    }

    fn layout_with_min_width(
        &self,
        container: Rect,
        min_leaf_width: u32,
        out: &mut Vec<(PaneId, Rect)>,
    ) -> Result<(), SplitLayoutError> {
        match self {
            Self::Leaf { pane_id } => {
                out.push((*pane_id, container));
                Ok(())
            }
            Self::Split {
                axis,
                first_weight,
                second_weight,
                first,
                second,
                ..
            } => match axis {
                SplitAxis::Vertical => {
                    let first_required = (first.leaf_count() as u32).saturating_mul(min_leaf_width);
                    let second_required =
                        (second.leaf_count() as u32).saturating_mul(min_leaf_width);
                    let required_total = first_required.saturating_add(second_required);
                    if container.width < required_total {
                        return Err(SplitLayoutError::TooNarrow {
                            required_width: required_total,
                            available_width: container.width,
                        });
                    }

                    let weight_total = (*first_weight as u32)
                        .saturating_add(*second_weight as u32)
                        .max(1);
                    let mut first_width =
                        container.width.saturating_mul(*first_weight as u32) / weight_total;
                    let mut second_width = container.width.saturating_sub(first_width);

                    if first_width < first_required {
                        first_width = first_required;
                        second_width = container.width.saturating_sub(first_width);
                    }
                    if second_width < second_required {
                        second_width = second_required;
                        first_width = container.width.saturating_sub(second_width);
                    }

                    if first_width < first_required || second_width < second_required {
                        return Err(SplitLayoutError::TooNarrow {
                            required_width: required_total,
                            available_width: container.width,
                        });
                    }

                    let first_rect =
                        Rect::new(container.x, container.y, first_width, container.height);
                    let second_rect = Rect::new(
                        container.x.saturating_add(first_width),
                        container.y,
                        second_width,
                        container.height,
                    );
                    first.layout_with_min_width(first_rect, min_leaf_width, out)?;
                    second.layout_with_min_width(second_rect, min_leaf_width, out)?;
                    Ok(())
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitLayoutError {
    TooNarrow {
        required_width: u32,
        available_width: u32,
    },
}

#[derive(Debug, Clone)]
pub struct SplitTree {
    next_pane_id: u64,
    next_split_id: u64,
    root: Node,
}

impl SplitTree {
    pub fn single() -> Self {
        Self {
            next_pane_id: 2,
            next_split_id: 1,
            root: Node::Leaf { pane_id: PaneId(1) },
        }
    }

    pub fn root_leaf(&self) -> PaneId {
        self.leaf_ids_in_order()
            .first()
            .copied()
            .unwrap_or(PaneId(1))
    }

    pub fn leaf_count(&self) -> usize {
        self.root.leaf_count()
    }

    pub fn leaf_ids_in_order(&self) -> Vec<PaneId> {
        let mut out = Vec::new();
        self.root.leaf_ids_in_order(&mut out);
        out
    }

    pub fn structural_snapshot(&self) -> SplitTopologyNode {
        self.root.structural_snapshot()
    }

    pub fn contains_leaf(&self, leaf: PaneId) -> bool {
        self.root.contains_leaf(leaf)
    }

    pub fn min_required_width(&self, min_leaf_width: u32) -> u32 {
        (self.leaf_count() as u32).saturating_mul(min_leaf_width)
    }

    pub fn split_leaf(&mut self, leaf: PaneId, axis: SplitAxis) -> Option<PaneId> {
        if !self.contains_leaf(leaf) {
            return None;
        }
        let new_leaf = PaneId(self.next_pane_id);
        self.next_pane_id = self.next_pane_id.saturating_add(1);
        let new_split = SplitId(self.next_split_id);
        self.next_split_id = self.next_split_id.saturating_add(1);
        if self.root.split_leaf(leaf, axis, new_leaf, new_split) {
            Some(new_leaf)
        } else {
            None
        }
    }

    pub fn remove_leaf(&mut self, leaf: PaneId) -> bool {
        if self.leaf_count() <= 1 {
            return false;
        }
        self.root.remove_leaf(leaf)
    }

    pub fn layout_with_min_width(
        &self,
        container: Rect,
        min_leaf_width: u32,
    ) -> Result<Vec<(PaneId, Rect)>, SplitLayoutError> {
        let mut out = Vec::with_capacity(self.leaf_count());
        self.root
            .layout_with_min_width(container, min_leaf_width, &mut out)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_tree_mints_stable_leaf_ids() {
        let mut tree = SplitTree::single();
        let root = tree.root_leaf();
        let second = tree
            .split_leaf(root, SplitAxis::Vertical)
            .expect("split should succeed");
        assert_eq!(root.value(), 1);
        assert_eq!(second.value(), 2);
        assert_eq!(tree.leaf_count(), 2);
        assert_eq!(tree.leaf_ids_in_order(), vec![root, second]);
    }

    #[test]
    fn structural_snapshot_preserves_nested_split_ids_order_and_weights() {
        let mut tree = SplitTree::single();
        let first = tree.root_leaf();
        let second = tree
            .split_leaf(first, SplitAxis::Vertical)
            .expect("outer split");
        let third = tree
            .split_leaf(second, SplitAxis::Vertical)
            .expect("inner split");

        let SplitTopologyNode::Split {
            split_id,
            first_weight,
            second_weight,
            first: outer_first,
            second: outer_second,
            ..
        } = tree.structural_snapshot()
        else {
            panic!("outer split snapshot")
        };
        assert_eq!(split_id.value(), 1);
        assert_eq!((first_weight, second_weight), (1, 1));
        assert!(matches!(
            *outer_first,
            SplitTopologyNode::Leaf { pane_id } if pane_id == first
        ));
        let SplitTopologyNode::Split {
            split_id,
            first_weight,
            second_weight,
            first: inner_first,
            second: inner_second,
            ..
        } = *outer_second
        else {
            panic!("inner split snapshot")
        };
        assert_eq!(split_id.value(), 2);
        assert_eq!((first_weight, second_weight), (1, 1));
        assert!(matches!(
            *inner_first,
            SplitTopologyNode::Leaf { pane_id } if pane_id == second
        ));
        assert!(matches!(
            *inner_second,
            SplitTopologyNode::Leaf { pane_id } if pane_id == third
        ));
    }

    #[test]
    fn layout_rejects_insufficient_width_for_leaf_minimums() {
        let mut tree = SplitTree::single();
        let root = tree.root_leaf();
        tree.split_leaf(root, SplitAxis::Vertical).unwrap();

        let err = tree
            .layout_with_min_width(Rect::new(0, 0, 600, 100), 420)
            .expect_err("should reject narrow containers");
        assert_eq!(
            err,
            SplitLayoutError::TooNarrow {
                required_width: 840,
                available_width: 600
            }
        );
    }

    #[test]
    fn layout_honors_leaf_minimums_when_width_is_sufficient() {
        let mut tree = SplitTree::single();
        let root = tree.root_leaf();
        let second = tree.split_leaf(root, SplitAxis::Vertical).unwrap();
        let layout = tree
            .layout_with_min_width(Rect::new(0, 0, 1000, 100), 420)
            .expect("layout should succeed");

        let left = layout.iter().find(|(id, _)| *id == root).unwrap().1;
        let right = layout.iter().find(|(id, _)| *id == second).unwrap().1;
        assert!(left.width >= 420);
        assert!(right.width >= 420);
        assert_eq!(left.height, 100);
        assert_eq!(right.height, 100);
        assert_eq!(left.x, 0);
        assert_eq!(right.x, left.right());
        assert_eq!(left.width + right.width, 1000);
    }
}
