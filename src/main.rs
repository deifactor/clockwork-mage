#[macro_use]
extern crate clap;
mod action;
mod player;
mod rotation;
mod simulator;
mod target;
mod time;

use crate::action::Action;
use sloggers::{terminal, Build};
use structopt::StructOpt;

arg_enum! {
    #[derive(Clone, Copy, Debug)]
    enum RotationArg {
        Empty,
        HitRecharge,
        EagerHit
    }
}

impl From<RotationArg> for Box<dyn rotation::Rotation> {
    fn from(arg: RotationArg) -> Self {
        match arg {
            RotationArg::Empty => Box::new(rotation::Empty {}),
            RotationArg::HitRecharge => {
                Box::new(rotation::Repeat::new(vec![Action::Hit, Action::Recharge]))
            }
            RotationArg::EagerHit => Box::new(rotation::EagerHit {}),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clockwork-mage")]
struct Opt {
    /// Rotation to use.
    #[structopt(
        short = "r",
        long = "rotation",
        raw(
            possible_values = "&RotationArg::variants()",
            case_insensitive = "true"
        )
    )]
    rotation: RotationArg,

    /// Number of centiseconds to simulate for.
    #[structopt(short = "d", long = "duration", default_value = "100000")]
    duration: i32,

    /// If true, logs debug messages as well.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();
    let rotation: Box<dyn rotation::Rotation> = opt.rotation.into();
    let mut builder = terminal::TerminalLoggerBuilder::new();
    if opt.verbose {
        builder.level(sloggers::types::Severity::Debug);
    }
    let logger = builder.build().expect("failed to build logger");
    let mut simulator = simulator::Simulator::new(rotation, logger);
    while simulator.now().0 <= opt.duration {
        simulator.step()
    }
}
