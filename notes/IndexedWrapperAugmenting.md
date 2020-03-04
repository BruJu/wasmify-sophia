# IndexedDatasetWrapper Augmenting

Today (2020-03-01), I tried to implement a way to construct a Sophia Dataset
by wrapping another Sophia Dataset.

Spoiler : I wasn't succesfull

The goal is to be able to write something like

## Trait `Augment`

### Definition

```rust
/// An augmentation of a dataset is the ability to wrap an already existing dataset with a wrapper, effectively
/// augmenting its ability to answer quickly to requests
pub trait Augment<T>
where
    T: IndexedDataset + Dataset,
{
    fn augment(_: T) -> Result<Self, <T as Dataset>::Error>
        where Self: std::marker::Sized;
}
```


### Usage

```rust
pub foo() {
    let lightdataset = get_some_light_dataset();
    let indexed_dataset /* : GspoWrapper<LightDataset> */ = GspoWrapper::augment(lightdataset);
    /* do interesting things */
}
```


## Problem 1 : We can't iterate on indexes

The easiest way to construct an index from an indexed dataset would be to be able to iterate on every quads by using
their indexes

```rust
pub construct_index(wrapped: &mut W, source_graph: &G) {
    source_graph.on_indexed_quads(|s, p, o, g| {
        wrapped.idw_hook_insert_indexed(Some([s, p, o, g]));
    })
}

```

Such `on_indexed_quads` method that returns a quad with the indexes doesn't exist.


## Problem 2 : Can't construct without indexes, can't index without a dataset

If we construct the wrapped dataset first we can't iterate on its quads while modifying the index.

If we construct the index first, we don't have the function to fill it


```rust
impl<T> Augment<T> for GspoWrapper<T>
where
    T: IndexedDataset + Dataset<Quad = ByTermRefs<<T as IndexedDataset>::TermData>>
{
    fn augment(t: T) -> Result<Self, <T as Dataset>::Error> {
        // We don't use ..Default::default() because we don't want to enforce
        // the wrapped dataset to have a default value.
        let mut r = GspoWrapper::<T> {
            wrapped: t, // Must be initialized
            g2s: Default::default(),
            gs2p: Default::default(),
            gsp2o: Default::default()
        };

        let wrapped = r.get_wrapped();
        fill_index(&mut r, wrapped); // r is mutable, but we borrowed one of its member
        Ok(r)
    }
}

fn fill_index<W, G>(w: &mut W, source_graph: &G) -> Result<(), G::Error>
where
    W: IndexedDatasetWrapper<G>,
    G: IndexedDataset + Dataset<Quad = ByTermRefs<<G as IndexedDataset>::TermData>>, 
{
    for q in source_graph.quads() {
        let q = q?;

        let (s, p, o, g) = (q.s(), q.p(), q.o(), q.g());
        let arry = [
            source_graph.get_index(s).unwrap(),
            source_graph.get_index(p).unwrap(),
            source_graph.get_index(o).unwrap(),
            source_graph.get_index_for_graph_name(g).unwrap()
        ];

        w.idw_hook_insert_indexed(&Some(arry));
    }

    Ok(())
}
```




```rust
impl<T> Augment<T> for GspoWrapper<T>
where
    T: IndexedDataset + Dataset<Quad = ByTermRefs<<T as IndexedDataset>::TermData>>
{
    fn augment(t: T) -> Result<Self, <T as Dataset>::Error> {
        // We don't use ..Default::default() because we don't want to enforce
        // the wrapped dataset to have a default value.
        let mut r = GspoWrapper::<T> {
            wrapped: t,
            g2s: Default::default(),
            gs2p: Default::default(),
            gsp2o: Default::default()
        };

        let wrapped = r.get_wrapped();
        fill_index(&mut r);
        Ok(r)
    }
}

fn fill_index<W>(w: &mut W) -> Result<(), <<W as DatasetWrapper>::Wrapped as Dataset>::Error>
where
    W: DatasetWrapper + IndexedDataset + IndexedDatasetWrapper<<W as DatasetWrapper>::Wrapped>,
    <W as DatasetWrapper>::Wrapped : IndexedDataset + Dataset<Quad = ByTermRefs<<W as IndexedDataset>::TermData>>, 
{
    for q in w.get_wrapped().quads() {
        let q = q?;

        let (s, p, o, g) = (q.s(), q.p(), q.o(), q.g());
        let arry = [
            w.get_wrapped().get_index(s).unwrap(),
            w.get_wrapped().get_index(p).unwrap(),
            w.get_wrapped().get_index(o).unwrap(),
            w.get_wrapped().get_index_for_graph_name(g).unwrap()
        ];

        w.idw_hook_insert_indexed(&Some(arry));
    }

    Ok(())
}
```

Doesn't work for the same reason (`w.idw_hook_insert_indexed(&Some(arry))` )