extern crate foundry;

use foundry::Grid;
use foundry::view::View;

use std::fs;

#[test]
fn test_render() {
    let grid = Grid::from_file("tests/input_files/view_rendering.life").unwrap();

    let view = View::new(&grid);
    println!("Grid:\n{}", grid);

    let rendering = view.render(grid.get_width() * 2, grid.get_height() * 2);
    println!("Rendering:\n{}", format_rendering(&rendering, &grid));

    panic!();
}

fn format_rendering(r: &Vec<u8>, g: &Grid) -> String {
    let mut ret = String::new();
    let mut idx = 0;

    for _ in 0..g.get_height() * 2 {
        for _ in 0..g.get_width() * 2 {
            if r[idx] != 0 {
                ret.push_str("*");
            } else {
                ret.push_str(".");
            }

            idx += 1;
        }

        ret.push_str("\n");
    }

    ret
}
