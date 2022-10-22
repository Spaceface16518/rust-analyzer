use crate::context::{CompletionContext, PathCompletionCtx};

use super::Completions;

pub(crate) fn complete_extern_crate(
    acc: &mut Completions,
    ctx: &CompletionContext<'_>,
    path_ctx: &PathCompletionCtx,
) {
    acc.add_crate_roots(ctx, path_ctx)
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
    fn single_crate() {
        check(
            r#"
//- /lib.rs crate:main extern-prelude:other_crate
extern crate $0
"#,
            expect![[r#"
            md other_crate
        "#]],
        );
    }

    #[test]
    fn multiple_crates() {
        check(
            r#"
//- /lib.rs crate:main extern-prelude:crate_a,crate_b
extern crate $0
"#,
            expect![[r#"
            md crate_a
            md crate_b
        "#]],
        )
    }

    #[test]
    fn semicolon() {
        check(
            r#"
//- /lib.rs crate:main deps:other_crate
extern crate other_crate$0
//- /other_crate/lib.rs crate:other_crate
// nothing here
"#,
            expect![[r#"
            md other_crate;
        "#]],
        );
    }
}
