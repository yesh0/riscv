use riscv_hypervisor_extension_proc_macro::generate_csr;
generate_csr!("Vsatp
640
mode,63,60,HgatpValues,Bare=0;Sv39x4=8;Sv48x4=9,Guest address translation mode.
as,59,44,number,ASID.
ppn,43,0,number,Physical Page Number for root page table.
end
Virtual Supervisor Guest Address Translation and Protection Register.");