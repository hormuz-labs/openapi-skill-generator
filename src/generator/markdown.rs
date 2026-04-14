use crate::schema::parser::ApiSpec;
use std::collections::HashMap;

/// Represents a grouped controller with its endpoints
pub struct Controller {
    pub endpoints: Vec<EndpointConfig>,
}

pub struct EndpointConfig {
    pub path: String,
    pub method: String,
    pub summary: Option<String>,
    pub description: Option<String>,
}

/// Parses the relaxed ApiSpec and groups paths by tags (Controllers)
pub fn generate_markdown_files(schema: &ApiSpec) -> HashMap<String, String> {
    let mut files = HashMap::new();
    let mut controllers: HashMap<String, Controller> = HashMap::new();

    // Group endpoints by Tag
    for ep in &schema.endpoints {
        let tag_name = ep
            .tags
            .first()
            .cloned()
            .unwrap_or_else(|| "Default".to_string());

        let controller = controllers.entry(tag_name).or_insert_with(|| Controller {
            endpoints: Vec::new(),
        });

        controller.endpoints.push(EndpointConfig {
            path: ep.path.clone(),
            method: ep.method.clone(),
            summary: ep.summary.clone(),
            description: ep.description.clone(),
        });
    }

    // Generate main SKILL.md
    let slugified_title: String = schema
        .title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    let safe_desc = schema
        .description
        .as_deref()
        .unwrap_or("API skill generated from OpenAPI schema.")
        .replace('\n', " ")
        .replace('"', "'");

    let mut skill_md = format!(
        "---\n\
name: {slug}\n\
description: {desc}\n\
license: Complete terms in LICENSE.txt\n\
---\n\n\
# {title} API Skill\n\n\
Use this skill to interact with the {title} API. When the user requests an action, identify the appropriate controller from the list below and use your tools to read the file and understand the endpoints, required parameters, and payloads.\n\n\
## Available Controllers\n\n",
        slug = slugified_title,
        desc = safe_desc,
        title = schema.title
    );

    for name in controllers.keys() {
        skill_md.push_str(&format!("- [{name} Controller](./{name}.md)\n"));
    }
    files.insert("SKILL.md".to_string(), skill_md);

    // Generate individual Controller markdown files
    for (name, controller) in controllers {
        let mut ctrl_md = format!("# {} Controller\n\n", name);

        for endpoint in controller.endpoints {
            ctrl_md.push_str(&format!("## {} {}\n\n", endpoint.method, endpoint.path));

            if let Some(summary) = &endpoint.summary {
                ctrl_md.push_str(&format!("**Summary:** {}\n\n", summary));
            }

            if let Some(desc) = &endpoint.description {
                ctrl_md.push_str(&format!("{}\n\n", desc));
            }

            ctrl_md.push_str("---\n\n");
        }

        files.insert(format!("{}.md", name), ctrl_md);
    }

    files
}
