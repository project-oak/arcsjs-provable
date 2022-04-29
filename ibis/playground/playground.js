// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
import {loadIbis, best_solutions_to_json, best_solutions_to_dot} from '../ibis.js';
import {FilePane} from './file-pane.js';

window.customElements.define('file-pane', FilePane);

const known_files = {
    chromium: '/chromium.json',
    demo: '/demo.json',
    "ArcsJs stdlib": '/libs/arcsjs.json',
    "TS stdlib": '/libs/typescript.json',
    "Rust stdlib": '/libs/rust.json',
};

var graphviz = d3.select('#graph').graphviz().transition(function () {
    return d3.transition('main')
    .ease(d3.easeLinear)
    .delay(0)
    .duration(1500);
});

function render(dot) {
    try {
        console.log('RENDERING');
        graphviz
            .renderDot(dot)
        console.log('DONE RENDERING');
    } catch(error) {
        // Possibly display the error
        console.error(error);
    };
}

async function addFileFromPath(pane, file) {
    const content = await fetch(file);
    const contentText = await content.text();
    pane.addFile(undefined, contentText);
}

async function getInputsFromURI() {
    const filePane = document.getElementById('filePane');
    filePane.dropAllFiles(); // Empty things out.

    const uri = new URL(window.location);
    // Read the inputs from the URI.
    const inputs = uri.searchParams && uri.searchParams.getAll('i') || [];
    for (let input of inputs) {
        filePane.addFile(undefined, input);
    }
    const files = uri.searchParams && uri.searchParams.getAll('p') || [];
    for (let file of files) {
        await addFileFromPath(filePane, file);
    }

    if (filePane.getFileContents().length === 0) {
        await addFileFromPath(filePane, known_files.chromium);
    }
}

window.onpopstate = function(event) {
  getInputsFromURI();
}

async function startup() {
    const outputPaneDot = document.getElementById('outputPaneDot');
    const outputPaneJSON = document.getElementById('outputPaneJSON');

    const to_json_callback = () => run(best_solutions_to_json, json => {
        return JSON.stringify(JSON.parse(json), undefined, 2);
    }, outputPaneJSON);
    const to_json = document.getElementById('to_json');
    to_json.addEventListener("click", to_json_callback);

    const to_dot_callback = () => run(best_solutions_to_dot, dot => dot, outputPaneDot);
    const to_dot = document.getElementById('to_dot');
    to_dot.addEventListener("click", to_dot_callback);

    const addFile = document.getElementById('add_file');
    addFile.addEventListener('change', async () => {
        for (const file of addFile.files) {
            filePane.addFile(undefined, await file.text());
        }
    });

    const filePane = document.getElementById('filePane');
    filePane.addExecuteCallback(to_dot_callback);

    outputPaneDot.addTabSwitchCallback(() => {
        const contents = outputPaneDot.active.value;
        render(contents);
    });

    outputPaneJSON.addTabSwitchCallback(() => {
        const contents = outputPaneJSON.active.value;
        console.log(JSON.parse(contents));
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
    await to_dot_callback();
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
    for (let content of contents) {
        uri.searchParams.append('i', content);
    }
    window.history.pushState({},"", uri);
}


async function run(ibis_function, formatter, destination) {
    const filePane = document.getElementById('filePane');
    const result = ibis_function(filePane.getFileContents());
    const outputFile = destination.addFile(undefined, formatter(result));
    outputFile.disabled = true;
}

window.onload = function() {
    startup();
};
