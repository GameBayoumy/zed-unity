; highlights.scm — C# syntax highlighting for Unity development
; Comprehensive Tree-sitter queries for C# language highlighting

; ============================================================================
; Comments
; ============================================================================
(comment) @comment

; XML documentation comments
(xml_comment) @comment.doc

; ============================================================================
; Literals
; ============================================================================
(integer_literal) @number
(real_literal) @number.float
(character_literal) @string
(string_literal) @string
(verbatim_string_literal) @string
(raw_string_literal) @string
(interpolated_string_expression) @string
(interpolation
  "{" @punctuation.special
  "}" @punctuation.special)
(boolean_literal) @constant.builtin
(null_literal) @constant.builtin

; ============================================================================
; Types
; ============================================================================
(predefined_type) @type.builtin

(implicit_type) @type.builtin

(type_parameter
  (identifier) @type.parameter)

(type_argument_list
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(type_parameter_list
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(generic_name
  (identifier) @type)

(nullable_type
  "?" @punctuation.special)

(array_type
  (identifier) @type)

(tuple_type) @type

; Attribute types
(attribute
  name: (identifier) @attribute)
(attribute
  name: (qualified_name
    (identifier) @attribute))

; ============================================================================
; Declarations
; ============================================================================
; Namespaces
(namespace_declaration
  name: (identifier) @namespace)
(namespace_declaration
  name: (qualified_name) @namespace)
(file_scoped_namespace_declaration
  name: (identifier) @namespace)
(file_scoped_namespace_declaration
  name: (qualified_name) @namespace)

; Using directives
(using_directive
  (identifier) @namespace)
(using_directive
  (qualified_name) @namespace)

; Class declarations
(class_declaration
  name: (identifier) @type.class)

; Struct declarations
(struct_declaration
  name: (identifier) @type.class)

; Interface declarations
(interface_declaration
  name: (identifier) @type.interface)

; Enum declarations
(enum_declaration
  name: (identifier) @type.enum)
(enum_member_declaration
  name: (identifier) @constant)

; Record declarations
(record_declaration
  name: (identifier) @type.class)

; Delegate declarations
(delegate_declaration
  name: (identifier) @type)

; ============================================================================
; Members
; ============================================================================
; Methods
(method_declaration
  name: (identifier) @function.method)

(local_function_statement
  name: (identifier) @function)

; Constructors
(constructor_declaration
  name: (identifier) @constructor)

; Properties
(property_declaration
  name: (identifier) @property)

; Fields
(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @variable.field)))

; Events
(event_declaration
  name: (identifier) @variable.field)
(event_field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @variable.field)))

; Indexers
(indexer_declaration
  (bracketed_parameter_list) @punctuation.bracket)

; ============================================================================
; Parameters and Variables
; ============================================================================
(parameter
  name: (identifier) @variable.parameter)

(variable_declarator
  (identifier) @variable)

(catch_declaration
  name: (identifier) @variable)

(for_each_statement
  left: (identifier) @variable)

; ============================================================================
; Expressions
; ============================================================================
; Method invocations
(invocation_expression
  function: (identifier) @function.call)
(invocation_expression
  function: (member_access_expression
    name: (identifier) @function.call))
(invocation_expression
  function: (generic_name
    (identifier) @function.call))

; Object creation
(object_creation_expression
  type: (identifier) @type)
(object_creation_expression
  type: (generic_name
    (identifier) @type))
(object_creation_expression
  type: (qualified_name
    (identifier) @type))

; Member access
(member_access_expression
  name: (identifier) @property)

; Element access
(element_access_expression
  (bracketed_argument_list
    "[" @punctuation.bracket
    "]" @punctuation.bracket))

; Lambda expressions
(lambda_expression
  (parameter_list
    (parameter
      name: (identifier) @variable.parameter)))

; Anonymous methods
(anonymous_method_expression) @function

; typeof, nameof, sizeof
(type_of_expression
  "typeof" @keyword.operator)
(size_of_expression
  "sizeof" @keyword.operator)

; Cast expressions
(cast_expression
  type: (identifier) @type)
(cast_expression
  type: (predefined_type) @type.builtin)

; Pattern matching
(declaration_pattern
  type: (identifier) @type
  (identifier) @variable)
(recursive_pattern) @type
(var_pattern
  "var" @type.builtin)

; ============================================================================
; Operators
; ============================================================================
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "|"
  "&"
  "~"
  "!"
  "<"
  ">"
  "="
  "?"
  ":"
  "+="
  "-="
  "*="
  "/="
  "%="
  "^="
  "|="
  "&="
  "<<"
  ">>"
  "<<="
  ">>="
  "=="
  "!="
  "<="
  ">="
  "&&"
  "||"
  "++"
  "--"
  "??"
  "??="
  "=>"
  ".."
] @operator

; ============================================================================
; Punctuation
; ============================================================================
[
  ";"
  "."
  ","
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; ============================================================================
; Keywords
; ============================================================================
; Declaration keywords
[
  "class"
  "struct"
  "interface"
  "enum"
  "record"
  "delegate"
  "namespace"
  "event"
] @keyword.type

; Modifier keywords
[
  "public"
  "private"
  "protected"
  "internal"
  "static"
  "readonly"
  "const"
  "volatile"
  "sealed"
  "abstract"
  "virtual"
  "override"
  "new"
  "partial"
  "extern"
  "unsafe"
  "async"
  "ref"
  "out"
  "in"
  "params"
  "required"
  "scoped"
  "file"
] @keyword.modifier

; Control flow keywords
[
  "if"
  "else"
  "switch"
  "case"
  "default"
  "while"
  "do"
  "for"
  "foreach"
  "break"
  "continue"
  "goto"
  "return"
  "yield"
  "throw"
  "try"
  "catch"
  "finally"
  "when"
] @keyword.control

; Other keywords
[
  "using"
  "var"
  "is"
  "as"
  "new"
  "this"
  "base"
  "where"
  "get"
  "set"
  "init"
  "add"
  "remove"
  "value"
  "await"
  "lock"
  "fixed"
  "checked"
  "unchecked"
  "stackalloc"
  "with"
  "global"
  "operator"
  "implicit"
  "explicit"
] @keyword

; ============================================================================
; Unity-specific highlighting
; ============================================================================
; Common Unity attributes
((attribute
  name: (identifier) @attribute.unity)
  (#any-of? @attribute.unity
    "SerializeField"
    "SerializeReference"
    "Header"
    "Tooltip"
    "Range"
    "Min"
    "Max"
    "TextArea"
    "Multiline"
    "ColorUsage"
    "GradientUsage"
    "Space"
    "HideInInspector"
    "NonSerialized"
    "FormerlySerializedAs"
    "RequireComponent"
    "DisallowMultipleComponent"
    "ExecuteInEditMode"
    "ExecuteAlways"
    "ContextMenu"
    "ContextMenuItem"
    "MenuItem"
    "AddComponentMenu"
    "CreateAssetMenu"
    "CustomEditor"
    "CustomPropertyDrawer"
    "CanEditMultipleObjects"
    "InitializeOnLoad"
    "InitializeOnLoadMethod"
    "RuntimeInitializeOnLoadMethod"
    "Preserve"
    "BurstCompile"
    "BurstDiscard"))

; Unity lifecycle methods
((method_declaration
  name: (identifier) @function.unity.lifecycle)
  (#any-of? @function.unity.lifecycle
    "Awake"
    "Start"
    "Update"
    "FixedUpdate"
    "LateUpdate"
    "OnEnable"
    "OnDisable"
    "OnDestroy"
    "OnApplicationQuit"
    "OnApplicationPause"
    "OnApplicationFocus"
    "OnBecameVisible"
    "OnBecameInvisible"
    "OnCollisionEnter"
    "OnCollisionStay"
    "OnCollisionExit"
    "OnCollisionEnter2D"
    "OnCollisionStay2D"
    "OnCollisionExit2D"
    "OnTriggerEnter"
    "OnTriggerStay"
    "OnTriggerExit"
    "OnTriggerEnter2D"
    "OnTriggerStay2D"
    "OnTriggerExit2D"
    "OnMouseDown"
    "OnMouseUp"
    "OnMouseUpAsButton"
    "OnMouseEnter"
    "OnMouseExit"
    "OnMouseOver"
    "OnMouseDrag"
    "OnGUI"
    "OnDrawGizmos"
    "OnDrawGizmosSelected"
    "OnValidate"
    "Reset"
    "OnAnimatorIK"
    "OnAnimatorMove"
    "OnAudioFilterRead"
    "OnControllerColliderHit"
    "OnJointBreak"
    "OnJointBreak2D"
    "OnParticleCollision"
    "OnParticleTrigger"
    "OnParticleSystemStopped"
    "OnParticleUpdateJobScheduled"
    "OnPostRender"
    "OnPreCull"
    "OnPreRender"
    "OnRenderImage"
    "OnRenderObject"
    "OnServerInitialized"
    "OnTransformChildrenChanged"
    "OnTransformParentChanged"
    "OnWillRenderObject"))

; ============================================================================
; Special identifiers
; ============================================================================
((identifier) @variable.builtin
  (#any-of? @variable.builtin "this" "base" "value"))

; ============================================================================
; Error handling
; ============================================================================
(ERROR) @error

