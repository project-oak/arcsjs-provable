let last = "";

async function loadDot() {
    const dot = await fetch('last.dot');
    const dotContent = await dot.text();
    const dotSvg = await fetch(`last.svg?d=${Date.now()}`);
    const dotSvgContent = await dotSvg.text();
    const dotImage = document.getElementById('dotImage');
    if (last != dotSvgContent) {
        last = dotSvgContent;
        // update it
        document.getElementById('dotContent').innerText = dotContent;
        dotImage.innerHTML = last;
    }
}

//refresh info every 1 second//
loadDot();
setInterval('loadDot()', 1000);
