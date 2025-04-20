use lxp_common::pool_definition::PoolDefinition;
use tabled::{settings::{object::{Columns, Rows}, Alignment, Color, Style}, Table, Tabled};

#[derive(Debug, Tabled)]
struct PoolDefinitionTableEntry {
    #[tabled(rename = "Name")]
    name: String,

    #[tabled(rename = "Live Count")]
    live_count: u32,

    #[tabled(rename = "Pool Size")]
    pool_size: u32,

    #[tabled(rename = "Operating System Base")]
    os_base: String,
}

impl From<PoolDefinition> for PoolDefinitionTableEntry {
    fn from(pool_definition: PoolDefinition) -> Self {
        PoolDefinitionTableEntry {
            name:       pool_definition.name,
            live_count: pool_definition.live_count,
            pool_size:  pool_definition.pool_size,
            os_base:    pool_definition.base,
        }
    }
}

pub fn pool_definitions_as_table(pool_definitions: Vec<PoolDefinition>) -> String {
    let table_data: Vec<PoolDefinitionTableEntry> = pool_definitions.iter()
        .map(|pool_definition| pool_definition.clone().into())
        .collect();

    let mut table: Table = Table::new(table_data);

    table.with(Style::modern_rounded());
    table.modify(Rows::first(), Color::FG_BRIGHT_WHITE);
    table.modify(Columns::single(1), Alignment::right());
    table.modify(Columns::single(2), Alignment::right());

    table.to_string()
}
