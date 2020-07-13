
use bjdatasets::treeddataset::TreedDataset;
use crate::dataset_exportableds::MatchRequestOnRcTerm;
use crate::dataset_exportableds::ExportableDataset;
use crate::datamodel_term::JsImportTerm;
use sophia::dataset::Dataset;
use sophia::quad::stream::QuadSource;

/// An exportable dataset to Web Assembly that priorizes indexes that are are
/// the opposite of the one used by the source dataset when building a new
/// dataset with the `match_quad` function.
pub struct BTreedDatasetAntiWrapper {
    base: TreedDataset
}

impl Default for BTreedDatasetAntiWrapper {
    fn default() -> Self {
        Self { base: TreedDataset::default() }
    }
}

impl ExportableDataset<TreedDataset> for BTreedDatasetAntiWrapper {
    fn wrap(dataset: TreedDataset) -> Self {
        Self { base : dataset }
    }

    fn dataset(&self) -> &TreedDataset {
        &self.base
    }

    fn mutable_dataset(&mut self) -> &mut TreedDataset {
        &mut self.base
    }

    fn match_quad(&self, subject: &JsImportTerm, predicate: &JsImportTerm, object: &JsImportTerm, graph: &JsImportTerm) -> Self {
        let m = MatchRequestOnRcTerm::new(subject, predicate, object, graph);
        let mut quads_iter = self.dataset().quads_matching(&m.s, &m.p, &m.o, &m.g);

        let s_is_some = match &m.s { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let p_is_some = match &m.p { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let o_is_some = match &m.o { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
        let g_is_some = match &m.g { sophia::term::matcher::AnyOrExactly::Any => false, _ => true };
    
        let mut dataset = TreedDataset::new_anti(s_is_some, p_is_some, o_is_some, g_is_some);
        quads_iter.in_dataset(&mut dataset).unwrap();
    
        Self::wrap(dataset)
    }
}
