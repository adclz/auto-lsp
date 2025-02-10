use crate::{self as auto_lsp};
use auto_lsp::{choice, seq};

pub static COMMENT_QUERY: &'static str = "
(comment) @comment
";

pub static CORE_QUERY: &'static str = "
(module) @module
(import_statement) @import_statement
(import_prefix) @import_prefix
(relative_import) @relative_import
(future_import_statement) @future_import_statement
(import_from_statement) @import_from_statement
; (vec_import_list) @vec_import_list
; (import_list) @import_list

(aliased_import) @aliased_import
(wildcard_import) @wildcard_import
; (print_chevron) @print_chevron
; (print) @print
(chevron) @chevron

; Statements

(assert_statement) @assert_statement
(expression_statement) @expression_statement
(named_expression) @named_expression
(return_statement) @return_statement
(delete_statement) @delete_statement
(raise_statement) @raise_statement
(pass_statement) @pass_statement
(break_statement) @break_statement
(continue_statement) @continue_statement

(if_statement) @if_statement
(elif_clause) @elif_clause
(else_clause) @else_clause

(match_statement) @match_statement
; (match_block) @match_block
(case_clause) @case_clause

(for_statement) @for_statement
(while_statement) @while_statement
(try_statement) @try_statement

(except_clause) @except_clause

; (value_alias) @value_alias
(except_group_clause) @except_group_clause
(finally_clause) @finally_clause

(with_statement) @with_statement

; Function

(function_definition) @function
(parameters) @parameters
(lambda_parameters) @lambda_parameters

(list_splat) @list_splat
(dictionary_splat) @dicitionary_splat
(global_statement) @global_statement
(nonlocal_statement) @nonlocal_statement
(exec_statement) @exec_statement
(type_alias_statement) @type_alias_statement

(class_definition) @class_definition
(type_parameter) @type_parameter

(parenthesized_list_splat) @parenthesized_list_splat
(argument_list) @argument_list

(decorated_definition) @decorated_definition
(decorator) @decorator
(block) @block

; (expression_list) @expression_list
(dotted_name) @dotted_name

(union_pattern) @union_pattern
(dict_pattern) @dict_pattern
; (key_value_pattern) @key_value_pattern
(keyword_pattern) @keyword_pattern

(splat_pattern) @splat_pattern
(class_pattern) @class_pattern
(complex_pattern) @complex_pattern

(tuple_pattern) @tuple_pattern
(list_pattern) @list_pattern

(default_parameter) @default_parameter
(typed_default_parameter) @typed_default_parameter

(as_pattern) @as_pattern
(not_operator) @not_operator

; (and) @and
; (or) @or

(binary_operator) @binary_operator
(unary_operator) @unary_operator
(comparison_operator) @comparison_operator
; (operators) @operators

(lambda) @lambda
; (lambda_within_for_clause) @lambda_within_for_clause

(assignment) @assignment
(augmented_assignment) @augmented_assignment

; (operator) @operator
(yield) @yield

(attribute) @attribute
(subscript) @subscript
(slice) @slice

(ellipsis) @ellipsis
(call) @call

(typed_parameter) @typed_parameter
(splat_type) @splat_type
(generic_type) @generic_type
(union_type) @union_type
(constrained_type) @constrained_type
(member_type) @member_type

(keyword_argument) @keyword_argument
(list) @list
(set) @set
(tuple) @tuple

(dictionary) @dictionary
(pair) @pair
(list_comprehension) @list_comprehension

(dictionary_comprehension) @dictionary_comprehension
(set_comprehension) @set_comprehension
(generator_expression) @generator_expression
; (comprehension_clause) @comprehension_clause

(parenthesized_expression) @parenthesized_expression
(for_in_clause) @for_in_clause
(if_clause) @if_clause

(conditional_expression) @conditional_expression

(concatenated_string) @concatenated_string
(string) @string
(interpolation) @interpolation
(format_specifier) @format_specifier
(type_conversion) @type_conversion

(integer) @integer
(float) @float
(identifier) @identifier
";

#[seq(
    query = "module",
    code_lenses,
    completions,
    document_symbols,
    inlay_hints,
    semantic_tokens
)]
pub struct Module {
    statements: Vec<Statement>,
}

#[choice]
pub enum Statement {
    Simple(SimpleStatement),
    Compound(CompoundStatement),
}

#[choice]
pub enum SimpleStatement {
    FutureImportStatement(FutureImportStatement),
    ImportStatement(ImportStatement),
    ImportFromStatement(ImportFromStatement),
    PrintStatement(PrintStatement),
    AssertStatement(AssertStatement),
    ExpressionStatement(ExpressionStatement),
    ReturnStatement(ReturnStatement),
    DeleteStatement(DeleteStatement),
    RaiseStatement(RaiseStatement),
    PassStatement(PassStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    GlobalStatement(GlobalStatement),
    NonlocalStatement(NonlocalStatement),
    ExecStatement(ExecStatement),
    TypeAliasStatement(TypeAliasStatement),
}

#[seq(query = "import_statement")]
pub struct ImportStatement {
    import_list: ImportList,
}

#[seq(query = "import_prefix")]
pub struct ImportPrefix {}

#[seq(query = "relative_import")]
pub struct RelativeImport {
    prefix: ImportPrefix,
    name: Option<DottedName>,
}

#[seq(query = "future_import_statement")]
pub struct FutureImportStatement {
    import_list: ImportList,
}

#[choice]
pub enum ImportListOrVecImportList {
    ImportList(ImportList),
    VecImportList(VecImportList),
}

#[seq(query = "import_from_statement")]
pub struct ImportFromStatement {
    module_name: DottedName,
    import_list: ImportList,
    wildcard_or_import: WildcardOrImportListOrVecImportList,
}
#[choice]
pub enum WildcardOrImportListOrVecImportList {
    Wildcard(WildcardImport),
    VecImportList(VecImportList),
}

#[seq(query = "vec_import_list")]
pub struct VecImportList {
    import_list: Vec<ImportList>,
}

#[choice]
pub enum RelativeImportOrDottedName {
    RelativeImport(RelativeImport),
    DottedName(DottedName),
}

// inline
#[seq(query = "import_list")]
pub struct ImportList {
    name: Vec<ImportName>,
}

// inline
#[choice]
pub enum ImportName {
    AliasedImport(AliasedImport),
    DottedName(DottedName),
}

#[seq(query = "aliased_import")]
pub struct AliasedImport {
    name: DottedName,
    alias: Identifier,
}

#[seq(query = "wildcard_import")]
pub struct WildcardImport {}

#[choice]
pub enum PrintStatement {
    PrintChevron(PrintChevron),
    Print(Print),
}

#[seq(query = "print_chevron")]
pub struct PrintChevron {
    chevron: Chevron,
    expressions: Vec<Expression>,
}

#[seq(query = "print")]
pub struct Print {
    expression: Expression,
}

#[seq(query = "chevron")]
pub struct Chevron {
    expression: Expression,
}

#[seq(query = "assert_statement")]
pub struct AssertStatement {
    expression: Expression,
    message: Option<Expression>,
}

#[seq(query = "expression_statement")]
pub struct ExpressionStatement {
    expression: Expression,
    expressions: Vec<Expression>,
    assignment: Assignment,
    augmented_assignment: AugmentedAssignment,
    yield_: Yield,
}

#[seq(query = "named_expression")]
pub struct NamedExpression {
    name: IdentifierOrKeyword,
    value: Expression,
}

#[choice]
pub enum IdentifierOrKeyword {
    Identifier(Identifier),
    Keyword(KeywordIdentifier),
}

#[seq(query = "return_statement")]
pub struct ReturnStatement {
    expression: Option<Expressions>,
}

#[seq(query = "delete_statement")]
pub struct DeleteStatement {
    targets: Expressions,
}

// inline
#[choice]
pub enum Expressions {
    Expression(Expression),
    ExpressionList(ExpressionList),
}

#[seq(query = "raise_statement")]
pub struct RaiseStatement {
    expression: Vec<Expression>,
    from_: Option<Expression>,
}

#[seq(query = "pass_statement", hover)]
pub struct PassStatement {}

#[seq(query = "break_statement")]
pub struct BreakStatement {}

#[seq(query = "continue_statement")]
pub struct ContinueStatement {}

#[choice]
pub enum CompoundStatement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    TryStatement(TryStatement),
    WithStatement(WithStatement),
    Function(Function),
    Class(Class),
    DecoratoratedDefinition(DecoratoratedDefinition),
    MatchStatement(MatchStatement),
}

#[seq(query = "if_statement")]
pub struct IfStatement {
    condition: Expression,
    consequence: SimpleStatement,
    elif: Vec<ElifClause>,
    alternative: Option<ElseClause>,
}

#[seq(query = "elif_clause")]
pub struct ElifClause {
    condition: Expression,
    body: SimpleStatement,
}

#[seq(query = "else_clause")]
pub struct ElseClause {
    body: SimpleStatement,
}

#[seq(query = "match_statement")]
pub struct MatchStatement {
    match_: Vec<Expression>,
    blocks: Vec<MatchBlock>,
}

#[seq(query = "match_block")]
pub struct MatchBlock {
    clauses: Vec<CaseClause>,
}

#[seq(query = "case_clause")]
pub struct CaseClause {
    case: CasePattern,
    if_clause: Option<IfClause>,
    consequence: SimpleStatement,
}

#[seq(query = "for_statement")]
pub struct ForStatement {
    left: LeftHandSide,
    right: ExpressionList,
    body: SimpleStatement,
    alternative: Option<ElseClause>,
}

#[seq(query = "while_statement")]
pub struct WhileStatement {
    condition: Expression,
    body: SimpleStatement,
    alternative: Option<ElseClause>,
}

#[seq(query = "try_statement")]
pub struct TryStatement {
    body: SimpleStatement,
    clause: OneOfExceptClauseOrExceptGroupClauseOrFinallyClause,
}

#[choice]
pub enum OneOfExceptClauseOrExceptGroupClauseOrFinallyClause {
    ExceptClause(ExceptClause),
    ExceptGroupClause(ExceptGroupClause),
    FinallyClause(FinallyClause),
}

#[seq(query = "except_clause")]
pub struct ExceptClause {
    except: Expression,
    value_alias: Option<ValueAlias>,
    body: SimpleStatement,
}

#[seq(query = "value_alias")]
pub struct ValueAlias {
    value: Expression,
    as_: Option<Expression>,
}

#[seq(query = "except_group_clause")]
pub struct ExceptGroupClause {
    except: Expression,
    as_: Option<Expression>,
    body: SimpleStatement,
}

#[seq(query = "finally_clause")]
pub struct FinallyClause {
    body: SimpleStatement,
}

#[seq(query = "with_statement")]
pub struct WithStatement {
    clause: Expression,
    body: SimpleStatement,
}

#[seq(
    query = "function",
    comment,
    code_lenses,
    completions,
    document_symbols,
    inlay_hints,
    semantic_tokens
)]
struct Function {
    name: Identifier,
    type_parameters: Option<TypeParameter>,
    parameters: Parameters,
    return_type: Option<Type>,
    body: Block,
}

#[seq(query = "parameters")]
pub struct Parameters {
    parameters: Vec<Parameter>,
}

#[seq(query = "lambda_parameters")]
pub struct LambdaParameters {
    parameters: Vec<Parameters>,
}

#[seq(query = "list_splat")]
pub struct ListSplat {
    expressions: Expression,
}

#[seq(query = "dicitionary_splat")]
pub struct DictionarySplat {
    expression: Expression,
}

#[seq(query = "global_statement")]
pub struct GlobalStatement {
    names: Vec<Identifier>,
}

#[seq(query = "nonlocal_statement")]
pub struct NonlocalStatement {
    names: Vec<Identifier>,
}

#[seq(query = "exec_statement")]
pub struct ExecStatement {
    code: OneOfStringOrIdentifier,
    in_: Vec<Expression>,
}

#[choice]
pub enum OneOfStringOrIdentifier {
    String(String),
    Identifier(Identifier),
}

#[seq(query = "type_alias_statement")]
pub struct TypeAliasStatement {
    left: Type,
    right: Type,
}

#[seq(query = "class_definition")]
pub struct Class {
    name: Identifier,
    type_parameters: Option<TypeParameter>,
    arguments: Option<ArgumentList>,
    body: Block,
}

#[seq(query = "type_parameter")]
pub struct TypeParameter {
    type_: Vec<Type>,
}

#[seq(query = "parenthesized_list_splat")]
pub struct ParenthesizedListSplat {
    list: Vec<OneOfListOrExpression>,
}

#[choice]
pub enum OneOfListOrExpression {
    ParenthesizedListSplat(ParenthesizedListSplat),
    ListSplat(ListSplat),
}

#[seq(query = "argument_list")]
pub struct ArgumentList {
    arguments: Vec<Args>,
}

#[choice]
pub enum Args {
    Expression(Expression),
    ListSplat(ListSplat),
    DictionarySplat(DictionarySplat),
    ParenthesizedExpression(ParenthesizedExpression),
    KeywordArgument(KeywordArgument),
}

#[seq(query = "decorated_definition")]
pub struct DecoratoratedDefinition {
    decorators: Vec<Decorator>,
    definition: OneOfFunctionOrClass,
}

#[choice]
pub enum OneOfFunctionOrClass {
    Function(Function),
    Class(Class),
}

#[seq(query = "decorator")]
pub struct Decorator {
    expression: Expression,
}

#[seq(query = "block", completions, document_symbols)]
pub struct Block {
    statements: Vec<Statement>,
}

#[seq(query = "expression_list")]
pub struct ExpressionList {
    expressions: Vec<Expression>,
}

#[seq(query = "dotted_name")]
pub struct DottedName {
    names: Vec<Identifier>,
}

#[choice]
pub enum CasePattern {
    AsPattern(AsPattern),
    KeywordPattern(KeywordPattern),
    SimplePattern(SimplePattern),
}

#[choice]
pub enum SimplePattern {
    ClassPattern(ClassPattern),
    SplatPattern(SplatPattern),
    UnionPattern(UnionPattern),
    DictPattern(DictPattern),
    String(String),
    ConcatenatedString(ConcatenatedString),
    True(True),
    False(False),
    None(None),
    Integer(Integer),
    Float(Float),
    ComplexPattern(ComplexPattern),
    DottedName(DottedName),
}

#[seq(query = "union_pattern")]
pub struct UnionPattern {
    patterns: Vec<CasePattern>,
}

#[seq(query = "dict_pattern")]
pub struct DictPattern {
    key_value_patterns: Vec<KeyValuePatternOrSplatPattern>,
}

#[choice]
pub enum KeyValuePatternOrSplatPattern {
    KeyValuePattern(KeyValuePattern),
    SplatPattern(SplatPattern),
}

#[seq(query = "key_value_pattern")]
pub struct KeyValuePattern {
    key: SimplePattern,
    value: CasePattern,
}

#[seq(query = "keyword_pattern")]
pub struct KeywordPattern {
    identifier: Identifier,
    pattern: SimplePattern,
}

#[seq(query = "splat_pattern")]
pub struct SplatPattern {
    identifier: Identifier,
}

#[seq(query = "class_pattern")]
pub struct ClassPattern {
    dotted_name: DottedName,
    case: Vec<CasePattern>,
}

#[seq(query = "complex_pattern")]
pub struct ComplexPattern {
    left: IntegerOrFloat,
    right: IntegerOrFloat,
}

#[choice]
pub enum IntegerOrFloat {
    Integer(Integer),
    Float(Float),
}

#[choice]
pub enum Parameter {
    Identifier(Identifier),
    Typed(TypedParameter),
    Default(DefaultParameter),
    TypedDefault(TypedDefaultParameter),
    ListSplat(ListSplatPattern),
    TuplePattern(TuplePattern),
    KeywordIdentifier(KeywordIdentifier),
    PositionalSeparator(PositionalSeparator),
    DictionarySplat(DictionarySplatPattern),
}

#[choice]
pub enum Pattern {
    Identifier(Identifier),
    KeywordIdentifier(KeywordIdentifier),
    Subscript(Subscript),
    Attribute(Attribute),
    ListSplatPattern(ListSplatPattern),
    TuplePattern(TuplePattern),
    ListPattern(ListPattern),
}

#[seq(query = "tuple_pattern")]
pub struct TuplePattern {
    elements: Vec<Pattern>,
}

#[seq(query = "list_pattern")]
pub struct ListPattern {
    elements: Vec<Pattern>,
}

#[seq(query = "default_parameter")]
pub struct DefaultParameter {
    name: OneOfIdentifierOrTuplePattern,
    value: Expression,
}

#[choice]
pub enum OneOfIdentifierOrTuplePattern {
    Identifier(Identifier),
    TuplePattern(TuplePattern),
}

#[seq(query = "typed_default_parameter", check)]
pub struct TypedDefaultParameter {
    name: Identifier,
    parameter_type: Type,
    value: Expression,
}

#[choice]
pub enum ListSplatPattern {
    Identifier(Identifier),
    KeywordIdentifier(KeywordIdentifier),
    Subscript(Subscript),
    Attribute(Attribute),
}

#[choice]
pub enum DictionarySplatPattern {
    Identifier(Identifier),
    KeywordIdentifier(KeywordIdentifier),
    Subscript(Subscript),
    Attribute(Attribute),
}

#[seq(query = "as_pattern")]
pub struct AsPattern {
    expression: Expression,
    right: Expression,
}

#[choice]
pub enum Expression {
    ComparisonOperator(ComparisonOperator),
    NotOperator(NotOperator),
    BooleanOperator(BooleanOperator),
    Lambda(Lambda),
    PrimaryExpression(PrimaryExpression),
    ConditionalExpression(ConditionalExpression),
    NamedExpression(NamedExpression),
    AsPattern(AsPattern),
}

#[choice]
pub enum PrimaryExpression {
    Await(Await),
    BinaryOperator(BinaryOperator),
    Identifier(Identifier),
    KeywordIdentifier(KeywordIdentifier),
    String(String),
    ConcatenatedString(ConcatenatedString),
    Integer(Integer),
    Float(Float),
    True(True),
    False(False),
    None(None),
    UnaryOperator(UnaryOperator),
    Attribute(Attribute),
    Subscript(Subscript),
    Call(Call),
    List(List),
    ListComprehension(ListComprehension),
    Dictionary(Dictionary),
    DictionaryComprehension(DictionaryComprehension),
    Set(Set),
    SetComprehension(SetComprehension),
    Tuple(Tuple),
    ParenthesizedExpression(ParenthesizedExpression),
    GeneratorExpression(GeneratorExpression),
    Ellipsis(Ellipsis),
    ListSplatPattern(ListSplatPattern),
}

#[seq(query = "not_operator")]
pub struct NotOperator {
    expression: Expression,
}

#[choice]
pub enum BooleanOperator {
    And(And),
    Or(Or),
}

#[seq(query = "and")]
pub struct And {
    left: PrimaryExpression,
    right: PrimaryExpression,
}

#[seq(query = "or")]
pub struct Or {
    left: PrimaryExpression,
    right: PrimaryExpression,
}

#[seq(query = "binary_operator")]
pub struct BinaryOperator {
    left: PrimaryExpression,
    operator: Operator,
    right: PrimaryExpression,
}

#[seq(query = "unary_operator")]
pub struct UnaryOperator {
    operator: Operator,
    expression: PrimaryExpression,
}

// todo
#[seq(query = "comparison_operator")]
pub struct ComparisonOperator {
    left: PrimaryExpression,
    operators: Vec<Operators>,
}

#[seq(query = "operators")]
pub struct Operators {}

#[seq(query = "lambda")]
pub struct Lambda {
    parameters: Option<Parameters>,
    body: Expression,
}

#[seq(query = "lambda_within_for_clause")]
pub struct LambdaWithinForClause {
    parameters: Option<Parameters>,
    body: ExpressionWithinForClause,
}

#[choice]
enum ExpressionWithinForClause {
    Expression(Expression),
    Lambda(LambdaWithinForClause),
}

#[seq(query = "assignment")]
pub struct Assignment {
    left: LeftHandSide,
    right: RightAssignment,
}

#[choice]
enum RightAssignment {
    RightHandSide(RightHandSide),
}

#[seq(query = "augmented_assignment")]
pub struct AugmentedAssignment {
    left: LeftHandSide,
    operator: Operator,
    right: RightHandSide,
}

#[seq(query = "operator")]
pub struct Operator {}

// inline
#[choice]
enum LeftHandSide {
    Pattern(Pattern),
    PatternList(PatternList),
}

#[seq(query = "pattern_list")]
pub struct PatternList {
    pattern: Pattern,
    next: Vec<Pattern>,
}

// inline
#[choice]
pub enum RightHandSide {
    Expression(Expression),
    Expressions(Expressions),
    Assignement(Assignment),
    AugmentedAssignment(AugmentedAssignment),
    PatternList(PatternList),
    Yield(Yield),
}

#[seq(query = "yield")]
struct Yield {
    yield_: OneOfExpressionOrExpressions,
}

#[choice]
enum OneOfExpressionOrExpressions {
    Expression(Expression),
    Expressions(Expressions),
}

#[seq(query = "attribute")]
pub struct Attribute {
    value: PrimaryExpression,
    subscript: Identifier,
}

#[seq(query = "subscript")]
pub struct Subscript {
    object: PrimaryExpression,
    subscript: Vec<Expression>,
}

#[seq(query = "slice")]
pub struct Slice {
    start: Option<Expression>,
    end: Option<Expression>,
    step: Option<Expression>,
}

#[seq(query = "ellipsis")]
pub struct Ellipsis {}

#[seq(query = "call")]
struct Call {
    function: PrimaryExpression,
    arguments: Vec<GenereratorOrArgumentList>,
}

#[choice]
pub enum GenereratorOrArgumentList {
    Generator(GeneratorExpression),
    ArgumentList(ArgumentList),
}

#[seq(query = "typed_parameter")]
struct TypedParameter {
    name: OneOfIdentifierListSplatDictSplat,
    parameter_type: Type,
}

#[choice]
enum OneOfIdentifierListSplatDictSplat {
    Identifier(Identifier),
    ListSplat(ListSplatPattern),
    DictionarySplat(DictionarySplatPattern),
}

#[choice]
enum Type {
    Expression(Expression),
    SplatType(SplatType),
    GenericType(GenericType),
    UnionType(UnionType),
    ConstrainedType(ConstrainedType),
    MemberType(MemberType),
}

#[seq(query = "splat_type")]
struct SplatType {
    type_: Type,
    generic: Vec<Type>,
}

#[seq(query = "generic_type")]
struct GenericType {
    type_: Identifier,
    parameter: TypedParameter,
}

#[seq(query = "union_type")]
struct UnionType {
    type_1: Type,
    type_2: Type,
}

#[seq(query = "constrained_type")]
struct ConstrainedType {
    type_1: Type,
    type_2: Type,
}

#[seq(query = "member_type")]
struct MemberType {
    type_: Type,
    identifier: Identifier,
}

#[seq(query = "keyword_argument")]
struct KeywordArgument {
    name: Identifier,
    value: Expression,
}

#[seq(query = "list")]
struct List {
    elements: Vec<CollectionElements>,
}

#[seq(query = "set")]
struct Set {
    elements: Vec<CollectionElements>,
}

#[seq(query = "tuple")]
struct Tuple {
    elements: Vec<CollectionElements>,
}

// inline
#[choice]
enum CollectionElements {
    Expression(Expression),
    Yield(Yield),
    ListSplat(ListSplat),
}

#[seq(query = "dictionary")]
struct Dictionary {
    pair: OneOfPairOrDictionarySplat,
}

#[choice]
enum OneOfPairOrDictionarySplat {
    Pair(Pair),
    Dictionary(DictionarySplat),
}

#[seq(query = "pair")]
struct Pair {
    key: Expression,
    value: Expression,
}

#[seq(query = "list_comprehension")]
pub struct ListComprehension {
    body: Expression,
    clauses: ComprehensionClauses,
}

#[seq(query = "dictionary_comprehension")]
pub struct DictionaryComprehension {
    body: Pair,
    clauses: ComprehensionClauses,
}

#[seq(query = "set_comprehension")]
pub struct SetComprehension {
    body: Expression,
    clauses: ComprehensionClauses,
}

#[seq(query = "generator_expression")]
pub struct GeneratorExpression {
    body: Expression,
    clauses: ComprehensionClauses,
}

#[seq(query = "comprehension_clause")]
pub struct ComprehensionClauses {
    for_clause: ForInClause,
    next: Vec<OneOfForIfClause>,
}

#[choice]
enum OneOfForIfClause {
    ForClause(ForInClause),
    IfClause(IfClause),
}

#[seq(query = "parenthesized_expression")]
pub struct ParenthesizedExpression {
    expression: Expression,
}

#[seq(query = "for_in_clause")]
struct ForInClause {
    left: Expression,
    right: Expression,
}

#[seq(query = "if_clause")]
struct IfClause {
    condition: Expression,
}

#[seq(query = "conditional_expression")]
struct ConditionalExpression {
    initial: Expression,
    if_expression: Expression,
    else_expression: Expression,
}

#[seq(query = "concatenated_string")]
struct ConcatenatedString {
    base: String,
    concat: Vec<String>,
}

#[seq(query = "string")]
struct String {}

#[seq(query = "interpolation")]
struct Interpolation {
    expression: FExpression,
    type_conversion: Option<TypeConversion>,
    format_specifier: Option<FormatSpecifier>,
}

#[choice]
enum FExpression {
    Expression(Expression),
    ExpressionList(ExpressionList),
    PatternList(PatternList),
    Yield(Yield),
}

#[seq(query = "format_specifier")]
struct FormatSpecifier {
    specifiers: Vec<Specifier>,
}

#[choice]
enum Specifier {
    Token(String),
    FormatExpression(Interpolation),
}

#[seq(query = "type_conversion")]
struct TypeConversion {}

#[seq(query = "integer")]
struct Integer {}

#[seq(query = "float")]
struct Float {}

#[seq(query = "identifier", hover)]
struct Identifier {}

#[choice]
pub enum KeywordIdentifier {
    PrintExecAsyncAwait(KeywordPrintExecAsyncAwait),
    TypeMatch(TypeMatch),
}
#[seq(query = "keyword_print_exec_async_await")]
pub struct KeywordPrintExecAsyncAwait {
    keyword: PrintExecAsyncAwait,
    identifier: Identifier,
}

#[seq(query = "print_exec_async_await")]
pub struct PrintExecAsyncAwait {}

#[seq(query = "keyword_type_match")]
pub struct KeywordTypeMatch {
    keyword: TypeMatch,
    identifier: Identifier,
}

#[seq(query = "type_match")]
pub struct TypeMatch {}

#[seq(query = "true")]
struct True {}

#[seq(query = "false")]
struct False {}

#[seq(query = "none")]
struct None {}

#[seq(query = "await")]
pub struct Await {
    expression: PrimaryExpression,
}

#[seq(query = "positional_separator")]
pub struct PositionalSeparator {}
