use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Pat};

#[derive(Default, FromMeta)]
#[darling(default)]
struct CxxAbiArgs {
    name: String,
    ctor: bool,
}

#[proc_macro_attribute]
pub fn cxxabi(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    let cxxabi_args = match CxxAbiArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(err) => return TokenStream::from(err.write_errors()),
    };

    let name = &sig.ident;
    let args = &sig.inputs.clone().into_iter().collect::<Vec<_>>();
    let vars = &sig
        .inputs
        .clone()
        .into_iter()
        .filter_map(|e: FnArg| match e {
            FnArg::Typed(ty) => Some(ty.pat),
            _ => None,
        })
        .collect::<Vec<_>>();

    let stmts = &block.stmts;
    let output = &sig.output;

    let cxx_name = &cxxabi_args.name;
    let thiscall_name = format_ident!("{}_cxx", name);

    let tokens = if !cxxabi_args.ctor {
        let is_mutable = match &args[0] {
            FnArg::Typed(ty) => {
                if let Pat::Ident(ref ident) = *ty.pat {
                    ident.mutability.is_none()
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        };

        let mutability = format_ident!("{}", if is_mutable { "mut" } else { "const" });

        let args = args.iter().skip(1).cloned().collect::<Vec<_>>();
        let vars = vars.iter().skip(1).cloned().collect::<Vec<_>>();

        quote! {
            #[export_name = #cxx_name]
            extern "stdcall" fn #thiscall_name(#(#args),*) #output {
                let this = get_this_ptr_cxx();
                let class = this as *#mutability std::os::raw::c_void as *#mutability _;

                #name(class, #(#vars),*)
            }
        }
    } else {
        quote! {
            #[export_name = #cxx_name]
            extern "stdcall" fn #thiscall_name(#(#args),*) #output {
                #name(#(#vars),*)
            }
        }
    };

    TokenStream::from(quote! {
        #tokens

        #(#attrs)* #vis #sig {
            #(#stmts)*
        }
    })
}
