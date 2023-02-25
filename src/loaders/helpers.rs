use swc_common::{BytePos, Span, SyntaxContext};
use swc_core::ecma::ast::Ident;

pub fn transpiler_ident() -> Ident {
    Ident {
        span: Span {
            lo: BytePos::default(),
            hi: BytePos::default(),
            ctxt: SyntaxContext::from_u32(69),
        },
        sym: "$transpiler".into(),
        optional: false,
    }
}
