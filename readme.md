# Rust REST

### Prerequisites
 - rust & cargo
 - docker & docker-compose
 - `libpq-dev`

### .env
```
DATABASE_URL=postgres://postgres:password@db/brands
```

### Database

You can connect to any database instance, and run the migrations.
Check [diesel](https://diesel.rs/guides/getting-started.html) for more info.
```
diesel migration run
```

### Build & run
```
#dev
cargo run

#release
cargo build --release
```

### Docker & docker-compose for dev
Check [Dockerfile](./Dockerfile) & [docker-compose.yml](./docker-compose.yml)
```
docker-compose -f docker-compose.yml up
docker-compose -f docker-compose.yml down
```