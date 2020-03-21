pub trait JsCtx {
    fn global() -> JsObjectRef;
    fn call(this: Option<JsObjectRef>, method: String) -> JsObjectRef;
    fn drop(js_obj: JsObjectRef);
}

pub struct JsObjectRef {
    remote_ptr: u32
}
