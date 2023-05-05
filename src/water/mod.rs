use bevy::{
    prelude::{
        default, shape, Assets, Color, Commands, Handle, Mesh, Query, Res, ResMut, Transform, Vec2,
        Vec3,
    },
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, MaterialMesh2dBundle, Mesh2dHandle},
    time::Time,
    window::Window,
};

use crate::board::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct WaterMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    size: Vec3,
    #[uniform(0)]
    resolution: Vec2,
    #[uniform(0)]
    time: f32,
}

impl Material2d for WaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/water.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/water.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

pub fn setup_water(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let win = window.single();
    let t = time.delta_seconds();
    let mesh2d = Mesh2dHandle(meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(
            BOARD_WIDTH as f32 * CELL_SIZE,
            BOARD_HEIGHT as f32 * CELL_SIZE,
        ),
        ..default()
    })));

    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(WaterMaterial {
            color: Color::rgba(0.2, 0.3, 0.6, 0.6),
            size: Vec3::new(
                BOARD_WIDTH as f32 * CELL_SIZE,
                BOARD_HEIGHT as f32 * CELL_SIZE,
                0.0,
            ),
            resolution: Vec2::new(win.width(), win.height()),
            time: t,
        }),
        mesh: mesh2d,
        transform: Transform::from_xyz(-BOARD_WIDTH as f32 / 2.0, -BOARD_HEIGHT as f32 / 2.0, 0.0),
        ..default()
    });
}

pub fn update_water(
    handle: Query<&Handle<WaterMaterial>>,
    mut material: ResMut<Assets<WaterMaterial>>,
    time: Res<Time>,
) {
    let h = handle.single();
    let m = material.get_mut(h).unwrap();

    m.time = time.elapsed_seconds();
}
