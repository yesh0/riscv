use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!("Vscause
578
interrupt,63,63,number,Is cause interrupt.
code,62,0,number,Exception code
end
Virtual Supervisor Cause Register.");