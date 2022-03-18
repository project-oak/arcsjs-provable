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

function render(dot) {
  var viz = new Viz();

  viz.renderSVGElement(dot)
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

async function getDemoContent() {
    const demo = await fetch('../demo.json');
    const demoText = await demo.text();
    const filePane = document.getElementById('filePane');
    filePane.addFile(undefined, demoText);
}

async function startup() {
    const filePane = document.getElementById('filePane');
    const to_json = document.getElementById('to_json');
    const clear_output = document.getElementById('clear_output');
    to_json.addEventListener("click", () => run(best_solutions_to_json,
        input => JSON.stringify(JSON.parse(input), undefined, 2)
    ));
    const to_dot = document.getElementById('to_dot');
    to_dot.addEventListener("click", () => run(best_solutions_to_dot, dot => {
        render(dot);
        return dot;
    }));
    clear_output.addEventListener("click", () => {
        const outputPane = document.getElementById('outputPane');
        outputPane.dropAllFiles();
    });

    await Promise.all([
        loadIbis(
            '../pkg/ibis_bg.wasm',
            (status_text, style) => {
            const feedback = document.getElementById('feedback');
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
        getDemoContent()
    ]);
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
