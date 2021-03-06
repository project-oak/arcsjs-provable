// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
import {loadIbis, run_ibis} from '../ibis.js';
import {FilePane} from './file-pane.js';
import {recipe_to_ir} from './converter.js';

window.customElements.define('file-pane', FilePane);

const known_files = {
    chromium: '../examples/chromium.json',
    demo: '../examples/demo.json',
    "ArcsJs stdlib": '../libs/arcsjs.json',
    "TS stdlib": '../libs/typescript.json',
    "Rust stdlib": '../libs/rust.json',
};

var graphviz = d3.select('#graph').graphviz().transition(function () {
    return d3.transition('main')
    .ease(d3.easeLinear)
    .delay(0)
    .duration(1500);
});

function noop(arg) { // also known as `id`
    return arg;
}

function render(dot) {
    try {
        graphviz
            .renderDot(dot)
    } catch(error) {
        // Possibly display the error
        console.error(error);
    };
}

async function addFileFromPath(pane, file) {
    const content = await fetch(file);
    const contentText = await content.text();
    pane.addFile(undefined, contentText, file);
}

async function getInputsFromURI() {
    const filePane = document.getElementById('filePane');
    filePane.dropAllFiles(); // Empty things out.

    const uri = new URL(window.location);
    // Read the inputs from the URI.
    const inputs = uri.searchParams && uri.searchParams.getAll('i') || [];
    for (let input of inputs) {
        let name = input.split('\n', 1)[0];
        input = input.substr(name.length+1);
        filePane.addFile(undefined, input, name);
    }
    const files = uri.searchParams && uri.searchParams.getAll('p') || [];
    for (let file of files) {
        await addFileFromPath(filePane, file);
    }

    if (Object.entries(filePane.getFileContents()).length === 0) {
        await addFileFromPath(filePane, known_files.chromium);
    }
}

window.onpopstate = function(event) {
  getInputsFromURI();
}

async function startup() {
    const filePane = document.getElementById('filePane');
    const outputPaneDot = document.getElementById('outputPaneDot');
    const outputPaneJSON = document.getElementById('outputPaneJSON');

    const examples = document.getElementById('examples');
    for (const [name, path] of Object.entries(known_files)) {
        const example = document.createElement('input');
        example.type="button";
        example.value = name;
        example.addEventListener('click', async () => {
            await addFileFromPath(filePane, path);
        });
        example.classList.add('button');
        examples.appendChild(example);
    }

    const run_button = document.getElementById('run');
    run_button.addEventListener("click", run_playground);

    const addFile = document.getElementById('add_file');
    addFile.addEventListener('change', async () => {
        for (const file of addFile.files) {
            filePane.addFile(undefined, await file.text(), file.name);
        }
    });

    filePane.addExecuteCallback(run_playground);

    outputPaneDot.addTabSwitchCallback(() => {
        const contents = outputPaneDot.active.value;
        render(contents);
    });

    outputPaneJSON.addTabSwitchCallback(() => {
        const contents = outputPaneJSON.active.value;
        console.info('output pane json:', JSON.parse(contents));
    });

    const feedback = document.getElementById('feedback');
    const share = document.getElementById('share');
    share.addEventListener("click", () => {
        setURIFromInputs();
        navigator.clipboard.writeText(window.location).then(function() {
          feedback.innerText = 'Link copied to clipboard!';
        }, function(err) {
          feedback.innerText = `Could not copy link (please copy the URL manually): ${err}`;
        });
    });

    await Promise.all([
        loadIbis(
            '../pkg/ibis_bg.wasm',
            (status_text, style) => {
            feedback.innerText = status_text;
            if (style === "error") {
                feedback.classList.add('error');
            } else {
                feedback.classList.remove('error');
            }
        },
            (version_info) => {
                const version_info_display = document.getElementById('version_info');
                version_info_display.innerText = version_info;
            }
        ),
        getInputsFromURI()
    ]);
    await run_playground();
}

async function setURIFromInputs() {
    const contents = filePane.getFileContents();
    // Store the inputs in the URI.
    const uri = new URL(window.location);
    if (uri.searchParams && contents === uri.searchParams.getAll('i')) {
        return;
    }
    uri.searchParams.delete('i');
    uri.searchParams.delete('p'); // Avoid sharing local file paths.
    for (let [name, content] of Object.entries(contents)) {
        uri.searchParams.append('i', `${name}\n${content}`);
    }
    window.history.pushState({},"", uri);
}


async function run_playground() {
    const outputPaneDot = document.getElementById('outputPaneDot');
    const outputPaneD3 = document.getElementById('outputPaneD3');
    const outputPaneJSON = document.getElementById('outputPaneJSON');
    const filePane = document.getElementById('filePane');
    const settings = {
        flags: {
            planning: false,
            dot: true,
            d3: true,
        },
    };

    const preparer = async (data) => {
        console.log('input data:', data);
        const files = [JSON.stringify(settings)];
        const recipes = {};
        for (const [key, value] of Object.entries(data)) {
            if (key.endsWith('.json')) { // Assume it is ibis IR.
                files.push(value);
            } else {
                recipes[key] = value; // Keep it for conversion.
            }
        }
        if (recipes !== {}) {
            const output = await recipe_to_ir(recipes);
            outputPaneJSON.addFile(undefined, output);
            files.push(output);
        }
        return files;
    };

    const prepared = await preparer(filePane.getFileContents());
    const result = run_ibis(prepared);
    const outputFileJSON = outputPaneJSON.addFile(undefined, result);
    outputFileJSON.disabled = true;
    const dot_output = JSON.parse(result).dot_output;
    const outputFileDot = outputPaneDot.addFile(undefined, dot_output);
    outputFileDot.disabled = true;
    const d3_output = JSON.parse(result).d3_output;
    const outputFileD3 = outputPaneD3.addFile(undefined, JSON.stringify(d3_output, undefined, 2));
    outputFileD3.disabled = true;
}

window.onload = function() {
    startup();
};
