mod diff;

const USAGE: &str = "Usage: [start filepath] [edit filepath]";

fn main() {
    let (start, edit) = get_files();
    let diff = diff::Diff::diff(&start, &edit);
    print!("{diff}");
}

fn get_files() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();
    
    let get_arg = |idx: usize| args.get(idx).unwrap_or_else(|| {
        eprintln!("{USAGE}");
        std::process::exit(1);
    });
    let read_file = |path: &str| std::fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Failed to read '{}'", path);
        std::process::exit(1)
    });

    let start = get_arg(1);
    let edit = get_arg(2);

    let start = read_file(start);
    let edit = read_file(edit);

    (start, edit)
}