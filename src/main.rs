use std::{collections::HashMap, fmt};

fn html_attrs_tostr(map: &HashMap<String, String>) -> String {
    let mut result = String::new();

    for (key, value) in map {
        result.push_str(&format!("{}=\"{}\" ", key, value));
    }

    // Remove the trailing space if there are entries in the map
    if !map.is_empty() {
        result.pop(); // Remove the last space
    }

    result
}

fn attrs(attributes: Vec<(&str, &str)>) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for (key, value) in attributes {
        result.insert(key.to_string(), value.to_string());
    }

    result
}

enum Displayable<'a> {
    Element {
        tag: &'a str,
        attrs: HashMap<String, String>,
        content: Option<Box<Displayable<'a>>>,
    },

    List(Vec<Box<Displayable<'a>>>),

    String(&'a str),
}

macro_rules! str {
    ($text:expr) => {
        Some(Box::new(Displayable::String($text)))
    };
}

macro_rules! elem {
    ($tag:expr, $attrs:expr, $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs($attrs),
            content: $content,
        })
    };

    ($tag:expr, $attrs:expr, raw $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs($attrs),
            content: str!($content),
        })
    };

    ($tag:expr, $attrs:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs($attrs),
            content: None,
        })
    };

    ($tag:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs(vec![]),
            content: None,
        })
    };
}

macro_rules! list {
    ($($element:expr),*) => {
        Some(Box::new(Displayable::List(vec![$($element),*])))
    };
}

impl fmt::Display for Displayable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Displayable::Element {
                tag,
                attrs,
                content,
            } => {
                let mut err: Result<_, std::fmt::Error> = Ok(());
                if attrs.len() > 0 {
                    match write!(
                        f,
                        "<{} {}",
                        tag,
                        html_attrs_tostr(attrs)
                    ) {
                        Ok(_) => {},
                        Err(e) => { err = Err(e); },
                    };
                } else {
                    match write!(
                        f,
                        "<{}",
                        tag,
                    ) {
                        Ok(_) => {},
                        Err(e) => { err = Err(e); },
                    };
                }
                
                match content {
                    None => {
                        match write!(f, "{}", ">") {
                            Ok(_) => {},
                            Err(e) => { err = Err(e); },
                        };
                    }

                    Some(c) => {
                        let cs = format!("{}", c).replace('\n', "\n\t");
                        match write!(f, ">\n\t{}\n</{}>", cs, tag) {
                            Ok(_) => {},
                            Err(e) => { err = Err(e); },
                        };
                    }
                }

                err
            }

            Displayable::List(li) => {
                let mut acc = String::new();
                for item in li {
                    acc.push_str(format!("{}\n", item).as_str())
                }
                acc.pop();
                write!(f, "{}", acc)
            }

            Displayable::String(str) => {
                write!(f, "{}", str)
            }
        }
    }
}

pub fn main() {
    print!("<!DOCTYPE html>\n{}",

        elem!("html", vec![],
            list![
                elem!("head", vec![], 
                    list![
                        elem!("meta", vec![
                            ("charset","utf-8")
                        ]),

                        elem!("meta", vec![
                            ("name", "viewport"),
                            ("content", "width=device-width"),
                            ("initial-scale", "1.0")
                        ]),

                        elem!("title", vec![], raw "Untitled Calamity Site")
                    ]
                ),
                elem!("body", vec![], list![
                    elem!("h1", vec![("class", "display-4")], raw "Hello, world!"),
                    elem!("p", vec![("class", "content")], raw "This site was generated with calamity")
                ])
            ]
        )

    );
}
