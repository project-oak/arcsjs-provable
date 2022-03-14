import {default as ibis, version_info, best_solutions_to_json, best_solutions_to_dot} from '../pkg/ibis.js';

async function loadIbis() {
    const feedback = document.getElementById('feedback');
    const version_info_display = document.getElementById('version_info');
    feedback.innerText = 'Loading ibis...';
    // Set up ibis
    await ibis('../pkg/ibis_bg.wasm');
    feedback.innerText = 'Loaded';
    version_info_display.innerText = version_info();
}

async function getDemoContent() {
    const demo = await fetch('../demo.json');
    const demoText = await demo.text();
    const input = document.getElementById('input');
    input.value = demoText;
}

async function startup() {
    const input = document.getElementById('input');
    const to_json = document.getElementById('to_json');
    to_json.addEventListener("click", () => run(best_solutions_to_json,
        input => JSON.stringify(JSON.parse(input), undefined, 2)
    ));
    const to_dot = document.getElementById('to_dot');
    to_dot.addEventListener("click", () => run(best_solutions_to_dot, x => x));

    await Promise.all([loadIbis(), getDemoContent()]);
}

async function run(ibis_function, formatter) {
    const input = document.getElementById('input');
    const output = document.getElementById('output');
    const feedback = document.getElementById('feedback');
    feedback.innerText = 'Running...';
    const inputText = input.value;
    if (!inputText) {
        output.value = 'no text';
        return;
    }
    // update it
    const startTime = performance.now()
    const result = ibis_function(inputText);
    const endTime = performance.now()
    feedback.innerText = `Done in ${(endTime-startTime)/1000.0} seconds`;
    output.value = formatter(result);
}

window.onload = function() {
    startup();
};
