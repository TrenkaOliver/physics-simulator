import init, {World, Square, Force} from "./pkg/physics_simulator.js"

const color = "#43c5bcff";

let size;
let canvas = document.querySelector("canvas");
let add_force_button = document.getElementById("add-force");
let context = canvas.getContext("2d");
let world;

async function start() {
    await init();

    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width;  
    canvas.height = rect.height;

    size = canvas.height / 15;

    world = new World(performance.now(), canvas.width, canvas.height);
    world.add_force(new Force("only", 0, 0));
    display_forces();

    context.fillStyle = color;
    requestAnimationFrame(render);
}

function render() {
    context.clearRect(0, 0, canvas.width, canvas.height);
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

function display_forces() {
    const forces = world.get_global_forces();
    const container = document.getElementById("force-container");
    container.innerHTML = "";

    for (let i = 0; i < forces.length; i++) {
        const f = forces[i];
        
        //container
        const inner_container = document.createElement("div");
        inner_container.className = "force-input";

        //force name
        const nameInput = document.createElement("input");
        nameInput.type = "text";
        nameInput.className = "name-input";
        nameInput.value = f.name;
        inner_container.appendChild(nameInput);
        nameInput.addEventListener("input", (e) => {
            world.change_force_title(i, e.target.value);
        });

        //x input
        const xLabel = document.createElement("label");
        xLabel.textContent = "X: ";
        const xInput = document.createElement("input");
        xInput.type = "number";
        xInput.className = "x-input";
        xInput.value = f.x.toFixed(2);
        xInput.step = "0.01";
        xLabel.appendChild(xInput);
        xInput.addEventListener("input", (e) => {
            world.change_force_x(i, e.target.value);
        });

        //y input
        const yLabel = document.createElement("label");
        yLabel.textContent = "Y: ";
        const yInput = document.createElement("input");
        yInput.type = "number";
        yInput.className = "y-input";
        yInput.value = f.y.toFixed(2);
        yInput.step = "0.01";
        yLabel.appendChild(yInput);
        yInput.addEventListener("input", (e) => {
            world.change_force_y(i, e.target.value);
        });

        inner_container.appendChild(xLabel);
        inner_container.appendChild(yLabel);

        container.appendChild(inner_container);
    }
}

canvas.addEventListener('click', (e) => {
    if (e.button === 0) {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left - size / 2;
        const y = e.clientY - rect.top - size  / 2;
        world.add_square("new square", false, x, y, size, 10);
    }

    console.log("alma");
});

canvas.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size  / 2;
    world.add_square("new square", true, x, y, size, 10);
});

window.addEventListener('resize', () => {
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width;
    canvas.height = rect.height;
    context.fillStyle = color;
});

add_force_button.addEventListener('click', () => {
    world.add_force(new Force("untitled", 0, 0));
    display_forces();
});

start();