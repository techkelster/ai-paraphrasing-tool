# AI Paraphrasing Tool

This project is a text paraphrasing tool that uses the Gemini AI API. It consists of a React TypeScript frontend and a Rust backend.

## Features

- Interactive text editor
- Select and paraphrase text with AI
- Simple and intuitive UI
- Secure API key management

## Project Structure

- `/frontend`: React TypeScript frontend (built with Vite)
- `/backend`: Rust backend with Actix-web

## Prerequisites

- Node.js 18+ and npm
- Rust and Cargo
- Gemini AI API key

## Setup

### Frontend

1. Navigate to the frontend directory:
   ```
   cd frontend
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Start the development server:
   ```
   npm run dev
   ```

### Backend

1. Navigate to the backend directory:
   ```
   cd backend
   ```

2. Create a `.env` file in the backend directory with the following content:
   ```
   GEMINI_API_KEY=your_gemini_api_key_here
   HOST=0.0.0.0
   PORT=8080
   ```

3. Replace `your_gemini_api_key_here` with your actual Gemini API key.

4. Run the backend:
   ```
   cargo run
   ```

## Usage

1. Open your browser and navigate to `http://localhost:5173`
2. Type or paste text into the editor
3. Select any portion of text you want to paraphrase
4. Click the "Paraphrase" button that appears
5. The selected text will be replaced with the AI-generated paraphrased version

## Security Note

The API key is stored in the backend's `.env` file and is never exposed to the frontend. All API requests are made through the backend to ensure the key remains secure. 