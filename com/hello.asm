org 0x100
jmp start
msg db "Hello, World", 0x0D, 0x0A, "$"

start:
    mov ah, 09h
    mov dx, msg
    int 21h

    mov ah, 4Ch
    int 21h
