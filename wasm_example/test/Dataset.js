/* global describe, it */

const assert = require('assert')
const namespace = require('@rdfjs/namespace')

function runTests (rdf) {
  const ex = namespace('http://example.org/', rdf)

  describe('Dataset', () => {
    describe('addAll', () => {
        it('should be a function', () => {
          const dataset = rdf.dataset()
  
          assert.strictEqual(typeof dataset.addAll, 'function')
        })
  
        it('should add the given sequence', () => {
          const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
          const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
          const quad3 = rdf.quad(ex.subject, ex.predicate, ex.object3)
          const quad4 = rdf.quad(ex.subject, ex.predicate, ex.object4)

          const dst = rdf.dataset()

          // Array
          dst.addAll([quad1, quad2])
          assert(dst.has(quad1))
          assert(dst.has(quad2))
          // Set
          dst.addAll(new Set([quad3, quad4]))
          assert(dst.has(quad1))
          assert(dst.has(quad2))
          assert(dst.has(quad3))
          assert(dst.has(quad4))
        })
  
        it('should add the given dataset', () => {
            const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
            const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
            const quad3 = rdf.quad(ex.subject, ex.predicate, ex.object3)
  
            const dst = rdf.dataset([quad1])
            const src = rdf.dataset([quad2, quad3])

            dst.addAll(src)
            assert(dst.has(quad1))
            assert(dst.has(quad2))
            assert(dst.has(quad3))
          })

        /*
        it('should not add duplicate Quads', () => {
          const quadA = rdf.quad(ex.subject, ex.predicate, ex.object)
          const quadB = rdf.quad(ex.subject, ex.predicate, ex.object)
          const dataset = rdf.dataset()
  
          dataset.add(quadA)
          dataset.add(quadB)
  
          assert.strictEqual(dataset.size, 1)
        })
        */
      })
    })
}

module.exports = runTests
