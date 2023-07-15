### complex_numbers.elf
#### Decompiled to C

![C_lang](https://img.shields.io/badge/C-00599C?style=for-the-badge&logo=c&logoColor=white)

**SHA-256:** _EC2B9564B95BB5E41EA7725C2D3333534348029C8790E85DB0296BEE2F5D139E_

**MD5:** _581421051205657BA04E6C80DBF32BB4_

**Name:** _complex_numbers.elf_

**Size:** _605_ (Kb)

Recompile:

    make

#### How was it done?

- #### Step 1. Machine code

At _AC90h_ displacement there is unknown function:

```
F2 0F 11 44 24 E0 F2 0F 11 4C 24 E8 F2 0F 11 54 24 
F0 F2 0F 11 5C 24 F8 F2 0F 58 C2 F2 0F 58 CB F2 0F 
11 4C 24 D8 F2 0F 11 44 24 C8 F2 0F 10 44 24 D8 F2 
0F 11 44 24 D0 F2 0F 10 44 24 C8 F2 0F 10 4C 24 D0
C3
```

- #### Step 2. Disassemble

To roughly understand, what it's doing, we need to disassemble it first. So the following output shows disassembled binary with _X86_ Intel assembler syntax:

```
movsd   [rsp+var1], xmm0
movsd   [rsp+var2], xmm1
movsd   [rsp+var3], xmm2
movsd   [rsp+var4], xmm3
addsd   xmm0, xmm2
addsd   xmm1, xmm3
movsd   [rsp+var5], xmm1
movsd   [rsp+var6], xmm0
movsd   xmm0, [rsp+var5]
movsd   [rsp+var7], xmm0
movsd   xmm0, [rsp+var6]
movsd   xmm1, [rsp+var7]
retn
```

Now we can clearly see, that it operates on xmm ( floating point ) registers, and sums them up. But it's still hard to tell what exactly is going on here.

- #### Step 3. Calling convention

To make the life of compiler easier, there are strict calling conventions for each function. They describe where arguments and return value will be stored. How to guess the calling convention of this function? Well, we need another function, like this, to see the repeating patterns. There is another one at address _ACE0h_. After dissasembling the beginning and the ending of that function, we get this output:

| Function | Beginning | Ending |
|:--------:|:----------|:----|
| _AC90h_  | ```movsd   [rsp+var1], xmm0```<br>```movsd   [rsp+var2], xmm1```<br>```movsd   [rsp+var3], xmm2```<br>```movsd   [rsp+var4], xmm3``` |```movsd   xmm0, [rsp+var6]```<br>```movsd   xmm1, [rsp+var7]```<br>```retn```|
| _ACE0h_ | ```sub     rsp, 98h```<br>```movsd   [rsp+var1], xmm0```<br>```movsd   [rsp+var2], xmm1```<br>```movsd   [rsp+var3], xmm2```<br>```movsd   [rsp+var4], xmm3```| ```movsd   xmm0, [rsp+var7]```<br>```movsd   xmm1, [rsp+var6]```<br>```retn```

There is a pattern! So now it's obvious, that arguments go into _xmm0_, _xmm1_, _xmm2_, _xmm3_, and return value goes into the pair <_xmm0_,_xmm1_>.

- #### Step 4. Any debugging information?

Sometimes, compilers with default settings put some debugging symbols into compiled binaries. There is one debugging symbol, which corresponds to address _AC90h_: 

```
complex_numbers::complex::Complex __cdecl _$LT$complex_numbers..complex..Complex$u20$as$u20$core..ops..arith..Add$GT$::add::had84bcab06465272(
        complex_numbers::complex::Complex self,
        complex_numbers::complex::Complex num)
```

So, it's a function, that adds two complex numbers together. And now, we can understand that xmm0/xmm1 are not different values, but parts of structure. Luckily, there is a definition for this structure in the file:

```
struct Imaginary {
    double i;
};

struct Complex {
    double a;
    struct Imaginary b;
}
```

Now we have the full signature of function, it's name, purpose, and calling convention:

```
struct Complex __usercall complex_numbers_add@<xmm0,xmm1>(struct Complex arg1@<xmm0,xmm1>, arg2@<xmm2,xmm3>);
```

- #### Step 5. Tracing

Let's see what it's doing, by tracing back it's execution.

- **Saving arguments**
First of all, it saves arguments ( maybe for later use ):

```
movsd   [rsp+var1], xmm0
movsd   [rsp+var2], xmm1
movsd   [rsp+var3], xmm2
movsd   [rsp+var4], xmm3
```

Let's name variables, according to arguments meaning:

```
movsd   [rsp+arg1.a], xmm0
movsd   [rsp+arg1.b.i], xmm1
movsd   [rsp+arg2.a], xmm2
movsd   [rsp+arg2.b.i], xmm3
```

- **Adding fields**
Then it adds same fields of structures:

```
addsd   xmm0, xmm2 ; arg1.a + arg2.a
addsd   xmm1, xmm3 ; arg1.b.i + arg2.b.i
```

- **Saving results**
It saves the results in temporary variables:

```
movsd   [rsp+var5], xmm1
movsd   [rsp+var6], xmm0
```

Let's name variables:

```
movsd   [rsp+b.i_sum], xmm1
movsd   [rsp+a_sum], xmm0
```

- **Storing return value**
After it, it shuffles around these results, until they'll be stored in <_xmm0_,_xmm1_>:

```
movsd   xmm0, [rsp+b.i_sum]
movsd   [rsp+b.i_sum2], xmm0
movsd   xmm0, [rsp+a_sum]
movsd   xmm1, [rsp+b.i_sum2]
retn
```

- #### Step 6. Decompiling

After all logic became clear, we can rewrite given assembly in C. Result:

``` C
/* ADDRESS: 0AC90h */
static inline Complex complex_numbers_add(const Complex num1, const Complex num2) {
    return (Complex) { .a = num1.a + num2.a, .b.i = num1.b.i + num2.b.i };
}
```