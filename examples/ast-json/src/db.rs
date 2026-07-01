use crate::generated::Document;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::{BaseDatabase, BaseDb, FileManager};
use auto_lsp::lsp_types::Url;
use auto_lsp::{configure_parser, lsp_types};

configure_parser!(
    JSON_PARSER,
        language: tree_sitter_json::LANGUAGE,
        ast_root: Document
);

pub fn create_json_db(source_code: &'static [&str]) -> impl BaseDatabase {
    let mut db = BaseDb::default();
    source_code.iter().enumerate().for_each(|(i, source_code)| {
        let url = Url::parse(&format!("file:///test{i}.json")).expect("Failed to parse URL");

        let file = File::from_string()
            .db(&db)
            .source(source_code.to_string())
            .url(&url)
            .parsers(&JSON_PARSER)
            .encoding(&lsp_types::PositionEncodingKind::UTF8)
            .call()
            .expect("Failed to create file");

        db.add_file(file).expect("Failed to add file");
    });

    db
}
