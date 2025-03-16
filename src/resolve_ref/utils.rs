/// Parse an OpenAPI reference string into its component parts
///
/// ### Examples
///
/// ```
/// # use rustval::resolve_ref::parse_ref;
/// let parts = parse_ref("#/components/schemas/User");
/// assert_eq!(parts, vec!["components", "schemas", "User"]);
/// ```
pub fn parse_ref(reference: &str) -> Vec<String> {
    reference
        .trim_start_matches("#/")
        .split('/')
        .map(String::from)
        .collect()
}
