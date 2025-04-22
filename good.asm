; decompiled by ndisasm from  com/build/test.com
00000100  EB0F              jmp short 0x111
00000102  48                dec ax
00000103  656C              gs insb
00000105  6C                insb
00000106  6F                outsw
00000107  2C20              sub al,0x20
00000109  57                push di
0000010A  6F                outsw
0000010B  726C              jc 0x179
0000010D  640D0A24          fs or ax,0x240a
00000111  8CC8              mov ax,cs
00000113  8ED8              mov ds,ax
00000115  BA0201            mov dx,0x102
00000118  E80300            call 0x11e
0000011B  E80900            call 0x127
0000011E  50                push ax
0000011F  52                push dx
00000120  B409              mov ah,0x9
00000122  CD21              int 0x21
00000124  5A                pop dx
00000125  58                pop ax
00000126  C3                ret
00000127  B44C              mov ah,0x4c
00000129  30C0              xor al,al
0000012B  CD21              int 0x21
