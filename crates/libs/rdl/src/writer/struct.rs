use super::*;
use windows_metadata::AsRow;

pub fn write_struct(item: &metadata::reader::TypeDef) -> TokenStream {
    if is_nested_type(item) {
        return quote! {};
    }

    let namespace = item.namespace();
    let name = write_ident(item.name());

    let fields = item
        .fields()
        .map(|field| write_field(namespace, item, &field));

    let keyword = if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! {
        #(#custom_attrs)*
        #keyword #name {
            #(#fields)*
        }
    }
}

fn write_field(
    namespace: &str,
    parent: &metadata::reader::TypeDef,
    item: &metadata::reader::Field,
) -> TokenStream {
    let name = write_ident(item.name());

    let ty = match item.ty() {
        metadata::Type::Name(ty_name) => {
            if let Some(_resolved) = item.index().get(namespace, &ty_name.name).next() {
                write_type(
                    namespace,
                    &metadata::Type::named(&ty_name.namespace, &ty_name.name),
                )
            } else if ty_name.namespace.is_empty() {
                let resolved = resolve_nested_type(&ty_name.name, parent, item.index());
                let field_attrs =
                    write_custom_attributes(item.attributes(), namespace, item.index());
                return write_nested_field(namespace, &name, &field_attrs, &resolved);
            } else {
                write_type(namespace, &metadata::Type::Name(ty_name.clone()))
            }
        }
        _ => write_type(namespace, &item.ty()),
    };

    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index());
    quote! { #(#field_attrs)* #name: #ty, }
}

/// Resolves a nested type by name, following `/`-separated path segments through
/// successive nesting levels within `parent`.
fn resolve_nested_type<'a>(
    ty_name: &str,
    parent: &metadata::reader::TypeDef<'a>,
    index: &'a metadata::reader::TypeIndex,
) -> metadata::reader::TypeDef<'a> {
    let mut segments = ty_name.split('/');
    let first = segments.next().unwrap();
    let mut resolved = index
        .nested(*parent)
        .find(|t| t.name() == first)
        .unwrap_or_else(|| panic!("Could not resolve nested type: {}", first));
    for segment in segments {
        resolved = index
            .nested(resolved)
            .find(|t| t.name() == segment)
            .unwrap_or_else(|| panic!("Could not resolve nested type: {}", segment));
    }
    resolved
}

/// Writes a field whose type is a nested (inline) type definition.  For struct/union
/// nested types this emits the braced field list; for enum nested types this emits
/// the `#[repr]` (and optional `#[flags]`) attributes before the field name, followed
/// by the enum keyword and its variant list.
fn write_nested_field(
    namespace: &str,
    name: &TokenStream,
    field_attrs: &[TokenStream],
    resolved: &metadata::reader::TypeDef,
) -> TokenStream {
    if resolved.category() == metadata::reader::TypeCategory::Enum {
        // Derive #[repr] from the `value__` field and #[flags] from the TypeDef attributes.
        let repr_field = resolved.fields().next().unwrap();
        let repr_ty = write_type(namespace, &repr_field.ty());

        let has_flags = resolved.attributes().any(|attr| {
            attr.name() == "FlagsAttribute" && attr.ctor().parent().namespace() == "System"
        });

        let variants: Vec<_> = resolved
            .fields()
            .filter_map(|field| {
                field.constant().map(|constant| {
                    let variant_name = write_ident(field.name());
                    let value = write_value(namespace, &constant.value());
                    quote! { #variant_name = #value, }
                })
            })
            .collect();

        if has_flags {
            quote! {
                #[repr(#repr_ty)]
                #[flags]
                #(#field_attrs)*
                #name: enum {
                    #(#variants)*
                },
            }
        } else {
            quote! {
                #[repr(#repr_ty)]
                #(#field_attrs)*
                #name: enum {
                    #(#variants)*
                },
            }
        }
    } else {
        let keyword = nested_keyword(resolved);
        let fields: Vec<_> = resolved
            .fields()
            .map(|f| write_field(namespace, resolved, &f))
            .collect();
        quote! { #(#field_attrs)* #name: #keyword { #(#fields)* }, }
    }
}

fn is_nested_type(item: &metadata::reader::TypeDef) -> bool {
    item.flags()
        .contains(metadata::TypeAttributes::NestedPublic)
}

fn nested_keyword(item: &metadata::reader::TypeDef) -> TokenStream {
    if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    }
}
