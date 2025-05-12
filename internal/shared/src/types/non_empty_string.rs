use std::{fmt::Display, ops::Deref};

use crate::errors::app::AppError;

use super::AppResult;

#[derive(Debug, Clone, PartialEq)]
pub struct NonEmptyString {
    value: String,
}

impl NonEmptyString {
    pub fn new(value: String) -> AppResult<Self> {
        if value.trim() == "" {
            return Err(AppError::NonEmptyString);
        }
        Ok(Self { value })
    }
}
impl Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<NonEmptyString> for String {
    fn from(nes: NonEmptyString) -> Self {
        nes.value
    }
}

impl From<&NonEmptyString> for String {
    fn from(nes: &NonEmptyString) -> Self {
        nes.value.clone()
    }
}

impl Deref for NonEmptyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PartialEq<str> for NonEmptyString {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialEq<&str> for NonEmptyString {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}
