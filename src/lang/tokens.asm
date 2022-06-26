%include "other.asm"

%define TEST 10

macro add_to! $a, $b, $result
    take $a
    add $b
    save $result
end

start:
    take    test        ; loads test into acc
    add     asdf        ; adds asdf to acc
    save    result      ; stores acc back into result

    add_to! test, asdf, result  ; macro version

data:
    test: 110
    asdf: 10
    result: 0 0
eof:
