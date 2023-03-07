FROM rust:alpine

RUN apk add --update libc-dev

WORKDIR /app
COPY docs/examples /root/.config/qas

# cache deps by faking a main.rs file, and remove it after done.
COPY Cargo.toml Cargo.lock .
RUN mkdir src; echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm src/main.rs

COPY . .

CMD ["cargo", "test"]
