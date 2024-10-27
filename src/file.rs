use std::{error::Error, path::PathBuf};

use {
    anyhow::{anyhow, Result},
    egui::{Context, TextureHandle},
    fitrs::Fits,
    image::{
        imageops::{resize, FilterType},
        GrayImage,
    },
};

use super::imgproc;

pub struct File {
    pub path: PathBuf,
    fits: Fits,
    pub texture: Option<TextureHandle>,
    pub culled: bool,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            path: path.clone(),
            fits: fitrs::Fits::open(path)?,
            texture: None,
            culled: false,
        })
    }

    pub fn build_texture(&mut self, ctx: &Context) -> Result<()> {
        if self.texture.is_some() {
            return Ok(());
        }

        for hdu in self.fits.iter() {
            for hv in hdu.iter() {
                println!("{:?}", hv);
            }
            if let fitrs::FitsData::IntegersI32(fd) = hdu.read_data() {
                let data: Vec<i32> = fd.data.iter().map(|d| d.unwrap_or(0)).collect();
                let mut gimage =
                    create_grayscale_image(fd.shape[0] as u32, fd.shape[1] as u32, data);
                imgproc::apply_stf_autostretch(&mut gimage);
                println!("Here");
                let width = 800;
                let height = ((800. / fd.shape[0] as f32) * fd.shape[1] as f32) as u32;
                let image = resize(&gimage, width, height, FilterType::Gaussian);
                self.texture = Some(ctx.load_texture(
                    "gray_image",
                    egui::ColorImage::from_gray(
                        [width as usize, height as usize],
                        &image.into_vec(),
                    ),
                    Default::default(),
                ));
            }
        }
        Ok(())
    }
}

fn create_grayscale_image(width: u32, height: u32, data: Vec<i32>) -> GrayImage {
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    let scale = if max > min {
        255.0 / (max - min) as f32
    } else {
        1.0
    };

    let pixels: Vec<u8> = data
        .iter()
        .map(|&val| ((val - min) as f32 * scale) as u8)
        .collect();

    GrayImage::from_vec(width, height, pixels)
        .expect("Image dimensions do not match the pixel data length")
}

fn process_file(path: PathBuf) -> Result<File> {
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "fits" {
                println!("File type: {:?}", ext);
                Ok(File::new(path)?)
            } else {
                Err(anyhow!("Not fits extension"))
            }
        } else {
            Err(anyhow!("No file extension"))
        }
    } else {
        Err(anyhow!("Not a file"))
    }
}

pub fn load(path: PathBuf) -> Result<Vec<File>, Box<dyn Error>> {
    let mut fits = vec![];
    if path.is_dir() {
        for entry in path.read_dir()? {
            let file = entry?;
            let path = file.path();
            fits.push(process_file(path)?);
        }
    } else {
        fits.push(process_file(path)?);
    }

    fits.sort_by(|a, b| {
        a.path
            .metadata()
            .unwrap()
            .created()
            .unwrap()
            .cmp(&b.path.metadata().unwrap().created().unwrap())
    });

    Ok(fits)
}
