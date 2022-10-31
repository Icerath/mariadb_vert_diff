#[cfg(test)]
use crate::diff::Diff;


#[test]
fn test_leading_rownum() {
    let start = "**row1.**\nfirst\n**row 2.**\nsecond\n**row3.**\nthird";
    let edit = "\n**row1.**\nfourth\n**row 2.**\nfirst\n**row4.**\nsecond";
    let diff = Diff::diff(start, edit);
    let expected = Diff {
        added: vec!["fourth"],
        removed: vec!["third"],
    };
    assert_eq!(diff, expected);
}

#[test]
fn test_no_leading_rownum() {
    let start = "first\n**row 2.**\nsecond\n**row3.**\nthird";
    let edit = "\nfourth\n**row 2.**\nfirst\n**row4.**\nsecond";
    let diff = Diff::diff(start, edit);
    let expected = Diff {
        added: vec!["fourth"],
        removed: vec!["third"],
    };
    assert_eq!(diff, expected);
}

#[test]
fn test_trailing_rownum() {
    let start = "\nfirst\n**row 2.**\nsecond\n**row3.**\nthird\n**row4.**";
    let edit = "fourth\n**row 2.**\nfirst\n**row4.**\nsecond\n**row5.**\n";
    let diff = Diff::diff(start, edit);
    let expected = Diff {
        added: vec!["fourth"],
        removed: vec!["third"],
    };
    assert_eq!(diff, expected);
}

#[test]
fn test_leading_and_trailing_rownum() {
    let start = "**row1.**\nfirst\n**row 2.**\nsecond\n**row3.**\nthird\n**row4.**";
    let edit = "\n**row1.**\nfourth\n**row 2.**\nfirst\n**row4.**\nsecond\n**row5.**\n";
    let diff = Diff::diff(start, edit);
    let expected = Diff {
        added: vec!["fourth"],
        removed: vec!["third"],
    };
    assert_eq!(diff, expected);
}