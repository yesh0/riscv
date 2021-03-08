use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!(
    "Hvip
1605
vssip,2,2,number,Software Interrupt
vstip,6,6,number,Timer Interrupt
vseip,10,10,number,External Interrupt 
end
Hypervisor Virtual Interrupt Pending Register."
);
