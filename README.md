![minicomp](https://github.com/Viacheslav-Romanov/minicomp/actions/workflows/rust.yml/badge.svg)

## Task instructions

&nbsp;&nbsp;&nbsp;&nbsp;Write a program in Rust that takes a series of mathematical function definitions (using the common operations + - / *) as a command line argument, and turns them into machine code that operates on integers and computes the specified functions.

&nbsp;&nbsp;&nbsp;&nbsp;The program must output a ELF binary containing machine code targeting the computer architecture of your choice (e.g. ARM, RISC-V, x86, ...) 
Using LLVM or a compiler as a run-time dependency is not permitted.
Using other third-party libraries and tools such as parser generators and ELF manipulation libraries is permitted.
Submit the entire source code in a tarball or zipfile along with any comments. Do not submit any binaries.

&nbsp;&nbsp;&nbsp;&nbsp;Example use of such a program:

``
$ ./minicomp miniout.elf "avg(x, y) = (x + y)/2; quad(x, a, b, c) = a*x*x + b*x + c"
``

``
$ objdump -d miniout.elf
miniout.elf:     file format elf64-x86-64
Disassembly of section .text:
``
<pre>

0000000000000000 &ltavg&gt:

    0: 01 f7  add    %esi,%edi
    2: 89 f8  mov    %edi,%eax
    4: c1 e8 1f   shr    $0x1f,%eax
    7: 01 f8  add    %edi,%eax
    9: d1 f8  sar    %eax
    b: c3   ret
    c: 0f 1f 40 00    nopl   0x0(%rax)
 
0000000000000010 &ltquad&gt:

   10: 0f af f7 imul   %edi,%esi
   13: 01 d6    add    %edx,%esi
   15: 0f af f7 imul   %edi,%esi
   18: 8d 04 0e lea    (%rsi,%rcx,1),%eax
   1b: c3   ret 
</pre>

## Build instructions

If you want to build the binary with a debug information included use the following command in the root folder.

`cargo build`

If you want to assemble a release binary use the following command enstead.

`cargo build --release`

After the build process over the target binaries will be placed into ./target/debug and ./target/release folders accordingly.

## Execution

To run the program. Run the following command from the project's root folder.

`./target/release/minicomp miniout.elf "avg(x, y) = (x + y)/2; quad(x, a, b, c) = a*x*x + b*x + c"`

It'll generate the *miniout.elf* file. Which can be executed. 

<code>
-rwxr-xr-x  1 slava slava  764 Sep 10 16:44 miniout.elf
</code>

To run the generated file make it executable by using the `chmod +x miniout.elf` command on it.

Run it in the console. It should produce the following output.

<pre>
slava@DESKTOP-88SF0VS:~/minicomp$ ./miniout.elf
Z <- result
D <- result
</pre>

Where Z = 90 in ASCII codes which corresponds to the result of the `avg(x,y) = (100 + 80) / 2 = 90` with x = 100 and y = 90. And D = 68 in ASCII codes which corresponds to the result of the `quad(x, a, b, c) = (2*2*1 + 30*2 + 4) = 68` with x = 2, a = 1, b = 30 and c =4.

Cheers! 👈(ﾟヮﾟ👈)
