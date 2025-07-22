//use gilrs::Gamepad;
//use uuid::Uuid;

//use crate::prelude::*;

//impl InputDevice for Gamepad<'_> {
//    fn get_name(&self) -> &str {
//        self.name()
//    }
//
//    fn get_uuid(&self) -> Uuid {
//        Uuid::from_bytes(self.uuid())
//    }
//}

//impl InputPoll for Gamepad<'_> {
//    fn poll_button(&mut self, code: InputButtonCode) -> Option<f32> {
//        None
//    }
//
//    fn poll_axis(&mut self, code: InputAxisCode) -> Option<f32> {
//        None
//    }
//}
