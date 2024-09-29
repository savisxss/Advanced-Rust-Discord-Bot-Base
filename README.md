# Advanced Rust Discord Bot

This project is an advanced Discord bot written in Rust, designed to provide a solid foundation for building complex bot functionalities. It includes features such as command handling, database integration, multi-language support, and more.

## Features

- Modular command system
- PostgreSQL database integration with SQLx
- Multi-language support
- Configuration management with TOML
- Advanced error handling
- Embed message builder
- Slash command support

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Discord Bot Token

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/advanced-rust-discord-bot.git
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

Once the bot is running, you can use slash commands in your Discord server. Use `/help` to see a list of available commands.

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/bot/`: Core bot functionality
- `src/commands/`: Command implementations
- `src/config/`: Configuration management
- `src/database/`: Database models and operations
- `src/lang/`: Language files for internationalization
- `src/utils/`: Utility functions and helpers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
