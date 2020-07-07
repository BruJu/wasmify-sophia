
use crate::dataset_exportableds::ExportableDataset;
use sophia::dataset::MutableDataset;
use sophia::dataset::Dataset;


/// A macro that constructs a struct named `$wrapper_dataset` which relies on a
/// `$sophia_dataset` with every default implementation to be exported as an
/// (almost) RDF.JS compliant dataset

pub struct ExportableConcreteDataset<D>
    where D: MutableDataset + Default,
    <D as MutableDataset>::MutationError: From<<D as Dataset>::Error>,
    <D as MutableDataset>::MutationError: From<std::convert::Infallible> {
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

impl<D> crate::dataset_exportableds::ExportableDataset<D>
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
