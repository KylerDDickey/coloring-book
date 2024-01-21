#![allow(unused)]
// TODO: Remove, not permanent.

mod args;
mod controller;
mod error;
mod model;
mod prelude;
mod view;

use clap::Parser;
use std::rc::Rc;

use crate::args::{CliArgs, Command};
use crate::controller::{Controller, TestController};
use crate::model::{CellModel, Model};
use crate::prelude::*;
use crate::view::{TestView, View};

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    match &cli_args.command {
        Command::Fill(fill_args) => {
            let model = CellModel::new(fill_args.template_file.clone());
            let controller = TestController::new(model);
            let view = TestView;

            println!("{:?}", controller.render_view(&view));
        }
    }

    Ok(())
}
