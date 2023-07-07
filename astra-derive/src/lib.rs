use darling::FromField;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Fields, Item};

#[derive(Debug, FromField)]
#[darling(attributes(astra))]
struct FieldOptions {
    pub key: String,
    #[darling(default)]
    pub public_array: bool,
    #[darling(default)]
    pub id: bool,
}

#[proc_macro_derive(Astra, attributes(astra))]
pub fn astra(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Item);

    let mut item = match input {
        Item::Struct(item_struct) => item_struct,
        item => {
            return quote_spanned! { item.span() =>
                compile_error!("#[astra] can only be used on structs");
            }
            .into();
        }
    };
    let fields = if let Fields::Named(fields) = &mut item.fields {
        fields
    } else {
        return quote_spanned! { item.span() =>
            compile_error!("#[astra] can only be used on structs with named fields");
        }
        .into();
    };

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let mut extractors = vec![];
    let mut initializers = vec![];
    let mut setters = vec![];
    let mut field_options = vec![];
    let mut public_array_entry = quote! {};
    let mut unique_book_entry = quote! {};
    for f in &fields.named {
        let options = match FieldOptions::from_field(f) {
            Ok(options) => options,
            Err(err) => return err.write_errors().into(),
        };
        let key = &options.key;
        let ident = f.ident.as_ref().unwrap();
        if options.public_array {
            public_array_entry = quote! {
                impl #impl_generics astra_formats::PublicArrayEntry for #name #ty_generics #where_clause {
                    fn get_key(&self) -> &str {
                        &self.#ident
                    }

                    fn key_identifier() -> &'static str {
                        #key
                    }
                }
            };
        }
        if options.id {
            unique_book_entry = quote! {
                impl #impl_generics astra_formats::UniqueBookEntry for #name #ty_generics #where_clause {
                    fn get_id(&self) -> &str {
                        &self.#ident
                    }
                }
            };
        }
        extractors.push(quote! {
            let raw_value = values.remove(#key)
                .ok_or_else(|| astra_formats::error::anyhow!("expected value for '{}'", #key))?;
            let #ident = astra_formats::FromSheetParamAttribute::from_sheet_param_attribute(raw_value)?;
        });
        initializers.push(quote! { #ident, });
        setters.push(quote! {
            map.insert(#key.to_string(), self.#ident.to_sheet_param_attribute());
        });
        field_options.push(options);
    }

    quote! {
        impl #impl_generics astra_formats::FromSheetDataParam for #name #ty_generics #where_clause {
            fn from_sheet_data_param(
                mut values: astra_formats::indexmap::IndexMap<String, String>
            ) -> astra_formats::error::Result<Self> {
                #(#extractors)*
                Ok(Self {
                    #(#initializers)*
                })
            }
        }

        impl #impl_generics astra_formats::ToSheetDataParam for #name #ty_generics #where_clause {
            fn to_sheet_data_param_values(
                &self
            ) -> astra_formats::indexmap::IndexMap<String, String> {
                use astra_formats::ToSheetParamAttribute;
                let mut map = astra_formats::indexmap::IndexMap::new();
                #(#setters)*
                map
            }
        }

        #public_array_entry

        #unique_book_entry
    }
    .into()
}

#[proc_macro_derive(AstraBook)]
pub fn astra_book(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Item);

    let mut item = match input {
        Item::Struct(item_struct) => item_struct,
        item => {
            return quote_spanned! { item.span() =>
                compile_error!("#[astra_book] can only be used on structs");
            }
            .into();
        }
    };
    let fields = if let Fields::Named(fields) = &mut item.fields {
        fields
    } else {
        return quote_spanned! { item.span() =>
            compile_error!("#[astra_book] can only be used on structs with named fields");
        }
        .into();
    };

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let sheet_count = fields.named.len();
    let mut from_sheet_conversions = vec![];
    let mut ref_to_sheet_conversions = vec![];
    let mut to_sheet_conversions = vec![];
    for f in &fields.named {
        let ident = f.ident.as_ref().unwrap();
        from_sheet_conversions.push(quote! {
            #ident: value.sheets
                .pop()
                .ok_or_else(|| astra_formats::error::anyhow!("ran out of sheets while converting"))?
                .try_into()?,
        });
        ref_to_sheet_conversions.push(quote! {
            sheets.push((&value.#ident).into());
        });
        to_sheet_conversions.push(quote! {
            sheets.push(value.#ident.into());
        });
    }
    from_sheet_conversions.reverse();

    quote! {
        impl #impl_generics astra_formats::AstraBook for #name #ty_generics #where_clause {
            fn load<PathTy: AsRef<std::path::Path>>(path: PathTy) -> astra_formats::error::Result<Self> {
                astra_formats::Book::load(path)?.try_into()
            }

            fn save<PathTy: AsRef<std::path::Path>>(&self, path: PathTy) -> astra_formats::error::Result<()> {
                let book: astra_formats::Book = self.into();
                book.save(path)
            }

            fn from_string(contents: impl AsRef<str>) -> astra_formats::error::Result<Self> {
                astra_formats::Book::from_string(contents.as_ref())?.try_into()
            }

            fn to_string(&self) -> astra_formats::error::Result<String> {
                let book: astra_formats::Book = self.into();
                book.serialize()
            }
        }

        impl #impl_generics TryFrom<astra_formats::Book> for #name #ty_generics #ty_generics #where_clause {
            type Error = astra_formats::error::Error;

            fn try_from(mut value: astra_formats::Book) -> astra_formats::error::Result<Self> {
                Ok(Self {
                    #(#from_sheet_conversions)*
                })
            }
        }

        impl #impl_generics From<&#name #ty_generics> for astra_formats::Book #ty_generics #where_clause {
            fn from(value: &#name #ty_generics) -> Self {
                let mut sheets = vec![];
                #(#ref_to_sheet_conversions)*
                Self {
                    count: #sheet_count,
                    sheets,
                }
            }
        }

        impl #impl_generics From<#name #ty_generics> for astra_formats::Book #ty_generics #where_clause {
            fn from(value: #name #ty_generics) -> Self {
                let mut sheets = vec![];
                #(#to_sheet_conversions)*
                Self {
                    count: #sheet_count,
                    sheets,
                }
            }
        }
    }
    .into()
}
