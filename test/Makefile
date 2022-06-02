all:
	riscv64-unknown-elf-gcc -ffreestanding -march=rv64id -mabi=lp64d -mcmodel=medany -c main.c -o main.o
	riscv64-unknown-elf-as entry.S -o entry.o
	riscv64-unknown-elf-ld -nostdlib -T link.ld entry.o main.o

objdump:
	riscv64-unknown-elf-objdump -D a.out