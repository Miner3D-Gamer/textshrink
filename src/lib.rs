mod internal;

pub fn replace_common(
    string: &str,
    watch_casing: bool,
    loosely: bool,
) -> String {
    if watch_casing {
        if loosely {
            return internal::optimize_loosely_without_watching_case(
                string,
                &internal::get_common(),
            );
        } else {
            return internal::optimize_without_watching_case(
                string,
                &internal::get_common(),
            );
        }
    } else {
        if loosely {
            return internal::optimize_loosely(string, &internal::get_common());
        } else {
            return internal::optimize(string, &internal::get_common());
        }
    }
}
pub fn replace_uncommon(
    string: &str,
    watch_casing: bool,
    loosely: bool,
) -> String {
    if watch_casing {
        if loosely {
            return internal::optimize_loosely_without_watching_case(
                string,
                &internal::get_uncommon(),
            );
        } else {
            return internal::optimize_without_watching_case(
                string,
                &internal::get_uncommon(),
            );
        }
    } else {
        if loosely {
            return internal::optimize_loosely(
                string,
                &internal::get_uncommon(),
            );
        } else {
            return internal::optimize(string, &internal::get_uncommon());
        }
    }
}

pub fn replace_all(string: &str, watch_casing: bool, loosely: bool) -> String {
    return replace_common(
        &replace_uncommon(string, watch_casing, loosely),
        watch_casing,
        loosely,
    );
}
