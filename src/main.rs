extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::io;
use std::io::*;

pub struct App {
  gl: GlGraphics,
  rotation: f64,
  parser: CmdParser,
}

#[derive(Copy, Clone, Debug)]
pub struct Led {
  r: u8,
  g: u8,
  b: u8
}

pub struct CmdParser {
  command: [u8; 4],
  byteCounter: u8
}

impl CmdParser {
  fn parse(&mut self, c: u8) -> Option<(u8, u8, u8, u8)> {
    if c&0b10000000 == 0b10000000 {
      self.byteCounter = 0;
    }

    self.command[self.byteCounter as usize] = c;

    self.byteCounter += 1;

    if self.byteCounter == 4 {
      self.byteCounter = 0; 

      Some((self.command[0]&0b01111111, self.command[1], self.command[2], self.command[3]))
    } else {
      None
    }
  }
}


impl App {
  fn render(&mut self, leds: &[Led], args: &RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 20.0);

    self.gl.draw(args.viewport(), |c, gl| {
      clear(BLACK, gl);

      for (i, led) in leds.iter().enumerate() {
        ellipse([(led.r as f32)/127 as f32, (led.g as f32)/127 as f32, (led.b as f32)/127 as f32, 1.0],
          square, c.transform.trans((i as f64)*20 as f64, 0 as f64), gl);
      }
    });
  }

  fn update(&mut self, leds: &mut [Led], args: &UpdateArgs) {
    let mut buffer = [0; 1];
    match io::stdin().read(&mut buffer) {
      Ok(n) => {
        for x in buffer.iter() {
          if let Some((n @ 0...49, r, g, b)) = self.parser.parse(*x) {
            //println!("DEBUG: LED {} old value {},{},{}, new value {},{},{}", n, leds[n as usize].r, leds[n as usize].g, leds[n as usize].b, r, g, b);
            leds[n as usize].r = r;
            leds[n as usize].g = g;
            leds[n as usize].b = b;
          }
        }
      },
      Error => {
        println!("{}", "Error reading stdin!");
      }
    }
  }
}

fn main() {
  let opengl = OpenGL::V3_2;
  let mut window: Window = WindowSettings::new(
      "LED SIMULATOR 2016",
      [20*50, 20],
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
  
  let mut leds = [Led{r:0,g:0,b:0}; 50];

  let mut app = App {
    gl: GlGraphics::new(opengl),
    rotation: 0.0,
    parser: CmdParser {command: [0; 4], byteCounter: 0}
  };

  let mut events = window.events();
  events.set_ups(3000);
  events.set_max_fps(120);
  while let Some(e) = events.next(&mut window) {
    if let Some(r) = e.render_args() {
      app.render(&leds, &r);
    }

    if let Some(u) = e.update_args() {
      app.update(&mut leds, &u);
    }
  }
}
