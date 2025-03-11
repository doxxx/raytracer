use std::str::FromStr;

use crate::color::Color;
use crate::direction::Direction;
use crate::materials::*;
use crate::matrix::Matrix44f;
use crate::object::Object;
use crate::point::Point;
use crate::sdl;
use crate::sdl::{Scene,SceneOptions};
use crate::shapes::*;
use crate::system::{Camera,Options};
use crate::texture::{Pattern,Texture};

peg::parser!{

    pub grammar sdl_grammar() for str {

        pub rule scene(render_options: &Options) -> Scene
            = options:options()? _ camera:camera(render_options) _ objects:one_or_more(<object()>) {
                Scene {
                options: options.unwrap_or(SceneOptions::default()),
                camera,
                objects,
                }
            }
        
        rule options() -> SceneOptions
            = "options" _ "{" _ bg:bg() _ "}" {
                SceneOptions {
                background_color: bg,
                }
            }
        
        rule bg() -> Color = "background" _ color:color() { color }

        pub rule camera(render_options: &Options) -> Camera
            = "camera" _ "{" _ o:origin() _ p:camera_lookat() _ fov:fov()? _ "}" {
                Camera::new(render_options.width as f64, render_options.height as f64, fov.unwrap_or(60.0), o, p)
            }
        
        rule camera_lookat() -> Point = "look_at" _ p:point() { p }
        
        rule fov() -> f64 = "fov" _ f:float() { f }

        pub rule object() -> Object
            = "object" _ name:string()? _ "{" _ shape:object_shape() _ material:object_material() _ "}" {
                sdl::new_object(name, shape, material)
            }

        rule object_shape() -> Box<dyn Shape>
            = planar_shape()
            / solid_shape()
            / mesh()

        rule object_material() -> Box<dyn Material> = "material" _ "{" _ material:material() _ "}" { material }

        rule solid_shape() -> Box<dyn Shape>
            = sphere()
            / cylinder()
            / torus()
            / cube()
            / csg()
            / homogenous_medium()

        rule sphere() -> Box<dyn Shape>
            = "sphere" _ "{" _ o:origin()? _ r:radius()? _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(Sphere::new(o.unwrap_or(Point::zero()), r.unwrap_or(1.0))), transform)
            }
        
        rule radius() -> f64 = "radius" _ r:float() { r }
        
        rule cylinder() -> Box<dyn Shape>
            = "cylinder" _ "{" _ r:radius()? _ h:height()? _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(Cylinder::new(r.unwrap_or(1.0), h.unwrap_or(1.0))), transform)
            }
        
        rule torus() -> Box<dyn Shape>
            = "torus" _ "{" _ r1:radius() _ r2:radius() _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(Torus::new(r1, r2)), transform)
            }

        rule cube() -> Box<dyn Shape>
            = "cube" _ "{" _ p1:point() _ p2:point() _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(Cube::new(p1, p2)), transform)
            }

        rule csg() -> Box<dyn Shape>
            = csg_union()
            / csg_intersection()
            / csg_difference()
        
        rule csg_union() -> Box<dyn Shape>
            = "union" _ "{" _ a:solid_shape() _ b:solid_shape()_  transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(CSGUnion::new(a, b)), transform)
            }
        
        rule csg_intersection() -> Box<dyn Shape>
            = "intersection" _ "{" _ a:solid_shape() _ b:solid_shape() _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(CSGIntersection::new(a, b)), transform)
            }
        
        rule csg_difference() -> Box<dyn Shape>
            = "difference" _ "{" _ a:solid_shape() _ b:solid_shape() _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(CSGDifference::new(a, b)), transform)
            }

        rule homogenous_medium() -> Box<dyn Shape>
            = "homogenous_medium" _ "{" _ density:density() boundary:solid_shape() transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(HomogenousMedium::new(boundary, density)), transform)
            }

        rule planar_shape() -> Box<dyn Shape>
            = plane()
            / xyrect()
            / xzrect()
            / zyrect()
            
        rule plane() -> Box<dyn Shape>
            = "plane" _ "{" _ o:origin()? _ n:plane_normal() _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(Plane::new(o.unwrap_or(Point::zero()), n)), transform)
            }
            
        rule plane_normal() -> Direction = "normal" _ n:direction() { n }
            
        rule mesh() -> Box<dyn Shape>
            = "mesh" _ "{" _ p:mesh_file() transform:transforms()? _ "}" {
                sdl::transform_shape(sdl::load_mesh_file(&p), transform)
            }
            
        rule mesh_file() -> String = "file" _ p:path() { p }
            
        rule xyrect() -> Box<dyn Shape>
            = "xyrect" _ "{" _ o:origin()? _ w:width() _ h:height() _ r:reverse()? _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(XYRectangle::new(o.unwrap_or(Point::zero()), w, h, r.is_some())), transform)
            }
            
        rule xzrect() -> Box<dyn Shape>
            = "xzrect" _ "{" _ o:origin()? _ w:width() _ h:height() _ r:reverse()? _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(XZRectangle::new(o.unwrap_or(Point::zero()), w, h, r.is_some())), transform)
            }
            
        rule zyrect() -> Box<dyn Shape>
            = "zyrect" _ "{" _ o:origin()? _ w:width() _ h:height() _ r:reverse()? _ transform:transforms()? _ "}" {
                sdl::transform_shape(Box::new(ZYRectangle::new(o.unwrap_or(Point::zero()), w, h, r.is_some())), transform)
            }
            
        rule reverse() -> () = "reverse"

        rule origin() -> Point = "origin" _ p:point() { p }
        
        rule width() -> f64 = "width" _ w:float() { w }
        
        rule height() -> f64 = "height" _ h:float() { h }
    
        rule density() -> f64 = "density" _ f:float() { f }

        rule material() -> Box<dyn Material>
            = lambertian()
            / metal()
            / dielectric()
            / diffuse_light()
            / isotropic()

        rule lambertian() -> Box<dyn Material>
            = "lambertian" _ texture:texture() {
                Box::new(Lambertian::new(texture))
            }

        rule metal() -> Box<dyn Material>
            = "metal" _ fuzz:fuzz() _ texture:texture() {
                Box::new(Metal::new(fuzz, texture))
            }

        rule dielectric() -> Box<dyn Material>
            = "dielectric" _ ior:ior() _ fuzz:fuzz()? {
                Box::new(Dielectric::new(ior, fuzz.unwrap_or(0.0)))
            }

        rule fuzz() -> f64 = "fuzz" _ n:float() { n }
        
        rule ior() -> f64 = "ior" _ n:float() { n }

        rule diffuse_light() -> Box<dyn Material>
            = "diffuse_light" _ i:intensity() _ texture:texture() {
                Box::new(DiffuseLight::new(i, texture))
            }

        rule intensity() -> f64 = "intensity" _ n:float() { n }

        rule isotropic() -> Box<dyn Material>
            = "isotropic" _ texture:texture() {
                Box::new(Isotropic::new(texture))
            }

        rule transforms() -> Matrix44f
            = "transform" _ "{" _ transforms:zero_or_more(<transform()>) _ "}" {
                sdl::combine_transforms(transforms)
            }

        rule transform() -> Matrix44f
            = translate()
            / rotate()
            / scale()
        
        rule translate() -> Matrix44f
            = "translate" _ d:direction() {
                Matrix44f::translation(d)
            }

        rule rotate() -> Matrix44f
            = rotate_x()
            / rotate_y()
            / rotate_z()
        
        rule rotate_x() -> Matrix44f
            = "rotate_x" _ n:float() {
                Matrix44f::rotation_x(n)
            }
        
        rule rotate_y() -> Matrix44f
            = "rotate_y" _ n:float() {
                Matrix44f::rotation_y(n)
            }

        rule rotate_z() -> Matrix44f
            = "rotate_z" _ n:float() {
                Matrix44f::rotation_z(n)
            }

        rule scale() -> Matrix44f
            = "scale" _ d:direction() {
                Matrix44f::scaling(d)
            }

        rule texture() -> Texture
            = "texture" _ "{" _ t:(texture_solid() / texture_pattern() / texture_image()) _ "}" { t }

        rule texture_solid() -> Texture
            = "solid" _ c:color() {
                Texture::Solid(c)
            }

        rule texture_pattern() -> Texture
            = "pattern" _ "{" _ p:pattern_checkerboard() _ "}" {
                Texture::Pattern(p)
            }

        rule pattern_checkerboard() -> Pattern
            = "checkerboard" _ c1:color() _ c2:color() _ s:float() {
                Pattern::Checkerboard(c1, c2, s)
            }

        rule texture_image() -> Texture
            = "image" _ p:path() _ s:float() {
                Texture::Image(sdl::load_image(&p), s)
            }

        rule path() -> String = string()

        rule string() -> String
            = "\"" s:$([^'"']*) "\"" whitespace()* {
                String::from_str(s).unwrap()
            }

        pub rule point() -> Point
            = v:vec3() { 
                Point::from_tuple(v)
            }

        pub rule direction() -> Direction
            = v:vec3() { Direction::from_tuple(v) }
            / "down"   { Direction::new(0.0, -1.0, 0.0) }
            / "up"     { Direction::new(0.0, 1.0, 0.0) }
            / "left"   { Direction::new(-1.0, 0.0, 0.0) }
            / "right"  { Direction::new(1.0, 0.0, 0.0) }
            / "back"   { Direction::new(0.0, 0.0, -1.0) }
            / "front"  { Direction::new(0.0, 0.0, 1.0) }
        
        pub rule color() -> Color
            = ("color"/"colour") _ c:(std_color() / rgb_color()) {
                Color::from_tuple(c)
            }

        rule std_color() -> (f64, f64, f64)
            = "white"   { (1.0, 1.0, 1.0) }
            / "black"   { (0.0, 0.0, 0.0) }
            / "red"     { (1.0, 0.0, 0.0) }
            / "green"   { (0.0, 1.0, 0.0) }
            / "blue"    { (0.0, 0.0, 1.0) }
            / "yellow"  { (1.0, 1.0, 0.0) }
            / "cyan"    { (0.0, 1.0, 1.0) }
            / "magenta" { (1.0, 0.0, 1.0) }

        rule rgb_color() -> (f64, f64, f64) = "rgb" _ v:vec3() { v }

        pub rule vec3() -> (f64, f64, f64) = full_vec3() / short_vec3()

        pub rule full_vec3() -> (f64, f64, f64) = "<" _ a:float() _ "," _ b:float() _ "," _ c:float() _ ">" {
            (a, b, c)
        }

        pub rule short_vec3() -> (f64, f64, f64) = "<" _ n:float() _ ">" {
            (n, n, n)
        }

        pub rule float() -> f64 
            = quiet!{
                s:$("-"? digit()+ ( "." digit()+ (['e' | 'E'] digit()+ )? )? ) {
                    f64::from_str(s).unwrap()
                }
            }
            / expected!("float literal")

        rule digit() = ['0'..='9']

        rule zero_or_more<E>(elem: rule<E>) -> Vec<E> = v:(e:elem() _ { e })* { v }
        rule one_or_more<E>(elem: rule<E>) -> Vec<E> = v:(e:elem() _ { e })+ { v }

        rule line_comment() = "//" [^'\n']*
        rule block_comment() = "/*" [^'*']* "*/"
        rule whitespace() = quiet!{[' ' | '\n' | '\r' | '\t']} / expected!("whitespace")

        rule _() = (whitespace() / line_comment() / block_comment())*

    }

}
