const SELF_CLOSING_TAGS: [&str; 13] = [
    "meta", "link", "img", "br", "hr", "input", "area", "base", "col", "embed", "source", "track",
    "wbr",
];

const INLINE_ELEMENTS: [&str; 33] = [
    "a", "abbr", "acronym", "b", "bdo", "big", "br", "button", "cite", "code", "dfn", "em", "i",
    "img", "input", "kbd", "label", "map", "object", "output", "q", "samp", "script", "select",
    "small", "span", "strong", "sub", "sup", "textarea", "time", "tt", "var",
];

pub fn is_inline(name: &str) -> bool {
    INLINE_ELEMENTS.contains(&name)
}

pub fn is_self_closing(name: &str) -> bool {
    SELF_CLOSING_TAGS.contains(&name)
}
