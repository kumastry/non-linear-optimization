extern crate nalgebra as na;
use na::Vector3;
fn main() {
    let a = na::Matrix3::new(2.0, -1.0, 0.0,
                          -1.0, 3.0, -1.0,
                          0.0, -1.0, 3.0);
    let b = na::Vector3::new(100.0, 50.0, 50.0);

    let mut xk = na::Vector3::new(1.0, 2.0, 3.0);

    let mut grad_xk = a * xk + b;
    let gg:f64 = grad_xk.norm();

    const EPS:f64 = 0.1;
    //let dk:Vector3<f64> = - grad_xk.clone();
    println!("{:?}", grad_xk);
    println!("{}", gg);
    println!("{:?}", grad_xk);

    println!("start");
    println!("{:?}", xk);
    while grad_xk.norm() > EPS {

        let dk:Vector3<f64> = - grad_xk.clone();
        let ak:f64 = 0.1;

        xk = xk + ak * dk;
        grad_xk = a * xk + b;
    }

    println!("ans");
    println!("{:?}", xk);
}
