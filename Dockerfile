FROM rust:1.47
WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT ["./target/release/apratemplate"]