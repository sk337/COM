; Disassembled by DosDisassm
jmp _start ; label
; Start of string data
; db "Hello, World", 0x0D, 0x0A, "$"
dec ax
insb
insb
outsw
sub al,0x20
push di
outsw
jb short 0x0179
or ax,0x240A
; Start of program
_start: ; label
    mov ax,cs
    mov ds,ax
    mov dx,0x102
    call FUNC_0x11e ; function
    call FUNC_0x127 ; function
FUNC_0x11e: ; function
    push ax
    push dx
    mov ah,9
    int 0x21 ; DisplayString 0x09
    pop dx
    pop ax
    ret
FUNC_0x127: ; function
    mov ah,0x4C
    xor al,al
    int 0x21 ; TerminateWithCode 0x4c
