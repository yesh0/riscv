trait BoolExt{
    fn from_usize(x: usize)->Self;
}

impl BoolExt for bool{
    #[inline]
    fn from_usize(x: usize)->Self{
        if x==0{
            return false;
        }
        if x==1{
            return true;
        }
        unreachable!();
    }
}

impl BoolExt for usize{
    #[inline]
    fn from_usize(x: usize)->Self{
        x
    }
}

pub mod hstatus;
pub mod hedeleg;
pub mod hideleg;
pub mod hvip;
pub mod hip;
pub mod hie;
pub mod hgeip;
pub mod hgeie;
pub mod hcounteren;
pub mod htimedelta;
pub mod htimedeltah;
pub mod htval;
pub mod htinst;
pub mod hgatp;
pub mod vsstatus;
pub mod vsip;
pub mod vsie;
pub mod vstvec;
pub mod vscause;
pub mod vstval;
pub mod vsatp;