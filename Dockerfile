FROM debian:buster-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="1.39.1" \
    RUST_TOOLCHAIN="1.43.0" \
    RUSTUP_VER="1.21.1"
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
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='ad1f8b5199b3b9e231472ed7aa08d2e5d1d539198a15c5b1e53c746aad81d27b' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='6c6c3789dabf12171c7f500e06d21d8004b5318a5083df8b0b02c0e5ef1d017b' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='26942c80234bac34b3c1352abbd9187d3e23b43dae3cf56a9f9c1ea8ee53076d' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='27ae12bc294a34e566579deba3e066245d09b8871dc021ef45fc715dced05297' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/${RUSTUP_VER}/${rustArch}/rustup-init"; \
    wget -q "$url"; \
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
    wget -qO- https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-x86-linux.tar.gz | tar xvz binaryen-${BINARYEN_VER}/wasm-opt ; \
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
    && npm run lint \
    && npm run build

FROM nginx:1.15.10-alpine
RUN addgroup -g 82 -S www-data \
    && adduser -u 82 -D -S -G www-data www-data
ADD nginx/h5bp/ /etc/nginx/h5bp/
ADD nginx/* /etc/nginx/
COPY --from=webpack-artifact /www/dist/* /var/www/html/
CMD ["nginx","-g", "daemon off;"]
