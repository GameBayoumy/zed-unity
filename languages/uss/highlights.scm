; USS (Unity Style Sheets) syntax highlighting
; Uses CSS tree-sitter grammar - USS is CSS-based

; Comments
(comment) @comment

; Selectors
(class_selector) @tag

(id_selector) @constant

(tag_name) @tag

(universal_selector) @tag

(attribute_selector
  (attribute_name) @attribute)

(child_selector
  ">" @operator)

(sibling_selector
  "~" @operator)

(adjacent_sibling_selector
  "+" @operator)

; Pseudo-classes (USS: hover, active, focus, disabled, enabled, checked, selected, root)
(pseudo_class_selector
  (class_name) @attribute)

(pseudo_element_selector
  (tag_name) @attribute)

; Property declarations
(declaration
  (property_name) @property)

; Property values
(plain_value) @variable

(integer_value) @number
(float_value) @number

(color_value) @constant

; Strings
(string_value) @string

; Functions (url(), rgb(), rgba(), var(), etc.)
(function_name) @function

; Important keyword
(important) @keyword

; At-rules
(at_rule
  (at_keyword) @keyword)

; Punctuation
"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
":" @punctuation.delimiter
";" @punctuation.delimiter
"," @punctuation.delimiter
