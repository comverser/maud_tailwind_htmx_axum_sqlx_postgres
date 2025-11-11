use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub fn format_price(amount: i32) -> String {
    amount
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("Price formatting should always produce valid UTF-8")
        .join(",")
}

pub fn format_datetime(dt: OffsetDateTime) -> String {
    let formatted_date = dt.format(&Rfc3339).unwrap_or("Invalid date".to_string());
    let datetime_parts: Vec<&str> = formatted_date.split('T').collect();
    let date_part = datetime_parts.first().unwrap_or(&"");
    let time_part = datetime_parts.get(1).and_then(|t| t.split('.').next()).unwrap_or("");
    if !time_part.is_empty() {
        format!("{} {}", date_part, time_part)
    } else {
        date_part.to_string()
    }
}
