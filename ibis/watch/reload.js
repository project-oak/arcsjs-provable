let last = "";

init('../pkg/ibis_bg.wasm');

async function loadDot() {
    const dot = await fetch(`last.dot?d=${Date.now()}`);
    const dotContent = await dot.text();
    const dotSvg = await fetch(`last.svg?d=${Date.now()}`);
    const dotSvgContent = await dotSvg.text();
    const dotImage = document.getElementById('dotImage');
    if (last != dotContent) {
        last = dotContent;
        // update it
        const demo = await fetch(`../demo.json?d=${Date.now()}`);
        console.log(best_solutions_to_json(await demo.text()));
        document.getElementById('dotContent').innerText = last;
        dotImage.innerHTML = dotSvgContent;
    }
}

console.log('starting');
//refresh info every 1 second//
loadDot();
set_panic_hook();
setInterval('loadDot()', 1000);
