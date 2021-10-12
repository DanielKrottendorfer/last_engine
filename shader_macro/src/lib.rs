extern crate cgmath;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

mod templates;
use templates::*;
 
#[proc_macro_derive(ShaderProgram)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let mut set_uniform_functions = proc_macro2::TokenStream::new();
    let mut setup_functions = proc_macro2::TokenStream::new();

    let mut setup_function_idents = Vec::new();
    let mut field_idents = Vec::new();

    let mut contains_field_program_id = false;
    let mut only_contains_shader_fields = true;

    if let syn::Data::Struct(ds) = input.data{
        for field in ds.fields{
            
            //extracting the types of the field
            let field_type_string = if let syn::Type::Path(tp) = field.ty {
                tp.path.segments.first().unwrap().ident.to_string()
            }else{
                continue;
            };

            let field_ident = field.ident.unwrap();
            let field_ident_string = field_ident.to_string();

            if !field_type_string.eq("i32"){
                if field_ident_string.eq("program_id") && field_type_string.eq("u32"){
                    contains_field_program_id = true;
                }
                continue;
            }


            let ident_segments:Vec<&str> = field_ident_string.splitn(3,'_').map(|s| s).collect();
            

            if ident_segments.len() == 3 && ident_segments[0].eq("uniform"){
                field_idents.push(field_ident.clone());

                let uniform_type = ident_segments[1];
                let uniform_name = ident_segments[2];


                let setup_function_ident = quote::format_ident!("setup_{}",uniform_name);

                setup_functions.extend(setup(&setup_function_ident, &field_ident, uniform_name));
                setup_function_idents.push(setup_function_ident);

                                                    
                let uniform_name_ident =        quote::format_ident!("{}",uniform_name);
                let set_uniform_function_ident = quote::format_ident!("set_{}",uniform_name_ident);
                let set_uniform_function = match uniform_type {
                    "f"    => f1_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    "i"    => i1_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    "i1v"  => i1v_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    "vec3" => vec3_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    "vec4" => vec4_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    "mat4" => mat4_setter(set_uniform_function_ident, field_ident, uniform_name_ident),
                    _ => {
                        panic!("uniform type not recognized => {}",uniform_type);
                    }
                };

                set_uniform_functions.extend(set_uniform_function);

            } else {
                only_contains_shader_fields = false;
            }
        }
    }else{
        panic!("no data in struct")
    }

    if !contains_field_program_id {
        panic!("program_id field missing");
    }

    set_uniform_functions.extend(quote!{
        pub fn use_program(&self){
            unsafe{
                gl::UseProgram(self.program_id);
            }
        }
    });

    let ident = input.ident;

    if only_contains_shader_fields {
        set_uniform_functions.extend(quote! {
            #[inline(always)]
            pub fn new() -> Self {
                #ident {
                    #( #field_idents :-1,)*
                    program_id:0
                }
            }
        })
    }


    setup_functions.extend(quote!{
        
        pub fn setup(&mut self,program: &u32){
            self.program_id = program.clone();

            #(self. #setup_function_idents() ;)*
        }
        
        pub fn cleanup(&self){
            unsafe {
                gl::DeleteProgram(self.program_id);
            }
        }
    });
    
    let temp = quote! {
        impl #ident {
            #set_uniform_functions
            #setup_functions
        }
    };

    //println!("{}",temp.to_string());

    TokenStream::from(temp)
}

#[proc_macro]
pub fn shader_program(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {


    let input = proc_macro2::TokenStream::from(_item);
    let q:proc_macro2::TokenStream = input.into_iter().map(|a|{
        a
    }).collect();

    let q = quote!{
        fn answer (){
            #q 
        }
    };
    //println!("{}",q.to_string());
    proc_macro::TokenStream::from(q)
}
