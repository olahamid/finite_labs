//! The homepage "Live proof" widget.
//!
//! On load: generate an Ed25519 key pair from the browser's CSPRNG.
//! On every keystroke: SHA-256 the text, sign it, and verify the signature
//! against either the same text or a tampered copy. Real cryptography, nothing
//! simulated. The HTML lives in `templates/index.html`; this file only fills it in.

use std::rc::Rc;

use crypto::{
    hash,
    sign::{self, Keypair, PUBLIC_KEY_LEN, SEED_LEN},
    to_hex,
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, Element, HtmlInputElement, Window};

use crate::tamper::tamper;

struct LiveProof {
    document: Document,
    keypair: Keypair,
    public_key: [u8; PUBLIC_KEY_LEN],
    input: HtmlInputElement,
    tamper_toggle: HtmlInputElement,
    hash_out: Element,
    signature_out: Element,
    received_out: Element,
    badge: Element,
}

/// Find a child element marked `data-lp="<name>"`.
fn part(root: &Element, name: &str) -> Result<Element, JsValue> {
    root.query_selector(&format!("[data-lp=\"{name}\"]"))?
        .ok_or_else(|| JsValue::from_str(&format!("live proof: missing data-lp=\"{name}\"")))
}

fn input_part(root: &Element, name: &str) -> Result<HtmlInputElement, JsValue> {
    part(root, name)?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str(&format!("live proof: {name} is not an <input>")))
}

/// `abcd1234…wxyz9876`: long hex shortened for display; full value in the title.
fn shorten(hex: &str, head: usize, tail: usize) -> String {
    if hex.len() <= head + tail + 1 {
        return hex.to_string();
    }
    format!("{}…{}", &hex[..head], &hex[hex.len() - tail..])
}

pub fn mount(window: &Window, document: &Document, root: Element) -> Result<(), JsValue> {
    // 32 bytes from the browser's cryptographically secure RNG become the secret key.
    let mut seed = [0u8; SEED_LEN];
    window
        .crypto()?
        .get_random_values_with_u8_array(&mut seed)?;
    let keypair = Keypair::from_seed(&seed);
    seed.fill(0);
    let public_key = keypair.public_key();

    let proof = Rc::new(LiveProof {
        document: document.clone(),
        keypair,
        public_key,
        input: input_part(&root, "input")?,
        tamper_toggle: input_part(&root, "tamper")?,
        hash_out: part(&root, "hash")?,
        signature_out: part(&root, "signature")?,
        received_out: part(&root, "received")?,
        badge: part(&root, "badge")?,
    });

    let public_hex = to_hex(&public_key);
    let pubkey_out = part(&root, "pubkey")?;
    pubkey_out.set_text_content(Some(&shorten(&public_hex, 16, 8)));
    pubkey_out.set_attribute("title", &public_hex)?;

    // The same handler runs for typing and for flipping the tamper switch.
    for (target, event) in [
        (proof.input.clone(), "input"),
        (proof.tamper_toggle.clone(), "change"),
    ] {
        let proof = Rc::clone(&proof);
        let handler = Closure::<dyn FnMut()>::new(move || {
            if let Err(err) = proof.update() {
                web_sys::console::error_1(&err);
            }
        });
        target.add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())?;
        // The listener lives as long as the page, so hand ownership to JS.
        handler.forget();
    }

    proof.input.set_disabled(false);
    proof.tamper_toggle.set_disabled(false);
    proof.update()?;
    root.set_attribute("data-ready", "true")?;
    Ok(())
}

impl LiveProof {
    fn update(&self) -> Result<(), JsValue> {
        let message = self.input.value();

        let digest = hash::sha256_hex(message.as_bytes());
        self.hash_out.set_text_content(Some(&digest));

        let signature = self.keypair.sign(message.as_bytes());
        let signature_hex = to_hex(&signature);
        self.signature_out
            .set_text_content(Some(&shorten(&signature_hex, 24, 8)));
        self.signature_out.set_attribute("title", &signature_hex)?;

        // What the verifier actually receives: the original, or a forged copy.
        let (received, changed_at) = if self.tamper_toggle.checked() {
            let t = tamper(&message);
            (t.text, Some(t.changed_at))
        } else {
            (message, None)
        };
        self.show_received(&received, changed_at)?;

        let verified = sign::verify(&self.public_key, received.as_bytes(), &signature);
        self.badge.set_text_content(Some(if verified {
            "Signature verified"
        } else {
            "Verification failed"
        }));
        self.badge
            .set_attribute("data-state", if verified { "ok" } else { "fail" })?;
        Ok(())
    }

    /// Render the received message, wrapping the changed character in <mark>.
    /// Built with text nodes, never innerHTML, so typed text can't inject HTML.
    fn show_received(&self, text: &str, changed_at: Option<usize>) -> Result<(), JsValue> {
        self.received_out.set_text_content(None);
        let Some(index) = changed_at else {
            self.received_out.set_text_content(Some(text));
            return Ok(());
        };

        let before: String = text.chars().take(index).collect();
        let changed: String = text.chars().skip(index).take(1).collect();
        let after: String = text.chars().skip(index + 1).collect();

        let mark = self.document.create_element("mark")?;
        mark.set_text_content(Some(&changed));
        self.received_out
            .append_child(&self.document.create_text_node(&before))?;
        self.received_out.append_child(&mark)?;
        self.received_out
            .append_child(&self.document.create_text_node(&after))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::shorten;

    #[test]
    fn shortens_long_hex_only() {
        assert_eq!(shorten("abcdef", 2, 2), "ab…ef");
        assert_eq!(shorten("abcde", 2, 2), "abcde");
    }
}
