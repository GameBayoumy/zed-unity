; brackets.scm — C# bracket matching for code folding and navigation

; Block brackets
("{" @open "}" @close)

; Parentheses
("(" @open ")" @close)

; Square brackets
("[" @open "]" @close)

; Angle brackets for generics
("<" @open ">" @close)
