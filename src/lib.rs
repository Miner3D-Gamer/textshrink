mod internal;

/// Replace the text using more common character sequences
pub fn replace(
    string: &str,
    watch_casing: bool,
    loosely: bool,
) -> String {
    if watch_casing {
        if loosely {
            return internal::optimize_loosely_without_watching_case(
                string,
                &internal::get_replacements(),
                internal::get_alias(),
            );
        } else {
            return internal::optimize_without_watching_case(
                string,
                &internal::get_replacements(),
            );
        }
    } else {
        if loosely {
            return internal::optimize_loosely(
                string,
                &internal::get_replacements(),
                internal::get_alias(),
            );
        } else {
            return internal::optimize(string, &internal::get_replacements());
        }
    }
}

/// Replace the text with a custom hashmap easily created with [crate::internal::convert_to_replaceable]
/// The alias vec contains equivilants -> * can also be used as °, to get the default aliases use [crate::internal::get_alias]
pub fn replace_custom(
    string: &str,
    watch_casing: bool,
    loosely: bool,
    map: &std::collections::HashMap<String, String>,
    aliases: Vec<Vec<String>>,
) -> String {
    if watch_casing {
        if loosely {
            return internal::optimize_loosely_without_watching_case(
                string, map, aliases,
            );
        } else {
            return internal::optimize_without_watching_case(string, map);
        }
    } else {
        if loosely {
            return internal::optimize_loosely(string, map, aliases);
        } else {
            return internal::optimize(string, map);
        }
    }
}
