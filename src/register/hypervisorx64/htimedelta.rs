//! Hypervisor Time Delta Register.
read_composite_csr!(super::htimedeltah::read(), read());
read_csr_as_usize!(1541);
write_csr_as_usize!(1541);
