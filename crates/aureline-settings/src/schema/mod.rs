//! Settings schema registry: stable id, value type, allowed scopes,
//! restart posture, and lifecycle.

pub mod definition;
pub mod registry;
pub mod restart;
pub mod scope;
pub mod value;

pub use definition::{SettingDefinition, ValueValidationError};
pub use registry::{SchemaRegistry, SchemaRegistryError};
pub use restart::{LifecycleLabel, RestartPosture};
pub use scope::SettingScope;
pub use value::{SettingValue, SettingValueType};
