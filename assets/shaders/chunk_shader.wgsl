#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path


@group(1) @binding(0) var<storage> tile_data: array<i32,256>;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

const blocks_per_chunk : f32 = 16.;
const tile_size_pixels : f32 = 8.;
const tile_map_size : f32 = 256.;


@fragment
fn fragment(mesh: VertexOutput,) -> @location(0) vec4<f32> {
    let amount = local_coords_to_1d_index(uv_to_local_coords(mesh.uv));
    return vec4(tile_data[bitcast(amount)],1.,1.,1.) * textureSample(base_color_texture, base_color_sampler, uv_to_tile_map(mesh.uv) + vec2f(0./16., 0./16.)) *1.0;
}


fn uv_to_tile_map(uv: vec2f) -> vec2f {
    return ((uv%(1./16.)));
}

fn uv_to_local_coords(uv : vec2f) -> vec2f {
    return floor(uv * blocks_per_chunk);
}

fn local_coords_to_1d_index(local_coords : vec2f) -> f32 {
    return (local_coords.y * blocks_per_chunk) + local_coords.x;
}