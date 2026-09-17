pub const QUERY: &str = r#"
(function_definition
  name: (name) @name) @definition.function

(class_declaration
  name: (name) @name) @definition.class

(method_declaration
  name: (name) @name) @definition.method

(interface_declaration
  name: (name) @name) @definition.interface

(trait_declaration
  name: (name) @name) @definition.class

(enum_declaration
  name: (name) @name) @definition.enum

(namespace_definition
  name: (namespace_name) @name) @definition.module

(const_element
  (name) @name) @definition.constant
"#;
