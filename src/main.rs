fn main() {
    // Read the Markdown input from a file
    let markdown_input = std::fs::read_to_string("markdown.md").unwrap();
    let parser = pulldown_cmark::Parser::new(&markdown_input);
    // println!("Parser:\n{:?}", parser);

    // Write the HTML output to a string
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Write the HTML output to a file
    std::fs::write("index.html", &html_output).unwrap();
}
