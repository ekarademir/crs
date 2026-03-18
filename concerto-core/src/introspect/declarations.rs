use concerto_metamodel::concerto_metamodel_1_0_0 as mm;

use super::properties::PropertyDecl;
use super::traits::{Decorated, HasProperties, Identifiable, Named};

// ===================================================================
// Newtype wrappers for each metamodel declaration struct
// ===================================================================

/// Wraps `mm::ConceptDeclaration`.
#[derive(Debug, Clone)]
pub struct ConceptDeclaration {
    pub(crate) inner: mm::ConceptDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::AssetDeclaration`.
#[derive(Debug, Clone)]
pub struct AssetDeclaration {
    pub(crate) inner: mm::AssetDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::ParticipantDeclaration`.
#[derive(Debug, Clone)]
pub struct ParticipantDeclaration {
    pub(crate) inner: mm::ParticipantDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::TransactionDeclaration`.
#[derive(Debug, Clone)]
pub struct TransactionDeclaration {
    pub(crate) inner: mm::TransactionDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::EventDeclaration`.
#[derive(Debug, Clone)]
pub struct EventDeclaration {
    pub(crate) inner: mm::EventDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::EnumDeclaration`.
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub(crate) inner: mm::EnumDeclaration,
    pub(crate) properties: Vec<PropertyDecl>,
}

/// Wraps `mm::MapDeclaration`.
#[derive(Debug, Clone)]
pub struct MapDeclaration(pub(crate) mm::MapDeclaration);

// ===================================================================
// ScalarDeclKind — sum over scalar sub-types
// ===================================================================

#[derive(Debug, Clone)]
pub enum ScalarDeclKind {
    Boolean(mm::BooleanScalar),
    Integer(mm::IntegerScalar),
    Long(mm::LongScalar),
    Double(mm::DoubleScalar),
    String(mm::StringScalar),
    DateTime(mm::DateTimeScalar),
}

/// Wraps one of the six metamodel scalar types.
#[derive(Debug, Clone)]
pub struct ScalarDeclaration(pub(crate) ScalarDeclKind);

// ===================================================================
// ClassDeclaration — sum type replacing JS inheritance subtree
// ===================================================================

#[derive(Debug, Clone)]
pub enum ClassDeclaration {
    Concept(ConceptDeclaration),
    Asset(AssetDeclaration),
    Participant(ParticipantDeclaration),
    Transaction(TransactionDeclaration),
    Event(EventDeclaration),
    Enum(EnumDeclaration),
}

// ===================================================================
// Declaration — top-level sum type
// ===================================================================

#[derive(Debug, Clone)]
pub enum Declaration {
    Class(ClassDeclaration),
    Scalar(ScalarDeclaration),
    Map(MapDeclaration),
}

// ===================================================================
// Trait impls — Decorated / Named for each newtype
// ===================================================================

macro_rules! impl_decorated_for_inner {
    ($ty:ty, $field:ident) => {
        impl Decorated for $ty {
            fn decorators(&self) -> &[mm::Decorator] {
                self.inner.$field.as_deref().unwrap_or(&[])
            }
        }
    };
}

impl_decorated_for_inner!(ConceptDeclaration, decorators);
impl_decorated_for_inner!(AssetDeclaration, decorators);
impl_decorated_for_inner!(ParticipantDeclaration, decorators);
impl_decorated_for_inner!(TransactionDeclaration, decorators);
impl_decorated_for_inner!(EventDeclaration, decorators);
impl_decorated_for_inner!(EnumDeclaration, decorators);

impl Decorated for MapDeclaration {
    fn decorators(&self) -> &[mm::Decorator] {
        self.0.decorators.as_deref().unwrap_or(&[])
    }
}

// --- Named ---

macro_rules! impl_named_for_inner {
    ($ty:ty) => {
        impl Named for $ty {
            fn name(&self) -> &str {
                &self.inner.name
            }
            fn location(&self) -> Option<&mm::Range> {
                self.inner.location.as_ref()
            }
        }
    };
}

impl_named_for_inner!(ConceptDeclaration);
impl_named_for_inner!(AssetDeclaration);
impl_named_for_inner!(ParticipantDeclaration);
impl_named_for_inner!(TransactionDeclaration);
impl_named_for_inner!(EventDeclaration);
impl_named_for_inner!(EnumDeclaration);

impl Named for MapDeclaration {
    fn name(&self) -> &str {
        &self.0.name
    }
    fn location(&self) -> Option<&mm::Range> {
        self.0.location.as_ref()
    }
}

// --- HasProperties ---

macro_rules! impl_has_properties {
    ($ty:ty) => {
        impl HasProperties for $ty {
            fn own_properties(&self) -> &[PropertyDecl] {
                &self.properties
            }
            fn super_type(&self) -> Option<&mm::TypeIdentifier> {
                self.inner.super_type.as_ref()
            }
            fn is_abstract(&self) -> bool {
                self.inner.is_abstract
            }
        }
    };
}

impl_has_properties!(ConceptDeclaration);
impl_has_properties!(AssetDeclaration);
impl_has_properties!(ParticipantDeclaration);
impl_has_properties!(TransactionDeclaration);
impl_has_properties!(EventDeclaration);

impl HasProperties for EnumDeclaration {
    fn own_properties(&self) -> &[PropertyDecl] {
        &self.properties
    }
    fn super_type(&self) -> Option<&mm::TypeIdentifier> {
        None
    }
    fn is_abstract(&self) -> bool {
        false
    }
}

// --- Identifiable ---

fn identified_from_ast(identified: &Option<mm::Identified>) -> (bool, bool, Option<&str>) {
    match identified {
        None => (false, false, None),
        Some(id) => {
            // The `Identified` struct just signals "system identified",
            // while `IdentifiedBy` (encoded by checking the name field in
            // the wrapper) would carry the explicit field name.
            //
            // In the metamodel the $class field discriminates:
            //   concerto.metamodel@1.0.0.IdentifiedBy  (has `name`)
            //   concerto.metamodel@1.0.0.Identified     (system)
            let is_explicit =
                id._class.ends_with(".IdentifiedBy") || id._class.contains("IdentifiedBy");
            if is_explicit {
                // IdentifiedBy – but the metamodel `Identified` struct
                // doesn't have a `name` field; the JS side stuffs it into
                // an extra IdentifiedBy struct.  For now we return true
                // and callers will search the properties for $identifier.
                (true, false, None)
            } else {
                (true, true, None)
            }
        }
    }
}

macro_rules! impl_identifiable {
    ($ty:ty) => {
        impl Identifiable for $ty {
            fn is_identified(&self) -> bool {
                identified_from_ast(&self.inner.identified).0
            }
            fn is_system_identified(&self) -> bool {
                identified_from_ast(&self.inner.identified).1
            }
            fn is_explicitly_identified(&self) -> bool {
                let (ident, sys, _) = identified_from_ast(&self.inner.identified);
                ident && !sys
            }
            fn identifier_field_name(&self) -> Option<&str> {
                if self.is_system_identified() {
                    Some("$identifier")
                } else {
                    identified_from_ast(&self.inner.identified).2
                }
            }
        }
    };
}

impl_identifiable!(ConceptDeclaration);
impl_identifiable!(AssetDeclaration);
impl_identifiable!(ParticipantDeclaration);
impl_identifiable!(TransactionDeclaration);
impl_identifiable!(EventDeclaration);

impl Identifiable for EnumDeclaration {
    fn is_identified(&self) -> bool {
        false
    }
    fn is_system_identified(&self) -> bool {
        false
    }
    fn is_explicitly_identified(&self) -> bool {
        false
    }
    fn identifier_field_name(&self) -> Option<&str> {
        None
    }
}

// ===================================================================
// Decorated / Named for ScalarDeclaration
// ===================================================================

impl Decorated for ScalarDeclaration {
    fn decorators(&self) -> &[mm::Decorator] {
        match &self.0 {
            ScalarDeclKind::Boolean(s) => s.decorators.as_deref().unwrap_or(&[]),
            ScalarDeclKind::Integer(s) => s.decorators.as_deref().unwrap_or(&[]),
            ScalarDeclKind::Long(s) => s.decorators.as_deref().unwrap_or(&[]),
            ScalarDeclKind::Double(s) => s.decorators.as_deref().unwrap_or(&[]),
            ScalarDeclKind::String(s) => s.decorators.as_deref().unwrap_or(&[]),
            ScalarDeclKind::DateTime(s) => s.decorators.as_deref().unwrap_or(&[]),
        }
    }
}

impl Named for ScalarDeclaration {
    fn name(&self) -> &str {
        match &self.0 {
            ScalarDeclKind::Boolean(s) => &s.name,
            ScalarDeclKind::Integer(s) => &s.name,
            ScalarDeclKind::Long(s) => &s.name,
            ScalarDeclKind::Double(s) => &s.name,
            ScalarDeclKind::String(s) => &s.name,
            ScalarDeclKind::DateTime(s) => &s.name,
        }
    }
    fn location(&self) -> Option<&mm::Range> {
        match &self.0 {
            ScalarDeclKind::Boolean(s) => s.location.as_ref(),
            ScalarDeclKind::Integer(s) => s.location.as_ref(),
            ScalarDeclKind::Long(s) => s.location.as_ref(),
            ScalarDeclKind::Double(s) => s.location.as_ref(),
            ScalarDeclKind::String(s) => s.location.as_ref(),
            ScalarDeclKind::DateTime(s) => s.location.as_ref(),
        }
    }
}

impl ScalarDeclaration {
    /// Returns the primitive type that this scalar wraps.
    pub fn scalar_type(&self) -> &str {
        match &self.0 {
            ScalarDeclKind::Boolean(_) => "Boolean",
            ScalarDeclKind::Integer(_) => "Integer",
            ScalarDeclKind::Long(_) => "Long",
            ScalarDeclKind::Double(_) => "Double",
            ScalarDeclKind::String(_) => "String",
            ScalarDeclKind::DateTime(_) => "DateTime",
        }
    }

    pub fn kind(&self) -> &ScalarDeclKind {
        &self.0
    }
}

// ===================================================================
// Dispatch on ClassDeclaration sum type
// ===================================================================

impl Decorated for ClassDeclaration {
    fn decorators(&self) -> &[mm::Decorator] {
        match self {
            Self::Concept(d) => d.decorators(),
            Self::Asset(d) => d.decorators(),
            Self::Participant(d) => d.decorators(),
            Self::Transaction(d) => d.decorators(),
            Self::Event(d) => d.decorators(),
            Self::Enum(d) => d.decorators(),
        }
    }
}

impl Named for ClassDeclaration {
    fn name(&self) -> &str {
        match self {
            Self::Concept(d) => d.name(),
            Self::Asset(d) => d.name(),
            Self::Participant(d) => d.name(),
            Self::Transaction(d) => d.name(),
            Self::Event(d) => d.name(),
            Self::Enum(d) => d.name(),
        }
    }
    fn location(&self) -> Option<&mm::Range> {
        match self {
            Self::Concept(d) => d.location(),
            Self::Asset(d) => d.location(),
            Self::Participant(d) => d.location(),
            Self::Transaction(d) => d.location(),
            Self::Event(d) => d.location(),
            Self::Enum(d) => d.location(),
        }
    }
}

impl HasProperties for ClassDeclaration {
    fn own_properties(&self) -> &[PropertyDecl] {
        match self {
            Self::Concept(d) => d.own_properties(),
            Self::Asset(d) => d.own_properties(),
            Self::Participant(d) => d.own_properties(),
            Self::Transaction(d) => d.own_properties(),
            Self::Event(d) => d.own_properties(),
            Self::Enum(d) => d.own_properties(),
        }
    }
    fn super_type(&self) -> Option<&mm::TypeIdentifier> {
        match self {
            Self::Concept(d) => d.super_type(),
            Self::Asset(d) => d.super_type(),
            Self::Participant(d) => d.super_type(),
            Self::Transaction(d) => d.super_type(),
            Self::Event(d) => d.super_type(),
            Self::Enum(d) => d.super_type(),
        }
    }
    fn is_abstract(&self) -> bool {
        match self {
            Self::Concept(d) => d.is_abstract(),
            Self::Asset(d) => d.is_abstract(),
            Self::Participant(d) => d.is_abstract(),
            Self::Transaction(d) => d.is_abstract(),
            Self::Event(d) => d.is_abstract(),
            Self::Enum(d) => d.is_abstract(),
        }
    }
}

impl Identifiable for ClassDeclaration {
    fn is_identified(&self) -> bool {
        match self {
            Self::Concept(d) => d.is_identified(),
            Self::Asset(d) => d.is_identified(),
            Self::Participant(d) => d.is_identified(),
            Self::Transaction(d) => d.is_identified(),
            Self::Event(d) => d.is_identified(),
            Self::Enum(d) => d.is_identified(),
        }
    }
    fn is_system_identified(&self) -> bool {
        match self {
            Self::Concept(d) => d.is_system_identified(),
            Self::Asset(d) => d.is_system_identified(),
            Self::Participant(d) => d.is_system_identified(),
            Self::Transaction(d) => d.is_system_identified(),
            Self::Event(d) => d.is_system_identified(),
            Self::Enum(d) => d.is_system_identified(),
        }
    }
    fn is_explicitly_identified(&self) -> bool {
        match self {
            Self::Concept(d) => d.is_explicitly_identified(),
            Self::Asset(d) => d.is_explicitly_identified(),
            Self::Participant(d) => d.is_explicitly_identified(),
            Self::Transaction(d) => d.is_explicitly_identified(),
            Self::Event(d) => d.is_explicitly_identified(),
            Self::Enum(d) => d.is_explicitly_identified(),
        }
    }
    fn identifier_field_name(&self) -> Option<&str> {
        match self {
            Self::Concept(d) => d.identifier_field_name(),
            Self::Asset(d) => d.identifier_field_name(),
            Self::Participant(d) => d.identifier_field_name(),
            Self::Transaction(d) => d.identifier_field_name(),
            Self::Event(d) => d.identifier_field_name(),
            Self::Enum(d) => d.identifier_field_name(),
        }
    }
}

impl ClassDeclaration {
    /// Returns the declaration kind string, matching JS `declarationKind()`.
    pub fn declaration_kind(&self) -> &'static str {
        match self {
            Self::Concept(_) => "ConceptDeclaration",
            Self::Asset(_) => "AssetDeclaration",
            Self::Participant(_) => "ParticipantDeclaration",
            Self::Transaction(_) => "TransactionDeclaration",
            Self::Event(_) => "EventDeclaration",
            Self::Enum(_) => "EnumDeclaration",
        }
    }

    pub fn is_concept(&self) -> bool {
        matches!(self, Self::Concept(_))
    }
    pub fn is_asset(&self) -> bool {
        matches!(self, Self::Asset(_))
    }
    pub fn is_participant(&self) -> bool {
        matches!(self, Self::Participant(_))
    }
    pub fn is_transaction(&self) -> bool {
        matches!(self, Self::Transaction(_))
    }
    pub fn is_event(&self) -> bool {
        matches!(self, Self::Event(_))
    }
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
}

// ===================================================================
// Dispatch on Declaration sum type
// ===================================================================

impl Decorated for Declaration {
    fn decorators(&self) -> &[mm::Decorator] {
        match self {
            Self::Class(d) => d.decorators(),
            Self::Scalar(d) => d.decorators(),
            Self::Map(d) => d.decorators(),
        }
    }
}

impl Named for Declaration {
    fn name(&self) -> &str {
        match self {
            Self::Class(d) => d.name(),
            Self::Scalar(d) => d.name(),
            Self::Map(d) => d.name(),
        }
    }
    fn location(&self) -> Option<&mm::Range> {
        match self {
            Self::Class(d) => d.location(),
            Self::Scalar(d) => d.location(),
            Self::Map(d) => d.location(),
        }
    }
}

impl Declaration {
    pub fn is_class(&self) -> bool {
        matches!(self, Self::Class(_))
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    pub fn as_class(&self) -> Option<&ClassDeclaration> {
        match self {
            Self::Class(c) => Some(c),
            _ => None,
        }
    }
    pub fn as_scalar(&self) -> Option<&ScalarDeclaration> {
        match self {
            Self::Scalar(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_map(&self) -> Option<&MapDeclaration> {
        match self {
            Self::Map(m) => Some(m),
            _ => None,
        }
    }
}
