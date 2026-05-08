//! Focus-return helpers for transient surfaces.
//!
//! Protected shell surfaces (dialogs, sheets, palettes, popovers) record a
//! focus-return target before stealing focus, then restore focus predictably
//! when the transient surface is dismissed.

/// A stack of focus-return targets for transient surfaces.
#[derive(Debug, Clone)]
pub struct FocusReturnStack<T> {
    stack: Vec<T>,
}

impl<T> FocusReturnStack<T> {
    /// Creates an empty focus-return stack.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Returns `true` when the stack holds no targets.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the number of recorded targets.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns the most recently recorded target.
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    /// Records a new focus-return target.
    pub fn record(&mut self, target: T) {
        self.stack.push(target);
    }

    /// Records `target` unless it matches the stack's current top.
    pub fn record_if_changed(&mut self, target: T)
    where
        T: PartialEq,
    {
        if self.stack.last().is_some_and(|top| *top == target) {
            return;
        }
        self.stack.push(target);
    }

    /// Pops and returns the most recently recorded target.
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Removes all recorded targets.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}

impl<T> Default for FocusReturnStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_and_pops_in_lifo_order() {
        let mut stack = FocusReturnStack::new();
        assert!(stack.is_empty());

        stack.record(1);
        stack.record(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn record_if_changed_deduplicates_top() {
        let mut stack = FocusReturnStack::new();
        stack.record_if_changed("a");
        stack.record_if_changed("a");
        stack.record_if_changed("b");
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some("b"));
        assert_eq!(stack.pop(), Some("a"));
    }
}
