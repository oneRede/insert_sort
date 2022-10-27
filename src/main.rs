use std::{ptr::NonNull};

#[derive(Debug)]
struct InsertionSort {
    // value 0 for big sequence,value 1 for small sequence.
    order: Order,
}

#[derive(Debug)]
enum Order {
    Big,
    Small,
}

impl<'a> InsertionSort {
    fn new(order: Order) -> Self {
        Self { order }
    }

    fn sort(&mut self, data: &'a mut [u32]) -> &'a mut [u32] {
        for i in 1..(data.len()) {
            let await_value = data[i];
            for b in 0..i {
                if await_value >= data[b] {
                    for c in 0..(i - b) {
                        data[i - c] = data[i - 1 - c]
                    }
                    data[b] = await_value;
                    break;
                }
            }
        }
        match self.order {
            Order::Small => {
                for i in 0..(data.len() / 2) {
                    let await_value = data[i];
                    data[i] = data[data.len() - i - 1];
                    data[data.len() - i - 1] = await_value;
                }
                data
            }
            _ => data,
        }
    }

    fn sort_by_linkchain(data: &[u32]) -> Node {
        let mut header: NonNull<Node> = Box::leak(Box::new(Node {
            data: data[0],
            next: None,
            up: None,
        }))
        .into();

        for i in 1..data.len() {
            println!("data {:?}", data[i]);
            'outer: loop {
                if data[i] > unsafe { (*header.as_ptr()).data } {
                    let node = Box::new(Node {
                        data: data[i],
                        next: None,
                        up: None,
                    });
                    header = add_up(header, node);
                    break 'outer;
                } else if data[i] < unsafe { (*header.as_ptr()).data }
                    && unsafe { (*header.as_ptr()).next } != None
                {
                    header = (unsafe { (*header.as_ptr()).next }).unwrap();
                } else if unsafe { (*header.as_ptr()).next } == None {
                    let node = Box::new(Node {
                        data: data[i],
                        next: None,
                        up: None,
                    });
                    header = add_next(header, node);
                    loop {
                        if unsafe { (*header.as_ptr()).up } != None {
                            header = (unsafe { (*header.as_ptr()).up }).unwrap();
                        } else {
                            break;
                        }
                    }
                    break 'outer;
                }
            }
        }
        unsafe { (*header.as_ptr()) }
    }
}
#[derive(Debug, Copy, Clone)]
struct Node {
    data: u32,
    up: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
}

impl Node {
    fn new(data: u32) -> Self {
        Self {
            data: data,
            next: None,
            up: None,
        }
    }

    fn add_next(&mut self, node: &mut Node) -> &mut Self {
        node.next = self.next;
        let next = NonNull::new(node as *mut Node);
        self.next = next;
        let up = NonNull::new(self as *mut Node);
        node.up = up;
        self
    }

    fn add_new_v2(&mut self, mut node: Box<Node>) {
        let up = NonNull::new(self as *mut Node);
        node.up = up;
        let new_node: NonNull<Node> = Box::leak(node).into();
        self.next = Some(new_node);
    }

    fn add_up(&mut self, node: &mut Node) -> &mut Self {
        match self.up {
            Some(up) => {
                node.next = NonNull::new(self as *mut Node);
                let up = NonNull::new(node as *mut Node);
                unsafe { (*self.up.unwrap().as_ptr()).next = up }
                self
            }
            None => {
                node.up = None;
                self.up = NonNull::new(node as *mut Node);
                node.next = NonNull::new(self as *mut Node);
                self
            }
        }
    }

    fn get_header(mut self) -> Node {
        loop {
            if self.up != None {
                self = unsafe { *(self.up.unwrap().as_ref()) };
            } else {
                return self;
            }
        }
    }

    fn get_next(&self) -> Node {
        unsafe { (*self.next.unwrap().as_ref()) }
    }

    fn test_leak(&mut self, mut node: Box<Node>) {
        let node = Some(Box::leak(node));
    }
}

fn get_header(mut node: Node) -> Node {
    println!("{:?}", node);
    loop {
        if node.up != None {
            node = unsafe { (*node.up.unwrap().as_ref()) };
        } else {
            return node;
        }
    }
}

fn add_next(header: NonNull<Node>, node: Box<Node>) -> NonNull<Node> {
    let new_node: NonNull<Node> = Box::leak(node).into();
    unsafe {
        (*new_node.as_ptr()).up = Some(header);
        (*header.as_ptr()).next = Some(new_node);
        header
    }
}

fn add_up(header: NonNull<Node>, node: Box<Node>) -> NonNull<Node>{
    let new_node: NonNull<Node> = Box::leak(node).into();
    unsafe {
        (*header.as_ptr()).up = Some(new_node);
        (*new_node.as_ptr()).next = Some(header);
        new_node
    }
}

fn main() {
    let mut data: [u32; 8] = [10, 9, 12, 8000, 11, 300000, 300, 600000000];
    let mut is = InsertionSort::new(Order::Big);
    let s_dat = is.sort(&mut data);
    let mut data: [u32; 8] = [10, 9, 12, 8000, 11, 300000, 300, 600000000];
    let mut is = InsertionSort::new(Order::Small);
    let b_dat = is.sort(&mut data);
    println!("{:?}", s_dat);
    println!("{:?}", b_dat);

    let data: [u32; 4] = [100, 1000, 10, 1];
    let header = InsertionSort::sort_by_linkchain(&data);
    println!("main1{:?}", &header);
    // println!("{:?}", unsafe { *header.up.unwrap().as_ptr() });
    println!("main2{:?}", unsafe { *header.next.unwrap().as_ptr() });
    let tt1 = unsafe { *header.next.unwrap().as_ptr() };
    println!("main2{:?}", unsafe { *tt1.next.unwrap().as_ptr() });
    let tt2 = unsafe { *tt1.next.unwrap().as_ptr() };
    println!("main2{:?}", unsafe { *tt2.next.unwrap().as_ptr() });
}
