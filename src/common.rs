// pub struct IpAddr {
//
// }

pub type Mail<T> = Box<T>;

pub trait IMailData {
    fn get_data(&self) -> &Vec<u8>;
    fn set_data(&self, data: &[u8]) -> bool;
}

pub fn create<T: Default>() -> T
{
    T::default()
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