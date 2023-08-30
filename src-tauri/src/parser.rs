use pulldown_cmark::{html, Options, Parser};
use tokio::fs;
use tokio::sync::{mpsc, Mutex};

pub struct FilePathTx {
    pub inner: Mutex<mpsc::Sender<String>>,
}

pub async fn md_to_html(path: &str) -> Result<String, String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let text = fs::read_to_string(path).await.map_err(|e| e.to_string())?;
    let parser = Parser::new_ext(&text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}
