// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

import {
    default as ibis,
    version_info,
    best_solutions_to_json as best_solutions_to_json_impl,
    best_solutions_to_dot as best_solutions_to_dot_impl
} from './pkg/ibis.js';

let ibisStatusCallback = undefined;

function logStatus(status, style) {
    if (ibisStatusCallback) {
        ibisStatusCallback(status, style);
    } else {
        if (style === 'error') {
            console.error(status);
        } else {
            console.log(`ibis: ${status}`);
        }
    }
}

export async function loadIbis(ibis_path, status_callback, version_info_callback) {
    ibisStatusCallback = status_callback;
    try {
        logStatus('Loading ibis...');
        await ibis(ibis_path); // Set up ibis
        logStatus('Loaded');

        if (version_info_callback) {
            version_info_callback(version_info());
        }
    } catch (err) {
        logStatus(`${err}`, 'error');
        throw err;
    }
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

function run(func, input) {
    try {
        logStatus(`Merging recipes...`);
        const inputData = {};
        input.forEach(file => {
            if (file != '') {
                const data = JSON.parse(file);
                merge_recipe(inputData, data);
            }
        });
        const inputJSON = JSON.stringify(inputData);
        logStatus(`Running...`);
        const startTime = performance.now()
        const result = func(inputJSON);
        const endTime = performance.now()
        logStatus(`Done in ${(endTime-startTime)/1000.0} seconds`);
        return result;
    } catch (err) {
        logStatus(`${err}`, 'error');
        throw err;
    }
}

export function best_solutions_to_json(input) {
    return run(best_solutions_to_json_impl, input);
}

export function best_solutions_to_dot(input) {
    return run(best_solutions_to_dot_impl, input);
}
