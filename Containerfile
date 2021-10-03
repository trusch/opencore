FROM docker.io/library/rust:1.55-bullseye AS build

RUN rustup component add rustfmt

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY src ./src
COPY proto ./proto
RUN cargo build --release
RUN cargo install --path . --verbose

FROM gcr.io/distroless/cc-debian11:latest AS runtime
COPY --from=build /usr/local/cargo/bin/opencore /bin/opencore
CMD ["/bin/opencore"]