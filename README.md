# Photosnipe


This is a simple web app to display photos of other people. It's really pretty
silly.



# Technology
- Frontend: Svelte with TypeScript
- Backend: Rust (Rocket framework)
- Database: MongoDB
- Authentication: authenticate using Discord account with Auth0

# Setup

## Prerequisites
- Rust (latest stable)
- Node.js and pnpm
- MongoDB (local or remote instance)
- Discord Bot Token

## Backend Setup

1. Install MongoDB locally or use a cloud instance (MongoDB Atlas)
2. Create a `.env` file in the project root with:
   ```
   MONGODB_URI=mongodb://localhost:27017
   MONGODB_DB=photosnipe
   DISCORD_BOT_TOKEN=your_discord_bot_token_here
   ```
3. Navigate to the backend directory:
   ```
   cd backend
   ```
4. Build and run:
   ```
   cargo run
   ```

## Frontend Setup

1. Navigate to the frontend directory:
   ```
   cd frontend
   ```
2. Install dependencies:
   ```
   pnpm install
   ```
3. Run development server:
   ```
   pnpm dev
   ```

## Database Structure

The application uses MongoDB with two collections:

### `persons` collection
- `id` (i64): Discord user ID
- `username` (String): Discord username
- `display_name` (Option<String>): Discord display name/nickname

### `snipes` collection
- `sniper_id` (i64): Discord ID of the person who posted
- `snipee_id` (i64): Discord ID of the person mentioned
- `picture_url` (String): URL of the image
- `text` (Option<String>): Message text
- `channel_id` (i64): Discord channel ID
- `guild_id` (Option<i64>): Discord guild/server ID
