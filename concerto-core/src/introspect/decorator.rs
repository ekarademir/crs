use concerto_metamodel::concerto_metamodel_1_0_0 as mm;

/// Newtype wrapper around the metamodel `Decorator`.
#[derive(Debug, Clone)]
pub struct Decorator(pub(crate) mm::Decorator);

impl Decorator {
    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn arguments(&self) -> &[mm::DecoratorLiteral] {
        self.0.arguments.as_deref().unwrap_or(&[])
    }

    pub fn location(&self) -> Option<&mm::Range> {
        self.0.location.as_ref()
    }

    pub fn inner(&self) -> &mm::Decorator {
        &self.0
    }
}

impl From<mm::Decorator> for Decorator {
    fn from(d: mm::Decorator) -> Self {
        Self(d)
    }
}
