#include <tree_sitter/parser.h>

#include <wctype.h>

enum TokenType {
    FIELD_SELECTOR,
};

void *tree_sitter_IEC61131_3_2_external_scanner_create() { return NULL; }

void tree_sitter_IEC61131_3_2_external_scanner_destroy(void *p) {}

unsigned tree_sitter_IEC61131_3_2_external_scanner_serialize(void *payload, char *buffer) { return 0; }

void tree_sitter_IEC61131_3_2_external_scanner_deserialize(void *p, const char *b, unsigned n) {}

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void skip(TSLexer *lexer) { lexer->advance(lexer, true); }

static bool is_identifier_char(int32_t c) {
    return iswalnum(c) || c == '_';
}

static bool scan_field_selector(TSLexer *lexer) {
    // Skip any leading whitespace (if necessary)
    while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
        skip(lexer);
    }

    // Immediately mark it as FIELD_SELECTOR, even if no identifier characters follow
    lexer->result_symbol = FIELD_SELECTOR;

    // Continue scanning for valid identifier characters
    while (lexer->lookahead != '\n' && lexer->lookahead != '\r' && lexer->lookahead != 0 && is_identifier_char(lexer->lookahead)) {
        advance(lexer);
    }

    return true;
}

bool tree_sitter_IEC61131_3_2_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    if (valid_symbols[FIELD_SELECTOR]) {
        return scan_field_selector(lexer);
    }

    return false;
}