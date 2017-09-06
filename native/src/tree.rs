use neon::js::class::Class;
use neon::js::{JsNull, JsString};
use neon::vm::{Call, JsResult, Lock};
use rsdb::Tree as RTree;

pub struct Tree(pub Option<RTree>);

declare_types! {
    pub class JsTree for Tree {
        init(_) {
            Ok(Tree(None))
        }

        method toString(call) {
            let scope = call.scope;
            let string = call.arguments.this(scope)
                .grab(|wrap| format!("{:?}", wrap.0));
            Ok(JsString::new_or_throw(scope, &string)?.upcast())
        }
    }
}

pub fn new(call: Call) -> JsResult<JsTree> {
    let mut scope = call.scope;
    let class = JsTree::class(scope)?;
    let ctor = class.constructor(scope)?;
    ctor.construct(scope, vec![JsNull::new()])
}
