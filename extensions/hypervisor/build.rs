extern crate cc;
use std::io::Write;
const HFENCE_GVMA_TEMPLATE: u32 = 0b0110001_00000_00000_000_00000_1110011;
const HFENCE_VVMA_TEMPLATE: u32 = 0b0010001_00000_00000_000_00000_1110011;
const HLV_B_TEMPLATE: u32 = 0b0110000_00000_00000_100_00000_1110011;
const HLV_BU_TEMPLATE: u32 = 0b0110000_00001_00000_100_00000_1110011;
const HLV_H_TEMPLATE: u32 = 0b0110010_00000_00000_100_00000_1110011;
const HLV_HU_TEMPLATE: u32 = 0b0110010_00001_00000_100_00000_1110011;
const HLVX_HU_TEMPLATE: u32 = 0b0110010_00011_00000_100_00000_1110011;
const HLV_W_TEMPLATE: u32 = 0b0110100_00000_00000_100_00000_1110011;
const HLVX_WU_TEMPLATE: u32 = 0b0110100_00011_00000_100_00000_1110011;
const HSV_B_TEMPLATE: u32 = 0b0110001_00000_00000_100_00000_1110011;
const HSV_H_TEMPLATE: u32 = 0b0110011_00000_00000_100_00000_1110011;
const HSV_W_TEMPLATE: u32 = 0b0110101_00000_00000_100_00000_1110011;
const HLV_WU_TEMPLATE: u32 = 0b0110100_00001_00000_100_00000_1110011;
const HLV_D_TEMPLATE: u32 = 0b0110110_00000_00000_100_00000_1110011;
const HSV_D_TEMPLATE: u32 = 0b0110111_00000_00000_100_00000_1110011;
#[derive(Copy, Clone, Debug)]
enum Register{
    x0 = 0,
    a0 = 10,
    a1 = 11
}
fn template_rs1_rs2(template: u32, rs1: Register, rs2: Register) -> (u32, bool){
    (((rs2 as u32)<<20) | ((rs1 as u32)<<15) | template, false)
}
fn template_rs1_rd(template: u32, rs1: Register, rd: Register) -> (u32, bool){
    (((rs1 as u32) << 15) | ((rd as u32) << 7) | template, true)
}
fn emit_instruction<T: std::io::Write>(writer: &mut T, writer_rust: &mut T, name: &str, insn: (u32, bool))->std::io::Result<()>{
    write!(writer, ".global invoke_insn_{}\ninvoke_insn_{}:\n    .word {}\n    ret\n", name, name, insn.0)?;
    write!(writer_rust, "    pub fn invoke_insn_{}(rs1: usize, {}: usize);\n", name, if insn.1 {"rd"} else {"rs2"})
}

// Generate instructions on the fly.
fn main() {
    let mut file = std::fs::File::create("asm.S").unwrap();
    let mut file_rs = std::fs::File::create("src/asm.rs").unwrap();
    write!(&mut file_rs, "extern \"C\" {{\n").unwrap();
    macro_rules! gen{
        (rs2, $name: expr, $template: ident)=>{
            emit_instruction(&mut file, &mut file_rs, $name, template_rs1_rs2($template, Register::a0, Register::a1)).unwrap();
        };
        (rd, $name: expr, $template: ident)=>{
            emit_instruction(&mut file, &mut file_rs, $name, template_rs1_rd($template, Register::a0, Register::a0)).unwrap();
        };
    }
    gen!(rs2, "hfence_gvma", HFENCE_GVMA_TEMPLATE);
    gen!(rs2, "hfence_vvma", HFENCE_VVMA_TEMPLATE);
    gen!(rs2, "hlv_b", HLV_B_TEMPLATE);
    gen!(rs2, "hlv_bu", HLV_BU_TEMPLATE);
    gen!(rs2, "hlv_h", HLV_H_TEMPLATE);
    gen!(rs2, "hlv_hu", HLV_HU_TEMPLATE);
    gen!(rs2, "hlvx_hu", HLVX_HU_TEMPLATE);
    gen!(rs2, "hlv_w", HLV_W_TEMPLATE);
    gen!(rs2, "hlvx_wu", HLVX_WU_TEMPLATE);
    gen!(rd, "hsv_b", HSV_B_TEMPLATE);
    gen!(rd, "hsv_h", HSV_H_TEMPLATE);
    gen!(rd, "hsv_w", HSV_W_TEMPLATE);
    gen!(rs2, "hlv_wu", HLV_WU_TEMPLATE);
    gen!(rs2, "hlv_d", HLV_D_TEMPLATE);
    gen!(rd, "hsv_d", HSV_D_TEMPLATE);

    write!(&mut file_rs, 
"}}
global_asm!(include_str!(\"../asm.S\"));
").unwrap();
    drop(file);
    drop(file_rs);
    /*
    let mut c = cc::Build::new();
    c.target("riscv64imac-unknown-none-elf");
    c.file("asm.S");
    println!("{:?}", c.get_compiler());
    c.compile("librvhasm.a");
    */
}