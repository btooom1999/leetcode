#[derive(Debug)]
struct MyCalendarThree {
    data: Vec<(i32, i32)>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        let k = self.data.partition_point(|&v| v.0 < start_time);
        self.data.insert(k, (start_time, end_time));

        let mut stack = vec![];
        let mut max_end_time = self.data[0].1;
        let mut res = 0;
        let mut max = 0;
        for &(start_time, end_time) in &self.data {
            if start_time >= max_end_time {
                max_end_time = 0;
                max = 0;
                stack.clear();
            }

            while stack.last().is_some_and(|&v| v <= start_time) {
                max -= 1;
                stack.pop();
            }

            let k = stack.partition_point(|&v| v > end_time);
            stack.insert(k, end_time);

            max_end_time = max_end_time.max(end_time);
            max += 1;
            res = res.max(max);
        }

        res
    }
}

pub fn main() {
    let mut my_calendar_three = MyCalendarThree::new();
    println!("{}" , my_calendar_three.book(24,40));
    println!("{}", my_calendar_three.book(43,50));
    println!("{}", my_calendar_three.book(27,43));
    println!("{}", my_calendar_three.book(5,21));
    println!("{}", my_calendar_three.book(30,40));
    println!("{}", my_calendar_three.book(14,29));
    println!("{}", my_calendar_three.book(3,19));
    println!("{}", my_calendar_three.book(3,14));
    println!("{}", my_calendar_three.book(25,39));
    println!("{:?}", my_calendar_three);
}
