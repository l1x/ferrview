use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

pub fn get_utc_formatter() -> &'static [FormatItem<'static>] {
    format_description!(
        "[year]-[month padding:zero]-[day padding:zero]T[hour padding:zero]:[minute padding:zero]:[second padding:zero]Z"
    )
}

pub fn get_utc_timestamp() -> String {
    OffsetDateTime::now_utc()
        .format(get_utc_formatter())
        .unwrap()
}
