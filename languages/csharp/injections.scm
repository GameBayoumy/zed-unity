; injections.scm — C# code injections for embedded languages

; Regex in string literals (common pattern)
; Note: This is a basic pattern and may need refinement

; SQL strings (common pattern in Unity/C# codebases)
; ((string_literal) @injection.content
;   (#match? @injection.content "^\"\\s*(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER)")
;   (#set! injection.language "sql"))

; JSON strings
; ((string_literal) @injection.content
;   (#match? @injection.content "^\"\\s*\\{")
;   (#set! injection.language "json"))

; XML documentation comments
(xml_comment) @injection.content
(#set! injection.language "xml")
