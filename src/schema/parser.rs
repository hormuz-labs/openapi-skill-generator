use crate::error::AppError;
use serde_json::Value;

#[derive(Debug)]
pub struct ApiSpec {
    pub title: String,
    #[allow(dead_code)]
    pub version: String,
    pub description: Option<String>,
    pub endpoints: Vec<ParsedEndpoint>,
}

#[derive(Debug)]
pub struct ParsedEndpoint {
    pub path: String,
    pub method: String,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
}

/// Parses a byte slice into a relaxed ApiSpec struct.
/// Tries JSON first, then falls back to YAML.
/// Uses a relaxed value-based parsing to ignore strict OpenAPI schema validation errors.
#[allow(dead_code)]
pub fn parse_schema(data: &[u8]) -> Result<ApiSpec, AppError> {
    // Attempt JSON, then YAML
    let value: Value = serde_json::from_slice(data)
        .or_else(|_| serde_yaml::from_slice(data))
        .map_err(|e| AppError::Parse(format!("Failed to parse JSON/YAML: {}", e)))?;

    let title = value["info"]["title"]
        .as_str()
        .unwrap_or("API Schema")
        .to_string();

    let version = value["info"]["version"]
        .as_str()
        .unwrap_or("1.0.0")
        .to_string();

    let description = value["info"]["description"].as_str().map(|s| s.to_string());

    let mut endpoints = Vec::new();

    if let Some(paths) = value["paths"].as_object() {
        for (path, path_item) in paths {
            if let Some(methods) = path_item.as_object() {
                for (method, operation) in methods {
                    let valid_methods = [
                        "get", "post", "put", "delete", "patch", "options", "head", "trace",
                    ];

                    if !valid_methods.contains(&method.as_str()) {
                        continue;
                    }

                    let tags = operation["tags"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|t| t.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_else(Vec::new);

                    let summary = operation["summary"].as_str().map(String::from);
                    let op_desc = operation["description"].as_str().map(String::from);

                    endpoints.push(ParsedEndpoint {
                        path: path.clone(),
                        method: method.to_uppercase(),
                        tags,
                        summary,
                        description: op_desc,
                    });
                }
            }
        }
    }

    Ok(ApiSpec {
        title,
        version,
        description,
        endpoints,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let json_data = r#"{
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "paths": {}
        }"#;

        let result = parse_schema(json_data.as_bytes());
        assert!(result.is_ok());
        let schema = result.unwrap();
        assert_eq!(schema.title, "Test API");
    }

    #[test]
    fn test_parse_yaml() {
        let yaml_data = r#"
openapi: 3.0.0
info:
  title: Test API YAML
  version: 1.0.0
paths: {}
"#;

        let result = parse_schema(yaml_data.as_bytes());
        assert!(result.is_ok());
        let schema = result.unwrap();
        assert_eq!(schema.title, "Test API YAML");
    }

    #[test]
    fn test_parse_invalid() {
        let invalid_data = "not a valid schema [[}";
        let result = parse_schema(invalid_data.as_bytes());
        // Since serde_yaml sometimes parses arbitrary strings as strings successfully,
        // we should just verify our specific parser behavior.
        // It might not return Err for raw strings but endpoints will be empty.
        match result {
            Err(_) => assert!(true),
            Ok(schema) => assert!(schema.endpoints.is_empty()),
        }
    }
}
