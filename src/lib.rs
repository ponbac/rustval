// Re-export the public API
pub mod resolve_ref;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_resolver() {
        use openapiv3::{OpenAPI, ReferenceOr};
        use resolve_ref::{OpenApiResolver, SchemaResolver};
        use std::fs;

        let data =
            fs::read_to_string("data/externaldata.swagger.yaml").expect("Could not read file");
        let openapi: OpenAPI = serde_yaml::from_str(&data).expect("Could not deserialize input");

        if let Some(components) = &openapi.components {
            for (_name, schema) in &components.schemas {
                if let ReferenceOr::Reference { reference } = schema {
                    let resolver = SchemaResolver::new();
                    assert!(resolver.resolve_reference(reference, &openapi).is_some());
                }
            }
        }
    }
}
