use std::env;
mod solve_edge;
use solve_edge::solve_edge;
use solve_edge::cube::Cube;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cube = Cube {
        top_tl: args[1].split(':').nth(1).unwrap().parse().unwrap(),
        top_tm: args[2].split(':').nth(1).unwrap().parse().unwrap(),
        top_tr: args[3].split(':').nth(1).unwrap().parse().unwrap(),
        top_ml: args[4].split(':').nth(1).unwrap().parse().unwrap(),
        top_mm: args[5].split(':').nth(1).unwrap().parse().unwrap(),
        top_mr: args[6].split(':').nth(1).unwrap().parse().unwrap(),
        top_bl: args[7].split(':').nth(1).unwrap().parse().unwrap(),
        top_bm: args[8].split(':').nth(1).unwrap().parse().unwrap(),
        top_br: args[9].split(':').nth(1).unwrap().parse().unwrap(),
        lef_tl: args[10].split(':').nth(1).unwrap().parse().unwrap(),
        lef_tm: args[11].split(':').nth(1).unwrap().parse().unwrap(),
        lef_tr: args[12].split(':').nth(1).unwrap().parse().unwrap(),
        lef_ml: args[13].split(':').nth(1).unwrap().parse().unwrap(),
        lef_mm: args[14].split(':').nth(1).unwrap().parse().unwrap(),
        lef_mr: args[15].split(':').nth(1).unwrap().parse().unwrap(),
        lef_bl: args[16].split(':').nth(1).unwrap().parse().unwrap(),
        lef_bm: args[17].split(':').nth(1).unwrap().parse().unwrap(),
        lef_br: args[18].split(':').nth(1).unwrap().parse().unwrap(),
        fro_tl: args[19].split(':').nth(1).unwrap().parse().unwrap(),
        fro_tm: args[20].split(':').nth(1).unwrap().parse().unwrap(),
        fro_tr: args[21].split(':').nth(1).unwrap().parse().unwrap(),
        fro_ml: args[22].split(':').nth(1).unwrap().parse().unwrap(),
        fro_mm: args[23].split(':').nth(1).unwrap().parse().unwrap(),
        fro_mr: args[24].split(':').nth(1).unwrap().parse().unwrap(),
        fro_bl: args[25].split(':').nth(1).unwrap().parse().unwrap(),
        fro_bm: args[26].split(':').nth(1).unwrap().parse().unwrap(),
        fro_br: args[27].split(':').nth(1).unwrap().parse().unwrap(),
        rig_tl: args[28].split(':').nth(1).unwrap().parse().unwrap(),
        rig_tm: args[29].split(':').nth(1).unwrap().parse().unwrap(),
        rig_tr: args[30].split(':').nth(1).unwrap().parse().unwrap(),
        rig_ml: args[31].split(':').nth(1).unwrap().parse().unwrap(),
        rig_mm: args[32].split(':').nth(1).unwrap().parse().unwrap(),
        rig_mr: args[33].split(':').nth(1).unwrap().parse().unwrap(),
        rig_bl: args[34].split(':').nth(1).unwrap().parse().unwrap(),
        rig_bm: args[35].split(':').nth(1).unwrap().parse().unwrap(),
        rig_br: args[36].split(':').nth(1).unwrap().parse().unwrap(),
        bac_tl: args[37].split(':').nth(1).unwrap().parse().unwrap(),
        bac_tm: args[38].split(':').nth(1).unwrap().parse().unwrap(),
        bac_tr: args[39].split(':').nth(1).unwrap().parse().unwrap(),
        bac_ml: args[40].split(':').nth(1).unwrap().parse().unwrap(),
        bac_mm: args[41].split(':').nth(1).unwrap().parse().unwrap(),
        bac_mr: args[42].split(':').nth(1).unwrap().parse().unwrap(),
        bac_bl: args[43].split(':').nth(1).unwrap().parse().unwrap(),
        bac_bm: args[44].split(':').nth(1).unwrap().parse().unwrap(),
        bac_br: args[45].split(':').nth(1).unwrap().parse().unwrap(),
        bot_tl: args[46].split(':').nth(1).unwrap().parse().unwrap(),
        bot_tm: args[47].split(':').nth(1).unwrap().parse().unwrap(),
        bot_tr: args[48].split(':').nth(1).unwrap().parse().unwrap(),
        bot_ml: args[49].split(':').nth(1).unwrap().parse().unwrap(),
        bot_mm: args[50].split(':').nth(1).unwrap().parse().unwrap(),
        bot_mr: args[51].split(':').nth(1).unwrap().parse().unwrap(),
        bot_bl: args[52].split(':').nth(1).unwrap().parse().unwrap(),
        bot_bm: args[53].split(':').nth(1).unwrap().parse().unwrap(),
        bot_br: args[54].split(':').nth(1).unwrap().parse().unwrap(),
    };

    // cube.b();

    #[derive(PartialEq)]#[derive(Copy, Clone)]
    struct TopFro {
        top: i32,
        fro: i32,
    }

    #[derive(PartialEq)]#[derive(Copy, Clone)]
    struct TopLef {
        top: i32,
        lef: i32,
    }

    #[derive(PartialEq)]#[derive(Copy, Clone)]
    struct TopBac {
        top: i32,
        bac: i32,
    }

    #[derive(PartialEq)]#[derive(Copy, Clone)]
    struct TopRig {
        top: i32,
        rig: i32,
    }

    struct BotFro {
        bot: i32,
        fro: i32,
    }

    struct BotLef {
        bot: i32,
        lef: i32,
    }

    struct BotBac {
        bot: i32,
        bac: i32,
    }

    struct BotRig {
        bot: i32,
        rig: i32,
    }

    struct MidFroLef {
        fro: i32,
        lef: i32,
    }

    struct MidBacLef {
        bac: i32,
        lef: i32,
    }

    struct MidBacRig {
        bac: i32,
        rig: i32,
    }

    struct MidFroRig {
        fro: i32,
        rig: i32,
    }

    struct Edges {
        top_fro: TopFro,
        top_lef: TopLef,
        top_bac: TopBac,
        top_rig: TopRig,
        bot_fro: BotFro,
        bot_lef: BotLef,
        bot_bac: BotBac,
        bot_rig: BotRig,
        mid_fro_lef: MidFroLef,
        mid_bac_lef: MidBacLef,
        mid_bac_rig: MidBacRig,
        mid_fro_rig: MidFroRig,
    }

    struct EdgeColors {
        top_fro: [i32; 2],
        top_lef: [i32; 2],
        top_bac: [i32; 2],
        top_rig: [i32; 2],
        bot_fro: [i32; 2],
        bot_lef: [i32; 2],
        bot_bac: [i32; 2],
        bot_rig: [i32; 2],
        mid_fro_lef: [i32; 2],
        mid_bac_lef: [i32; 2],
        mid_bac_rig: [i32; 2],
        mid_fro_rig: [i32; 2],
    }

    // impl Edges {
    //     pub fn r(&mut self) {
    //         let top_rig = self.top_rig;
    //         self.top_rig = self.mid_fro_rig;
    //         self.mid_fro_rig = self.bot_rig;
    //         self.bot_rig = self.mid_bac_rig;
    //         self.mid_bac_rig = top_rig;
    //     }
    // }

    let edges = Edges {
        top_fro: TopFro {
            top: cube.top_bm.clone(),
            fro: cube.fro_tm.clone(),
        },
        top_rig: TopRig {
            top: cube.top_mr.clone(),
            rig: cube.rig_tm.clone(),
        },
        top_lef: TopLef {
            top: cube.top_ml.clone(),
            lef: cube.lef_tm.clone(),
        },
        top_bac: TopBac {
            top: cube.top_tm.clone(),
            bac: cube.bac_tm.clone(),
        },
        bot_fro: BotFro {
            bot: cube.bot_tm.clone(),
            fro: cube.fro_bm.clone(),
        },
        bot_rig: BotRig {
            bot: cube.bot_mr.clone(),
            rig: cube.rig_bm.clone(),
        },
        bot_lef: BotLef {
            bot: cube.bot_ml.clone(),
            lef: cube.lef_bm.clone(),
        },
        bot_bac: BotBac {
            bot: cube.bot_bm.clone(),
            bac: cube.bac_bm.clone(),
        },
        mid_fro_lef: MidFroLef {
            fro: cube.fro_ml.clone(),
            lef: cube.lef_mr.clone(),
        },
        mid_bac_lef: MidBacLef {
            bac: cube.bac_mr.clone(),
            lef: cube.lef_ml.clone(),
        },
        mid_bac_rig: MidBacRig {
            bac: cube.bac_ml.clone(),
            rig: cube.rig_mr.clone(),
        },
        mid_fro_rig: MidFroRig {
            fro: cube.fro_mr.clone(),
            rig: cube.rig_ml.clone(),
        },
    };

    let edge_colors = EdgeColors {
        top_fro: [
            cube.top_bm.clone(),
            cube.fro_tm.clone(),
        ],
        top_rig: [
            cube.top_mr.clone(),
            cube.rig_tm.clone(),
        ],
        top_lef: [
            cube.top_ml.clone(),
            cube.lef_tm.clone(),
        ],
        top_bac: [
            cube.top_tm.clone(),
            cube.bac_tm.clone(),
        ],
        bot_fro: [
            cube.bot_tm.clone(),
            cube.fro_bm.clone(),
        ],
        bot_rig: [
            cube.bot_mr.clone(),
            cube.rig_bm.clone(),
        ],
        bot_lef: [
            cube.bot_ml.clone(),
            cube.lef_bm.clone(),
        ],
        bot_bac: [
            cube.bot_bm.clone(),
            cube.bac_bm.clone(),
        ],
        mid_fro_lef: [
            cube.fro_ml.clone(),
            cube.lef_mr.clone(),
        ],
        mid_bac_lef: [
            cube.bac_mr.clone(),
            cube.lef_ml.clone(),
        ],
        mid_bac_rig: [
            cube.bac_ml.clone(),
            cube.rig_mr.clone(),
        ],
        mid_fro_rig: [
            cube.fro_mr.clone(),
            cube.rig_ml.clone(),
        ],
    };

    let solved_edges = Edges {
        top_fro: TopFro {
            top: 1,
            fro: 2,
        },
        top_rig: TopRig {
            top: 1,
            rig: 5,
        },
        top_lef: TopLef {
            top: 1,
            lef: 3,
        },
        top_bac: TopBac {
            top: 1,
            bac: 4,
        },
        bot_fro: BotFro {
            bot: 6,
            fro: 2,
        },
        bot_rig: BotRig {
            bot: 6,
            rig: 5,
        },
        bot_lef: BotLef {
            bot: 6,
            lef: 3,
        },
        bot_bac: BotBac {
            bot: 6,
            bac: 4,
        },
        mid_fro_lef: MidFroLef {
            fro: 2,
            lef: 3,
        },
        mid_bac_lef: MidBacLef {
            bac: 4,
            lef: 3,
        },
        mid_bac_rig: MidBacRig {
            bac: 4,
            rig: 5,
        },
        mid_fro_rig: MidFroRig {
            fro: 2,
            rig: 5,
        },
    };

    #[derive(PartialEq)]
    struct WhiteCross {
        fro: TopFro,
        lef: TopLef,
        bac: TopBac,
        rig: TopRig,
    }

    let top_cross = WhiteCross {
        fro: edges.top_fro,
        lef: edges.top_lef,
        bac: edges.top_bac,
        rig: edges.top_rig,
    };

    let solved_cross = WhiteCross {
        fro: solved_edges.top_fro,
        lef: solved_edges.top_lef,
        bac: solved_edges.top_bac,
        rig: solved_edges.top_rig,
    };

    let solved_edge_colors: [[i32; 2]; 12] = [
        [solved_edges.top_fro.top, solved_edges.top_fro.fro],
        [solved_edges.top_lef.top, solved_edges.top_lef.lef],
        [solved_edges.top_bac.top, solved_edges.top_bac.bac],
        [solved_edges.top_rig.top, solved_edges.top_rig.rig],
        [solved_edges.mid_fro_lef.fro, solved_edges.mid_fro_lef.lef],
        [solved_edges.mid_bac_lef.bac, solved_edges.mid_bac_lef.lef],
        [solved_edges.mid_bac_rig.bac, solved_edges.mid_bac_rig.rig],
        [solved_edges.mid_fro_rig.fro, solved_edges.mid_fro_rig.rig],
        [solved_edges.bot_fro.bot, solved_edges.bot_fro.fro],
        [solved_edges.bot_lef.bot, solved_edges.bot_lef.lef],
        [solved_edges.bot_bac.bot, solved_edges.bot_bac.bac],
        [solved_edges.bot_rig.bot, solved_edges.bot_rig.rig],
    ];

    let solve_white_cross = || {

        println!("fro top: {:?}", top_cross.fro.top);
        println!("lef top: {:?}", top_cross.lef.top);
        println!("bac top: {:?}", top_cross.bac.top);
        println!("rig top: {:?}", top_cross.rig.top);

        println!("fro side: {:?}", top_cross.fro.fro);
        println!("lef side: {:?}", top_cross.lef.lef);
        println!("bac side: {:?}", top_cross.bac.bac);
        println!("rig side: {:?}", top_cross.rig.rig);

        for (i, el) in [
            edge_colors.top_fro,
            edge_colors.top_lef,
            edge_colors.top_bac,
            edge_colors.top_rig,
            edge_colors.mid_fro_lef,
            edge_colors.mid_bac_lef,
            edge_colors.mid_bac_rig,
            edge_colors.mid_fro_rig,
            edge_colors.bot_fro,
            edge_colors.bot_lef,
            edge_colors.bot_bac,
            edge_colors.bot_rig
        ].iter().enumerate() {
            // write some logic that considers each position of a white piece and puts it in place according to that position. find commonalities
            if el.contains(&1) && el[0] != solved_edge_colors[i][0] && el[1] != solved_edge_colors[i][1] {
                solve_edge(i, el, cube);
                break;
            }
        }
        if top_cross == solved_cross { print!("solved"); } else { print!("not solved") }
    };
    
    solve_white_cross();

    // for arg in &args[1..55] {
    //     println!("{:?}", arg.split(':').nth(1).unwrap());
    // }
    
}

// solved
// cargo run {top-tl:1,top-tm:1,top-tr:1,top-ml:1,top-mm:1,top-mr:1,top-bl:1,top-bm:1,top-br:1,lef-tl:3,lef-tm:3,lef-tr:3,lef-ml:3,lef-mm:3,lef-mr:3,lef-bl:3,lef-bm:3,lef-br:3,fro-tl:2,fro-tm:2,fro-tr:2,fro-ml:2,fro-mm:2,fro-mr:2,fro-bl:2,fro-bm:2,fro-br:2,rig-tl:5,rig-tm:5,rig-tr:5,rig-ml:5,rig-mm:5,rig-mr:5,rig-bl:5,rig-bm:5,rig-br:5,bac-tl:4,bac-tm:4,bac-tr:4,bac-ml:4,bac-mm:4,bac-mr:4,bac-bl:4,bac-bm:4,bac-br:4,bot-tl:6,bot-tm:6,bot-tr:6,bot-ml:6,bot-mm:6,bot-mr:6,bot-bl:6,bot-bm:6,bot-br:6}
// not solved
// cargo run {top-tl:5,top-tm:6,top-tr:6,top-ml:4,top-mm:1,top-mr:6,top-bl:4,top-bm:3,top-br:6,lef-tl:6,lef-tm:6,lef-tr:6,lef-ml:1,lef-mm:3,lef-mr:5,lef-bl:1,lef-bm:4,lef-br:5,fro-tl:3,fro-tm:1,fro-tr:4,fro-ml:2,fro-mm:2,fro-mr:6,fro-bl:2,fro-bm:2,fro-br:3,rig-tl:5,rig-tm:5,rig-tr:2,rig-ml:3,rig-mm:5,rig-mr:4,rig-bl:4,rig-bm:4,rig-br:1,bac-tl:3,bac-tm:2,bac-tr:2,bac-ml:1,bac-mm:4,bac-mr:5,bac-bl:2,bac-bm:2,bac-br:5,bot-tl:1,bot-tm:1,bot-tr:1,bot-ml:5,bot-mm:6,bot-mr:3,bot-bl:4,bot-bm:3,bot-br:3}