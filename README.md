# Advanced Rust Discord Bot

This project is an advanced Discord bot written in Rust, designed to provide a solid foundation for building complex bot functionalities. It includes features such as slash command handling, database integration, metrics, caching, and more.

## Features

- Slash command system
- PostgreSQL database integration with SQLx
- Metrics tracking
- In-memory caching
- Asynchronous task management
- Rate limiting
- Guild-specific data management
- Advanced error handling
- Configurable logging system

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Discord Bot Token

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/savisxss/advanced-rust-discord-bot.git
   cd advanced-rust-discord-bot
   ```

2. Copy the `.env.example` file to `.env` and fill in your Discord bot token and database URL:
   ```
   cp .env.example .env
   ```

3. Edit the `config.toml` file to match your Discord server setup.

4. Set up the database:
   ```
   psql -c "CREATE DATABASE botdb;"
   sqlx database create
   sqlx migrate run
   ```

5. Build and run the bot:
   ```
   cargo run
   ```

## Usage

Once the bot is running, you can use slash commands in your Discord server. The bot comes with basic commands like `/ping` and `/help`. You can extend the functionality by adding more commands in the `commands` module.

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/bot/`: Core bot functionality
  - `mod.rs`: Main bot module with interaction handling
  - `handler.rs`: Event handler for Discord events
  - `error.rs`: Error types and handling
- `src/commands/`: Command implementations
- `src/config/`: Configuration management
- `src/database/`: Database models and operations
- `src/utils/`: Utility modules
  - `metrics.rs`: Metrics tracking system
  - `cache.rs`: In-memory caching system
  - `task_manager.rs`: Asynchronous task management
  - `rate_limiter.rs`: Rate limiting implementation
  - `guild_data.rs`: Guild-specific data management
  - `logger.rs`: Configurable logging system

## Extending the Bot

To add new commands:
1. Create a new file in the `commands` directory.
2. Implement the command logic.
3. Register the command in `bot/mod.rs` in the `handle_interaction` method.

To add new features:
1. Utilize the existing systems (metrics, cache, task manager, etc.) as needed.
2. Extend the `Bot` struct in `bot/mod.rs` if new fields are required.
3. Update the event handlers in `bot/handler.rs` to incorporate new functionality.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.