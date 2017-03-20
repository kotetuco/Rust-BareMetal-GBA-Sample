.text

  .global _start
_start:
  b entry  @ Branch to entry()
loop:
  b loop
