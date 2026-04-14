pub mod archive;
pub mod markdown;

use crate::error::AppError;
use crate::schema::parser::parse_schema;
use archive::create_zip_archive;
use markdown::generate_markdown_files;

pub fn process_openapi_to_zip(content: &[u8]) -> Result<Vec<u8>, AppError> {
    // 1. Parse Schema
    let schema = parse_schema(content)?;

    // 2. Generate Markdown files
    let files = generate_markdown_files(&schema);

    // 3. Create ZIP Archive
    let zip_bytes = create_zip_archive(files)?;

    Ok(zip_bytes)
}
