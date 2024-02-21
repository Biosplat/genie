use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalData {
    date: DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    close: Decimal,
    adjusted_close: Decimal,
    volume: Decimal,
}