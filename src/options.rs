use std::path::PathBuf;

use clap::Parser;

use crate::viewer::Mode;
use crate::viewer::Preview;

/// A pager for JSON data
#[derive(Debug, Parser)]
#[clap(name = "jless", version)]
pub struct Opt {
    /// Input file. jless will read from stdin if no input
    /// file is provided, or '-' is specified.
    #[clap(parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Initial viewing mode. In line mode (--mode line), opening
    /// and closing curly and square brackets are shown and all
    /// Object keys are quoted. In data mode (--mode data; the default),
    /// closing braces, commas, and quotes around Object keys are elided.
    /// The active mode can be toggled by pressing 'm'.
    #[clap(short, long, arg_enum, hide_possible_values = true, default_value_t = Mode::Data)]
    pub mode: Mode,

    /// Initial preview of container nodes. In full mode (--preview full;
    /// the default), containers will be rendered as much as they can be in
    /// the width of the terminal. In count mode (--preview count), only
    /// the child node count will be rendered. This can be toggled by
    /// pressing 'p'.
    #[clap(short, long, arg_enum, hide_possible_values = true, default_value_t = Preview::Full)]
    pub preview: Preview,

    /// Number of lines to maintain as padding between the currently
    /// focused row and the top or bottom of the screen. Setting this to
    /// a large value will keep the focused in the middle of the screen
    /// (except at the start or end of a file).
    #[clap(long = "scrolloff", default_value_t = 3)]
    pub scrolloff: u16,
}
