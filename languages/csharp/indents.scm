; indents.scm — C# indentation rules

; Indent after opening braces
[
  (class_declaration)
  (struct_declaration)
  (interface_declaration)
  (enum_declaration)
  (record_declaration)
  (namespace_declaration)
  (method_declaration)
  (constructor_declaration)
  (property_declaration)
  (indexer_declaration)
  (event_declaration)
  (accessor_declaration)
  (block)
  (switch_body)
  (switch_section)
  (if_statement)
  (else_clause)
  (while_statement)
  (for_statement)
  (for_each_statement)
  (do_statement)
  (try_statement)
  (catch_clause)
  (finally_clause)
  (using_statement)
  (lock_statement)
  (fixed_statement)
  (checked_statement)
  (unchecked_statement)
  (unsafe_statement)
  (lambda_expression)
  (anonymous_method_expression)
  (anonymous_object_creation_expression)
  (initializer_expression)
  (array_creation_expression)
  (implicit_array_creation_expression)
  (object_creation_expression)
  (switch_expression)
  (with_expression)
] @indent

; Dedent at closing braces
[
  "}"
  "]"
  ")"
] @outdent
