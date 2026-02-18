use crate::math::context::*;

#[cfg(test)]
mod tests;
pub mod err;


/// linear search over a generic iterator.
/// Takes an Iterator and target item of same type as <I as Iterator>::Item
/// Returns an Option of Some(usize) if index is found, otherwise None
pub fn linear_search<I, T>(iter: &mut I, target: T) -> Option<usize> where I: Iterator, <I as Iterator>::Item: PartialEq<T> {

    for (idx, element) in iter.enumerate() {
        if element == target {
            return Some(idx);
        }
    }

    return None;
}

// TODO: expand functionality?
pub fn sanitize_string(data: &String) -> String {
    return data.trim().to_string();
}

pub fn discover_base(data: &String) -> Option<&BaseContext<'_>> {
    
    if data.len() < 3 { return None; } // line too short to get prefix!

    for context in CONTEXT_REGISTRY.contexts {
        if data.as_bytes()[0] as char == context.prefix[0] && data.as_bytes()[1] as char == context.prefix[1] {
            return Some(&context);
        }
    }

    return None;
}