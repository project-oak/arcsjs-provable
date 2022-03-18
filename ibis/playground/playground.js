// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
import './third_party/viz.js';
import './third_party/full.render.js';
import {default as ibis, version_info, best_solutions_to_json, best_solutions_to_dot} from '../pkg/ibis.js';
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

async function loadIbis() {
    const feedback = document.getElementById('feedback');
    const version_info_display = document.getElementById('version_info');
    feedback.innerText = 'Loading ibis...';
    // Set up ibis
    await ibis('../pkg/ibis_bg.wasm');
    feedback.innerText = 'Loaded';
    version_info_display.innerText = version_info();
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
    to_json.addEventListener("click", () => run(best_solutions_to_json,
        input => JSON.stringify(JSON.parse(input), undefined, 2)
    ));
    const to_dot = document.getElementById('to_dot');
    to_dot.addEventListener("click", () => run(best_solutions_to_dot, dot => {
        render(dot);
        return dot;
    }));

    await Promise.all([loadIbis(), getDemoContent()]);
}

function merge_recipe(dest, new_recipe) {
    for (const prop in new_recipe) {
        if (!Object.prototype.hasOwnProperty.call(new_recipe, prop)) {
            continue;
        }
        // Add the data
        if (prop in dest) {
            if (dest[prop] instanceof Array) {
                dest[prop].push(...new_recipe[prop]);
            } else {
                merge_recipe(dest[prop], new_recipe[prop]);
            }
        } else {
            // TODO: Handle different data types differently
            dest[prop] = new_recipe[prop];
        }
    }
}

async function run(ibis_function, formatter) {
    const filePane = document.getElementById('filePane');
    const outputPane = document.getElementById('outputPane');
    const feedback = document.getElementById('feedback');
    feedback.innerText = 'Running...';
    const inputData = {};
    filePane.getFilesContents().forEach(file => {
        if (file != "") {
            const data = JSON.parse(file);
            merge_recipe(inputData, data);
        }
    });
    const inputText = JSON.stringify(inputData);
    // update it
    const startTime = performance.now()
    const result = ibis_function(inputText);
    const endTime = performance.now()
    feedback.innerText = `Done in ${(endTime-startTime)/1000.0} seconds`;
    const outputFile = outputPane.addFile(undefined, formatter(result));
    outputFile.disabled = true;
}

window.onload = function() {
    startup();
};
