/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// B.2 Language IL (Instruction List)

module.exports = {
    // B.2.1 Instructions and operands

    rules: {
        instruction_list: $ => seq(
            "#IL", 
            $.il_instruction, 
            repeat($.il_instruction)
        ),

        il_instruction: $ => prec.right(seq(
            optional(seq($.label, ":")),
            optional(
                choice(
                    $.il_simple_operation,
                    $.il_expression,
                    $.il_jump_operation,
                    $.il_fb_call,
                    $.il_formal_funct_call,
                    $.il_return_operator
                )),
            $.EOL,
            repeat($.EOL)
        )),

        label: $ => $.identifier,

        il_simple_operation: $ => choice(
            seq($.il_simple_operator, optional($.il_operand)),
            seq($.function_name, optional($.il_operand_list))
        ),

        il_expression: $ => seq(
            $.il_expr_operator,
            "(", optional($.il_operand), $.EOL, repeat($.EOL),
            optional($.simple_instr_list), ")"
        ),

        il_jump_operation: $ => seq($.il_jump_operator, $.label),

        il_fb_call: $ => prec.left(seq(
            $.il_call_operator,
            $.fb_name,
            optional(
                seq("(",
                    choice(
                        seq("[", $.EOL, repeat($.EOL), optional($.il_param_list)),
                        seq(optional($.il_operand_list)),
                        ")"
                    ))
            )
        )),

        il_formal_funct_call: $ => seq(
            $.function_name,
            "(", $.EOL, repeat($.EOL), optional($.il_param_list), ")"
        ),

        il_operand: $ => choice($.constant, $.variable, $.enumerated_value),

        il_operand_list: $ => seq(
            $.il_operand,
            repeat(seq(",", $.il_operand))
        ),

        simple_instr_list: $ => prec.right(seq(
            $.il_simple_instruction,
            repeat($.il_simple_instruction)
        )),

        il_simple_instruction: $ => prec.right(seq(
            choice($.il_simple_operation, $.il_expression, $.il_formal_funct_call),
            $.EOL, repeat($.EOL)
        )),

        il_param_list: $ => seq(
            repeat($.il_param_assignment),
            $.il_param_last_instruction
        ),

        il_param_instruction: $ => seq(
            choice($.il_param_assignment, $.il_param_out_assignment), ",",
            $.EOL, repeat($.EOL)
        ),

        il_param_last_instruction: $ => prec.right(seq(
            choice($.il_param_assignment, $.il_param_out_assignment),
            $.EOL, repeat($.EOL)
        )),

        il_param_assignment: $ => seq(
            $.il_assign_operator,
            choice($.il_operand, seq("(", $.EOL, repeat($.EOL), $.simple_instr_list), ")")
        ),

        il_param_out_assignment: $ => $.il_assign_out_operator,

        // B.2.2 Operators

        il_simple_operator: $ => choice(
            'LD', 'LDN', 'ST', 'STN', 'NOT', 'S',
            'R', 'S1', 'R1', 'CLK', 'CU', 'CD', 'PV',
            'IN', 'PT', $.il_expr_operator
        ),

        il_expr_operator: $ => choice(
            'AND', '& ', 'OR', 'XOR', 'ANDN', '&N', 'ORN',
            'XORN', 'ADD', 'SUB', 'MUL', 'DIV', 'MOD', 'GT', 'GE', 'EQ',
            'LT', 'LE', 'NE'
        ),

        il_assign_operator: $ => seq($.variable_name, ":="),

        il_assign_out_operator: $ => seq(
            optional("NOT"), 
            $.variable_name,
            "=>"
        ),

        il_call_operator: $ => choice("CALL", "CALC", "CALCN"),

        il_return_operator: $ => choice("RET", "RETC", "RETCN"),

        il_jump_operator: $ => choice("JMP", "JMPC", "JMPCN"),
    }
}