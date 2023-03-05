FROM rust:alpine
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build
CMD ["cargo", "test"]
