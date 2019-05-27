FROM debian:stretch-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="1.38.29" \
    RUST_TOOLCHAIN="1.35.0"
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
    rustup component add clippy; \
    wget -qO- https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-x86_64-linux.tar.gz | tar xvz binaryen-${BINARYEN_VER}/wasm-opt ; \
    mv binaryen-${BINARYEN_VER}/wasm-opt /usr/bin/; \
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

FROM node:8.12.0-alpine as webpack-artifact
WORKDIR /www
ADD www/package*.json ./
RUN npm install
ADD www .
COPY --from=wasm-artifact /wasm ./src/wasm
RUN npm test \
    && npm run build

FROM nginx:1.15.10-alpine
RUN addgroup -g 82 -S www-data \
    && adduser -u 82 -D -S -G www-data www-data
ADD nginx/h5bp/ /etc/nginx/h5bp/
ADD nginx/* /etc/nginx/
COPY --from=webpack-artifact /www/dist/* /var/www/html/
CMD ["nginx","-g", "daemon off;"]
