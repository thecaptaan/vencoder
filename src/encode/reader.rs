use std::process::Command;

fn reader() {
    let input_file = "input.mp4";
    let output_file = "output.webm";

    let output = Command::new("ffmpeg")
        .args(&[
            "-i", input_file,
            //"-c:v", "h264_nvenc",  // Use NVIDIA GPUe encoder (ensure ffmpeg was compiled with NVENC support)
            "-c:a", "libvorbis",    // Use Vorbis audio codec
            "-strict", "-2",        // Enable experimental codec (Vorbis)
            output_file
        ])
        .output()
        .expect("failed to execute ffmpeg");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("✅ Conversion completed! Check {}", output_file);
}
