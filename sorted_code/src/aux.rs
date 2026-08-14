use syn::{
    parse::{Parse, ParseStream},
    {Item, Result},
};

pub struct Items {
    pub items: Vec<Item>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Items { items })
    }
}
