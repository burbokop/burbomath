#![deny(warnings)]

use litrs::Literal;
use proc_macro::TokenStream;
use quote::quote;

fn validate_numeric_literal(
    input: TokenStream,
    predicate: fn(&Literal<String>) -> Result<(), TokenStream>,
) -> Result<(), TokenStream> {
    let input = input.into_iter().collect::<Vec<_>>();
    if input.len() != 1 {
        let msg = format!(
            "expected exactly one input token, but found {}",
            input.len()
        );
        return Err(quote! { compile_error!(#msg) }.into());
    }

    let lit = match Literal::try_from(&input[0]) {
        Ok(lit) => lit,
        Err(e) => return Err(e.to_compile_error()),
    };

    match lit {
        Literal::Integer(_) | Literal::Float(_) => predicate(&lit),
        _ => {
            let msg = format!("expected integer or float literal, but found {}", lit);
            Err(quote! { compile_error!(#msg) }.into())
        }
    }
}

fn must_be_positive(lit: &Literal<String>) -> Result<(), TokenStream> {
    match lit {
        Literal::Integer(lit) => {
            println!("int lit: {}", lit);
            if lit.value::<u128>().unwrap() == 0 {
                let msg = format!("expected positive number, but found {}", lit);
                return Err(quote! { compile_error!(#msg) }.into());
            }
        }
        Literal::Float(lit) => {
            println!("float lit: {}", lit);
            if lit.raw_input().parse::<f64>().unwrap() == 0. {
                let msg = format!("expected positive number, but found {}", lit);
                return Err(quote! { compile_error!(#msg) }.into());
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

#[proc_macro]
pub fn non_neg(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.clone().into();

    if let Err(err) = validate_numeric_literal(input, |_| Ok(())) {
        return err;
    }

    quote! {
        unsafe { burbomath::NonNeg::new_const_unchecked(#input2) }
    }
    .into()
}

#[proc_macro]
pub fn positive(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.clone().into();

    if let Err(err) = validate_numeric_literal(input, must_be_positive) {
        return err;
    }

    quote! {
        unsafe { burbomath::Positive::new_const_unchecked(#input2) }
    }
    .into()
}
