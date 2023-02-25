#[allow(unused_imports)]
use super::syntax;
use crate::loaders::replace_ref_on_attributes::ReplaceRefVisitor;
use swc_core::ecma::transforms::testing::test;
use swc_core::ecma::visit::{as_folder, Fold};

#[allow(dead_code)]
fn tr() -> impl Fold {
    as_folder(ReplaceRefVisitor::default())
}

test!(
    syntax(),
    |_| tr(),
    replace_simple_ref,
    r#"class Component {
        render() {
            return <div ref={this.element} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div ref={{object: this, property: "element"}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_ref_array_with_literal_index,
    r#"class Component {
        render() {
            return <div ref={this.elements[1]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div ref={{object: this.elements, property: 1}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_ref_array_with_variable_index,
    r#"class Component {
        render() {
            return <div ref={this.elements[this.index]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div ref={{object: this.elements, property: this.index}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_ref_array_with_private_index,
    r#"class Component {
        render() {
            return <div ref={this.elements[this.#index]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div ref={{object: this.elements, property: this.#index}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_simple_bind,
    r#"class Component {
        render() {
            return <div bind={this.element} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div bind={{object: this, property: "element"}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_bind_array_with_literal_index,
    r#"class Component {
        render() {
            return <div bind={this.elements[1]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div bind={{object: this.elements, property: 1}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_bind_array_with_variable_index,
    r#"class Component {
        render() {
            return <div bind={this.elements[this.index]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div bind={{object: this.elements, property: this.index}} />
        }
    };"#
);

test!(
    syntax(),
    |_| tr(),
    replace_bind_array_with_private_index,
    r#"class Component {
        render() {
            return <div bind={this.elements[this.#index]} />
        }
    };"#,
    r#"class Component {
        render() {
            return <div bind={{object: this.elements, property: this.#index}} />
        }
    };"#
);
