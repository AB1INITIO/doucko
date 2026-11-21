//! Builds the site from the sources in the current directory into [`OUT`].
//!
//! The sources are a few top-level files (templates, landing page, stylesheet)
//! and *sections*: the visible top-level directories other than tooling and
//! output. A section's Markdown pages are rendered by pandoc, its other files
//! are copied as they are, and an index page lists its pages. Each page links
//! to its PDF, which [`Site::pdf`] renders for the PDF workflow to publish.
//!
//! Each output depends on one source, except a section's index (on the list of
//! its pages) and every page (on the page template), so [`Site::update`] can
//! refresh just what a change affects. Outputs are written to a hidden temp
//! file and renamed into place: a server never sees a half-written file.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::preprocess::preprocess;

/// The output directory.
pub const OUT: &str = "_site";
/// The output directory of [`Site::pdf`].
const PDF_OUT: &str = "_pdf";
/// Where each page links to its PDF: the `pdf` release, kept up to date by the
/// PDF workflow (`.github/workflows/pdf.yml`).
const PDF_URL: &str = "https://github.com/AB1INITIO/doucko/releases/download/pdf";
/// pandoc's template for every page.
const PAGE_TEMPLATE: &str = "pandoc-template.html";
/// A section's index page, with `{{SECTION}}` and `{{ITEMS}}` to fill in.
const INDEX_TEMPLATE: &str = "index-template.html";
/// Top-level files copied into the site as they are.
const STATIC: [&str; 2] = ["index.html", "style.css"];
/// Top-level directories that aren't sections.
const NOT_SECTIONS: [&str; 4] = [OUT, PDF_OUT, "scripts", "templates"];
/// pandoc's options for every page, as HTML or as PDF.
const PANDOC_ARGS: [&str; 2] = ["--standalone", "--from=markdown+tex_math_dollars"];

/// The site's builder, set up for the installed pandoc.
pub struct Site {
    /// pandoc's syntax-highlighting option: newer pandoc renamed it and warns
    /// about the old name on every page, older pandoc (as on CI) knows only it.
    highlight: &'static str,
}

impl Site {
    /// Checks that this is the site's root and that pandoc is installed.
    pub fn new() -> io::Result<Self> {
        if !Path::new(PAGE_TEMPLATE).is_file() {
            let msg = format!("{PAGE_TEMPLATE} not found: run this from the site's root");
            return Err(io::Error::other(msg));
        }
        let help = Command::new("pandoc")
            .arg("--help")
            .output()
            .map_err(|e| match e.kind() {
                io::ErrorKind::NotFound => {
                    io::Error::other("pandoc not found: see https://pandoc.org/installing.html")
                }
                _ => io::Error::new(e.kind(), format!("cannot run pandoc: {e}")),
            })?;
        let renamed = String::from_utf8_lossy(&help.stdout).contains("--syntax-highlighting");
        let highlight = if renamed {
            "--syntax-highlighting=haddock"
        } else {
            "--highlight-style=haddock"
        };
        Ok(Site { highlight })
    }

    /// Builds the whole site from scratch.
    pub fn build(&self) -> io::Result<()> {
        println!("Building into {OUT}/ ...");
        remove_all(Path::new(OUT))?;
        self.build_all()?;
        println!("Done. Site written to {OUT}/");
        Ok(())
    }

    /// Refreshes just the outputs that depend on `paths`, sources which may
    /// have been added, changed or deleted.
    pub fn update(&self, paths: &[PathBuf]) -> io::Result<()> {
        if paths.iter().any(|path| path == Path::new(PAGE_TEMPLATE)) {
            println!("  {PAGE_TEMPLATE} changed — rebuilding every page");
            return self.build_all();
        }
        let mut run = Run::default();
        let mut indexes = BTreeSet::new(); // sections whose index is out of date
        for path in paths {
            let section = match classify(path) {
                Source::PageTemplate | Source::Other => continue,
                Source::IndexTemplate => {
                    indexes.extend(sections()?);
                    continue;
                }
                Source::Static => {
                    run.check(path, sync(path));
                    continue;
                }
                Source::Page(section) => {
                    // The index lists pages, so adding or deleting one changes
                    // it, while editing one doesn't.
                    let html = page_output(path);
                    let exists = path.is_file();
                    if !(exists && html.is_file()) {
                        indexes.insert(section.to_owned());
                    }
                    if exists {
                        run.check(path, self.render(path));
                    } else {
                        run.check(path, remove(path, &html));
                    }
                    section
                }
                Source::Asset { section, listed } => {
                    if listed {
                        indexes.insert(section.to_owned());
                    }
                    run.check(path, sync(path));
                    section
                }
            };
            if !out(section).join("index.html").is_file() {
                indexes.insert(section.to_owned()); // a new section
            }
        }
        for section in indexes.iter().filter(|section| Path::new(section).is_dir()) {
            run.check(section, render_index(section));
        }
        run.finish()
    }

    /// Writes every output, over whatever the output directory already holds.
    fn build_all(&self) -> io::Result<()> {
        let mut run = Run::default();
        for file in STATIC {
            run.check(file, sync(Path::new(file)));
        }
        for section in sections()? {
            println!("  [{section}]");
            for path in entries(Path::new(&section))? {
                if let Source::Page(_) = classify(&path) {
                    run.check(&path, self.render(&path));
                } else {
                    log("copy", arrow(&path, &out(&path)));
                    run.check(&path, copy_tree(&path, &out(&path)));
                }
            }
            run.check(&section, render_index(&section));
        }
        run.finish()
    }

    /// Renders into `PDF_OUT` the PDFs of the pages that `paths` affect, as
    /// [`affected`] finds them.
    pub fn pdf(&self, paths: &[impl AsRef<Path>]) -> io::Result<()> {
        let mut run = Run::default();
        for md in affected(paths)? {
            let pdf = Path::new(PDF_OUT).join(pdf_name(&md));
            log("pdf", arrow(&md, &pdf));
            run.check(&md, self.pandoc(&md, &pdf, &["--pdf-engine=pdflatex"]));
        }
        run.finish()
    }

    /// Renders the page `SECTION/NAME.md` into `OUT/SECTION/NAME.html`, which
    /// links to its PDF.
    fn render(&self, md: &Path) -> io::Result<()> {
        let html = page_output(md);
        log("pandoc", arrow(md, &html));
        let pdf = format!("pdf-url={PDF_URL}/{}", pdf_name(md));
        let args = [
            "--template",
            PAGE_TEMPLATE,
            "--to=html5",
            "--katex",
            "--variable",
            &pdf,
        ];
        self.pandoc(md, &html, &args)
    }

    /// Renders the page `md` into `out` with pandoc, given options for the format.
    fn pandoc(&self, md: &Path, out: &Path, args: &[&str]) -> io::Result<()> {
        let markdown = preprocess(&fs::read_to_string(md)?);
        write_atomic(out, |tmp| {
            let mut child = Command::new("pandoc")
                .args(PANDOC_ARGS)
                .args(args)
                .arg(self.highlight)
                .arg("--resource-path")
                .arg(md.parent().unwrap_or(Path::new(".")))
                .arg("--output")
                .arg(tmp)
                .stdin(Stdio::piped())
                .spawn()?;
            // On failure, pandoc's exit status explains more than the broken pipe.
            let stdin = child.stdin.take();
            let written = stdin
                .expect("stdin is piped")
                .write_all(markdown.as_bytes());
            let status = child.wait()?;
            if !status.success() {
                return Err(io::Error::other(format!("pandoc failed ({status})")));
            }
            written
        })
    }
}

/// The pages whose PDF depends on any of `paths`, which changed: the page
/// itself, or a file it embeds, such as an image (found by its path in the
/// page); every page if `paths` is empty.
pub fn affected(paths: &[impl AsRef<Path>]) -> io::Result<Vec<PathBuf>> {
    let mut pages = Vec::new();
    for section in sections()? {
        for path in entries(Path::new(&section))? {
            let page = matches!(classify(&path), Source::Page(_));
            if page && (paths.is_empty() || depends(&path, paths)?) {
                pages.push(path);
            }
        }
    }
    Ok(pages)
}

fn depends(page: &Path, paths: &[impl AsRef<Path>]) -> io::Result<bool> {
    if paths.iter().any(|path| path.as_ref() == page) {
        return Ok(true);
    }
    let dir = page.parent().expect("pages are in sections");
    let embeds: Vec<_> = paths
        .iter()
        .filter_map(|path| path.as_ref().strip_prefix(dir).ok()?.to_str())
        .filter(|path| !path.is_empty())
        .collect();
    if embeds.is_empty() {
        return Ok(false);
    }
    let text = fs::read_to_string(page)?;
    Ok(embeds.iter().any(|path| text.contains(path)))
}

/// The file name of the page's PDF, `SECTION-NAME.pdf`, with every run of
/// characters other than ASCII letters, digits, `.` and `_` made a single `-`:
/// GitHub would rename such a release asset, breaking its link.
pub fn pdf_name(md: &Path) -> String {
    let mut name = String::new();
    for c in md.with_extension("").to_string_lossy().chars() {
        if c.is_ascii_alphanumeric() || c == '.' || c == '_' {
            name.push(c);
        } else if !name.ends_with('-') {
            name.push('-');
        }
    }
    format!("{}.pdf", name.trim_matches('-'))
}

/// Every file the site is built from, for the watcher to track.
pub fn sources() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<_> = [PAGE_TEMPLATE, INDEX_TEMPLATE]
        .into_iter()
        .chain(STATIC)
        .map(PathBuf::from)
        .collect();
    for section in sections()? {
        walk(Path::new(&section), &mut files)?;
    }
    Ok(files)
}

fn walk(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for path in entries(dir)? {
        if path.is_dir() {
            walk(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

/// What a source feeds into, which decides what a change to it refreshes.
#[derive(Debug, PartialEq)]
enum Source<'a> {
    /// The page template, used by every page.
    PageTemplate,
    /// The index template, used by every section's index.
    IndexTemplate,
    /// A top-level file, copied as it is.
    Static,
    /// `SECTION/NAME.md`: rendered to `SECTION/NAME.html`, listed in the index.
    Page(&'a str),
    /// Any other file in a section: copied as it is, and `listed` in the index
    /// if it's an HTML file at the section's top level.
    Asset { section: &'a str, listed: bool },
    /// Not part of the site.
    Other,
}

fn classify(path: &Path) -> Source<'_> {
    let Some(path) = path.to_str() else {
        return Source::Other;
    };
    match path.split_once('/') {
        None if path == PAGE_TEMPLATE => Source::PageTemplate,
        None if path == INDEX_TEMPLATE => Source::IndexTemplate,
        None if STATIC.contains(&path) => Source::Static,
        Some((section, name)) if is_section(section) => {
            let top = !name.contains('/');
            if top && name.ends_with(".md") {
                Source::Page(section)
            } else {
                Source::Asset {
                    section,
                    listed: top && name.ends_with(".html"),
                }
            }
        }
        _ => Source::Other,
    }
}

fn is_section(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('.') && !NOT_SECTIONS.contains(&name)
}

/// The names of all sections, sorted.
fn sections() -> io::Result<Vec<String>> {
    Ok(entries(Path::new(""))?
        .into_iter()
        .filter(|path| path.is_dir())
        .filter_map(|path| path.into_os_string().into_string().ok())
        .filter(|name| is_section(name))
        .collect())
}

/// The visible (not dot-prefixed) entries of `dir`, sorted; `""` is the
/// current directory.
fn entries(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    })? {
        let name = entry?.file_name();
        if !name.as_encoded_bytes().starts_with(b".") {
            paths.push(dir.join(name));
        }
    }
    paths.sort();
    Ok(paths)
}

/// Writes the section's index page, listing the pages in its output directory.
fn render_index(section: &str) -> io::Result<()> {
    let dir = out(section);
    fs::create_dir_all(&dir)?;
    let mut pages: Vec<_> = entries(&dir)?
        .into_iter()
        .filter(|path| path.is_file())
        .filter_map(|path| {
            Some(
                path.file_name()?
                    .to_str()?
                    .strip_suffix(".html")?
                    .to_owned(),
            )
        })
        .filter(|page| page != "index")
        .collect();
    pages.sort_by_cached_key(|page| page.to_lowercase()); // as a reader expects, on any OS
    let items: Vec<_> = pages
        .iter()
        .map(|page| format!(r#"      <li><a href="{0}.html">{0}</a></li>"#, escape(page)))
        .collect();
    let html = fs::read_to_string(INDEX_TEMPLATE)?
        .replace("{{SECTION}}", &escape(section))
        .replace("{{ITEMS}}", &items.join("\n"));
    let index = dir.join("index.html");
    log("index", format_args!("→ {}", index.display()));
    write_atomic(&index, |tmp| fs::write(tmp, html))
}

/// Copies the file `src` into the site, or removes its output if it's gone.
fn sync(src: &Path) -> io::Result<()> {
    let dst = out(src);
    if !src.is_file() {
        return remove(src, &dst);
    }
    log("copy", arrow(src, &dst));
    copy(src, &dst)
}

/// Copies `src`, a file or a directory, to `dst`, skipping hidden files.
fn copy_tree(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return copy(src, dst);
    }
    fs::create_dir_all(dst)?;
    for path in entries(src)? {
        copy_tree(
            &path,
            &dst.join(path.file_name().expect("entries have names")),
        )?;
    }
    Ok(())
}

fn copy(src: &Path, dst: &Path) -> io::Result<()> {
    write_atomic(dst, |tmp| fs::copy(src, tmp).map(drop))
}

/// Removes `dst`, the output of the deleted `src`, along with the output
/// directories whose sources are gone too (a deleted `img/` or section).
fn remove(src: &Path, dst: &Path) -> io::Result<()> {
    if dst.is_file() {
        log("remove", dst.display());
        fs::remove_file(dst)?;
    }
    for dir in src
        .ancestors()
        .skip(1)
        .take_while(|dir| !dir.as_os_str().is_empty() && !dir.is_dir())
    {
        remove_all(&out(dir))?;
    }
    Ok(())
}

/// Removes the directory `path` and everything in it, if it exists.
fn remove_all(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// Fills `dst` through `write`, which writes a hidden temp file beside it that
/// is then renamed into place: readers see either the old file or the new one.
fn write_atomic(dst: &Path, write: impl FnOnce(&Path) -> io::Result<()>) -> io::Result<()> {
    let dir = dst.parent().expect("outputs are in OUT");
    fs::create_dir_all(dir)?;
    let mut name = OsString::from(".tmp."); // keeps the extension, which tells pandoc the format
    name.push(dst.file_name().expect("outputs are files"));
    let tmp = dir.join(name);
    let result = write(&tmp).and_then(|()| fs::rename(&tmp, dst));
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Where `source` goes in the site.
fn out(source: impl AsRef<Path>) -> PathBuf {
    Path::new(OUT).join(source)
}

fn page_output(md: &Path) -> PathBuf {
    out(md).with_extension("html")
}

/// Escapes text for HTML content and attribute values.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn arrow(from: &Path, to: &Path) -> String {
    format!("{} → {}", from.display(), to.display())
}

fn log(action: &str, what: impl Display) {
    println!("    {action:<7} {what}");
}

/// Counts failed steps, so that one broken file doesn't stop a whole build.
#[derive(Default)]
struct Run {
    failed: usize,
}

impl Run {
    fn check(&mut self, what: impl AsRef<Path>, result: io::Result<()>) {
        if let Err(e) = result {
            eprintln!("    error   {}: {e}", what.as_ref().display());
            self.failed += 1;
        }
    }

    fn finish(self) -> io::Result<()> {
        match self.failed {
            0 => Ok(()),
            n => Err(io::Error::other(format!(
                "{n} file(s) failed to build, see above"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Source::*, *};

    #[test]
    fn names_pdfs() {
        let cases = [
            ("mat/Kruh a Kruznica.md", "mat-Kruh-a-Kruznica.pdf"),
            (
                "inf/Zaklady Pythonu - Doplnkove Cvicenia.md",
                "inf-Zaklady-Pythonu-Doplnkove-Cvicenia.pdf",
            ),
            ("inf/2D List.md", "inf-2D-List.pdf"),
            ("mat/a_b (c).md", "mat-a_b-c.pdf"),
        ];
        for (md, pdf) in cases {
            assert_eq!(pdf_name(Path::new(md)), pdf);
        }
    }

    #[test]
    fn classifies_sources() {
        let cases = [
            ("pandoc-template.html", PageTemplate),
            ("index-template.html", IndexTemplate),
            ("style.css", Static),
            ("mat/IB.md", Page("mat")),
            (
                "mat/extra.html",
                Asset {
                    section: "mat",
                    listed: true,
                },
            ),
            (
                "mat/img/a.png",
                Asset {
                    section: "mat",
                    listed: false,
                },
            ),
            (
                "mat/notes/a.md",
                Asset {
                    section: "mat",
                    listed: false,
                },
            ),
            ("Makefile", Other),
            ("scripts/src/main.rs", Other),
            ("_site/mat/IB.html", Other),
            ("_pdf/mat-IB.pdf", Other),
            (".git/HEAD", Other),
        ];
        for (path, source) in cases {
            assert_eq!(classify(Path::new(path)), source, "{path}");
        }
    }
}
