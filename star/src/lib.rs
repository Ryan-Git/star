mod topo;

use std::future::Future;
use std::thread::JoinHandle;
//
// pub fn spawn<T>(f: T) -> JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,
// {
//     unimplemented!()
// }
//
// pub fn spawn_local<T>(f: impl Future<Output = T> + 'static) -> Task<T>
// where
//     T: 'static,
// {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
