use crate::test::helpers;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use syn::{
    File, GenericParam, Ident, ImplItem, ImplItemFn, Item, ItemImpl, ItemMod, Path, PathSegment,
    Type, TypePath,
};

fn get_optics_list(root: &File) -> Vec<String> {
    // Find the `optics` module at the crate root
    let optics_mod = root
        .items
        .iter()
        .find_map(|item| {
            if let Item::Mod(m) = item {
                if m.ident == "optics" {
                    return Some(m);
                }
            }
            None
        })
        .expect("crate::optics module not found");

    // Extract the optics submodules inside `optics`
    if let Some((_, items)) = &optics_mod.content {
        items
            .iter()
            .filter_map(|item| {
                if let Item::Mod(submod) = item {
                    Some(submod.ident.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        panic!("crate::optics module has no inline content");
    }
}

fn collect_inherent_functions_for_structs(
    root: &File,
    struct_paths: &[String],
) -> HashMap<String, Vec<ImplItemFn>> {
    let mut result: HashMap<String, Vec<ImplItemFn>> = HashMap::new();

    for path in struct_paths {
        // Recursively walk the file/module items to find impl blocks
        fn visit_items(items: &[Item], struct_name: &str, methods: &mut Vec<ImplItemFn>) {
            for item in items {
                match item {
                    Item::Mod(ItemMod {
                        content: Some((_, subitems)),
                        ..
                    }) => {
                        visit_items(subitems, struct_name, methods);
                    }
                    Item::Impl(ItemImpl {
                        trait_: None,
                        self_ty,
                        items: impl_items,
                        ..
                    }) => {
                        if let Type::Path(TypePath {
                            path: Path { segments, .. },
                            ..
                        }) = &**self_ty
                        {
                            if let Some(PathSegment { ident, .. }) = segments.last() {
                                if ident == struct_name {
                                    for impl_item in impl_items {
                                        if let ImplItem::Fn(m) = impl_item {
                                            methods.push(m.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let struct_name = path.split("::").last().expect("empty path");
        let mut methods = Vec::new();

        visit_items(&root.items, struct_name, &mut methods);
        result.insert(path.clone(), methods);
    }

    result
}

fn extract_fn_names_and_type_params(input: &[ImplItemFn]) -> Vec<(Ident, Vec<Ident>)> {
    input
        .iter()
        .map(|f| {
            let type_params = f
                .sig
                .generics
                .params
                .iter()
                .filter_map(|p| {
                    if let GenericParam::Type(ty) = p {
                        Some(ty.ident.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            (f.sig.ident.clone(), type_params)
        })
        .collect::<Vec<_>>()
}

#[test]
fn test_all_optics_have_combine_with_functions() {
    helpers::CRATE_AST.with(|ast| {
        let optics = get_optics_list(ast);

        let fns = collect_inherent_functions_for_structs(
            ast,
            optics
                .iter()
                .map(|o| {
                    let struct_name = o.to_case(Case::UpperCamel);
                    format!("optics::{o}::{struct_name}Impl")
                })
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let mut missing = Vec::<String>::new();

        let fns = fns
            .iter()
            .map(|(k, v)| (k, extract_fn_names_and_type_params(v)))
            .collect::<HashMap<_, _>>();

        for o in &optics {
            if o == "setter" {
                continue;
            }
            let struct_name = o.to_case(Case::UpperCamel);

            let empty = Vec::new();
            let fns = fns
                .get(&format!("optics::{o}::{struct_name}Impl"))
                .unwrap_or(&empty);

            for w in &optics {
                let struct_name = o.to_case(Case::UpperCamel);
                #[allow(clippy::collapsible_if)]
                if let Some(f) = fns.iter().find(|f| f.0 == format!("compose_with_{w}")) {
                    if f.1
                        .iter()
                        .any(|p| p.to_string().len() <= 2 && p.to_string().ends_with('E'))
                    {
                        if !fns
                            .iter()
                            .any(|f| f.0 == format!("compose_with_{w}_with_mappers"))
                        {
                            missing.push(format!(
                                "optics::{o}::{struct_name}Impl::compose_with_{w}_with_mappers"
                            ));
                        }
                    }
                } else {
                    missing.push(format!("optics::{o}::{struct_name}Impl::compose_with_{w}"));
                }
            }
        }

        missing.sort();
        assert!(
            missing.is_empty(),
            "No implementation found for combinators: \n{}",
            missing.join("\n")
        );
    });
}
