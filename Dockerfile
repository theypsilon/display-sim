FROM debian:stretch-slim as rust-wasm
WORKDIR /app
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    BINARYEN_VER="1.38.13"
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
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='0077ff9c19f722e2be202698c037413099e1188c0c233c12a2297bf18e9ff6e7' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='f139e5be4ea2db7ff151c122f5d24af3c587c4fc74a7414e262cb34403278ad3' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='c7d5471e71a315134e7499af75eb177d1f574858f1c6b8e61b436702d671a4e2' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='909ce4e2d0c9bf60ba5a85426c38cceb5ae77979ab2b1e354e76b9851b5ec5ed' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain 1.30.0; \
    rm rustup-init; \
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