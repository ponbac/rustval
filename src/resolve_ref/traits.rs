use openapiv3::{OpenAPI, ReferenceOr};

/// Trait for accessing component maps in the OpenAPI spec
pub trait ComponentGetter<T> {
    /// The path segment used in references (e.g., "schemas", "responses")
    fn component_type() -> &'static str;

    /// Get a component by name from the OpenAPI spec
    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<T>>;
}

/// Generic trait for resolving references in OpenAPI components
pub trait OpenApiResolver<T> {
    /// Resolve a reference string to the actual component
    fn resolve_reference(&self, reference: &str, spec: &OpenAPI) -> Option<T>;

    /// Resolve a ReferenceOr to the actual component
    fn resolve_reference_or(&self, reference_or: &ReferenceOr<T>, spec: &OpenAPI) -> Option<T>
    where
        T: Clone,
    {
        match reference_or {
            ReferenceOr::Reference { reference } => self.resolve_reference(reference, spec),
            ReferenceOr::Item(item) => Some(item.clone()),
        }
    }
}
