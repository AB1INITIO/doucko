//! Rewrites LaTeX-style macros into pandoc-native Markdown, leaving fenced code
//! blocks and inline code spans untouched:
//!
//! | Source            | Output               |
//! |-------------------|----------------------|
//! | `\url{URL}`       | `[URL](URL){.uri}`   |
//! | `\footnote{TEXT}` | `^[TEXT]`            |
//!
//! Arguments are brace-balanced, so they may hold braces themselves, as math
//! does: `\footnote{see $f(x) = \{x\}$}`.

/// Turns a macro's argument into its expansion.
type Expand = fn(&str) -> String;

/// Each macro `\NAME{ARG}`, by name. To support another macro, add it here.
const MACROS: &[(&str, Expand)] = &[
    ("url", |url| format!("[{url}]({url}){{.uri}}")),
    ("footnote", |text| format!("^[{}]", preprocess(text))),
];

pub fn preprocess(src: &str) -> String {
    // Scans bytes: everything matched is ASCII, so every slice boundary is a
    // char boundary, while non-ASCII text is passed over and copied whole.
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let (mut i, mut copied) = (0, 0); // `src[copied..i]` is still to be copied
    while i < bytes.len() {
        let rest = &bytes[i..];
        let line_start = i == 0 || bytes[i - 1] == b'\n';
        if line_start && (rest.starts_with(b"```") || rest.starts_with(b"~~~")) {
            i = fence_end(src, i);
        } else if rest[0] == b'`' {
            i = src[i + 1..].find('`').map_or(src.len(), |j| i + j + 2);
        } else if let Some((expansion, end)) = expand(src, i) {
            out.push_str(&src[copied..i]);
            out.push_str(&expansion);
            (i, copied) = (end, end);
        } else {
            i += 1;
        }
    }
    out.push_str(&src[copied..]);
    out
}

/// Where the fenced code block opening at `start` ends: past its closing
/// fence's line, or at the end of input if it's never closed.
fn fence_end(src: &str, start: usize) -> usize {
    let fence = &src[start..start + 3];
    let Some(close) = src[start + 3..].find(&format!("\n{fence}")) else {
        return src.len();
    };
    let after = start + 3 + close + 1 + fence.len();
    src[after..].find('\n').map_or(src.len(), |j| after + j + 1)
}

/// The expansion of the macro starting at `at`, if any, and where it ends.
fn expand(src: &str, at: usize) -> Option<(String, usize)> {
    if src.as_bytes()[at] != b'\\' {
        return None;
    }
    let rest = &src[at + 1..];
    MACROS.iter().find_map(|(name, expand)| {
        if !rest.strip_prefix(name)?.starts_with('{') {
            return None;
        }
        let (arg, end) = braced(src, at + 1 + name.len())?;
        Some((expand(arg), end))
    })
}

/// The contents of the brace group opening at `open`, and the index past its
/// closing brace; `None` if the braces never balance.
fn braced(src: &str, open: usize) -> Option<(&str, usize)> {
    let mut depth = 0;
    for (i, &byte) in src.as_bytes().iter().enumerate().skip(open) {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&src[open + 1..i], i + 1));
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::preprocess;

    #[test]
    fn expands_macros() {
        let cases = [
            (
                r"see \url{https://a.sk}.",
                "see [https://a.sk](https://a.sk){.uri}.",
            ),
            (r"x\footnote{note}", "x^[note]"),
            (r"x\footnote{$f(x) = \{x\}$}", r"x^[$f(x) = \{x\}$]"),
            (r"\footnote{at \url{u}}", "^[at [u](u){.uri}]"),
            (r"Kružnica\footnote{č}ľ", "Kružnica^[č]ľ"),
            ("```\nx\n```\n\\url{u}", "```\nx\n```\n[u](u){.uri}"),
        ];
        for (src, expected) in cases {
            assert_eq!(preprocess(src), expected, "{src:?}");
        }
    }

    #[test]
    fn leaves_code_and_non_macros_alone() {
        let cases = [
            "`\\url{a}` and\n```py\n\\footnote{b}\n```\n~~~\n\\url{c}\n~~~\n",
            r"\footnote{never closed",
            "`never closed \\url{a}",
            "```\nnever closed \\url{a}",
            r"\urls{a} \url {a} \footnote",
        ];
        for src in cases {
            assert_eq!(preprocess(src), src, "{src:?}");
        }
    }
}
