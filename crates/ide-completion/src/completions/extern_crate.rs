use hir::{HasAttrs, ScopeDef};
use ide_db::SymbolKind;
use syntax::{ast, AstNode};

use crate::{context::CompletionContext, CompletionItem, CompletionItemKind};

use super::Completions;

/// Complete `extern crate $0` with modules from the extern prelude.
pub(crate) fn complete_extern_crate(
    acc: &mut Completions,
    ctx: &CompletionContext<'_>,
    name_ref: &Option<ast::NameRef>,
) {
    let name_ref = name_ref.as_ref().map(|n| n.syntax().text());
    ctx.process_all_names(&mut |name, res| match (&name_ref, name.to_smol_str().as_str(), res) {
        (Some(name_ref), name, ScopeDef::ModuleDef(hir::ModuleDef::Module(m)))
            if m.is_crate_root(ctx.db) && name_ref == name =>
        {
            // manually add a module completion item since `add_module` is not usable without a
            // path context
            let kind = CompletionItemKind::SymbolKind(SymbolKind::Module);
            let source_range = ctx.source_range();
            let label = name;
            let mut completion_item = CompletionItem::new(kind, source_range, label);
            completion_item.set_documentation(m.docs(ctx.db));
            completion_item.add_to(acc);
        }
        _ => (),
    });
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::tests::completion_list_no_kw;

    fn check(ra_fixture: &str, expect: Expect) {
        let actual = completion_list_no_kw(ra_fixture);
        expect.assert_eq(&actual);
    }

    #[test]
    fn test_single_crate() {
        check(
            r#"
//- /lib.rs crate:main deps:other_crate extern-prelude:other_crate
extern crate $0
//- /other_crate/lib.rs crate:other_crate
// nothing here
"#,
            expect![
                "
            md other_crate
            "
            ],
        );
    }

    #[test]
    fn test_multiple_crates() {
        check(
            r#"
//- /lib.rs crate:main deps:crate_a,crate_b extern-prelude:crate_a,crate_b
//- /crate_a/lib.rs crate:crate_a
// nothing here
//- /crate_b/lib.rs crate:crate_b
// nothing here
extern crate $0
"#,
            expect![
                "
            md crate_a
            md crate_b
            "
            ],
        )
    }

    #[test]
    fn test_single_crate_prefix() {
        check(
            r#"
//- /lib.rs crate:main deps:other_crate extern-prelude:other_crate
extern crate o$0
//- /other_crate/lib.rs crate:other_crate
// nothing here
"#,
            expect![
                "
            md other_crate
            "
            ],
        );
    }
}
