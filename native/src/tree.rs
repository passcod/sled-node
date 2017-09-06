use neon::js::class::Class;
use neon::js::{JsArray, JsInteger, JsNull, JsString, Object, Value};
use neon::vm::{Call, JsResult, Lock};
use rsdb::Tree as RTree;
use std::ops::DerefMut;

pub struct Tree(pub Option<RTree>);

declare_types! {
    pub class JsTree for Tree {
        init(_) {
            Ok(Tree(None))
        }

        method get(call) {
            let scope = call.scope;
            let args = call.arguments;
            let key = args.require(scope, 0)?.check::<JsString>()?.value();
            let bytes = args.this(scope)
                .grab(|wrap| wrap.0.as_ref()
                      .and_then(|t| t.get(&key.into_bytes())));

            match bytes {
                None => Ok(JsNull::new().as_value(scope)),
                Some(bytes) => {
                    let mut array = JsArray::new(scope, bytes.len() as u32);

                    {
                        let mut i = 0u32;
                        let raw_array = array.deref_mut();
                        for byte in bytes {
                            raw_array.set(i, JsInteger::new(scope, byte as i32))?;
                            i += 1;
                        }
                    }

                    Ok(array.upcast())
                }
            }
        }

        method set(call) {
            let scope = call.scope;
            let args = call.arguments;
            let key = args.require(scope, 0)?.check::<JsString>()?.value();
            let ints = args.require(scope, 1)?.check::<JsArray>()?.to_vec(scope)?;

            let key_bytes = key.into_bytes();
            let bytes: Vec<u8> = ints
                .into_iter()
                .flat_map(|i| i.downcast::<JsInteger>())
                .map(|i| i.value() as u8)
                .collect();

            args.this(scope).grab(|wrap| {
                wrap.0.as_ref().unwrap().set(key_bytes, bytes);
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
