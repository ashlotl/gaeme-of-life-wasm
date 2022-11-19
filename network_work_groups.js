import * as mod from "./pkg/network_work_groups.js";

const BOARD_WIDTH = 300;
const BOARD_HEIGHT = 300;

var rust_mem_canvas = document.getElementById("rust_mem_canvas");
var pause_button = document.getElementById("pause");
var pause_div = document.getElementById("pause_div");

document.oncontextmenu = null;
window.oncontextmenu = null;

rust_mem_canvas.width = BOARD_WIDTH;
rust_mem_canvas.height = BOARD_HEIGHT;

const image = new Image;
image.src = "https://upload.wikimedia.org/wikipedia/commons/1/15/EasternGraySquirrel_GAm.jpg";
image.onload = () => {
    rust_mem_canvas.getContext("2d").drawImage(image, 0, 0, 1482, 1482, 0, 0, BOARD_WIDTH, BOARD_HEIGHT);
}

var paused;
pause_button.onclick = () => {
    if (!paused) {
        pause_div.innerText = "paused...";
    } else {
        pause_div.innerText = "resuming...";
    }
    paused = !paused;
}

const sleep = ms => new Promise(r => setTimeout(r, ms));

const frame_time = 30;

const renderLoop = (timeStamp, previousTimeStamp, board, u8Array, canvasContext, aabb) => {
    let delta = timeStamp - previousTimeStamp;
    sleep(frame_time - delta).then(() => {
        if (!paused) {
            board.tick(aabb);

            pause_div.innerText = "(resuming, " + board.tick_count + ")";

            let start_ptr;
            if (board.tick_count % 2 == 0) {
                start_ptr = board.start_ptr_even();
            } else {
                start_ptr = board.start_ptr_odd();
            }
            let sliced = u8Array.slice(start_ptr, start_ptr + BOARD_WIDTH * BOARD_HEIGHT * 4);
            let imageData = new ImageData(sliced, BOARD_WIDTH, BOARD_HEIGHT);
            canvasContext.putImageData(imageData, 0, 0);
        }
        requestAnimationFrame((newTimeStamp) => { renderLoop(newTimeStamp, performance.now(), board, u8Array, canvasContext, aabb) });
    });
};

mod.default().then(() => {
    mod.init_rust_side();
    let Board = mod.Board;
    let AABB = mod.AABB;
    let board = Board.new(BOARD_WIDTH, BOARD_HEIGHT, [255, 0, 0, 255]);

    console.log(board.bit_index(101, 0));

    console.log(board.start_ptr_even(), board.start_ptr_odd());

    let memory = mod.wasm_memory();
    let u8Array = new Uint8ClampedArray(memory.buffer);

    let canvasContext = rust_mem_canvas.getContext("2d");

    rust_mem_canvas.onclick = (event) => {

        let g_x = event.clientX;
        let g_y = event.clientY;

        let rect = rust_mem_canvas.getBoundingClientRect();
        let c_x = rect.left;
        let c_y = rect.top;

        let s_x = BOARD_WIDTH / rust_mem_canvas.clientWidth;
        let s_y = BOARD_HEIGHT / rust_mem_canvas.clientHeight;

        let l_x = (g_x - c_x) * s_x;
        let l_y = (g_y - c_y) * s_y;

        if (event.altKey) {
            console.log(board.count_neighbors(l_x, l_y), board.tick_count);
        } else {

            if (board.get(l_x, l_y)) {
                board.set(l_x, l_y, 0);
            } else {
                board.set(l_x, l_y, 255);
            }

            let start_ptr;
            if (board.tick_count % 2 == 1) {
                start_ptr = board.start_ptr_odd();
            } else {
                start_ptr = board.start_ptr_even();
            }
            let sliced = u8Array.slice(start_ptr, start_ptr + BOARD_WIDTH * BOARD_HEIGHT * 4);
            let imageData = new ImageData(sliced, BOARD_WIDTH, BOARD_HEIGHT);
            canvasContext.putImageData(imageData, 0, 0);
        }
        return false;
    };

    renderLoop(1, 0, board, u8Array, canvasContext, AABB.new(0, 0, BOARD_WIDTH, BOARD_HEIGHT));


});