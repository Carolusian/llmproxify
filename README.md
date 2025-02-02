# llmproxify

`llmproxify` is a lightweight reverse proxy designed to route requests to various Large Language Model (LLM) providers such as OpenAI, Anthropic, and others. The application runs as an HTTP server and proxies requests based on the specified provider and endpoint. Deploy `llmproxify` in regions which are not blocked by those LLM providers, allowing you to bypass regional restrictions. It is tested with Google Cloud Run.

## Deploying on Google Cloud Run

Google Cloud Run offers a free tier with usage limits that reset each month. You will only be billed for usage that exceeds these limits. For more details, refer to the [Cloud Run Pricing documentation](https://cloud.google.com/run/pricing#tables).

To deploy your container on Cloud Run, follow these steps:

1. **Navigate to Cloud Run**:
   - Go to the [Google Cloud Console](https://console.cloud.google.com/).
   - Navigate to the Cloud Run page.

2. **Deploy a Container**:
   - Click on **Create Service**.
   - Choose **Deploy one revision from an existing container image**.

3. **Configure the Service**:
   - **Container Image URL**: Enter `index.docker.io/carolusian/llmproxify:0.1.3`.
   - **Service Name**: Enter a unique service name or use the default value provided, e.g. `llmproxify`.
   - **Region**: Select `us-central1` or choose another region based on your preference.

4. **Authentication**:
   - Set **Allow unauthenticated invocations** to enable public access.

5. **Deploy**:
   - Click **Create** to deploy the service.
   - Once deployed, the URL for your service will be displayed, e.g., `https://llmproxify-xxxxxxxxxxx.us-central1.run.app`.

6. **Access the Service**:
   - Copy and paste the service URL into your web browser.
   - You should see "Hello, llmproxify!" displayed in the browser.

## Usage Examples

To test the deployed service, you can use tools like `curl` or make HTTP requests from a client application. Please note that the first segment of the URI after `https://llmproxify-xxxxxxxxxxx.us-central1.run.app` is the configured LLM provider. The following LLM providers and their HTTP domain entries are pre-configured:

```json
{
    "gemini": "https://generativelanguage.googleapis.com/",
    "anthropic": "https://api.anthropic.com/",
    "openai": "https://api.openai.com/",
    "groq": "https://api.groq.com/",
    "cerebras": "https://api.cerebras.ai/"
}
```

Here are some examples using `curl`:

### Gemini

```bash
curl "https://llmproxify-xxxxxxxxxxx.us-central1.run.app/gemini/v1beta/models/gemini-1.5-flash:generateContent?key=${YOUR_API_KEY}" \
-H 'Content-Type: application/json' \
-X POST \
-d '{
  "contents": [{
    "parts":[{"text": "Write a story about a magic backpack."}]
    }]
   }'
```

### Anthropic

```bash
curl https://llmproxify-xxxxxxxxxxx.us-central1.run.app/anthropic/v1/messages \
     --header "x-api-key: ${YOUR_ANTHROPIC_API_KEY}" \
     --header "anthropic-version: 2023-06-01" \
     --header "content-type: application/json" \
     --data \
'{
    "model": "claude-3-5-sonnet-20241022",
    "max_tokens": 1024,
    "messages": [
        {"role": "user", "content": "Hello, world"}
    ]
}'
```

### Groq

```bash
curl "https://llmproxify-xxxxxxxxxxx.us-central1.run.app/groq/openai/v1/chat/completions" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${YOUR_GROQ_API_KEY}" \
  -d '{
                  "messages": [{
                        "role": "system",
                        "content": "You are a helpful assistant"
                },
                {
                        "role": "user",
                        "content": "Hello"
                }],
         "model": "llama-3.3-70b-versatile",
         "temperature": 1,
         "max_completion_tokens": 1024,
         "top_p": 1,
         "stop": null
       }'
```

### Cerebras

```bash
curl "https://llmproxify-xxxxxxxxxxx.us-central1.run.app/cerebras/v1/chat/completions" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${YOUR_CEREBRAS_API_KEY}" \
  -d '{
                  "messages": [{
                        "role": "system",
                        "content": "You are a helpful assistant"
                },
                {
                        "role": "user",
                        "content": "Hello"
                }],
         "model": "llama3.1-8b",
         "temperature": 1,
         "max_completion_tokens": 1024,
         "top_p": 1,
         "stop": null
       }'
```

### OpenAI

```bash
curl "https://llmproxify-xxxxxxxxxxx.us-central1.run.app/openai/v1/chat/completions" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $YOUR_OPENAI_API_KEY" \
    -d '{
        "model": "gpt-4o-mini",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "Write a haiku that explains the concept of recursion."
            }
        ]
    }'
```

Make sure to replace `${YOUR_XXXXX_API_KEY}` with your actual API key for each provider.

If you are using the libraries provided by the LLM providers, ensure that you have updated the endpoint base URLs as follows:

- **Gemini**: `https://llmproxify-xxxxxxxxxxx.us-central1.run.app/gemini/v1beta`
- **Anthropic**: `https://llmproxify-xxxxxxxxxxx.us-central1.run.app/anthropic/v1`
- **OpenAI**: `https://llmproxify-xxxxxxxxxxx.us-central1.run.app/openai/v1`
- **Groq**: `https://llmproxify-xxxxxxxxxxx.us-central1.run.app/groq/openai/v1`
- **Cerebras**: `https://llmproxify-xxxxxxxxxxx.us-central1.run.app/cerebras/v1`

## Standalone Deployment

This project is developed using [Rust](https://www.rust-lang.org/), and it can be deployed as a standalone service. Below are the steps to build and run the project:

### Using Cargo

1. **Build the Project:**
   - To build the project in release mode, execute the following command:
     ```bash
     cargo build --release
     ```
   - This will compile the project and generate an optimized binary in the `target/release` directory.

2. **Run the Project:**
   - After building, you can run the project using:
     ```bash
     cargo run --release
     ```
   - Alternatively, you can directly run the binary from the `target/release` directory:
     ```bash
     ./target/release/your_project_name
     ```

### Using Docker

1. **Build and Run with Docker Compose:**
   - If you prefer using Docker, you can build and run the project using Docker Compose. First, ensure you have Docker and Docker Compose installed.
   - Then, use the following command to start the service in detached mode:
     ```bash
     docker compose up -d
     ```
   - This command will read the `docker-compose.yml` file to set up and run the service.

2. **View Docker Compose Configuration:**
   - For more details on the Docker Compose setup, refer to the [docker-compose.yml](docker-compose.yml) file in the repository.

## Configuration

You can extend the integration with other LLM platforms and configure an HTTP proxy by setting environment variables.

### Environment Variables

- **API_PROVIDERS**: A JSON string defining the API endpoints for additional LLM platforms. For example:
  ```json
  {"cerebras": "https://api.cerebras.ai/", "sambanova": "https://api.sambanova.ai/"}
  ```
- **ALL_PROXY**: The URL for your HTTP proxy.