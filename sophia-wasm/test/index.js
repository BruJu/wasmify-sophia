'use strict'

function testDataset(name, rdf, datasetInstancier, iterable) {
  require('./DatasetCore')(name, rdf, datasetInstancier, iterable)     // Dataset
  require('./Dataset')(name, rdf, datasetInstancier)         // Dataset
}

function runTests (rdf, datasets) {
  //require('./named-node')(rdf)      // Data Model
  //require('./blank-node')(rdf)      // Data Model
  //require('./literal')(rdf)         // Data Model
  //require('./default-graph')(rdf)   // Data Model
  //require('./variable')(rdf)        // Data Model
  //require('./triple')(rdf)          // Data Model
  //require('./quad')(rdf)            // Data Model
  //testDataset("FromFactory", rdf, undefined);
  
  //testDataset('FastDataset', rdf, datasets['FastDataset'].init);
  //testDataset('WrappedFastDataset', rdf, datasets['WrappedFastDataset'].init, true);

  for (let dataset in datasets) {
    testDataset(dataset, rdf, datasets[dataset].init, datasets[dataset].isIterable);
  }
}

if (global.rdf) {
  runTests(global.rdf, global.datasets)
}

module.exports = runTests
