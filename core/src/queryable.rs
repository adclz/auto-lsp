pub const fn check_conflicts(
    input_name: &str,
    fields: &'static [&'static str],
    names: &'static [&'static str],
) {
    let mut i = 0;
    while i < names.len() {
        let mut j = i + 1;
        while j < names.len() {
            const_panic::concat_assert!(
                !const_str::equal!(names[i], names[j]),
                "\n\n\n**** Conflicting Queries detected! ****\n\n",
                "\nSymbol -->           ",
                input_name,
                "\nQuery name -->       ",
                names[i],
                "\n\n\n1# : ",
                fields[i],
                "\n2# : ",
                fields[j],
                "\n\n\n\n"
            );
            j += 1;
        }
        i += 1;
    }
}

pub trait Queryable {
    const QUERY_NAMES: &'static [&'static str];
}

pub trait CheckQueryable {
    const CHECK: ();
}
