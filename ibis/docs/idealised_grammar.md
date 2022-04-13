# Idealised language EBNF

The follow can be used with ebnf tooling like https://matthijsgroen.github.io/ebnf2railroad/try-yourself.html

```ebnf
grammar = type;
type = {capability, " " }, structure, {" + ", tag};

tag=label;
capability = label;
structure = "*" | parenthesized | product | simple;

simple = type_name, [args];
args = "(", type, {",", type}, ")";

named = label, ": ", type;

parenthesized = "(", type, ")";

product = "{", (named | type), {",", (named | type)}, "}";
union = "(", type, {"|", type }, ")";

label = lower_letter , { letter | digit | "_" };
type_name = upper_letter , { letter | digit | "_" };

(*
  Basic components
  ----------------
  These are low level components, the small building blocks.
*)

letter = upper_letter | lower_letter ;

upper_letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
       | "H" | "I" | "J" | "K" | "L" | "M" | "N"
       | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
       | "V" | "W" | "X" | "Y" | "Z";
lower_letter = "a" | "b"
       | "c" | "d" | "e" | "f" | "g" | "h" | "i"
       | "j" | "k" | "l" | "m" | "n" | "o" | "p"
       | "q" | "r" | "s" | "t" | "u" | "v" | "w"
       | "x" | "y" | "z" ;

digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```
