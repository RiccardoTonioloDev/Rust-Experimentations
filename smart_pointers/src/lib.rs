pub trait Messenger{
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize
}
impl <'a,T> LimitTracker<'a,T> 
    where T: Messenger{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker { messenger, value: 0, max }
    }
    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max = self.value as f64 /self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }else if percentage_of_max >= 0.9{
            self.messenger.send("Error: You've used up over 90% of your quota!");
        }else if percentage_of_max >= 0.75{
            self.messenger.send("Error: You've used up over 75% of your quota!");
        }
    }
}

use std::cell::RefCell;

#[cfg(test)]
mod tests{

    use super::*;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger{
        fn new()->MockMessenger{
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn over_75_percent_warning() {
        let mock_msg = MockMessenger::new();
        let mut lmt_trckr = LimitTracker::new(&mock_msg,100);

        lmt_trckr.set_value(80);

        assert_eq!(mock_msg.sent_messages.borrow().len(),1);
    }
}
