use select::document::Document;
use select::predicate::Name;

pub fn extract_words(html: &str) -> Vec<String> {
    let document = Document::from(html);
    let texts = document.find(Name("body"))
        .next()  
        .map_or(vec![], |body| { 
            body.descendants()
                .filter(|node| {
                    node.as_text().is_some() && 
                        node.parent().map_or(true, |parent| {  
                            !parent.is(Name("script")) && !parent.is(Name("style"))
                        })
                })
                .flat_map(|node| node.text().split_whitespace().map(String::from).collect::<Vec<_>>())
                .collect()
        });

    texts.into_iter()
        .map(|word| word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|word| !word.is_empty())
        .collect()
}
