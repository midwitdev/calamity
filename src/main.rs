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
        Box::new(Displayable::String($text))
    };
}

macro_rules! tag {
    ($tag:expr, $class:expr, $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs(vec![("class", $class)]),
            content: Some($content),
        })
    };

    ($tag:expr, $class:expr, raw $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs(vec![("class", $class)]),
            content: Some(str!($content)),
        })
    };

    ($tag:expr, raw $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs(vec![]),
            content: Some(str!($content)),
        })
    };
}

macro_rules! elem {
    ($tag:expr, $attrs:expr, $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs($attrs),
            content: Some($content),
        })
    };

    ($tag:expr, $attrs:expr, raw $content:expr) => {
        Box::new(Displayable::Element {
            tag: $tag,
            attrs: attrs($attrs),
            content: Some(str!($content)),
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
        Box::new(Displayable::List(vec![$($element),*]))
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
                        match if cs.contains('\n') {
                            write!(f, ">\n\t{}\n</{}>", cs, tag)
                        } else {
                            write!(f, ">{}</{}>", cs, tag)
                        }
                        {
                            Ok(_) => {},
                            Err(e) => { err = Err(e); },
                        }
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
    let elem_bootstrap_link = elem!(
        "link",
        vec![
            ("rel",  "stylesheet"),
            ("href", "https://cdn.jsdelivr.net/npm/bootstrap@4.4.1/dist/css/bootstrap.min.css"),
            ("integrity", "sha384-Vkoo8x4CGsO3+Hhxv8T/Q5PaXtkKtu6ug5TOeNV6gBiFeWPGFN9MuhOf23Q9Ifjh"),
            ("crossorigin", "anonymous")
        ]
    );

    let elem_jquery_script = elem!(
        "script",
        vec![
            ("src","https://code.jquery.com/jquery-3.4.1.slim.min.js"),
            ("integrity", "sha384-J6qa4849blE2+poT4WnyKhv5vZF5SrPo0iEjwBvKU7imGFAV0wwj1yYfoRSJoZ+n"),
            ("crossorigin", "anonymous")
        ],
        raw ""
    );

    let elem_popper_script = elem!(
        "script",
        vec![
            ("src","https://cdn.jsdelivr.net/npm/popper.js@1.16.0/dist/umd/popper.min.js"),
            ("integrity", "sha384-Q6E9RHvbIyZFJoft+2mJbHaEWldlvI9IOYy5n3zV9zzTtmI3UksdQRVvoxMfooAo"),
            ("crossorigin", "anonymous")
        ],
        raw ""
    );

    let elem_bootstrap_script = elem!(
        "script",
        vec![
            ("src","https://cdn.jsdelivr.net/npm/bootstrap@4.4.1/dist/js/bootstrap.min.js"),
            ("integrity", "sha384-wfSDF2E50Y2D1uUdj0O3uMBJnjuUD4Ih7YwaYd1iqfktj0Uod8GCExl3Og8ifwB6"),
            ("crossorigin", "anonymous")
        ],
        raw ""
    );

    print!("<!DOCTYPE html>\n{}",

        elem!("html", vec![],
            list![
                elem!("head", vec![], 
                    list![
                        elem_bootstrap_link,
                        elem_jquery_script,
                        elem_popper_script,
                        elem_bootstrap_script,

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
                tag!("body", "container", list![
                    tag!("div", "row", raw "Titlebar and additional information"),
                    tag!("div", "row", list![
                        tag!("div", "col", raw "Column 1"),
                        tag!("div", "col", raw "Column 2"),
                        tag!("div", "col", raw "Column 3")
                    ]),
                    tag!("div", "row", raw "Footer and additional information")
                ])
            ]
        )
    );
}
