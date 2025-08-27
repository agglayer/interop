use std::fmt;

use agglayer_interop_types::primitives::SignatureError;
use tonic_types::FieldViolation;

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error(transparent)]
    Bincode(#[from] bincode::Error),

    #[error(transparent)]
    Signature(#[from] SignatureError),
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    InvalidData,
    MissingField,
}

impl fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::InvalidData => {
                write!(f, "Invalid data")
            }
            ErrorKind::MissingField => {
                write!(f, "Missing field")
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    field: Vec<&'static str>,
    #[source]
    source: Option<SourceError>,
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.field.is_empty() {
            write!(f, "{}: ", self.field_str())?;
        }
        write!(f, "{}", self.message)
    }
}

impl Error {
    #[inline]
    pub fn missing_field(f: &'static str) -> Self {
        Error {
            kind: ErrorKind::MissingField,
            message: "required field is missing".to_string(),
            field: vec![f],
            source: None,
        }
    }

    #[inline]
    pub fn invalid_data(m: String) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: m,
            field: vec![],
            source: None,
        }
    }

    #[inline]
    pub fn inside_field(mut self, f: &'static str) -> Self {
        self.field.push(f);
        self.field.rotate_right(1);
        self
    }

    #[inline]
    pub fn serializing_proof(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to serialize proof".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn serializing_vkey(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to serialize vkey".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn serializing_context(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to serialize context".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn deserializing_proof(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to deserialize proof".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn deserializing_vkey(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to deserialize vkey".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn deserializing_aggchain_proof_public_values(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to deserialize aggchain proof public values".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn serializing_aggchain_proof_public_values(e: bincode::Error) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to serialize aggchain proof public values".to_string(),
            field: vec![],
            source: Some(SourceError::Bincode(e)),
        }
    }

    #[inline]
    pub fn parsing_signature(e: SignatureError) -> Self {
        Error {
            kind: ErrorKind::InvalidData,
            message: "failed to parse signature".to_string(),
            field: vec![],
            source: Some(SourceError::Signature(e)),
        }
    }

    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[inline]
    pub fn field(&self) -> &[&'static str] {
        &self.field
    }

    #[inline]
    pub fn field_str(&self) -> String {
        if self.field.is_empty() {
            ".".to_string()
        } else {
            self.field.join(".")
        }
    }
}

impl From<&Error> for Vec<FieldViolation> {
    #[inline]
    fn from(value: &Error) -> Self {
        vec![FieldViolation::new(
            value.field_str(),
            value.message.clone(),
        )]
    }
}
