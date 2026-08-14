pub fn derive_godot_node_component(
    input: syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let plan = crate::bevy_attr::parse_component_first(&input)?;
    Ok(crate::emit::emit(&plan, &input))
}

pub fn derive_bevy_components(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let plan = crate::bevy_attr::parse_godot_first(&input)?;
    Ok(crate::emit::emit(&plan, &input))
}

pub fn derive_godot_resource(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut plan = crate::bevy_attr::parse_component_first(&input)?;

    // Default base to `Resource` instead of `Node` if user didn't specify #[gdbevy(base = ...)]
    if plan.base == "Node" {
        plan.base = syn::parse_quote!(Resource);
    }

    // Resources don't enter the SceneTree, so they don't trigger required-component tree registrations
    plan.trigger = None;

    Ok(crate::emit::emit_resource(&plan, &input))
}
