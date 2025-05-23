use crate::test::helpers;
use convert_case::{Case, Casing};
use syn::visit::Visit;
use syn::{ItemFn, ItemMod, ItemStruct, Visibility, visit};

#[test]
fn optic_implementations_exported_struct_and_fns() {
    #[derive(Default)]
    struct Check {
        current_module: Vec<String>,
    }

    impl<'ast> Visit<'ast> for Check {
        fn visit_item_mod(&mut self, i: &'ast ItemMod) {
            self.current_module.push(i.ident.to_string());
            visit::visit_item_mod(self, i);
            self.current_module.pop();
        }

        fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
            if let Visibility::Public(_) = i.vis {
                match self
                    .current_module
                    .iter()
                    .map(std::string::String::as_str)
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    [.., "mapped" | "composed"] => {
                        panic!(
                            "Found public struct in module {}::{}",
                            self.current_module.join("::"),
                            i.ident
                        );
                    }
                    [.., "wrapped"] => {
                        let optic_type = self
                            .current_module
                            .get(self.current_module.len() - 2)
                            .unwrap()
                            .as_str()
                            .to_case(Case::UpperCamel);

                        let expected_exported_type = format!("{optic_type}Impl");

                        assert!(
                            i.ident == expected_exported_type,
                            "Found public struct in module {}::{} that is not a {}Impl",
                            self.current_module.join("::"),
                            i.ident,
                            optic_type
                        );
                    }
                    _ => (),
                }
            }
        }

        fn visit_item_fn(&mut self, i: &'ast ItemFn) {
            if let Visibility::Public(_) = i.vis {
                match self
                    .current_module
                    .iter()
                    .map(std::string::String::as_str)
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    [.., "mapped" | "composed"] => {
                        assert!(
                            i.sig.ident == "new",
                            "Found public fn in module {}::{} that is not new()",
                            self.current_module.join("::"),
                            i.sig.ident
                        );
                    }

                    [optic_type] => {
                        let expected_exported_type = format!("identity_{optic_type}");

                        assert!(
                            i.sig.ident == expected_exported_type,
                            "Found public fn in module {}::{} that is not {}",
                            self.current_module.join("::"),
                            i.sig.ident,
                            expected_exported_type
                        );
                    }

                    _ => (),
                }
            }
        }
    }

    helpers::CRATE_AST.with(|syn| Check::default().visit_file(syn));
}
