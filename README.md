# Rustle - social media application written in pure Rust

This is a web application built with Rust.

## Prerequisites

- Rust and Cargo (latest stable version)
- Docker and Docker Compose
- PostgreSQL (via Docker)

## Installation

1. Install required global binaries:

   ```bash
   cargo install cargo-watch
   cargo install sqlx
   ```

## Database Setup

1. Start the PostgreSQL database using Docker:

   ```bash
   docker-compose up -d
   ```

2. Run database migrations:

   ```bash
   sqlx migrate run
   ```

## Running the Application

1. Start the development server with hot reload:

   ```bash
   cargo-watch -r "run --bin rustle" -w ./src
   ```

## Database Seeding

To populate the database with initial data:

```bash
cargo run --bin seed
```
