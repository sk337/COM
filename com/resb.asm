org 0x100
jmp start

msg db "Hello, World", 0x0D, 0x0A, "$"
resb 0x100
start:
    ; Print the message
    mov ah, 09h
    lea dx, msg
    int 21h

    ; Exit with code 0
    mov ah, 0x4C
    int 0x21