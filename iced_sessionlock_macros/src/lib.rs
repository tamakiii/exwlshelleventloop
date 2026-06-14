use darling::{FromDeriveInput, ast::Data, util::Ignored};
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, Generics, Ident, Path, Variant, Visibility};

use quote::quote;

#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn to_session_message(
    _attr: TokenStream2,
    input: TokenStream2,
) -> manyhow::Result<TokenStream2> {
    let derive_input = syn::parse2::<DeriveInput>(input)?;
    let attrs = &derive_input.attrs;
    let MessageEnum {
        vis,
        ident,
        generics,
        data,
    } = MessageEnum::from_derive_input(&derive_input)?;

    let (impl_gen, ty_gen, where_gen) = generics.split_for_impl();
    let variants = data.take_enum().unwrap();

    let unlock_action: Path = syn::parse_quote!(iced_sessionlock::actions::UnLockAction);
    let locked_info: Path = syn::parse_quote!(iced_sessionlock::LockedInfo);
    let from_locked_info: Path = syn::parse_quote!(iced_sessionlock::FromLockedInfo);

    let try_into = quote! {
        impl #impl_gen TryInto<#unlock_action> for #ident #ty_gen #where_gen {
            type Error = Self;

            fn try_into(self) -> Result<#unlock_action, Self::Error> {
                match self {
                    Self::UnLock => Ok(#unlock_action),
                    _ => Err(self)
                }
            }
        }

        impl #impl_gen #from_locked_info for #ident #ty_gen #where_gen {
            fn get(_info: #locked_info) -> Self {
                Self::Locked
            }
        }
    };

    Ok(quote! {
        #(#attrs)*
        #vis enum #ident #ty_gen #where_gen {
            #(#variants,)*
            UnLock,
            /// Delivered by the runtime once the compositor has activated the
            /// lock (the `ext_session_lock_v1.locked` event)
            Locked
        }

        #try_into
    })
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_any))]
struct MessageEnum {
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    data: Data<Variant, Ignored>,
}
