use std::arch::{/*asm, global_asm,*/ naked_asm};
use std::io;
use std::io::Read;

fn main() {
    let mut handles = Vec::with_capacity(32);
    for _ in 0..32{
        handles.push(std::thread::spawn(asm_wrap));
    }

    // for handle in handles{
    //     handle.join().unwrap();
    // }


    let avx2:bool = get_avx2();
    if !avx2{
        println!("Cpu doesn't support AVX2 - CPU Toaster needs avx2 to run!");
        return;
    }

    let cores = get_core_count();
    println!("Running CPU Toaster on {} cores.", cores);
    println!("Press Enter to stop and exit.");
    let _ = io::stdin().read(&mut [0u8]); // wait for Enter
}


fn asm_wrap(){
    asm();
}


macro_rules! hot_block { () => { 
    "vfmadd132pd ymm0, ymm1, ymm2\n
    vfmadd132pd ymm3, ymm4, ymm5\n
    vfmadd132pd ymm6, ymm7, ymm8\n
    vfmadd132pd ymm9, ymm10, ymm11\n
    vfmadd132pd ymm12, ymm13, ymm14\n
    vfmadd132pd ymm15, ymm0, ymm1\n
    vfmadd132pd ymm2, ymm3, ymm4\n
    vfmadd132pd ymm5, ymm6, ymm7\n
    vfmadd132pd ymm8, ymm9, ymm10\n
    vfmadd132pd ymm11, ymm12, ymm13\n
    vfmadd132pd ymm14, ymm15, ymm0\n
    vfmadd132pd ymm1, ymm2, ymm3\n
    vfmadd132pd ymm4, ymm5, ymm6\n
    vfmadd132pd ymm7, ymm8, ymm9\n
    vfmadd132pd ymm10, ymm11, ymm12\n
    vfmadd132pd ymm13, ymm14, ymm15\n" 
} }

macro_rules! hot_blocks { () => { 
    concat!(hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),hot_block!(),)
} }

macro_rules! hot_blocks_blocks { () => { 
    concat!(hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),hot_blocks!(),)
} }

#[unsafe(naked)]
extern "C" fn asm() -> usize{
    // let mut x:u64 = 10;
    naked_asm!{

        "sub rsp, 256",
       // Save callee-saved registers YMM8-15
        "vmovdqu [rsp + 0], ymm8",   // Save YMM8
        "vmovdqu [rsp + 32], ymm9",  // Save YMM9
        "vmovdqu [rsp + 64], ymm10", // Save YMM10
        "vmovdqu [rsp + 96], ymm11", // Save YMM11
        "vmovdqu [rsp + 128], ymm12",// Save YMM12
        "vmovdqu [rsp + 160], ymm13",// Save YMM13
        "vmovdqu [rsp + 192], ymm14",// Save YMM14
        "vmovdqu [rsp + 224], ymm15",// Save YMM15

        "vpxor   ymm0, ymm0, ymm0",
        "vpxor   ymm1, ymm1, ymm1",
        "vpxor   ymm2, ymm2, ymm2",
        "vpxor   ymm3, ymm3, ymm3",
        "vpxor   ymm4, ymm4, ymm4",
        "vpxor   ymm5, ymm5, ymm5",
        "vpxor   ymm6, ymm6, ymm6",
        "vpxor   ymm7, ymm7, ymm7",
        "vpxor   ymm8, ymm8, ymm8",
        "vpxor   ymm9, ymm9, ymm9",
        "vpxor   ymm10, ymm10, ymm10",
        "vpxor   ymm11, ymm11, ymm11",
        "vpxor   ymm12, ymm12, ymm12",
        "vpxor   ymm13, ymm13, ymm13",
        "vpxor   ymm14, ymm14, ymm14",
        "vpxor   ymm15, ymm15, ymm15",


        // "mov rcx, 300000000000",
        "2:",

        //16*16 = 256 hot block isntances back to back,* 16 lines * 7 bytes per opcode = 28.672 bytes
        // 28.672+ some change fits in typical L1 cache of 32k So ideal size. (and measurement supports this)
        hot_blocks_blocks!(),


        // "dec rcx",
        // "jnz 2b",
        // "loop 2b",
        "jmp 2b",

        // Restore callee-saved registers YMM8-15
        "vmovdqu ymm8, [rsp + 0]",   // Restore YMM8
        "vmovdqu ymm9, [rsp + 32]",  // Restore YMM9
        "vmovdqu ymm10, [rsp + 64]", // Restore YMM10
        "vmovdqu ymm11, [rsp + 96]", // Restore YMM11
        "vmovdqu ymm12, [rsp + 128]",// Restore YMM12
        "vmovdqu ymm13, [rsp + 160]",// Restore YMM13
        "vmovdqu ymm14, [rsp + 192]",// Restore YMM14
        "vmovdqu ymm15, [rsp + 224]",// Restore YMM15

        "add rsp, 256",
        "ret",
    }


}


#[unsafe(naked)]
extern "C" fn get_core_count() -> usize{

    naked_asm!{
        "push rbx",

        "mov eax, 1",
        "xor ecx, ecx",
        "cpuid",
        "shr ebx, 16",
        "and ebx, 0xFF",
        "mov eax, ebx",

        "pop rbx",
        "ret",
    }

}


#[unsafe(naked)]
extern "C" fn get_avx2() -> bool{

    naked_asm!{
        "push rbx",

        "mov eax, 7",
        "xor ecx, ecx",
        "cpuid",
        "xor rax, rax",
        "bt ebx, 5",
        "setc al",

        // "xor rax, rax", //test for no avx

        "pop rbx",
        "ret",
    }

}
