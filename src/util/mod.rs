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

    None
}

// TODO: expand functionality
pub fn sanitize_string(data: &String) -> String {
    return data.trim().to_string();
}

pub fn discover_base(data: &String) -> Option<(&[char], &[char; 2])> {
    
    // TODO: update to pull from a static struct array  
    for context in vec![&CONTEXT_B16,&CONTEXT_B8] {
        if data.as_bytes()[0] as char == context.prefix[0] && data.as_bytes()[1] as char == context.prefix[1] {
            // found!
            return Some((&context.charset,&context.prefix));
        }
    }

    return None;
}