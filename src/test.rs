#![allow(warnings)]

use super::*;
use minifb as mf;

const WHITE:(u8,u8,u8) = (255,255,255);

#[test]
fn main() {
    let mut window = super::Window::new("test",640,360);

    let mut args: Vec<(f32,f32)> = vec![];
    let mut clicked = false;
    let mut last_click = -1;
    while window.is_open() {

        //framebuffer.set_pixel(20,20,(250,100,100));
        //framebuffer.rectangle((100,100),(200,200),WHITE);
        //ramebuffer.line((0,0),(framebuffer.width as i16,
                                //framebuffer.height as i16 / 2)
                                //,WHITE);
        if(window.get_mouse_down(mf::MouseButton::Left)){
            let mouse_pos = window.get_mouse_pos(mf::MouseMode::Clamp).unwrap();
            if !args.iter().any(|a| a.0 == mouse_pos.0) && mouse_pos.0 as i64 != last_click{
                println!("clicked");
                args.push(mouse_pos);
                println!("{:?}",args);
            }
            clicked = true;
        }

        let framebuffer = &mut window.framebuffer;
        /*
        if clicked && !args.is_empty(){
            framebuffer.set_pixel(
                args[args.len()-1].0 as i64,
                args[args.len()-1].1 as i64,
                WHITE
                );
            clicked = false;
        }*/

        if(args.len() == 3){
            last_click = args[args.len()-1].0 as i64;
            framebuffer.parabola(   
                args.pop().unwrap(),
                args.pop().unwrap(),
                args.pop().unwrap(),
                 WHITE
             );
        }

        framebuffer.line((10,10),(15,100),(255,255,255));
        framebuffer.parabola((50.0,50.0),(150.0,300.0),(200.0,50.0),(255,255,255));
        framebuffer.parabola((50.0+200.0,300.0),(150.0+200.0,50.0),(200.0+200.0,300.0),(255,255,255));
    

        /*
        let mut args = vec![];
        while window.is_open() {
            if(window.get_mouse_down()){
                args.push(window.get_mouse_pos().unwrap());
            }
            if(args.len() == 3){
                break;
            }
        }
        let framebuffer = &mut window.framebuffer;

        framebuffer.parabola(   
            args.pop().unwrap(),
            args.pop().unwrap(),
            args.pop().unwrap(),
             WHITE
             );
        */
            window.display();
    }
}


