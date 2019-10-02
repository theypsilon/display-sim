FROM debian:buster-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="version_81" \
    RUST_TOOLCHAIN="1.38.0" \
    RUSTUP_VER="1.19.0"
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
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='36285482ae5c255f2decfab27d32ba19465804cb3ddf5a23e6ff2a7b0f6e0250' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='cb20e54566d4b13434dea1776a961cf7f97afcc292cb4b0fec533503dd2434d0' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='58e19ae12101103ccc50b04a2579b9238163f87a27da5078cefc900098f257ab' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='d3c42fb8b25f87eb049b6177611eea7d4fd51273de4113706f43cccf5610cfc7' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/${RUSTUP_VER}/${rustArch}/rustup-init"; \
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
    && npm run build

FROM nginx:1.15.10-alpine
RUN addgroup -g 82 -S www-data \
    && adduser -u 82 -D -S -G www-data www-data
ADD nginx/h5bp/ /etc/nginx/h5bp/
ADD nginx/* /etc/nginx/
COPY --from=webpack-artifact /www/dist/* /var/www/html/
CMD ["nginx","-g", "daemon off;"]
