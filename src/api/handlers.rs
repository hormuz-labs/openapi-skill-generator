use axum::{
    extract::Multipart,
    http::{header, HeaderMap},
    response::IntoResponse,
};

use crate::{
    error::AppError,
    generator::{archive::create_zip_archive, markdown::generate_markdown_files},
    schema::parser::parse_schema,
};

#[allow(dead_code)]
pub async fn health_check() -> impl IntoResponse {
    "OK"
}

pub async fn convert_schema(mut multipart: Multipart) -> Result<impl IntoResponse, AppError> {
    let mut file_data = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Multipart(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "schema" {
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::Multipart(e.to_string()))?;
            file_data.extend_from_slice(&data);
            break;
        }
    }

    if file_data.is_empty() {
        return Err(AppError::Parse("No 'schema' file found in request".into()));
    }

    // 1. Parse Schema
    let schema = parse_schema(&file_data)?;

    // 2. Generate Markdown files
    let files = generate_markdown_files(&schema);

    // 3. Create ZIP Archive
    let zip_bytes = create_zip_archive(files)?;

    // 4. Return as downloadable file
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/zip".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"skills.zip\"".parse().unwrap(),
    );

    Ok((headers, zip_bytes))
}
