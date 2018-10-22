FROM liuchong/rustup:stable as rust-wasm
WORKDIR /app
RUN rustup toolchain install beta && rustup default beta \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && curl -sSfL https://github.com/WebAssembly/binaryen/releases/download/version_50/binaryen-version_50-x86_64-linux.tar.gz | tar xvz
ADD Cargo.toml /app/
RUN mkdir -p src && touch src/lib.rs \
    && cargo build --release
ADD src /app/src
RUN cargo test --release \
    && wasm-pack build \
    && mkdir -p wasm && cp -r pkg/crt_3d_sim* wasm/ \
    && cd binaryen-version_50 && ./wasm-opt --debug -O3 -o ../wasm/crt_3d_sim_bg.wasm ../wasm/crt_3d_sim_bg.wasm >/dev/null 2>&1

FROM node:8.12.0-alpine as webpack
WORKDIR /www
ADD www/package.json .
RUN npm install --dev
ADD www .
COPY --from=rust-wasm /app/wasm/* ./
RUN npm run build \
    && cp *.css dist/ \
    && cp favicon.ico dist/ \
    && mkdir -p dist/assets/ && cp -r assets/ dist/assets/

FROM nginx:1.14.0-alpine
COPY --from=webpack /www/dist/* /var/www/html/
ADD nginx/* /etc/nginx/
CMD ["nginx","-g", "daemon off;"]