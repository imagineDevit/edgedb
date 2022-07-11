use proc_macro2::Ident;
pub fn path_ident_equals<'a>(path: &'a syn::Path, i: &str) -> Option<(bool, &'a Ident)> {
    return if path.segments.len() == 1 {
        let ref ident = path.segments[0].ident;

        if ident == i {
            Some((true, ident))
        } else {
            Some((false, ident))
        }
    } else {
        None
    };
}
