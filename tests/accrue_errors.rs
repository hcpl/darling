//! These tests verify that multiple errors will be collected up from throughout
//! the parsing process and returned correctly to the caller.

#[macro_use]
extern crate darling;
extern crate syn;

use darling::ast;
use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(accrue))]
struct Lorem {
    ipsum: String,
    dolor: Dolor,
    body: ast::Body<(), LoremField>,
}

#[derive(Debug, FromMetaItem)]
struct Dolor {
    sit: bool
}

#[derive(Debug, FromField)]
#[darling(attributes(accrue))]
struct LoremField {
    ident: Option<syn::Ident>,
    aliased_as: syn::Ident,
}

#[test]
fn bad_type_and_missing_fields() {
    let input = syn::parse_str(r#"
    #[accrue(ipsum = true, dolor(amet = "Hi"))]
    pub struct NonConforming {
        foo: ()
    }
    "#).unwrap();

    let s_result: ::darling::Error = Lorem::from_derive_input(&input).unwrap_err();
    //assert_eq!(3, s_result.len());
    let err = s_result.flatten();
    println!("{}", err);
    assert_eq!(3, err.len());
}

#[test]
fn body_only_issues() {
    let input = syn::parse_str(r#"
    #[accrue(ipsum = "Hello", dolor(sit))]
    pub struct NonConforming {
        foo: (),
        bar: bool,
    }
    "#).unwrap();

    let s_err: ::darling::Error = Lorem::from_derive_input(&input).unwrap_err();
    println!("{:?}", s_err);
    assert_eq!(2, s_err.len());
}

#[derive(Debug, FromMetaItem)]
enum Week {
    Monday,
    Tuesday {
        morning: bool,
        afternoon: String,
    },
    Wednesday(Dolor),
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(accrue))]
struct Month {
    schedule: Week
}

#[test]
fn error_in_enum_fields() {
    let input = syn::parse_str(r#"
    #[accrue(schedule(tuesday(morning = "yes")))]
    pub struct NonConforming {
        foo: (),
        bar: bool,
    }
    "#).unwrap();

    let s_err: ::darling::Error = Month::from_derive_input(&input).unwrap_err();
    assert_eq!(2, s_err.len());
    let err = s_err.flatten();
    // TODO add tests to check location path is correct
    println!("{}", err);
}

#[test]
fn error_in_newtype_variant() {
    let input = syn::parse_str(r#"
    #[accrue(schedule(wednesday(sit = "yes")))]
    pub struct NonConforming {
        foo: (),
        bar: bool,
    }
    "#).unwrap();

    let s_err: ::darling::Error = Month::from_derive_input(&input).unwrap_err();
    assert_eq!(1, s_err.len());
    println!("{}", s_err);
    println!("{}", s_err.flatten());
}
