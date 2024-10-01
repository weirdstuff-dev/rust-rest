FROM rust

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .

RUN ls -l

#RUN cargo install --path .
RUN cargo build --release

CMD bash -c "diesel migration run && cargo run --release"
