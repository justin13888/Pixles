mod mutation;
mod query;

pub use mutation::*;
pub use query::*;

pub use sea_orm;

// TODO: remove this package unless we actually have shared and non-domain-specific code between APIs
