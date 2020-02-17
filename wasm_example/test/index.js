'use strict'

function runTests (rdf) {
  require('./named-node')(rdf)      // Data Model
  require('./blank-node')(rdf)      // Data Model
  require('./literal')(rdf)         // Data Model
  require('./default-graph')(rdf)   // Data Model
  require('./variable')(rdf)        // Data Model
  require('./triple')(rdf)          // Data Model
  require('./quad')(rdf)            // Data Model
  require('./DatasetCore')(rdf)     // Dataset
}

if (global.rdf) {
  runTests(global.rdf)
}

module.exports = runTests
