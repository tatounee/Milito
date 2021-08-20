use js_sys::Math::random;
use yew::services::ConsoleService;

// The log! macro is copied from the crate seed
// See: https://docs.rs/seed/0.8.0/seed/macro.log.html

#[allow(dead_code)]
pub fn wrap_debug<T: std::fmt::Debug>(object: T) -> T {
    object
}

#[macro_export]
macro_rules! log {
    { $($expr:expr),* $(,)? } => {
        {
            let mut formatted_exprs = Vec::new();
            $(
                formatted_exprs.push(format!("{:#?}", $crate::utils::wrap_debug(&$expr)));
            )*
            $crate::utils::log_1(
                &formatted_exprs
                    .as_slice()
                    .join(" ")
            );
        }
     };
}

#[allow(dead_code)]
pub fn log_1(data_1: &str) {
    ConsoleService::log(data_1);
}

#[inline]
#[allow(unused_unsafe)]
pub fn rng() -> f64 {
    // SAFETY: Milito is design to be runned into a web browser
    // unsafe { random() }
    rand::random::<f64>()
}

pub trait Median<T> {
    fn median(&mut self) -> Option<T>;
}

impl Median<f32> for Vec<f32> {
    fn median(&mut self) -> Option<f32> {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.len() % 2 == 0 {
            if self.is_empty() {
                None
            } else {
                Some(
                    (self.get((self.len() / 2) - 1).unwrap() + self.get(self.len() / 2).unwrap())
                        / 2.,
                )
            }
        } else {
            self.get(self.len() / 2).cloned()
        }
    }
}

pub trait GetRandom<T> {
    fn get_random(&self) -> Option<&T>;
    fn get_random_and_index(&self) -> Option<(&T, usize)>;
}

impl<T> GetRandom<T> for Vec<T> {
    fn get_random(&self) -> Option<&T> {
        self.get((rng() * self.len() as f64).floor() as usize)
    }

    fn get_random_and_index(&self) -> Option<(&T, usize)> {
        let index = (rng() * self.len() as f64).floor() as usize;
        self.get(index).map(|x| (x, index))
    }
}

impl<T> GetRandom<T> for [T] {
    fn get_random(&self) -> Option<&T> {
        self.get((rng() * self.len() as f64).floor() as usize)
    }

    fn get_random_and_index(&self) -> Option<(&T, usize)> {
        let index = (rng() * self.len() as f64).floor() as usize;
        self.get(index).map(|x| (x, index))
    }
}
