// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn get_between_quotes(s: &str) -> &str {
    s.split('"').skip(1).next().unwrap_or("")
}

fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut f = File::open(p).expect("read_file::open failed");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read_file::read_to_end failed");
    content
}

fn get_headers(entries: &str) -> Vec<String> {
    entries.split(',')
        .map(|x| get_between_quotes(x).to_owned())
        .filter(|x| !x.is_empty())
        .collect()
}

fn check_inner_imports(
    line_pos: usize,
    some_text: &str,
    imports: &[String]
) -> u32 {
    let mut errors = 0;
    if imports.len() > 1 {
        for pos in 0..imports.len() - 1 {
            if imports[pos] > imports[pos + 1] {
                writeln!(&mut io::stderr(),
                         "[build.rs:{}] In {}: \"{}\" should be after \"{}\"",
                         line_pos,
                         some_text,
                         imports[pos],
                         imports[pos + 1]).unwrap();
                errors += 1;
            }
        }
        if errors > 0 {
            let mut sorted = imports.to_vec();
            sorted.sort();
            writeln!(&mut io::stderr(), "==> Correct imports: \"&{:?}\"",
                     sorted).unwrap();
        }
    }
    errors
}

#[test]
fn check_build_headers_sorted() {
    let content = read_file("build.rs");
    let mut inside = false;
    let mut files: Vec<Vec<(String, usize, String)>> = Vec::new();
    let mut errors = 0;
    let mut new_group = false;

    for (pos, line) in content.lines().enumerate() {
        let line = line.trim_left();
        if !inside && line.starts_with("const DATA: ") {
            inside = true;
        } else if inside == true {
            let line = line.trim_left();
            if line.starts_with("//") {
                new_group = true;
                continue;
            } else if !line.starts_with("(\"") {
                break;
            }
            let parts: Vec<&str> = line.split("&[").collect();
            let header_dependencies = get_headers(parts[1]);
            let library_dependencies = get_headers(parts[2]);
            errors += check_inner_imports(pos + 1, "header dependencies", &header_dependencies);
            errors += check_inner_imports(pos + 1, "library dependencies", &library_dependencies);
            if new_group == true {
                files.push(Vec::new());
                new_group = false;
            }
            let len = files.len() - 1;
            files[len].push((parts[0].to_owned(), pos + 1, line.to_owned()));
        }
    }
    for entry in files {
        if entry.len() > 1 {
            for pos in 0..entry.len() - 1 {
                if entry[pos].0 > entry[pos + 1].0 {
                    writeln!(&mut io::stderr(),
                             "[build.rs:{}] \"{}\" should be after \"{}\"",
                             entry[pos].1,
                             entry[pos].2,
                             entry[pos + 1].2).unwrap();
                    errors += 1;
                }
            }
        }
    }
    assert!(errors == 0, "Not sorted header(s) found");
}
