use crate::models::money::{MonetaryValue, Unit};

pub fn parse_money(raw: &str) -> Option<MonetaryValue> {
    
    let mut s = raw.trim();
    //Remove the currency
    if let Some(str) = s.strip_prefix("€") {
        s = str
    }

    //Match onto the units of the string to get the right units later
    let unit = match s.to_lowercase(){
        _ if s.ends_with("k") => Unit::K,
        _ if s.ends_with("m") => Unit::M,
        _ if s.ends_with("bn") => Unit::Bn,
        _ => Unit::None
    };

    //We renove the Ubits from our string to later get the correct amount from the string
    s = match unit {
        Unit::Bn  => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'b') || c.eq_ignore_ascii_case(&'n')),
        Unit::M   => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'m')),
        Unit::K   => s.trim_end_matches(|c: char| c.eq_ignore_ascii_case(&'k')),
        Unit::None=> s,
    };

    let amount = s.parse::<f32>().ok()?;
    return Some(MonetaryValue { amount, unit });
}