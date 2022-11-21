use js_sys::Promise;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

fn window() -> Window {
    web_sys::window().expect("must have a window")
}

/// execute the function at a certain specified timeout in ms
fn delay_exec(closure_delay: Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    let timeout_id = window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure_delay.as_ref().unchecked_ref(),
            timeout,
        )
        .expect("should register the setTimeout call");

    closure_delay.forget();

    Some(timeout_id)
}

fn future_delay(timeout: i32) -> JsFuture {
    let promise = Promise::new(&mut |resolve, _reject| {
        delay_exec(
            Closure::once(move || {
                resolve
                    .call0(&JsValue::NULL)
                    .expect("must be able to call resolve");
            }),
            timeout,
        );
    });
    JsFuture::from(promise)
}

/// simulate a delay using promise in js
pub async fn async_delay(timeout: i32) {
    future_delay(timeout).await.expect("must not error");
}

pub fn now() -> i32 {
    let n = window().performance().expect("must have performance").now();
    n as i32
}
