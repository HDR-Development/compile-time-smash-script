use std::str::FromStr;

use proc_macro::TokenStream;
use unsynn::*;

#[derive(Debug)]
struct PublicInner {
    is_public: bool,
    span: Span,
}

unsynn! {
    struct Public(PublicInner) from Optional<Ident>:
    parse_with |value, tokens| {
        match value.get(0) {
            Some(value) if value.value == "pub" => Ok(Self(PublicInner { is_public: true, span: value.value.span()})),
            Some(value) => {
                Error::unexpected_token(Some(value.value.clone().into()), tokens)
            },
            None => Ok(Self(PublicInner { is_public: false, span: Span::call_site()}))
        }
    }
    to_tokens |s, tokens| {
        if s.0.is_public {
            Ident::new("pub", s.0.span.clone()).to_tokens(tokens)
        }
    };
}

unsynn! {
    struct TemplateField {
        field_name: Ident,
        _colon: Colon,
        expr: Either<LiteralString, Literal, TokenStreamUntil<Comma>>
    }

    struct TemplateStruct {
        visibility: Public,
        template_name: Ident,
        _assigns: Assign,
        group: BraceGroupContaining<CommaDelimitedVec<TemplateField>>
    }

    struct TemplateInput {
        delimited: SemicolonDelimitedVec<TemplateStruct>
    }

    struct HitboxShortcutInput {
        agent: TokenStreamUntil<Comma>,
        _comma: Comma,
        group: BraceGroupContaining<CommaDelimitedVec<TemplateField>>,
    }
}

fn make_hitbox_template(fields: CommaDelimitedVec<TemplateField>) -> proc_macro2::TokenStream {
    let mut group_stream = proc_macro2::TokenStream::new();

    let mut extends = None;
    for field in fields {
        if field.value.field_name == "extends" {
            extends = Some(field.value.expr);
            continue;
        }

        let name = &field.value.field_name;
        match field.value.expr {
            Either::First(lit_string) => {
                let literal = lit_string.as_str();
                quote::quote! {
                    #name: Some(smash::phx::Hash40 { hash: smash::hash40(#literal) }),
                }
                .to_tokens(&mut group_stream);
            }
            Either::Second(literal) => quote::quote! {
                #name: Some(#literal as _),
            }
            .to_tokens(&mut group_stream),
            Either::Third(tree) => {
                let tokens = tree.to_token_stream();
                quote::quote! {
                    #name: Some(#tokens),
                }
                .to_tokens(&mut group_stream);
            }
            Either::Fourth(_) => unreachable!(),
        }
    }

    if let Some(extends) = extends {
        let extends = extends.to_token_stream();
        quote::quote! {
            ..#extends
        }
        .to_tokens(&mut group_stream);
    } else {
        quote::quote! {
            ..crate::HitboxTemplate::new()
        }
        .to_tokens(&mut group_stream);
    }

    quote::quote! {
        crate::HitboxTemplate {
            #group_stream
        }
    }
}

#[proc_macro]
pub fn hitbox_templates(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let values = match TemplateInput::parse_all(&mut input.to_token_iter()) {
        Ok(values) => values,
        Err(e) => {
            let tokens = format!("::std::compile_error!(\"{e}\");");
            let stream = proc_macro2::TokenStream::from_str(&tokens).unwrap();

            let span = match e.failed_at() {
                Some(token) => token.span(),
                None => Span::call_site(),
            };

            let mut out_stream = proc_macro2::TokenStream::new();
            out_stream.extend(stream.into_iter().map(|mut token| {
                token.set_span(span.clone());
                token
            }));

            return out_stream.into();
        }
    };

    let mut output_stream = proc_macro2::TokenStream::new();

    for template in values.delimited {
        let content = template.value.group.content;
        let generated = make_hitbox_template(content);

        let name = &template.value.template_name;

        template.value.visibility.to_tokens(&mut output_stream);
        quote::quote! {
            const #name: crate::HitboxTemplate = #generated;
        }
        .to_tokens(&mut output_stream);
    }

    output_stream.into()
}

#[proc_macro]
pub fn decl_hitbox(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let content = <CommaDelimitedVec<TemplateField>>::parse(&mut input.into_token_iter());

    let content = match content {
        Ok(content) => content,
        Err(e) => {
            let tokens = format!("::std::compile_error!(\"{e}\");");
            let stream = proc_macro2::TokenStream::from_str(&tokens).unwrap();

            let span = match e.failed_at() {
                Some(token) => token.span(),
                None => Span::call_site(),
            };

            let mut out_stream = proc_macro2::TokenStream::new();
            out_stream.extend(stream.into_iter().map(|mut token| {
                token.set_span(span.clone());
                token
            }));

            return out_stream.into();
        }
    };

    let template = make_hitbox_template(content);

    quote::quote! {
        crate::HitboxData::from_template_or_panic(#template)
    }
    .into()
}

#[proc_macro]
pub fn hitbox(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let content = HitboxShortcutInput::parse(&mut input.into_token_iter());

    let content = match content {
        Ok(content) => content,
        Err(e) => {
            let tokens = format!("::std::compile_error!(\"{e}\");");
            let stream = proc_macro2::TokenStream::from_str(&tokens).unwrap();

            let span = match e.failed_at() {
                Some(token) => token.span(),
                None => Span::call_site(),
            };

            let mut out_stream = proc_macro2::TokenStream::new();
            out_stream.extend(stream.into_iter().map(|mut token| {
                token.set_span(span.clone());
                token
            }));

            return out_stream.into();
        }
    };

    let agent = content.agent.to_token_stream();
    let template = make_hitbox_template(content.group.content);

    quote::quote! {
        crate::create_hitbox(#agent, &crate::HitboxData::from_template_or_panic(#template))
    }
    .into()
}
