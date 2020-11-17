  .global _start
_start:
  auipc ra, 0
  jalr ra, 8(ra)
  lui ra, 0x10
  jalr ra, 0x94(ra)
  li a0, 1
  li a7, 93
  ecall
1: /* pc: 0x10094 */
  li a0, 0
  li a7, 93
  ecall
