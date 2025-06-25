use crate::models::money::{MonetaryValue, Unit};

pub fn parse_money(raw: &str) -> Option<MonetaryValue> {
    
    let mut s = raw.trim();
    if let Some(str) = s.strip_prefix("€") {
        s = str
    }

    let unit = match s.to_lowercase(){
        _ if s.ends_with("k") => Unit::K,
        _ if s.ends_with("m") => Unit::M,
        _ if s.ends_with("bn") => Unit::Bn,
        _ => Unit::None
    };

    s = match unit {
        Unit::Bn  => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'b') || c.eq_ignore_ascii_case(&'n')),
        Unit::M   => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'m')),
        Unit::K   => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'k')),
        Unit::None=> s,
    };

    let amount = s.parse::<f32>().ok()?;
    return Some(MonetaryValue { amount, unit });
}