#include <stdint.h>

static volatile uint64_t* PRINT_OUT = (uint64_t *)0x1000;

void write_str(const char *str) {
    while(*str) {
        *PRINT_OUT = *str;
        str++;
    }
}

void _main() {
    write_str("Hello World from RiscV C\n");
    while(1) {}
}