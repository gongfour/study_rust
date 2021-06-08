
fn panic1()
{
    panic!("crash and burn");
}

fn panic2()
{
    let v = vec![1, 2, 3];
    v[99];
}

pub fn run()
{
    // panic1();
    panic2();
}