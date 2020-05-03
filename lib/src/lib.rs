pub mod audio;
pub mod midi;
pub mod osc;
pub mod gfx;
pub mod interp;

use nannou::geom::pt2;
use nannou::wgpu;
use log::*;
use audio::Audio;

use wavefront_obj::{mtl, mtl::Material, obj};

use std::cmp;
use std::path::{Path, PathBuf};
use std::env;

// TODO: Put this shit in multiple files

// TODO: Global font store

pub const RESOURCES_PATH: &'static str = "resources/";
pub fn resource(file: &str) -> PathBuf {
    let curr = env::current_exe().unwrap();
    // TODO: Recursively search for resources dir
    let resources = curr // phantoma/sketches/___/target/debug/___
        .parent().unwrap() // sketches/___/target/debug/
        .parent().unwrap() // sketches/___/target/
        .parent().unwrap() // sketches/___/
        .parent().unwrap() // sketches/
        .parent().unwrap() // /
        .join(RESOURCES_PATH); // sketches/resources/
    let file = Path::new(Path::new(file).file_name().unwrap());

    let dir = match file.extension() {
        Some(os) => {
            match os.to_str().unwrap() {
                "spv" => "shaders",
                "obj" => "models",
                "mtl" => "models",
                "dds" => "textures",
                _ => "",
            }
        }
        None => ""
    };

    resources.join(dir).join(file)
}

pub fn read_resource(file: &str) -> String {
    std::fs::read_to_string(resource(file)).unwrap()
}

pub fn read_resource_raw(file: &str) -> Vec<u8> {
    std::fs::read(resource(file)).unwrap()
}

pub fn read_model(file: &str) -> Vec<ObjectData> {
    let set = obj::parse(read_resource(file)).unwrap();
    let mtl = mtl::parse(read_resource(&set.material_library.unwrap())).unwrap();

    set.objects
        .into_iter()
        .map(|o| ObjectData::from(o, &mtl.materials))
        .collect()
}

// TODO: put this in gfx?
pub fn read_shader(device: &wgpu::Device, file: &str) -> wgpu::ShaderModule {
    wgpu::shader_from_spirv_bytes(device, &read_resource_raw(file))
}

pub fn init_logging(level: u8) {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let mut overridden = false;
    let verbosity = if std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| "0".into()) == "1" {
        overridden = true;
        "trace"
    } else {
        match level {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }
    };
    std::env::set_var("RUST_LOG", verbosity);

    pretty_env_logger::init();

    if overridden {
        warn!("RUST_BACKTRACE is set, overriding user verbosity level");
    } else if verbosity == "trace" {
        std::env::set_var("RUST_BACKTRACE", "1");
        trace!("RUST_BACKTRACE has been set");
    };
    info!(
        "Set verbosity to {}",
        std::env::var("RUST_LOG").expect("Should set RUST_LOG environment variable")
    );
}

pub fn rand(seed: f32) -> f32 {
    let p = pt2(seed + 10.0, seed + 3.0);
    let dt = p.perp_dot(pt2(12.9898, 78.233));
    let sn = dt % 3.14;
    (sn.sin() * 43758.5453).fract()
}

struct CharsIter {
    seed: f32,
}

impl CharsIter {
    const CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
}

impl Iterator for CharsIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed += 1000.0;
        let i = (rand(self.seed) * Self::CHARS.len() as f32).trunc() as isize;
        Some(unsafe {
            let ptr = Self::CHARS.as_ptr().offset(i);
            let slice = std::slice::from_raw_parts(ptr, 1);
            std::str::from_utf8_unchecked(slice)
        })
    }
}

pub fn chars(seed: f32) -> impl Iterator<Item = &'static str> {
    CharsIter { seed }
}

pub struct Decay {
    t: i32,
}

impl Decay {
    // 6 place fixed-point [0.0, 1.0]
    const MAX: i32 = 1_000_000_000;

    pub fn new() -> Self {
        Self { t: 0 }
    }

    pub fn v(&self) -> f32 {
        self.t as f32 / Self::MAX as f32
    }

    fn clamp(&mut self) {
        self.t = cmp::max(0, cmp::min(self.t, Self::MAX));
    }

    pub fn update(&mut self, mut delta: f32) {
        // TODO: BAD!!!
        if delta > 1.0 {
            delta = 1.0;
        }
        self.t -= (delta * Self::MAX as f32).round() as i32;
        self.clamp();
    }

    pub fn set(&mut self, t: f32) {
        self.t = (t * Self::MAX as f32).round() as i32;
        self.clamp();
    }

    pub fn set_max(&mut self) {
        self.t = Self::MAX;
    }

    pub fn is_zero(&self) -> bool {
        self.t == 0
    }
}

pub struct BeatDecay {
    pub f0: f32,
    pub f1: f32,
    pub thres: f32,
    pub sens: f32,
    pub overlap: bool,
    decay: Decay,
    e0: f32,
}

impl BeatDecay {
    pub fn new(f0: f32, f1: f32, thres: f32, overlap: bool, bpm: f32) -> Self {
        Self {
            f0,
            f1,
            thres,
            overlap,
            sens: 1.0 / (bpm / 60.0) * 1000.0,
            e0: 0.0,
            decay: Decay::new(),
        }
    }

    pub fn update(&mut self, delta: f32, audio: &dyn Audio) {
        let (e, e0) = (audio.rms_range(self.f0, self.f1), self.e0);
        self.e0 = e;

        self.decay.update(delta / self.sens);

        if e - e0 > self.thres && (self.overlap || self.decay.is_zero()) {
            self.decay.set_max();
        }
    }

    pub fn v(&self) -> f32 {
        self.decay.v()
    }
}

// TODO: Move this to its own module
#[derive(Debug)]
pub struct ObjectData {
    pub name: String,
    pub meshes: Vec<MeshData>,
}

impl ObjectData {
    fn from(o: obj::Object, materials: &Vec<Material>) -> Self {
        let meshes = o
            .geometry
            .iter()
            .map(|g| {
                let name = g.material_name.as_ref().unwrap();
                let material = materials.iter().find(|m| &m.name == name).unwrap();
                MeshData::from(&o, g, material.clone())
            })
            .collect();

        Self {
            name: o.name,
            meshes,
        }
    }
}

pub type VertTexNorm = ([f32; 3], [f32; 2], [f32; 3]);

#[derive(Debug)]
pub struct MeshData {
    pub data: Vec<VertTexNorm>,
    pub material: Material,
}

impl MeshData {
    fn from(o: &obj::Object, g: &obj::Geometry, material: Material) -> Self {
        println!("{}: {:?}", o.name, material);
        let mut data = vec![];

        let v = |i: usize| {
            let v = o.vertices[i];
            [v.x as f32, v.y as f32, v.z as f32]
        };

        let t = |i: usize| {
            let t = o.tex_vertices[i];
            [t.u as f32, t.v as f32]
        };

        let n = |i: usize| {
            let n = o.normals[i];
            [n.x as f32, n.y as f32, n.z as f32]
        };

        for s in &g.shapes {
            match s.primitive {
                obj::Primitive::Triangle(i, j, k) => {
                    data.push((v(i.0), t(i.1.unwrap()), n(i.2.unwrap())));
                    data.push((v(j.0), t(j.1.unwrap()), n(j.2.unwrap())));
                    data.push((v(k.0), t(k.1.unwrap()), n(k.2.unwrap())));
                }
                _ => unimplemented!(),
            }
        }

        Self { data, material }
    }
}