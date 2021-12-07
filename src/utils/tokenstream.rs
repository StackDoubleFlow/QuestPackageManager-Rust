use std::collections::HashMap;

// OVERENGINEERING REPLACING STRINGS!
// Iterate through an entire string, check if a string matches anything in the hashmap
// if anything matches, replace and continue

pub fn replace_fast(original_s: &str, replacements: &HashMap<&'static str, &str>) -> String {
    // The resulting string
    let mut string_stream: Vec<u8> = Vec::with_capacity(original_s.len());

    // Keeps track of the char to check of each replacement
    let mut compared_strings: Vec<usize> = vec![0; replacements.len()];

    // The chars of the string
    let chars = original_s.char_indices();

    for (_index, original_char) in chars {
        let offset: usize = string_stream.len(); // for performance

        let mut replaced = false;

        // The key of the replacement string we're checking

        'check_replacements: for (key_i, (key_string, replacement)) in
            replacements.iter().enumerate()
        {
            let compared_string_index = compared_strings[key_i]; // index of char to check

            let compare_char: char = key_string.as_bytes()[compared_string_index].into();

            // Chars do not match, restart
            if compare_char != original_char {
                compared_strings[key_i] = 0;
                continue 'check_replacements;
            }

            // Not end of the compared string, just continue next round
            if compared_string_index != key_string.len() - 1 {
                compared_strings[key_i] = compared_string_index + 1;
                continue 'check_replacements;
            }

            // End of the string to replace, time to do replacing magic
            let replacement_bytes = replacement.as_bytes();

            let replace_start = offset - key_string.len() + 1; // where replace starts
            let replace_end_real = offset; // where the current built string ends

            let replace_end_iter = (replace_start + replacement_bytes.len()).min(replace_end_real); // end clamped to end of string if needed
            let replace_end_offset = (replace_start + replacement_bytes.len()) - replace_end_iter; // how many chars left to add if needed

            // Iterate through existing chars and replace them
            for (j, item) in string_stream
                .iter_mut()
                .enumerate()
                .take(replace_end_iter)
                .skip(replace_start)
            {
                let char_index = j - replace_start;
                let char_to_push: char = replacement_bytes[char_index].into();
                *item = char_to_push as u8;
            }

            // Add remaining chars
            if replace_end_offset > 0 {
                for char_to_push in replacement_bytes.iter().take(replace_end_offset) {
                    string_stream.push(*char_to_push);
                }

            // Trim ends, as is to say, remove the rest of the chars
            } else if replace_end_iter < replace_end_real {
                for _ in replace_end_iter..replace_end_real {
                    string_stream.pop();
                }
            }

            replaced = true;

            // Reset all comparisons
            for j in &mut compared_strings {
                *j = 0;
            }

            break 'check_replacements;
        }

        if !replaced {
            string_stream.push(original_char as u8);
        }
    }

    unsafe { String::from_utf8_unchecked(string_stream) }
}
