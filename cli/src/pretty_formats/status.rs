use lxp_common::lxp_status::{LxpMachineStatus, LxpPoolStatus};
use tabled::{settings::{object::Rows, Color, Style}, Table};

pub fn display_pools(pools: Vec<LxpPoolStatus>) {
    let mut table: Table = Table::new(pools);

    table.with(Style::empty());
    table.modify(Rows::first(), Color::FG_BRIGHT_WHITE);

    println!("{}", table.to_string());
}

pub fn display_units(units: Vec<LxpMachineStatus>) {
    let mut table: Table = Table::new(units);

    table.with(Style::empty());
    table.modify(Rows::first(), Color::FG_BRIGHT_WHITE);

    println!("{}", table.to_string());
}
