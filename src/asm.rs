#[link(name = "rvhasm")]
extern {
    pub fn invoke_insn_hfence_gvma(rs1: usize, rs2: usize);
    pub fn invoke_insn_hfence_vvma(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_b(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_bu(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_h(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_hu(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlvx_hu(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_w(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlvx_wu(rs1: usize, rs2: usize);
    pub fn invoke_insn_hsv_b(rs1: usize, rd: usize);
    pub fn invoke_insn_hsv_h(rs1: usize, rd: usize);
    pub fn invoke_insn_hsv_w(rs1: usize, rd: usize);
    pub fn invoke_insn_hlv_wu(rs1: usize, rs2: usize);
    pub fn invoke_insn_hlv_d(rs1: usize, rs2: usize);
    pub fn invoke_insn_hsv_d(rs1: usize, rd: usize);
}
