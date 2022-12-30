use std::{fs::File, io::{BufReader, SeekFrom, Seek, Read}, time::{Duration, SystemTime}, thread};

use ev3dev_lang_rust::{sound, Ev3Result, Screen};
use image::Rgb;

pub struct Player {
    height: usize,
    width: usize,
    framerate: u64,
    file: File
}

impl Player {
    pub fn new(file: File) -> Self {
        Player {
            height: 64,
            width: 96,
            framerate: 30u64,
            file
        }
    }

    pub fn play_audio(&self) {
        _ = sound::play("./badapple.wav");
    }

    pub fn play(&self) -> Ev3Result<()> {
        let mut screen = Screen::new()?;

        let bpf: usize = self.width * self.height;
        let mut reader = BufReader::new(&self.file);
        let mut buffer: Vec<u8> = vec![0; bpf];
        let frame_time = Duration::from_nanos(1000000000 / self.framerate);
        let total_frames =
            usize::try_from(self.file.metadata().unwrap().len() / u64::try_from(bpf).unwrap())
                .unwrap();

        for i in 0..total_frames {
            let start_t = SystemTime::now();
            reader.seek(SeekFrom::Start(
                ((self.width * self.height * i) as u64)
                    .try_into()
                    .expect("msg"),
            ))?;
            reader.read_exact(&mut buffer)?;

            for x in 0..self.width{
                for y in 0..self.height{
                    let p = buffer[self.width*y+x];
                    if p == 0{
                        screen.image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
                    }
                    else{
                        screen.image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
                    }
                }
            }
            screen.update();
            let duration = SystemTime::now().duration_since(start_t).unwrap();
            thread::sleep(frame_time - duration);
        }
        Ok(())
    }
}
