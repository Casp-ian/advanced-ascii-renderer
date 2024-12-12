@group(0)
@binding(0)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(1)
var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

var<workgroup> test: u32;

// fn toFlat(x: u32, y: u32) -> u32 {
//     return ()
// }

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = u32(textureLoad(inputTexture, vec2<u32>(global_id.x, global_id.y), 0).r * 255);
    // v_indices[global_id.x] = collatz_iterations(v_indices[global_id.x]);
}
