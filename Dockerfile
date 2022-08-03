FROM rust:latest
EXPOSE 8080
VOLUME ["/code"]
RUN rutup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
WORKDIR /code
COPY ["trunk", "serve"]
