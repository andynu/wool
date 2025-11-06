use clap::{App, Arg, ArgMatches};

pub fn get_cli_matches<'a>() -> ArgMatches<'static> {
    App::new("ynot")
        .version("0.1")
        .author("m")
        .about("See https://github.com/grapegrip/wool")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("export-flag")
                .short("e")
                .long("export")
                .help("Export html"),
        )
        .arg(
            Arg::with_name("no-preview-frame")
                .short("n")
                .long("no-preview-frame")
                .help("Don't render the preview frame"),
        )
        .arg(
            Arg::with_name("no-highlight")
                .long("no-highlight")
                .help("Disable syntax highlighting (enabled by default)"),
        )
        .arg(
            Arg::with_name("no-browser")
                .short("b")
                .long("no-browser")
                .help("Don't open browser (browser opens by default)"),
        )
        .arg(
            Arg::with_name("no-katex")
                .long("no-katex")
                .help("Disable KaTeX math rendering (enabled by default)"),
        )
        .arg(
            Arg::with_name("no-d2")
                .long("no-d2")
                .help("Disable D2 diagram rendering (enabled by default)"),
        )
        .arg(Arg::with_name("outfile").help("Sets the output file to use"))
        .get_matches()
}
