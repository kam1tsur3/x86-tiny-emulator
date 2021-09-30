BITS 32
  call 0x17
  jmp 0xffff8400
  push ebp
  mov ebp, esp
  mov edx, dword[ebp+0x8]
  mov eax, dword[ebp+0xc]
  add eax,edx
  pop ebp
  ret
  push ebp
  mov ebp, esp
  push byte +0x5
  push byte +0x2
  call 0xa 
  add esp, byte +0x8
  leave
  ret
  