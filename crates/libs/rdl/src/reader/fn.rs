use super::*;

#[derive(Debug)]
pub struct Fn {
    pub attrs: Vec<syn::Attribute>,
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Fn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![extern]>()?;
        let abi = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, abi, sig })
    }
}

impl<'a, 'b: 'a> Encoder<'a, 'b> {
    pub(super) fn encode_fn(&mut self, ty: &Fn) -> Result<(), Error> {
        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Static
            | metadata::MethodAttributes::PInvokeImpl;

        let params = self.collect_params(&ty.sig)?;

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let mut call_flags = metadata::MethodCallAttributes::default();

        if ty.sig.variadic.is_some() {
            call_flags |= metadata::MethodCallAttributes::VARARG;
        }

        let signature = metadata::Signature {
            flags: call_flags,
            return_type: self.encode_return_type(&ty.sig.output)?,
            types,
        };

        let name = ty.sig.ident.to_string();

        let method_def = self
            .output
            .MethodDef(&name, &signature, flags, Default::default());

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

        let Some(attribute) = ty
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("library"))
        else {
            return self.err(&ty.sig, "`library` attribute not found");
        };

        let (library, last_error) = attribute
            .parse_args_with(
                |input: syn::parse::ParseStream| -> syn::Result<(syn::LitStr, bool)> {
                    let library: syn::LitStr = input.parse()?;
                    let mut last_error = false;
                    if input.peek(syn::Token![,]) {
                        input.parse::<syn::Token![,]>()?;
                        let ident: syn::Ident = input.parse()?;
                        if ident != "last_error" {
                            return Err(syn::Error::new(ident.span(), "unknown library option"));
                        }
                        input.parse::<syn::Token![=]>()?;
                        let value: syn::LitBool = input.parse()?;
                        last_error = value.value();
                    }
                    Ok((library, last_error))
                },
            )
            .or_else(|_| self.err(attribute.span(), "`library` name missing"))?;

        let mut flags = metadata::PInvokeAttributes::NoMangle;

        if last_error {
            flags |= metadata::PInvokeAttributes::SupportsLastError;
        }

        if let Some(abi) = &ty.abi {
            match abi.value().as_str() {
                "system" => flags |= metadata::PInvokeAttributes::CallConvPlatformapi,
                "C" => flags |= metadata::PInvokeAttributes::CallConvCdecl,
                "fastcall" => flags |= metadata::PInvokeAttributes::CallConvFastcall,
                _ => return self.err(abi, "function abi not supported"),
            }
        } else {
            flags |= metadata::PInvokeAttributes::CallConvPlatformapi;
        }

        self.output
            .ImplMap(method_def, flags, &name, library.value().as_str());

        // Emit any Named attributes (defined in metadata or RDL) attached to this function.
        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(method_def),
            &ty.attrs,
            &["library"],
        )?;

        Ok(())
    }
}
