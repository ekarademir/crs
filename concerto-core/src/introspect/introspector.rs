use crate::error::Result;
use crate::introspect::declarations::ClassDeclaration;
use crate::model_manager::ModelManager;

/// Provides high-level introspection queries over a [`ModelManager`].
pub struct Introspector<'a> {
    model_manager: &'a ModelManager,
}

impl<'a> Introspector<'a> {
    pub fn new(model_manager: &'a ModelManager) -> Self {
        Self { model_manager }
    }

    /// Returns all class declarations across all loaded models.
    pub fn class_declarations(&self) -> Vec<&ClassDeclaration> {
        self.model_manager.class_declarations()
    }

    /// Look up a class declaration by fully-qualified name.
    pub fn get_class_declaration(&self, fqn: &str) -> Result<&ClassDeclaration> {
        self.model_manager.get_class_declaration(fqn)
    }

    pub fn model_manager(&self) -> &ModelManager {
        self.model_manager
    }
}
