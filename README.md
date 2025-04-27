# Rustle - social media application written in pure Rust

This is a web application built with Rust.

## Prerequisites

- Rust and Cargo (latest stable version)
- Docker and Docker Compose
- PostgreSQL (via Docker)

## Installation

Install required global binaries:

```bash
cargo install cargo-watch
cargo install sqlx-cli
```

## Set up environment variables

Copy `.env.example` file into `.env`, and set up environment variables based on your own credentials

```bash
cp .env.example .env
```

## Database Setup

Start the PostgreSQL database using Docker:

```bash
docker-compose up -d
```

Run database migrations:

```bash
sqlx migrate run
```

## Running the Application

Start the development server with hot reload:

```bash
cargo-watch -r "run --bin rustle" -w ./src
```

## Database Seeding

To populate the database with initial data:

```bash
cargo run --bin seed
```
