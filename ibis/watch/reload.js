let last = "";

async function loadDot() {
    init('../pkg/ibis_bg.wasm');

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
setInterval('loadDot()', 1000);
