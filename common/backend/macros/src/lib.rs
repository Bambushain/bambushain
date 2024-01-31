use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Responder)]
pub fn responder(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl actix_web::Responder for #name {
            type Body = actix_web::body::BoxBody;

            fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                if req.method() == actix_web::http::Method::POST {
                    actix_web::HttpResponse::Created()
                } else {
                    actix_web::HttpResponse::Ok()
                }
                .body(serde_json::to_string(&self).unwrap())
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
