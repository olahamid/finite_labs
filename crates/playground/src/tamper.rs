//! How the "Tamper" toggle changes a message. Pure Rust, so it is unit-tested
//! natively with `cargo test`.

/// The message an attacker forwards: identical except for exactly one character.
pub struct Tampered {
    pub text: String,
    /// Index (in chars) of the changed character, for highlighting.
    pub changed_at: usize,
}

/// Change exactly one character of `text`.
///
/// The first digit is changed if there is one, so "Send ₦5,000 to Ada" becomes
/// "Send ₦9,000 to Ada": a forged payment amount is the clearest story.
/// Otherwise the last letter or digit changes, and an empty message becomes "!".
pub fn tamper(text: &str) -> Tampered {
    let mut chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return Tampered {
            text: "!".to_string(),
            changed_at: 0,
        };
    }

    let index = chars
        .iter()
        .position(char::is_ascii_digit)
        .or_else(|| chars.iter().rposition(|c| c.is_alphanumeric()))
        .unwrap_or(chars.len() - 1);

    chars[index] = replacement(chars[index]);
    Tampered {
        text: chars.into_iter().collect(),
        changed_at: index,
    }
}

fn replacement(c: char) -> char {
    match c {
        '0'..='9' => {
            let digit = c as u8 - b'0';
            char::from(b'0' + (digit + 4) % 10)
        }
        'z' => 'a',
        'Z' => 'A',
        'a'..='y' | 'A'..='Y' => char::from(c as u8 + 1),
        '*' => '#',
        _ => '*',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn changed_chars(a: &str, b: &str) -> usize {
        a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
    }

    #[test]
    fn changes_the_payment_amount() {
        let t = tamper("Send ₦5,000 to Ada");
        assert_eq!(t.text, "Send ₦9,000 to Ada");
        assert_eq!(t.changed_at, 6);
    }

    #[test]
    fn changes_last_letter_when_no_digits() {
        let t = tamper("hello");
        assert_eq!(t.text, "hellp");
        assert_eq!(t.changed_at, 4);
    }

    #[test]
    fn wraps_letters_and_digits() {
        assert_eq!(tamper("zz").text, "za");
        assert_eq!(tamper("7").text, "1");
    }

    #[test]
    fn handles_symbols_and_empty_input() {
        assert_eq!(tamper("!!").text, "!*");
        assert_eq!(tamper("").text, "!");
    }

    #[test]
    fn always_changes_exactly_one_character() {
        for input in [
            "Send ₦5,000 to Ada",
            "hello",
            "zz",
            "***",
            "é",
            "a b c",
            "₦",
        ] {
            let t = tamper(input);
            assert_eq!(t.text.chars().count(), input.chars().count(), "{input}");
            assert_eq!(changed_chars(input, &t.text), 1, "{input}");
        }
    }
}
