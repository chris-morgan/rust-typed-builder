use syn;

use syn::parse::Error;
use syn::spanned::Spanned;
use quote::ToTokens;

pub fn make_identifier(kind: &str, name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("TypedBuilder_{}_{}", kind, name), proc_macro2::Span::call_site())
}

// Returns error if there is more than one.
pub fn map_only_one<S, T, F>(iter: &[S], dlg: F) -> Result<Option<T>, Error>
where F: Fn(&S) -> Result<Option<T>, Error>,
      S: Spanned,
{
    let mut result = None;
    for item in iter {
        if let Some(answer) = dlg(item)? {
            if result.is_some() {
                return Err(Error::new(item.span(), "Multiple #[builder] on the same field"))
            }
            result = Some(answer);
        }
    }
    Ok(result)
}

pub fn path_to_single_string(path: &syn::Path) -> Option<String> {
    if path.leading_colon.is_some() {
        return None;
    }
    let mut it = path.segments.iter();
    let segment = it.next()?;
    if it.next().is_some() {
        // Multipart path
        return None;
    }
    if segment.arguments != syn::PathArguments::None {
        return None;
    }
    Some(segment.ident.to_string())
}

pub fn expr_to_single_string(expr: &syn::Expr) -> Option<String> {
    if let syn::Expr::Path(path) = &*expr {
        path_to_single_string(&path.path)
    } else {
        None
    }
}

pub fn ident_to_type(ident: syn::Ident) -> syn::Type {
    let mut path = syn::Path {
        leading_colon: None,
        segments: Default::default(),
    };
    path.segments.push(syn::PathSegment {
        ident: ident,
        arguments: Default::default(),
    });
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: path,
    })
}

pub fn empty_type() -> syn::Type {
    syn::TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    }.into()
}

pub fn make_punctuated_single<T, P: Default>(value: T) -> syn::punctuated::Punctuated<T, P> {
    let mut punctuated = syn::punctuated::Punctuated::new();
    punctuated.push(value);
    punctuated
}

pub fn modify_types_generics_hack<F>(ty_generics: &syn::TypeGenerics, mut mutator: F) -> syn::AngleBracketedGenericArguments
where F: FnMut(&mut syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>)
{
    let mut abga: syn::AngleBracketedGenericArguments =
        syn::parse(ty_generics.clone().into_token_stream().into())
        .unwrap_or_else(|_|
                        syn::AngleBracketedGenericArguments{
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: Default::default(),
                            gt_token: Default::default(),
                        });
    mutator(&mut abga.args);
    abga
}

