org 0x100
jmp start

msg db "Hello, World", 0x0D, 0x0A, "$"

start:
    ; Set up data segment
    mov ax, cs
    mov ds, ax

    ; Print the message
    mov dx, msg      ; load offset of message
    call puts        ; call puts function to print the message

    call exit

; puts(DX: string)
puts:
    push ax
    push dx
    mov ah, 09h
    int 0x21
    pop dx
    pop ax
    ret

exit:
    mov ah, 0x4C
    xor al, al        ; return code 0
    int 0x21
    ret
    mov ah, 0x4C
    xor al, al        ; return code 0
    int 0x21