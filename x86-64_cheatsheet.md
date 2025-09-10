# x86-64 Assembly Cheatsheet for Compiler Backends

## 1. Data Movement

| Instruction | Size | Description | Example |
|------------|------|------------|--------|
| `movb`     | 1B   | Move 8-bit value | `movb $1, %al` |
| `movw`     | 2B   | Move 16-bit value | `movw $2, %ax` |
| `movl`     | 4B   | Move 32-bit value | `movl $42, %eax` |
| `movq`     | 8B   | Move 64-bit value | `movq $100, %rax` |
| `movsd`    | 8B FP| Move double float | `movsd %xmm1, %xmm0` |
| `movss`    | 4B FP| Move single float | `movss %xmm1, %xmm0` |
| `lea`      | 64-bit addr | Load effective address | `lea (%rax,%rbx,4), %rcx` |

## 2. Arithmetic

| Instruction | Size | Description |
|------------|------|------------|
| `addb/addw/addl/addq` | 1/2/4/8B | Integer addition |
| `subb/subw/subl/subq` | 1/2/4/8B | Integer subtraction |
| `imul`     | 2/4/8B | Signed multiply |
| `idiv`     | 4/8B | Signed divide (`rax` or `rdx:rax`) |
| `inc/dec`  | 1/2/4/8B | Increment / decrement |

## 3. Logical / Bitwise

| Instruction | Description |
|------------|------------|
| `andb/andl/andq` | Bitwise AND |
| `orb/orl/orq` | Bitwise OR |
| `xorb/xorl/xorq` | Bitwise XOR |
| `notb/notl/notq` | Bitwise NOT |
| `shl/shr/sar` | Shift left / right / arithmetic |

## 4. Control Flow

| Instruction | Description |
|------------|------------|
| `jmp label` | Unconditional jump |
| `je/jz`    | Jump if equal / zero |
| `jne/jnz`  | Jump if not equal / nonzero |
| `jl/jnge`  | Jump if less (signed) |
| `jg/jnle`  | Jump if greater (signed) |
| `jle/jng`  | Jump if less or equal (signed) |
| `jge/jnl`  | Jump if greater or equal (signed) |
| `call label` | Call function |
| `ret`       | Return from function |

## 5. Stack / Function Frame

| Instruction | Description |
|------------|------------|
| `pushq %reg` | Push 64-bit register |
| `popq %reg` | Pop 64-bit register |
| `subq $imm, %rsp` | Allocate stack space |
| `addq $imm, %rsp` | Deallocate stack space |
| `movq %rsp, %rbp` | Set frame pointer |
| `leave` | Restore frame pointer & deallocate |

**Function arguments (System V AMD64 ABI):**
- Integers / pointers: `%rdi, %rsi, %rdx, %rcx, %r8, %r9`
- Floats: `%xmm0–%xmm7`

## 6. Floating Point (SSE)

| Instruction | Description |
|------------|------------|
| `movsd %xmm1, %xmm0` | Move double float |
| `addsd %xmm1, %xmm0` | Add double float |
| `subsd %xmm1, %xmm0` | Subtract double float |
| `mulsd %xmm1, %xmm0` | Multiply double float |
| `divsd %xmm1, %xmm0` | Divide double float |
| `movss/addss/...` | Single-precision float variants |

## 7. Directives / Labels

| Directive | Description |
|-----------|------------|
| `.globl symbol` | Make symbol global for linker |
| `.text` | Start code section |
| `.data` | Start data section |
| `.bss`  | Start uninitialized data section |
| `.byte value` | 1-byte literal |
| `.long value` | 4-byte literal |
| `.quad value` | 8-byte literal |
| `.align n` | Align to 2^n bytes |
| `.section .note.GNU-stack,"",@progbits` | Marks stack as non-executable |

## 8. Notes / Tips

- Use `movq` for 64-bit integers and pointers.
- Include `.globl main` and `.text` at top of every executable asm file.
- Always include `.section .note.GNU-stack,"",@progbits` to avoid warnings.
- Allocate locals with `subq $N, %rsp` and deallocate with `addq $N, %rsp`.
- Return from functions with `ret` after restoring the stack.
- `lea` is only for computing addresses, not loading data.

*Compiled for use with compiler backends (like nqcc) and C-style codegen.*
