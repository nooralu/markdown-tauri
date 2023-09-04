use pulldown_cmark::html;
use pulldown_cmark::CodeBlockKind;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use tokio::sync::{mpsc, Mutex};

pub struct FilePathTx {
    pub inner: Mutex<mpsc::Sender<String>>,
}

pub async fn md_to_html(text: &str) -> Result<String, String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(text, options);
    let events = highlight(parser, "base16-ocean.light")?;
    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    Ok(html_output)
}

pub fn highlight<'a, T>(events: T, theme_name: &str) -> Result<Vec<Event<'a>>, String>
where
    T: Iterator<Item = Event<'a>>,
{
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let plain_text_syntax = syntax_set.find_syntax_plain_text();
    let mut syntax = plain_text_syntax;

    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set
        .themes
        .get(&theme_name.to_string())
        .ok_or("Theme not found")?;

    let mut in_code_block = false;
    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        syntax = syntax_set
                            .find_syntax_by_token(&lang)
                            .unwrap_or(plain_text_syntax)
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                if !in_code_block {
                    unreachable!();
                }
                let html = highlighted_html_for_string(&to_highlight, &syntax_set, syntax, theme)
                    .map_err(|_| "Failed to highlight code")?;

                to_highlight.clear();
                in_code_block = false;
                out_events.push(Event::Html(CowStr::from(html)));
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }

    Ok(out_events)
}
