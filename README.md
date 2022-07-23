# Display Sim [![Build Status](https://travis-ci.com/theypsilon/display-sim.svg?branch=master)](https://travis-ci.com/theypsilon/display-sim)

This is a tool that helps you to recreate the visual feeling of old displays.

Demo at [theypsilon.github.io/display-sim](https://theypsilon.github.io/display-sim).

Display Sim runs both in web and native targets.

-------
## Web Target
This means Display Sim will be converted into WebAssembly, and will launch a local server that you can access with your browser.
#### Prerequisites:
- Graphic card drivers
- Browser with wasm/webgl support (common browsers comply with this)
- Docker: https://docs.docker.com/get-docker/

#### How to run:

1. On your terminal, launch the following command: `./run_web.sh`
2. After building, you'll see a message saying "Server running on port 80...". At that time just open your browser and enter the following url: http://localhost:80

---------
## Native Target
This means Display Sim will run natively on your system, using OpenGL for rendering the graphics.
#### Prerequisites:

- Graphic card drivers
- Rust: https://www.rust-lang.org/tools/install

#### How to run:

1. On your terminal, launch the following command: `./run_native.sh`
2. After compiling, a window will automatically open with the application running in it.
