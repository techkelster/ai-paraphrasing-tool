# Deploying to Shuttle

This guide will help you deploy the backend of your AI paraphrasing tool to Shuttle.

## Prerequisites

1. Install the Shuttle CLI:
   ```bash
   cargo install cargo-shuttle
   ```

2. Create a Shuttle account at https://shuttle.rs/

3. Log in to Shuttle:
   ```bash
   cargo shuttle login
   ```

## Deployment Steps

1. **Project Setup**
   - The backend has been prepared for Shuttle deployment with:
     - `Shuttle.toml` - Shuttle configuration
     - `src/lib.rs` - Shuttle entry point
     - Updated Cargo.toml with Shuttle dependencies

2. **Environment Variables**
   - The API key is read from:
     - Environment variables when deployed
     - The local `.env` file when running locally

3. **Deploy**
   - Navigate to the backend directory and run:
     ```bash
     cargo shuttle deploy --allow-dirty
     ```
   - This will deploy your application to Shuttle

4. **View Deployment**
   - After deployment, Shuttle will provide a URL for your project
   - You can also view your deployments with:
     ```bash
     cargo shuttle status
     ```

## Setting Environment Variables in Shuttle

Once deployed, set your API key as an environment variable in Shuttle:

```bash
cargo shuttle resource create --var GEMINI_API_KEY=your_actual_api_key
```

## Local Development

You can still run the backend locally using:
```bash
cargo run
```

Or with Shuttle's local environment:
```bash
cargo shuttle run
```

## Troubleshooting

- If you get dependency errors, make sure all your Shuttle dependencies are compatible versions
- If CORS issues occur, check that your frontend URL is included in the allowed origins in `src/lib.rs`
- For application errors, check the logs with:
  ```bash
  cargo shuttle logs
  ```

## Frontend Configuration

The frontend has been configured to automatically detect the environment:
- In development, it will use `http://localhost:8080`
- In production, it will use `https://ai-paraphrase-tool.shuttleapp.rs`

No manual configuration is needed for the frontend when deploying.

## Updating Your Frontend

After deploying to Shuttle, update your frontend to point to your new backend URL:

1. Update the API endpoint in `frontend/src/components/TextEditor.tsx`:
   ```typescript
   const response = await axios.post('https://your-shuttle-project.shuttleapp.rs/api/paraphrase', {
     text: selection.text
   });
   ```

2. Replace `your-shuttle-project` with your actual Shuttle project name. 