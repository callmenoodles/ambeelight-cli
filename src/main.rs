use std::{env, time::Duration};
use yeelight::{Bulb, Effect, Mode, Power};
use captrs::*;
use shuteye::sleep;
use rgb2hex;
use tokio;
use dotenv::dotenv;

// TODO: drop music conn

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    dotenv().ok();

    let ip: String = env::var("IP_DESK").unwrap();
    let host: String = env::var("IP_HOST").unwrap();

    // both in ms
    let interval: u64 = 250;
    let transition_duration: u64 = 250;

    let mut lamp: Bulb = Bulb::connect(&*ip, 0).await?;

    let power_res = lamp.set_power(
        Power::On,
        Effect::Sudden,
        Duration::from_millis(0), Mode::Normal
    ).await;

    if power_res.is_err() {
        println!("{}", power_res.err().unwrap());
    }

    let mut music_conn = lamp.start_music(&*host).await?;

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
            let res = music_conn.set_rgb(
                average,
                Effect::Smooth,
                Duration::from_millis(transition_duration)
            ).await;

            if res.is_err() {
                println!("{}", res.err().unwrap());
            }
        }

        prev_average = average;
        sleep(Duration::from_millis(interval));
    }
}