pub use regex::Regex;
use regex::Error;

#[cfg(test)] pub mod test;

bitflags! {
    flags Modifiers: u8 {
        const MULTI_LINE        = 0b00000001,
        const CASE_INSENSITIVE    = 0b00000010,
    }
}

/// The struct used for keeping track of the modifiers
impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers::empty()
    }
}

/// The struct used for building verbal expression objects
#[derive(Debug, Clone)]
pub struct VerEx {
    string: String,
    modifiers: Modifiers,
    source: String
}

impl VerEx {
    // constructors
    /// Standard Constructor
    pub fn new() -> VerEx {
        VerEx::from_string(String::new())
    }

    /// Create a `VerEx` object from a `String`
    pub fn from_string(string: String) -> VerEx {
        let mut verex = VerEx {
            string: string,
            modifiers: Modifiers::new(),
            source: String::new()
        };
        verex.update_source_with_modifiers();
        verex
    }

    /// Create a `VerEx` object from a `&str`
    pub fn from_str(string: &str) -> VerEx {
        VerEx::from_string(string.to_string())
    }

    // --------------------------------------------------
    // fundamental methods
    /// Add a string to the regex string in the `VerEx` and return self
    pub fn add(&mut self, value: &str) -> &mut VerEx {
        self.string.push_str(value);
        self
    }

    /// Update the source string from the (presumably changed) builder string
    pub fn update_source_with_modifiers(&mut self) -> &mut VerEx {
        self.source.clear();
        self.source.push_str(r"(?");
        if self.modifiers.contains(CASE_INSENSITIVE) {
            self.source.push('i');
        }
        if self.modifiers.contains(MULTI_LINE) {
            self.source.push('m');
        }
        self.source.push(':');
        self.source.push_str(self.string.as_ref());
        self.source.push(')');
        self
    }

    /// Compile the `VerEx` to a `Regex` and return the result
    pub fn compile(& self) -> Result<Regex, Error> {
        Regex::new(self.source.as_ref())
    }

    /// Return the raw regex string contained in the `VerEx`
    pub fn raw(& self) -> &str {
        self.source()
    }

    /// Compile the `VerEx` to a `Regex` and return the result
    pub fn regex(& self) -> Result<Regex, Error> {
        self.compile()
    }

    /// Return the raw regex string contained in the `VerEx`
    pub fn source(& self) -> &str {
        self.source.as_ref()
    }

    /// Return the raw regex string contained in the `VerEx`
    pub fn value(& self) -> &str {
        self.source()
    }

    /// Open a character class
    pub fn open_class(&mut self) -> &mut VerEx {
        self.add(r"[")
    }

    /// Close a character class
    pub fn close_class(&mut self) -> &mut VerEx {
        self.add(r"]")
    }

    /// Open a non-capturing group
    pub fn open_group(&mut self) -> &mut VerEx {
        self.add(r"(?:")
    }

    /// Open a capturing group
    pub fn open_capturing_group(&mut self) -> &mut VerEx {
        self.add(r"(")
    }

    /// Close a capturing or non-capturing group
    pub fn close_group(&mut self) -> &mut VerEx {
        self.add(r")")
    }

    // --------------------------------------------------

    /// Any of the given characters
    pub fn any(&mut self, chars: &str) -> &mut VerEx {
        self.open_class()
            .add(chars)
            .close_class();
        self.update_source_with_modifiers()
    }

    /// See `any()`
    pub fn any_of(&mut self, chars: &str) -> &mut VerEx {
        self.any(chars)
    }

    /// Any character zero or more times
    pub fn anything(&mut self) -> &mut VerEx {
        self.add(r"(.*)");
        self.update_source_with_modifiers()
    }

    /// Any character zero or more times except the provided characters
    pub fn anything_but(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(value)
            .close_class()
            .add(r"*")
            .close_group();
        self.update_source_with_modifiers()
    }

    /// A line break!
    pub fn br(&mut self) -> &mut VerEx {
        self.line_break()
    }

    /// Find a specific string and capture it
    pub fn capture(&mut self, value: &str) -> &mut VerEx {
        self.open_capturing_group()
            .add(value)
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Add the token for matching digits
    pub fn digit(&mut self) -> &mut VerEx {
        self.add(r"\d");
        self.update_source_with_modifiers()
    }

    /// Add a token for matching the end of a line
    pub fn end_of_line(&mut self) -> &mut VerEx {
        self.add(r"$");
        self.update_source_with_modifiers()
    }

    /// Find a specific string
    pub fn find(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .add(value)
            .close_group();
        self.update_source_with_modifiers()
    }

    /// A line break!
    pub fn line_break(&mut self) -> &mut VerEx {
        self.open_group()
            .add(r"\n")
            .or_find(r"\r\n")
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Any string either one or zero times
    pub fn maybe(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .add(value)
            .close_group()
            .add(r"?");
        self.update_source_with_modifiers()
    }

    /// Either match the sub-expression before or after this
    pub fn or(&mut self) -> &mut VerEx {
        self.add(r"|");
        self.update_source_with_modifiers()
    }

    /// Either match the sub-expression before or the provided value
    pub fn or_find(&mut self, value: &str) -> &mut VerEx {
        self.or()
            .find(value)
    }

    /// A range of characters e.g. [A-Z]
    /// Usage example: verex.range(vec![('a', 'z'),('A', 'Z')])
    pub fn range(&mut self, range: Vec<(char, char)>) -> &mut VerEx {
        let mut string = r"[".to_string();
        for tuple in range {
            let from = tuple.0;
            let to = tuple.1;
            string.push(from);
            string.push('-');
            string.push(to);
        }
        string.push(']');
        self.add(string.as_ref());
        self.update_source_with_modifiers()
    }

    /// Replace a substring
    pub fn replace(& self, text: &str, replacement: &str) -> Result<String, Error> {
        let regex = try!(self.compile());
        Ok(regex.replace(text, replacement))
    }

    /// Toggle whether ^ and $ match line start and end or string start and end
    pub fn search_one_line(&mut self, enable: bool) -> &mut VerEx {
        if enable {
            self.modifiers.remove(MULTI_LINE);
        }
        else {
            self.modifiers.insert(MULTI_LINE);
        }
        self.update_source_with_modifiers()
    }

    /// Any character at least one time
    pub fn something(&mut self) -> &mut VerEx {
        self.add(r"(.+)");
        self.update_source_with_modifiers()
    }

    /// Any character at least one time except for these characters
    pub fn something_but(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(value)
            .close_class()
            .add(r"+")
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Add a token for the start of a line
    pub fn start_of_line(&mut self) -> &mut VerEx {
        self.add(r"^");
        self.update_source_with_modifiers()
    }

    /// Add a token for a tab
    pub fn tab(&mut self) -> &mut VerEx {
        self.add(r"\t");
        self.update_source_with_modifiers()
    }

    /// To use find "in the sentence" and make the chaining flow better
    pub fn then(&mut self, value: &str) -> &mut VerEx {
        self.find(value)
    }

    /// Toggle whether to match case-sensitively or not
    pub fn with_any_case(&mut self, enable: bool) -> &mut VerEx {
        if enable {
            self.modifiers.insert(CASE_INSENSITIVE);
        }
        else {
            self.modifiers.remove(CASE_INSENSITIVE);
        }
        self.update_source_with_modifiers()
    }

    /// Any alphanumeric characters
    pub fn word(&mut self) -> &mut VerEx {
        self.find(r"\w+")
    }
}

use std::fmt;

impl fmt::Display for VerEx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Eq for VerEx {}

use std::str::FromStr;
pub enum Void {}

impl FromStr for VerEx {
    type Err = Void;

    fn from_str(s: &str) -> Result<VerEx, Void> {
        Ok(VerEx::from_str(s))
    }
}

/// Equality comparison is based on the original string. It is possible that different verbal expressions have the same matching behavior, but are still compared unequal.
impl PartialEq for VerEx {
    fn eq(&self, other: &VerEx) -> bool {
        self.string == other.string
    }
}
