use concerto_metamodel::concerto_metamodel_1_0_0 as mm;

/// Sum type for imports, mirroring the JS import hierarchy.
#[derive(Debug, Clone)]
pub enum ImportDecl {
    All(mm::ImportAll),
    Type(mm::ImportType),
    Types(mm::ImportTypes),
}

impl ImportDecl {
    pub fn namespace(&self) -> &str {
        match self {
            Self::All(i) => &i.namespace,
            Self::Type(i) => &i.namespace,
            Self::Types(i) => &i.namespace,
        }
    }

    pub fn uri(&self) -> Option<&str> {
        match self {
            Self::All(i) => i.uri.as_deref(),
            Self::Type(i) => i.uri.as_deref(),
            Self::Types(i) => i.uri.as_deref(),
        }
    }

    /// Returns the fully-qualified names introduced by this import.
    pub fn fully_qualified_names(&self) -> Vec<String> {
        match self {
            Self::All(_) => {
                // ImportAll doesn't enumerate specific types;
                // resolution requires the model manager.
                vec![]
            }
            Self::Type(i) => {
                vec![crate::model_util::fully_qualified_name(
                    &i.namespace,
                    &i.name,
                )]
            }
            Self::Types(i) => i
                .types
                .iter()
                .map(|t| crate::model_util::fully_qualified_name(&i.namespace, t))
                .collect(),
        }
    }

    /// Construct from a serde_json::Value by inspecting `$class`.
    pub fn from_value(value: &serde_json::Value) -> crate::error::Result<Self> {
        let class = value
            .get("$class")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let kind = crate::model_util::get_short_name(class);

        match kind {
            "ImportAll" => Ok(Self::All(serde_json::from_value(value.clone()).map_err(
                |e| crate::error::ConcertoError::IllegalModel {
                    message: format!("Invalid ImportAll: {e}"),
                    file_name: None,
                    location: None,
                },
            )?)),
            "ImportType" => Ok(Self::Type(serde_json::from_value(value.clone()).map_err(
                |e| crate::error::ConcertoError::IllegalModel {
                    message: format!("Invalid ImportType: {e}"),
                    file_name: None,
                    location: None,
                },
            )?)),
            "ImportTypes" => Ok(Self::Types(
                serde_json::from_value(value.clone()).map_err(|e| {
                    crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid ImportTypes: {e}"),
                        file_name: None,
                        location: None,
                    }
                })?,
            )),
            _ => Err(crate::error::ConcertoError::IllegalModel {
                message: format!("Unknown import type: {class}"),
                file_name: None,
                location: None,
            }),
        }
    }
}
