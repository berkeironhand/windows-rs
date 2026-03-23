use super::guid;
use super::*;

syn::custom_keyword!(interface);

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub requires: Vec<syn::Path>,
    pub methods: Vec<Method>,
    pub winrt: bool,
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;

        let requires = if input.parse::<syn::Token![:]>().is_ok() {
            let mut requires = vec![input.parse::<syn::Path>()?];
            while input.parse::<syn::Token![+]>().is_ok() {
                requires.push(input.parse::<syn::Path>()?);
            }
            requires
        } else {
            vec![]
        };

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            generics,
            requires,
            methods,
            winrt: false,
        })
    }
}

impl<'a, 'b: 'a> Encoder<'a, 'b> {
    pub(super) fn encode_interface(&mut self, ty: &Interface) -> Result<(), Error> {
        let mut flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::Interface;

        if ty.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let mut generics = Vec::with_capacity(ty.generics.params.len());
        for generic in ty.generics.params.iter() {
            let syn::GenericParam::Type(generic) = generic else {
                return self.err(generic, "only type generic parameters are supported");
            };
            generics.push(generic.ident.to_string());
        }
        self.generics = generics;

        let mut name = self.name.to_string();

        if !self.generics.is_empty() {
            name = format!("{name}`{}", self.generics.len());
        }

        let interface = self.output.TypeDef(
            self.namespace,
            &name,
            metadata::writer::TypeDefOrRef::default(),
            flags,
        );

        for (number, name) in self.generics.iter().enumerate() {
            self.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(interface),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        // Emit any Named attributes (defined in metadata or RDL) attached to this interface.
        // Skip GUID derivation if an explicit GuidAttribute is already present.
        let already_has_guid = ty.attrs.iter().any(|attr| self.is_guid_attribute(attr));

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(interface),
            &ty.attrs,
            &[],
        )?;

        if !ty.winrt && ty.requires.len() > 1 {
            return self.err(
                &ty.requires[1],
                "non-WinRT interface can only inherit from one interface",
            );
        }

        for require in &ty.requires {
            let t = self.encode_path(require)?;
            self.output.InterfaceImpl(interface, &t);
        }

        let base_flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        // Collect method signatures for GUID derivation (for any interface without an explicit
        // GuidAttribute — both WinRT and Win32 interfaces benefit from this).
        let mut method_signatures: Vec<(String, Vec<metadata::Type>, metadata::Type)> = Vec::new();

        for method in &ty.methods {
            let mut params = vec![];

            if method.sig.inputs.is_empty() {
                return self.err(&method.sig.ident, "`&self` parameter not found");
            }

            for (sequence, arg) in method.sig.inputs.iter().enumerate() {
                match arg {
                    syn::FnArg::Receiver(receiver) => {
                        if *receiver != syn::parse_quote! { &self } {
                            return self.err(receiver, "`&self` parameter not found");
                        }
                    }
                    syn::FnArg::Typed(pt) => {
                        if sequence == 0 {
                            // This may seems a little redundant but is consistent with Rust
                            // and leaves room for WinRT classes to model static methods.
                            return self.err(arg, "`&self` parameter not found");
                        }
                        params.push(self.param(pt)?);
                    }
                }
            }

            let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
            let return_type = self.encode_return_type(&method.sig.output)?;

            if !already_has_guid {
                method_signatures.push((
                    method.sig.ident.to_string(),
                    types.clone(),
                    return_type.clone(),
                ));
            }

            let signature = metadata::Signature {
                flags: metadata::MethodCallAttributes::HASTHIS,
                return_type,
                types,
            };

            // Check for the built-in `#[special]` pseudo-attribute which sets the
            // SpecialName bit on the MethodDef, preserving properties and events.
            let mut is_special = false;
            for attr in &method.attrs {
                if attr.path().is_ident("special") {
                    if !matches!(attr.meta, syn::Meta::Path(_)) {
                        return self.err(attr, "`special` attribute does not accept arguments");
                    }
                    is_special = true;
                }
            }

            let mut flags = base_flags;
            if is_special {
                flags |= metadata::MethodAttributes::SpecialName;
            }

            let method_def = self.output.MethodDef(
                &method.sig.ident.to_string(),
                &signature,
                flags,
                Default::default(),
            );

            // Emit any Named attributes attached to this method.
            // `special` is a built-in pseudo-attribute and must not be emitted as metadata.
            self.encode_attrs(
                metadata::writer::HasAttribute::MethodDef(method_def),
                &method.attrs,
                &["special"],
            )?;

            for (sequence, param) in params.iter().enumerate() {
                let param_id = self.output.Param(
                    &param.name,
                    (sequence + 1).try_into().unwrap(),
                    param.attributes,
                );

                self.encode_attrs(
                    metadata::writer::HasAttribute::Param(param_id),
                    &param.attrs,
                    &["input", "output", "optional"],
                )?;
            }
        }

        // For interfaces without an explicit GuidAttribute (both WinRT and Win32), derive the GUID
        // from the interface name and method signatures using the midlrt algorithm (RFC 4122 UUID v5).
        if !already_has_guid {
            let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = method_signatures
                .iter()
                .map(|(name, types, ret)| (name.as_str(), types.as_slice(), ret))
                .collect();

            guid::derive_and_emit_guid(
                self.output,
                metadata::writer::HasAttribute::TypeDef(interface),
                self.namespace,
                self.name,
                &methods,
            );
        }

        Ok(())
    }
}
