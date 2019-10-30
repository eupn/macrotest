use difference::{Changeset, Difference};

/// Prints the difference of the two snippets of expanded code.
pub(crate) fn message_different(name: &str, a: &[u8], b: &[u8]) {
    let a = String::from_utf8_lossy(&a);
    let b = String::from_utf8_lossy(&b);

    let Changeset { diffs, .. } = Changeset::new(&a, &b, "\n");

    let mut lines_added = 0;
    let mut lines_removed = 0;
    for diff in &diffs {
        match diff {
            Difference::Add(s) => lines_added += s.lines().count(),
            Difference::Rem(s) => lines_removed += s.lines().count(),
            _ => (),
        }
    }

    eprintln!("{} - different!", name);

    eprintln!("Diff [lines: {} added, {} removed]:", lines_added, lines_removed);
    eprintln!("--------------------------");

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                for line in x.lines() {
                    eprintln!(" {}", line);
                }
            }
            Difference::Add(ref x) => {
                for line in x.lines() {
                    eprintln!("+{}", line);
                }
            }
            Difference::Rem(ref x) => {
                for line in x.lines() {
                    eprintln!("-{}", line);
                }
            }
        }
    }

    eprintln!("--------------------------");
}

/// Prints an error from `cargo expand` invocation.
/// Makes some suggestions when possible.
pub(crate) fn message_expansion_error(msg: Vec<u8>) {
    let msg = String::from_utf8(msg);

    eprintln!("Expansion error:");
    if let Ok(msg) = msg {
        eprintln!("{}", msg);

        // No `cargo expand` subcommand installed, make a suggestion
        if msg.contains("no such subcommand: `expand`") {
            eprintln!("Perhaps, `cargo expand` is not installed?");
            eprintln!("Install it by running:");
            eprintln!();
            eprintln!("\tcargo install cargo-expand");
            eprintln!();
        }

        // No nightly installed, make a suggestion
        if msg.starts_with("error: toolchain '") && msg.ends_with("is not installed") {
            eprintln!("You have `cargo expand` installed but it requires *nightly* compiler to be installed as well.");
            eprintln!("To install it via rustup, run:");
            eprintln!();
            eprintln!("\trustup toolchain install nightly");
            eprintln!();
        }
    } else {
        eprintln!("<unprintable>");
    }
}
