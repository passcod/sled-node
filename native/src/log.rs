use neon::js::class::Class;
use neon::js::JsNull;
use neon::vm::{Call, JsResult};
use rsdb::LockFreeLog as RLog;

pub struct Log(pub Option<RLog>);

declare_types! {
    pub class JsLog for Log {
        init(_) {
            Ok(Log(None))
        }
    }
}

pub fn new(call: Call) -> JsResult<JsLog> {
    let mut scope = call.scope;
    let class = JsLog::class(scope)?;
    let ctor = class.constructor(scope)?;
    ctor.construct(scope, vec![JsNull::new()])
}
