/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// B.3 Language ST (Structured Text)

module.exports = {

    rules: {
        // B.3.1 Expressions

        expression: $ => seq(
            $.xor_expression,
            repeat(seq("OR", $.xor_expression))
        ),

        xor_expression: $ => seq(
            $.and_expression,
            repeat(seq("XOR", $.and_expression))
        ),

        and_expression: $ => seq(
            $.comparison,
            repeat(seq(choice("&", "AND"), $.comparison))
        ),

        comparison: $ => seq(
            $.equ_expression,
            repeat(seq(choice("=", "<>"), $.equ_expression))
        ),

        equ_expression: $ => seq(
            $.add_expression,
            repeat(seq($.comparison_operator, $.add_expression))
        ),

        comparison_operator: $ => choice("<", "<=", ">", ">="),

        add_expression: $ => seq(
            $.term,
            repeat(seq($.add_operator, $.term))
        ),

        add_operator: $ => choice("+", "-"),

        term: $ => seq(
            $.power_expression,
            repeat(seq($.multiply_operator, $.power_expression))
        ),

        multiply_operator: $ => choice("*", "/", "MOD"),

        power_expression: $ => seq(
            $.unary_expression,
            repeat(seq("**", $.unary_expression))
        ),

        unary_expression: $ => seq(
            optional($.unary_operator),
            $.primary_expression
        ),

        unary_operator: $ => prec.right(choice("-", "NOT")),

        primary_expression: $ => choice(
            $.constant,
            $.enumerated_value,
            $.variable,
            seq("(", $.expression, ")"),
            seq($.function_name,
                "(", $.param_assignment, 
                repeat(seq(",", $.param_assignment)), ")"),
        ),

        // B.3.2 Statements

        statement_list: $ => prec.left(repeat1(seq($.statement, $.SEMICOLON))),
        

        statement: $ => choice(
            //$.NIL, NOTE: NIL is removed in rev-3
            $.assignment_statement,
            $.subprogram_control_statement,
            $.selection_statement,
            $.iteration_statement
        ),

        // B.3.2.1 Assignment statements

        assignment_statement: $ => seq(
            $.variable, ":=", $.expression
        ),

        // B.3.2.2 Subprogram control statements

        subprogram_control_statement: $ => choice(
            $.fb_invocation,
            "RETURN"
        ),

        fb_invocation: $ => seq(
            $.fb_name,
            "(", optional(seq($.param_assignment, repeat(seq(",", $.param_assignment)))), ")"
        ),

        param_assignment: $ => choice(
            seq(optional(seq($.variable_name, ":=")), $.expression),
            seq(optional("NOT"), $.variable_name, "=>", $.variable)
        ),

        // B.3.2.3 Selection statements

        selection_statement: $ => choice(
            $.if_statement,
            $.case_statement
        ),

        if_statement: $ => seq(
            "IF", $.expression, "THEN", 
            optional($.statement_list),
            repeat(seq("ELSIF", $.expression, "THEN", $.statement_list)),
            optional(seq("ELSE", $.statement_list)),
            "END_IF"
        ),

        case_statement: $ => seq(
            "CASE", $.expression, "OF",
            $.case_element,
            repeat($.case_element),
            optional(seq("ELSE", $.statement_list)),
            "END_CASE"
        ),

        case_element: $ => seq(
            $.case_list, ":", $.statement_list
        ),

        case_list: $ => seq(
            $.case_list_element,
            repeat(seq(",", $.case_list_element))
        ),

        case_list_element: $ => choice(
            $.subrange,
            $.signed_integer,
            $.enumerated_value
        ),

        // B.3.2.4 Iteration statements

        iteration_statement: $ => choice(
            $.for_statement,
            $.while_statement,
            $.repeat_statement,
            $.exit_statement
        ),

        for_statement: $ => seq(
            "FOR", $.control_variable, ":=", $.for_list, "DO", $.statement_list, "END_FOR"
        ),

        control_variable: $ => $.identifier,

        for_list: $ => seq(
            $.expression, "TO", $.expression, 
            optional(seq("BY", $.expression))
        ),

        while_statement: $ => seq(
            "WHILE", $.expression, "DO", $.statement_list, "END_WHILE"
        ),

        repeat_statement: $ => seq(
            "REPEAT", $.statement_list, "UNTIL", $.expression, "END_REPEAT"
        ),

        exit_statement: $ => "EXIT",
    }
}