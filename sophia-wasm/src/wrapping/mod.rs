

// Provides the macro to annotate an ExportableDataset with wasm_bindgen
pub mod macro_to_wasm;

// The trait that describes how to wrap a Sophia Dataset to be exported
mod exportable_dataset;

pub use exportable_dataset::MatchRequestOnRcTerm;
pub use exportable_dataset::ExportableDataset;

// A default wrapping struct
mod default;

pub use default::DefaultExporter;
