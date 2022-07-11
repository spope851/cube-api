#[derive(Copy, Clone)]
pub struct Cube {
    pub top_tl: i32,
    pub top_tm: i32,
    pub top_tr: i32,
    pub top_ml: i32,
    pub top_mm: i32,
    pub top_mr: i32,
    pub top_bl: i32,
    pub top_bm: i32,
    pub top_br: i32,
    pub lef_tl: i32,
    pub lef_tm: i32,
    pub lef_tr: i32,
    pub lef_ml: i32,
    pub lef_mm: i32,
    pub lef_mr: i32,
    pub lef_bl: i32,
    pub lef_bm: i32,
    pub lef_br: i32,
    pub fro_tl: i32,
    pub fro_tm: i32,
    pub fro_tr: i32,
    pub fro_ml: i32,
    pub fro_mm: i32,
    pub fro_mr: i32,
    pub fro_bl: i32,
    pub fro_bm: i32,
    pub fro_br: i32,
    pub rig_tl: i32,
    pub rig_tm: i32,
    pub rig_tr: i32,
    pub rig_ml: i32,
    pub rig_mm: i32,
    pub rig_mr: i32,
    pub rig_bl: i32,
    pub rig_bm: i32,
    pub rig_br: i32,
    pub bac_tl: i32,
    pub bac_tm: i32,
    pub bac_tr: i32,
    pub bac_ml: i32,
    pub bac_mm: i32,
    pub bac_mr: i32,
    pub bac_bl: i32,
    pub bac_bm: i32,
    pub bac_br: i32,
    pub bot_tl: i32,
    pub bot_tm: i32,
    pub bot_tr: i32,
    pub bot_ml: i32,
    pub bot_mm: i32,
    pub bot_mr: i32,
    pub bot_bl: i32,
    pub bot_bm: i32,
    pub bot_br: i32,
} 

impl Cube {
    pub fn r(&mut self) {
        let rig_tm = self.rig_tm.clone();
        self.rig_tm = self.rig_ml.clone();
        self.rig_ml = self.rig_bm.clone();
        self.rig_bm = self.rig_mr.clone();
        self.rig_mr = rig_tm;
        let top_mr = self.top_mr.clone();
        self.top_mr = self.fro_mr.clone();
        self.fro_mr = self.bot_mr.clone();
        self.bot_mr = self.bac_ml.clone();
        self.bac_ml = top_mr;
    }
    pub fn rz(&mut self) {
        let rig_tm = self.rig_tm.clone();
        self.rig_tm = self.rig_mr.clone();
        self.rig_mr = self.rig_bm.clone();
        self.rig_bm = self.rig_ml.clone();
        self.rig_ml = rig_tm;
        let top_mr = self.top_mr.clone();
        self.top_mr = self.bac_ml.clone();
        self.bac_ml = self.bot_mr.clone();
        self.bot_mr = self.fro_mr.clone();
        self.fro_mr = top_mr;
    } 
    pub fn l(&mut self) {
        let lef_tm = self.lef_tm.clone();
        self.lef_tm = self.lef_ml.clone();
        self.lef_ml = self.lef_bm.clone();
        self.lef_bm = self.lef_mr.clone();
        self.lef_mr = lef_tm;
        let top_ml = self.top_ml.clone();
        self.top_ml = self.bac_mr.clone();
        self.bac_mr = self.bot_ml.clone();
        self.bot_ml = self.fro_ml.clone();
        self.fro_ml = top_ml;
    }
    pub fn lz(&mut self) {
        let lef_tm = self.lef_tm.clone();
        self.lef_tm = self.lef_mr.clone();
        self.lef_mr = self.lef_bm.clone();
        self.lef_bm = self.lef_ml.clone();
        self.lef_ml = lef_tm;
        let top_ml = self.top_ml.clone();
        self.top_ml = self.fro_ml.clone();
        self.fro_ml = self.bot_ml.clone();
        self.bot_ml = self.bac_mr.clone();
        self.bac_mr = top_ml;
    }
    pub fn f(&mut self) {
        let fro_tm = self.fro_tm.clone();
        self.fro_tm = self.fro_ml.clone();
        self.fro_ml = self.fro_bm.clone();
        self.fro_bm = self.fro_mr.clone();
        self.fro_mr = fro_tm;
        let top_bm = self.top_bm.clone();
        self.top_bm = self.lef_mr.clone();
        self.lef_mr = self.bot_tm.clone();
        self.bot_tm = self.rig_ml.clone();
        self.rig_ml = top_bm;
    }
    pub fn fz(&mut self) {
        let fro_tm = self.fro_tm.clone();
        self.fro_tm = self.fro_mr.clone();
        self.fro_mr = self.fro_bm.clone();
        self.fro_bm = self.fro_ml.clone();
        self.fro_ml = fro_tm;
        let top_bm = self.top_bm.clone();
        self.top_bm = self.rig_ml.clone();
        self.rig_ml = self.bot_tm.clone();
        self.bot_tm = self.lef_mr.clone();
        self.lef_mr = top_bm;
    }
    pub fn t(&mut self) {
        let top_tm = self.top_tm.clone();
        self.top_tm = self.top_ml.clone();
        self.top_ml = self.top_bm.clone();
        self.top_bm = self.top_mr.clone();
        self.top_mr = top_tm;
        let bac_tm = self.bac_tm.clone();
        self.bac_tm = self.lef_tm.clone();
        self.lef_tm = self.fro_tm.clone();
        self.fro_tm = self.rig_ml.clone();
        self.rig_ml = bac_tm;
    }
    pub fn tz(&mut self) {
        let top_tm = self.top_tm.clone();
        self.top_tm = self.top_mr.clone();
        self.top_mr = self.top_bm.clone();
        self.top_bm = self.top_ml.clone();
        self.top_ml = top_tm;
        let bac_tm = self.bac_tm.clone();
        self.bac_tm = self.rig_tm.clone();
        self.rig_ml = self.fro_tm.clone();
        self.fro_tm = self.lef_tm.clone();
        self.lef_tm = bac_tm;
    }
    pub fn b(&mut self) {
        let bac_tm = self.bac_tm.clone();
        self.bac_tm = self.bac_ml.clone();
        self.bac_ml = self.bac_bm.clone();
        self.bac_bm = self.bac_mr.clone();
        self.bac_mr = bac_tm;
        let top_tm = self.top_tm.clone();
        self.top_tm = self.rig_mr.clone();
        self.rig_mr = self.bot_bm.clone();
        self.bot_bm = self.lef_ml.clone();
        self.lef_ml = top_tm;
    }
    pub fn bz(&mut self) {
        let bac_tm = self.bac_tm.clone();
        self.bac_tm = self.bac_mr.clone();
        self.bac_mr = self.bac_bm.clone();
        self.bac_bm = self.bac_ml.clone();
        self.bac_ml = bac_tm;
        let top_tm = self.top_tm.clone();
        self.top_tm = self.lef_ml.clone();
        self.lef_ml = self.bot_bm.clone();
        self.bot_bm = self.rig_mr.clone();
        self.rig_mr = top_tm;
    }
    pub fn m(&mut self) {
        let bot_tm = self.bot_tm.clone();
        self.bot_tm = self.bot_ml.clone();
        self.bot_ml = self.bot_bm.clone();
        self.bot_bm = self.bot_mr.clone();
        self.bot_mr = bot_tm;
        let fro_bm = self.fro_bm.clone();
        self.fro_bm = self.lef_bm.clone();
        self.lef_bm = self.bac_bm.clone();
        self.bac_bm = self.rig_bm.clone();
        self.rig_bm = fro_bm;
    }
    pub fn mz(&mut self) {
        let bot_tm = self.bot_tm.clone();
        self.bot_tm = self.bot_mr.clone();
        self.bot_mr = self.bot_bm.clone();
        self.bot_bm = self.bot_ml.clone();
        self.bot_ml = bot_tm;
        let fro_bm = self.fro_bm.clone();
        self.rig_bm = self.rig_bm.clone();
        self.rig_bm = self.bac_bm.clone();
        self.bac_bm = self.lef_bm.clone();
        self.lef_bm = fro_bm;
    }
}
