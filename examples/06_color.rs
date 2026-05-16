use coreimage::prelude::*;

fn main() {
    let color = CIColor::named(CIColorName::Magenta);
    println!(
        "magenta rgba = {:.1}, {:.1}, {:.1}, {:.1}",
        color.red(),
        color.green(),
        color.blue(),
        color.alpha()
    );
}
