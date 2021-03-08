trait BoolExt {
    fn from_usize(x: usize) -> Self;
}

impl BoolExt for bool {
    #[inline]
    fn from_usize(x: usize) -> Self {
        if x == 0 {
            return false;
        }
        if x == 1 {
            return true;
        }
        unreachable!();
    }
}

impl BoolExt for usize {
    #[inline]
    fn from_usize(x: usize) -> Self {
        x
    }
}

pub mod hcounteren;
pub mod hedeleg;
pub mod hgatp;
pub mod hgeie;
pub mod hgeip;
pub mod hideleg;
pub mod hie;
pub mod hip;
pub mod hstatus;
pub mod htimedelta;
pub mod htimedeltah;
pub mod htinst;
pub mod htval;
pub mod hvip;
pub mod vsatp;
pub mod vscause;
pub mod vsepc;
pub mod vsie;
pub mod vsip;
pub mod vsscratch;
pub mod vsstatus;
pub mod vstval;
pub mod vstvec;
