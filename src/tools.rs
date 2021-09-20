#[doc(hidden)]
#[macro_export]
macro_rules! char_millis_to_utc {
    ($ts: expr) => {
        $ts.parse::<i64>()
            .map_or(None, |lt| match Utc.timestamp_millis_opt(lt) {
                LocalResult::Single(dt) => Some(dt),
                _ => None,
            })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! seconds_to_utc {
    ($ts: expr) => {
        match Utc.timestamp_opt($ts, 0_u32) {
            LocalResult::Single(dt) => Some(dt),
            _ => None,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! millis_to_utc {
    ($ts: expr) => {
        match Utc.timestamp_millis_opt($ts) {
            LocalResult::Single(dt) => Some(dt),
            _ => None,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! valid_list {
    ($val: expr, $( $elem: expr ),+ ) => {
        match $val.as_ref() {
            $(src @ $elem => Some(src.to_string()),)+
            _ => None
        }
    }
}
