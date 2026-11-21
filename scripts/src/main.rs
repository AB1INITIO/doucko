//! Builds the "ab initio" teaching site: sections of Markdown pages enriched
//! with KaTeX math, rendered to HTML by pandoc.

mod preprocess;
mod serve;
mod site;
mod watch;

use std::env;
use std::io;
use std::process::ExitCode;
use std::thread;

use site::Site;
use watch::Watcher;

const USAGE: &str = "\
Usage: abinitio <COMMAND>

Commands:
  build                 Build the site into _site/ from scratch
  watch                 Build, then rebuild whatever changes as you edit
  serve [ADDR]          Watch, and serve the site at ADDR: a port on
                        localhost, or HOST:PORT [default: 8000]
  pdf [PATH...]         Render into _pdf/ the PDFs of the pages that PATHs
                        affect (pages, or files they embed), or of all pages
  pdf --list [PATH...]  Print the names of those PDFs instead

Run it from the site's root directory. It needs pandoc, and LaTeX for PDFs.";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match args[..] {
        ["build"] => Site::new().and_then(|site| site.build()),
        ["watch"] => develop(None),
        ["serve"] => develop(Some("8000")),
        ["serve", addr] => develop(Some(addr)),
        ["pdf", "--list", ref paths @ ..] => site::affected(paths).map(|pages| {
            for page in pages {
                println!("{}", site::pdf_name(&page));
            }
        }),
        ["pdf", ref paths @ ..] => Site::new().and_then(|site| site.pdf(paths)),
        ["help" | "-h" | "--help"] => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Builds the site, serves it at `addr` if given, and rebuilds on every change.
/// Errors from building are reported but don't stop it: fix and save to retry.
fn develop(addr: Option<&str>) -> io::Result<()> {
    let site = Site::new()?;
    let listener = addr.map(serve::bind).transpose()?; // fail before building
    let watcher = Watcher::new();
    if let Err(e) = site.build() {
        eprintln!("error: {e}");
    }
    if let Some(listener) = listener {
        println!(
            "Serving {}/ at http://{}/",
            site::OUT,
            listener.local_addr()?
        );
        thread::spawn(move || serve::run(listener));
    }
    watcher.run(&site)
}
