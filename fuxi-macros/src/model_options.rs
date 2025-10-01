use syn::{
    Error, Ident, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod kw {
    syn::custom_keyword!(python);
}

pub enum OptionType {
    Python(Ident),
}

impl Parse for OptionType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(kw::python) {
            let ident = input.parse::<Ident>()?;
            Ok(Self::Python(ident))
        } else {
            Err(input.error("无效属性"))
        }
    }
}

#[derive(Default)]
pub struct Options {
    pub python: bool,
}

impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut options = Self::default();
        for option in Punctuated::<OptionType, Token![,]>::parse_terminated(input)? {
            match option {
                OptionType::Python(ident) => {
                    if options.python {
                        return Err(Error::new_spanned(ident, "重复属性: python"));
                    }
                    options.python = true;
                }
            }
        }
        Ok(options)
    }
}
