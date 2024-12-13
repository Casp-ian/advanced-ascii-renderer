@group(0)
@binding(0)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(1)
var outputTexture: texture_storage_2d<rgba8unorm, write>; // this is used as both input and output for convenience

fn average(vec: vec3<f32>) -> f32 {
    return dot(vec, vec3<f32>(1.0 / 3.0)); // Efficient averaging using the dot product
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let gx = abs(
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 0), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 0), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );
    let gy = abs(
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y - 1), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );
    textureStore(outputTexture, global_id.xy, vec4<f32>(gx, gy, 0.0, 0.0));    // let texel = textureLoad(inputTexture, vec2<u32>(global_id.x, global_id.y), 0);
    // textureStore(outputTexture, vec2<u32>(global_id.x, global_id.y), texel);
    // Determine the pixel position in the texture
}
