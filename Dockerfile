FROM debian:buster-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="version_97" \
    RUST_TOOLCHAIN="1.48.0" \
    RUSTUP_VER="1.22.1"
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
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='49c96f3f74be82f4752b8bffcf81961dea5e6e94ce1ccba94435f12e871c3bdb' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='5a2be2919319e8778698fa9998002d1ec720efe7cb4f6ee4affb006b5e73f1be' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='d93ef6f91dab8299f46eef26a56c2d97c66271cea60bf004f2f088a86a697078' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='e3d0ae3cfce5c6941f74fed61ca83e53d4cd2deb431b906cbd0687f246efede4' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    rustupUrl="https://static.rust-lang.org/rustup/archive/${RUSTUP_VER}/${rustArch}/rustup-init"; \
    wget -q "${rustupUrl}"; \
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
    binaryenPath="https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VER}/binaryen-${BINARYEN_VER}-x86_64-linux.tar.gz" ; \
    binaryenSha256=$(wget -qO- "${binaryenPath}.sha256" | awk '{print $1}') ; \
    wget -q -O binaryen.tar.gz "${binaryenPath}"; \
    echo "${binaryenSha256} *binaryen.tar.gz" | sha256sum -c -; \
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

FROM node:14.15-alpine3.11 as webpack-artifact
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
