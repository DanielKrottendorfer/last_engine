
use proc_macro2::Ident;
use quote::quote;

pub fn setup(function_name: &Ident, field_ident: &Ident, uniform_name: &str) -> proc_macro2::TokenStream {
    quote!{
        fn #function_name(&mut self){
            self.#field_ident = unsafe{gl::GetUniformLocation(self.program_id,CString::new(#uniform_name).unwrap().as_ptr())};
            if self.#field_ident < 0 {
                panic!("error {} could not be located",#uniform_name);
            }
        }               
    }
}

pub fn i1_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : i32){
            
            unsafe {
                gl::Uniform1i(
                    self.#field_ident,
                    #uniform_name
                );
            }
        }
    }
}

pub fn i1v_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : i32){
            
            unsafe {
                gl::Uniform1i(
                    self.#field_ident,
                    #uniform_name
                );
            }
        }
    }
}

pub fn vec4_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : Vector4<f32>){
            
            unsafe {
                gl::Uniform4fv(
                    self.#field_ident,
                    1,
                    cgmath::conv::array4(#uniform_name).as_ptr() as *const f32
                );
            }
        }
    }
}

pub fn f1_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : f32){
            
            unsafe {
                gl::Uniform1f(
                    self.#field_ident,
                    #uniform_name
                );
            }
        }
    }
}


pub fn vec3_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : Vector3<f32>){
            
            unsafe {
                gl::Uniform3fv(
                    self.#field_ident,
                    1,
                    cgmath::conv::array3(#uniform_name).as_ptr() as *const f32
                );
            }
        }
    }
}

pub fn mat4_setter(function_name: Ident, field_ident: Ident, uniform_name: Ident) -> proc_macro2::TokenStream {

    quote!{
        pub fn #function_name(&self,#uniform_name : Matrix4<f32>){
            
            unsafe {
                gl::UniformMatrix4fv(
                    self.#field_ident,
                    1,
                    gl::FALSE,
                    cgmath::conv::array4x4(#uniform_name).as_ptr() as *const f32
                );
            }
        }
    }
}