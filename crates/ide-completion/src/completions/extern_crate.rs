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
//- /lib.rs crate:main deps:other_crate
extern crate $0
//- /other_crate/lib.rs crate:other_crate
// nothing here
"#,
            expect![[r#"
            md other_crate;
        "#]],
        );
    }

    #[test]
    fn multiple_crates() {
        check(
            r#"
//- /lib.rs crate:main deps:crate_a,crate_b
extern crate $0
//- /crate_a/lib.rs crate:crate_a
// nothing here
//- /crate_B/lib.rs crate:crate_b
// nothing here
"#,
            expect![[r#"
            md crate_a;
            md crate_b;
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
