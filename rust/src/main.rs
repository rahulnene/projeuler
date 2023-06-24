const NUM_POINTS: usize = 2_000_000;
fn main() {
    let mut s: Vec<u64> = vec![0; NUM_POINTS * 2 + 10];
    let mut points: Vec<Point> = vec![Point::new(0, 0); NUM_POINTS];
    s[0] = 290_797;
    for n in 0..s.len() - 1 {
        s[n + 1] = s[n] * s[n] % 50_515_093;
    }
    for (n, point) in points.iter_mut().enumerate().take(NUM_POINTS) {
        let index = 2 * n;
        *point = Point::new(s[index], s[index + 1]);
    }
    let ans = shortest_distance(&points);
    dbg!(ans);
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    const fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    fn distance(&self, other: &Self) -> f64 {
        let dx = u64::abs_diff(self.x, other.x);
        let dy = u64::abs_diff(self.y, other.y);
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}

fn brute_force(points: &[Point]) -> f64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| p1.distance(p2)))
        .fold(f64::INFINITY, f64::min)
}

fn strip_closest(strip: &[Point], d: f64) -> f64 {
    strip
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            strip
                .iter()
                .skip(i + 1)
                .take_while(move |p2| u64::abs_diff(p1.y, p2.y) < d as u64)
                .map(move |p2| p1.distance(p2))
        })
        .fold(d, f64::min)
}

fn closest_pair(points_x: &[Point], points_y: &[Point]) -> f64 {
    if points_x.len() <= 50 {
        return brute_force(points_x);
    }

    let mid = points_x.len() / 2;
    let mid_point = &points_x[mid];

    let (left_x, right_x) = points_x.split_at(mid);
    let left_y: Vec<_> = points_y
        .iter()
        .filter(|point| point.x <= mid_point.x)
        .copied()
        .collect();
    let right_y: Vec<_> = points_y
        .iter()
        .filter(|point| point.x > mid_point.x)
        .copied()
        .collect();

    let dist_left = closest_pair(left_x, &left_y);
    let dist_right = closest_pair(right_x, &right_y);

    let min_dist = f64::min(dist_left, dist_right);

    let strip: Vec<_> = points_y
        .iter()
        .filter(|point| u64::abs_diff(point.x, mid_point.x) < min_dist as u64)
        .copied()
        .collect();

    f64::min(min_dist, strip_closest(&strip, min_dist))
}

fn shortest_distance(points: &[Point]) -> f64 {
    let mut points_x = points.to_vec();
    points_x.sort_by(|a, b| a.x.partial_cmp(&b.x).expect("Couldn't compare points x"));

    let mut points_y = points.to_vec();
    points_y.sort_by(|a, b| a.y.partial_cmp(&b.y).expect("Couldn't compare points y"));

    closest_pair(&points_x, &points_y)
}
