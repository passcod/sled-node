use neon::js::binary::JsBuffer;
use neon::js::class::Class;
use neon::js::{JsNull, JsString, Value};
use neon::vm::{Call, JsResult, Lock};
use rsdb::Tree as RTree;

pub struct Tree(pub Option<RTree>);

declare_types! {
    pub class JsTree for Tree {
        init(_) {
            Ok(Tree(None))
        }

        method get(call) {
            let scope = call.scope;
            let args = call.arguments;
            let mut keybuf = args.require(scope, 0)?.check::<JsBuffer>()?;
            let key = keybuf.grab(|b| b.as_slice());
            let bytes = args.this(scope).grab(|wrap| {
                wrap.0.as_ref().and_then(|t| t.get(key))
            });

            match bytes {
                None => Ok(JsNull::new().as_value(scope)),
                Some(bytes) => {
                    let mut buf = JsBuffer::new(scope, bytes.len() as u32)?;
                    buf.grab(|mut b| b.as_mut_slice().copy_from_slice(bytes.as_ref()));
                    Ok(buf.upcast())
                }
            }
        }

        method set(call) {
            let scope = call.scope;
            let args = call.arguments;
            let mut keybuf = args.require(scope, 0)?.check::<JsBuffer>()?;
            let mut valbuf = args.require(scope, 1)?.check::<JsBuffer>()?;

            let key: Vec<u8> = keybuf.grab(|b| b.as_slice().into());
            let val: Vec<u8> = valbuf.grab(|b| b.as_slice().into());

            args.this(scope).grab(|wrap| {
                wrap.0.as_ref().unwrap().set(key, val);
            });

            Ok(JsNull::new().as_value(scope))
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
