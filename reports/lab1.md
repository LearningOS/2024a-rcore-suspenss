# lab1 

## 实现功能
实现了系统调用 `sys_task_info`，具体实现为，修改任务控制块，添加任务起始时间，任务所进行的系统调用表，其中起始时间为 `option` 可以避免多次修改。在系统调用 `sys_task_info` 中，首先拷贝一份当先任务的控制块，提取这些信息，然后构造一个 `TaskInfo` 赋值给参数 `ti` 即可。


## 问答题
### 第一题
以 qemu-riscv64 虚 拟 环 境 分 别 执 行 ch2b_bad_register.elf, ch2b_bad_instructions.elf, ch2b_bad_address.elf 输出如下
```
1 root@ubuntu:~/rCore/user/build/elf# qemu-riscv64 ch2b_bad_register.elf
2 Illegal instruction
3
4 root@ubuntu:~/rCore/user/build/elf# qemu-riscv64 ch2b_bad_address.elf
5 Segmentation fault
6
7 root@ubuntu:~/rCore/user/build/elf# qemu-riscv64 ch2b_bad_instructions.elf
8 Illegal instruction
SBI为[rustsbi] Implementation: RustSBI-QEMU Version 0.2.0-alpha.2
```
### 第二题
1. 引入 Task 抽象后，`__restore` 函数并不需要通过 `a0` 来传递数据，其除了控制应用 Trap 中断，同时还负责应用第一次执行时，将应用的寄存器组从内核栈恢复到用户栈。
2. 恢复保存的用户态的寄存器组到能够恢复到trap之前的用户态的计算状态，保证 trap 对应用的透明以及应用的正确运行。
3. x2 是栈指针，其保存与恢复是用 csrr 来处理的，x4 则是几乎不会用到的寄存器，无需处理
4. sp 为用户栈的栈指针，sscratch 为内核栈的栈指针
5. sret
6. 正好与问题4相反
7. call trap_handler

## 荣誉守则
1. 在完成本次实验的过程(含此前学习的过程)中，我曾分别与以下各位就(与本次实验相关的)
 以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容:无。
2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内 容: 无。
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以 上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不 会向他人(含此后各届同学)复制或公开我的实验代码，我有义务妥善保管好它们。 我提交 至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知 道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
