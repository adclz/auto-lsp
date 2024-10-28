import XCTest
import SwiftTreeSitter
import TreeSitterIec6113132

final class TreeSitterIec6113132Tests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_iec61131_3_2())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Iec6113132 grammar")
    }
}
