

#![warn(use_self)]
#![allow(dead_code)]
#![allow(should_implement_trait)]
#![allow(plugin_as_library)]

#[macro_use]
extern crate clippy_mini_macro_test;

fn main() {}

mod use_self {
    struct Foo {}

    impl Foo {
        fn new() -> Foo {
            Foo {}
        }
        fn test() -> Foo {
            Foo::new()
        }
    }

    impl Default for Foo {
        fn default() -> Foo {
            Foo::new()
        }
    }
}

mod internal_macro_fails {
    macro_rules! expand_wrong_functions {
        () => (
            fn new() -> FooWrong {
                FooWrong {}
            }
            fn test() -> FooWrong {
                FooWrong::new()
            }
        )
    }

    macro_rules! expand_better_functions {
        () => (
            fn new() -> Self {
                Self {}
            }
            fn test() -> Self {
                Self::new()
            }
        )
    }

    struct FooWrong {}

    impl FooWrong {
        expand_wrong_functions!();
    }

    struct FooBetter {}

    impl FooBetter {
        expand_better_functions!();
    }
}

mod external_macro_not_linted {
    struct FooWrong {}

    impl FooWrong {
        use_self_expand_wrong_functions!();
    }
}

mod better {
    struct Foo {}

    impl Foo {
        fn new() -> Self {
            Self {}
        }
        fn test() -> Self {
            Self::new()
        }
    }

    impl Default for Foo {
        fn default() -> Self {
            Self::new()
        }
    }
}

// todo the lint does not handle lifetimed struct
// the following module should trigger the lint on the third method only
mod lifetimes {
    struct Foo<'a> {
        foo_str: &'a str,
    }

    impl<'a> Foo<'a> {
        // Cannot use `Self` as return type, because the function is actually `fn foo<'b>(s: &'b str) ->
        // Foo<'b>`
        fn foo(s: &str) -> Foo {
            Foo { foo_str: s }
        }
        // cannot replace with `Self`, because that's `Foo<'a>`
        fn bar() -> Foo<'static> {
            Foo { foo_str: "foo" }
        }

        // `Self` is applicable here
        fn clone(&self) -> Foo<'a> {
            Foo { foo_str: self.foo_str }
        }
    }
}
