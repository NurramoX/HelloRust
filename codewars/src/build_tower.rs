pub fn tower_builder(n: usize) -> Vec<String> {
    let mut tower: Vec<String> = Vec::with_capacity(n);

    for i in 0..n {
        let spaces = " ".repeat(n - 1 - i);
        let starts = "*".repeat(i * 2 + 1);
        let floor = format!("{}{}{}", spaces, starts, spaces);
        tower.push(floor);
    }

    tower
}