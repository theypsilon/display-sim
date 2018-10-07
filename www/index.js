let event_loop;
function run(module) {
    const dpi = window.devicePixelRatio;
    const width = window.screen.width;
    const height = window.screen.height;
    
    const canvas = document.getElementById("1-canvas");
    
    canvas.width = width * dpi;
    canvas.height = height * dpi;
    
    canvas.style.width = width;
    canvas.style.height = height;
    
    canvas.addEventListener('webglcontextlost', function() {});
    canvas.addEventListener('webglcontextrestored', function() {});

    const gl = canvas.getContext('webgl2', { 
        alpha: true, 
        antialias: true, 
        depth: true, 
        failIfMajorPerformanceCaveat: false, 
        powerPreference: "high-performance",
        premultipliedAlpha: true, 
        preserveDrawingBuffer: false, 
        stencil: false 
    });

    window.addEventListener('request_pointer_lock', function() {
        canvas.requestPointerLock();
    }, false);

    window.addEventListener('exit_pointer_lock', function() {
        document.exitPointerLock();
    }, false);

    canvas.requestPointerLock = canvas.requestPointerLock || canvas.mozRequestPointerLock;
    document.exitPointerLock = document.exitPointerLock || document.mozExitPointerLock;
    
    if (!gl) throw new Error("Could not get webgl context.");

    event_loop = module.main(gl);
}

import('wasm-game-of-life').then(run)