#![feature(drain_filter)]
mod app;

fn main() {
  app::run("temples", "temples");
}
