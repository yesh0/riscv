use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!(
    "Vstvec
517
base,63,2,number,
mode,1,0,number,
end
Virtual Supervisor Trap Vector Base Address Register."
);
