extern crate tts;
extern crate wasm_bindgen;

use js_sys::Promise;
use std::time::{Duration, Instant};
use tts::Tts;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::window;

#[wasm_bindgen]
pub fn speek_async(
    text: String,
    voice_index: usize,
    rate: f32,
    volume: f32,
    timeout: u64,
) -> Promise {
    future_to_promise(async move {
        // 创建和存储动画帧回调
        let window = window().ok_or_else(|| JsValue::from_str("无法获取window对象"))?;

        struct TtsWrapper {
            tts: Tts,
        }

        impl TtsWrapper {
            fn new() -> Result<Self, String> {
                Ok(Self {
                    tts: Tts::default().map_err(|e| e.to_string())?,
                })
            }
        }

        impl Drop for TtsWrapper {
            fn drop(&mut self) {
                let _ = self.tts.stop();
            }
        }

        let result = async move {
            let mut wrapper = TtsWrapper::new()
                .map_err(|e| JsValue::from_str(&format!("初始化TTS失败: {}", e)))?;

            wrapper
                .tts
                .set_rate(rate)
                .map_err(|e| JsValue::from_str(&format!("设置语速失败: {}", e)))?;

            wrapper
                .tts
                .set_volume(volume)
                .map_err(|e| JsValue::from_str(&format!("设置音量失败: {}", e)))?;

            let voices = wrapper
                .tts
                .voices()
                .map_err(|e| JsValue::from_str(&format!("获取音色失败: {}", e)))?;

            if let Some(voice) = voices.get(voice_index) {
                wrapper
                    .tts
                    .set_voice(voice)
                    .map_err(|e| JsValue::from_str(&format!("设置音色失败: {}", e)))?;
            }

            wrapper
                .tts
                .speak(&text, false)
                .map_err(|e| JsValue::from_str(&format!("朗读失败: {}", e)))?;

            let start = Instant::now();

            let closure = Closure::wrap(Box::new(move || {
                // 空函数
            }) as Box<dyn FnMut()>);

            while wrapper
                .tts
                .is_speaking()
                .map_err(|e| JsValue::from_str(&format!("检查播放状态失败: {}", e)))?
            {
                if start.elapsed() >= Duration::from_secs(timeout) {
                    return Err(JsValue::from_str("播放超时"));
                }

                window.request_animation_frame(closure.as_ref().unchecked_ref())?;

                // 短暂等待避免CPU占用过高
                std::thread::sleep(Duration::from_millis(10));
            }

            closure.forget();
            Ok(JsValue::from_str("播放完成"))
        }
        .await;

        result
    })
}
