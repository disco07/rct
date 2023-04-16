fn generate_random_name() -> syn::Ident {
    // Generate a random name using a combination of lowercase letters
    let mut name = String::new();
    let letters = "abcdefghijklmnopqrstuvwxyz".to_uppercase();
    for _ in 0..8 {
        let random_index = (letters.len() as f32 * rand::random::<f32>()) as usize;
        name.push(letters.chars().nth(random_index).unwrap());
    }
    syn::Ident::new(&name, proc_macro2::Span::call_site())
}

pub fn new_generic(mut generics: syn::Generics) -> (syn::Generics, syn::Ident) {
    #[allow(unused)]
    let mut new_generic_param = None;
    let ident: syn::Ident;
    loop {
        // Generate a random name for GenericParam
        let name = generate_random_name();

        // Check if the name already exists in Generics
        let exists = generics.params.iter().any(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                type_param.ident == name
            } else {
                false
            }
        });

        // If name is not exists, create new GenericParam
        if !exists {
            new_generic_param = Some(syn::parse_quote! { #name });
            ident = name;
            break;
        }
    }

    // Add the new GenericParam in Generics
    if let Some(param) = new_generic_param {
        generics.params.push(param);
    }

    (generics, ident)
}
