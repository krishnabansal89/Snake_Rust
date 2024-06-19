
use macroquad::prelude::*;
use ::rand::Rng;

const SPEED:f32 = 5.0;
struct Snake {
    body: Vec<Vec2>,
    direction: Vec2,
    head: Vec2,
    tail: Vec2,
}

struct food {
    position: Vec2,
}

fn greater_than(a: Vec2, b: Vec2) -> bool{
    if a.x > b.x && a.y > b.y{
        return true;
    }
    false
}
fn less_than(a: Vec2, b: Vec2) -> bool{
    if a.x < b.x && a.y < b.y{
        return true;
    }
    false
}

impl Snake{
    fn new() -> Self{
        let body = vec![vec2(0.0, 0.0) , vec2(1.0, 0.0) , vec2(2.0, 0.0) , vec2(3.0, 0.0)];
        let direction = vec2(1.0, 0.0);
        let head = vec2(0.0, 0.0);
        let tail = vec2(0.0, 0.0);
        Self {body, direction, head, tail}
    }
    fn update(&mut self){
        self.head = self.body[0];
        self.tail = self.body.pop().unwrap();
        self.head += self.direction * SPEED;
        self.body.insert(0, self.head);
    }
    fn draw_snake(&self){
        for part in &self.body{
            draw_rectangle(part.x, part.y, 10.0, 10.0, macroquad::color::WHITE);
        }
    }
    fn change_direction(&mut self, direction: Vec2){
        self.direction = direction;
    }
    fn eat(&mut self){
        self.body.push(self.tail);
    }
}

impl food {
    fn new() ->Self{
        let x:f32 = ::rand::thread_rng().gen_range(0.0..screen_width());
        let y:f32 = ::rand::thread_rng().gen_range(0.0..screen_height());
        let position = vec2(x, y);
        Self {position}
    }
    fn update(&mut self){
        self.position = vec2(::rand::thread_rng().gen_range(0.0..screen_width()), ::rand::thread_rng().gen_range(0.0..screen_height()));
    }
    fn draw_food(&self){
        draw_rectangle(self.position.x, self.position.y, 10.0, 10.0, macroquad::color::RED);
    }
}

#[macroquad::main("Snake 🐍")]
async fn main() {
    clear_background(macroquad::color::BLACK);
    let mut snake = Snake::new();
    let mut food = food::new();
    loop {
        snake.update();
        snake.draw_snake();
        food.draw_food();
        if is_key_pressed(macroquad::input::KeyCode::W ){
            snake.change_direction(vec2(0.0, -1.0));
        }
        if is_key_pressed(macroquad::input::KeyCode::S ){
            snake.change_direction(vec2(0.0, 1.0));
        }
        if is_key_pressed(macroquad::input::KeyCode::A ){
            snake.change_direction(vec2(-1.0, 0.0));
        }
        if is_key_pressed(macroquad::input::KeyCode::D ){
            snake.change_direction(vec2(1.0, 0.0));
        }

        if less_than((snake.head - food.position), vec2(10.0, 10.0)) && greater_than((snake.head - food.position), vec2(-10.0, -10.0)) {
            snake.eat();
            food.update();
        }

    
        // if(is_key_pressed(KeyCode::D)){
        //     snake.change_direction(vec2(1.0, 0.0));
        // }
        next_frame().await
    }
}

