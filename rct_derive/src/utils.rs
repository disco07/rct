fn generate_random_name() -> syn::Ident {
    // Génère un nom aléatoire en utilisant une combinaison de lettres minuscules
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
        // Génère un nom aléatoire pour le GenericParam
        let name = generate_random_name();

        // Vérifie si le nom existe déjà dans les Generics
        let exists = generics.params.iter().any(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                type_param.ident == name
            } else {
                false
            }
        });

        // Si le nom n'existe pas, crée un nouveau GenericParam
        if !exists {
            new_generic_param = Some(syn::parse_quote! { #name });
            ident = name;
            break;
        }
    }

    // Ajoute le nouveau GenericParam aux Generics
    if let Some(param) = new_generic_param {
        generics.params.push(param);
    }

    (generics, ident)

}