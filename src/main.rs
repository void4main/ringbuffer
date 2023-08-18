mod rb;
use crate::rb::ringbuffer::RingBuffer;

fn main() {
    // let mut my_buff:RingBuffer<i32> = RingBuffer::new(10);
    let mut my_buff = RingBuffer::<i32>::new(10);

    for i in 1..12 {
        my_buff.push(i);
    }

    let result = my_buff.pull();
    my_buff.print();
    println!("{:?}", result);
    let result = my_buff.pull();
    my_buff.print();
    println!("{:?}", result);
}
