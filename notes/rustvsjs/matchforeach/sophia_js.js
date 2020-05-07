#!/usr/bin/env node

const n3 = require("n3");
const sophia_js = require('../../Portable-Reasoning-in-Web-Assembly/wasm_example/wasm_example.js')
const { task, filename, stream, get_vmsize, performance } = require("./common.js")

const do_task = {
    'query': () => query_nt(1),
    'query2': () => query_nt(2),
    'query3': () => query_nt(3),
    'query4': () => query_nt(4)
}[task];

do_task();

function query_nt(query_num) {
    let t_load, m_graph, t_first, t_rest;
    let start, duration;
    const format = "N-Triples";    
    let parser = new n3.Parser({ format: format });
    const mem0 = get_vmsize();
    start = performance.now();

    let store;
    switch (process.argv[4]) {
        case "Full":
            store = new sophia_js.FullDataset();
            break;
        case "Tree":
            store = new sophia_js.TreeDataset();
            break;
        case "Fast":
            store = new sophia_js.FastDataset();
            break;
        case "Light":
            store = new sophia_js.LightDataset();
            break;
        default:
            console.error("Unknown dataset " + dataset);
            process.exit(7);        
    }

    parser.parse(stream, (error, quad, prefixes) => {
        if (error) {
            console.error(error);
            process.exit(2);
        }
        if (quad) {
            store.add(quad);
        } else {
            duration = performance.now() - start;
            const mem1 = get_vmsize();
            t_load = duration/1000;
            m_graph = mem1-mem0

            let subject, predicate, object, graph;
            if (query_num == 1 || query_num == 3) {
                subject = undefined;
                predicate = n3.DataFactory.namedNode('http://www.w3.org/1999/02/22-rdf-syntax-ns#type');
                object = n3.DataFactory.namedNode('http://dbpedia.org/ontology/Person');
            } else if (query_num == 2 || query_num == 4) {
                subject = n3.DataFactory.namedNode('http://dbpedia.org/resource/Vincent_Descombes_Sevoie');;
                predicate = undefined;
                object = undefined;
            }

            if (query_num <= 2) {
                graph = n3.DataFactory.defaultGraph();
            } else {
                graph = undefined;
            }

            let bench = function() {
                let start = performance.now();
                let filtered_dataset = store.match(subject, predicate, object, graph);

                let counter = 0;
                let t_first, t_last;
                if (true) {
                    filtered_dataset.forEach(quad => {
                        if (counter == 0) {
                            duration = performance.now() - start;
                            t_first = duration/1000;
                            start = performance.now();
                        }
                        counter += 1;
                    });

                    duration = performance.now() - start;
                    t_last = duration/1000;
                } else {
                    counter = filtered_dataset.size;
                    
                    let duration = performance.now() - start;
                    t_first = duration/1000;
                    t_last = 0;
                }

                return [t_first, t_last, counter];
            }

            let firstData = bench();
            let secondData = bench();
            const mem2 = get_vmsize() - mem0;

            console.error(`retrieved: ${firstData[2]}`);
            console.log(`${t_load},${m_graph},${mem2},${firstData[0]},${firstData[1]},${secondData[0]},${secondData[1]}`);
            process.exit(0);
        }
    });
}

