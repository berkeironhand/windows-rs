use super::*;

#[derive(Default)]
pub struct Layout {
    modules: BTreeMap<String, Layout>,
    winrt: BTreeMap<String, Vec<TokenStream>>,
    win32: BTreeMap<String, Vec<TokenStream>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            winrt: BTreeMap::new(),
            win32: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, namespace: &str, name: &str, winrt: bool, tokens: TokenStream) {
        if let Some((first, rest)) = namespace.split_once('.') {
            self.modules
                .entry(first.to_string())
                .or_default()
                .insert(rest, name, winrt, tokens)
        } else if winrt {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .winrt
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        } else {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .win32
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        }
    }

    pub fn has_content(&self) -> bool {
        !self.winrt.is_empty()
            || !self.win32.is_empty()
            || self.modules.values().any(|m| m.has_content())
    }

    pub fn to_token_stream(&self) -> TokenStream {
        self.modules
            .iter()
            .map(|(name, module)| module.to_module_token_stream(name))
            .collect()
    }

    fn to_module_token_stream(&self, name: &str) -> TokenStream {
        let name_ident = format_ident!("{}", name);
        let mut output = TokenStream::new();

        if !self.modules.is_empty() {
            let inner: TokenStream = self
                .modules
                .iter()
                .map(|(n, m)| m.to_module_token_stream(n))
                .collect();
            output.extend(quote! { mod #name_ident { #inner } });
        }

        if !self.winrt.is_empty() {
            let items = sorted_items(&self.winrt);
            output.extend(quote! { #[winrt] mod #name_ident { #(#items)* } });
        }

        if !self.win32.is_empty() {
            let items = sorted_items(&self.win32);
            output.extend(quote! { #[win32] mod #name_ident { #(#items)* } });
        }

        output
    }
}

fn sorted_items(map: &BTreeMap<String, Vec<TokenStream>>) -> Vec<TokenStream> {
    // BTreeMap iterates in sorted key order; sort within each Vec for determinism
    // when the same name has multiple definitions (e.g. overloaded functions).
    map.values()
        .flat_map(|v| {
            let mut v = v.clone();
            v.sort_by_cached_key(|ts| ts.to_string());
            v
        })
        .collect()
}
