use captrs::*;
use shuteye::sleep;
use std::time::Duration;
use yeelight::{Bulb, Effect, Mode, Power};
use rgb2hex;
use tokio;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let ip: String = env::var("IP_DESK").unwrap();
    let host: &str = "192.168.2.12";
    let interval: Duration = Duration::from_millis(250);             // ms
    let transition_duration: Duration = Duration::from_millis(250);  // ms
    let no_duration: Duration = Duration::from_millis(0);

    let mut lamp: Bulb = Bulb::connect(&*ip, 0).await?;
    lamp.set_power(Power::On, Effect::Sudden, no_duration, Mode::Normal).await;
    let mut music_conn = lamp.start_music(host).await?;

    let mut capturer = Capturer::new(0).unwrap();

    let (w, h) = capturer.geometry();
    let size = w as u64 * h as u64;
    let mut prev_average = 0;

    loop {
        let ps = capturer.capture_frame().unwrap();
        let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);

        for Bgr8 { r, g, b, .. } in ps.into_iter() {
            tot_r += r as u64;
            tot_g += g as u64;
            tot_b += b as u64;
        }

        let average = rgb2hex::rgb2hex(
            (tot_r / size) as u8,
            (tot_g / size) as u8,
            (tot_b / size) as u8
        ).unwrap();

        if average != prev_average {
            let res = music_conn.set_rgb(average, Effect::Smooth, transition_duration).await;

            if res.is_err() {
                println!("{}", res.err().unwrap());
            }
        }

        prev_average = average;
        sleep(interval);
    }

    drop(music_conn);
}