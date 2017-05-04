#[macro_use]
extern crate darling;

extern crate syn;

use darling::FromDeriveInput;

#[derive(Default, FromMetaItem, PartialEq, Debug)]
#[darling(default)]
struct Lorem {
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput, PartialEq, Debug)]
#[darling(attributes(darling_demo))]
struct Container {
    ident: syn::Ident,
    vis: syn::Visibility,
    generics: syn::Generics,
    lorem: Lorem
}

#[derive(FromDeriveInput, PartialEq, Debug)]
#[darling(attributes(darling_demo))]
struct TraitContainer {
    ident: syn::Ident,
    generics: syn::Generics,
    lorem: Lorem,
}

#[test]
fn simple() {
    let di = syn::parse_derive_input(r#"
        #[derive(Foo)]
        #[darling_demo(lorem(ipsum))]
        pub struct Bar;
    "#).unwrap();

    assert_eq!(Container::from_derive_input(&di).unwrap(), Container {
        ident: syn::Ident::new("Bar"),
        vis: syn::Visibility::Public,
        generics: Default::default(),
        lorem: Lorem {
            ipsum: true,
            dolor: None,
        }
    });
}

#[test]
fn trait_type() {
    let di = syn::parse_derive_input(r#"
        #[derive(Foo)]
        #[darling_demo(lorem(dolor = "hello"))]
        pub struct Bar;
    "#).unwrap();

    assert_eq!(TraitContainer::from_derive_input(&di).unwrap(), TraitContainer {
        ident: syn::Ident::new("Bar"),
        generics: Default::default(),
        lorem: Lorem {
            ipsum: false,
            dolor: Some("hello".to_owned()),
        }
    });
}