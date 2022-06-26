# von neumann assembler

## features
take custom microcode for instuction-set

## syntax

```
label ---
[name]:
---------

comment ---
; comment
-----------

instruction / macro-call ---
[ins] ...[parameters]
----------------------------

value ---
[value-literal]                 ; (string | number)
---------

macro ---
macro [name] ...[$args],
    [definition]
end
---------

include ---
%include "[path]"
-----------

```

```assembly
%include "other.asm"

%define TEST 10

macro add_to $a, $b, $result
    take $a
    add $b
    save $result
end

start:
    take    test        ; loads test into acc
    add     asdf        ; adds asdf to acc
    save    result      ; stores acc back into result

data:
    test: 110
    asdf: 10
    result: 0 0
eof:
```