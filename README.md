[![CI](https://github.com/platonvin/unsafe_math/actions/workflows/ci.yml/badge.svg)](https://github.com/platonvin/unsafe_math/actions/workflows/ci.yml)

# Unsafe Math

`unsafe_math` is a proc_macro that replaces math with unchecked / fast-math versions.\
On practice, this makes math match assembly of GCC/Clang with "-Ofast" \
Requires nightly

## Usage
Simply add `#[unsafe_math]` attribute to the scope you want it to apply to

```rust
use unsafe_math::*;

#[unsafe_math]
fn function(...) -> ... {
    ...
}
```

---

## Examples

This section demonstrates `unsafe_math` effect on produced assembly for few examples.\
`somefun_slow` corresponds to slow version, `somefun_fast` corresponds to version with `#[unsafe math]`:

#### Example 1
```rust
fn convert(block: i32) -> i32 {
    (block * 16) / 8
}
```

Difference in assembly will be as follows:

```assembly
convert_slow:
    mov eax, ecx
    shl eax, 4
    sar eax, 3
    ret

convert_fast:
    lea eax, [rcx + rcx]
    ret

```

#### Example 2

```rust
fn sum(a: u32) -> u32 {
    (0..a).map(|i| 1 << i as u32).sum()
}
```

<details>

<summary>slow_sum assembly (44 instructions, 92ns / characteristic iter) (click to expand)</summary>

```assembly
sum_sum
    push rsi
    test ecx, ecx
    je .LBB7_1
    mov r8d, ecx
    mov r9d, ecx
    and r9d, 3
    cmp ecx, 4
    jae .LBB7_4
    xor eax, eax
    xor edx, edx
    jmp .LBB7_6
.LBB7_1:
    xor eax, eax
    pop rsi
    ret
.LBB7_4:
    and r8d, -4
    xor eax, eax
    xor edx, edx
.LBB7_5:
    mov ecx, edx
    and cl, 28
    mov r10d, 1
    shl r10d, cl
    mov r11d, 2
    shl r11d, cl
    mov esi, 4
    shl esi, cl
    add r10d, eax
    add esi, r11d
    mov eax, 8
    shl eax, cl
    add esi, r10d
    add edx, 4
    add eax, esi
    cmp r8d, edx
    jne .LBB7_5
.LBB7_6:
    test r9d, r9d
    je .LBB7_8
.LBB7_7:
    mov r8d, 1
    mov ecx, edx
    shl r8d, cl
    inc edx
    add eax, r8d
    dec r9d
    jne .LBB7_7
.LBB7_8:
    pop rsi
    ret
```
</details>

<details>
<summary>fast_sum assembly (32 instructions, 77ns / characteristic iter) (click to expand)</summary>

```assembly
sum_fast:
    test ecx, ecx
    je .LBB10_1
    mov edx, ecx
    mov r9d, ecx
    and r9d, 7
    cmp ecx, 8
    jae .LBB10_4
    xor eax, eax
    xor ecx, ecx
    jmp .LBB10_6
.LBB10_1:
    xor eax, eax
    ret
.LBB10_4:
    and edx, -8
    xor eax, eax
    xor r8d, r8d
.LBB10_5:
    mov r10d, 255
    mov ecx, r8d
    shl r10d, cl
    lea ecx, [r8 + 8]
    add eax, r10d
    mov r8d, ecx
    cmp edx, ecx
    jne .LBB10_5
.LBB10_6:
    test r9d, r9d
    je .LBB10_8
.LBB10_7:
    mov edx, 1
    shl edx, cl
    inc ecx
    add eax, edx
    dec r9d
    jne .LBB10_7
.LBB10_8:
    ret
```
</details>

<br>

Using your brain is still better though:

```rust
#[unsafe(no_mangle)]
fn sum_smart(a: u32) -> u32 {
    (2 << (a - 1)) - 1
}
```

(5 instructions, ~11ns / characteristic iter)
```assembly
sum_smart:
    dec cl
    mov eax, 2
    shl eax, cl
    dec eax
    ret
```

#### Example 3

```rust
pub fn bilinear_sample(a00: f64, a10: f64, a01: f64, a11: f64, fx: f64, fy: f64) -> f64 {
    let inv_fx = 1.0 - fx;
    let inv_fy = 1.0 - fy;

    let w00 = inv_fx * inv_fy;
    let w10 = fx * inv_fy;
    let w01 = inv_fx * fy;
    let w11 = fx * fy;

    let mut result = 0.0f64;
    result += a00 * w00;
    result += a10 * w10;
    result += a01 * w01;
    result += a11 * w11;

    result
}
```

<details>
<summary>bilinear_sample_slow assembly (28 instructions, 3.1 ns / iter) (click to expand)</summary>

```assembly
bilinear_sample_slow:
	sub rsp, 40
	movaps xmmword ptr [rsp + 16], xmm7
	movaps xmmword ptr [rsp], xmm6
	movsd xmm4, qword ptr [rsp + 88]
	movsd xmm5, qword ptr [rsp + 80]
	movapd xmm6, xmm4
	unpcklpd xmm6, xmm5
	movapd xmm7, xmmword ptr [rip + __xmm@3ff00000000000003ff0000000000000]
	subpd xmm7, xmm6
	movapd xmm6, xmm7
	unpckhpd xmm6, xmm7
	mulsd xmm6, xmm7
	mulsd xmm5, xmm4
	mulsd xmm0, xmm6
	xorpd xmm4, xmm4
	addsd xmm0, xmm4
	mulpd xmm7, xmmword ptr [rsp + 80]
	unpcklpd xmm1, xmm2
	mulpd xmm7, xmm1
	addsd xmm0, xmm7
	unpckhpd xmm7, xmm7
	addsd xmm0, xmm7
	mulsd xmm5, xmm3
	addsd xmm0, xmm5
	movaps xmm6, xmmword ptr [rsp]
	movaps xmm7, xmmword ptr [rsp + 16]
	add rsp, 40
	ret
```
</details>

<details>
<summary>bilinear_sample_fast assembly (17 instructions, 2.7 ns / iter) (click to expand)</summary>

```assembly
bilinear_sample_fast:
	movsd xmm4, qword ptr [rsp + 40]
	movsd xmm5, qword ptr [rip + __real@3ff0000000000000]
	subsd xmm5, xmm4
	movddup xmm4, xmm4
	unpcklpd xmm3, xmm1
	mulpd xmm4, xmm3
	movddup xmm1, xmm5
	unpcklpd xmm2, xmm0
	mulpd xmm1, xmm2
	addpd xmm1, xmm4
	movapd xmm0, xmm1
	unpckhpd xmm0, xmm1
	subsd xmm1, xmm0
	mulsd xmm1, qword ptr [rsp + 48]
	addsd xmm1, xmm0
	movapd xmm0, xmm1
	ret
```
</details>

---

## Testing

I am actually not sure how to properly test what happens on overflow, since it is literally UB now. Tell me if you have any ideas 

`cargo test` / `cargo bench` will run all tests / benches - as usual

---

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for details.
