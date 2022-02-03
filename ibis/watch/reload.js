async function loadDot() {
    const dot = await fetch('last.dot');
    const dotContent = await dot.text();
    document.getElementById('dotContent').innerText = dotContent;
    document.getElementById('dotImage').data = `last.svg?${new Date().getTime()}`;
}

//refresh info every 5 seconds//
loadDot();
setInterval('loadDot()', 5000);
