use clap::Parser;
use random_image::RandomImage;
fn main() {
    RandomImage::parse().run().unwrap()
}
