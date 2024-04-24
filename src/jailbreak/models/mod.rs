pub mod dupers;
pub mod jbtc;
pub mod jbtr;
pub use jbtc::JBTC;
pub use jbtr::JBTR;

pub fn update() {
    jbtc::parse().unwrap();
    jbtr::parse().unwrap();
    dupers::parse().unwrap();
}
