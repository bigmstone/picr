use std::{error::Error, ffi::OsStr, path::PathBuf};

use {
    anyhow::{anyhow, Result},
    egui::{ColorImage, Context, TextureHandle},
    fitrs::{Fits, HeaderValue},
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
    pub metadata: Vec<(String, String)>,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self> {
        let fits = fitrs::Fits::open(path.clone())?;
        let mut metadata = vec![];

        for hdu in fits.iter() {
            for hv in hdu.iter() {
                metadata.push((
                    hv.0.clone(),
                    header_display(hv.1.unwrap_or(&HeaderValue::CharacterString("".to_string()))),
                ));
            }
        }

        metadata.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        metadata.insert(
            0,
            (
                "File Name".to_string(),
                path.file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
        );

        Ok(Self {
            path,
            fits,
            texture: None,
            culled: false,
            metadata,
        })
    }

    pub fn build_texture(&mut self, ctx: &Context) -> Result<()> {
        if self.texture.is_some() {
            return Ok(());
        }

        for hdu in self.fits.iter() {
            if let fitrs::FitsData::IntegersI32(fd) = hdu.read_data() {
                let data: Vec<i32> = fd.data.iter().map(|d| d.unwrap_or(0)).collect();
                let mut gimage =
                    create_grayscale_image(fd.shape[0] as u32, fd.shape[1] as u32, data);
                imgproc::apply_stf_autostretch(&mut gimage);
                let width = 1024 * 2;
                let height = ((width as f32 / fd.shape[0] as f32) * fd.shape[1] as f32) as u32;
                let image = resize(&gimage, width, height, FilterType::Gaussian);
                self.texture = Some(ctx.load_texture(
                    "gray_image",
                    ColorImage::from_gray([width as usize, height as usize], &image.into_vec()),
                    Default::default(),
                ));
            }
        }
        Ok(())
    }
}

fn header_display(header: &HeaderValue) -> String {
    match header {
        HeaderValue::CharacterString(c) => c.to_owned(),
        HeaderValue::Logical(l) => {
            format!("{}", l)
        }
        HeaderValue::IntegerNumber(i) => {
            format!("{}", i)
        }
        HeaderValue::RealFloatingNumber(f) => {
            format!("{}", f)
        }
        HeaderValue::ComplexIntegerNumber(r, i) => {
            format!("{} {}i", r, i)
        }
        HeaderValue::ComplexFloatingNumber(r, i) => {
            format!("{} {}i", r, i)
        }
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

            if let Some(ext) = path.extension() {
                if ext == "fits" {
                    fits.push(process_file(path)?);
                }
            }
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
