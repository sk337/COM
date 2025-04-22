org 0x100
jmp start

msg db "Hello, World", 0x0D, 0x0A, "$"

start:
    ; Print the message
    mov ah, 09h
    lea dx, msg
    int 21h

    mov al, 0x00
    call exit_with_code

exit_with_code:
    mov ah, 0x4C    ; DOS function 4Ch - terminate with return code
    int 0x21        ; call DOS interrupt