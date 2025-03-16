use std::marker::PhantomData;

use openapiv3::{Example, OpenAPI, Parameter, ReferenceOr, RequestBody, Response, Schema};

use crate::resolve_ref::traits::{ComponentGetter, OpenApiResolver};
use crate::resolve_ref::utils::parse_ref;

/// Generic resolver implementation for all component types
pub struct GenericResolver<T, G: ComponentGetter<T>>(PhantomData<(T, G)>);

impl<T, G> GenericResolver<T, G>
where
    T: Clone,
    G: ComponentGetter<T>,
{
    pub fn new() -> Self {
        GenericResolver(PhantomData)
    }
}

impl<T, G> Default for GenericResolver<T, G>
where
    T: Clone,
    G: ComponentGetter<T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, G> OpenApiResolver<T> for GenericResolver<T, G>
where
    T: Clone,
    G: ComponentGetter<T>,
{
    fn resolve_reference(&self, reference: &str, spec: &OpenAPI) -> Option<T> {
        let parts = parse_ref(reference);

        if parts.len() < 3 || parts[0] != "components" || parts[1] != G::component_type() {
            return None;
        }

        let name = &parts[2];
        let components = spec.components.as_ref()?;

        G::get_component(components, name).and_then(|reference_or| match reference_or {
            ReferenceOr::Reference { reference } => self.resolve_reference(reference, spec),
            ReferenceOr::Item(item) => Some(item.clone()),
        })
    }
}

// Component getter implementations

/// Schema component getter
pub struct SchemaGetter;
impl ComponentGetter<Schema> for SchemaGetter {
    fn component_type() -> &'static str {
        "schemas"
    }

    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<Schema>> {
        components.schemas.get(name)
    }
}

/// Response component getter
pub struct ResponseGetter;
impl ComponentGetter<Response> for ResponseGetter {
    fn component_type() -> &'static str {
        "responses"
    }

    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<Response>> {
        components.responses.get(name)
    }
}

/// Parameter component getter
pub struct ParameterGetter;
impl ComponentGetter<Parameter> for ParameterGetter {
    fn component_type() -> &'static str {
        "parameters"
    }

    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<Parameter>> {
        components.parameters.get(name)
    }
}

/// RequestBody component getter
pub struct RequestBodyGetter;
impl ComponentGetter<RequestBody> for RequestBodyGetter {
    fn component_type() -> &'static str {
        "requestBodies"
    }

    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<RequestBody>> {
        components.request_bodies.get(name)
    }
}

/// Example component getter
pub struct ExampleGetter;
impl ComponentGetter<Example> for ExampleGetter {
    fn component_type() -> &'static str {
        "examples"
    }

    fn get_component<'a>(
        components: &'a openapiv3::Components,
        name: &str,
    ) -> Option<&'a ReferenceOr<Example>> {
        components.examples.get(name)
    }
}

// Resolver type aliases
pub type SchemaResolver = GenericResolver<Schema, SchemaGetter>;
pub type ResponseResolver = GenericResolver<Response, ResponseGetter>;
pub type ParameterResolver = GenericResolver<Parameter, ParameterGetter>;
pub type RequestBodyResolver = GenericResolver<RequestBody, RequestBodyGetter>;
pub type ExampleResolver = GenericResolver<Example, ExampleGetter>;

// Convenience functions
/// Resolve a schema reference
pub fn resolve_schema_ref(reference: &str, spec: &OpenAPI) -> Option<Schema> {
    SchemaResolver::new().resolve_reference(reference, spec)
}

/// Resolve a response reference
pub fn resolve_response_ref(reference: &str, spec: &OpenAPI) -> Option<Response> {
    ResponseResolver::new().resolve_reference(reference, spec)
}

/// Resolve a parameter reference
pub fn resolve_parameter_ref(reference: &str, spec: &OpenAPI) -> Option<Parameter> {
    ParameterResolver::new().resolve_reference(reference, spec)
}

/// Resolve a request body reference
pub fn resolve_request_body_ref(reference: &str, spec: &OpenAPI) -> Option<RequestBody> {
    RequestBodyResolver::new().resolve_reference(reference, spec)
}

/// Resolve an example reference
pub fn resolve_example_ref(reference: &str, spec: &OpenAPI) -> Option<Example> {
    ExampleResolver::new().resolve_reference(reference, spec)
}

/// General purpose function to resolve a ReferenceOr of any OpenAPI component type
pub fn resolve_reference_or<T, R>(
    reference_or: &ReferenceOr<T>,
    spec: &OpenAPI,
    resolver: &R,
) -> Option<T>
where
    T: Clone,
    R: OpenApiResolver<T>,
{
    resolver.resolve_reference_or(reference_or, spec)
}
