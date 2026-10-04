use crate::infer::{self, Ty};
use crate::value;

pub trait ReplValue: Clone {
    fn is_boxed(&self) -> bool;
    fn immediate(&self) -> i64;
    fn field(&self, index: usize) -> Self;
    fn length(&self) -> usize;
    fn bytes(&self) -> Vec<u8>;
    fn real(&self) -> f64;
    fn builtin(&self, name: &str) -> bool;
    fn is_immediate(&self, word: i64) -> bool {
        !self.is_boxed() && self.immediate() == word
    }
    fn with_root<T>(&self, action: impl FnOnce() -> T) -> T {
        action()
    }
}

pub struct Printer<'a> {
    pub types: &'a infer::Session,
}

impl Printer<'_> {
    pub fn show<V: ReplValue>(&self, word: V, ty: &Ty, depth: usize) -> String {
        if depth == 0 {
            return "#".to_string();
        }
        match ty {
            Ty::Con {
                name,
                stamp: 0,
                args,
            } => match (name.as_str(), args.as_slice()) {
                ("int", _) => {
                    let value = word.immediate() >> 1;
                    if value < 0 {
                        format!("~{}", -value)
                    } else {
                        value.to_string()
                    }
                }
                ("bool", _) => (word.is_immediate(value::TRUE)).to_string(),
                ("word", _) => format!("0wx{:X}", word.immediate() >> 1),
                ("order", _) => {
                    ["LESS", "EQUAL", "GREATER"][(word.immediate() >> 1) as usize].to_string()
                }
                ("option", [element]) if word.is_immediate(value::NIL) => "NONE".to_string(),
                ("option", [element]) => {
                    format!(
                        "SOME {}",
                        argument(self.show(word.field(0), element, depth - 1))
                    )
                }
                ("ref", [element]) => {
                    format!(
                        "ref {}",
                        argument(self.show(word.field(0), element, depth - 1))
                    )
                }
                ("exn", _) => {
                    let identity = word.field(0);
                    let name = self.show(
                        identity.field(0),
                        &Ty::Con {
                            name: "string".into(),
                            stamp: 0,
                            args: vec![],
                        },
                        depth,
                    );
                    let name = name.trim_matches('"');
                    let described = if identity.length() > 1 {
                        let descriptor = identity.field(1);
                        (!descriptor.is_immediate(value::NIL)).then(|| descriptor_type(descriptor))
                    } else {
                        (identity.builtin("Fail")).then(|| Ty::Con {
                            name: "string".into(),
                            stamp: 0,
                            args: vec![],
                        })
                    };
                    match described {
                        Some(argument_ty) => format!(
                            "{name} {}",
                            argument(self.show(word.field(1), &argument_ty, depth - 1))
                        ),
                        None => name.to_string(),
                    }
                }
                ("unit", _) => "()".to_string(),
                ("char", _) => {
                    format!("#\"{}\"", escape(&[(word.immediate() >> 1) as u8]))
                }
                ("string", _) => {
                    let text = word.bytes();
                    format!("\"{}\"", escape(&text))
                }
                ("real", _) => {
                    let real = word.real();
                    let text = format!("{real:?}");
                    text.replace('-', "~")
                }
                ("list", [element]) => {
                    let mut items = Vec::new();
                    let mut cell = word;
                    while !cell.is_immediate(value::NIL) && items.len() < 20 {
                        items.push(self.show(cell.field(0), element, depth - 1));
                        cell = cell.field(1);
                    }
                    if !cell.is_immediate(value::NIL) {
                        items.push("...".to_string());
                    }
                    format!("[{}]", items.join(","))
                }
                _ => "-".to_string(),
            },
            Ty::Con { .. } => {
                let Some(constructors) = self.types.constructors(ty) else {
                    return "-".to_string();
                };
                let carries = word.is_boxed();
                let carriers = constructors
                    .iter()
                    .filter(|(_, argument)| argument.is_some())
                    .count();
                let index = if carries {
                    if carriers > 1 {
                        word.field(0).immediate() >> 1
                    } else {
                        0
                    }
                } else {
                    word.immediate() >> 1
                };
                let Some((name, payload)) = constructors
                    .iter()
                    .filter(|(_, argument)| argument.is_some() == carries)
                    .nth(index as usize)
                else {
                    return "-".to_string();
                };
                match payload {
                    Some(payload) => format!(
                        "{name} {}",
                        argument(self.show(
                            word.field(usize::from(carriers > 1)),
                            payload,
                            depth - 1
                        ))
                    ),
                    None => name.clone(),
                }
            }
            Ty::Arrow(..) => "fn".to_string(),
            Ty::Record(fields) if fields.is_empty() => "()".to_string(),
            Ty::Record(fields) => {
                let items: Vec<String> = fields
                    .iter()
                    .enumerate()
                    .map(|(index, (_, ty))| self.show(word.field(index), ty, depth - 1))
                    .collect();
                let tuple = fields
                    .iter()
                    .enumerate()
                    .all(|(index, (label, _))| *label == (index + 1).to_string());
                if tuple {
                    format!("({})", items.join(","))
                } else {
                    let items: Vec<String> = fields
                        .iter()
                        .zip(items)
                        .map(|((label, _), item)| format!("{label}={item}"))
                        .collect();
                    format!("{{{}}}", items.join(","))
                }
            }
            _ => "-".to_string(),
        }
    }
}

pub fn uncaught<V: ReplValue>(exception: V) -> String {
    let text = |value: V| String::from_utf8_lossy(&value.bytes()).into_owned();
    let identity = exception.field(0);
    let name = text(identity.field(0));
    let builtin = |known: &str| identity.builtin(known);
    let detail = if builtin("Fail") {
        format!(" [Fail: {}]", text(exception.field(1)))
    } else {
        [
            ("Div", "divide by zero"),
            ("Overflow", "overflow"),
            ("Match", "nonexhaustive match failure"),
            ("Bind", "nonexhaustive binding failure"),
            ("Subscript", "subscript out of bounds"),
        ]
        .iter()
        .find(|(known, _)| builtin(known))
        .map_or_else(String::new, |(_, message)| format!(" [{message}]"))
    };
    format!(
        "uncaught exception {name}{detail}\n  raised at: {}",
        text(exception.field(2))
    )
}

fn descriptor_type<V: ReplValue>(descriptor: V) -> Ty {
    let text = |value: V| String::from_utf8_lossy(&value.bytes()).into_owned();
    let children = |value: V| {
        (0..value.length())
            .map(|index| descriptor_type(value.field(index)))
            .collect()
    };
    match descriptor.field(0).immediate() >> 1 {
        0 => Ty::Con {
            name: text(descriptor.field(1)),
            stamp: (descriptor.field(2).immediate() >> 1) as usize,
            args: children(descriptor.field(3)),
        },
        1 => {
            let fields = descriptor.field(1);
            Ty::Record(
                (0..fields.length())
                    .map(|index| {
                        let field = fields.field(index);
                        (text(field.field(0)), descriptor_type(field.field(1)))
                    })
                    .collect(),
            )
        }
        2 => Ty::Arrow(Box::new(Ty::Record(vec![])), Box::new(Ty::Record(vec![]))),
        _ => Ty::Var {
            id: 0,
            equality: false,
        },
    }
}

fn argument(shown: String) -> String {
    if shown.contains(' ') && !shown.starts_with(['(', '[', '{', '"', '#']) {
        format!("({shown})")
    } else {
        shown
    }
}

fn escape(text: &[u8]) -> String {
    let mut escaped = String::new();
    for &byte in text {
        match byte {
            b'"' => escaped.push_str("\\\""),
            b'\\' => escaped.push_str("\\\\"),
            b'\n' => escaped.push_str("\\n"),
            b'\t' => escaped.push_str("\\t"),
            b'\r' => escaped.push_str("\\r"),
            8 => escaped.push_str("\\b"),
            12 => escaped.push_str("\\f"),
            0..32 => escaped.push_str(&format!("\\^{}", (byte + 64) as char)),
            127..=255 => escaped.push_str(&format!("\\{byte:03}")),
            byte => escaped.push(byte as char),
        }
    }
    escaped
}
