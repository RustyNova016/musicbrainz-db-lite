mod derives;
mod database_atributes;
mod utils;
mod sql_gen;
extern crate proc_macro;

use darling::{util::PathList, FromDeriveInput, FromMeta};
use derives::main_entity::derive_main_entity_impl;
use proc_macro::TokenStream;
use quote::quote;
use sql_gen::{
    get_insert_fields_from_idents, get_insert_values_fields_from_idents,
    get_on_conflict_fields_from_idents,
};
use syn::{Data, Path};
use crate::database_atributes::DatabaseAtribute;
use crate::utils::field_in_pathlist;

#[proc_macro_derive(MainEntity, attributes(database))]
pub fn derive_main_entity(item: TokenStream) -> TokenStream {
    derive_main_entity_impl(item)
}

#[proc_macro_derive(Upsert, attributes(database))]
pub fn derive_upsert(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let args = match DatabaseAtribute::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();

            let sql_statement = format!("INSERT INTO `{}` {} VALUES {} ON CONFLICT DO UPDATE SET {} RETURNING *;", 
                args.table,
                get_insert_fields_from_idents(fields),
                get_insert_values_fields_from_idents(fields, &args.ignore_insert_keys),
                get_on_conflict_fields_from_idents(fields, &args.ignore_update_keys)
            );

            let mut binds = quote!{};

            // Generate the binds
            for field in fields {
                if !field_in_pathlist(field, &args.ignore_insert_keys) {
                    let identifier = field.ident.as_ref().unwrap();
                    binds.extend(quote!{
                        query = query.bind(&self.#identifier);
                    })
                }
            }

            quote! {
                #[automatically_derived]
                impl #struct_identifier {
                    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, sqlx::Error> {
                        let mut query = sqlx::query_as(#sql_statement);
                        #binds

                        query.fetch_one(conn).await
                    }
                }
            }
        }
        _ => unimplemented!()
    }.into()
}