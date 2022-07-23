FROM rust:1.62.1-slim-buster as rust-wasm
WORKDIR /app
ARG RUST_TOOLCHAIN="1.62.1" \
    BINARYEN_VER="version_109" \
    BINARYEN_ARCH="x86_64-linux" \
    BINARYEN_SHA="c3698aa14d14655f382a5dc73effb6fb3a88b9d03a1ef0acc24cbb1e0f592840"
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
    cargo install wasm-pack; \
    rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}; \
    rustup component add clippy; \
    wget -q -O binaryen.tar.gz "https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-${BINARYEN_ARCH}.tar.gz"; \
    echo "${BINARYEN_SHA} *binaryen.tar.gz" | sha256sum -c -; \
    tar xf binaryen.tar.gz binaryen-${BINARYEN_VER}/bin/wasm-opt ; \
    mv binaryen-${BINARYEN_VER}/bin/wasm-opt /usr/bin/ ; \
    rm -rf binaryen* ; \
    apt-get remove -y --auto-remove wget; \
    rm -rf /var/lib/apt/lists/*

FROM rust-wasm as wasm-artifact
ENV RUST_BACKTRACE=1
ADD rust/ /app/rust/
ADD Cargo.* /app/
ADD scripts/build.sh /app/scripts/build.sh
ARG BUILD_WASM_PARAMS="--release-wasm"
RUN cargo test --all \
    && ./scripts/build.sh ${BUILD_WASM_PARAMS} \
    && cargo clean \
    && cp -r /app/www/src/wasm /wasm \
    && rm -rf /app

FROM node:16.16.0-alpine3.16 as webpack-artifact
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
