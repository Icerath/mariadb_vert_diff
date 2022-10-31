#![allow(clippy::self_named_constructors)]

use std::collections::HashSet;
use std::fmt;


pub struct Diff<'a> {
    added: Vec<&'a str>,
    removed: Vec<&'a str>,
}

// Public Creation Method
impl<'a> Diff<'a> {
    pub fn diff(from: &'a str, to: &'a str) -> Self {
        let start: Vec<&'a str> = Diff::seperate(from);
        let edit: Vec<&'a str> = Diff::seperate(to);

        Diff::diff_hash(start, edit)
    }
}

// Private Utility Methods
impl<'a> Diff<'a> {
    /// Gets the items in start but not in edit, and the items in edit but not in start.
    /// Should be O(n) time complexity and O(n) extra space complexity
    fn diff_hash(start: Vec<&'a str>, edit: Vec<&'a str>) -> Self {
        let get_missing = |vec: Vec<&'a str>, set: HashSet<&'a str>| vec
            .iter()
            .filter(|s| !set.contains(*s))
            .copied()
            .collect();
        
        let hash_start: HashSet<&str> = start.iter().copied().collect();
        let hash_edit: HashSet<&str> = edit.iter().copied().collect();

        let added: Vec<&str> = get_missing(edit, hash_start);
        let removed: Vec<&str> = get_missing(start, hash_edit);
        
        Diff {
            added,
            removed,
        }
    }
    fn seperate(file: &str) -> Vec<&str> {
        let mut rows: Vec<&str> = vec![];
        let mut linestart: usize = 0;
        let mut cur: usize = 0;
        let mut next_newline: bool = true;

        while let Some(next) = file.get(cur..) {
            if next_newline && next.starts_with('\n') {
                linestart = cur;
                next_newline = false;
            }
            if next.starts_with("\n*") {
                let s = &file[linestart..cur];
                rows.push(s);
                linestart = cur;
                next_newline = true;
            }
            cur += 1;
        }
        let s = &file[linestart..cur-1];
        if !s.starts_with("\n*") {
            rows.push(s);
        }

        rows.into_iter().map(|s| s.trim()).filter(|v| !v.is_empty()).collect()
    }
}

impl<'a> fmt::Display for Diff<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = vec![];
        if !self.added.is_empty() {
            output.push(Self::format(&self.added, "> "));
        }
        if !self.removed.is_empty() {
            output.push(Self::format(&self.removed, "< "));
        }
        write!(f, "{}", output.join("\n\n").trim())
    }
}

impl<'a> Diff<'a> {
    fn format(text: &[&str], start: &str) -> String {
        text
        .iter()
        .map(|s| {
            s.trim().lines()
                .map(|line| start.to_string() + line + "\n")
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
    }
}

#[cfg(test)]
mod seperate {
    use super::Diff;
    #[test]
    fn not_leading_or_trailing() {
        let input = r#"
first
**row 1.
second
**row 2.
third
        "#;
        let seperate = Diff::seperate(input);
        assert_eq!(seperate, vec!["first", "second", "third"]);
    }
    #[test]
    fn trailing() {
        let input = r#"
first
**row 1.
second
**row 2.
third
**row 3.
        "#;
        let seperate = Diff::seperate(input);
        assert_eq!(seperate, vec!["first", "second", "third"]);
    }
    #[test]
    fn leading() {
        let input = r#"
**row 1.**
first
**row 2.
second
**row 3.
third
        "#;
        let seperate = Diff::seperate(input);
        assert_eq!(seperate, vec!["first", "second", "third"]);
    }
    #[test]
    fn leading_and_trailing() {
        let input = r#"
**row 1.**
first
**row 2.
second
**row 3.
third
**row 4.
        "#;
        let seperate = Diff::seperate(input);
        assert_eq!(seperate, vec!["first", "second", "third"]);
    }
}


#[cfg(test)]
mod diff_hash {
    use super::Diff;
    #[test]
    fn no_diff() {
        let start = vec!["first", "second", "third"];
        let edit = vec!["first", "second", "third"];

        let diff = Diff::diff_hash(start, edit);
        let empty: Vec<&str> = vec![];

        assert_eq!(diff.added, empty);
        assert_eq!(diff.removed, empty);
    }
    #[test]
    fn added() {
        let start = vec!["first", "second", "third"];
        let edit = vec!["first", "second", "third", "fourth"];

        let diff = Diff::diff_hash(start, edit);
        
        let added: Vec<&str> = vec!["fourth"];
        let removed: Vec<&str> = vec![];

        assert_eq!(diff.added, added);
        assert_eq!(diff.removed, removed);
    }

    #[test]
    fn removed() {
        let start = vec!["first", "second", "third", "fourth"];
        let edit = vec!["first", "second", "third"];

        let diff = Diff::diff_hash(start, edit);
        
        let added: Vec<&str> = vec![];
        let removed: Vec<&str> = vec!["fourth"];

        assert_eq!(diff.added, added);
        assert_eq!(diff.removed, removed);
    }

    #[test]
    fn added_and_removed() {
        let start = vec!["first", "second", "third", "fourth", "sixth"];
        let edit = vec!["first", "second", "third", "fifth", "seventh"];

        let diff = Diff::diff_hash(start, edit);
        
        let added: Vec<&str> = vec!["fifth", "seventh"];
        let removed: Vec<&str> = vec!["fourth", "sixth"];

        assert_eq!(diff.added, added);
        assert_eq!(diff.removed, removed);
    }
}

#[cfg(test)]
mod format {
    use super::Diff;

    #[test]
    fn no_diff() {
        let start = vec!["first", "second", "third"];
        let edit = vec!["first", "second", "third"];

        let diff = Diff::diff_hash(start, edit);
        
        let expected = String::from("");
        assert_eq!(format!("{diff}"), expected)
    }
    #[test]
    fn added() {
        let start = vec!["first", "second", "third"];
        let edit = vec!["first", "second", "third", "fourth"];

        let diff = Diff::diff_hash(start, edit);
        
        let expected = String::from("> fourth");
        assert_eq!(format!("{diff}"), expected)
    }
    #[test]
    fn removed() {
        let start = vec!["first", "second", "third", "fourth"];
        let edit = vec!["first", "second", "third"];

        let diff = Diff::diff_hash(start, edit);
        
        let expected = String::from("< fourth");
        assert_eq!(format!("{diff}"), expected)
    }
    #[test]
    fn added_and_removed() {
        let start = vec!["first", "second", "third", "fourth", "sixth"];
        let edit = vec!["first", "second", "third", "fifth", "seventh"];

        let diff = Diff::diff_hash(start, edit);

        let expected = String::from("> fifth\n\n> seventh\n\n\n< fourth\n\n< sixth");
        assert_eq!(format!("{diff}"), expected)
    }
}