import {default as ibis, best_solutions_to_json} from '../pkg/ibis.js';

async function loadIbis() {
    const feedback = document.getElementById('feedback');
    feedback.innerText = 'Loading ibis...';
    // Set up ibis
    await ibis('../pkg/ibis_bg.wasm');
    feedback.innerText = 'Loaded';
}

async function getDemoContent() {
    const demo = await fetch('../demo.json');
    const demoText = await demo.text();
    const input = document.getElementById('input');
    input.value = demoText;
}

async function startup() {
    const input = document.getElementById('input');
    input.addEventListener("change", loadDot);

    await Promise.all([loadIbis(), getDemoContent()]);
    await loadDot();
}

async function loadDot() {
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
    const result = best_solutions_to_json(inputText);
    const resultJson = JSON.parse(result);
    const endTime = performance.now()
    console.log(resultJson);
    feedback.innerText = `Done in ${(endTime-startTime)/1000.0} seconds`;
    output.value = JSON.stringify(resultJson, undefined, 4);
}

window.onload = function() {
    startup();
};
