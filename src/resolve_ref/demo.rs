use std::fs;

use indexmap::map::IndexMap;
use openapiv3::{OpenAPI, ReferenceOr, Schema};

use crate::resolve_ref::resolvers::{resolve_request_body_ref, resolve_response_ref};
use crate::resolve_ref::schema::resolve_schema_fully;

/// Example function to demonstrate reference resolution by fully resolving all paths
pub fn demonstrate_reference_resolution() {
    println!("Loading OpenAPI specification...");
    let data = fs::read_to_string("data/externaldata.swagger.yaml").expect("Could not read file");
    let openapi: OpenAPI = serde_yaml::from_str(&data).expect("Could not deserialize input");

    println!("\n=== Resolving all paths and their schema references ===");

    // Process each path and operation
    for (path, path_item_ref) in &openapi.paths.paths {
        println!("\nPath: {}", path);

        // Handle the potential reference in path_item
        match path_item_ref {
            ReferenceOr::Reference { reference } => {
                println!("  Path is a reference to: {}", reference);
                // In a full implementation, we would resolve the reference here
            }
            ReferenceOr::Item(path_item) => {
                // Process each operation (GET, POST, etc.)
                if let Some(get) = &path_item.get {
                    process_operation("GET", get, &openapi);
                }
                if let Some(post) = &path_item.post {
                    process_operation("POST", post, &openapi);
                }
                if let Some(put) = &path_item.put {
                    process_operation("PUT", put, &openapi);
                }
                if let Some(delete) = &path_item.delete {
                    process_operation("DELETE", delete, &openapi);
                }
                // Add other methods if needed
            }
        }
    }

    // Additionally demonstrate resolving all components directly
    if let Some(components) = &openapi.components {
        println!("\n=== Resolving all component schemas ===");
        for (name, schema) in &components.schemas {
            println!("\nSchema: {}", name);
            match schema {
                ReferenceOr::Reference { reference } => {
                    println!("  Reference to: {}", reference);
                    if let Some(resolved) = resolve_schema_fully(
                        &ReferenceOr::Reference {
                            reference: reference.clone(),
                        },
                        &openapi,
                    ) {
                        println!("  Successfully resolved reference");
                        print_schema_structure(&resolved);
                    } else {
                        println!("  Failed to resolve reference");
                    }
                }
                ReferenceOr::Item(schema) => {
                    println!("  Inline schema definition");
                    let schema_ref = ReferenceOr::Item(schema.clone());
                    if let Some(resolved) = resolve_schema_fully(&schema_ref, &openapi) {
                        println!("  Successfully resolved nested references");
                        print_schema_structure(&resolved);
                    } else {
                        println!("  Failed to resolve nested references");
                    }
                }
            }
        }
    }
}

/// Helper function to process an operation and resolve its schemas
fn process_operation(method: &str, operation: &openapiv3::Operation, openapi: &OpenAPI) {
    println!(
        "  Operation: {} - {}",
        method,
        operation.operation_id.as_deref().unwrap_or("unnamed")
    );

    // Process request body schemas
    if let Some(request_body) = &operation.request_body {
        println!("    Request Body:");
        match request_body {
            ReferenceOr::Reference { reference } => {
                println!("      Reference to: {}", reference);
                if let Some(resolved) = resolve_request_body_ref(reference, openapi) {
                    println!("      Successfully resolved request body reference");
                    process_content(&resolved.content, openapi);
                }
            }
            ReferenceOr::Item(body) => {
                process_content(&body.content, openapi);
            }
        }
    }

    // Process response schemas
    println!("    Responses:");
    for (status, response) in &operation.responses.responses {
        println!("      Status: {}", status);
        match response {
            ReferenceOr::Reference { reference } => {
                println!("        Reference to: {}", reference);
                if let Some(resolved) = resolve_response_ref(reference, openapi) {
                    println!("        Successfully resolved response reference");
                    process_content(&resolved.content, openapi);
                }
            }
            ReferenceOr::Item(response) => {
                process_content(&response.content, openapi);
            }
        }
    }
}

/// Helper function to process content and resolve its schemas
fn process_content(content: &IndexMap<String, openapiv3::MediaType>, openapi: &OpenAPI) {
    for (media_type, content_type) in content {
        println!("        Media Type: {}", media_type);
        if let Some(schema) = &content_type.schema {
            match schema {
                ReferenceOr::Reference { reference } => {
                    println!("          Schema reference: {}", reference);
                    if let Some(resolved) = resolve_schema_fully(
                        &ReferenceOr::Reference {
                            reference: reference.clone(),
                        },
                        openapi,
                    ) {
                        println!("          Successfully resolved schema reference");
                        print_schema_structure(&resolved);
                    } else {
                        println!("          Failed to resolve schema reference");
                    }
                }
                ReferenceOr::Item(schema) => {
                    println!("          Inline schema");
                    let schema_ref = ReferenceOr::Item(schema.clone());
                    if let Some(resolved) = resolve_schema_fully(&schema_ref, openapi) {
                        println!("          Successfully resolved nested references");
                        print_schema_structure(&resolved);
                    } else {
                        println!("          Failed to resolve nested references");
                    }
                }
            }
        }
    }
}

/// Helper function to print the structure of a schema
fn print_schema_structure(schema: &Schema) {
    match &schema.schema_kind {
        openapiv3::SchemaKind::Type(type_obj) => match type_obj {
            openapiv3::Type::Object(obj) => {
                println!("          Type: Object");
                println!("          Properties:");
                for (name, _) in &obj.properties {
                    println!("            - {}", name);
                }
            }
            openapiv3::Type::Array(array) => {
                println!("          Type: Array");
                if let Some(items) = &array.items {
                    match items {
                        ReferenceOr::Reference { reference: _ } => {
                            println!("          Items: Reference (Resolved)");
                        }
                        ReferenceOr::Item(schema) => {
                            println!("          Items: Inline schema");
                            match &schema.schema_kind {
                                openapiv3::SchemaKind::Type(t) => match t {
                                    openapiv3::Type::Object(_) => {
                                        println!("            Type: Object")
                                    }
                                    openapiv3::Type::Array(_) => {
                                        println!("            Type: Array")
                                    }
                                    openapiv3::Type::String(_) => {
                                        println!("            Type: String")
                                    }
                                    openapiv3::Type::Number(_) => {
                                        println!("            Type: Number")
                                    }
                                    openapiv3::Type::Integer(_) => {
                                        println!("            Type: Integer")
                                    }
                                    openapiv3::Type::Boolean(_) => {
                                        println!("            Type: Boolean")
                                    }
                                },
                                _ => println!("            Complex schema"),
                            }
                        }
                    }
                }
            }
            openapiv3::Type::String(_) => println!("          Type: String"),
            openapiv3::Type::Number(_) => println!("          Type: Number"),
            openapiv3::Type::Integer(_) => println!("          Type: Integer"),
            openapiv3::Type::Boolean(_) => println!("          Type: Boolean"),
        },
        openapiv3::SchemaKind::OneOf { one_of } => {
            println!("          OneOf ({} options)", one_of.len());
        }
        openapiv3::SchemaKind::AllOf { all_of } => {
            println!("          AllOf ({} components)", all_of.len());
        }
        openapiv3::SchemaKind::AnyOf { any_of } => {
            println!("          AnyOf ({} options)", any_of.len());
        }
        openapiv3::SchemaKind::Not { not: _ } => {
            println!("          Not schema");
        }
        openapiv3::SchemaKind::Any(_) => {
            println!("          Any schema");
        }
    }
}
