import {all} from './quill_tests/all.mjs';

const {entries, fromEntries} = Object;
const META = '$';

function add_store(ir, store_types, store_name, meta) {
    let structure = meta.$type || '*';
    if (structure.startsWith('[') && structure.endsWith(']')) {
        structure = `List(${structure.substr(1, structure.length - 2)})`;
    }
    const tags = meta.$tags || [];//.map(tag => ` +${tag}`).join('');
    const ty = structure;// + tags;
    const store_id = `store_${store_name}`;
    store_types[store_name] = ty;
    ir.nodes.push([store_id, `${store_id}_in`, `read ${ty}`]);
    ir.nodes.push([store_id, `${store_id}_out`, `write ${ty}`]);
    for (const tag of tags) {
        ir.claims.push([`${store_id}_out`, tag]);
    }
}

function add_slot(ir, recipe_name, particle_id, slot_name, meta) {
    for (const [particle, particle_meta] of entries(meta)) {
        add_particle(ir, recipe_name, particle, particle_meta) // TODO: Namespacing?
    }
    // TODO: add slot itself
}

function add_particle(ir, recipe_name, particle_name, meta) {
    const particle_id = `particle_${recipe_name}_${particle_name}`;
    const kind = meta.$kind; // TODO: unknown usage
    const staticInputs = meta.$staticInputs; // TODO: unknown usage
    if (meta.$bindings) {
        throw new Error('Should be using $inputs and $outputs instead of $bindings');
    }
    const handle_id = (handle_name) => `${particle_id}_${handle_name}`;
    const store_id = (store_name) => `store_${store_name}`;
    const inputs = meta.$inputs || {};
    for (let [handle_name, store_name] of entries(inputs)) {
        if (store_name === '') {
            store_name = handle_name;
        }
        const store_type = store_types[store_name];
        ir.nodes.push([particle_id, handle_id(handle_name), `read ${store_type}`]);
        ir.edges.push([`${store_id(store_name)}_out`, handle_id(handle_name)]);
    }
    const outputs = meta.$outputs || {};
    for (let [handle_name, store_name] of entries(outputs)) {
        if (store_name === '') {
            store_name = handle_name;
        }
        const store_type = store_types[store_name];
        ir.nodes.push([particle_id, handle_id(handle_name), `write ${store_type}`]);
        ir.edges.push([handle_id(handle_name), `${store_id(store_name)}_in`]);
    }

    const slots = meta.$slots;
    for (const [slot, meta] of entries(slots || {})) {
        add_slot(ir, recipe_name, particle_id, slot, meta);
    }
}

function convert_to_ibis(ir, store_types, recipe_name, recipe) {
    // console.dir(recipe, {depth: null});
    const particles = fromEntries(entries(recipe).filter(([key, value]) => !key.startsWith(META)));
    const stores = recipe.$stores;
    for (const [store_name, meta] of entries(stores)) {
        add_store(ir, store_types, store_name, meta);
    }

    for (const [particle_name, meta] of entries(particles)) {
        add_particle(ir, particle_name, recipe_name, meta);
    }
    // console.log(ir);
    return ir;
}

const all_ir = {
    flags: { planning: false },
    subtypes: [
        ["any", "read"],
        ["any", "write"],
    ],
    capabilities: [
        ["write", "read"],
    ],
    less_private_than: [
        ["public", "private"]
    ],
    recipes: [],
    nodes: [],
    edges: [],
    checks: [],
    claims: [],
    trusted_to_remove_tag: [],
};

const store_types = {};
for(let [key, value] of entries(all)) {
    convert_to_ibis(all_ir, store_types, key, value);
}
// console.dir(all_ir, {depth: null});
// console.log(JSON.stringify(all_ir, undefined, 2));
console.log(JSON.stringify(all_ir));
