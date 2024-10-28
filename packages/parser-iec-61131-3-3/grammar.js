/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
    name: 'IEC61131_rev_3',

    extras: $ => [
        /\s/, // Whitespace
        $.comment,
        $.WS,
        $.EOL,
    ],

    precedences: $ => [
        // string type name > byte string access
        [$.string_type_name, $.s_byte_str_spec, $.d_byte_str_spec],
        // solve $.fb_body ambiguity since tree sitter could mistake a std_name with a il or st keyword
        // in this case, tree sitter will first check if a keyword exists and then check std func name
        [$.il_expr_operator, $.il_simple_operator, $.std_func_name],
        [$.il_label, $.access_name],

        [$.std_func_name, $.unary_expr],
        [$.unsigned_int, $.signed_int, $.int_literal,  $.bit_str_literal],
    ],

    conflicts: $ => [
        [$.signed_int],
        [$.signed_int, $.bit_str_literal],

        // ambiguity in PROGRAM declaration
        [$.class_name, $.namespace_name, $.variable_name],
        [$.class_name, $.namespace_name],
        [$.derived_func_name, $.method_name],
        [$.fb_name, $.derived_func_name, $.class_name],
        [$.variable_name, $.ref_name],
        [$.variable_name, $.fb_name],
        [$.variable_name, $.namespace_name],
        [$.variable_name, $.class_name],
        [$.fb_name, $.class_name],
        [$.derived_fb_name, $.class_type_name],
        [$.array_elem_init_value, $.primary_expr],
        [$.struct_elem_init, $.primary_expr],
        [$.string_type_name, $.var_spec],

        // local variable declarations
        [$.var_decls, $.loc_var_decls],
        [$.var_decls, $.loc_var_decls, $.loc_partly_var_decl],
        [$.var_decls, $.loc_partly_var_decl],

        [$.retain_var_decls, $.loc_var_decls, $.loc_partly_var_decl],
        [$.retain_var_decls, $.loc_partly_var_decl],
        [$.no_retain_var_decls, $.loc_var_decls, $.loc_partly_var_decl],
        [$.no_retain_var_decls, $.loc_partly_var_decl],

        // ambiguity in NAMESPACE declaration
        [$.simple_type_name, $.subrange_type_name, $.enum_type_name, $.array_type_name, $.struct_type_name, $.ref_type_name, $.class_type_name, $.interface_type_name],
        [$.simple_type_name, $.subrange_type_name, $.enum_type_name, $.array_type_name, $.struct_type_name, $.ref_type_name, $.interface_type_name],
        [$.simple_type_name, $.subrange_type_name, $.enum_type_name, $.array_type_name, $.struct_type_name, $.ref_type_name],
        [$.simple_type_name, $.subrange_type_name, $.enum_type_name, $.array_type_name, $.struct_type_name],
        [$.simple_type_name, $.array_type_name, $.struct_type_name, $.ref_type_name, $.derived_fb_name],
        [$.simple_type_name, $.array_type_name, $.struct_type_name, $.derived_fb_name],
        [$.simple_type_name, $.array_type_name, $.struct_type_name],
        [$.simple_type_name, $.interface_type_name],
        [$.elem_type_name, $.string_type_access],
        [$.enum_spec_init, $.enum_value_spec],
        [$.array_type_name, $.struct_type_name],
        [$.numeric_literal, $.enum_value_spec],

        // ambiguity in CONFIGURATION declaration
        [$.global_var_name, $.resource_name, $.prog_name],
        [$.global_var_name, $.resource_name],
        [$.global_var_name, $.enum_value],
    ],

    word: $ => $.identifier,

    rules: {
        // Source file declaration
        source_file : $ => repeat(
            choice(
                $.config_decl, // Declaration of CONFIGURATION and RESOURCE
                $.prog_decl, // Declaration of PROGRAM
                $.namespace_decl, // Declaration of NAMESPACE (including all other declarations)

                // To ensure inter-operability with rev 2, we also add all declarations without namespace
                $.fb_decl, // Declaration of FUNCTION_BLOCK
                $.func_decl, // Declaration of FUNCTION
                $.enum_type_decl, // Declaration of ENUM
                $.data_type_decl, // Declaration of TYPE
            )
        ),

        // Table 3 - Comments 

        comment: $ => choice(
            seq('//', repeat(/[^(\n|\r)]*/), optional('\r'), '\n'),
            seq('(*', repeat(choice(/[^*]/, /\*[^)]/)), '*)'),
            seq('/*', repeat(choice(/[^*]/, /\*[^/]/)), '*/')
        ),

        WS: $ => token(choice('\t', '\r', '\n')),
        EOL: $ => token("\n"),

        // Table 4 - Pragma 

        pragma: $ => seq('{', repeat(choice(/[^}]/, /}[^}]/)), '}'),

        // Table 5 - Numeric literal

        constant: $ => choice(
            $.numeric_literal,
            $.char_literal,
            $.time_literal,
            $.bit_str_literal,
            $.bool_literal
        ),

        numeric_literal: $ => choice(
            $.int_literal,
            $.real_literal
        ),

        int_literal: $ => seq(
            optional(seq($.int_type_name, '#')),
            choice(
                $.signed_int,
                $.binary_int,
                $.octal_int,
                $.hex_int
            )
        ),

        unsigned_int: $ => seq(
            $.digit,
            repeat(seq(optional('_',), $.digit))
        ),

        signed_int: $ => seq(
            optional(choice('+', '-')),
            $.unsigned_int
        ),

        binary_int: $ => seq(
            '2#',
            $.bit,
            repeat1(seq(optional('_'), $.bit))
        ),

        octal_int: $ => seq(
            '8#',
            $.octal_digit,
            repeat1(seq(optional('_'), $.octal_digit))
        ),

        hex_int: $ => seq(
            '16#',
            $.hex_digit,
            repeat1(seq(optional('_'), $.hex_digit))
        ),

        real_literal: $ => seq(
            optional(seq($.real_type_name, '#')),
            $.signed_int,
            '.',
            $.unsigned_int,
            optional(seq('E', $.signed_int))
        ),

        bit_str_literal: $ => seq(
            optional(seq($.multibits_type_name, '#')),
            choice(
                $.unsigned_int,
                $.binary_int,
                $.octal_int,
                $.hex_int
            )
        ),

        bool_literal: $ => seq(
            optional(seq($.bool_type_name, '#')),
            token(choice('0', '1', 'TRUE', 'FALSE'))
        ),

        // Table 6 - Character String literals
        // Table 7 - Two-character combinations in character strings

        char_literal: $ => seq(
            optional('STRING#'),
            $.char_str
        ),

        char_str: $ => choice(
            $.s_byte_char_str,
            $.d_byte_char_str
        ),

        s_byte_char_str: $ => seq(
            "'",
            $.s_byte_char_value,
            "'"
        ),

        d_byte_char_str: $ => seq(
            '"',
            $.d_byte_char_value,
            '"'
        ),

        s_byte_char_value: $ => choice(
            $.common_char_value,
            '$\'',
            '"',
            seq('$', $.hex_digit, $.hex_digit)
        ),

        d_byte_char_value: $ => choice(
            $.common_char_value,
            "'",
            '$"',
            seq('$', $.hex_digit, $.hex_digit, $.hex_digit, $.hex_digit)
        ),

        common_char_value: $ => choice(
            ' ',
            '!',
            '#',
            '%',
            '&',
            ...Array.from({ length: 11 }, (_, i) => String.fromCharCode(40 + i)), // '('..'/'
            ...Array.from({ length: 10 }, (_, i) => String.fromCharCode(48 + i)), // '0'..'9'
            ...Array.from({ length: 7 }, (_, i) => String.fromCharCode(58 + i)),  // ':'..'@'
            ...Array.from({ length: 26 }, (_, i) => String.fromCharCode(65 + i)), // 'A'..'Z'
            ...Array.from({ length: 6 }, (_, i) => String.fromCharCode(91 + i)),  // '['..'`'
            ...Array.from({ length: 26 }, (_, i) => String.fromCharCode(97 + i)), // 'a'..'z'
            ...Array.from({ length: 4 }, (_, i) => String.fromCharCode(123 + i)), // '{'..'~'
            '$$',
            '$L',
            '$N',
            '$P',
            '$R',
            '$T'
        ),

        // Table 8 - Duration literals
        // Table 9 – Date and time of day literals 

        time_literal: $ => choice(
            $.duration,
            $.time_of_day,
            $.date,
            $.date_and_time
        ),

        duration: $ => seq(
            choice($.time_type_name, 'T', 'LT'),
            '#',
            optional(choice('+', '-')),
            $.interval
        ),

        fix_point: $ => prec.left(seq(
            $.unsigned_int,
            optional(seq('.', $.unsigned_int))
        )),

        interval: $ => choice(
            $.days,
            $.hours,
            $.minutes,
            $.seconds,
            $.milliseconds,
            $.microseconds,
            $.nanoseconds
        ),

        days: $ => choice(
            seq($.fix_point, 'd'),
            seq(
                $.unsigned_int, 'd', optional('_'),
                optional($.hours)
            )
        ),

        hours: $ => choice(
            seq($.fix_point, 'h'),
            seq(
                $.unsigned_int, 'h', optional('_'),
                optional($.minutes)
            )
        ),

        minutes: $ => choice(
            seq($.fix_point, 'm'),
            seq(
                $.unsigned_int, 'm', optional('_'),
                optional($.seconds)
            )
        ),

        seconds: $ => choice(
            seq($.fix_point, 's'),
            seq(
                $.unsigned_int, 's', optional('_'),
                optional($.milliseconds)
            )
        ),

        milliseconds: $ => choice(
            seq($.fix_point, 'ms'),
            seq(
                $.unsigned_int, 'ms', optional('_'),
                optional($.microseconds)
            )
        ),

        microseconds: $ => choice(
            seq($.fix_point, 'us'),
            seq(
                $.unsigned_int, 'us', optional('_'),
                optional($.nanoseconds)
            )
        ),

        nanoseconds: $ => seq(
            $.fix_point,
            'ns'
        ),

        time_of_day: $ => seq(
            choice($.tod_type_name, 'LTIME_OF_DAY'),
            '#',
            $.daytime
        ),

        daytime: $ => seq(
            $.day_hour,
            ':',
            $.day_minute,
            ':',
            $.day_second
        ),

        day_hour: $ => $.unsigned_int,

        day_minute: $ => $.unsigned_int,

        day_second: $ => $.fix_point,

        date: $ => seq(
            choice($.date_type_name, 'D', 'LD'),
            '#',
            $.date_literal
        ),

        date_literal: $ => seq(
            $.year,
            '-',
            $.month,
            '-',
            $.day
        ),

        year: $ => $.unsigned_int,

        month: $ => $.unsigned_int,

        day: $ => $.unsigned_int,

        date_and_time: $ => seq(
            choice($.dt_type_name, 'LDATE_AND_TIME'),
            '#',
            $.date_literal,
            '-',
            $.daytime
        ),

        // Table 10 - Elementary data types

        data_type_access: $ => choice(
            $.elem_type_name,
            $.derived_type_access
        ),

        elem_type_name: $ => choice(
            $.numeric_type_name,
            $.bit_str_type_name,
            $.string_type_name,
            $.date_type_name,
            $.time_type_name,
        ),

        numeric_type_name: $ => choice(
            $.int_type_name,
            $.real_type_name
        ),

        int_type_name: $ => choice(
            $.sign_int_type_name,
            $.unsign_int_type_name
        ),

        sign_int_type_name: $ => choice(
            'SINT',
            'INT',
            'DINT',
            'LINT'
        ),

        unsign_int_type_name: $ => choice(
            'USINT',
            'UINT',
            'UDINT',
            'ULINT'
        ),

        real_type_name: $ => choice(
            'REAL',
            'LREAL'
        ),

        string_type_name: $ => choice(
            seq('STRING', optional(seq('[', $.unsigned_int, ']'))),
            seq('WSTRING', optional(seq('[', $.unsigned_int, ']')))
        ),

        time_type_name: $ => choice(
            'TIME',
            'LTIME'
        ),

        date_type_name: $ => choice(
            'DATE',
            'LDATE'
        ),

        tod_type_name: $ => choice(
            'TIME_OF_DAY',
            'TOD',
            'LTOD'
        ),

        dt_type_name: $ => choice(
            'DATE_AND_TIME',
            'DT',
            'LDT'
        ),

        bit_str_type_name: $ => choice(
            $.bool_type_name,
            $.multibits_type_name
        ),

        bool_type_name: $ => 'BOOL',

        multibits_type_name: $ => choice(
            'BYTE',
            'WORD',
            'DWORD',
            'LWORD'
        ),

        // Table 11 - Declaration of user-defined data types and initialization

        derived_type_access: $ => choice(
            $.single_elem_type_access,
            $.array_type_access,
            $.struct_type_access,
            $.string_type_access,
            $.class_type_access,
            $.ref_type_access,
            $.interface_type_access
        ),

        string_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.string_type_name
        ),

        single_elem_type_access: $ => choice(
            $.simple_type_access,
            $.subrange_type_access,
            $.enum_type_access
        ),

        simple_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.simple_type_name
        ),

        subrange_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.subrange_type_name
        ),

        enum_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.enum_type_name
        ),

        array_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.array_type_name
        ),

        struct_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.struct_type_name
        ),

        simple_type_name: $ => $.identifier,

        subrange_type_name: $ => $.identifier,

        enum_type_name: $ => $.identifier,

        array_type_name: $ => $.identifier,

        struct_type_name: $ => $.identifier,

        data_type_decl: $ => seq(
            'TYPE',
            repeat1(seq($.type_decl, ';')),
            'END_TYPE'
        ),

        type_decl: $ => choice(
            $.simple_type_decl,
            $.subrange_type_decl,
            $.enum_type_decl,
            $.array_type_decl,
            $.struct_type_decl,
            $.str_type_decl,
            $.ref_type_decl
        ),

        simple_type_decl: $ => seq(
            $.simple_type_name,
            ':',
            $.simple_spec_init
        ),

        simple_spec_init: $ => seq(
            $.simple_spec,
            optional(seq(':=', $.constant_expr))
        ),

        simple_spec: $ => choice(
            $.elem_type_name,
            $.simple_type_access
        ),

        subrange_type_decl: $ => seq(
            $.subrange_type_name,
            ':',
            $.subrange_spec_init
        ),

        subrange_spec_init: $ => seq(
            $.subrange_spec,
            optional(seq(':=', $.signed_int))
        ),

        subrange_spec: $ => choice(
            seq($.int_type_name, '(', $.subrange, ')'),
            $.subrange_type_access
        ),

        subrange: $ => seq(
            $.constant_expr,
            '..',
            $.constant_expr
        ),

        enum_type_decl: $ => seq(
            $.enum_type_name,
            ':',
            choice(seq(optional($.elem_type_name), $.named_spec_init), $.enum_spec_init)
        ),

        named_spec_init: $ => seq(
            '(',
            $.enum_value_spec,
            repeat(seq(',', $.enum_value_spec)),
            ')',
            optional(seq(':=', $.enum_value))
        ),

        enum_spec_init: $ => seq(
            choice(
                seq(
                    '(',
                    $.identifier,
                    repeat(seq(',', $.identifier)),
                    ')'
                ),
                $.enum_type_access
            ),
            optional(seq(':=', $.enum_value))
        ),

        enum_value_spec: $ => seq(
            $.identifier,
            optional(seq(
                ':=',
                choice($.int_literal, $.constant_expr)
            ))
        ),

        enum_value: $ => seq(
            optional(seq($.enum_type_name, '#')),
            $.identifier
        ),

        array_type_decl: $ => seq(
            $.array_type_name,
            ':',
            $.array_spec_init
        ),

        array_spec_init: $ => seq(
            $.array_spec,
            optional(seq(':=', $.array_init))
        ),

        array_spec: $ => choice(
            $.array_type_access,
            seq('ARRAY', '[', $.subrange, repeat(seq(',', $.subrange)), ']', 'OF', $.data_type_access)
        ),

        array_init: $ => seq(
            '[',
            $.array_elem_init,
            repeat(seq(',', $.array_elem_init)),
            ']'
        ),

        array_elem_init: $ => choice(
            $.array_elem_init_value,
            seq($.unsigned_int, '(', optional($.array_elem_init_value), ')')
        ),

        array_elem_init_value: $ => choice(
            $.constant_expr,
            $.enum_value,
            $.struct_init,
            $.array_init
        ),

        struct_type_decl: $ => seq(
            $.struct_type_name,
            ':',
            $.struct_spec
        ),

        struct_spec: $ => choice(
            $.struct_decl,
            $.struct_spec_init
        ),

        struct_spec_init: $ => seq(
            $.struct_type_access,
            optional(seq(':=', $.struct_init))
        ),

        struct_decl: $ => seq(
            'STRUCT',
            optional('OVERLAP'),
            repeat1(seq($.struct_elem_decl, ';')),
            'END_STRUCT'
        ),

        struct_elem_decl: $ => seq(
            $.struct_elem_name,
            optional(seq(
                $.located_at,
                optional($.multibit_part_access)
            )),
            ':',
            choice(
                $.simple_spec_init,
                $.subrange_spec_init,
                $.enum_spec_init,
                $.array_spec_init,
                $.struct_spec_init
            )
        ),

        struct_elem_name: $ => $.identifier,

        struct_init: $ => seq(
            '(',
            $.struct_elem_init,
            repeat(seq(',', $.struct_elem_init)),
            ')'
        ),

        struct_elem_init: $ => seq(
            $.struct_elem_name,
            ':=',
            choice($.constant_expr, $.enum_value, $.array_init, $.struct_init, $.ref_value)
        ),

        str_type_decl: $ => seq(
            $.string_type_name,
            ':',
            $.string_type_name,
            optional(seq(':=', $.char_str))
        ),

        // Table 16 - Directly represented variables 

        direct_variable: $ => prec.left(seq(
            '%',
            choice('I', 'Q', 'M'),
            optional(choice('X', 'B', 'W', 'D', 'L')),
            $.unsigned_int,
            repeat(seq('.', $.unsigned_int))
        )),

        // Table 12 - Reference operations 

        ref_type_decl: $ => seq(
            $.ref_type_name,
            ':',
            $.ref_spec_init
        ),

        ref_spec_init: $ => seq(
            $.ref_spec,
            optional(seq(':=', $.ref_value))
        ),

        ref_spec: $ => seq(
            'REF_TO',
            $.data_type_access
        ),

        ref_type_name: $ => $.identifier,

        ref_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.ref_type_name
        ),

        ref_name: $ => $.identifier,

        ref_value: $ => choice(
            $.ref_addr,
            'NULL'
        ),

        ref_addr: $ => seq(
            'REF',
            '(',
            choice($.symbolic_variable, $.fb_instance_name, $.class_instance_name),
            ')'
        ),

        ref_assign: $ => seq(
            $.ref_name,
            ':=',
            choice($.ref_name, $.ref_deref, $.ref_value)
        ),

        ref_deref: $ => (
            $.ref_name,
            '^'
        ),

        // Table 13 - Declaration of variables/Table 14 – Initialization of variables 

        variable: $ => choice($.direct_variable, $.symbolic_variable),

        symbolic_variable: $ => prec.right(seq(
            optional(choice(
                seq('THIS', '.'),
                repeat1(seq($.namespace_name, '.'))
            )),
            choice($.var_access, $.multi_elem_var)
        )),

        var_access: $ => choice($.variable_name, $.ref_deref),

        variable_name: $ => $.identifier,

        multi_elem_var: $ => prec.left(seq(
            $.var_access,
            repeat1(choice($.subscript_list, $.struct_variable))
        )),

        subscript_list: $ => seq(
            '[',
            $.subscript,
            repeat(seq(',', $.subscript)),
            ']'
        ),

        subscript: $ => $.expression,

        struct_variable: $ => seq(
            '.',
            $.struct_elem_select
        ),

        struct_elem_select: $ => $.var_access,

        input_decls: $ => seq(
            'VAR_INPUT',
            optional(choice('RETAIN', 'NON_RETAIN')),
            repeat(seq($.input_decl, ';')),
            'END_VAR'
        ),

        input_decl: $ => choice(
            $.var_decl_init,
            $.edge_decl,
            $.array_conform_decl
        ),

        edge_decl: $ => seq(
            $.variable_list,
            ':',
            'BOOL',
            choice('R_EDGE', 'F_EDGE')
        ),

        var_decl_init: $ => choice(
            seq(
                $.variable_list,
                ':',
                choice($.simple_spec_init, $.str_var_decl, $.ref_spec_init)
            ),
            $.array_var_decl_init,
            $.struct_var_decl_init,
            $.fb_decl_init,
            $.interface_spec_init
        ),

        ref_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.ref_spec
        ),

        interface_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.interface_type_access
        ),

        variable_list: $ => seq(
            $.variable_name,
            repeat(seq(',', $.variable_name))
        ),

        array_var_decl_init: $ => seq(
            $.variable_list,
            ':',
            $.array_spec_init
        ),

        //todo: check if this is correct
        array_conformand: $ => seq(
            'ARRAY',
            '[',
            '*',
            repeat(seq(',', '*')),
            ']',
            'OF',
            $.data_type_access
        ),

        array_conform_decl: $ => seq(
            $.variable_list,
            ':',
            $.array_conformand
        ),

        struct_var_decl_init: $ => seq(
            $.variable_list,
            ':',
            $.struct_spec_init
        ),

        fb_decl_no_init: $ => seq(
            $.fb_name,
            repeat(seq(',', $.fb_name)),
            ':',
            $.fb_type_access
        ),

        fb_decl_init: $ => seq(
            $.fb_decl_no_init,
            optional(seq(':=', $.struct_init))
        ),

        fb_name: $ => $.identifier,

        fb_instance_name: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.fb_name,
            '^' //check
        ),

        output_decls: $ => seq(
            'VAR_OUTPUT',
            optional(choice('RETAIN', 'NON_RETAIN')),
            repeat(seq($.output_decl, ';')),
            'END_VAR'
        ),

        output_decl: $ => choice(
            $.var_decl_init,
            $.array_conform_decl
        ),

        in_out_decls: $ => seq(
            'VAR_IN_OUT',
            repeat(seq($.in_out_var_decl, ';')),
            'END_VAR'
        ),

        in_out_var_decl: $ => choice(
            $.var_decl,
            $.array_conform_decl,
            $.fb_decl_no_init
        ),

        var_decl: $ => seq(
            $.variable_list,
            ':',
            choice($.simple_spec, $.str_var_decl, $.array_var_decl, $.struct_var_decl)
        ),

        array_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.array_spec
        ),

        struct_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.struct_type_access
        ),

        var_decls: $ => seq(
            'VAR',
            optional('CONSTANT'),
            optional($.access_spec),
            repeat(seq($.var_decl_init, ';')),
            'END_VAR'
        ),

        retain_var_decls: $ => seq(
            'VAR',
            'RETAIN',
            optional($.access_spec),
            repeat(seq($.var_decl_init, ';')),
            'END_VAR'
        ),

        loc_var_decls: $ => seq(
            'VAR',
            optional(choice('CONSTANT', 'RETAIN', 'NON_RETAIN')),
            repeat(seq($.loc_var_decl, ';')),
            'END_VAR'
        ),

        loc_var_decl: $ => seq(
            optional($.variable_name),
            $.located_at,
            ':',
            $.loc_var_spec_init
        ),

        temp_var_decls: $ => seq(
            'VAR_TEMP',
            repeat(seq(choice($.var_decl, $.ref_var_decl, $.interface_var_decl), ';')),
            'END_VAR'
        ),

        external_var_decls: $ => seq(
            'VAR_EXTERNAL',
            optional('CONSTANT'),
            repeat(seq($.external_decl, ';')),
            'END_VAR'
        ),

        external_decl: $ => seq(
            $.global_var_name,
            ':',
            choice($.simple_spec, $.array_spec, $.struct_type_access, $.fb_type_access, $.ref_type_access),
        ),

        global_var_name: $ => $.identifier,

        global_var_decls: $ => seq(
            'VAR_GLOBAL',
            optional(choice('CONSTANT', 'RETAIN')),
            repeat(seq($.global_var_decl, ';')),
            'END_VAR'
        ),

        global_var_decl: $ => seq(
            $.global_var_spec,
            ':',
            choice($.loc_var_spec_init, $.fb_type_access)
        ),

        global_var_spec: $ => choice(
            seq(
                $.global_var_name,
                repeat(seq(',', $.global_var_name))
            ),
            seq(
                $.global_var_name,
                $.located_at
            )
        ),

        loc_var_spec_init: $ => choice(
            $.simple_spec_init,
            $.array_spec_init,
            $.struct_spec_init,
            $.s_byte_str_spec,
            $.d_byte_str_spec,
        ),

        located_at: $ => seq(
            'AT',
            $.direct_variable
        ),

        str_var_decl: $ => choice(
            $.s_byte_str_var_decl,
            $.d_byte_str_var_decl
        ),

        s_byte_str_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.s_byte_str_spec
        ),

        s_byte_str_spec: $ => seq(
            'STRING',
            optional(seq('[', $.unsigned_int, ']')),
            optional(seq(':=', $.s_byte_char_str))
        ),

        d_byte_str_var_decl: $ => seq(
            $.variable_list,
            ':',
            $.d_byte_str_spec
        ),

        d_byte_str_spec: $ => seq(
            'WSTRING',
            optional(seq('[', $.unsigned_int, ']')),
            optional(seq(':=', $.d_byte_char_str))
        ),

        loc_partly_var_decl: $ => seq(
            'VAR',
            optional(choice('RETAIN', 'NON_RETAIN')),
            repeat($.loc_partly_var),
            'END_VAR'
        ),

        loc_partly_var: $ => seq(
            $.variable_name,
            'AT',
            '%',
            choice('I', 'Q', 'M'),
            '*',
            ':',
            $.var_spec,
            ';'
        ),

        var_spec: $ => choice(
            $.simple_spec,
            $.array_spec,
            $.struct_type_access,
            seq(choice('STRING', 'WSTRING'), optional(seq('[', $.unsigned_int, ']'))),
        ),

        // Table 19 - Function declaration

        func_name: $ => choice($.std_func_name, $.derived_func_name),

        func_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.func_name
        ),

        std_func_name: $ => choice(
            'TRUNC', 'ABS', 'SQRT', 'LN', 'LOG', 'EXP',
            'SIN', 'COS', 'TAN', 'ASIN', 'ACOS', 'ATAN', 'ATAN2',
            'ADD', 'SUB', 'MUL', 'DIV', 'MOD', 'EXPT', 'MOVE',
            'SHL', 'SHR', 'ROL', 'ROR',
            'AND', 'OR', 'XOR', 'NOT',
            'SEL', 'MAX', 'MIN', 'LIMIT', 'MUX',
            'GT', 'GE', 'EQ', 'LE', 'LT', 'NE',
            'LEN', 'LEFT', 'RIGHT', 'MID', 'CONCAT', 'INSERT', 'DELETE', 'REPLACE', 'FIND'
            // incomplete list ?
        ),

        derived_func_name: $ => $.identifier,

        func_decl: $ => seq(
            'FUNCTION',
            $.derived_func_name,
            optional(seq(':', $.data_type_access)),
            repeat($.using_directive),
            repeat(choice($.io_var_decls, $.func_var_decls, $.temp_var_decls)),
            $.func_body,
            'END_FUNCTION'
        ),

        io_var_decls: $ => choice(
            $.input_decls,
            $.output_decls,
            $.in_out_decls
        ),

        func_var_decls: $ => choice(
            $.external_var_decls,
            $.var_decls
        ),

        func_body: $ => choice(
            $.ladder_diagram,
            $.fb_diagram,
            $.instruction_list,
            $.stmt_list,
            $.other_languages
        ),

        // Table 40 – Function block type declaration
        // Table 41 - Function block instance declaration

        fb_type_name: $ => choice($.std_fb_name, $.derived_fb_name),

        fb_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.fb_type_name
        ),

        std_fb_name: $ => choice(
            'SR', 'RS', 'R_TRIG', 'F_TRIG', 'CTU', 'CTD', 'CTUD', 'TP', 'TON', 'TOF'
        ), // incomplete list ?

        derived_fb_name: $ => $.identifier,

        fb_decl: $ => seq(
            'FUNCTION_BLOCK',
            optional(choice('FINAL', 'ABSTRACT')),
            $.derived_fb_name,
            repeat($.using_directive),
            optional(seq("EXTENDS", choice($.fb_type_access, $.class_type_access))),
            optional(seq("IMPLEMENTS", $.interface_name_list)),
            repeat(choice($.fb_io_var_decls, $.func_var_decls, $.temp_var_decls, $.other_var_decls)),
            repeat($.method_decl),
            optional($.fb_body),
            "END_FUNCTION_BLOCK"
        ),

        fb_io_var_decls: $ => choice(
            $.fb_input_decls,
            $.fb_output_decls,
            $.in_out_decls
        ),

        fb_input_decls: $ => seq(
            'VAR_INPUT',
            optional(choice('RETAIN', 'NON_RETAIN')),
            repeat(seq($.fb_input_decl, ';')),
            'END_VAR'
        ),

        fb_input_decl: $ => choice(
            $.var_decl_init,
            $.edge_decl,
            $.array_conform_decl
        ),

        fb_output_decls: $ => seq(
            'VAR_OUTPUT',
            optional(choice('RETAIN', 'NON_RETAIN')),
            repeat(seq($.fb_output_decl, ';')),
            'END_VAR'
        ),

        fb_output_decl: $ => choice(
            $.var_decl_init,
            $.array_conform_decl
        ),

        other_var_decls: $ => choice(
            $.retain_var_decls,
            $.no_retain_var_decls,
            $.loc_partly_var_decl
        ),

        no_retain_var_decls: $ => seq(
            'VAR',
            'NON_RETAIN',
            optional($.access_spec),
            repeat(seq($.var_decl_init, ';')),
            'END_VAR'
        ),

        fb_body: $ => choice(
            $.SFC,
            $.ladder_diagram,
            $.fb_diagram,
            $.instruction_list,
            $.stmt_list,
            $.other_languages
        ),

        method_decl: $ => seq(
            'METHOD',
            $.access_spec,
            optional(choice('FINAL', 'ABSTRACT')),
            optional('OVERRIDE'),
            $.method_name,
            optional(seq(':', $.data_type_access)),
            repeat(choice($.io_var_decls, $.func_var_decls, $.temp_var_decls)),
            $.func_body,
            'END_METHOD'
        ),

        method_name: $ => $.identifier,

        // Table 48 - Class
        // Table 50 Textual call of methods – Formal and non-formal parameter list 

        class_decl: $ => seq(
            'CLASS',
            optional(choice('FINAL', 'ABSTRACT')),
            repeat(seq($.class_type_name, $.using_directive)),
            optional(seq("EXTENDS", $.class_type_access)),
            optional(seq("IMPLEMENTS", $.interface_name_list)),
            repeat(choice($.func_var_decls, $.other_var_decls)),
            repeat($.method_decl),
            'END_CLASS'
        ),

        class_type_name: $ => $.identifier,

        class_type_access: $ => (
            repeat(seq($.namespace_name, '.')),
            $.class_type_name
        ),

        class_name: $ => $.identifier,

        class_instance_name: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.class_name,
            repeat('^') //todo: check
        ),

        interface_decl: $ => seq(
            'INTERFACE',
            repeat(seq($.interface_type_name, $.using_directive)),
            optional(seq('EXTENDS', $.interface_name_list)),
            repeat($.method_prototype),
            'END_INTERFACE'
        ),

        method_prototype: $ => seq(
            'METHOD',
            $.method_name,
            optional(seq(':', $.data_type_access)),
            repeat($.io_var_decls),
            'END_METHOD'
        ),

        interface_spec_init: $ => seq(
            $.variable_list,
            optional(seq(':=', $.interface_value))
        ),

        interface_value: $ => choice(
            $.symbolic_variable,
            $.fb_instance_name,
            $.class_instance_name,
            'NULL'
        ),

        interface_name_list: $ => seq(
            $.interface_type_access,
            repeat(seq(',', $.interface_type_access))
        ),

        interface_type_name: $ => $.identifier,

        interface_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.interface_type_name
        ),

        interface_name: $ => $.identifier,

        access_spec: $ => choice('PUBLIC', 'PROTECTED', 'PRIVATE', 'INTERNAL'),

        // Table 47 - Program declaration 

        prog_decl: $ => seq(
            'PROGRAM',
            $.prog_type_name,
            repeat(choice($.io_var_decls, $.func_var_decls, $.temp_var_decls, $.other_var_decls, $.loc_var_decls, $.prog_access_decl)),
            $.fb_body,
            'END_PROGRAM'
        ),

        prog_type_name: $ => $.identifier,

        prog_type_access: $ => seq(
            repeat(seq($.namespace_name, '.')),
            $.prog_type_name
        ),

        prog_access_decls: $ => seq(
            'VAR_ACCESS',
            repeat(seq($.prog_access_decl, ';')),
            $.prog_type_name
        ),

        prog_access_decl: $ => seq(
            $.access_name,
            ':',
            $.symbolic_variable,
            optional($.multibit_part_access),
            ':',
            $.data_type_access,
            $.access_direction
        ),

        // Table 54 - 61 - Sequential Function Chart (SFC) 

        SFC: $ => repeat1($.SFC_network),

        SFC_network: $ => seq(
            $.initial_step,
            repeat(choice($.step, $.transition, $.action))
        ),

        initial_step: $ => seq(
            'INITIAL_STEP',
            $.step_name,
            ':',
            repeat(seq($.action_association, ';')),
            'END_STEP'
        ),

        step: $ => seq(
            'STEP',
            $.step_name,
            ':',
            repeat(seq($.action_association, ';')),
            'END_STEP'
        ),

        step_name: $ => $.identifier,

        action_association: $ => seq(
            $.action_name,
            '(',
            optional($.action_qualifier),
            repeat(seq($.indicator_name, ';')),
            ')'
        ),

        action_name: $ => $.identifier,

        action_qualifier: $ => choice(
            'N',
            'R',
            'S',
            'P',
            seq(
                choice('L', 'D', 'SD', 'DS', 'SL'),
                ',',
                $.action_time
            )
        ),

        action_time: $ => choice(
            $.duration,
            $.variable_name
        ),

        indicator_name: $ => $.variable_name,

        transition: $ => seq(
            'TRANSITION',
            optional($.transition_name),
            ':',
            optional(seq('(', 'PRIORITY', ':=', $.unsigned_int, ')')),
            'FROM',
            $.steps,
            'TO',
            $.steps,
            $.transition_cond,
            'END_TRANSITION'
        ),

        transition_name: $ => $.identifier,

        steps: $ => choice(
            $.step_name,
            seq('(', $.step_name, repeat1(seq(',', $.step_name)), ')')
        ),

        transition_cond: $ => choice(
            seq(':=', $.expression, ';'),
            seq(':', choice($.fbd_network, $.ld_rung),
                seq(':=', $.il_simple_inst)
            )
        ),

        action: $ => seq(
            'ACTION',
            $.action_name,
            ':',
            $.fb_body,
            'END_ACTION'
        ),

        // Table 62 - Configuration and resource declaration 

        config_name: $ => $.identifier,

        resource_type_name: $ => $.identifier,

        config_decl: $ => seq(
            'CONFIGURATION',
            $.config_name,
            optional($.global_var_decls),
            choice($.single_resource_decl, repeat1($.resource_decl)),
            'END_CONFIGURATION'
        ),

        resource_decl: $ => seq(
            'RESOURCE',
            $.resource_name,
            'ON',
            $.resource_type_name,
            optional($.global_var_decls),
            $.single_resource_decl,
            'END_RESOURCE'
        ),

        single_resource_decl: $ => seq(
            repeat(seq($.task_config, ';')),
            repeat1(seq($.prog_config, ';'))
        ),

        resource_name: $ => $.identifier,

        access_decls: $ => seq(
            'VAR_ACCESS',
            repeat(seq($.access_decl, ';')),
            'END_VAR'
        ),

        access_decl: $ => seq(
            $.access_name,
            ':',
            $.access_path,
            ':',
            optional(seq($.data_type_access, $.access_direction)),
        ),

        access_path: $ => choice(
            seq(optional(seq($.resource_name, '.')), $.direct_variable),
            seq(
                optional(seq($.resource_name, '.')),
                optional(seq($.prog_name, '.')),
                repeat(seq(choice($.fb_instance_name, $.class_instance_name), '.')),
                $.symbolic_variable
            )
        ),

        global_var_access: $ => seq(
            optional(seq($.resource_name, '.')),
            $.global_var_name,
            optional(seq('.', $.struct_elem_name))
        ),

        access_name: $ => $.identifier,

        prog_output_access: $ => seq(
            $.prog_name,
            '.',
            $.symbolic_variable
        ),

        prog_name: $ => $.identifier,

        access_direction: $ => choice('READ_WRITE', 'READ_ONLY'),

        task_config: $ => seq(
            'TASK',
            $.task_name,
            $.task_init
        ),

        task_name: $ => $.identifier,

        task_init: $ => seq(
            '(',
            optional(seq('SINGLE', ':=', $.data_source, ',')),
            optional(seq('INTERVAL', ':=', $.data_source, ',')),
            'PRIORITY', ':=', $.unsigned_int,
            ')'
        ),

        data_source: $ => choice(
            $.constant,
            $.global_var_access,
            $.prog_output_access,
            $.direct_variable
        ),

        prog_config: $ => seq(
            'PROGRAM',
            optional(choice('RETAIN', 'NON_RETAIN')),
            $.prog_name,
            optional(seq('WITH', $.task_name)),
            ':',
            $.prog_type_access,
            optional(seq('(', $.prog_conf_elems, ')'))
        ),

        prog_conf_elems: $ => seq(
            $.prog_conf_elem,
            repeat(seq(',', $.prog_conf_elem))
        ),

        prog_conf_elem: $ => choice(
            $.fb_task,
            $.prog_cnxn
        ),

        fb_task: $ => seq(
            $.fb_instance_name,
            'WITH',
            $.task_name
        ),

        prog_cnxn: $ => choice(
            seq($.symbolic_variable, ':=', $.prog_data_source),
            seq($.symbolic_variable, '=>', $.data_sink),
        ),

        prog_data_source: $ => choice(
            $.constant,
            $.enum_value,
            $.global_var_access,
            $.direct_variable
        ),

        data_sink: $ => choice(
            $.global_var_access,
            $.direct_variable
        ),

        config_init: $ => seq(
            'VAR_CONFIG',
            repeat(seq($.config_inst_init, ';')),
            'END_VAR'
        ),

        config_inst_init: $ => seq(
            $.resource_name, '.', $.prog_name, '.',
            repeat(seq(choice($.fb_instance_name, $.class_instance_name), '.')),
            choice(
                seq($.variable_name, optional($.located_at), ':', $.loc_var_spec_init),
                seq(
                    choice(
                        seq($.fb_instance_name, ':', $.fb_type_access),
                        seq($.class_instance_name, ':', $.class_type_access)
                    ),
                    ':=',
                    $.struct_init
                )
            )
        ),

        // Table 64 - Namespace 

        namespace_decl: $ => seq(
            'NAMESPACE',
            optional('INTERNAL'),
            $.namespace_h_name,
            repeat($.using_directive),
            $.namespace_elements,
            'END_NAMESPACE'
        ),

        namespace_elements: $ => repeat1(
            choice(
                $.data_type_decl,
                $.func_decl,
                $.fb_decl,
                $.class_decl,
                $.interface_decl,
                $.namespace_decl
            )
        ),

        namespace_h_name: $ => seq(
            $.namespace_name,
            repeat(seq('.', $.namespace_name))
        ),

        namespace_name: $ => $.identifier,

        using_directive: $ => seq(
            'USING',
            $.namespace_h_name,
            repeat(seq(',', $.namespace_h_name)),
            ';'
        ),

        pou_decl: $ => seq(
            repeat($.using_directive),
            repeat1(
                choice(
                    $.global_var_decls,
                    $.data_type_decl,
                    $.access_decls,
                    $.func_decl,
                    $.fb_decl,
                    $.class_decl,
                    $.interface_decl,
                    $.namespace_decl
                )
            )
        ),

        // Table 67 - 70 - Instruction List (il) 

        instruction_list: $ => seq(
            repeat1($.il_instruction),
        ),

        il_instruction: $ => prec.left(seq(
            optional(seq($.il_label, ':')),
            optional(choice(
                $.il_simple_operation,
                $.il_expr,
                $.il_jump_operation,
                $.il_invocation,
                $.il_formal_func_call,
                $.il_return_operator
            )),
            repeat1($.EOL)
        )),

        il_simple_inst: $ => choice(
            $.il_simple_operation,
            $.il_expr,
            $.il_formal_func_call,
        ),

        il_label: $ => $.identifier,

        il_simple_operation: $ => choice(
            seq($.il_simple_operator, optional($.il_operand)),
            seq($.func_access, optional($.il_operand_list))
        ),

        il_expr: $ => seq(
            $.il_expr_operator,
            '(',
            optional($.il_operand),
            repeat1($.EOL),
            optional($.il_simple_inst),
            ')'
        ),

        il_jump_operation: $ => seq(
            $.il_jump_operator,
            $.il_label
        ),

        il_invocation: $ => seq(
            $.il_call_operator,
            choice(
                seq(
                    choice(
                        $.fb_instance_name,
                        $.func_name,
                        $.method_name,
                        'THIS',
                        seq(
                            'THIS',
                            '.',
                            repeat(seq(choice($.fb_instance_name, $.class_instance_name), '.')),
                            $.method_name
                        )
                    ),
                    optional(seq(
                        '(',
                        choice(
                            seq(repeat1($.EOL), optional($.il_param_list)),
                            optional($.il_operand_list)
                        ),
                        ')'
                    ))
                ),
                seq('SUPER', '(', ')')
            )
        ),

        il_formal_func_call: $ => seq(
            $.func_access,
            '(',
            repeat1($.EOL),
            optional($.il_param_list),
            ')'
        ),

        il_operand: $ => choice(
            $.constant,
            $.enum_value,
            $.variable_access
        ),

        il_operand_list: $ => seq(
            $.il_operand,
            repeat(seq(',', $.il_operand))
        ),

        il_simple_inst_list: $ => seq(
            repeat1($.il_simple_instruction)
        ),

        il_simple_instruction: $ => seq(
            choice($.il_simple_operation, $.il_expr, $.il_formal_func_call),
            repeat1($.EOL)
        ),

        il_param_list: $ => seq(
            repeat($.il_param_inst),
            $.il_param_last_inst
        ),

        il_param_inst: $ => seq(
            choice($.il_param_assign, $.il_param_out_assign),
            ',',
            repeat1($.EOL)
        ),

        il_param_last_inst: $ => seq(
            choice($.il_param_assign, $.il_param_out_assign),
            repeat1($.EOL)
        ),

        il_param_assign: $ => seq(
            $.il_assignment,
            choice(
                seq(':=', $.il_operand),
                seq('(', repeat1($.EOL), $.il_simple_inst_list, ')')
            )
        ),

        il_param_out_assign: $ => seq(
            $.il_assign_out_operator,
            $.variable_access
        ),

        il_simple_operator: $ => choice(
            'LD', 'LDN', 'ST', 'STN', 'ST?', 'NOT', 'S', 'R',
            'S1', 'R1', 'CLK', 'CU', 'CD', 'PV',
            'IN', 'PT', $.il_expr_operator
        ),

        il_expr_operator: $ => choice(
            'AND', '&', 'OR', 'XOR', 'ANDN', '&N', 'ORN',
            'XORN', 'ADD', 'SUB', 'MUL', 'DIV',
            'MOD', 'GT', 'GE', 'EQ', 'LT', 'LE', 'NE'
        ),

        il_assignment: $ => seq(
            $.variable_name,
            ':='
        ),

        il_assign_out_operator: $ => seq(
            optional('NOT'),
            $.variable_name,
            '=>'
        ),

        il_call_operator: $ => choice(
            'CAL',
            'CALC',
            'CALCN'
        ),

        il_return_operator: $ => choice(
            'RT',
            'RETC',
            'RETCN',
        ),

        il_jump_operator: $ => choice(
            'JMP',
            'JMPC',
            'JMPCN'
        ),

        // Table 71 - 72 - Language Structured Text (ST) 

        expression: $ => seq(
            $.xor_expr,
            repeat(seq('OR', $.xor_expr))
        ),

        constant_expr: $ => $.expression,
        //todo: a constant expression must evaluate to a constant value at compile time 

        xor_expr: $ => seq(
            $.and_expr,
            repeat(seq('XOR', $.and_expr))
        ),

        and_expr: $ => seq(
            $.compare_expr,
            repeat(seq(choice('&', 'AND'), $.compare_expr))
        ),

        compare_expr: $ => seq(
            $.equ_expr,
            repeat(seq(choice('=', '<>'), $.equ_expr))
        ),

        equ_expr: $ => seq(
            $.add_expr,
            repeat(seq(choice('<', '>', '<=', '>='), $.add_expr))
        ),

        add_expr: $ => seq(
            $.term,
            repeat(seq(choice('+', '-'), $.term))
        ),

        term: $ => seq(
            $.power_expr,
            repeat(seq(choice('*', '/', seq('MOD', $.power_expr))))
        ),

        power_expr: $ => seq(
            $.unary_expr,
            repeat(seq('**', $.unary_expr))
        ),

        unary_expr: $ => seq(
            optional(choice('-', '+', 'NOT')),
            $.primary_expr
        ),

        primary_expr: $ => choice(
            $.constant,
            $.enum_value,
            $.variable_access,
            $.func_call,
            $.ref_value,
            seq('(', $.expression, ')')
        ),

        variable_access: $ => seq(
            $.variable,
            $.multibit_part_access
        ),

        multibit_part_access: $ => seq(
            '.',
            choice(
                $.unsigned_int,
                seq('%', optional(choice('X', 'B', 'W', 'D', 'L')), $.unsigned_int)
            )
        ),

        func_call: $ => seq(
            $.func_access,
            '(',
            optional(seq($.param_assign, repeat(seq(',', $.param_assign)))),
            ')'
        ),

        // note: IEC does not specify at least one occurence of a statement in a statement list
        // but since tree sitter does not support empty string, we have to add at least one statement.
        stmt_list: $ => prec.left(repeat1(
            seq(optional($.stmt), ';')
        )),

        stmt: $ => choice(
            $.assign_stmt,
            $.subprog_ctrl_stmt,
            $.selection_stmt,
            $.iteration_stmt
        ),

        assign_stmt: $ => choice(
            seq($.variable, ':=', $.expression),
            $.ref_assign,
            $.assignment_attempt
        ),

        assignment_attempt: $ => seq(
            choice($.ref_name, $.ref_deref),
            '?=',
            choice($.ref_name, $.ref_deref, $.ref_value)
        ),

        invocation: $ => seq(
            choice(
                $.fb_instance_name,
                $.method_name,
                'THIS',
                seq(
                    optional(seq('THIS', '.')),
                    repeat1(seq(choice($.fb_instance_name, $.class_instance_name), '.')),
                    $.method_name
                )
            ),
            '(',
            optional(seq($.param_assign, repeat(seq(',', $.param_assign)))),
            ')'
        ),

        subprog_ctrl_stmt: $ => choice(
            $.func_call,
            $.invocation,
            seq('SUPER', '(', ')'),
            'RETURN'
        ),

        param_assign: $ => choice(
           seq(optional(seq($.variable_name, ':=')), $.expression),
           $.ref_assign,
           seq(optional('NOT'), $.variable_name, '=>', $.variable)
        ),

        selection_stmt: $ => choice(
            $.if_stmt,
            $.case_stmt
        ),

        if_stmt: $ => seq(
            'IF',
            $.expression,
            'THEN',
            $.stmt_list,
            repeat(seq('ELSE IF', $.expression, 'THEN', $.stmt_list)),
            optional(seq('ELSE', $.stmt_list)),
            'END_IF'
        ),

        case_stmt: $ => seq(
            'CASE',
            $.expression,
            'OF',
            repeat1($.case_selection),
            optional(seq('ELSE', $.stmt_list)),
            'END_CASE'
        ),

        case_selection: $ => seq(
            $.case_list,
            ':',
            $.stmt_list
        ),

        case_list: $ => seq(
            $.case_list_elem,
            repeat(seq(',', $.case_list_elem))
        ),

        case_list_elem: $ => choice(
            $.subrange,
            $.constant_expr
        ),

        iteration_stmt: $ => choice(
            $.for_stmt,
            $.while_stmt,
            $.repeat_stmt,
            'EXIT',
            'CONTINUE'
        ),

        for_stmt: $ => seq(
            'FOR',
            $.control_variable,
            ':=',
            $.for_list,
            'DO',
            $.stmt_list,
            'END_FOR'
        ),

        control_variable: $ => $.identifier,

        for_list: $ => seq(
            $.expression,
            'TO',
            $.expression,
            optional(seq('BY', $.expression))
        ),

        while_stmt: $ => seq(
            'WHILE',
            $.expression,
            'DO',
            $.stmt_list,
            'END_WHILE'
        ),

        repeat_stmt: $ => seq(
            'REPEAT',
            $.stmt_list,
            'UNTIL',
            $.expression,
            'END_REPEAT'
        ),

        // Table 73 - 76 - Graphic languages elements 

        // note: IEC does not specify at least one occurence of a statement in a ladder diagram
        // but since tree sitter does not support empty string, we have to add at least one rung.
        ladder_diagram: $ => repeat1(
            $.ld_rung,
        ),

        ld_rung: $ => "todo_lad",

        // same
        fb_diagram: $ => repeat1(
            $.fbd_network,
        ),

        fbd_network: $ => "todo_fbd",

        other_languages: $ => "todo_other",

                // Table 1 - Character sets
        // Table 2 - Identifiers

        letter: $ => /[a-zA-Z_]/,
        digit: $ => /[0-9]/,
        bit: $ => /[01]/,
        octal_digit: $ => /[0-7]/,
        hex_digit: $ => /[0-9a-fA-F]/,
        identifier: $ =>  /[0-9a-zA-Z_]+/,
    }
})