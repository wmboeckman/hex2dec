use crate::{math::{context::*, conversion::*}, util::err::ConversionErrors};

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

pub fn conv2dec(line: &String) -> Result<usize, ConversionErrors> {
    let line_sanitized = sanitize_string(line);
    
    let possible_context = discover_base(&line_sanitized);

    let decimal_val = match possible_context {
        Some(context) => {
            match base2dec(&line_sanitized, context.base, context.charset, &context.prefix) {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        None => match line_sanitized.parse::<usize>() { // check for base-10 decimal
            Ok(i) => i,
            Err(_e) => return Err(ConversionErrors::IntConversionError)
        }
    };

    return Ok(decimal_val);
}

pub fn conv2base(input: usize, target_base: usize) -> Result<String, ConversionErrors> {
    let convert_result = match target_base {

        10 => Ok(input.to_string()),
        
        n =>  {
            for context in CONTEXT_REGISTRY.contexts {
                if context.base == n {
                    return dec2base(&input, context.base, context.charset, &context.prefix);
                }
            }
            return Err(ConversionErrors::UndefinedBaseContextError);
        }
    };

    return convert_result;
}