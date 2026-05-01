mod axe;
mod normal;
mod phantom;
mod special;

pub fn install() {
    axe::install();
    normal::install();
    phantom::install();
    special::install();
}