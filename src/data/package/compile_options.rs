use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompileOptions {
    /// Additional include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paths: Option<Vec<String>>,

    /// Additional system include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_includes: Option<Vec<String>>,

    /// Additional C++ features to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_features: Option<Vec<String>>,

    /// Additional C++ flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_flags: Option<Vec<String>>,

    /// Additional C flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_flags: Option<Vec<String>>,
}
