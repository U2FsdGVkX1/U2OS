pub mod status;
pub mod ie;
pub mod ret;

macro_rules! w_reg {
    ($name: expr, $value: expr) => {
        {
            asm!(concat!("csrw ", $name, ",{0}"), in(reg) ($value));
        }
    };
}

macro_rules! r_reg {
    ($name: expr) => {
        {
            let t: usize;
            asm!(concat!("csrr {0},", $name), out(reg) t);
            t
        }
    };
}
