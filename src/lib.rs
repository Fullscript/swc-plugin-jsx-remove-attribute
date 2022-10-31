use serde::Deserialize;
use swc_core::ecma::{
    ast::{JSXAttrName, JSXAttrOrSpread, JSXOpeningElement, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[cfg(test)]
use swc_ecma_parser::{Syntax, TsConfig};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Options {
    attributes_to_remove: Vec<String>,
}

pub struct TransformVisitor {
    config: Options,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        let mut attrs_to_remove_iter = self.config.attributes_to_remove.iter();

        n.attrs.retain(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(a) => match &a.name {
                JSXAttrName::Ident(i) => bool::from(
                    attrs_to_remove_iter.all(|attr_to_remove| attr_to_remove != &i.sym as &str),
                ),
                _ => true,
            },
            _ => true,
        })
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Options>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for emotion"),
    )
    .expect("invalid config for emotion");
    program.fold_with(&mut as_folder(TransformVisitor { config }))
}

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor {
        config: Options {
            attributes_to_remove: vec!["data-testid".to_string()]
        }
    }),
    removes_data_test_id,
    // Input codes
    r#"<div data-testid='testing'>Hello world</div>"#,
    // Output codes after transformed with plugin
    r#"<div>Hello world</div>"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor {
        config: Options {
            attributes_to_remove: vec![]
        }
    }),
    ignores_attributes_when_not_specified,
    // Input codes
    r#"<div data-testid='testing'>Hello world</div>"#,
    // Output codes after transformed with plugin
    r#"<div data-testid='testing'>Hello world</div>"#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |_| as_folder(TransformVisitor {
        config: Options {
            attributes_to_remove: vec!["data-testid".to_string()]
        }
    }),
    ignores_spread_operator,
    // Input codes
    r#"<div {...rest}>Hello world</div>"#,
    // Output codes after transformed with plugin
    r#"<div {...rest}>Hello world</div>"#
);
