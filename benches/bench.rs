use aar::{cli::Args, video::FrameGrabber};
use criterion::{criterion_group, criterion_main, Criterion};

fn grab_video_frames(c: &mut Criterion) {
    let mut group = c.benchmark_group("frames");
    group.sample_size(10);

    let args = Args {
        path: "examples/steamboat_willie.mp4".into(), // TODO need example data
        width: None,                                  // doesnt matter for this test
        height: None,                                 // doesnt matter for this test
        color: aar::cli::ColorSet::None,              // doesnt matter for this test
        set: aar::cli::CharSet::Ascii,                // doesnt matter for this test
        quality: 5,
        volume: 0, // doesnt matter for this test
        format: None,
        inverted: false, // doesnt matter for this test
        no_lines: false, // doesnt matter for this test
        char_width: 0,   // doesnt matter for this test
        char_height: 0,  // doesnt matter for this test
        media_mode: aar::cli::MediaModes::Video,
        processing_mode: aar::cli::ProcessingModes::Gpu,
    };
    let grabber = &FrameGrabber::new(&args).unwrap();

    group.bench_function("grab_video_frames", |b| {
        b.iter(|| {
            let text = grabber.grab();
            assert!(text.is_some());
        });
    });
}

fn grab_device_frames(c: &mut Criterion) {
    let mut group = c.benchmark_group("frames");
    group.sample_size(10);

    let args = Args {
        path: "/dev/video0".into(),      // TODO this is linux hardcoded, fix
        width: None,                     // doesnt matter for this test
        height: None,                    // doesnt matter for this test
        color: aar::cli::ColorSet::None, // doesnt matter for this test
        set: aar::cli::CharSet::Ascii,   // doesnt matter for this test
        quality: 5,
        volume: 0, // doesnt matter for this test
        format: None,
        inverted: false, // doesnt matter for this test
        no_lines: false, // doesnt matter for this test
        char_width: 0,   // doesnt matter for this test
        char_height: 0,  // doesnt matter for this test
        media_mode: aar::cli::MediaModes::Stream,
        processing_mode: aar::cli::ProcessingModes::Gpu,
    };
    let grabber = &FrameGrabber::new(&args).unwrap();

    group.bench_function("grab_device_frames", |b| {
        b.iter(|| {
            let text = grabber.grab();
            assert!(text.is_some());
        });
    });
}
criterion_group!(frames, grab_video_frames, grab_device_frames);
criterion_main!(frames);
