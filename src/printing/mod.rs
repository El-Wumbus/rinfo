pub mod ascii_art;
pub use ascii_art::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum OsArt
{
    ArchLinux,
    AlpineLinux,
}

pub fn print_with_logo(os: OsArt, s: &str)
{
    let art = match os
    {
        OsArt::ArchLinux => ARCH_LINUX,
        OsArt::AlpineLinux => ALPINE_LINUX,
    };

    let indentation = {
        let mut indent = String::from("\t");
        let (min, max) = min_max_line_len(art);
        let space_diff = (max - min) / 8;

        if space_diff > 0
        {
            for _ in 0..space_diff
            {
                indent.push_str("  ");
            }
        }
        else if space_diff == 0
        {
            indent.push('\t');
        }
        indent
    };


    println!("{}", with_both(art, s, &indentation));
}

/// Returns the length of the longest line in the string
fn min_max_line_len(s: &str) -> (usize, usize)
{
    let mut max = 0;
    let mut min = s.split("\n").nth(0).unwrap_or("").len();

    for line in s.split("\n")
    {
        let len = line.len();

        if len > max
        {
            max = len;
        }

        if len < min
        {
            min = len;
        }
    }

    (min, max)
}

fn with_both(first: &str, second: &str, indentation: &str) -> String
{
    let first: Vec<&str> = first.split('\n').collect();
    let second: Vec<&str> = second.split('\n').collect();
    let mut s = String::new();

    let mut i = 0;
    while i < first.len() || i < second.len()
    {
        let fir = first.get(i).unwrap_or(&"");
        let sec = second.get(i).unwrap_or(&"");

        s.push_str(&format!("{fir}{indentation}{sec}\n"));

        i += 1;
    }

    s
}
