use std::collections::HashMap;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::AppError;

/// Creates an in-memory ZIP archive from a map of filenames to string content.
pub fn create_zip_archive(files: HashMap<String, String>) -> Result<Vec<u8>, AppError> {
    let mut buffer = Cursor::new(Vec::new());

    {
        let mut zip = ZipWriter::new(&mut buffer);
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for (filename, content) in files {
            zip.start_file(filename, options)
                .map_err(|e| AppError::Archive(format!("Failed to start file in zip: {}", e)))?;

            zip.write_all(content.as_bytes())
                .map_err(|e| AppError::Archive(format!("Failed to write to zip: {}", e)))?;
        }

        zip.finish()
            .map_err(|e| AppError::Archive(format!("Failed to finish zip: {}", e)))?;
    }

    Ok(buffer.into_inner())
}
