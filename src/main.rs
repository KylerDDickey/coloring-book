#![allow(unused)]
// TODO: Remove, not permanent.

mod args;
mod controller;
mod error;
mod model;
mod prelude;
mod view;

use clap::Parser;

use crate::args::{CliArgs, Command};
use crate::controller::HasView;
use crate::prelude::*;
use crate::view::{TestView, View};

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    match &cli_args.command {
        Command::Fill(fill_args) => {
            let test_view = TestView::bind_into_controller(&fill_args.template_file);

            let output = test_view.view().render();

            println!("{:?}", output);
        }
    }

    Ok(())
}
