use syn::{
    Error, Ident, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod kw {
    syn::custom_keyword!(python);
    syn::custom_keyword!(abs);
    syn::custom_keyword!(ext);
}

pub enum OptionType {
    Python(Ident),
    Abs(Ident),
    Ext(Ident, Ident),
}

impl Parse for OptionType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::python) {
            let ident = input.parse::<Ident>()?;
            Ok(Self::Python(ident))
        } else if input.peek(kw::abs) {
            let ident = input.parse::<Ident>()?;
            Ok(Self::Abs(ident))
        } else if input.peek(kw::ext) {
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            let ext = input.parse::<Ident>()?;
            Ok(Self::Ext(ident, ext))
        } else {
            Err(input.error("无效属性"))
        }
    }
}

#[derive(Default)]
pub struct Options {
    pub python: Option<Ident>,
    pub abs: Option<Ident>,
    pub ext: Option<(Ident, Ident)>,
}

impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut options = Self::default();
        for option in Punctuated::<OptionType, Token![,]>::parse_terminated(input)? {
            match option {
                OptionType::Python(ident) => {
                    if options.python.is_some() {
                        return Err(Error::new_spanned(ident, "重复属性: python"));
                    }
                    options.python = Some(ident);
                }
                OptionType::Abs(ident) => {
                    if options.abs.is_some() {
                        return Err(Error::new_spanned(ident, "重复属性: abs"));
                    }
                    options.abs = Some(ident);
                }
                OptionType::Ext(ident, ext) => {
                    if options.ext.is_some() {
                        return Err(Error::new_spanned(ident, "重复属性: ext"));
                    }
                    options.ext = Some((ident, ext));
                }
            }
        }
        Ok(options)
    }
}
