
use crate::dataset_exportableds::ExportableDataset;
use sophia::dataset::MutableDataset;
use sophia::dataset::Dataset;

/// A basic implementation of `ExportableDataset` that uses a `Dataset` from
/// Sophia and uses the naive implementation of every exported methods
pub struct ExportableConcreteDataset<D>
    where D: MutableDataset + Default,
    <D as MutableDataset>::MutationError: From<<D as Dataset>::Error>,
    <D as MutableDataset>::MutationError: From<std::convert::Infallible> {
    /// The Sophia Dataset that actually contains the quads
    base: D
}

impl<D> Default for ExportableConcreteDataset<D>
    where D: MutableDataset + Default,
    <D as MutableDataset>::MutationError: From<<D as Dataset>::Error>,
    <D as MutableDataset>::MutationError: From<std::convert::Infallible> {
    fn default() -> Self {
        Self { base: D::default() }
    }
}

impl<D> ExportableDataset<D>
    for ExportableConcreteDataset<D>
    where D: MutableDataset + Default,
    <D as MutableDataset>::MutationError: From<<D as Dataset>::Error>,
    <D as MutableDataset>::MutationError: From<std::convert::Infallible> {
    fn dataset(&self) -> &D {
        &self.base
    }

    fn mutable_dataset(&mut self) -> &mut D {
        &mut self.base
    }

    fn wrap(base: D) -> Self {
        Self { base }
    }
}
