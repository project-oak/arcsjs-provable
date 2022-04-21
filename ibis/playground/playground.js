// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
import './third_party/viz.js';
import './third_party/full.render.js';
import {loadIbis, best_solutions_to_json, best_solutions_to_dot} from '../ibis.js';
import {FilePane} from './file-pane.js';

window.customElements.define('file-pane', FilePane);

const graphviz_options = {}; // Can include engine: dot|fdp|circo|osage...

function render(dot, options) {
  var viz = new Viz();

  viz.renderSVGElement(dot, options)
  .then(function(element) {
    const graph = document.getElementById('graph');
    graph.replaceChildren(element);
  })
  .catch(error => {
    // Create a new Viz instance (@see Caveats page for more info)
    viz = new Viz();

    // Possibly display the error
    console.error(error);
  });
}

async function getInputsFromURI() {
    const filePane = document.getElementById('filePane');
    filePane.dropAllFiles(); // Empty things out.

    const uri = new URL(window.location);
    // Read the inputs from the URI.
    const contentsParams = uri.searchParams && uri.searchParams.getAll('i');
    if (contentsParams && contentsParams.length > 0) {
        for (let content of contentsParams) {
            filePane.addFile(undefined, content);
        }
    } else {
        const demo = await fetch('../chromium.json');
        const demoText = await demo.text();
        filePane.addFile(undefined, demoText);
    }
}

window.onpopstate = function(event) {
  getInputsFromURI();
}

async function startup() {
    const filePane = document.getElementById('filePane');
    const outputPane = document.getElementById('outputPane');
    const to_json = document.getElementById('to_json');
    const share = document.getElementById('share');
    to_json.addEventListener("click", () => run(best_solutions_to_json,
        input => JSON.stringify(JSON.parse(input), undefined, 2)
    ));

    const to_dot_callback = () => run(best_solutions_to_dot, dot => {
        render(dot, graphviz_options);
        return dot;
    });
    const to_dot = document.getElementById('to_dot');
    to_dot.addEventListener("click", to_dot_callback);
    filePane.addExecuteCallback(to_dot_callback);

    outputPane.addTabSwitchCallback(() => {
        const contents = outputPane.active.value;
        if (contents.startsWith('digraph')) {
            render(contents, graphviz_options);
        } else {
            console.log(JSON.parse(contents));
        }
    });

    const feedback = document.getElementById('feedback');
    share.addEventListener("click", () => {
        setURIFromInputs();
        navigator.clipboard.writeText(window.location).then(function() {
          alert('Link copied to clipboard!');
        }, function(err) {
          alert('Could not copy link (please copy the URL manually): ', err);
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
    for (let content of contents) {
        uri.searchParams.append('i', content);
    }
    window.history.pushState({},"", uri);
}


async function run(ibis_function, formatter) {
    const filePane = document.getElementById('filePane');
    const outputPane = document.getElementById('outputPane');
    const result = ibis_function(filePane.getFileContents());
    const outputFile = outputPane.addFile(undefined, formatter(result));
    outputFile.disabled = true;
}

window.onload = function() {
    startup();
};
