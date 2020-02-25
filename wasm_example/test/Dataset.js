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

    describe('contains', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.contains, 'function')
      })

      it('should contains itself', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)

        const dst = rdf.dataset([quad1, quad2])
        const other_graph = rdf.dataset([quad1, quad2])

        assert(dst.contains(other_graph));
        assert(dst.contains(dst));
      })

      it('should contain an empty graph', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)

        const dst = rdf.dataset([quad1, quad2])
        const an_empty_graph = rdf.dataset();

        assert(dst.contains(an_empty_graph));
        assert(!an_empty_graph.contains(dst));
      })

      it('should contain small graph', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)

        const big = rdf.dataset([quad1, quad2])
        const small = rdf.dataset([quad1])

        assert(big.contains(small));
        assert(!small.contains(big));
      })

      it('should not contain a graph that a differente lement', () => {
        const quad1 = rdf.quad(ex.subject, ex.predicate, ex.object1)
        const quad2 = rdf.quad(ex.subject, ex.predicate, ex.object2)
        const quad3 = rdf.quad(ex.subject, ex.predicate, ex.object3)

        const graph12 = rdf.dataset([quad1, quad2])
        const graph13 = rdf.dataset([quad1, quad3])

        assert(!graph12.contains(graph13));
      })
    })


    describe('deleteMatches', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.deleteMatches, 'function')
      })

      it('delete all by default', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad21 = rdf.quad(ex.subject2, ex.predicate, ex.object1)

        const graph = rdf.dataset([quad11, quad12, quad13, quad21])

        assert.strictEqual(graph.size, 4)
        graph.deleteMatches()
        assert.strictEqual(graph.size, 0)
      })

      it('delete all if removing a shared predicate', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad21 = rdf.quad(ex.subject2, ex.predicate, ex.object1)

        const graph = rdf.dataset([quad11, quad12, quad13, quad21])

        assert.strictEqual(graph.size, 4)
        graph.deleteMatches(undefined, ex.predicate, undefined, undefined)
        assert.strictEqual(graph.size, 0)
      })

      it('delete only matching term', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad21 = rdf.quad(ex.subject2, ex.predicate, ex.object1)

        const graph = rdf.dataset([quad11, quad12, quad13, quad21])

        assert.strictEqual(graph.size, 4)
        graph.deleteMatches(ex.subject1)
        assert.strictEqual(graph.size, 1)
        assert(graph.has(quad21))
      })

      it('delete only matching term (bis)', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad21 = rdf.quad(ex.subject2, ex.predicate, ex.object1)

        const graph = rdf.dataset([quad11, quad12, quad13, quad21])

        assert.strictEqual(graph.size, 4)
        graph.deleteMatches(undefined, undefined, ex.object1)
        assert.strictEqual(graph.size, 2)
        assert(graph.has(quad12))
        assert(graph.has(quad13))
      })

      it('work properly with default graph', () => {
        const in_default = rdf.quad(ex.subject1, ex.predicate, ex.object1, rdf.defaultGraph())
        const in_other = rdf.quad(ex.subject1, ex.predicate, ex.object1, ex.other)

        const graph = rdf.dataset([in_default, in_other])

        graph.deleteMatches(undefined, undefined, undefined, rdf.defaultGraph())
        assert.strictEqual(graph.size, 1)
        assert(graph.has(in_other))
      })

      it('work properly with another graph', () => {
        const in_default = rdf.quad(ex.subject1, ex.predicate, ex.object1, rdf.defaultGraph())
        const in_other = rdf.quad(ex.subject1, ex.predicate, ex.object1, ex.other)
        const in_another = rdf.quad(ex.subject1, ex.predicate, ex.object1, ex.another)

        const graph = rdf.dataset([in_default, in_other, in_another])

        graph.deleteMatches(undefined, undefined, undefined, ex.another)
        assert.strictEqual(graph.size, 2)
        assert(graph.has(in_default))
        assert(graph.has(in_other))
      })
    })

    describe('difference', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.difference, 'function')
      })

      it('have no effect on empty datasets', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graphSrc = rdf.dataset([quad11, quad12, quad13])
        const graphDst = rdf.dataset()

        const graphDiff = graphSrc.difference(graphDst);

        assert.strictEqual(graphDiff.size, 3);
        assert(graphDiff.has(quad11));
        assert(graphDiff.has(quad12));
        assert(graphDiff.has(quad13));
      })

      it('make an empty graph when differencing the same graph', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graphSrc = rdf.dataset([quad11, quad12, quad13])

        const graphDiff = graphSrc.difference(graphSrc);

        assert.strictEqual(graphDiff.size, 0);
      })

      it('differentiate two different graphes', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad23 = rdf.quad(ex.subject2, ex.predicate, ex.object3)

        const graphSrc = rdf.dataset([quad11, quad12, quad13])
        const graphDst = rdf.dataset([quad11, quad23])

        const graphDiff = graphSrc.difference(graphDst);

        assert.strictEqual(graphDiff.size, 2);
        assert(graphDiff.has(quad12));
        assert(graphDiff.has(quad13));
      })
    })

    describe('equals', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.equals, 'function')
      })

      it('should be able to compare empty graphes', () => {
        const graph1 = rdf.dataset()
        const graph2 = rdf.dataset()

        assert(graph1.equals(graph2))
        assert(graph1.equals(graph1))
        assert(graph2.equals(graph1))
      })

      it('should be able to compare two identical graphs', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graph1 = rdf.dataset([quad11, quad12, quad13])
        const graph2 = rdf.dataset([quad11, quad12, quad13])

        assert(graph1.equals(graph2));
        assert(graph1.equals(graph1));
        assert(graph2.equals(graph1));
      })

      it('should be able to compare different graphes', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graph1 = rdf.dataset([quad11, quad12])
        const graph2 = rdf.dataset([quad13])
        const empty_graph = rdf.dataset()

        assert(!graph1.equals(graph2))
        assert(!graph2.equals(graph1))
        assert(!graph1.equals(empty_graph))
        assert(!empty_graph.equals(graph1))
      })

      it('should have a different result when two graphes become identical', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graph1 = rdf.dataset([quad11, quad12])
        const graph2 = rdf.dataset([quad13])

        assert(!graph1.equals(graph2))
        graph2.add(quad11)
        graph2.add(quad12)
        graph1.add(quad13)
        assert(graph1.equals(graph2))
      })
    })

    describe('intersection', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.intersection, 'function')
      })

      it('should be able to intersect empty graphes', () => {
        const graph1 = rdf.dataset()
        const graph2 = rdf.dataset()

        assert.strictEqual(graph1.intersection(graph2).size, 0)
        assert.strictEqual(graph1.intersection(graph1).size, 0)
      })

      it('should not modify the intersected graphes', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const graph1 = rdf.dataset([quad11, quad12])
        const graph2 = rdf.dataset([quad13])

        graph1.intersection(graph2)

        assert(graph1.equals([quad11, quad12]))
        assert(graph2.equals([quad13]))
      })

      it('should be able to intersect the same graph', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)

        const graph1 = rdf.dataset([quad11, quad12, quad13])
        const graph2 = rdf.dataset([quad11, quad12, quad13])

        const intersection11 = graph1.intersection(graph1)
        const intersection12 = graph1.intersection(graph2)

        assert.strictEqual(graph1.size, 3)
        assert(graph1.equals(intersection11))
        assert(graph1.equals(intersection12))
      })

      it('should be able to intersect two disjoint graphes', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad14 = rdf.quad(ex.subject1, ex.predicate, ex.object4)

        const graph1 = rdf.dataset([quad11, quad12])
        const graph2 = rdf.dataset([quad13, quad14])

        const intersection = graph1.intersection(graph2)

        assert.strictEqual(intersection.size, 0)
      })

      it('should be able to intersect two graphes', () => {
        const quad11 = rdf.quad(ex.subject1, ex.predicate, ex.object1)
        const quad12 = rdf.quad(ex.subject1, ex.predicate, ex.object2)
        const quad13 = rdf.quad(ex.subject1, ex.predicate, ex.object3)
        const quad14 = rdf.quad(ex.subject1, ex.predicate, ex.object4)
        const quadCommon = rdf.quad(ex.common, ex.come, ex.on)
        const quadAnotherCommon = rdf.quad(ex.anothercommon, ex.come, ex.on)

        const graph1 = rdf.dataset([quad11, quad12, quadCommon, quadAnotherCommon])
        const graph2 = rdf.dataset([quad13, quad14, quadCommon, quadAnotherCommon])

        const intersection = graph1.intersection(graph2)

        assert.strictEqual(intersection.size, 2)
        assert(intersection.equals(rdf.dataset([quadCommon, quadAnotherCommon])))
      })

    })

    describe('some', () => {
      it('should be a function', () => {
        const dataset = rdf.dataset()

        assert.strictEqual(typeof dataset.some, 'function')
      })

      it('should find existing elements', () => {
        const quad = rdf.quad(ex.subject, ex.predicate, ex.object)
        const graph = rdf.dataset([quad])

        assert(graph.some(q => quad.equals(q)));
        assert(graph.some(q => q.subject.value == ex.subject.value));
        assert(!graph.some(q => q.subject.value == ex.subject145.value));
      })
    })
}

module.exports = runTests
