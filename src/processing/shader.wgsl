@group(0)
@binding(0)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(1)
var outputTexture: texture_storage_2d<rgba8snorm, write>; // this is used as both input and output for convenience

fn average(vec: vec3<f32>) -> f32 {
    return dot(vec, vec3<f32>(1.0 / 3.0)); // Efficient averaging using the dot product
}

// const sobelX = array<array<f32, 3>, 3>(
//     array<f32, 3>(1.0, 0.0, -1.0),
//     array<f32, 3>(2.0, 0.0, -2.0),
//     array<f32, 3>(1.0, 0.0, -1.0)
// );
// const sobelY = array<array<f32, 3>, 3>(
//     array<f32, 3>( 1.0,  2.0,  1.0),
//     array<f32, 3>( 0.0,  0.0,  0.0),
//     array<f32, 3>(-1.0, -2.0, -1.0)
// );

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    // var gx: f32 = 0.0;
    // var gy: f32 = 0.0;
    // for (var i = -1; i <= 1; i++) {
    //     for (var j = -1; j <= 1; j++) {
    //         let pixel = average(textureLoad(inputTexture, vec2<u32>(global_id.x + u32(i), global_id.y + u32(j)), 0).rgb);
    //         gx += pixel * sobelX[i + 1][j + 1];
    //         gy += pixel * sobelY[i + 1][j + 1];
        
    //     }
    // }
            
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
    // let gaussian1 = (
    //       1.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
    //     + 2.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y - 1), 0).rgb)
    //     + 1.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
    //     - 1.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
    //     - 2.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y + 1), 0).rgb)
    //     - 1.0 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    // );

    textureStore(outputTexture, global_id.xy, vec4<f32>(gx, gy, 0.0, 0.0));    // let texel = textureLoad(inputTexture, vec2<u32>(global_id.x, global_id.y), 0);
    // textureStore(outputTexture, vec2<u32>(global_id.x, global_id.y), texel);
    // Determine the pixel position in the texture
}
