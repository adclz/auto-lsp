pub(crate) type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! snap {
    ($input: expr) => {{
        use ::auto_lsp::default::db::tracked::get_ast;
        use ::auto_lsp::default::db::BaseDatabase;

        let db = $crate::db::create_python_db(&[$input]);
        let file = db
            .get_file(&::auto_lsp::lsp_types::Url::parse("file:///test0.py").unwrap())
            .unwrap();

        insta::assert_debug_snapshot!(get_ast(&db, file));

        let errors =
            get_ast::accumulated::<auto_lsp::core::errors::ParseErrorAccumulator>(&db, file);
        if !errors.is_empty() {
            panic!("Errors found: {:#?}", errors);
        }
        Ok(())
    }};
}
