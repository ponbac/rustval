//! OpenAPI reference resolution module
//!
//! This module provides utilities for resolving references in OpenAPI specifications.

mod resolvers;
mod schema;
mod traits;
mod utils;

// Re-export the public API
pub use resolvers::{
    ExampleResolver, GenericResolver, ParameterResolver, RequestBodyResolver, ResponseResolver,
    SchemaResolver,
};
pub use schema::{resolve_schema_fully, resolve_schema_list};
pub use traits::{ComponentGetter, OpenApiResolver};
pub use utils::parse_ref;

// Convenience functions
pub use resolvers::{
    resolve_example_ref, resolve_parameter_ref, resolve_reference_or, resolve_request_body_ref,
    resolve_response_ref, resolve_schema_ref,
};
