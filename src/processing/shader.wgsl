@group(0)
@binding(0)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(1)
var outputTexture: texture_storage_2d<rgba8snorm, write>; // this is used as both input and output for convenience

// @group(0)
// @binding(1)
// var<storage, read_write> outputTexture: array<vec4<f32>>; // this is used as both input and output for convenience

@group(0)
@binding(2)
var<uniform> resolutions: vec4<u32>;


fn coords(x: u32, y: u32) -> u32 {
    return x + (y * resolutions[0]);
}

fn average(vec: vec3<f32>) -> f32 {
    return dot(vec, vec3<f32>(1.0 / 3.0)); // Efficient averaging using the dot product
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x == 0) || (global_id.y == 0) {
        return;
    }
    if (global_id.x >= resolutions[0] - 1) || (global_id.y >= resolutions[1] - 1) {
        return;
    }
            
    let gx = (
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 0), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 0), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );
    let gy = (
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y - 1), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );

    // TODO, how do i improve the soble edge detection, could use DoG, or some other approach making use of our weird scaling situation

    textureStore(outputTexture, global_id.xy, vec4<f32>(gx, gy, 0.0, 0.0));
}
