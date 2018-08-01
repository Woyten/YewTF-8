#![recursion_limit = "128"]

#[macro_use]
extern crate yew;

use std::fmt::Write;
use yew::prelude::*;

struct Context {}

struct Model {
    utf8_input: String,
    hex_from_utf8: String,
    hex_input: String,
    ascii_from_hex: String,
}

enum Msg {
    Utf8Typed(String),
    HexTyped(String),
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            utf8_input: String::new(),
            hex_from_utf8: String::new(),
            hex_input: String::new(),
            ascii_from_hex: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Utf8Typed(utf8) => {
                let mut hex = String::new();
                for byte in utf8.bytes() {
                    write!(hex, "{:02x} ", byte);
                }
                hex.pop();
                self.utf8_input = utf8;
                self.hex_from_utf8 = hex;
                true
            }
            Msg::HexTyped(hex) => {
                let mut ascii = String::new();
                let mut hex_buffer = String::new();
                for chunk in hex.split(char::is_whitespace) {
                    for char in chunk.chars() {
                        hex_buffer.push(char);
                        if hex_buffer.len() > 1 {
                            consume_hex_buffer(&mut ascii, &mut hex_buffer)
                        }
                    }
                    if !hex_buffer.is_empty() {
                        consume_hex_buffer(&mut ascii, &mut hex_buffer)
                    }
                }
                self.hex_input = hex;
                self.ascii_from_hex = ascii;
                true
            }
        }
    }
}

fn consume_hex_buffer(ascii: &mut String, hex_buffer: &mut String) {
    match u8::from_str_radix(&hex_buffer, 16) {
        Ok(byte) => ascii.push(byte as char),
        Err(_) => ascii.push_str("<NO_HEX>"),
    };
    hex_buffer.clear();
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div style="display: flex; flex-wrap: wrap",>
                <div>
                    <p>{ "UTF-8" }</p>
                    <textarea style="resize:none", rows=20, cols=80, oninput=|e| Msg::Utf8Typed(e.value), value=self.utf8_input.clone(),/>
                </div>
                <div>
                    <p>{ "Bytes from UTF-8" }</p>
                    <textarea readonly="", style="resize:none", rows=20, cols=80, value=self.hex_from_utf8.clone(),/>
                </div>
                <div>
                    <p>{ "Bytes (hex)" }</p>
                    <textarea style="resize:none", rows=20, cols=80, oninput=|e| Msg::HexTyped(e.value), value=self.hex_input.clone(),/>
                </div>
                <div>
                    <p>{ "ASCII from bytes" }</p>
                    <textarea readonly="", style="resize:none", rows=20, cols=80, value=self.ascii_from_hex.clone(),/>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(Context {});
    app.mount_to_body();
    yew::run_loop();
}
