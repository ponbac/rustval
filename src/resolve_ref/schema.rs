use indexmap::map::IndexMap;
use openapiv3::{OpenAPI, ReferenceOr, Schema};

use crate::resolve_ref::resolvers::{SchemaResolver, resolve_schema_ref};
use crate::resolve_ref::traits::OpenApiResolver;

/// Resolve all references in a schema, including nested ones
///
/// This function not only resolves direct references to schemas, but also
/// recursively resolves references inside the schema's properties, array items,
/// and composition fields (oneOf, allOf, anyOf).
pub fn resolve_schema_fully(schema: &ReferenceOr<Schema>, spec: &OpenAPI) -> Option<Schema> {
    let resolver = SchemaResolver::new();
    let resolved_schema = resolver.resolve_reference_or(schema, spec)?;

    // Create a new schema with resolved references
    let mut new_schema = resolved_schema.clone();

    // Resolve references based on schema kind
    match &mut new_schema.schema_kind {
        openapiv3::SchemaKind::Type(type_obj) => {
            match type_obj {
                openapiv3::Type::Object(obj) => {
                    // Handle object properties
                    let mut resolved_properties = IndexMap::new();

                    for (prop_name, prop_schema) in &obj.properties {
                        match prop_schema {
                            ReferenceOr::Reference { reference } => {
                                if let Some(resolved_prop) = resolve_schema_ref(reference, spec) {
                                    resolved_properties.insert(
                                        prop_name.clone(),
                                        ReferenceOr::Item(Box::new(resolved_prop)),
                                    );
                                } else {
                                    resolved_properties
                                        .insert(prop_name.clone(), prop_schema.clone());
                                }
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                let schema_ref = ReferenceOr::Item(*boxed_schema.clone());
                                if let Some(resolved_prop) = resolve_schema_fully(&schema_ref, spec)
                                {
                                    resolved_properties.insert(
                                        prop_name.clone(),
                                        ReferenceOr::Item(Box::new(resolved_prop)),
                                    );
                                } else {
                                    resolved_properties
                                        .insert(prop_name.clone(), prop_schema.clone());
                                }
                            }
                        }
                    }

                    obj.properties = resolved_properties;
                }
                openapiv3::Type::Array(array) => {
                    // Handle array items
                    if let Some(items) = &array.items {
                        match items {
                            ReferenceOr::Reference { reference } => {
                                if let Some(resolved_items) = resolve_schema_ref(reference, spec) {
                                    array.items = Some(ReferenceOr::Item(Box::new(resolved_items)));
                                }
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                let schema_ref = ReferenceOr::Item(*boxed_schema.clone());
                                if let Some(resolved_items) =
                                    resolve_schema_fully(&schema_ref, spec)
                                {
                                    array.items = Some(ReferenceOr::Item(Box::new(resolved_items)));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        openapiv3::SchemaKind::OneOf { one_of } => {
            let mut one_of_vec = one_of.clone();
            resolve_schema_list(&mut one_of_vec, spec);
            new_schema.schema_kind = openapiv3::SchemaKind::OneOf { one_of: one_of_vec };
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            let mut all_of_vec = all_of.clone();
            resolve_schema_list(&mut all_of_vec, spec);
            new_schema.schema_kind = openapiv3::SchemaKind::AllOf { all_of: all_of_vec };
        }
        openapiv3::SchemaKind::AnyOf { any_of } => {
            let mut any_of_vec = any_of.clone();
            resolve_schema_list(&mut any_of_vec, spec);
            new_schema.schema_kind = openapiv3::SchemaKind::AnyOf { any_of: any_of_vec };
        }
        _ => {}
    }

    Some(new_schema)
}

/// Helper function to resolve a list of schemas in-place
///
/// This is used for resolving schema compositions like oneOf, allOf, and anyOf.
pub fn resolve_schema_list(schema_list: &mut [ReferenceOr<Schema>], spec: &OpenAPI) {
    (0..schema_list.len()).for_each(|i| {
        let schema_ref = &schema_list[i];
        if let Some(resolved) = resolve_schema_fully(schema_ref, spec) {
            schema_list[i] = ReferenceOr::Item(resolved);
        }
    });
}
