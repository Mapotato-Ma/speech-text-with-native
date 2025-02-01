extern crate tts;
extern crate wasm_bindgen;

use std::{thread, time::{Duration, Instant}};
use tts::Tts;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn speek(text: &str, voice_index: usize, rate: f32, _volume: f32, timeout: u64) {
    // 初始化TTS引擎
    let mut tts = Tts::default().expect("初始化TTS引擎失败");

    // 设置语音速度
    tts.set_rate(rate).expect("设置语速失败");

    // 设置音量
    tts.set_volume(_volume).expect("设置音量失败");

    // 设置音色
    if let Some(voice) = tts.voices().expect("获取音色失败").get(voice_index) {
        tts.set_voice(voice).expect("设置音色失败");
    }

    // 开始朗读文本
    tts.speak(text, false).expect("朗读失败");

    // 记录开始时间
    let start_time = Instant::now();

    // 等待直到播放完成或超时
    while tts.is_speaking().expect("检查播放状态失败") {
        thread::sleep(Duration::from_millis(100));

        // 检查是否超时
        if start_time.elapsed() >= Duration::from_secs(timeout) {
            tts.stop().expect("停止播放失败");
            break;
        }
    }
}
