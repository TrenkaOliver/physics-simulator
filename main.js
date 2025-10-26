import init, {World, Square, Force} from "./pkg/physics_simulator.js"

const window_width = window.innerWidth;
const window_height = window.innerHeight;
const size = window_height / 15;
let canvas = document.querySelector("canvas");
let context = canvas.getContext("2d");
let world;

async function start() {
    await init();
    canvas.width = window_width;
    canvas.height = window_height;

    world = new World(performance.now(), canvas.width, canvas.height);
    world.add_force(new Force("vertical", 0, 0.1));
    world.add_force(new Force("horizontal", 0, 0.1));
    context.fillStyle = "#128493ff";
    requestAnimationFrame(render);
}

function render() {
    context.clearRect(0, 0, window_width, window_height);
    world.update(performance.now());

    const props = world.get_square_props();
    for (let index = 0; index < props.length; index += 3) {
        const x = props[index];
        const y = props[index + 1];
        const size = props[index + 2];

        context.fillRect(x, y, size, size);
    }

    requestAnimationFrame(render);
} 

canvas.addEventListener('click', (e) => {
    if (e.button === 0) {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left - size / 2;
        const y = e.clientY - rect.top - size  / 2;
        world.add_square("new square", false, x, y, size, 10);
    }
});

canvas.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size  / 2;
    world.add_square("new square", true, x, y, size, 10);
});

start();