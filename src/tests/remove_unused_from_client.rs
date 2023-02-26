#[allow(unused_imports)]
use super::syntax;
use super::tr;
use crate::loaders::remove_unused_from_client::RemoveUnusedVisitor;
use swc_core::ecma::transforms::testing::test;

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_import,
    r#"import fs from 'fs'; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_when_member_conflict,
    r#"import fs from 'fs'; class Component { fs() {} };"#,
    r#"class Component { fs() {} };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_when_this_conflict,
    r#"import fs from 'fs'; class Component { prepare() { this.fs = 69 } };"#,
    r#"class Component { prepare() { this.fs = 69 } };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_when_redeclared,
    r#"import fs from 'fs'; class Component { prepare() { const fs = 0 } };"#,
    r#"class Component { prepare() { const fs = 0 } };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_when_redeclared_and_reused,
    r#"import fs from 'fs'; class Component { prepare() { let fs = 0; fs = 1; } };"#,
    r#"class Component { prepare() { let fs = 0; fs = 1; } };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    remove_unused_when_used_as_key,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_used_when_jsx_conflict,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_used_imports,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_css_imports,
    r#"import './styles.css'; class Component { };"#,
    r#"import './styles.css'; class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_scss_imports,
    r#"import './styles.scss'; class Component { };"#,
    r#"import './styles.scss'; class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_sass_imports,
    r#"import './styles.sass'; class Component { };"#,
    r#"import './styles.sass'; class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_imports_without_identifiers,
    r#"import 'env/setup'; class Component { };"#,
    r#"import 'env/setup'; class Component { };"#
);

test!(
    syntax(),
    |_| tr(RemoveUnusedVisitor::default()),
    keep_runtime_import,
    r#"import $runtime from 'nullstack/runtime'; class Component { };"#,
    r#"import $runtime from 'nullstack/runtime'; class Component { };"#
);
