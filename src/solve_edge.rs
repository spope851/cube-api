#[path = "cube.rs"] pub mod cube;
use cube::Cube;

pub fn solve_edge (index: usize, colors: &[i32; 2], mut cube: Cube) {
    println!("solving {:?} {:?}!", index, colors);
    if index == 0 {
        if colors[1] == 1 {
            println!("F' L'");
            cube.fz();
            cube.lz();
        }
    }
    if index == 5 {
        if colors[1] == 1 {
            println!("F' L'");
            cube.fz();
            cube.lz();
        }
    }
}

// 0 top_fro
// 1 top_lef
// 2 top_bac
// 3 top_rig
// 4 mid_fro_lef
// 5 mid_bac_lef
// 6 mid_bac_rig
// 7 mid_fro_rig
// 8 bot_fro
// 9 bot_lef
// 11 bot_bac
// 12 bot_rig
