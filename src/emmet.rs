use colored::Colorize;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

mod helpers;
use helpers::{is_inline, is_self_closing};

const INDENT: &str = "   ";

type ParentType = Option<Weak<RefCell<Tag>>>;

#[derive(Default, Debug)]
pub struct Tag {
    name: String,
    parent: ParentType,
    is_inline: bool,
    is_self_close: bool,
    children: Option<Vec<Node>>,
    attrs: Option<IndexMap<String, String>>,
}

#[derive(Default, Debug)]
pub struct Text {
    value: String,
    parent: ParentType,
}

#[derive(Debug)]
pub enum Node {
    Tag(Rc<RefCell<Tag>>),
    Text(Rc<RefCell<Text>>),
}

impl Node {
    pub fn tag(name: &str) -> Self {
        Self::Tag(Rc::new(RefCell::new(Tag {
            name: name.to_string(),
            is_inline: is_inline(name),
            is_self_close: is_self_closing(name),

            ..Default::default()
        })))
    }

    pub fn text(self, text: &str) -> Self {
        if let Node::Tag(ref tag_ref) = self {
            let mut tag = tag_ref.borrow_mut();

            let text = Self::Text(Rc::new(RefCell::new(Text {
                value: text.to_string(),
                parent: Some(Rc::downgrade(tag_ref)),
            })));

            match tag.children {
                Some(ref mut children) => children.push(text),
                None => tag.children = Some(vec![text]),
            }
        }

        self
    }

    pub fn child(self, child: Node) -> Self {
        if let Node::Tag(ref parent_ref) = self {
            match child {
                Node::Tag(ref tag_ref) => {
                    tag_ref.borrow_mut().parent = Some(Rc::downgrade(parent_ref));
                }
                Node::Text(ref text_ref) => {
                    text_ref.borrow_mut().parent = Some(Rc::downgrade(parent_ref));
                }
            }

            let mut parent = parent_ref.borrow_mut();

            match parent.children {
                None => parent.children = Some(vec![child]),
                Some(ref mut children) => children.push(child),
            }
        }

        self
    }

    pub fn attr(self, name: &str, value: &str) -> Self {
        if let Self::Tag(ref tag_rc) = self {
            let mut tag = tag_rc.borrow_mut();

            match tag.attrs {
                None => tag.attrs = Some(IndexMap::from([(name.to_string(), value.to_string())])),
                Some(ref mut attrs) => {
                    if let Some(val) = attrs.get_mut(name) {
                        val.push_str(&format!(" {value}"));
                    } else {
                        attrs.insert(name.to_string(), value.to_string());
                    }
                }
            };
        }

        self
    }
}

impl Tag {
    fn render_attrs(&self) -> String {
        match self.attrs {
            None => "".to_string(),
            Some(ref attrs) => {
                let char_count: usize = attrs.iter().map(|(key, val)| key.len() + val.len()).sum();
                let capacity = (char_count + attrs.len() * 4) - 1;

                attrs
                    .iter()
                    .fold(String::with_capacity(capacity), |mut acc, (k, v)| {
                        acc.push_str(&format!(" {}={}", k.green(), format!("\"{v}\"").yellow()));
                        acc
                    })
            }
        }
    }

    fn render_children(&self) -> Option<String> {
        match self.children {
            None => None,
            Some(ref children) => Some(children.iter().fold(String::new(), |mut acc, child| {
                let nl = if !acc.is_empty() { "\n" } else { "" };
                acc.push_str(&format!("{nl}{child}"));
                acc
            })),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attrs = self.render_attrs();
        let tag_name = self.name.red();

        let indent = self.get_indent();

        let open = format!("{0}<{tag_name}{attrs}>", indent);
        let close = format!("</{tag_name}>");

        match self.is_self_close {
            true => write!(f, "{1}<{0}{attrs}/>", tag_name, indent),
            false => {
                if let Some(children) = self.render_children() {
                    write!(f, "{open}\n{children}\n{indent}{close}")
                } else {
                    write!(f, "{open}{close}")
                }
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Tag(ref tag) => write!(f, "{}", tag.borrow()),
            Node::Text(ref text) => write!(f, "{}", text.borrow()),
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if false {
            // let all = self.all_neighbors_inline();
            write!(f, "{}", self.value)
        } else {
            let indent = self.get_indent();
            let text = &self.value;

            write!(f, "{indent}{text}")
        }
    }
}

impl Parent for Tag {
    fn get_parent(&self) -> Option<Rc<RefCell<Tag>>> {
        if let Some(ref parent) = self.parent {
            return parent.upgrade();
        } else {
            None
        }
    }
}

impl Parent for Text {
    fn get_parent(&self) -> Option<Rc<RefCell<Tag>>> {
        if let Some(ref parent) = self.parent {
            return parent.upgrade();
        } else {
            None
        }
    }
}

impl Deep for Tag {}
impl Deep for Text {}

impl Neighbors for Tag {}
impl Neighbors for Text {}

trait Parent {
    fn get_parent(&self) -> Option<Rc<RefCell<Tag>>>;
}

trait Neighbors {
    fn all_neighbors_inline(&self) -> bool
    where
        Self: Parent,
    {
        self.get_parent().map_or(true, |parent_ref| {
            parent_ref
                .borrow()
                .children
                .as_ref()
                .map_or(true, |children| {
                    children.iter().all(|node| match node {
                        Node::Tag(tag) => tag.borrow().is_inline,
                        _ => true,
                    })
                })
        })
    }
}

trait Deep
where
    Self: Parent,
{
    fn get_deep(&self) -> usize {
        self.get_parent()
            .map_or(0, |tag_ref| tag_ref.borrow().get_deep() + 1)
    }

    fn get_indent(&self) -> String {
        INDENT.repeat(self.get_deep())
    }
}
