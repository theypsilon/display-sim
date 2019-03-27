FROM debian:stretch-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="1.38.13" \
    RUST_TOOLCHAIN="1.33.0"
RUN set -eux; \
    apt-get update || true; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        ; \
    \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='02c0464459b2f88ce99f927b14f6aa4d09c96b9eb6e57808d6c567edce66260a' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='c7a3094b5e81974a5f752c3d6d78f0202e9ee45962140167880a2e0fe5bb3eb7' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='d1d6ca6c91fa5c22a53f9c7a79dbc49ac2c9056e2d74636e8f091310f157e351' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='63dd42cdc70b9b026a86d514be4392ab24110ae4537285b5d04e98cdc2cf27d1' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.15.0/${rustArch}/rustup-init"; \
    wget -q "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain ${RUST_TOOLCHAIN}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    wget -qO- https://rustwasm.github.io/wasm-pack/installer/init.sh | sh; \
    rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}; \
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
ADD www/package*.json ./
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