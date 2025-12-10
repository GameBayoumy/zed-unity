; outline.scm — C# document outline/symbols

; Namespaces
(namespace_declaration
  name: (_) @name) @item

(file_scoped_namespace_declaration
  name: (_) @name) @item

; Types
(class_declaration
  name: (identifier) @name) @item

(struct_declaration
  name: (identifier) @name) @item

(interface_declaration
  name: (identifier) @name) @item

(enum_declaration
  name: (identifier) @name) @item

(record_declaration
  name: (identifier) @name) @item

(delegate_declaration
  name: (identifier) @name) @item

; Members
(method_declaration
  name: (identifier) @name) @item

(constructor_declaration
  name: (identifier) @name) @item

(property_declaration
  name: (identifier) @name) @item

(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @name))) @item

(event_declaration
  name: (identifier) @name) @item

(event_field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @name))) @item

(indexer_declaration) @item

(operator_declaration) @item

; Enum members
(enum_member_declaration
  name: (identifier) @name) @item

; Local functions
(local_function_statement
  name: (identifier) @name) @item
