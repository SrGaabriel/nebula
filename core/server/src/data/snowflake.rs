use std::fmt::Display;
use sea_orm::{ColIdx, QueryResult, TryFromU64, TryGetError, TryGetable};
use sea_orm::sqlx::{Database, Postgres, Type};
use sea_orm::sqlx::postgres::PgHasArrayType;
use sea_query::{ArrayType, ColumnType, Nullable, Value, ValueType, ValueTypeErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Snowflake(pub u64);

impl Type<Postgres> for Snowflake {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <i64 as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &<Postgres as Database>::TypeInfo) -> bool {
        <i64 as Type<Postgres>>::compatible(ty)
    }
}

impl PgHasArrayType for Snowflake {
    fn array_type_info() -> <Postgres as Database>::TypeInfo {
        <i64 as PgHasArrayType>::array_type_info()
    }
}

impl From<u64> for Snowflake {
    fn from(value: u64) -> Self {
        Snowflake(value)
    }
}

impl ValueType for Snowflake {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::BigInt(Some(i)) if i >= 0 => Ok(Snowflake(i as u64)),
            Value::BigInt(None) => Err(ValueTypeErr),
            _ => Err(ValueTypeErr)
        }
    }

    fn type_name() -> String {
        "BIGINT".to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::BigInt
    }

    fn column_type() -> ColumnType {
        ColumnType::BigInteger
    }
}

impl TryFromU64 for Snowflake {
    fn try_from_u64(v: u64) -> Result<Self, sea_orm::DbErr> {
        Ok(Snowflake(v))
    }
}

impl TryGetable for Snowflake {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let raw: i64 = res.try_get_by(index)?;
        if raw < 0 {
            return Err(TryGetError::DbErr(sea_orm::DbErr::Type(format!("negative value for u64 column"))));
        }
        Ok(Snowflake(raw as u64))
    }
}

impl From<Snowflake> for Value {
    fn from(s: Snowflake) -> Self {
        Value::BigInt(Some(s.0 as i64))
    }
}

impl Nullable for Snowflake {
    fn null() -> Value {
        Value::BigInt(None)
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}