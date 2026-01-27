# Axum CLI

CLI tool for [axum_template](https://github.com/neokofg/axum_template) project scaffolding and development.

## Installation

```bash
cargo install --path .
```

## Commands

### Create New Project

```bash
axum new <project-name>
axum new my-app --template /path/to/template
```

### Generate Components

Generate various project components. Use `generate` or shorthand `g`:

```bash
# Generate a new feature module
axum generate feature <name>
axum g feature posts

# Generate a database migration
axum generate migration <name>
axum g migration create_posts

# Generate a model
axum generate model <name> --feature <feature> --field <fields...>
axum g model Post --feature posts --field title:String --field content:Text

# Generate a handler
axum generate handler <name> --feature <feature>
axum g handler list --feature posts

# Generate CRUD for a feature (model + handlers + migration)
axum generate crud <name> --field <fields...>
axum g crud posts --field title:String --field content:Text

# Generate a background worker
axum generate worker <name>
axum g worker email_sender

# Generate a middleware
axum generate middleware <name>
axum g middleware rate_limiter
```

### Database Operations

```bash
# Run pending migrations
axum db migrate

# Rollback migrations
axum db rollback
axum db rollback --step 3

# Reset database (drop + create + migrate)
axum db reset
axum db reset --force

# Create database
axum db create

# Drop database
axum db drop
axum db drop --force

# Show migration status
axum db status

# Generate schema.rs from database
axum db schema

# Seed database with test data
axum db seed
```

### Docker Operations

```bash
# Start containers
axum docker up
axum docker up --detach

# Stop containers
axum docker down
axum docker down --volumes

# Show container status
axum docker status

# View logs
axum docker logs
axum docker logs postgres
axum docker logs --follow

# Restart containers
axum docker restart
```

### Development

```bash
# Run dev server with auto-reload
axum dev
axum dev --port 8080
```

### Testing

```bash
# Run all tests
axum test

# Run tests with filter
axum test user

# Run only unit tests
axum test --unit

# Run only integration tests
axum test --integration
```

### Code Quality

```bash
# Check code (clippy + fmt check)
axum check

# Check and auto-fix
axum check --fix

# Format code
axum fmt

# Check formatting only
axum fmt --check
```

### Build

```bash
# Build project
axum build

# Build in release mode
axum build --release
```

### Project Info

```bash
# List all features
axum features

# Show project info
axum info
```

## License

MIT
