FROM debian:buster-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
ARG RUST_TOOLCHAIN="1.52.0" \
    RUSTUP_VER="1.24.1" \
    RUSTUP_ARCH="x86_64-unknown-linux-gnu" \
    RUSTUP_SHA="fb3a7425e3f10d51f0480ac3cdb3e725977955b2ba21c9bdac35309563b115e8" \
    BINARYEN_VER="version_101" \
    BINARYEN_ARCH="x86_64-linux" \
    BINARYEN_SHA="20d0b19ca716c51d927f181802125f04d5685250c8a22ec3022ac28bf4f20c57"
RUN set -eux; \
    apt-get update || true; \
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
    wget -q "https://static.rust-lang.org/rustup/archive/${RUSTUP_VER}/${RUSTUP_ARCH}/rustup-init"; \
    echo "${RUSTUP_SHA} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain ${RUST_TOOLCHAIN}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    cargo install wasm-pack; \
    rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}; \
    rustup component add clippy; \
    wget -q -O binaryen.tar.gz "https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-${BINARYEN_ARCH}.tar.gz"; \
    echo "${BINARYEN_SHA} *binaryen.tar.gz" | sha256sum -c -; \
    tar xf binaryen.tar.gz binaryen-${BINARYEN_VER}/bin/wasm-opt ; \
    mv binaryen-${BINARYEN_VER}/bin/wasm-opt /usr/bin/; \
    rm -rf binaryen* ; \
    apt-get remove -y --auto-remove wget; \
    rm -rf /var/lib/apt/lists/*; \
    bash -c 'rm -rf ${RUSTUP_HOME}/toolchains/*/share'

FROM rust-wasm as wasm-artifact
ENV RUST_BACKTRACE=1
ADD rust/ /app/rust/
ADD Cargo.* /app/
ADD scripts/ /app/scripts/
ARG BUILD_WASM_PARAMS="--release-wasm"
RUN ./scripts/test.sh --rust-only \
    && ./scripts/build.sh ${BUILD_WASM_PARAMS} \
    && cargo clean \
    && cp -r /app/www/src/wasm /wasm \
    && rm -rf /app

FROM node:16.1.0-alpine3.12 as webpack-artifact
WORKDIR /www
ADD www/package*.json ./
RUN npm install
ADD www .
COPY --from=wasm-artifact /wasm ./src/wasm
RUN npm test \
    && npm run lint \
    && npm run build

FROM nginx:1.20.0-alpine
RUN adduser -u 82 -D -S -G www-data www-data
ADD nginx/h5bp/ /etc/nginx/h5bp/
ADD nginx/* /etc/nginx/
COPY --from=webpack-artifact /www/dist/* /var/www/html/
CMD ["nginx","-g", "daemon off;"]
