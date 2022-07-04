const canvas = document.querySelector("canvas");
// We assume width and height are the same - CSS code is responsible for that
export const board_pixel_height = canvas.offsetHeight;
export const board_cell_height = 50;
export const cell_pixel_height = board_pixel_height / board_cell_height;

canvas.height = board_pixel_height;
canvas.width = board_pixel_height;

// Import wasm-bindgen glue code
import init from "./pkg/ai_snake_wasm.js"

async function main() {
    // Run wasm code
    await init();
}
main();
