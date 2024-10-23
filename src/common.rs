// pub struct IpAddr {
//
// }

pub type Mail<T> = Box<T>;


pub trait IMailData {
    fn get_data(&self) -> &Vec<u8>;
    fn get_data_len(&self) -> u32;
    fn set_data(&mut self, data: &[u8]) -> bool;

}

#[derive(Default)]
pub struct GenericsFactory<T> {
    f1:T,
}


impl<T:Default> GenericsFactory<T> {
    pub fn create_instance() -> T {
        T::default()
    }
}
//
// //
// #[derive(Debug)]
// pub struct DefaultMail {
//     pub data: Vec<u8>,
// }
//
// impl IMailData for DefaultMail {
//     fn get_data(&self) -> &Vec<u8> {
//         &self.data
//     }
//
//     fn set_data(&self, data: &[u8]) -> bool {
//         true
//     }
// }