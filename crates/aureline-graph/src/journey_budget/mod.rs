//! Metadata-only budget ledger for graph query journeys.
//!
//! The ledger records synthetic query-work units such as query-family entry
//! and emitted result rows. It does not sample process memory, wall-clock time,
//! allocator state, or host counters; producers increment it only at explicit
//! graph runtime checkpoints.

use std::collections::BTreeMap;

/// Stable identity used to roll repeated graph queries into one journey.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JourneyId(String);

impl JourneyId {
    /// Creates a journey id from a deterministic, caller-owned token.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the stable journey id token.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Synthetic unit counted by the graph journey-budget ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BudgetUnit {
    /// A graph query-family entry point was invoked.
    QueryFamilyEntry,
    /// One graph query result row was admitted into the response envelope.
    ResultRow,
}

impl BudgetUnit {
    /// Returns the stable token used in fixtures and support rollups.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryFamilyEntry => "query_family_entry",
            Self::ResultRow => "result_row",
        }
    }
}

/// One admitted budget increment for a graph query journey.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumedRecord {
    /// Journey identity this increment contributes to.
    pub journey_id: JourneyId,
    /// Synthetic unit consumed by this increment.
    pub unit: BudgetUnit,
    /// Positive amount consumed for [`Self::unit`].
    pub amount: u64,
    /// Stable metadata-only source for the increment, such as a query class or row id.
    pub source_ref: String,
}

/// First budget limit that blocked more query work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetOverrun {
    /// Unit whose configured limit was exceeded.
    pub unit: BudgetUnit,
    /// Configured maximum for [`Self::unit`].
    pub limit: u64,
    /// Amount already admitted before the rejected increment.
    pub consumed_before_attempt: u64,
    /// Amount the runtime attempted to admit.
    pub attempted_amount: u64,
    /// Stable metadata-only source for the rejected increment.
    pub source_ref: String,
}

/// Rollup of all budget increments observed for one graph query journey.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerRollup {
    /// Journey identity shared by semantically identical query requests.
    pub journey_id: JourneyId,
    /// Optional per-unit limits applied while producing this rollup.
    pub budget_limits: BTreeMap<BudgetUnit, u64>,
    /// Totals by synthetic budget unit.
    pub totals: BTreeMap<BudgetUnit, u64>,
    /// Ordered admitted increments.
    pub consumed_records: Vec<ConsumedRecord>,
    /// First rejected increment, when the query exceeded a configured limit.
    pub overrun: Option<BudgetOverrun>,
}

impl LedgerRollup {
    /// Builds an empty rollup for a journey with no configured limits.
    pub fn unlimited(journey_id: JourneyId) -> Self {
        Self {
            journey_id,
            budget_limits: BTreeMap::new(),
            totals: BTreeMap::new(),
            consumed_records: Vec::new(),
            overrun: None,
        }
    }

    /// Returns the total amount consumed for one synthetic unit.
    pub fn consumed_total(&self, unit: BudgetUnit) -> u64 {
        self.totals.get(&unit).copied().unwrap_or(0)
    }

    /// Returns true when this rollup contains a rejected increment.
    pub fn exceeded_budget(&self) -> bool {
        self.overrun.is_some()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JourneyBudgetLedger {
    rollup: LedgerRollup,
}

impl JourneyBudgetLedger {
    pub(crate) fn new(journey_id: JourneyId, budget_limits: BTreeMap<BudgetUnit, u64>) -> Self {
        Self {
            rollup: LedgerRollup {
                journey_id,
                budget_limits,
                totals: BTreeMap::new(),
                consumed_records: Vec::new(),
                overrun: None,
            },
        }
    }

    pub(crate) fn consume(
        &mut self,
        unit: BudgetUnit,
        amount: u64,
        source_ref: impl Into<String>,
    ) -> bool {
        if amount == 0 {
            return true;
        }
        if self.rollup.overrun.is_some() {
            return false;
        }

        let source_ref = source_ref.into();
        let consumed_before_attempt = self.rollup.consumed_total(unit);
        if let Some(limit) = self.rollup.budget_limits.get(&unit).copied() {
            if consumed_before_attempt.saturating_add(amount) > limit {
                self.rollup.overrun = Some(BudgetOverrun {
                    unit,
                    limit,
                    consumed_before_attempt,
                    attempted_amount: amount,
                    source_ref,
                });
                return false;
            }
        }

        self.rollup
            .totals
            .entry(unit)
            .and_modify(|total| *total += amount)
            .or_insert(amount);
        self.rollup.consumed_records.push(ConsumedRecord {
            journey_id: self.rollup.journey_id.clone(),
            unit,
            amount,
            source_ref,
        });
        true
    }

    pub(crate) fn exceeded_budget(&self) -> bool {
        self.rollup.exceeded_budget()
    }

    pub(crate) fn into_rollup(self) -> LedgerRollup {
        self.rollup
    }
}
