use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Shape)]
pub fn rect_shape_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_rect_shape(&ast)
}

fn impl_rect_shape(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Shape for #name {
            ///Associated function used to create a new Shape
            fn new(length: f32, width: f32, name: &'static str) -> Self {
                #name {
                    length,
                    width,
                    name,
                }
            }
            ///Area method
            fn area(&self) -> f32 {
                self.length * self.width
            }

            fn get_length(&self) -> f32 {
                self.length
            }

            fn set_length(&mut self, length: f32) {
                self.length = length;
            }

            fn get_width(&self) -> f32 {
                self.width
            }

            fn set_width(&mut self, width: f32) {
                self.width = width;
            }

            fn get_name(&self) -> &'static str {
                self.name
            }

            fn set_name(&mut self, name: &'static str) {
                self.name = name;
            }
        }
    };
    gen.into()
}