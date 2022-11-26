/// We need to enumerate all initial velocities that will make the
/// probe end inside the target.  This can be done by trial and error,
/// the issue is that there are two variables (xv, yv).  I *think* that finding the bad cases will help

/// If the drone falls short, that is ends under target_y2 before
/// reaching target_x, each of (vx,vy) is a low bound to its other
/// value.
///
/// If the drone goes too far, that is ends > target_x2 before
/// reaching target_y1, each value is a high bound for the other.
///
/// If the drone goes through the target, just ignore the case.
///

struct Probe {
    x: i32,
    y: i32,
    xv: i32,
    yv: i32,
}

struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Probe {
    fn new(xv: i32, yv: i32) -> Probe {
        Probe { x: 0, y: 0, xv, yv }
    }

    fn launch(&mut self, target: &Target) -> Option<i32> {
        let mut y_max = 0;
        loop {
            y_max = std::cmp::max(y_max, self.y);
            if self.is_on_target(target) {
                return Some(y_max);
            } else if self.is_lost(target) {
                return None
            }
            self.step();
        }
    }

    fn step(&mut self) {
        self.x += self.xv;
        self.y += self.yv;
        if self.xv < 0 {
            self.xv += 1
        } else if self.xv > 0 {
            self.xv -= 1
        }
        self.yv -= 1;
        // println!("Drone at {},{}", self.x, self.y);
    }

    fn is_on_target(&self, t: &Target) -> bool {
        (self.x >= t.x1)
            && (self.x <= t.x2)
            && (self.y >= t.y1)
            && (self.y <= t.y2)
    }

    fn is_lost(&self, t: &Target) -> bool {
        self.x > t.x2
            || self.y < t.y1
    }
}

fn main() {
    let target = Target {
        x1: 111,
        x2: 161,
        y1: -154,
        y2: -101,
        // x1: 20,
        // x2: 30,
        // y1: -10,
        // y2: -5,
    };

    let mut best: i32 = 0;
    let mut count: i32 = 0;
    let xvmax = std::cmp::max(target.x1, target.x2) + 1;
    let yvmin = std::cmp::min(target.y1, target.y2) - 1;
    println!("Range is (0..{}, {},{})", xvmax, yvmin, -yvmin);
    for xv in 0..xvmax {
        for yv in yvmin..-yvmin {
            if let Some(y) = Probe::new(xv, yv).launch(&target) {
                if y > best {
                    best = y;
                }
                count += 1;
            }
        }
    }

    println!("Best altitude is {} in {} solutions", best, count);
}
