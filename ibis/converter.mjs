import {all} from './quill_tests/all.mjs';

const {entries, fromEntries} = Object;
const META = '$';

function add_store(ir, store_types, name, meta) {
    const structure = meta.$type || '*';
    const capabilities = (meta.$tags || []).map(tag => ` +${tag}`).join('');
    const ty = `read write ` + structure + capabilities;
    store_types[name] = ty;
    ir.nodes.push([`store_${name}`, `store_${name}`, ty]);
}

function add_slot(ir, particle_id, name, meta) {
    console.log('add_slot', particle_id, name, meta);
    throw new Error("Don't know what to do with slots");
}

function add_particle(ir, name, meta) {
    const particle_id = `particle_${name}`;
    const kind = meta.$kind; // TODO: unknown usage
    const bindings = meta.$bindings;
    for (const [handle, store] of entries(bindings)) {
        const handle_id = `${particle_id}_${handle}`;
        const store_id = `store_${store}`;
        ir.nodes.push([particle_id, handle_id, `*`]);
        ir.edges.push([handle_id, store_id]); // IFF is output (we don't know)
        ir.edges.push([store_id, handle_id]); // IFF is input (we don't know)
    }

    const slots = meta.$slots; // TODO: unknown usage
    for (const [slot, meta] of entries(slots || {})) {
        add_slot(ir, particle_id, slot, meta);
    }
}

function convert_to_ibis(name, recipe) {
    console.dir(recipe, {depth: null});
    const particles = fromEntries(entries(recipe).filter(([key, value]) => !key.startsWith(META)));
    const stores = recipe.$stores;

    // ibis ir
    const ir = {
        nodes: [],
        edges: [],
        checks: [],
        claims: [],
        trusted_to_remove_tag: [],
        subtypes: [
            ["any", "read"],
            ["any", "write"],
        ],
        capabilities: [
            ["write", "read"],
        ],
        flags: { planning: false },
        less_private_than: [
            ["public", "private"]
        ]
    };

    const store_types = {};
    for (const [name, meta] of entries(stores)) {
        add_store(ir, store_types, name, meta);
    }

    for (const [name, meta] of entries(particles)) {
        add_particle(ir, name, meta);
    }
    // console.log(ir);
}

for(let [key, value] of entries(all)) {
    const out = convert_to_ibis(key, value);
}
