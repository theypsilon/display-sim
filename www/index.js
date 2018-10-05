var event_loop;
function run(module) {
    event_loop = module.main();
}

import('wasm-game-of-life').then(run)