// main.rs - MemProcFS Rust VMM API usage examples
//
// (c) Ulf Frisk, 2023
// Author: Ulf Frisk, pcileech@frizk.net
// https://github.com/ufrisk/MemProcFS
//

use memprocfs::*;
use pretty_hex::*;

pub fn main() {
    let vmm_lib_path;
    let memdump_path;
    if cfg!(windows) {
        vmm_lib_path = "C:\\Github\\MemProcFS-dev\\files\\vmm.dll";
        memdump_path = "C:\\Dumps\\trickbot-ram.pmem";
    } else {
        vmm_lib_path = "/home/user/memprocfs/vmm.so";
        memdump_path = "/dumps/warren.mem";
    }

    println!("MemProcFS Rust API Example - START");

    // Example arguments to initialize MemProcFS VMM.DLL/VMM.SO with:
    // For complete argument list please see MemProcFS command line documentation.
    let vmm_args = ["-printf", "-v", "-waitinitialize", "-device", memdump_path, "-vm"].to_vec();
    {
        // Example: Vmm::new
        // Instantiate a new Vmm object/struct both on the rust layer and the
        // underlying native layer.
        // The first argument is the path to the native vmm.dll or vmm.so.
        // The second argument is a Vec with MemProcFS command line arguments.
        // NB! For example simplicity we'll use unwrap() here which will panic
        //     when instantiation fails.
        let vmm = Vmm::new(vmm_lib_path, &vmm_args).unwrap();
        println!("vmm result = ok!");


        // Example: vmm.get_config():
        // Retrieve max native address and print it on the screen.
        println!("========================================");
        println!("Vmm.get_config():");
        println!("max native address: {:#x} -> {:#x}", CONFIG_OPT_CORE_MAX_NATIVE_ADDRESS, vmm.get_config(CONFIG_OPT_CORE_MAX_NATIVE_ADDRESS).unwrap_or(0));


        // Example: vmm.set_config():
        // For a full refresh of internal data caches.
        println!("========================================");
        println!("Vmm.set_config():");
        let _r = vmm.set_config(CONFIG_OPT_REFRESH_ALL, 1);
        println!("full refresh: -> Ok");


        // Example: vmm.mem_write():
        // Write to physical memory at address 0x1000
        // (Writes are only possible if underlying layers are write-capable.)
        println!("========================================");
        println!("Vmm.mem_write():");
        let data_to_write = [0x56u8, 0x4d, 0x4d, 0x52, 0x55, 0x53, 0x54].to_vec();
        match vmm.mem_write(0x1000, &data_to_write) {
            Ok(()) => println!("Vmm.mem_write(): success"),
            Err(e) => println!("Vmm.mem_write(): fail [{}]", e),
        }


        // Example: vmm.mem_read():
        // Read 0x100 bytes from physical address 0x1000.
        println!("========================================");
        println!("Vmm.mem_read():");
        if let Ok(data_read) = vmm.mem_read(0x1000, 0x100) {
            println!("{:?}", data_read.hex_dump());
        }


        // Example: vmm.mem_read_ex():
        // Read 0x100 bytes from physical address 0x1000 with vmm flags.
        println!("========================================");
        println!("Vmm.mem_read_ex():");
        if let Ok(data_read) = vmm.mem_read_ex(0x1000, 0x100, FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL) {
            println!("{:?}", data_read.hex_dump());
        }


        // Example: vmm.log():
        // Log a message to VMM/MemProcFS
        println!("========================================");
        println!("Vmm.log():");
        vmm.log(&VmmLogLevel::_1Critical, "Test Message Critical!");


        // Example: vmm.process_from_pid():
        // Retrieve the 'System' process by its PID.
        println!("========================================");
        println!("Vmm.process_from_pid():");
        if let Ok(process) = vmm.process_from_pid(4) {
            println!("{}", process);    
        }


        // Example: vmm.process_from_name():
        // Retrieve the 'System' process by its name.
        println!("========================================");
        println!("Vmm.process_from_name():");
        if let Ok(process) = vmm.process_from_name("System") {
            println!("{}", process);    
        }


        // Example: vmm.process_list():
        // Retrieve all processes of the running system as a Vec<process>
        println!("========================================");
        println!("Vmm.process_list():");
        if let Ok(process_all) = vmm.process_list() {
            for process in &*process_all {
                print!("{process} ");
            }
            println!("");
            // Example: Convert process list into a HashMap<K:pid, V:&VmmProcess>.
            let process_map : std::collections::HashMap<u32, VmmProcess> = process_all.into_iter().map(|s| (s.pid, s)).collect();
            for process in process_map {
                print!("{},{} ", process.0, process.1);
            }
            println!("");
        }


        // Example: vmm.process_map():
        // Retrieve all processes of the running system as a HashMap<pid, process>
        println!("========================================");
        println!("Vmm.process_map():");
        if let Ok(process_all) = vmm.process_map() {
            for process in process_all {
                print!("<{},{}> ", process.0, process.1);
            }
            println!("");
        }


        // Example: vmm.map_pfn():
        // Retrieve the first 10 page frame numbers PFNs and display extended info about them.
        // NB! extended PFN info is rather expensive so use with caution.
        println!("========================================");
        println!("vmm.map_pfn():");
        let pfns: Vec<u32> = (1..=10).collect();
        if let Ok(pfn_all) = vmm.map_pfn(&pfns, true) {
            for pfn in &*pfn_all {
                println!("{pfn} \t location={} tp_ex={} pid={:x} va={:x} color={}", pfn.location, pfn.tp_ex, pfn.pid, pfn.va, pfn.color);
            }
        }


        // Example: vmm.map_memory():
        // Retrieve the physical memory map as seen by the operating system:
        println!("========================================");
        println!("vmm.map_memory():");
        if let Ok(memory_range_all) = vmm.map_memory() {
            for memory_range in &*memory_range_all {
                println!("{memory_range} \t pa={:x} cb={:x}", memory_range.pa, memory_range.cb);
            }
        }


        // Example: vmm.map_net():
        // Retrieve the network connection information:
        println!("========================================");
        println!("vmm.map_net():");
        if let Ok(net_all) = vmm.map_net() {
            for net in &*net_all {
                println!("{net}");
            }
        } else {
            println!("Error retrieving network information.");
        }


        // Example: vmm.map_pool():
        // Retrieve kernel pool allocations and display the 'Proc' allocations.
        // NB! here we retrieve all pool allocations which is substantially
        //     slower than retrieving the big pool only.
        println!("========================================");
        println!("vmm.map_pool():");
        if let Ok(pool_all) = vmm.map_pool(false) {
            println!("Number of pool allocations: {}.", pool_all.len());
            let pool_proc_all : Vec<&VmmMapPoolEntry> = pool_all.iter().filter(|e| e.tag == 0x636f7250 /* 'Proc' backwards */).collect();
            println!("Number of pool 'Proc' allocations: {}.", pool_all.len());
            for pool_proc in &*pool_proc_all {
                print!("{pool_proc} ");
            }
            println!("");
        } else {
            println!("Error retrieving pool allocations.");
        }


        // Example: vmm.map_service():
        // Retrieve all services in the system:
        println!("========================================");
        println!("vmm.map_service():");
        if let Ok(service_all) = vmm.map_service() {
            for service in &*service_all {
                print!("{service} ");
            }
            println!("");
        }


        // Example: vmm.map_user():
        // Retrieve the detected users in the system:
        println!("========================================");
        println!("vmm.map_user():");
        if let Ok(user_all) = vmm.map_user() {
            for user in &*user_all {
                println!("{:x}:: {} :: {} :: {user}", user.va_reg_hive, user.sid, user.user);
            }
        }


        // Example: vmm.map_virtual_machine():
        // Retrieve any virtual machines detected:
        // NB! vm parsing must be enabled (-vm startup option).
        println!("========================================");
        println!("vmm.map_virtual_machine():");
        if let Ok(virtualmachine_all) = vmm.map_virtual_machine() {
            for virtualmachine in &*virtualmachine_all {
                println!("{virtualmachine}");
                if virtualmachine.is_active {
                    // for active vms it's possible to create a new vmm object for
                    // the vm. it's possible to treat this as any other vmm object
                    // to read memory, query processes etc.
                    let vmm_vm = match Vmm::new_from_virtual_machine(&vmm, &virtualmachine) {
                        Err(_) => continue,
                        Ok(r) => r,
                    };
                    println!("vm max native address: {:#x} -> {:#x}", CONFIG_OPT_CORE_MAX_NATIVE_ADDRESS, vmm_vm.get_config(CONFIG_OPT_CORE_MAX_NATIVE_ADDRESS).unwrap_or(0));
                }
            }
        }


        // Example: vmm.kernel().process():
        // Retrieve the system process (PID 4).
        // NB! vmm.kernel() is a lightweight operation so ok to call multiple times...
        println!("========================================");
        println!("vmm.kernel().process():");
        println!("{}", vmm.kernel().process());


        // Example: vmm.kernel().build():
        // Retrieve the kernel build number.
        println!("========================================");
        println!("vmm.kernel().build():");
        println!("{}", vmm.kernel().build());


        // Example: vmm.kernel().pdb():
        // Retrieve the VmmPdb object containg debug symbols for ntoskrnl.
        // NB! This call will always succeed even if the symbols aren't loaded.
        //     Subsequent calls to the pdb methods may however fail.
        println!("========================================");
        println!("vmm.kernel().pdb():");
        let kernel = vmm.kernel();
        let pdb = kernel.pdb();
        println!("{pdb}");


        // Example: pdb.symbol_address_from_name():
        // Retrieve the address of the symbol nt!MiMapContiguousMemory
        // NB! this requires that the MemProcFS symbol-subsystem is working.
        println!("========================================");
        println!("pdb.symbol_address_from_name():");
        let mut va_kernel_symbol = 0u64;
        if let Ok(va) = pdb.symbol_address_from_name("MiMapContiguousMemory") {
            va_kernel_symbol = va;
            println!("Address of 'MiMapContiguousMemory' = {:x}", va_kernel_symbol);
        } else {
            println!("Error retrieving symbol address for 'MiMapContiguousMemory'");
        }


        // Example: pdb.symbol_name_from_address():
        // Retrieve the symbol name from an address.
        // Use the already retrieved address of nt!MiMapContiguousMemory.
        if va_kernel_symbol != 0 {
            println!("========================================");
            println!("pdb.symbol_name_from_address():");
            if let Ok(r) = pdb.symbol_name_from_address(va_kernel_symbol) {
                println!("Address: {:x} Name: '{}' Displacement: {:x}", va_kernel_symbol, r.0, r.1);
            } else {
                println!("Error retrieving symbol name for address {:x}", va_kernel_symbol);
            }
        }


        // Example: pdb.type_size():
        // Retrieve the size of a type. In this example use _EPROCESS.
        println!("========================================");
        println!("pdb.type_size:");
        match pdb.type_size("_EPROCESS") {
            Err(_) => println!("Error retrieving type size."),
            Ok(type_size) => println!("Type Size of _EPROCESS: {:x}", type_size),
        }


        // Example: pdb.type_child_offset():
        // Retrieve the offset of a type child.
        // In this example use _EPROCESS.VadRoot
        println!("========================================");
        println!("pdb.type_size:");
        match pdb.type_child_offset("_EPROCESS", "VadRoot") {
            Err(_) => println!("Error retrieving type child offset."),
            Ok(offset_child) => println!("Offset of _EPROCESS.VadRoot: {:x}", offset_child),
        }






        // Examples below deal with virtual file system (vfs) access.
        // Example: vmm.vfs_list():
        // Retrieve a directory listing of the /sys/ folder.
        // NB! forward-slash '/' and back-slash '\\' both work fine!
        println!("========================================");
        println!("vmm.vfs_list():");
        let vfs_list_path = "/sys/";
        if let Ok(vfs_all) = vmm.vfs_list(vfs_list_path) {
            println!("VFS directory listing for directory: {vfs_list_path}");
            println!("Number of file/directory entries: {}.", vfs_all.len());
            for vfs in &*vfs_all {
                println!("{vfs}");
            }
        }
        

        // Example: vmm.vfs_read():
        // Read (a part) of a file in the virtual file system.
        // In this case try reading the file /sys/memory/physmemmap.txt
        println!("========================================");
        println!("vmm.vfs_read():");
        if let Ok(vfs_file_data) = vmm.vfs_read("/sys/memory/physmemmap.txt", 0x2000, 0) {
            println!("Number of bytes file contents read from file '/sys/memory/physmemmap.txt': {}.", vfs_file_data.len());
            println!("{:?}", vfs_file_data.hex_dump());
        }


        // Example: vmm.vfs_write():
        // Write (to a part) of a file in the virtual file system.
        // In this case write '1' to /conf/config_process_show_terminated.txt
        // to enable listings of terminated processes in the filesystem.
        // NB! vfs_write() writes are undertaken on a best-effort!
        //     please verify with vfs_read() (if this is possible).
        println!("========================================");
        println!("vmm.vfs_write():");
        let vfs_write_data = vec![1u8; 1];
        vmm.vfs_write("/conf/config_process_show_terminated.txt", vfs_write_data, 0);


        // Example: vmm.vfs_read():
        // Read (a part) of a file in the virtual file system.
        // In this case try reading the file /sys/memory/physmemmap.txt
        println!("========================================");
        println!("vmm.vfs_read():");
        if let Ok(vfs_file_data) = vmm.vfs_read("/conf/config_process_show_terminated.txt", 0x2000, 0) {
            println!("Number of bytes file contents read from file '/conf/config_process_show_terminated.txt': {}.", vfs_file_data.len());
            println!("{:?}", vfs_file_data.hex_dump());
        }






        // Example vmm.reg_hive_list():
        // List the registry hives of the system:
        println!("========================================");
        println!("vmm.reg_hive_list():");
        let mut hive_software : Option<VmmRegHive> = None;
        let hive_all = vmm.reg_hive_list();
        if hive_all.is_ok() {
            for hive in hive_all.unwrap() {
                println!("{hive} size={} path={}", hive.size, hive.path);
                if hive.path.contains("SOFTWARE") {
                    hive_software = Some(hive);
                }
            }
        }


        // Example: vmm.reg_hive_read():
        // Read 0x100 bytes from the address 0x1000 in the software registry hive.
        println!("========================================");
        println!("vmm.reg_hive_read():");
        if let Some(hive_software) = &hive_software {
            if let Ok(data_read) = hive_software.reg_hive_read(0x1000, 0x100, FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL) {
                println!("{:?}", data_read.hex_dump());
            }
        }


        // Example: vmm.reg_hive_write():
        // Write to the registry address 0x1000.
        // This have been commented out since this is extremely dangerous on live
        // systems and is likely to bluescreen / cause registry corruption.
        /*
        println!("========================================");
        println!("vmm.reg_hive_write():");
        if let Some(hive_software) = &hive_software {
            let data_to_write = [0x56u8, 0x4d, 0x4d, 0x52, 0x55, 0x53, 0x54].to_vec();
            let _r = hive_software.reg_hive_write(0x1000, &data_to_write);
        }
        */


        // Example: vmm.reg_key()
        // Retrieve the current version key given the full named path.
        // Registry paths are case sensitive and use backslashes.
        // Since this key will be used in following examples it's unwrap().
        println!("========================================");
        println!("vmm.reg_key():");
        let reg_path = "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion";
        println!("path: {reg_path}");
        let vmmregkey = vmm.reg_key(reg_path).unwrap();
        println!("{vmmregkey}");


        // Example: vmm.reg_key()
        // Retrieve the run key given the software hive address and hive path.
        // Registry paths are case sensitive and use backslashes.
        println!("========================================");
        println!("vmm.reg_key():");
        if let Some(hive_software) = &hive_software {
            let reg_path = format!("0x{:x}\\ROOT\\Microsoft\\Windows\\CurrentVersion\\Run", hive_software.va);
            println!("path: {reg_path}");
            if let Ok(regkey) = vmm.reg_key(reg_path.as_str()) {
                println!("{regkey}");
            }
        }


        // Example: vmmregkey.parent()
        println!("========================================");
        println!("vmmregkey.parent():");
        if let Ok(parentkey) = vmmregkey.parent() {
            println!("{parentkey}");
        }


        // Example: vmmregkey.subkeys()
        println!("========================================");
        println!("vmmregkey.subkeys():");
        if let Ok(key_all) = vmmregkey.subkeys() {
            for key in key_all {
                print!("{key} ") ;
            }
            println!("");
        }


        // Example: vmmregkey.subkeys_map()
        println!("========================================");
        println!("vmmregkey.subkeys_map():");
        if let Ok(key_all) = vmmregkey.subkeys_map() {
            for e in key_all {
                print!("<{},{}> ", e.0, e.1) ;
            }
            println!("");
        }


        // Example: vmm.reg_value()
        // Registry paths are case sensitive and use backslashes.
        // Since this value will be used in following examples it's unwrap().
        println!("========================================");
        println!("vmm.reg_value():");
        let reg_path = "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\ProgramFilesDir";
        println!("path: {reg_path}");
        let vmmregvalue = vmm.reg_value(reg_path).unwrap();
        println!("{vmmregvalue} raw_type={} raw_size={}", vmmregvalue.raw_type, vmmregvalue.raw_size);


        // Example: vmmregvalue.raw_value()
        // Retrieve the raw underlying data backing the actual value.
        println!("========================================");
        println!("vmm.raw_value():");
        if let Ok(raw_value) = vmmregvalue.raw_value() {
            println!("{:?}", raw_value.hex_dump())
        }


        // Example: vmmregvalue.value()
        // REG_SZ
        println!("========================================");
        println!("vmm.raw_value(): REG_SZ");
        if let Ok(VmmRegValueType::REG_SZ(s)) = vmmregvalue.value() {
            println!("REG_SZ: {s}");
        }


        // Example: vmmregvalue.value()
        // REG_MULTI_SZ
        println!("========================================");
        println!("vmm.raw_value(): REG_MULTI_SZ");
        if let Ok(regvalue) = vmm.reg_value("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileAssociation\\UseLocalMachineSoftwareClassesWhenImpersonating") {
            if let Ok(VmmRegValueType::REG_MULTI_SZ(multistr)) = regvalue.value() {
                for s in multistr {
                    println!("REG_MULTI_SZ: {s}");
                }
            }
        }


        // Example: vmmregvalue.value()
        // REG_DWORD
        println!("========================================");
        println!("vmm.raw_value(): REG_DWORD");
        if let Ok(regvalue) = vmm.reg_value("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FSIASleepTimeInMs") {
            if let Ok(VmmRegValueType::REG_DWORD(dw)) = regvalue.value() {
                println!("REG_DWORD: 0x{:08x}", dw);
            }
        }






        // Example: vmm.process_from_name():
        // Retrieve the process object for 'explorer.exe'.
        // If explorer.exe does not exist just panic since the remainder of the
        // examples forward on are process related.
        println!("========================================");
        println!("vmm.process_from_name():");
        let vmmprocess = vmm.process_from_name("explorer.exe").unwrap();
        println!("PID of explorer.exe: {}", vmmprocess.pid);


        // Example: vmmprocess.info():
        // Retrieve common process info such as process pid, ppid and name.
        println!("========================================");
        println!("vmmprocess.info():");
        if let Ok(procinfo) = vmmprocess.info() {
            println!("struct   -> {procinfo}");
            println!("pid      -> {}", procinfo.pid);
            println!("ppid     -> {}", procinfo.pid);
            println!("peb      -> {:x}", procinfo.va_peb);
            println!("eprocess -> {:x}", procinfo.va_eprocess);
            println!("name     -> {}", procinfo.name);
            println!("longname -> {}", procinfo.name_long);
            println!("SID      -> {}", procinfo.sid);
        }


        // Example: vmmprocess.get_module_base():
        // Retrieve the base address of a module.
        println!("========================================");
        println!("vmmprocess.get_module_base():");
        if let Ok(modulebaseaddress) = vmmprocess.get_module_base("kernel32.dll") {
            println!("kernel32.dll -> {:x}", modulebaseaddress);
        }


        // Example: vmmprocess.get_proc_address():
        // Retrieve the function address inside a module i.e. GetProcAddress().
        println!("========================================");
        println!("vmmprocess.get_proc_address():");
        if let Ok(procaddress) = vmmprocess.get_proc_address("kernel32.dll", "GetProcAddress") {
            println!("kernel32.dll!GetProcAddress -> {:x}", procaddress);
        }


        // Example: vmmprocess.get_cmdline():
        // Retrieve the process commandline.
        println!("========================================");
        println!("vmmprocess.get_cmdline():");
        if let Ok(s_cmdline) = vmmprocess.get_cmdline() {
            println!("-> {s_cmdline}");
        }


        // Example: vmmprocess.get_path_user():
        // Retrieve the process image path in user-mode (derived from PEB).
        println!("========================================");
        println!("vmmprocess.get_path_user():");
        if let Ok(s) = vmmprocess.get_path_user() {
            println!("-> {s}");
        }


        // Example: vmmprocess.get_path_kernel():
        // Retrieve the process image path in user-mode (derived from EPROCESS).
        println!("========================================");
        println!("vmmprocess.get_path_kernel():");
        if let Ok(s) = vmmprocess.get_path_kernel() {
            println!("-> {s}");
        }


        // Example: vmmprocess.map_pte():
        // Retrieve the page table entry (PTE) map for explorer.
        println!("========================================");
        println!("vmmprocess.map_pte():");
        if let Ok(pte_all) = vmmprocess.map_pte(true) {
            println!("Number of pte entries: {}.", pte_all.len());
            for pte in &*pte_all {
                let s = if pte.is_s { 's' } else { '-' };
                let r = if pte.is_r { 'r' } else { '-' };
                let w = if pte.is_w { 'w' } else { '-' };
                let x = if pte.is_x { 'x' } else { '-' };
                println!("{pte} :: {s}{r}{w}{x} :: {}", pte.info);
            }
        } else {
            println!("Error retrieving process pte map.");
        }


        // Example: vmmprocess.map_vad():
        // Retrieve the virtual address descriptor (VAD) map for explorer.
        println!("========================================");
        println!("vmmprocess.map_vad():");
        if let Ok(vad_all) = vmmprocess.map_vad(true) {
            println!("Number of vad entries: {}.", vad_all.len());
            for vad in &*vad_all {
                println!("{vad} :: {}", vad.info);
            }
        } else {
            println!("Error retrieving process vad map.");
        }


        // Example: vmmprocess.map_handle():
        // Retrieve the open handles associated with the process.
        println!("========================================");
        println!("vmmprocess.map_handle():");
        if let Ok(handle_all) = vmmprocess.map_handle() {
            println!("Number of handle entries: {}.", handle_all.len());
            for handle in &*handle_all {
                println!("{handle}");
            }
        } else {
            println!("Error retrieving process handle map.");
        }


        // Example: vmmprocess.map_heap():
        // Retrieve info about the process heaps:
        println!("========================================");
        println!("vmmprocess.map_heap():");
        if let Ok(heap_all) = vmmprocess.map_heap() {
            println!("Number of heap entries: {}.", heap_all.len());
            for heap in &*heap_all {
                println!("{heap}");
            }
        } else {
            println!("Error retrieving process heap map.");
        }


        // Example: vmmprocess.map_heapalloc():
        // Retrieve info about the allocated heap entries for heap 0.
        println!("========================================");
        println!("vmmprocess.map_heapalloc():");
        if let Ok(heapalloc_all) = vmmprocess.map_heapalloc(0) {
            println!("Number of allocated heap entries: {}.", heapalloc_all.len());
            for heapalloc in &*heapalloc_all {
                print!("{heapalloc} ");
            }
            println!("");
        } else {
            println!("Error retrieving process heap allocation map.");
        }


        // Example: vmmprocess.map_thread():
        // Retrieve information about the process threads.
        println!("========================================");
        println!("vmmprocess.map_thread():");
        if let Ok(thread_all) = vmmprocess.map_thread() {
            println!("Number of process threads: {}.", thread_all.len());
            for thread in &*thread_all {
                println!("{thread}");
            }
        } else {
            println!("Error retrieving process thread map.");
        }


        // Example: vmmprocess.map_unloaded_module():
        // Retrieve information about unloaded modules (if any).
        println!("========================================");
        println!("vmmprocess.map_unloaded_module():");
        if let Ok(unloaded_all) = vmmprocess.map_unloaded_module() {
            println!("Number of process unloaded modules: {}.", unloaded_all.len());
            for unloaded in &*unloaded_all {
                println!("{unloaded}");
            }
        } else {
            println!("Error retrieving process unloaded modules map.");
        }


        // Example: vmmprocess.map_module():
        // Retrieve information about process modules.
        // NB! process map is used in subsequent example calls so panic if
        //     there is some error for example simplicity.
        // NB! Debug Info and Version Info will slow down parsing somewhat.
        println!("========================================");
        println!("vmmprocess.map_module():");
        let module_all = vmmprocess.map_module(true, true).unwrap();
        println!("Number of process modules: {}.", module_all.len());
        for module in &*module_all {
            println!("{module}");
        }


        // Example: locate the module of kernel32 within the modules.
        // NB! this will panic if kernel32 is not found, but use this
        //     for simplicity below.
        println!("========================================");
        println!("locate module of kernel32.dll:");
        let kernel32 : &VmmProcessMapModuleEntry = (&*module_all).into_iter().find(| m| m.name.to_lowercase() == "kernel32.dll").unwrap();
        println!("module kernel32 located: {kernel32}");


        // Show debug information related to kernel32.
        // NB! debug info is retrieved on a best-effort way if the flag
        //     is_info_debug is set to true in the vmmprocess.map_module() call.
        println!("========================================");
        println!("kernel32 debug info:");
        if let Some(debug_info) = &kernel32.debug_info {
            println!("kernel32 debug info found. {}", debug_info);
            println!("kernel32 debug guid={}, pdb={}", debug_info.guid, debug_info.pdb_filename);
        } else {
            println!("kernel32 debug info not found.");
        }


        // Show version information related to kernel32.
        // NB! version info is retrieved on a best-effort way if the flag
        //     is_info_version is set to true in the vmmprocess.map_module() call.
        println!("========================================");
        println!("kernel32 version info:");
        if let Some(ver_info) = &kernel32.version_info {
            println!("kernel32 version info found. {}", ver_info);
            println!("kernel32 company_name:       {}", ver_info.company_name);
            println!("kernel32 file_description:   {}", ver_info.file_description);
            println!("kernel32 file_version:       {}", ver_info.file_version);
            println!("kernel32 internal_name:      {}", ver_info.internal_name);
            println!("kernel32 legal_copyright:    {}", ver_info.legal_copyright);
            println!("kernel32 original_file_name: {}", ver_info.original_file_name);
            println!("kernel32 product_name:       {}", ver_info.product_name);
            println!("kernel32 product_version:    {}", ver_info.product_version);
        } else {
            println!("kernel32 version info not found.");
        }


        // Example: vmmprocess.pdb_from_module_address():
        // Retrieve debugging information (for microsoft modules) given
        // the module base address.
        println!("========================================");
        println!("vmmprocess.pdb_from_module_address():");
        if let Ok(pdb) = vmmprocess.pdb_from_module_address(kernel32.va_base) {
            println!("-> {pdb}");
        }


        // Example: vmmprocess.map_module_eat():
        // Retrieve exported functions in the export address table (EAT) of a
        // module (kernel32 in this case).
        println!("========================================");
        println!("vmmprocess.map_module_eat():");
        if let Ok(eat_all) = vmmprocess.map_module_eat("kernel32.dll") {
            println!("Number of module exported functions: {}.", eat_all.len());
            for eat in &*eat_all {
                println!("{eat} :: {}", eat.forwarded_function);
            }
        } else {
            println!("Error retrieving module exported functions (EAT).");
        }


        // Example: vmmprocess.map_module_iat():
        // Retrieve imported functions in the import address table (IAT) of a
        // module (kernel32 in this case).
        println!("========================================");
        println!("vmmprocess.map_module_iat():");
        if let Ok(iat_all) = vmmprocess.map_module_iat("kernel32.dll") {
            println!("Number of module imported functions: {}.", iat_all.len());
            for iat in &*iat_all {
                println!("{iat}");
            }
        } else {
            println!("Error retrieving module imported functions (IAT).");
        }

        // Example: vmmprocess.map_module_data_directory():
        // Retrieve info about the PE data directories, which always equal 16.
        println!("========================================");
        println!("vmmprocess.map_module_data_directory():");
        if let Ok(data_directory_all) = vmmprocess.map_module_data_directory("kernel32.dll") {
            println!("Number of module data directories: {}.", data_directory_all.len());
            for data_directory in &*data_directory_all {
                println!("{data_directory}");
            }
        }

        // Example: vmmprocess.map_module_section():
        // Retrieve info about the PE sections.
        println!("========================================");
        println!("vmmprocess.map_module_section():");
        if let Ok(section_all) = vmmprocess.map_module_section("kernel32.dll") {
            println!("Number of module sections: {}.", section_all.len());
            for section in &*section_all {
                println!("{section}");
            }
        }






        // Example: vmmprocess.mem_write():
        // Write to virtual memory of kernel32 PE header (dangerous)
        // (Writes are only possible if underlying layers are write-capable.)
        println!("========================================");
        println!("vmmprocess.mem_write():");
        let data_to_write = [0x56u8, 0x4d, 0x4d, 0x52, 0x55, 0x53, 0x54].to_vec();
        match vmmprocess.mem_write(kernel32.va_base + 8, &data_to_write) {
            Err(e) => println!("vmmprocess.mem_write(): fail [{}]", e),
            Ok(()) => println!("vmmprocess.mem_write(): success"),
        }


        // Example: vmmprocess.mem_read():
        // Read 0x100 bytes from beginning of explorer.exe!kernel32.dll
        println!("========================================");
        println!("vmmprocess.mem_read():");
        match vmmprocess.mem_read(kernel32.va_base, 0x100) {
            Err(e) => println!("vmmprocess.mem_read(): fail [{}]", e),
            Ok(data_read) => println!("{:?}", data_read.hex_dump()),
        }


        // Example: vmmprocess.mem_read_ex():
        // Read 0x100 bytes from beginning of explorer.exe!kernel32.dll with vmm flags.
        println!("========================================");
        println!("vmmprocess.mem_read_ex():");
        match vmmprocess.mem_read_ex(kernel32.va_base, 0x100, FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL) {
            Err(e) => println!("vmmprocess.mem_read_ex(): fail [{}]", e),
            Ok(data_read) => println!("{:?}", data_read.hex_dump()),
        }


        // Example: vmmprocess.mem_virt2phys():
        // Retrieve the physical base address of explorer.exe!kernel32.dll.
        println!("========================================");
        println!("vmmprocess.mem_virt2phys():");
        match vmmprocess.mem_virt2phys(kernel32.va_base) {
            Err(e) => println!("vmmprocess.mem_virt2phys(): fail [{}]", e),
            Ok(pa_kernel32) => println!("explorer.exe!kernel32 va={:x} pa={:x}", kernel32.va_base, pa_kernel32),
        }


        // Example: vmmprocess.mem_scatter() #1:
        // This example will show how it's possible to use VmmScatterMemory to
        // more efficiently read memory from the underlying device.
        {
            // Example: vmmprocess.mem_scatter():
            // Retrieve a scatter memory read object that may be used to batch
            // several reads/writes into one efficient call to the memory device.
            println!("========================================");
            println!("vmmprocess.mem_scatter() #1:");
            let mem_scatter = vmmprocess.mem_scatter(FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL).unwrap();
            println!("mem_scatter = {mem_scatter}");
            // Prepare three memory ranges to read.
            let _r = mem_scatter.prepare(kernel32.va_base + 0x0000, 0x100);
            let _r = mem_scatter.prepare(kernel32.va_base + 0x1000, 0x100);
            let _r = mem_scatter.prepare(kernel32.va_base + 0x2000, 0x100);
            // Perform the actual read (and writes) by calling the execute() function.
            let _r = mem_scatter.execute();
            // Fetch data read. It's possible (but wasteful) to read less data than was prepared.
            if let Ok(data_read) = mem_scatter.read(kernel32.va_base + 0x0000, 0x80) {
                println!("memory range: va={:x} cb={:x} cb_read={:x}", kernel32.va_base + 0x0000, 0x80, data_read.len());
                println!("{:?}", data_read.hex_dump());
                println!("-----------------------");
            }
            if let Ok(data_read) = mem_scatter.read(kernel32.va_base + 0x1000, 0x100) {
                println!("memory range: va={:x} cb={:x} cb_read={:x}", kernel32.va_base + 0x1000, 0x100, data_read.len());
                println!("{:?}", data_read.hex_dump());
                println!("-----------------------");
            }
            // It's possible to do a re-read of the ranges by calling execute again!
            let _r = mem_scatter.execute();
            if let Ok(data_read) = mem_scatter.read(kernel32.va_base + 0x0000, 0x80) {
                println!("memory range: va={:x} cb={:x} cb_read={:x}", kernel32.va_base + 0x0000, 0x80, data_read.len());
                println!("{:?}", data_read.hex_dump());
                println!("-----------------------");
            }
            // It's also possible to clear the VmmScatterMemory to start anew.
            // Clearing is slightly more efficient than creating a new object.
            // let _r = mem_scatter.clear();

            // NB! the VmmScatterMemory struct will be automatically free'd
            //     on the native backend when it goes out of scope.
        }


        // Example: vmmprocess.mem_scatter() #2:
        // This example demo how it's possible to use the prepare_ex function
        // which will populate the prepared data regions automatically when the
        // VmmScatterMemory is dropped.
        // It's not recommended to mix the #1 and #2 syntaxes.
        {
            // memory ranges to read are tuples:
            // .0 = the virtual address to read.
            // .1 = vector of u8 which memory should be read into.
            // .2 = u32 receiving the bytes successfully read data.
            let mut memory_range_1 = (kernel32.va_base + 0x0000, vec![0u8; 0x100], 0u32);
            let mut memory_range_2 = (kernel32.va_base + 0x1000, vec![0u8; 0x100], 0u32);
            let mut memory_range_3 = (kernel32.va_base + 0x2000, vec![0u8; 0x100], 0u32);
            // Feed the ranges into a mutable VmmScatterMemory inside a
            // separate scope. The actual memory read will take place when
            // the VmmScatterMemory goes out of scope and are dropped.
            println!("========================================");
            println!("vmmprocess.mem_scatter() #2:");
            if let Ok(mut mem_scatter) = vmmprocess.mem_scatter(FLAG_NOCACHE | FLAG_ZEROPAD_ON_FAIL) {
                let _r = mem_scatter.prepare_ex(&mut memory_range_1);
                let _r = mem_scatter.prepare_ex(&mut memory_range_2);
                let _r = mem_scatter.prepare_ex(&mut memory_range_3);
            }
            // Results should now be available in the memory ranges if the read
            // was successful. Note that there is no guarantee that memory is
            // read - make sure to check the .2 item - number of bytes read.
            for memory_range in [memory_range_1, memory_range_2, memory_range_3] {
                println!("memory range: va={:x} cb={:x} cb_read={:x}", memory_range.0, memory_range.1.len(), memory_range.2);
                println!("{:?}", memory_range.1.hex_dump());
                println!("-----------------------");
            }
        }






        // Example: close:
        // The underlying native VMM instance will be automatically dropped
        // when the Rust Vmm struct goes out of scope and is dropped.
    }

    println!("MemProcFS Rust API Example - COMPLETED");
}