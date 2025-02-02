# llmproxify

`llmproxify` is a reverse proxy server designed to route requests to various Large Language Model (LLM) providers such as OpenAI, Anthropic, and others. The application runs as an HTTP server and proxies requests based on the specified provider and endpoint. Deploy `llmproxify` in regions which are not blocked by those LLM providers, allowing you to bypass regional restrictions. It is tested with Google Cloud Run.

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
   - **Service Name**: Enter a unique service name or use the default value provided, e.g. `llmproxify:0.1.3`.
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