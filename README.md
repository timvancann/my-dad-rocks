# My Dad Rocks

## Install dev dependencies

```bash
cargo install cargo-leptos sqlx-cli
npm install -D tailwindcss prettier prettier-plugin-tailwindcss @tailwindcss/typography daisyui@latest
```

### Running the project in development mode

Run and prepare the database
```bash
docker-compose up db -d
sqlx mirgrate run
```

Compile tailwindcss and watch for changes
```bash
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

Run the server and watch for changes
```bash
DATABASE_URL=postgres://postgrse:tits@localhost:5432/mydadrocks cargo leptos watch
```

## Building the project (make sure the database is running)
```bash
cargo sqlx prepare
docker-compose build
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need
to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Dev

```bash
cargo install leptosfmt
leptosfmt .
```