
// trait Animal {
//     fn speak(&self);
// }

// struct Dog;
// struct Cat;

// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Woof!");
//     }
// }

// impl Animal for Cat {
//     fn speak(&self) {
//         println!("Meow!");
//     }
// }

// fn make_it_speak(animal: &dyn Animal) {
//     animal.speak();
// }

// fn main() {
//     let dog = Dog;
//     let cat = Cat;

//     make_it_speak(&dog);
//     make_it_speak(&cat);
// }

// trait Animal	Defines a common behavior — “things that can speak.”
// impl Animal for Dog	Dog knows how to speak.
// impl Animal for Cat	Cat also knows how to speak.
// fn make_it_speak(animal: &dyn Animal)	Function that can work with any Animal.
// make_it_speak(&dog) and make_it_speak(&cat)	Same function, different behavior.
// So the code doesn’t care what kind of animal it is —
// it just knows the animal can .speak().
// That’s polymorphism!


trait Metrics {
    fn record(&self, metric: &str, value: f64);
}

struct ConsoleMetrics; 

impl Metrics for ConsoleMetrics {
    fn record(&self, metric: &str, value: f64) {
     println!("Metric recorded → {} = {}", metric, value);

    }
}


fn run_metrics_demo(metrics : &dyn Metrics) {
    metrics.record("logins", 1.0);
    metrics.record("errors", 0.0);
    metrics.record("views", 42.0);

}


fn main() {
    let conmet = ConsoleMetrics; 

    run_metrics_demo(&conmet);
}