FROM debian:stretch-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="1.38.13"
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain none; \
    rm rustup-init; \
    rustup toolchain install beta; \
    rustup default beta; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    wget -O- https://rustwasm.github.io/wasm-pack/installer/init.sh | sh; \
    wget -qO- https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-x86-linux.tar.gz | tar xvz binaryen-${BINARYEN_VER}/wasm-opt ; \
    mv binaryen-${BINARYEN_VER}/wasm-opt .; \
    apt-get remove -y --auto-remove wget; \
    rm -rf /var/lib/apt/lists/*; \
    bash -c 'rm -rf ${RUSTUP_HOME}/toolchains/*/share'

FROM rust-wasm as wasm-artifact
ENV RUST_BACKTRACE=1
ADD Cargo.* /app/
RUN mkdir -p src && touch src/lib.rs \
    && cargo build --release \
    && wasm-pack build --debug \
    && bash -c 'rm -rf ${CARGO_HOME}/registry/src/*/*/{!Cargo.toml}' \
    && rm -rf target/debug target/wasm32-unknown-unknown/debug
ADD src src
RUN cargo test --release \
    && wasm-pack build \
    && mkdir -p /wasm && cp -r pkg/crt_3d_sim* /wasm/ \
    && ./wasm-opt --debug -O3 -o ../wasm/crt_3d_sim_bg.wasm ../wasm/crt_3d_sim_bg.wasm >/dev/null 2>&1 \
    && cargo clean \
    && rm -rf /app

FROM node:8.12.0-alpine as webpack-artifact
WORKDIR /www
ADD www/package*.json .
RUN npm install --dev
ADD www .
COPY --from=wasm-artifact /wasm/* ./
RUN npm run build \
    && cp *.css dist/ \
    && cp favicon.ico dist/ \
    && mkdir -p dist/assets/ && cp -r assets/ dist/assets/

FROM nginx:1.14.0-alpine
COPY --from=webpack-artifact /www/dist/* /var/www/html/
ADD nginx/* /etc/nginx/
CMD ["nginx","-g", "daemon off;"]