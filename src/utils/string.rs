use alloc::string::{String, ToString};

pub fn ellipse(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        return text[0..max_len - 3].to_string() + "...";
    }

    text.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    mod ellipse {
        use super::*;

        #[test]
        fn should_return_text_if_less_than_max_length() {
            let result = ellipse("test", 5);
            assert_eq!(result, "test");
        }

        #[test]
        fn should_return_text_if_equal_to_max_length() {
            let result = ellipse("test", 4);
            assert_eq!(result, "test");
        }

        #[test]
        fn should_return_truncated_text_if_greater_than_max_length() {
            let result = ellipse("test!", 4);
            assert_eq!(result, "t...");
        }
    }
}
