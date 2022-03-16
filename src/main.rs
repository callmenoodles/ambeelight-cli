use std::{time::Duration};
use yeelight::{Bulb, BulbError, Effect, Mode, Power, Properties, Property, Response};
use clap::{Arg, arg, Parser};
use captrs::*;
use shuteye::sleep;
use rgb2hex;
use tokio;

#[derive(Parser, Debug)]
#[clap(author = "Noodles", version, about = "Cross-platform ambilight for Yeelight.", long_about = None)]
struct Args {
    #[clap(long, help = "IP address of the Yeelight", long_help = "Find your Yeelight's IP address in the Yeelight app under <Your Lamp> -> Settings -> Device Info", env = "AMBEELIGHT_YEELIGHT_IP")]
    ip: String,

    #[clap(long, help = "IP address of the host", long_help = "Usually your local network IP address", env = "AMBEELIGHT_HOST_IP")]
    host: String,

    #[clap(short, long, help = "Yeelight brightness between 1-100")]
    brightness: Option<u8>,

    #[clap(short, long, default_value_t = 250, help = "Time between screen reads in milliseconds")]
    interval: u64,

    #[clap(short, long, default_value_t = 250, help = "Transition duration in milliseconds")]
    transition: u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let ip: String = args.ip;
    let host: String = args.host;
    let interval: u64 = args.interval;
    let transition_duration: u64 = args.transition;

    let mut lamp: Bulb = Bulb::connect(&*ip, 0).await?;

    let lamp_properties = Properties(vec![
        Property::Bright
    ]);

    let curr_brightness: Option<Response> = lamp.get_prop(&lamp_properties).await?;
    const DEFAULT_BRIGHTNESS: u8 = 50;

    let brightness: u8 = match args.brightness {
        Some(val) => val,
        None => match curr_brightness {
            Some(val) => match val.first() {
                Some(v) => v.parse().unwrap(),
                _ => DEFAULT_BRIGHTNESS
            },
            _ => DEFAULT_BRIGHTNESS
        }
    };

    let power_res = lamp.set_power(
        Power::On,
        Effect::Sudden,
        Duration::from_millis(0), Mode::Normal
    ).await;

    if power_res.is_err() {
        println!("{}", power_res.err().unwrap());
    }

    let bright_res = lamp.set_bright(
        brightness,
        Effect::Smooth,
        Duration::from_millis(transition_duration),
    ).await;

    if bright_res.is_err() {
        println!("{}", bright_res.err().unwrap());
    }

    let mut music_conn = lamp.start_music(&*host).await?;
    let mut capturer = Capturer::new(0).unwrap();

    let (w, h) = capturer.geometry();
    let size = w as u64 * h as u64;
    let mut prev_average = 0;
    let mut ps_res;

    loop {
        ps_res = capturer.capture_frame();

        if ps_res.is_err() {
            println!("{:?}", ps_res.err().unwrap());
        }
        else {
            let ps = ps_res.unwrap();
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
        }

        sleep(Duration::from_millis(interval));
    }
}