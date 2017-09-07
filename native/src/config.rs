use neon::js::class::Class;
use neon::js::{JsBoolean, JsFunction, JsInteger, JsNull, JsObject, JsString, Object};
use neon::vm::{Call, JsResult, Lock};
use sled::Config as SledConfig;
use super::log::{self, JsLog};
use super::tree::{self, JsTree};

pub struct Config(SledConfig);

declare_types! {
    pub class JsConfig for Config {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;
            let opts = args.require(scope, 0)?.check::<JsObject>()?;

            let mut config = SledConfig::default();

            let keys = opts.get_own_property_names(scope)?.to_vec(scope)?;
            for jskey in keys {
                let key = jskey.check::<JsString>()?.value();
                match key.as_str() {
                    k @ "ioBufs" => { config.set_io_bufs(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "ioBufSize" => { config.set_io_buf_size(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "bLinkFanout" => { config.set_blink_fanout(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "pageConsolidationThreshold" => { config.set_page_consolidation_threshold(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "path" => { config.set_path(
                        opts.get(scope,k)?.check::<JsString>()?.value()
                    ); },
                    k @ "cacheBits" => { config.set_cache_bits(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "cacheCapacity" => { config.set_cache_capacity(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "useOSCache" => { config.set_use_os_cache(
                        opts.get(scope,k)?.check::<JsBoolean>()?.value()
                    ); },
                    k @ "useCompression" => { config.set_use_compression(
                        opts.get(scope,k)?.check::<JsBoolean>()?.value()
                    ); },
                    k @ "flushEveryMs" => { config.set_flush_every_ms(
                        opts.get(scope,k)?.downcast::<JsInteger>()
                            .and_then(|h| Some(h.value() as u64))
                    ); },
                    k @ "snapshotAfterOps" => { config.set_snapshot_after_ops(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    k @ "snapshotPath" => { config.set_snapshot_path(
                        opts.get(scope,k)?.downcast::<JsString>()
                            .and_then(|h| Some(h.value()))
                    ); },
                    k @ "cacheFixupThreshold" => { config.set_cache_fixup_threshold(
                        opts.get(scope,k)?.check::<JsInteger>()?.value() as usize
                    ); },
                    _ => {}
                }
            }

            Ok(Config(config))
        }

        method log(call) {
            let scope = call.scope;

            let data = call.arguments.this(scope)
                .grab(|config| config.0.log());

            let farg = vec![JsNull::new()];
            let mut log = JsFunction::new(scope, log::new)?
                .call(scope, JsNull::new(), farg)?
                .check::<JsLog>()?;

            log.grab(|log| log.0 = Some(data));
            Ok(log.upcast())
        }

        method tree(call) {
            let scope = call.scope;

            let data = call.arguments.this(scope)
                .grab(|config| config.0.tree());

            let farg = vec![JsNull::new()];
            let mut tree = JsFunction::new(scope, tree::new)?
                .call(scope, JsNull::new(), farg)?
                .check::<JsTree>()?;

            tree.grab(|tree| tree.0 = Some(data));
            Ok(tree.upcast())
        }

        method toString(call) {
            let scope = call.scope;
            let string = call.arguments.this(scope)
                .grab(|wrap| format!("{:?}", wrap.0));
            Ok(JsString::new_or_throw(scope, &string)?.upcast())
        }
    }
}

pub fn new(call: Call) -> JsResult<JsConfig> {
    let mut scope = call.scope;
    let class = JsConfig::class(scope)?;
    let arg0 = call.arguments.require(scope, 0)?;
    let ctor = class.constructor(scope)?;
    ctor.construct(scope, vec![arg0])
}
