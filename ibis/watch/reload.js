let last = "";

async function loadDot() {
    const dot = await fetch(`last.dot?d=${Date.now()}`);
    const dotContent = await dot.text();
    const dotSvg = await fetch(`last.svg?d=${Date.now()}`);
    const dotSvgContent = await dotSvg.text();
    const dotImage = document.getElementById('dotImage');
    if (last != dotContent) {
        last = dotContent;
        // update it
        document.getElementById('dotContent').innerText = last;
        dotImage.innerHTML = dotSvgContent;
    }
}

//refresh info every 1 second//
loadDot();
setInterval('loadDot()', 1000);
