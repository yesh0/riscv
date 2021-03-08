use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!(
    "Vsie
516
ssie,1,1,number,Software Interrupt
stie,5,5,number,Timer Interrupt
seie,9,9,number,External Interrupt 
end
Virtual Supevisor Interrupt Enable Register."
);
