# Advanced Rust Discord Bot

This is an advanced Discord bot written in Rust, designed to provide a solid foundation for building complex bot functionalities. It supports slash commands and includes features such as database integration, metrics, caching, and more.

## Features

- Slash command system
- PostgreSQL integration via SQLx
- Metrics tracking
- In-memory caching
- Asynchronous task management
- Rate limiting
- Guild-specific data management
- Advanced error handling
- Configurable logging
- Localization support

## Prerequisites

Ensure you have the following installed before proceeding:

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/)
- [Discord Bot Token](https://discord.com/developers/applications)

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/savisxss/advanced-rust-discord-bot.git
    cd advanced-rust-discord-bot
    ```

2. **Set up the environment variables**:
    Copy the example environment configuration file and update it with your Discord bot token and database URL:
    ```bash
    cp .env.example .env
    ```
    Edit the `.env` file with your own credentials.

3. **Configure the bot**:
    Update the `config.toml` file to match your bot's configuration.

4. **Set up the database**:
    Create the PostgreSQL database and run the migrations:
    ```bash
    psql -c "CREATE DATABASE botdb;"
    sqlx database create
    sqlx migrate run
    ```

5. **Build and run the bot**:
    Run the bot using Cargo:
    ```bash
    cargo run
    ```

## Usage

Once the bot is running, you can interact with it using slash commands in your Discord server. The bot comes with basic commands like `/ping` and `/help`. You can extend its functionality by adding more commands in the `commands` module.

## Project Structure

- **`src/main.rs`**: Entry point of the application
- **`src/bot/`**: Core bot functionality
  - `mod.rs`: Main bot module with interaction handling
  - `handler.rs`: Event handler for Discord events
  - `error.rs`: Error types and handling
- **`src/commands/`**: Command implementations
- **`src/config/`**: Configuration management
- **`src/database/`**: Database models and operations
- **`src/lang/`**: Localization system
- **`src/utils/`**: Utility modules
  - `metrics.rs`: Metrics tracking system
  - `cache.rs`: In-memory caching system
  - `task_manager.rs`: Asynchronous task management
  - `rate_limiter.rs`: Rate limiting implementation
  - `guild_data.rs`: Guild-specific data management
  - `logger.rs`: Configurable logging system
  - `embed_builder.rs`: Embed message builder

## Extending the Bot

### Adding New Commands

1. Create a new file in the `commands` directory.
2. Implement the `Command` trait for your new command.
3. Register the command in `commands/mod.rs` within the `CommandHandler::register_commands` method.

### Adding New Features

1. Utilize existing systems (e.g., metrics, cache, task manager) as needed.
2. Extend the `Bot` struct in `bot/mod.rs` if new fields are required.
3. Update event handlers in `bot/handler.rs` to incorporate new functionality.
4. Add new language strings to `lang/en.toml` and other localization files, if necessary.

## Contributing

Contributions are welcome! If you would like to improve the project, feel free to open a pull request.

## License

This project is licensed under the MIT License. For more information, see the [LICENSE](LICENSE) file.