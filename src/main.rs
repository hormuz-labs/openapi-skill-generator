mod api;
mod error;
mod generator;
mod schema;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "openapi-skill-generator")]
#[command(about = "Generate Agent Skills from OpenAPI/Swagger schemas", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value_t = 3000)]
        port: u16,

        /// Address to bind to
        #[arg(short, long, default_value = "0.0.0.0")]
        address: String,
    },
    /// Convert a schema file directly via CLI
    Convert {
        /// Path to the OpenAPI schema (.json or .yaml)
        #[arg(short, long)]
        input: PathBuf,

        /// Output path for the generated zip file
        #[arg(short, long, default_value = "skills.zip")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Server { port, address }) => {
            run_server(address, port).await?;
        }
        Some(Commands::Convert { input, output }) => {
            run_convert(input, output).await?;
        }
        None => {
            // Default to server if no subcommand is provided
            run_server("0.0.0.0".to_string(), 3000).await?;
        }
    }

    Ok(())
}

async fn run_server(address: String, port: u16) -> anyhow::Result<()> {
    use axum::{
        routing::{get, post},
        Router,
    };
    use tower_http::services::ServeDir;

    let addr = format!("{}:{}", address, port);
    println!("🚢 OpenAPI Skill Generator server starting on http://{}", addr);

    let app = Router::new()
        .route("/health", get(api::handlers::health_check))
        .route("/api/convert", post(api::handlers::convert_schema))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_convert(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Read;

    println!("📂 Reading schema from: {:?}", input);
    let mut file = File::open(&input)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    println!("⚙️ Generating skills...");
    let zip_data = generator::process_openapi_to_zip(&content).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("💾 Saving zip to: {:?}", output);
    std::fs::write(output, zip_data)?;

    println!("✅ Done!");
    Ok(())
}
