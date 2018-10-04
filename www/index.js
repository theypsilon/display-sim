// For more comments about what's going on here, check out the `hello_world`
// example.
import('wasm-game-of-life')
  .then(app => app.draw())
  .catch(console.error);
