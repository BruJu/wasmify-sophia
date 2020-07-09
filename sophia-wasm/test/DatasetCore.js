/* global describe, it */

const assert = require('assert')
const namespace = require('@rdfjs/namespace')

function runTests (name, rdf, instancier, iterable) {
  const ex = namespace('http://example.org/', rdf)

  describe(name + '::DatasetCore', () => {
    if (instancier === undefined) {
      describe('factory', () => {
        it('should be a function', () => {
          assert.strictEqual(typeof rdf.dataset, 'function')
        })

        it('should add the given Quads', () => {
          const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
          const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)

          const dataset = rdf.dataset([quad1, quad2])

          assert(dataset.has(quad1))
          assert(dataset.has(quad2))
        })
      })

      instancier = rdf.dataset;
    }

    describe('size', () => {
      it('should be a number property', () => {
        const dataset = instancier()

        assert.strictEqual(typeof dataset.size, 'number')
      })

      it('should be 0 if there are no Quads in the Dataset', () => {
        const dataset = instancier()

        assert.strictEqual(dataset.size, 0)
      })

      it('should be equal to the number of Quads in the Dataset', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1, quad2])

        assert.strictEqual(dataset.size, 2)
      })
    })

    describe('add', () => {
      it('should be a function', () => {
        const dataset = instancier()

        assert.strictEqual(typeof dataset.add, 'function')
      })

      it('should add the given Quad', () => {
        const quad = rdf.quad(ex.subject, ex.predicate, ex.object)
        const dataset = instancier()

        dataset.add(quad)

        assert(dataset.has(quad))
      })

      it('should not add duplicate Quads', () => {
        const quadA = rdf.quad(ex.subject, ex.predicate, ex.object)
        const quadB = rdf.quad(ex.subject, ex.predicate, ex.object)
        const dataset = instancier()

        dataset.add(quadA)
        dataset.add(quadB)

        assert.strictEqual(dataset.size, 1)
      })
    })

    describe('delete', () => {
      it('should be a function', () => {
        const dataset = instancier()

        assert.strictEqual(typeof dataset.delete, 'function')
      })

      it('should remove the given Quad', () => {
        const quad = rdf.quad(ex.subject, ex.predicate, ex.object)
        const dataset = instancier([quad])

        dataset.delete(quad)

        assert(!dataset.has(quad))
      })

      it('should remove only the given Quad', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1, quad2])

        dataset.delete(quad1)

        assert(!dataset.has(quad1))
        assert(dataset.has(quad2))
      })

/*
      it('should be chainable', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1, quad2])

        dataset.delete(quad1).delete(quad2)

        assert(!dataset.has(quad1))
        assert(!dataset.has(quad2))

        assert.strictEqual(dataset.size, 0)
      })
*/

      it('should remove the Quad with the same SPOG as the given Quad', () => {
        const quad = rdf.quad(ex.subject, ex.predicate, ex.object)
        const quadCloned = rdf.quad(quad.subject, quad.predicate, quad.object, quad.graph)
        const dataset = instancier([quad])

        dataset.delete(quadCloned)

        assert(!dataset.has(quad))
      })
    })

    describe('has', () => {
      it('should be a function', () => {
        const dataset = instancier()

        assert.strictEqual(typeof dataset.has, 'function')
      })

      it('should return false if the given Quad is not in the Dataset', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1])

        assert(!dataset.has(quad2))
      })

      it('should return true if the given Quad is in the Dataset', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1, quad2])

        assert(dataset.has(quad2))
      })
    })

    describe('match', () => {
      it('should be a function', () => {
        const dataset = instancier()

        assert.strictEqual(typeof dataset.match, 'function')
      })

      it('should use the given subject to select Quads', () => {
        const quad1 = rdf.quad(ex.subject1, ex.predicate, ex.object)
        const quad2 = rdf.quad(ex.subject2, ex.predicate, ex.object)
        const dataset = instancier([quad1, quad2])

        const matches = dataset.match(ex.subject2)

        assert.strictEqual(matches.size, 1)
        assert(matches.has(quad2))
      })

      it('should use the given predicate to select Quads', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate1, ex.object)
        const quad2 = rdf.quad(ex.subject, ex.predicate2, ex.object)
        const dataset = instancier([quad1, quad2])

        const matches = dataset.match(null, ex.predicate2)

        assert.strictEqual(matches.size, 1)
        assert(matches.has(quad2))
      })

      it('should use the given object to select Quads', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const dataset = instancier([quad1, quad2])

        const matches = dataset.match(null, null, ex.object2)

        assert.strictEqual(matches.size, 1)
        assert(matches.has(quad2))
      })

      it('should use the given graph to select Quads', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object, ex.graph1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object, ex.graph2)
        const dataset = instancier([quad1, quad2])

        const matches = dataset.match(null, null, null, ex.graph2)

        assert.strictEqual(matches.size, 1)
        assert(matches.has(quad2))
      })

      it('should return an empty Dataset if there are no matches', () => {
        const quad1 = rdf.quad(ex.subject1, ex.predicate, ex.object)
        const quad2 = rdf.quad(ex.subject2, ex.predicate, ex.object)
        const dataset = instancier([quad1, quad2])

        const matches = dataset.match(null, null, ex.object3)

        assert.strictEqual(matches.size, 0)
      })
    })

    let obtainIterator = dataset => {
      if (iterable) {
        return dataset[Symbol.iterator]();
      } else {
        return dataset.getIterator();
      }
    };

    describe('Symbol.iterator', () => {
      it('should be a function', () => {
        const dataset = instancier()

        let iterator;
        if (iterable) {
          iterator = dataset[Symbol.iterator];
        } else {
          iterator = dataset.getIterator;
        }

        assert.strictEqual(typeof iterator, 'function')
      })

      it('should return an iterator', () => {
        const quad1 = rdf.quad(ex.subject1, ex.predicate, ex.object)
        const quad2 = rdf.quad(ex.subject2, ex.predicate, ex.object)
        const dataset = instancier([quad1, quad2])
        
        //const iterator = obtainIterator(dataset);

        let iterator;
        if (iterable) {
          iterator = dataset[Symbol.iterator]();
        } else {
          iterator = dataset.getIterator();
        }

        assert.strictEqual(typeof iterator.next, 'function')
        assert.strictEqual(typeof iterator.next().value, 'object')
      })

      it('should iterate over all Quads', () => {
        const quad1 = rdf.quad(ex.subject1, ex.predicate, ex.object)
        const quad2 = rdf.quad(ex.subject2, ex.predicate, ex.object)
        const dataset = instancier([quad1, quad2])

        const output = instancier()

        if (iterable) {
          for (let item of dataset) {
            output.add(item);
          }
        } else {
          const iterator = dataset.getIterator();
          for (let item = iterator.next(); item.value; item = iterator.next()) {
            output.add(item.value)
          }
        }



        assert.strictEqual(output.size, 2)
        assert(output.has(quad1))
        assert(output.has(quad2))
      })
    })
  })
}

module.exports = runTests
