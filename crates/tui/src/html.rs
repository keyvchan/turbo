use std::default::Default;

use html5ever::interface::QualName;
use html5ever::tendril::{fmt, ByteTendril};
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{StartTag, TagToken};
use html5ever::tokenizer::{Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts};
use html5ever::{namespace_url, ns, LocalName};

#[derive(Clone)]
struct TokenPrinter {
    links: Vec<String>,
}

impl TokenSink for TokenPrinter {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        let link_name = QualName::new(None, ns!(), LocalName::from("href"));
        if let TagToken(tag) = token {
            if tag.kind == StartTag && tag.name.to_string() == "a" {
                let attrs = tag.attrs;
                for attr in attrs {
                    if attr.name == link_name {
                        self.links.push(attr.value.to_string());
                    }
                }
            }
        }
        TokenSinkResult::Continue
    }
}

pub fn parse_html(content: &str) -> Vec<String> {
    let sink = TokenPrinter { links: vec![] };
    let mut chunk = ByteTendril::new();
    chunk.push_slice(content.as_bytes());

    let mut input = BufferQueue::new();
    input.push_back(chunk.try_reinterpret::<fmt::UTF8>().unwrap());

    let mut tok = Tokenizer::new(sink, TokenizerOpts::default());
    let _ = tok.feed(&mut input);
    assert!(input.is_empty());
    tok.end();

    tok.sink.links
}
