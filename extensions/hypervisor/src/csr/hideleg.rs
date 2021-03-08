use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!(
    "Hideleg
1539
sip,2,2,number,Software Interrupt
tip,6,6,number,Timer Interrupt
eip,10,10,number,External Interrupt 
end
Hypervisor Interrupt Delegation Register."
);
