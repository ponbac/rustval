use std::fs;

use openapiv3::OpenAPI;

fn print_openapi() {
    let data = fs::read_to_string("data/externaldata.swagger.yaml").expect("Could not read file");
    let openapi: OpenAPI = serde_yaml::from_str(&data).expect("Could not deserialize input");
    println!("{:#?}", openapi.paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        print_openapi();
    }
}
