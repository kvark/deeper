extern crate specs;
extern crate cgmath;
use zerocopy::{AsBytes, FromBytes};

use cgmath::{Vector2, Vector3};


// TODO: Switch to legion? https://docs.rs/legion/0.2.1/legion/
use specs::prelude::*;
use specs::{Component, VecStorage};

use std::f32::consts::PI;
use crate::graphics;
use crate::graphics::LocalUniforms;
use self::cgmath::{Matrix4, Deg};

// Note(Jökull): Begin entity pointers
pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn from_entity(entity: Entity) -> Self {
        return Self {
            entity,
        };
    }
}

pub struct ActiveCamera(pub Entity);

pub struct PlayerCamera(pub Entity);

// end entity pointers

pub struct FrameTime(pub f32);

#[derive(Component, Debug, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position(pub Vector2<f32>);

impl Position {
    pub fn to_vec3(self) -> Vector3<f32> {
        Vector3::new(self.0.x, self.0.y, 0.0)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2<f32>);

impl Velocity {
    pub fn new() -> Velocity { Velocity(Vector2::new(0.0, 0.0)) }
}

#[derive(Component)]
pub struct Orientation(pub Deg<f32>);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct StaticBody;

#[derive(Component)]
pub struct DynamicBody(pub f32);

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct SquareCollider {
    pub side_length: f32,
}

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct AIFollow {
    pub target: Entity,
    pub minimum_distance: f32,
}

#[derive(Component)]
pub struct Destination(pub Vector2<f32>);

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum Faction {
    Enemies,
    Friends,
}

#[derive(Component)]
pub struct HitPoints {
    pub max: f32,
    pub health: f32,
}

#[derive(Copy, Clone)]
pub enum MapTransition {
    None,
    Deeper, // Down to the next floor
}

#[derive(Component)]
pub struct MapSwitcher(pub MapTransition);

#[derive(Component)]
pub struct Camera {
    pub fov: f32,
    pub up: Vector3<f32>,
}

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct Position3D(pub Vector3<f32>);

#[derive(Component)]
pub struct SphericalOffset {
    pub theta: f32,
    pub phi: f32,
    pub radius: f32,
    pub theta_delta: f32,
    pub phi_delta: f32,
    pub radius_delta: f32,
}

// Note(Jökull): Until we have a standardized way of interacting or setting these values,
//               we can have the defaults as the most practical
impl SphericalOffset {
    pub fn new() -> Self {
        Self {
            theta: PI / 3.0,
            phi: 0.2 * PI,
            radius: 15.0,
            // TODO: Not satisfactory, but need to limit untraceable magic constants
            theta_delta: -0.005,
            phi_delta: 0.005,
            radius_delta: 0.1,
        }
    }
}

#[derive(Component)]
pub struct StaticModel {
    pub idx: usize,
    pub bind_group: wgpu::BindGroup,
}

impl StaticModel {
    pub fn new(context: &graphics::Context, idx: usize, offset: Vector3<f32>, scale: f32, z_rotation: f32, material: graphics::Material) -> Self {
        let uniforms_size = std::mem::size_of::<graphics::LocalUniforms>() as u64;

        let mut matrix = Matrix4::from_scale(scale);
        matrix = Matrix4::from_angle_z(cgmath::Deg(z_rotation)) * matrix;
        matrix = Matrix4::from_translation(offset) * matrix;

        let local_uniforms = graphics::LocalUniforms {
            model_matrix: matrix.into(),
            material,
        };

        let uniform_buf = context.device.create_buffer_with_data(
            local_uniforms.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let bind_group = context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &context.local_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buf,
                            range: 0..uniforms_size,
                        }
                    },
                ],
            }
        );

        Self {idx, bind_group}
    }
}

#[derive(Component)]
pub struct Model3D {
    pub idx: usize,
    pub offset: Vector3<f32>,
    pub scale: f32,
    pub z_rotation: f32,
    pub material : graphics::Material,

    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

// Note(Jökull): Probably not great to have both constructor and builder patterns
impl Model3D {
    pub fn new(context: &graphics::Context) -> Self {

        let uniforms_size = std::mem::size_of::<graphics::LocalUniforms>() as u64;

        let uniform_buf = context.device.create_buffer(&wgpu::BufferDescriptor {
            size: uniforms_size,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        });

        let bind_group = context.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &context.local_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buf,
                            range: 0..uniforms_size,
                        }
                    },
                ],
            }
        );

        Self {
            idx: 0,
            offset: Vector3::new(0.0, 0.0, 0.0),
            material: graphics::Material::default(),
            bind_group,
            scale: 1.0,
            z_rotation: 0.0,
            uniform_buffer: uniform_buf,
        }
    }

    pub fn from_index(context: &graphics::Context, index: usize) -> Model3D {
        let mut m = Self::new(context);
        m.idx = index;
        return m;
    }

    pub fn with_offset(mut self, offset: Vector3<f32>) -> Model3D {
        self.offset = offset;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_z_rotation(mut self, z_rotation: f32) -> Self {
        self.z_rotation = z_rotation;
        self
    }

    pub fn with_material(mut self, material: graphics::Material) -> Self {
        self.material = material;
        self
    }
}

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall(Option<WallDirection>),
    Floor,
    Path,
    Nothing,
    LadderDown,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum WallDirection {
    North,
    West,
    South,
    East,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Position3D>();
    world.register::<Orientation>();
    world.register::<Velocity>();
    world.register::<Speed>();
    world.register::<Acceleration>();
    world.register::<Camera>();
    world.register::<Target>();
    world.register::<SphericalOffset>();
    world.register::<Model3D>();
    world.register::<StaticModel>();
    world.register::<TileType>();
    world.register::<StaticBody>();
    world.register::<DynamicBody>();
    world.register::<CircleCollider>();
    world.register::<SquareCollider>();
    world.register::<AIFollow>();
    world.register::<Destination>();
    world.register::<MapSwitcher>();
    world.register::<Faction>();
    world.register::<HitPoints>();
}
