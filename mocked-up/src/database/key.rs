mod alternate;
mod composite;
mod foreign;
mod primary;

pub use alternate::Alternate;
pub use composite::Composite;
pub use foreign::Foreign;
pub use primary::Primary;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum IdentKey {
    Primary,
    Composite,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Key {
    Alternate,
    Foreign,
}
