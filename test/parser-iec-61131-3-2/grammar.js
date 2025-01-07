/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// Grammar for IEC 61131-2 standard
// From B.0 Programming model to B.1.7 Configuration elements

const B2 = require("./grammars/B2");
const B3 = require("./grammars/B3");

module.exports = grammar({
    name: "IEC61131_3_2",

    externals: $ => [
        $.field_selector,
    ],

    extras: $ => [
        /\s/, // Whitespace
        $.single_line_comment,
        $.multi_line_comment,
        //$.EOL,
        //$.WS
    ],

    precedences: $ => [
        [$.unary_operator, $.signed_integer],
        [$.variable_name, $.enumerated_value],
        [$.bit_string_literal, $.integer_literal],

        [$.boolean_literal, $.numeric_literal],

        // Conflicts between IL operators & fn names
        [$.il_expr_operator, $.standard_function_name],
        [$.il_simple_operator, $.standard_function_name],
        [$.unary_operator, $.standard_function_name],

        [$.bit_string_literal, $.signed_integer],
        [$.variable_name, $.fb_name],
    ],

    conflicts: $ => [
        // All rules below are ambiguous with each other and all refer to $.identifier rule.
        // Unfortunately, we can't set a precedence to distinguish them since they are not related.
        [$.simple_type_name,
        $.enumerated_type_name,
        $.derived_function_name,
        $.derived_function_block_name,
        $.program_type_name,
        $.configuration_name,
        $.resource_type_name,
        $.subrange_type_name,
        $.array_type_name,
        $.structure_type_name,
        $.string_type_name],

        [$.simple_type_name,
        $.subrange_type_name,
        $.enumerated_type_name,
        $.array_type_name,
        $.structure_type_name,
        $.string_type_name
        ],

        [$.enumerated_value, $.variable_name, $.global_var_name],
        [$.resource_name, $.variable_name, $.global_var_name],
        [$.global_var_name, $.resource_name],
        [$.global_var_name, $.resource_name, $.program_name],
        [$.variable_name, $.fb_name, $.program_name],

        [$.elementary_type_name, $.single_byte_string_spec],
        [$.double_byte_string_spec, $.elementary_type_name],
        [$.array_type_name, $.simple_type_name, $.structure_type_name, $.subrange_type_name],
        [$.array_type_name, $.derived_function_block_name, $.simple_type_name, $.structure_type_name, $.subrange_type_name],

        [$.fb_name, $.program_name, $.resource_name, $.variable_name],
        [$.elementary_type_name, $.var_spec],
    ],

    rules: {
        source_file: $ => repeat(
            choice($.library_element_name, $.library_element_declaration)
        ),

        // COMMENTS
        // extras
        single_line_comment: $ => seq("//", /.*/),
        multi_line_comment: $ => seq(
            "(*",
            optional($.comment_text),
            "*)"
        ),
        comment_text: $ => repeat1(choice(/.|\n|\r/)),

        // A second special terminal symbol utilized in this syntax is the "null string", that is, a string
        // containing no characters. This is represented by the terminal symbol NIL.
        // NOTE: NIL is not defined in the rev-3 standard
        // Moreover tree-sitter doesn't support empty strings as tokens
        //NIL: $ => token(""),

        // This symbol shall normally consist of the "paragraph
        // separator" character defined as hexadecimal code 2029 by ISO/IEC 10646
        // NOTE: rev-3 defines this symobl as '\n'
        EOL: $ => token("\n"),

        //WS: $ => token(choice('\t', '\r', '\n')),

        // B.0 Programming model

        library_element_name: $ => choice(
            $.data_type_name,
            $.function_name,
            $.function_block_type_name,
            $.program_type_name,
            $.resource_type_name,
            $.configuration_name
        ),

        library_element_declaration: $ => choice(
            $.data_type_declaration,
            $.function_declaration,
            $.function_block_declaration,
            $.program_declaration,
            $.configuration_declaration
        ),

        // B.1.2 Constants

        constant: $ => prec.right(choice(
            $.numeric_literal,
            $.character_string,
            $.time_literal,
            $.bit_string_literal,
            $.boolean_literal
        )),

        // B 2.1 Numeric literals

        numeric_literal: $ => choice(
            $.integer_literal,
            $.real_literal
        ),

        integer_literal: $ => seq(
            optional(seq($.integer_type_name, "#")),
            choice($.signed_integer,
                $.binary_integer,
                $.octal_integer,
                $.hex_integer)
        ),

        signed_integer: $ => seq(
            optional(choice("+", "-")),
            $.integer
        ),

        integer: $ => seq(
            $._digit,
            repeat(seq(optional("_"), $._digit))
        ),

        binary_integer: $ => seq(
            "2#",
            $.bit,
            repeat(seq(optional("_"), $.bit))
        ),

        bit: $ => choice("1", "0"),

        octal_integer: $ => seq(
            "8#",
            $._octal_digit,
            repeat(seq(optional("_"), $._octal_digit))
        ),

        hex_integer: $ => seq(
            "16#",
            $._hex_digit,
            repeat(seq(optional("_"), $._hex_digit))
        ),

        real_literal: $ => seq(
            optional(seq($.real_type_name, "#")),
            $.signed_integer,
            ".",
            $.integer,
            optional($.exponent)
        ),

        exponent: $ => seq(
            choice("e", "E"),
            optional(choice("+", "-")),
            $.integer
        ),

        bit_string_literal: $ => seq(
            optional(seq(choice("BYTE", "WORD", "DWORD", "LWORD"), "#")),
            //NOTE: IEC says we should use unsigned_integer here, but it's not defined
            choice($.integer, $.binary_integer, $.octal_integer, $.hex_integer)
        ),

        boolean_literal: $ => seq(
            optional(seq("BOOL", "#")),
            choice(token(choice("1", "0")), choice("TRUE", "FALSE"))
        ),

        // B.1.2.2 Character strings

        character_string: $ => choice($.single_byte_character_string, $.double_byte_character_string),

        single_byte_character_string: $ => seq(
            "'",
            repeat($.single_byte_character_representation),
            "'"
        ),

        double_byte_character_string: $ => seq(
            '"',
            repeat($.double_byte_character_representation),
            '"'
        ),

        single_byte_character_representation: $ => seq(
            $.common_character_representation,
            choice("$'", '"', seq('$', $._hex_digit, $._hex_digit))
        ),

        double_byte_character_representation: $ => seq(
            $.common_character_representation,
            choice('$"', "'", seq('$', $._hex_digit, $._hex_digit, $._hex_digit, $._hex_digit))
        ),

        common_character_representation: $ => choice(
            /[^\$"'\\]/,
            '$$',
            '$L',
            '$N',
            '$P',
            '$R',
            '$T',
            '$l',
            '$n',
            '$p',
            '$r',
            '$t'
        ),

        // B.1.2.3 Time literals

        time_literal: $ => choice(
            $.duration,
            $.time_of_day,
            $.date,
            $.date_and_time
        ),

        // B.1.2.3.1 Durations

        duration: $ => seq(
            choice("TIME", "T"),
            "#",
            optional("-"),
            $.interval
        ),

        interval: $ => choice(
            $.days, $.hours, $.minutes, $.seconds, $.milliseconds
        ),

        days: $ => choice(
            prec(2, seq($.fixed_point, "d")),
            prec(1, seq($.integer, "d", optional("_"), $.hours))
        ),

        fixed_point: $ => seq(
            $.integer,
            optional(seq(".", $.integer))
        ),

        hours: $ => choice(
            prec(2, seq($.fixed_point, "h")),
            prec(1, seq($.integer, "h", optional("_"), $.minutes))
        ),

        minutes: $ => choice(
            prec(2, seq($.fixed_point, "m")),
            prec(1, seq($.integer, "m", optional("_"), $.seconds))
        ),

        seconds: $ => choice(
            prec(2, seq($.fixed_point, "s")),
            prec(1, seq($.integer, "s", optional("_"), $.milliseconds))
        ),

        milliseconds: $ => seq(
            $.fixed_point,
            "ms"
        ),

        // B.1.2.3.2 Time of day and date

        time_of_day: $ => seq(
            choice("TIME_OF_DAY", "TOD"),
            "#",
            $.daytime
        ),

        daytime: $ => seq(
            $.day_hour,
            ":",
            $.day_minute,
            ":",
            $.day_second,
        ),

        day_hour: $ => $.integer,

        day_minute: $ => $.integer,

        day_second: $ => $.fixed_point,

        date: $ => seq(
            choice("DATE", "D"),
            "#",
            $.date_literal
        ),

        date_literal: $ => seq(
            $.year,
            "-",
            $.month,
            "-",
            $.day
        ),

        year: $ => $.integer,

        month: $ => $.integer,

        day: $ => $.integer,

        date_and_time: $ => seq(
            choice("DATE_AND_TIME", "DT"),
            "#",
            $.date_literal,
            "-",
            $.daytime
        ),

        // B.1.3 Data types

        data_type_name: $ => choice($.non_generic_type_name, $.generic_type_name),

        non_generic_type_name: $ => choice($.elementary_type_name, $.derived_type_name),

        // B.1.3.1 Elementary data types

        elementary_type_name: $ => choice(
            $.numeric_type_name,
            $.date_type_name,
            $.bit_string_type_name,
            'STRING', 'WSTRING', 'TIME'
        ),

        numeric_type_name: $ => choice($.integer_type_name, $.real_type_name),

        integer_type_name: $ => choice($.signed_integer_type_name, $.unsigned_integer_type_name),

        signed_integer_type_name: $ => choice("SINT", "INT", "DINT", "LINT"),

        unsigned_integer_type_name: $ => choice("USINT", "UINT", "UDINT", "ULINT"),

        real_type_name: $ => choice("REAL", "LREAL"),

        date_type_name: $ => choice("DATE", "TIME_OF_DAY", "TOD", "DATE_AND_TIME", "DT"),

        bit_string_type_name: $ => choice("BOOL", "BYTE", "WORD", "DWORD", "LWORD"),

        // B.1.3.2 Generic data types

        generic_type_name: $ => choice(
            'ANY', 'ANY_DERIVED', 'ANY_ELEMENTARY',
            'ANY_MAGNITUDE', 'ANY_NUM', 'ANY_REAL', 'ANY_INT', 'ANY_BIT',
            'ANY_STRING', 'ANY_DATE'
        ),

        // B.1.3.3 Derived data types

        derived_type_name: $ => choice(
            $.single_element_type_name,
            $.array_type_name,
            $.structure_type_name,
            $.string_type_name
        ),

        single_element_type_name: $ => choice(
            $.simple_type_name,
            $.subrange_type_name,
            $.enumerated_type_name
        ),

        simple_type_name: $ => $.identifier,

        subrange_type_name: $ => $.identifier,

        enumerated_type_name: $ => $.identifier,

        array_type_name: $ => $.identifier,

        structure_type_name: $ => $.identifier,

        data_type_declaration: $ => seq(
            "TYPE",
            $.type_declaration,
            $.SEMICOLON,
            repeat(seq($.type_declaration, $.SEMICOLON)),
            "END_TYPE"
        ),

        type_declaration: $ => choice(
            $.single_element_type_declaration,
            $.array_type_declaration,
            $.structure_type_declaration,
            $.string_type_declaration
        ),

        single_element_type_declaration: $ => choice(
            $.simple_type_declaration,
            $.subrange_type_declaration,
            $.enumerated_type_declaration
        ),

        simple_type_declaration: $ => seq(
            $.simple_type_name,
            ":",
            $.simple_spec_init
        ),

        simple_spec_init: $ => seq(
            $.simple_specification,
            optional(seq(":=", $.constant))
        ),

        simple_specification: $ => choice(
            $.elementary_type_name,
            $.simple_type_name
        ),

        subrange_type_declaration: $ => seq(
            $.subrange_type_name,
            ":",
            $.subrange_spec_init
        ),

        subrange_spec_init: $ => seq(
            $.subrange_specification,
            optional(seq(":=", $.signed_integer))
        ),

        subrange_specification: $ => choice(
            seq(
                $.integer_type_name,
                "(",
                $.subrange,
                ")"
            ),
            ($.subrange_type_name)
        ),

        subrange: $ => seq(
            $.signed_integer,
            "..",
            $.signed_integer
        ),

        enumerated_type_declaration: $ => seq(
            $.enumerated_type_name,
            ":",
            $.enumerated_spec_init
        ),

        enumerated_spec_init: $ => seq(
            $.enumerated_specification,
            optional(seq(":=", $.enumerated_value))
        ),

        enumerated_specification: $ => seq(
            "(",
            $.enumerated_value,
            repeat(seq(",", $.enumerated_value)),
            ")"
        ),

        enumerated_value: $ => seq(
            optional(seq($.enumerated_type_name, "#")),
            $.identifier
        ),

        array_type_declaration: $ => seq(
            $.array_type_name,
            ":",
            $.array_spec_init
        ),

        array_spec_init: $ => seq(
            $.array_specification,
            optional(seq(":=", $.array_initialization))
        ),

        array_specification: $ => choice(
            $.array_type_name,
            seq("ARRAY", "[", $.subrange, repeat(seq(",", $.subrange)), "]", "OF", $.non_generic_type_name)
        ),

        array_initialization: $ => seq(
            "[", $.array_initial_elements, repeat(seq(",", $.array_initial_elements)), "]"
        ),

        array_initial_elements: $ => choice(
            $.array_initial_element,
            seq($.integer, "(", optional($.array_initial_element), ")")
        ),

        array_initial_element: $ => choice(
            $.constant,
            $.enumerated_value,
            $.structure_initialization,
            $.array_initialization
        ),

        structure_type_declaration: $ => seq(
            $.structure_type_name,
            ":",
            $.structure_specification
        ),

        structure_specification: $ => choice(
            $.structure_declaration,
            $.initialized_structure
        ),

        initialized_structure: $ => seq(
            $.structure_type_name,
            optional(seq(":=", $.structure_initialization))
        ),

        structure_declaration: $ => seq(
            "STRUCT", $.structure_element_declaration, $.SEMICOLON,
            repeat(seq($.structure_element_declaration, $.SEMICOLON)),
            "END_STRUCT"
        ),

        structure_element_declaration: $ => seq(
            $.structure_element_name, ":",
            choice($.simple_spec_init, $.subrange_spec_init, $.enumerated_spec_init, $.array_spec_init, $.initialized_structure)
        ),

        structure_element_name: $ => $.identifier,

        structure_initialization: $ => seq(
            // Note: IEC 61131-3 standard does not define as optionnal but i had to so otherwise tree sitter can't parse it
            "(", optional($.structure_element_initialization),
            repeat(seq(",", $.structure_element_initialization)),
            ")"
        ),

        structure_element_initialization: $ => seq(
            $.structure_element_name, ":=",
            choice($.constant, $.enumerated_value, $.array_initialization, $.structure_initialization)
        ),

        string_type_name: $ => $.identifier,

        string_type_declaration: $ => seq(
            $.string_type_name,
            ":",
            choice("STRING", "WSTRING"),
            optional(seq("[", $.integer, "]")),
            optional(seq(":=", $.character_string))
        ),

        // B.1.4 Variables

        variable: $ => choice($.direct_variable, $.symbolic_variable),

        symbolic_variable: $ => choice(
            $.variable_name,
            $.multi_element_variable
        ),

        variable_name: $ => $.identifier,

        // B.1.4.1 Directly represented variables

        direct_variable: $ => seq(
            "%",
            $.location_prefix,
            $.size_prefix,
            $.integer,
            repeat(seq(".", $.integer))
        ),

        location_prefix: $ => choice("I", "Q", "M"),

        size_prefix: $ => choice("X", "B", "W", "D", "L"),

        // B.1.4.2 Multi-element variables

        multi_element_variable: $ => choice(
            $.array_variable,
            $.structured_variable
        ),

        array_variable: $ => seq(
            $.subscripted_variable,
            $.subscript_list
        ),

        subscripted_variable: $ => $.symbolic_variable,

        subscript_list: $ => seq(
            "[", $.subscript, repeat(seq(",", $.subscript)), "]"
        ),

        subscript: $ => $.expression,

        structured_variable: $ => seq(
            $.record_variable,
            ".",
            $.field_selector
        ),

        record_variable: $ => $.symbolic_variable,

        // B.1.4.3 Declaration and initialization

        input_declarations: $ => seq(
            "VAR_INPUT", choice("RETAIN", "NON_RETAIN"),
            $.input_declaration, $.SEMICOLON,
            repeat(seq($.input_declaration, $.SEMICOLON)),
            "END_VAR"
        ),

        input_declaration: $ => choice($.var_init_decl, $.edge_declaration),

        edge_declaration: $ => seq(
            $.var1_list,
            ":",
            "BOOL",
            choice("R_EDGE", "F_EDGE")
        ),

        var_init_decl: $ => choice(
            $.var1_init_decl,
            $.array_var_init_decl,
            $.structured_var_init_decl,
            $.fb_name_decl,
            $.string_var_declaration
        ),

        var1_init_decl: $ => seq(
            $.var1_list,
            ":",
            choice($.simple_spec_init, $.subrange_spec_init, $.enumerated_spec_init)
        ),

        var1_list: $ => seq(
            $.variable_name,
            repeat(seq(",", $.variable_name))
        ),

        array_var_init_decl: $ => seq(
            $.var1_list,
            ":",
            $.array_spec_init
        ),

        structured_var_init_decl: $ => seq(
            $.var1_list,
            ":",
            $.initialized_structure
        ),

        fb_name_decl: $ => seq(
            $.fb_name_list, ":", $.function_block_type_name,
            optional(seq(":=", $.structure_initialization))
        ),

        fb_name_list: $ => seq(
            $.fb_name, repeat(seq(",", $.fb_name))
        ),

        fb_name: $ => $.identifier,

        output_declarations: $ => seq(
            "VAR_OUTPUT", choice("RETAIN", "NON_RETAIN"),
            $.var_init_decl, $.SEMICOLON,
            repeat(seq($.var_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        input_output_declarations: $ => seq(
            "VAR_IN_OUT", choice("RETAIN", "NON_RETAIN"),
            $.var_declaration, $.SEMICOLON,
            repeat(seq($.var_declaration, $.SEMICOLON)),
            "END_VAR"
        ),

        var_declaration: $ => choice(
            $.temp_var_decl,
            $.fb_name_decl
        ),

        temp_var_decl: $ => choice(
            $.var1_declaration,
            $.array_var_declaration,
            $.structured_var_declaration,
            $.string_var_declaration
        ),

        var1_declaration: $ => seq(
            $.var1_list, ":",
            choice($.simple_specification, $.subrange_specification, $.enumerated_specification)
        ),

        array_var_declaration: $ => seq(
            $.var1_list, ":", $.array_specification
        ),

        structured_var_declaration: $ => seq(
            $.var1_list, ":", $.structure_type_name
        ),

        var_declarations: $ => seq(
            "VAR",
            "CONSTANT",
            $.var_init_decl, $.SEMICOLON,
            repeat(seq($.var_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        retentive_var_declarations: $ => seq(
            "VAR",
            "RETAIN",
            $.var_init_decl, $.SEMICOLON,
            repeat(seq($.var_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        // TODO! Not in the standard, but still referenced
        non_retentive_var_declarations: $ => seq(
            "VAR",
            "NON_RETAIN",
            $.var_init_decl, $.SEMICOLON,
            repeat(seq($.var_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        located_var_declarations: $ => seq(
            "VAR",
            optional(choice("CONSTANT", "RETAIN", "NON_RETAIN")),
            $.located_var_decl, $.SEMICOLON,
            repeat(seq($.located_var_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        located_var_decl: $ => seq(
            optional($.variable_name),
            $.location,
            ":",
            $.located_var_spec_init
        ),

        external_var_declarations: $ => seq(
            "VAR_EXTERNAL", optional("CONSTANT"),
            $.external_declaration, $.SEMICOLON,
            repeat(seq($.external_declaration, $.SEMICOLON)),
            "END_VAR"
        ),

        external_declaration: $ => seq(
            $.global_var_name, ":",
            choice(
                $.simple_specification,
                $.subrange_specification,
                $.enumerated_specification,
                $.array_specification,
                $.structure_type_name,
                $.function_block_type_name
            )
        ),

        global_var_name: $ => $.identifier,

        global_var_declarations: $ => seq(
            "VAR_GLOBAL", optional(choice("CONSTANT", "RETAIN")),
            $.global_var_decl, $.SEMICOLON,
            repeat(seq($.global_var_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        global_var_decl: $ => seq(
            $.global_var_spec, ":",
            optional(choice($.located_var_spec_init, $.function_block_type_name))
        ),

        global_var_spec: $ => choice($.global_var_list, seq(optional($.global_var_name), $.location)),

        located_var_spec_init: $ => choice(
            $.simple_spec_init,
            $.subrange_spec_init,
            $.enumerated_spec_init,
            $.array_spec_init,
            $.initialized_structure,
            $.single_byte_string_spec,
            $.double_byte_string_spec
        ),

        location: $ => seq("AT", $.direct_variable),

        global_var_list: $ => seq(
            $.global_var_name,
            repeat(seq(",", $.global_var_name))
        ),

        string_var_declaration: $ => choice(
            $.single_byte_string_var_declaration,
            $.double_byte_string_var_declaration
        ),

        single_byte_string_var_declaration: $ => seq(
            $.var1_list, ":", $.single_byte_string_spec
        ),

        single_byte_string_spec: $ => seq(
            "STRING",
            optional(seq("[", $.integer, "]")),
            optional(seq(":=", $.single_byte_character_string))
        ),

        double_byte_string_var_declaration: $ => seq(
            $.var1_list, ":", $.double_byte_string_spec
        ),

        double_byte_string_spec: $ => seq(
            "WSTRING",
            optional(seq("[", $.integer, "]")),
            optional(seq(":=", $.double_byte_character_string))
        ),

        incompl_located_var_declarations: $ => seq(
            "VAR", optional(choice("RETAIN", "NON_RETAIN")),
            $.incompl_located_var_decl, $.SEMICOLON,
            optional(seq($.incompl_located_var_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        incompl_located_var_decl: $ => seq(
            $.variable_name,
            $.incompl_location,
            ":",
            $.var_spec
        ),

        incompl_location: $ => seq("AT", "%", choice("I", "Q", "M"), "*"),

        var_spec: $ => choice(
            $.simple_specification,
            $.subrange_specification,
            $.enumerated_specification,
            $.array_specification,
            $.structure_type_name,
            seq("STRING", optional(seq("[", $.integer, "]"))),
            seq("WSTRING", optional(seq("[", $.integer, "]")))
        ),

        // B.1.5 Program organization units

        // B.1.5.1 Functions

        function_name: $ => choice($.standard_function_name, $.derived_function_name),

        // TODO Add standard function names
        standard_function_name: $ => choice(
            'TRUNC', 'ABS', 'SQRT', 'LN', 'LOG', 'EXP',
            'SIN', 'COS', 'TAN', 'ASIN', 'ACOS', 'ATAN', 'ATAN2',
            'ADD', 'SUB', 'MUL', 'DIV', 'MOD', 'EXPT', 'MOVE',
            'SHL', 'SHR', 'ROL', 'ROR',
            'AND', 'OR', 'XOR', 'NOT',
            'SEL', 'MAX', 'MIN', 'LIMIT', 'MUX',
            'GT', 'GE', 'EQ', 'LE', 'LT', 'NE',
            'LEN', 'LEFT', 'RIGHT', 'MID', 'CONCAT', 'INSERT', 'DELETE', 'REPLACE', 'FIND'
            // incomplete list ?
        ), //NOTE: list taken from rev-3

        derived_function_name: $ => $.identifier,

        function_declaration: $ => seq(
            "FUNCTION", $.derived_function_name, ":",
            choice($.elementary_type_name, $.derived_type_name),
            repeat(choice($.function_var_decls, $.io_var_declarations)),
            optional($.function_body),
            "END_FUNCTION"
        ),

        io_var_declarations: $ => choice(
            $.input_declarations,
            $.output_declarations,
            $.input_output_declarations
        ),

        function_var_decls: $ => seq(
            "VAR", optional("CONSTANT"),
            $.var2_init_decl, $.SEMICOLON,
            repeat(seq($.var2_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        // TODO Add support for more languages
        function_body: $ => choice(
            //$.instruction_list,
            $.statement_list
        ),

        var2_init_decl: $ => choice(
            $.var1_init_decl,
            $.array_var_init_decl,
            $.structured_var_init_decl,
            $.string_var_declaration
        ),

        // B.1.5.2 Function blocks

        function_block_type_name: $ => choice($.standard_function_block_name, $.derived_function_block_name),

        // TODO Add standard function block names
        standard_function_block_name: $ => choice(
            'SR', 'RS', 'R_TRIG', 'F_TRIG', 'CTU', 'CTD', 'CTUD', 'TP', 'TON', 'TOF'
        ), // incomplete list ?
        // NOTE: list taken from rev-3

        derived_function_block_name: $ => $.identifier,

        function_block_declaration: $ => seq(
            "FUNCTION_BLOCK", $.derived_function_block_name,
            repeat(choice($.io_var_declarations, $.other_var_declarations)),
            $.function_block_body,
            "END_FUNCTION_BLOCK"
        ),

        other_var_declarations: $ => choice(
            $.external_var_declarations,
            $.var_declarations,
            $.retentive_var_declarations,
            $.non_retentive_var_declarations,
            $.temp_var_decls,
            $.incompl_located_var_declarations
        ),

        temp_var_decls: $ => seq(
            "VAR_TEMP",
            $.temp_var_decl, $.SEMICOLON,
            repeat(seq($.temp_var_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        non_retentive_var_decls: $ => seq(
            "VAR_NON_RETAIN",
            $.var_init_decl, $.SEMICOLON,
            repeat(seq($.var_init_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        // TODO Add support for more languages
        function_block_body: $ => choice(
            //$.instruction_list,
            $.statement_list
        ),

        // B.1.5.3 Programs

        program_type_name: $ => $.identifier,

        program_declaration: $ => seq(
            "PROGRAM", $.program_type_name,
            repeat(choice(
                $.io_var_declarations,
                $.other_var_declarations,
                $.located_var_declarations,
                $.program_access_decls)
            ),
            $.function_block_body,
            "END_PROGRAM"
        ),

        program_access_decls: $ => seq(
            "VAR_ACCESS", $.program_access_decl, $.SEMICOLON,
            repeat(seq($.program_access_decl, $.SEMICOLON)),
            "END_VAR"
        ),

        program_access_decl: $ => seq(
            $.access_name, ":", $.symbolic_variable,
            $.non_generic_type_name, optional($.direction)
        ),

        // B.1.7 Configuration elements

        configuration_name: $ => $.identifier,

        resource_type_name: $ => $.identifier,

        configuration_declaration: $ => seq(
            "CONFIGURATION", $.configuration_name,
            optional($.global_var_declarations),
            choice(
                $.single_resource_declaration,
                seq($.resource_declaration, repeat(seq($.resource_declaration)))
            ),
            optional($.access_declaration),
            optional($.instance_specific_initializations),
            "END_CONFIGURATION"
        ),

        resource_declaration: $ => seq(
            "RESOURCE", $.resource_name, "ON", $.resource_type_name,
            optional($.global_var_declarations),
            $.single_resource_declaration,
            "END_RESOURCE"
        ),

        single_resource_declaration: $ => seq(
            repeat(seq($.task_configuration, $.SEMICOLON)),
            $.program_configuration, $.SEMICOLON,
            repeat(seq($.program_configuration, $.SEMICOLON)),
        ),

        resource_name: $ => $.identifier,

        access_declarations: $ => seq(
            "VAR_ACCESS", $.access_declaration, $.SEMICOLON,
            repeat(seq($.access_declaration, $.SEMICOLON)),
            "END_VAR"
        ),

        access_declaration: $ => seq(
            $.access_name, ":", $.access_path, $.non_generic_type_name,
            optional($.direction)
        ),

        access_path: $ => choice(
            seq(
                optional(seq($.resource_name, ".")), $.direct_variable,
            ),
            seq(
                optional(seq($.resource_name, ".")),
                optional(seq($.program_name, ".")),
                repeat(seq($.fb_name, ".")),
                $.symbolic_variable
            )
        ),

        global_var_reference: $ => seq(
            optional(seq($.resource_name, ".")),
            $.global_var_name,
            optional(seq(".", $.structure_element_name))
        ),

        access_name: $ => $.identifier,

        program_output_reference: $ => seq(
            $.program_name, ".", $.symbolic_variable
        ),

        program_name: $ => $.identifier,

        direction: $ => choice("READ_WRITE", "READ_ONLY"),

        task_configuration: $ => seq(
            "TASK", $.task_name, $.task_initialization
        ),

        task_name: $ => $.identifier,

        task_initialization: $ => seq(
            "(", optional(seq("SINGLE", ":=", $.data_source, ",")),
            optional(seq("INTERVAL", ":=", $.data_source, ",")),
            "PRIORITY", ":=", $.integer, ")"
        ),

        data_source: $ => choice(
            $.constant,
            $.global_var_reference,
            $.program_output_reference,
            $.direct_variable
        ),

        program_configuration: $ => seq(
            "PROGRAM", optional(choice("RETAIN", "NON_RETAIN")),
            $.program_name, optional(seq("WITH", $.task_name)), ":", $.program_type_name,
            optional(seq("(", $.prog_conf_elements, ")"))
        ),

        prog_conf_elements: $ => seq(
            $.prog_conf_element, repeat(seq(",", $.prog_conf_element))
        ),

        prog_conf_element: $ => choice(
            $.fb_task,
            $.prog_cnxn
        ),

        fb_task: $ => seq(
            $.fb_name,
            "WITH",
            $.task_name
        ),

        prog_cnxn: $ => seq(
            choice($.symbolic_variable, ":=", $.prog_data_source),
            $.symbolic_variable, "=>", $.data_sink
        ),

        prog_data_source: $ => choice(
            $.constant,
            $.enumerated_value,
            $.global_var_reference,
            $.direct_variable
        ),

        data_sink: $ => choice(
            $.global_var_reference,
            $.direct_variable
        ),

        instance_specific_initializations: $ => seq(
            "VAR_CONFIG",
            $.instance_specific_init, $.SEMICOLON,
            repeat(seq($.instance_specific_init, $.SEMICOLON)),
            "END_VAR"
        ),

        instance_specific_init: $ => seq(
            $.resource_name, ".", $.program_name, ".", repeat(seq($.fb_name, ".")),
            choice(
                seq($.variable_name, optional($.location), ":", $.located_var_spec_init),
                seq($.fb_name, ":", $.function_block_type_name, ":=", $.structure_initialization))
        ),

        ...B2.rules,
        ...B3.rules,

        _letter: $ => /[A-Z_a-z_]/,
        _digit: $ => /[0-9]/,
        _octal_digit: $ => /[0-7]/,
        _hex_digit: $ => /[0-9A-F]/,
        identifier: $ => /[A-Za-z_][A-Za-z0-9_]*/,

        // Improve error recovery for missing punctuation:
        SEMICOLON: $ => choice(
            seq(";", $.EOL),
            $.EOL,
            ";",
        )


        // B.1 Common elements
    },

    word: $ => $.identifier
});
