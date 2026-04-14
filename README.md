# OpenAPI Skill Generator 🚢

**OpenAPI Skill Generator** is a web-based utility designed to safely escort your massive OpenAPI payloads through the narrow chokepoint of LLM context windows. It converts OpenAPI/Swagger schemas (.json, .yaml) into streamlined Agent Skills that can be downloaded as a zip archive.

## Features

- **Schema Conversion:** Upload an OpenAPI schema file, and the application will extract and package the endpoints into individual Agent Skills.
- **Web Interface:** A sleek, modern web UI built with Tailwind CSS, drag-and-drop file upload, and Lottie animations.
- **Fast and Safe:** Built purely in Rust using the Axum web framework to ensure fast execution, concurrency, and memory safety.

## Tech Stack

- **Backend:** Rust, Axum, Tokio, Tower HTTP, OpenAPIv3, Zip, Serde
- **Frontend:** HTML, Tailwind CSS (via CDN), Vanilla JS, Lottie Web

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.80+ recommended)
- Cargo (comes with Rust)

### Installation & Running

1. **Clone the repository:**
   ```bash
   git clone <your-repository-url>
   cd swagger2skills
   ```

2. **Run the server:**
   ```bash
   cargo run
   ```

3. **Access the application:**
   Open your browser and navigate to `http://127.0.0.1:3000`.

## Usage

1. Open the application in your browser.
2. Drag and drop your OpenAPI `.json` or `.yaml` file into the upload zone, or click to browse.
3. Click **"Generate Skills Zip"**.
4. Once processing is complete, a `skills.zip` file containing your new Agent Skills will be downloaded automatically.

## API Endpoints

- `GET /health` - Health check endpoint.
- `POST /api/convert` - Upload an OpenAPI schema (multipart/form-data with a `schema` field) and receive a zip file of generated skills.

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
