mod game;

fn main() {
    let fb = game::FrameBuffer {
        width: 40,
        height: 20,
        chars: [' '; 800],
    };
    fb.clear_screen();
    let mut g = game::create_game();
    let result = g.run(fb);
    let message = if result == game::Result::Win {"You win"} else {"You lose"};
    println!("{}", message);
}
