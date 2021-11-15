use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0; // duration of a frame in milliseconds

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // clear screen with navy colour
        self.frame_time += ctx.frame_time_ms; // time elapsed since current frame time and context frame time
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // if user is pressing space call flap fx
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // render obstacle on screen. if pass obstacle, score += 1 and replace with new obstacle.
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // if player is off screen end game
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Crappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // ctx.key manages the currently pressed key state. respond accordingly.
        if let Some(key) = ctx.key {
            match key {
                // match P to restart (for now)
                VirtualKeyCode::P => self.restart(),
                // Q tells bracket-lib that you are ready to terminate
                VirtualKeyCode::Q => ctx.quitting = true,
                // match any input to nothing
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    // tick is the traffic cop directing the flow of the game modes
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // set a single char to the screen
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x; // if player.x and obstacle.x match, likely collision
        let player_above_gap = player.y < self.gap_y - half_size; // compare y vals to determine if x and y match
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap) // return true if either obstacles collide w/ player
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Crappy Dragon")
        .build()?; // finalize initialization into context. also pass errors to parent fx with ? if unable to initialize

    // implicitly run the main loop and return using context object and state. main and main_loop return a BError.
    main_loop(context, State::new())
}
