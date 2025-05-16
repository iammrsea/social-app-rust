use async_graphql::*;
use chrono::{DateTime, Utc};

pub struct DateTimeScalar(DateTime<Utc>);

#[Scalar(name = "DateTime")]
impl ScalarType for DateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            DateTime::parse_from_rfc3339(s)
                .map_err(|e| InputValueError::custom(format!("Invalid DateTime format: {}", e)))
                .map(|dt| dt.with_timezone(&Utc))
                .map(|dt| DateTimeScalar(dt))
        } else {
            Err(InputValueError::custom("Expected a string"))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for DateTimeScalar {
    fn from(value: DateTime<Utc>) -> Self {
        DateTimeScalar(value)
    }
}

impl Into<DateTime<Utc>> for DateTimeScalar {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}
