use concerto_metamodel::concerto_metamodel_1_0_0 as mm;

// ===================================================================
// PropertyDecl — sum type replacing JS Property inheritance hierarchy
// ===================================================================

#[derive(Debug, Clone)]
pub enum PropertyDecl {
    Boolean(mm::BooleanProperty),
    String(mm::StringProperty),
    Integer(mm::IntegerProperty),
    Long(mm::LongProperty),
    Double(mm::DoubleProperty),
    DateTime(mm::DateTimeProperty),
    Object(mm::ObjectProperty),
    Relationship(mm::RelationshipProperty),
    Enum(mm::EnumProperty),
}

/// Common accessors shared by all property variants.
impl PropertyDecl {
    pub fn name(&self) -> &str {
        match self {
            Self::Boolean(p) => &p.name,
            Self::String(p) => &p.name,
            Self::Integer(p) => &p.name,
            Self::Long(p) => &p.name,
            Self::Double(p) => &p.name,
            Self::DateTime(p) => &p.name,
            Self::Object(p) => &p.name,
            Self::Relationship(p) => &p.name,
            Self::Enum(p) => &p.name,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Boolean(p) => p.is_array,
            Self::String(p) => p.is_array,
            Self::Integer(p) => p.is_array,
            Self::Long(p) => p.is_array,
            Self::Double(p) => p.is_array,
            Self::DateTime(p) => p.is_array,
            Self::Object(p) => p.is_array,
            Self::Relationship(p) => p.is_array,
            Self::Enum(_) => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::Boolean(p) => p.is_optional,
            Self::String(p) => p.is_optional,
            Self::Integer(p) => p.is_optional,
            Self::Long(p) => p.is_optional,
            Self::Double(p) => p.is_optional,
            Self::DateTime(p) => p.is_optional,
            Self::Object(p) => p.is_optional,
            Self::Relationship(p) => p.is_optional,
            Self::Enum(_) => false,
        }
    }

    /// Returns `true` when the property holds a primitive Concerto type
    /// (Boolean, String, Integer, Long, Double, DateTime).
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Boolean(_)
                | Self::String(_)
                | Self::Integer(_)
                | Self::Long(_)
                | Self::Double(_)
                | Self::DateTime(_)
        )
    }

    /// Returns `true` when this property is a relationship reference.
    pub fn is_relationship(&self) -> bool {
        matches!(self, Self::Relationship(_))
    }

    /// Returns `true` when this is an enum value member.
    pub fn is_enum_value(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    /// For `Object` and `Relationship` properties, returns the referenced
    /// type identifier. For primitives this returns `None`.
    pub fn type_identifier(&self) -> Option<&mm::TypeIdentifier> {
        match self {
            Self::Object(p) => Some(&p.type_),
            Self::Relationship(p) => Some(&p.type_),
            _ => None,
        }
    }

    /// The primitive type name for primitive properties, or the type
    /// reference name for Object/Relationship. `None` only for Enum values.
    pub fn type_name(&self) -> Option<&str> {
        match self {
            Self::Boolean(_) => Some("Boolean"),
            Self::String(_) => Some("String"),
            Self::Integer(_) => Some("Integer"),
            Self::Long(_) => Some("Long"),
            Self::Double(_) => Some("Double"),
            Self::DateTime(_) => Some("DateTime"),
            Self::Object(p) => Some(&p.type_.name),
            Self::Relationship(p) => Some(&p.type_.name),
            Self::Enum(_) => None,
        }
    }

    pub fn decorators(&self) -> &[mm::Decorator] {
        match self {
            Self::Boolean(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::String(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Integer(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Long(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Double(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::DateTime(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Object(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Relationship(p) => p.decorators.as_deref().unwrap_or(&[]),
            Self::Enum(p) => p.decorators.as_deref().unwrap_or(&[]),
        }
    }

    pub fn location(&self) -> Option<&mm::Range> {
        match self {
            Self::Boolean(p) => p.location.as_ref(),
            Self::String(p) => p.location.as_ref(),
            Self::Integer(p) => p.location.as_ref(),
            Self::Long(p) => p.location.as_ref(),
            Self::Double(p) => p.location.as_ref(),
            Self::DateTime(p) => p.location.as_ref(),
            Self::Object(p) => p.location.as_ref(),
            Self::Relationship(p) => p.location.as_ref(),
            Self::Enum(p) => p.location.as_ref(),
        }
    }
}

/// Construct a [`PropertyDecl`] from serde_json by inspecting `$class`.
impl PropertyDecl {
    pub fn from_value(value: &serde_json::Value) -> crate::error::Result<Self> {
        let class = value
            .get("$class")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // The $class field ends with the type name, e.g.
        // "concerto.metamodel@1.0.0.BooleanProperty"
        let kind = crate::model_util::get_short_name(class);

        match kind {
            "BooleanProperty" => Ok(Self::Boolean(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid BooleanProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "StringProperty" => Ok(Self::String(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid StringProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "IntegerProperty" => Ok(Self::Integer(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid IntegerProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "LongProperty" => Ok(Self::Long(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid LongProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "DoubleProperty" => Ok(Self::Double(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid DoubleProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "DateTimeProperty" => Ok(Self::DateTime(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid DateTimeProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "ObjectProperty" => Ok(Self::Object(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid ObjectProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "RelationshipProperty" => Ok(Self::Relationship(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid RelationshipProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            "EnumProperty" => Ok(Self::Enum(
                serde_json::from_value(value.clone())
                    .map_err(|e| crate::error::ConcertoError::IllegalModel {
                        message: format!("Invalid EnumProperty: {e}"),
                        file_name: None,
                        location: None,
                    })?,
            )),
            _ => Err(crate::error::ConcertoError::IllegalModel {
                message: format!("Unknown property type: {class}"),
                file_name: None,
                location: None,
            }),
        }
    }
}
