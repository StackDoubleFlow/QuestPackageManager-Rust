use clap::{Clap, AppSettings};
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Qmod {
    #[clap(subcommand)]
    pub op: QmodOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum QmodOperation {
    Build,
    Edit,

}

pub fn execute_qmod_operation(operation: Qmod)
{
    match operation.op.clone() {
        QmodOperation::Build => execute_qmod_build(),
        QmodOperation::Edit => execute_qmod_edit(),
    }
}

fn execute_qmod_build()
{

}

fn execute_qmod_edit()
{

}