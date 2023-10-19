FROM rust:1.73-buster as rust-wasm
WORKDIR /app
ARG RUST_TOOLCHAIN="1.73.0"
RUN set -eux; \
    apt-get update ; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        pkgconf \
        libsdl2-dev \
        cmake \
        curl \
        build-essential \
        libssl-dev \
    ; \
    rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}; \
    rustup component add clippy; \
    cargo install wasm-pack

FROM rust-wasm as wasm-artifact
ENV RUST_BACKTRACE=1
ADD rust/ /app/rust/
ADD Cargo.* /app/
ADD scripts/build.sh /app/scripts/build.sh
ARG BUILD_WASM_PARAMS="--release-wasm"
RUN cargo test --all \
    && ./scripts/build.sh ${BUILD_WASM_PARAMS} \
    && cp -r /app/www/src/wasm /wasm

FROM node:18-buster as webpack-artifact
WORKDIR /www
ADD www/package*.json ./
RUN npm install
ADD www .
COPY --from=wasm-artifact /wasm ./src/wasm
RUN npm test && npm run build

FROM nginx:1.22.0-alpine
RUN adduser -u 82 -D -S -G www-data www-data
ENV NGINX_ENTRYPOINT_QUIET_LOGS=1
ADD nginx/h5bp/ /etc/nginx/h5bp/
ADD nginx/* /etc/nginx/
COPY --from=webpack-artifact /www/dist/* /var/www/html/
CMD ["nginx","-g", "daemon off;"]
