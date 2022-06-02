.text
.global _entry
_entry:
    la sp, stack_top
    j _main
    1:
        beqz x0, 1b

.data
stack_bottom:
    .skip 4096 * 10
stack_top:
