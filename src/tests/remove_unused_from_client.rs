use crate::loaders::remove_unused_from_client::RemoveUnusedVisitor;

#[allow(unused_imports)]
use super::syntax;

use swc_common::{chain, Mark};
use swc_core::ecma::{
    transforms::{base::resolver, testing::test},
    visit::{as_folder, Fold},
};

#[allow(dead_code)]
fn tr() -> impl Fold {
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        as_folder(RemoveUnusedVisitor::default())
    )
}

test!(
    syntax(),
    |_| tr(),
    remove_unused_import,
    r#"import fs from 'fs'; class Component { };"#,
    r#"class Component { };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_member_conflict,
    r#"import fs from 'fs'; class Component { fs() {} };"#,
    r#"class Component { fs() {} };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_this_conflict,
    r#"import fs from 'fs'; class Component { prepare() { this.fs = 69 } };"#,
    r#"class Component { prepare() { this.fs = 69 } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_redeclared,
    r#"import fs from 'fs'; class Component { prepare() { const fs = 0 } };"#,
    r#"class Component { prepare() { const fs = 0 } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_redeclared_and_reused,
    r#"import fs from 'fs'; class Component { prepare() { let fs = 0; fs = 1; } };"#,
    r#"class Component { prepare() { let fs = 0; fs = 1; } };"#
);

test!(
    syntax(),
    |_| tr(),
    remove_unused_when_used_as_key,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#,
    r#"import key from 'key'; class Component { prepare() { const obj = {}; return obj[key] } };"#
);

test!(
    syntax(),
    |_| tr(),
    keep_used_when_jsx_conflict,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#,
    r#"import Tag from 'tag'; class Component { render() { return <Tag />} };"#
);

test!(
    syntax(),
    |_| tr(),
    keep_used_imports,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#,
    r#"import fs from 'fs'; class Component { fs3() { this.fs2 = true; fs4.readFileSync(); const fs5 = 0; fs.existsSync("yourmom.txt") } };"#
);
