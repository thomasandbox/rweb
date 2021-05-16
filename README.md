# RWEB

> 2021/5/16  
> RWEB has a simple CRUD application.  
> That means this still needs refactoring.

## Tech Stacks

### Server

- Rust
- Actix-web
- Diesel
- PostgreSQL

Each version is written on [Cargo.toml](./Cargo.toml).

### Client

- React
- Next.js
- TypeScript

Each version is written on [package.json](./package.json).

## Prerequisites

### Server

- Rust
- PostgreSQL

### Client

- Node.js
- Yarn

## Development

### Server

Create `.env` on the root directory to connect to the PostgreSQL database, then write DATABASE_URL that looks like this.

```
DATABASE_URL=postgres://username:password@localhost/todos
```

Install `diesel_cli` by using `cargo install diesel_cli --no-default-features --features postgres`, then execute `diesel setup` and `diesel migration run` to setup database things.

Execute `cargo run`, then go to `http://localhost:8080/todos` to see all the todos data.

### Client

Execute `yarn` and `yarn dev`, then go to `http://localhost:3000/`.
