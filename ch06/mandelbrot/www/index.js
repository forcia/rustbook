'use strict'

const logic = {
    isDiverged: function (x0, y0, max_iter) {
        let xn = 0.0;
        let yn = 0.0;
        for (let i = 1; i < max_iter; i++) {
            let x_next = xn * xn - yn * yn + x0;
            let y_next = 2.0 * xn * yn + y0;
            xn = x_next;
            yn = y_next;
            if (yn * yn + xn * xn > 4.0) {
                return i;
            }
        }
        return max_iter;
    },
    generateMandelbrotSet: function (
        canvasWidth,
        canvasHeight,
        x_min,
        x_max,
        y_min,
        y_max,
        max_iter,
    ) {
        let data = [];
        for (let i = 0; i < canvasHeight; i++) {
            let y = y_min + (y_max - y_min) * i / canvasHeight;
            for (let j = 0; j < canvasWidth; j++) {
                let x = x_min + (x_max - x_min) * j / canvasWidth;
                let iter_index = this.isDiverged(x, y, max_iter);
                let v = iter_index % 8 * 32
                data.push(v); // R
                data.push(v); // G
                data.push(v); // B
                data.push(255); // A
            }
        }
        return new Uint8ClampedArray(data);
    }

}


function draw(ctx, canvasWidth, canvasHeight, data) {
    let img = new ImageData(new Uint8ClampedArray(data), canvasWidth, canvasHeight);
    ctx.putImageData(img, 0, 0);
}

const X_MIN = -1.5;
const X_MAX = 0.5;
const Y_MIN = -1.0;
const Y_MAX = 1.0;
const MAX_ITER = 1 << 7 - 1;

console.log("start loading wasm");
const mandelbrot = import('../pkg')
    .catch(console.error);
Promise.all([mandelbrot]).then(async function ([{ generate_mandelbrot_set, draw_mandelbrot_set }]) {
    console.log("finished loading wasm");

    const renderBtn = document.getElementById('render');
    renderBtn.addEventListener('click', () => {
        let jsResult = null;
        let wasmResult = null;
        {
            console.log("wasm only");
            draw_mandelbrot_set();
        }
        {
            console.log("wasm+js");
            const CANVAS_ID = "canvas_hybrid";
            let canvas = document.getElementById(CANVAS_ID);
            let context = canvas.getContext("2d");
            const canvasWidth = canvas.width;
            const canvasHeight = canvas.height;

            const generateStartTime = Date.now();
            wasmResult = generate_mandelbrot_set(canvasWidth, canvasHeight, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
            const generateEndTime = Date.now();
            const drawStartTime = Date.now();
            draw(context, canvasWidth, canvasHeight, wasmResult);
            const drawEndTime = Date.now();
            console.log(`\tgenerate:wasm\tgenerate_elapsed:${generateEndTime - generateStartTime}[ms]`);
            console.log(`\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime} [ms]`);
        }
        {
            console.log("js only");
            const CANVAS_ID = "canvas_js";
            let canvas = document.getElementById(CANVAS_ID);
            let context = canvas.getContext("2d");
            const canvasWidth = canvas.width;
            const canvasHeight = canvas.height;

            const generateStartTime = Date.now();
            jsResult = logic.generateMandelbrotSet(canvasWidth, canvasHeight, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
            const generateEndTime = Date.now();
            const drawStartTime = Date.now();
            draw(context, canvasWidth, canvasHeight, wasmResult);
            const drawEndTime = Date.now();
            console.log(`\tgenerate:wasm\tgenerate_elapsed:${generateEndTime - generateStartTime}[ms]`);
            console.log(`\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime} [ms]`);
        }

        {
            let isSame = true;
            for (let i = 0; i < wasmResult.length; i++) {
                if (wasmResult[i] !== jsResult[i]) {
                    console.log(i, wasmResult[i], jsResult[i]);
                    isSame = false;
                    break;
                }
            }
            console.log(`(wasmResult === jsResult):${isSame}`);
        }
    });


    const N = 1000;
    let jsResult = null;
    let wasmResult = null;
    {
        const CANVAS_ID = "canvas_hybrid";
        let result = [];
        let canvas = document.getElementById(CANVAS_ID);
        let context = canvas.getContext("2d");
        const canvasWidth = canvas.width;
        const canvasHeight = canvas.height;

        const generateStartTime = Date.now();
        for (let i = 0; i < N; i++) {
            const iterStartTime = Date.now();
            wasmResult = generate_mandelbrot_set(canvasWidth, canvasHeight, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
            result.push(Date.now() - iterStartTime);
        }
        const generateEndTime = Date.now();
        console.log(`generate:wasm\tgenerate_elapsed:${generateEndTime - generateStartTime}[ms]/${N}iter`);
        console.log(JSON.stringify(result));
    }
    {
        const CANVAS_ID = "canvas_js";
        let result = [];
        let canvas = document.getElementById(CANVAS_ID);
        let context = canvas.getContext("2d");
        const canvasWidth = canvas.width;
        const canvasHeight = canvas.height;

        const generateStartTime = Date.now();
        for (let i = 0; i < N; i++) {
            const iterStartTime = Date.now();
            jsResult = logic.generateMandelbrotSet(canvasWidth, canvasHeight, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
            result.push(Date.now() - iterStartTime);
        }
        const generateEndTime = Date.now();
        console.log(`generate: js\tgenerate_elapsed: ${generateEndTime - generateStartTime}[ms]/${N}iter`);
        console.log(JSON.stringify(result));
    }


});
