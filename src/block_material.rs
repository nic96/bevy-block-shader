use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayout};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat,
};

pub struct BlockMaterialPlugin;

impl Plugin for BlockMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlockMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlockMaterial {
    /// The texture component of the material's color before lighting.
    #[texture(101, dimension = "2d_array")]
    #[sampler(102)]
    #[dependency]
    pub base_color_array_texture: Handle<Image>,
    #[texture(103, dimension = "2d_array")]
    #[sampler(104)]
    #[dependency]
    pub emissive_array_texture: Handle<Image>,
    #[texture(105, dimension = "2d_array")]
    #[sampler(106)]
    #[dependency]
    pub metallic_roughness_array_texture: Handle<Image>,
    pub alpha_mode: AlphaMode,
}

impl BlockMaterial {
    pub const ATTRIBUTE_VERT_DATA: MeshVertexAttribute =
        MeshVertexAttribute::new("VERT_DATA", 65763143005, VertexFormat::Uint32x3);
}

impl Material for BlockMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/block_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/block_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/block_shader_prepass.wgsl".into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/block_shader_prepass.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout =
            layout.get_layout(&[Self::ATTRIBUTE_VERT_DATA.at_shader_location(0)])?;

        // descriptor.primitive.polygon_mode = PolygonMode::Line;

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
