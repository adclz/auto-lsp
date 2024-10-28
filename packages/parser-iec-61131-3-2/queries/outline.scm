; Function

(function_declaration) @function
(derived_function_name) @name

; Function block

(function_block_declaration) @function
(derived_function_block_name) @name

; Variables

(function_var_decls) @variables.var
(var_declarations) @variables.constant

(input_declarations
    (input_declaration) @variable.input
)

(output_declarations 
    (var_init_decl) @variable.output
)

(var1_list
 (variable_name)
) @name

; Elementary Types

;(bit_string_type_name) @bit.strings
(signed_integer_type_name) @signed.integers
;(unsigned_integer_type_name) @unsigned.integers
;(real_type_name) @reals
;(date_type_name) @dates